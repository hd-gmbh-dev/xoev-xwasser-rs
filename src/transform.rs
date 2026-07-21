//! Streaming XML transform for XWasser messages using quick-xml.
//!
//! Mutates `<leser>` and `<autor>` inside `nachrichtenkopf.g2g` and
//! replaces `<zusatzinformationen>` content entirely in a single pass,
//! preserving all comments, processing instructions, whitespace text nodes,
//! and attribute order.
//!
//! Element matching uses resolved namespace URIs, not prefixes — so the
//! transform works regardless of which prefix the source XML uses for the
//! XWasser namespace.
//!
//! Inserted elements use the indentation style of the surrounding XML
//! (captured from the first whitespace text node at the parent level),
//! falling back to 4-space (g2g children) or 2-space (root children) when
//! no indentation is detected (e.g. single-line compact XML).
//!
//! A no-op transform (all options `None`) produces output that is
//! byte-identical to the input, keeping XML digital signatures valid.

use raxb::quick_xml::{
    NsReader, Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
    name::{Namespace, ResolveResult},
};

/// The XWasser namespace URI as a quick-xml `Namespace` value.
const XWAS_NS: Namespace = Namespace(crate::TNS);

// ---------------------------------------------------------------------------
// Options structs
// ---------------------------------------------------------------------------

/// Top-level options for the XML transform.
#[derive(Debug, Clone, Default)]
pub struct TransformOptions<'a> {
    /// Optional update for the `<leser>` element.
    pub leser: Option<ElementUpdate>,
    /// Optional update for the `<autor>` element.
    pub autor: Option<ElementUpdate>,
    /// Replacement content for `<zusatzinformationen>`.
    /// - `None`: no change / do not insert if missing.
    /// - `Some(&[])`: replace existing with empty block.
    /// - `Some(&[...])`: replace full content with given entries.
    pub zusatzinformationen: Option<&'a [ZustaendigeBehoerdeUpdate]>,
}

/// Update parameters for an element inside `nachrichtenkopf.g2g`
/// that has `<kennung>` and `<name>` children (e.g. `<leser>`, `<autor>`).
#[derive(Debug, Clone, Default)]
pub struct ElementUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

/// A single `<zustaendigeBehoerde>` entry.
#[derive(Debug, Clone, Default)]
pub struct ZustaendigeBehoerdeUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

// ---------------------------------------------------------------------------
// Core streaming transform
// ---------------------------------------------------------------------------

/// Run the XML transform in a single streaming pass.
pub fn transform_xml(xml: &str, options: &TransformOptions) -> String {
    let mut rdr = NsReader::from_str(xml);
    rdr.config_mut().trim_text(false);
    rdr.config_mut().allow_unmatched_ends = true;

    let mut writer = Writer::new(Vec::<u8>::new());

    let has_zusatzinfo_updates = options.zusatzinformationen.is_some();
    let has_leser = options.leser.is_some();
    let has_autor = options.autor.is_some();

    let mut state = TransformState::default();
    let mut buf = Vec::new();

    loop {
        match rdr.read_resolved_event_into(&mut buf) {
            Ok((ns, Event::Start(e))) => {
                state.depth += 1;
                let lok = e.local_name().as_ref().to_vec();
                handle_start(
                    &mut state,
                    has_leser,
                    has_autor,
                    has_zusatzinfo_updates,
                    options,
                    &mut writer,
                    ns,
                    &lok,
                    &e,
                );
            }

            Ok((ns, Event::End(e))) => {
                let lok = e.local_name().as_ref().to_vec();
                handle_end(
                    &mut state,
                    has_leser,
                    has_autor,
                    has_zusatzinfo_updates,
                    options,
                    &mut writer,
                    ns,
                    &lok,
                    &e,
                );
                state.depth = state.depth.saturating_sub(1);
            }

            Ok((_ns, Event::Empty(e))) => {
                let lok = e.local_name().as_ref().to_vec();
                handle_empty(
                    &mut state,
                    has_leser,
                    has_autor,
                    has_zusatzinfo_updates,
                    &mut writer,
                    &lok,
                    &e,
                );
            }

            Ok((_ns, Event::Text(e))) => {
                let owned = Event::Text(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok((_ns, Event::CData(e))) => {
                let owned = Event::CData(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok((_ns, Event::Comment(e))) => {
                let owned = Event::Comment(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok((_ns, Event::PI(e))) => {
                let owned = Event::PI(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok((_ns, Event::Decl(e))) => {
                write_event(&mut writer, Event::Decl(e.clone().into_owned()));
            }

            Ok((_ns, Event::DocType(e))) => {
                write_event(&mut writer, Event::DocType(e.clone().into_owned()));
            }

            Ok((_ns, Event::Eof)) => break,

            Err(_) => break,
        }
        buf.clear();
    }

    String::from_utf8(writer.into_inner()).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Helpers: namespace check
// ---------------------------------------------------------------------------

fn ns_is_xwas(ns: &ResolveResult) -> bool {
    matches!(ns, ResolveResult::Bound(b) if *b == XWAS_NS)
}

fn ns_is_foreign(ns: &ResolveResult) -> bool {
    matches!(ns, ResolveResult::Bound(b) if *b != XWAS_NS)
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Default)]
struct TransformState {
    depth: usize,
    root_depth: usize,

    // nachrichtenkopf.g2g tracking
    nk_depth: usize,
    seen_leser: bool,
    seen_autor: bool,
    should_insert_leser: bool,
    should_insert_autor: bool,

    // Buffering for g2g element (leser/author) mutation
    in_g2g_element: bool,
    g2g_buf: Vec<Event<'static>>,
    g2g_element_name: Vec<u8>,

    // Measured indentation from surrounding XML
    nk_child_indent: Vec<u8>,
    root_child_indent: Vec<u8>,

    // XWasser prefix from root element (e.g. "xwas" or "xw")
    root_ns_prefix: Vec<u8>,

    // zusatzinformationen tracking
    zi_depth: usize,
    seen_zi: bool,
    in_zi: bool,
    zi_buf: Vec<Event<'static>>,
}

impl TransformState {
    fn g2g_child_indent(&self) -> &[u8] {
        if self.nk_child_indent.is_empty() {
            b"    "
        } else {
            &self.nk_child_indent
        }
    }

    fn root_child_indent(&self) -> &[u8] {
        if self.root_child_indent.is_empty() {
            b"  "
        } else {
            &self.root_child_indent
        }
    }

    fn zi_child_indent(&self) -> Vec<u8> {
        // One level deeper than root child indent (for zustaendigeBehoerde)
        [self.root_child_indent(), b"  "].concat()
    }

    fn zi_sub_indent(&self) -> Vec<u8> {
        // Two levels deeper (for kennung/name)
        [self.root_child_indent(), b"    "].concat()
    }
}

// ---------------------------------------------------------------------------
// Event handlers
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn handle_start(
    state: &mut TransformState,
    has_leser: bool,
    has_autor: bool,
    has_zusatzinfo_updates: bool,
    options: &TransformOptions,
    writer: &mut Writer<Vec<u8>>,
    ns: ResolveResult,
    lok: &[u8],
    e: &BytesStart<'_>,
) {
    if state.in_g2g_element {
        state.g2g_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }
    if state.in_zi {
        state.zi_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }

    if lok == b"vorgang.transportieren.2010" && state.root_depth == 0 {
        state.root_depth = state.depth;
        state.root_child_indent.clear();
        state.root_ns_prefix = prefix_of_qualified(e.name().as_ref());
    }

    if lok == b"nachrichtenkopf.g2g" && !ns_is_foreign(&ns) {
        state.nk_depth = state.depth;
        state.seen_leser = false;
        state.seen_autor = false;
        state.should_insert_leser = false;
        state.should_insert_autor = false;
        state.nk_child_indent.clear();
    }

    if lok == b"zusatzinformationen" && ns_is_xwas(&ns) {
        state.zi_depth = state.depth;
        state.seen_zi = true;
        if has_zusatzinfo_updates {
            state.in_zi = true;
            state.zi_buf.clear();
            state.zi_buf.push(Event::Start(e.clone().into_owned()));
            return;
        }
    }

    if state.should_insert_leser && lok != b"nachrichtenkopf.g2g" {
        if let Some(r) = &options.leser {
            insert_g2g_element(writer, "leser", r, state.g2g_child_indent());
        }
        state.should_insert_leser = false;
        if has_autor && !state.seen_autor {
            state.should_insert_autor = true;
        }
    }
    if state.should_insert_autor && lok != b"nachrichtenkopf.g2g" && lok != b"leser" {
        if let Some(r) = &options.autor {
            insert_g2g_element(writer, "autor", r, state.g2g_child_indent());
        }
        state.should_insert_autor = false;
    }

    if state.nk_depth > 0 && lok == b"leser" {
        state.seen_leser = true;
        if has_leser {
            start_g2g_element_buf(state, e, b"leser");
            return;
        }
    }

    if state.nk_depth > 0 && lok == b"autor" {
        state.seen_autor = true;
        if has_autor {
            start_g2g_element_buf(state, e, b"autor");
            return;
        }
    }

    write_event(writer, Event::Start(e.clone().into_owned()));
}

#[allow(clippy::too_many_arguments)]
fn handle_end(
    state: &mut TransformState,
    has_leser: bool,
    has_autor: bool,
    _has_zusatzinfo_updates: bool,
    options: &TransformOptions,
    writer: &mut Writer<Vec<u8>>,
    _ns: ResolveResult,
    lok: &[u8],
    e: &BytesEnd<'_>,
) {
    if state.in_g2g_element && lok == state.g2g_element_name {
        state.in_g2g_element = false;
        let r = if state.g2g_element_name == b"leser" {
            options.leser.as_ref()
        } else {
            options.autor.as_ref()
        };
        if let Some(r) = r
            && !state.g2g_buf.is_empty()
        {
            emit_mutated_g2g_element(writer, &state.g2g_buf, r);
            let end_name = std::str::from_utf8(&state.g2g_element_name).unwrap_or("");
            write_event(writer, Event::End(BytesEnd::new(end_name)));
            state.g2g_buf.clear();
            return;
        }
    }

    if state.in_zi && lok == b"zusatzinformationen" {
        state.in_zi = false;
        if !state.zi_buf.is_empty() {
            write_zusatzinfo_content(
                writer,
                options.zusatzinformationen,
                state.zi_child_indent(),
                &state.root_ns_prefix,
            );
            state.zi_buf.clear();
            return;
        }
    }

    if state.nk_depth > 0 && lok == b"identifikation.nachricht" && has_leser && !state.seen_leser {
        state.should_insert_leser = true;
    }
    if state.nk_depth > 0 && lok == b"leser" && has_autor && !state.seen_autor {
        state.should_insert_autor = true;
    }

    if state.in_g2g_element {
        state.g2g_buf.push(Event::End(e.clone().into_owned()));
        return;
    }
    if state.in_zi {
        state.zi_buf.push(Event::End(e.clone().into_owned()));
        return;
    }

    if state.nk_depth > 0 && lok == b"nachrichtenkopf.g2g" {
        if state.should_insert_leser {
            if let Some(r) = &options.leser {
                insert_g2g_element(writer, "leser", r, state.g2g_child_indent());
            }
            state.should_insert_leser = false;
            if has_autor && !state.seen_autor {
                state.should_insert_autor = true;
            }
        }
        if state.should_insert_autor {
            if let Some(r) = &options.autor {
                insert_g2g_element(writer, "autor", r, state.g2g_child_indent());
            }
            state.should_insert_autor = false;
        }
        state.nk_depth = 0;
    }

    if lok == b"zusatzinformationen" && !ns_is_foreign(&ResolveResult::Bound(XWAS_NS)) {
        state.zi_depth = 0;
    }

    if state.root_depth > 0
        && lok == b"vorgang.transportieren.2010"
        && state.depth == state.root_depth
    {
        let should_insert = options.zusatzinformationen.is_some_and(|a| !a.is_empty());
        if !state.seen_zi && should_insert {
            insert_zusatzinformationen_element(
                writer,
                options.zusatzinformationen.unwrap_or(&[]),
                if state.root_ns_prefix.is_empty() {
                    b"xwas"
                } else {
                    &state.root_ns_prefix
                },
                state.root_child_indent(),
            );
        }
        state.root_depth = 0;
    }

    write_event(writer, Event::End(e.clone().into_owned()));
}

fn handle_empty(
    state: &mut TransformState,
    has_leser: bool,
    has_autor: bool,
    has_zusatzinfo_updates: bool,
    writer: &mut Writer<Vec<u8>>,
    lok: &[u8],
    e: &BytesStart<'_>,
) {
    if state.in_g2g_element {
        state.g2g_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    if state.in_zi {
        state.zi_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    let _ = (has_leser, has_autor, has_zusatzinfo_updates, lok);
    write_event(writer, Event::Empty(e.clone().into_owned()));
}

fn handle_generic(state: &mut TransformState, writer: &mut Writer<Vec<u8>>, event: Event<'static>) {
    if state.in_g2g_element {
        state.g2g_buf.push(event);
        return;
    }
    if state.in_zi {
        state.zi_buf.push(event);
        return;
    }

    if let Event::Text(t) = &event {
        let bytes = t.as_ref();
        if !bytes.is_empty()
            && bytes
                .iter()
                .all(|b| *b == b' ' || *b == b'\n' || *b == b'\r' || *b == b'\t')
        {
            if state.nk_depth > 0
                && state.depth == state.nk_depth
                && state.nk_child_indent.is_empty()
            {
                state.nk_child_indent = bytes.to_vec();
            }
            if state.root_depth > 0
                && state.depth == state.root_depth
                && state.root_child_indent.is_empty()
            {
                state.root_child_indent = bytes.to_vec();
            }
        }
    }

    write_event(writer, event);
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn start_g2g_element_buf(state: &mut TransformState, e: &BytesStart<'_>, element_name: &[u8]) {
    state.in_g2g_element = true;
    state.g2g_buf.clear();
    state.g2g_element_name = element_name.to_vec();
    state.g2g_buf.push(Event::Start(e.clone().into_owned()));
}

fn write_event<W: std::io::Write>(writer: &mut Writer<W>, event: Event<'_>) {
    writer.write_event(event).ok();
}

fn write_text_bytes<W: std::io::Write>(writer: &mut Writer<W>, bytes: &[u8]) {
    writer
        .write_event(Event::Text(BytesText::new(
            std::str::from_utf8(bytes).unwrap_or(""),
        )))
        .ok();
}

/// Extract the prefix part of a qualified name (e.g. `"xwas:kennung"` -> `b"xwas"`).
fn prefix_of_qualified(name: &[u8]) -> Vec<u8> {
    if let Some(pos) = name.iter().position(|b| *b == b':') {
        name[..pos].to_vec()
    } else {
        Vec::new()
    }
}

/// Write `qname(prefix, local)` as a string-slice, trimming empty prefix case.
fn qn_str(prefix: &[u8], local: &str) -> String {
    if prefix.is_empty() {
        local.to_string()
    } else {
        format!(
            "{}{}{}",
            std::str::from_utf8(prefix).unwrap_or("xwas"),
            ":",
            local
        )
    }
}

/// Emit the entire `<zusatzinformationen>` content (start tag, entries, end tag)
/// using the provided authority updates.
fn write_zusatzinfo_content<W: std::io::Write>(
    writer: &mut Writer<W>,
    updates: Option<&[ZustaendigeBehoerdeUpdate]>,
    indent: Vec<u8>,
    prefix: &[u8],
) {
    let inner = [&indent[..], b"  "].concat();
    let inner2 = [&indent[..], b"    "].concat();

    // Write start tag from buffered first event (preserves prefix/attrs)
    // Use hardcoded xwas prefix for new content; original start tag already written before.

    let zi = qn_str(prefix, "zusatzinformationen");
    let zb = qn_str(prefix, "zustaendigeBehoerde");
    let kn = qn_str(prefix, "kennung");
    let nm = qn_str(prefix, "name");

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &indent);
    write_event(writer, Event::Start(BytesStart::new(&zi)));

    if let Some(entries) = updates {
        for auth in entries {
            write_text_bytes(writer, b"\n");
            write_text_bytes(writer, &inner);
            write_event(writer, Event::Start(BytesStart::new(&zb)));
            write_text_bytes(writer, b"\n");
            write_text_bytes(writer, &inner2);
            write_event(writer, Event::Start(BytesStart::new(&kn)));
            write_text_bytes(writer, auth.kennung.as_deref().unwrap_or("").as_bytes());
            write_event(writer, Event::End(BytesEnd::new(&kn)));
            write_text_bytes(writer, b"\n");
            write_text_bytes(writer, &inner2);
            write_event(writer, Event::Start(BytesStart::new(&nm)));
            write_text_bytes(writer, auth.name.as_deref().unwrap_or("").as_bytes());
            write_event(writer, Event::End(BytesEnd::new(&nm)));
            write_text_bytes(writer, b"\n");
            write_text_bytes(writer, &inner);
            write_event(writer, Event::End(BytesEnd::new(&zb)));
        }
    }

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &indent);
    write_event(writer, Event::End(BytesEnd::new(&zi)));
}

// ---------------------------------------------------------------------------
// Emit a mutated g2g element (leser/author) from buffered events
// ---------------------------------------------------------------------------

fn emit_mutated_g2g_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    buffered: &[Event<'static>],
    update: &ElementUpdate,
) {
    let mut inside_kennung = false;
    let mut inside_name = false;
    let has_kennung_update = update.kennung.is_some();
    let has_name_update = update.name.is_some();

    for ev in buffered {
        match ev {
            Event::Start(e) => {
                let en = e.local_name().as_ref().to_vec();
                if en == b"kennung" {
                    inside_kennung = true;
                    write_event(writer, Event::Start(e.clone()));
                    if let Some(ref k) = update.kennung {
                        write_text_bytes(writer, k.as_bytes());
                        continue;
                    }
                } else if en == b"name" {
                    inside_name = true;
                    write_event(writer, Event::Start(e.clone()));
                    if let Some(ref n) = update.name {
                        write_text_bytes(writer, n.as_bytes());
                        continue;
                    }
                } else {
                    write_event(writer, Event::Start(e.clone()));
                }
            }
            Event::End(e) => {
                let en = e.local_name().as_ref().to_vec();
                if en == b"kennung" {
                    inside_kennung = false;
                    if has_kennung_update {
                        write_event(writer, Event::End(e.clone()));
                        continue;
                    }
                } else if en == b"name" {
                    inside_name = false;
                    if has_name_update {
                        write_event(writer, Event::End(e.clone()));
                        continue;
                    }
                }
                write_event(writer, Event::End(e.clone()));
            }
            Event::Text(_) => {
                if inside_kennung && has_kennung_update {
                    continue;
                }
                if inside_name && has_name_update {
                    continue;
                }
                write_event(writer, ev.clone());
            }
            _ => {
                write_event(writer, ev.clone());
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Insert a new g2g element (leser/autor)
// ---------------------------------------------------------------------------

fn insert_g2g_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    element_name: &str,
    update: &ElementUpdate,
    indent: &[u8],
) {
    let kennung = update.kennung.as_deref().unwrap_or("");
    let name = update.name.as_deref().unwrap_or("");

    let sub = [indent, b"  "].concat();
    let subsub = [indent, b"    "].concat();

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, indent);
    write_event(writer, Event::Start(BytesStart::new(element_name)));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &sub);
    write_event(
        writer,
        Event::Start(BytesStart::from_content(
            r#"verzeichnisdienst listVersionID="""#,
            4,
        )),
    );
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &subsub);
    write_event(writer, Event::Start(BytesStart::new("code")));
    write_event(writer, Event::End(BytesEnd::new("code")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &sub);
    write_event(writer, Event::End(BytesEnd::new("verzeichnisdienst")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &sub);
    write_event(writer, Event::Start(BytesStart::new("kennung")));
    write_text_bytes(writer, kennung.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("kennung")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, &sub);
    write_event(writer, Event::Start(BytesStart::new("name")));
    write_text_bytes(writer, name.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("name")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, indent);
    write_event(writer, Event::End(BytesEnd::new(element_name)));
}

// ---------------------------------------------------------------------------
// Insert a new `<xwas:zusatzinformationen>` at the end of the root element
// ---------------------------------------------------------------------------

fn insert_zusatzinformationen_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    zusatzinformationen: &[ZustaendigeBehoerdeUpdate],
    prefix: &[u8],
    indent: &[u8],
) {
    let non_empty: Vec<&ZustaendigeBehoerdeUpdate> = zusatzinformationen
        .iter()
        .filter(|a| a.kennung.is_some() || a.name.is_some())
        .collect();

    if non_empty.is_empty() {
        return;
    }

    let sub = [indent, b"  "].concat();
    let subsub = [indent, b"    "].concat();

    let zi = qn_str(prefix, "zusatzinformationen");
    let zb = qn_str(prefix, "zustaendigeBehoerde");
    let kn = qn_str(prefix, "kennung");
    let nm = qn_str(prefix, "name");

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, indent);
    write_event(writer, Event::Start(BytesStart::new(&zi)));

    for auth in &non_empty {
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, &sub);
        write_event(writer, Event::Start(BytesStart::new(&zb)));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, &subsub);
        write_event(writer, Event::Start(BytesStart::new(&kn)));
        write_text_bytes(writer, auth.kennung.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new(&kn)));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, &subsub);
        write_event(writer, Event::Start(BytesStart::new(&nm)));
        write_text_bytes(writer, auth.name.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new(&nm)));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, &sub);
        write_event(writer, Event::End(BytesEnd::new(&zb)));
    }

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, indent);
    write_event(writer, Event::End(BytesEnd::new(&zi)));
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_xml() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<!-- root comment -->
<xwas:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="0.800.0" standard="XWasser" test="true" version="1.0.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>693c64d6-456f-4d14-abe7-fe9681c74aae</nachrichtenUUID>
    </identifikation.nachricht>
    <leser>
      <verzeichnisdienst listVersionID="">
        <code></code>
      </verzeichnisdienst>
      <kennung>psw:11113110</kennung>
      <name>Reader</name>
    </leser>
    <autor>
      <verzeichnisdienst listVersionID="">
        <code></code>
      </verzeichnisdienst>
      <kennung>psw:01003110</kennung>
      <name>Author</name>
    </autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</xwas:vorgangsID>
    </xwas:identifikationVorgang>
    <xwas:vorgangType>
      <xwas:pruefbericht>
        <xwas:pruefberichtUUID>id</xwas:pruefberichtUUID>
        <xwas:versionsnummer>1</xwas:versionsnummer>
        <xwas:auftragsnummer>order</xwas:auftragsnummer>
      </xwas:pruefbericht>
    </xwas:vorgangType>
  </xwas:vorgang>
  <ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#">
    <ds:SignedInfo>
      <ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"/>
      <ds:SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"/>
      <ds:Reference>
        <ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/>
        <ds:DigestValue/>
      </ds:Reference>
    </ds:SignedInfo>
    <ds:SignatureValue/>
    <ds:KeyInfo>
      <ds:KeyName/>
      <ds:X509Data>
        <ds:X509Certificate/>
      </ds:X509Data>
    </ds:KeyInfo>
  </ds:Signature>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_with_zi() -> String {
        let base = load_quality_report();
        transform_xml(
            &base,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Existing Authority".into()),
                }]),
                ..Default::default()
            },
        )
    }

    fn sample_xml_no_leser_no_autor() -> String {
        let base = load_quality_report();
        let mut lines: Vec<&str> = base.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].contains("<leser>") || lines[i].contains("<autor>") {
                let tag = if lines[i].contains("<leser>") {
                    "</leser>"
                } else {
                    "</autor>"
                };
                lines.remove(i);
                while i < lines.len() && !lines[i].contains(tag) {
                    lines.remove(i);
                }
                if i < lines.len() {
                    lines.remove(i);
                }
            } else {
                i += 1;
            }
        }
        lines.join("\n")
    }

    fn sample_xml_no_zi() -> String {
        load_quality_report()
    }

    fn sample_xml_custom_prefix() -> String {
        let base = sample_xml_with_zi();
        let result = base
            .replace("xmlns:xwas=", "xmlns:xw=")
            .replace("xwas:", "xw:")
            .replace("xmlns:xw:Signature", "xmlns:ds:Signature")
            .replace("xw:Signature", "ds:Signature");
        let parsed =
            raxb::de::from_str::<crate::model::transport::VorgangTransportieren2010>(&result);
        assert!(
            parsed.is_ok(),
            "custom prefix fixture must be raxb-parseable"
        );
        result
    }

    fn load_quality_report() -> String {
        let dir = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => panic!("cannot get current dir: {e}"),
        };
        let path = dir.join("tests/quality_report_minimal.xml");
        match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => panic!("cannot read {:?}: {e}", path),
        }
    }

    fn assert_raxb_roundtrip(xml: &str) -> crate::model::transport::VorgangTransportieren2010 {
        match raxb::de::from_str(xml) {
            Ok(p) => p,
            Err(e) => panic!("raxb round-trip failed: {e:?}"),
        }
    }

    fn sample_xml_two_space_indent() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="0.800.0" standard="XWasser" test="true" version="1.0.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>id</nachrichtenUUID>
    </identifikation.nachricht>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>id</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    // ---- tests ----

    #[test]
    fn test_noop_is_byte_identical() {
        let xml = sample_xml();
        let result = transform_xml(&xml, &TransformOptions::default());
        assert_eq!(result, xml);
    }

    #[test]
    fn test_leser_mutation() {
        let xml = load_quality_report();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:99999999".into()),
                    name: Some("NewReader".into()),
                }),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:99999999");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "NewReader");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Author");
    }

    #[test]
    fn test_autor_mutation() {
        let xml = load_quality_report();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                autor: Some(ElementUpdate {
                    kennung: Some("psw:autor123".into()),
                    name: Some("Updated Autor".into()),
                }),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:11113110");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Reader");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:autor123");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Updated Autor");
    }

    #[test]
    fn test_leser_and_autor_mutation() {
        let xml = load_quality_report();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:leser1".into()),
                    name: Some("Leser1".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:autor1".into()),
                    name: Some("Autor1".into()),
                }),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:leser1");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Leser1");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:autor1");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.name, "Autor1");
    }

    #[test]
    fn test_zusatzinfo_full_replacement() {
        let base = load_quality_report();
        let with_zi = transform_xml(
            &base,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Original".into()),
                }]),
                ..Default::default()
            },
        );
        assert!(with_zi.contains("xwas:zusatzinformationen"));

        let result = transform_xml(
            &with_zi,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Replaced".into()),
                }]),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert!(
            parsed.zusatzinformationen.is_some(),
            "zusatzinfo must be present"
        );
    }

    #[test]
    fn test_zusatzinfo_replace_with_multiple_entries() {
        let base = load_quality_report();
        let with_zi = transform_xml(
            &base,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("original".into()),
                    name: Some("Original".into()),
                }]),
                ..Default::default()
            },
        );
        // Replace with multiple entries
        let result = transform_xml(
            &with_zi,
            &TransformOptions {
                zusatzinformationen: Some(&[
                    ZustaendigeBehoerdeUpdate {
                        kennung: Some("new-1".into()),
                        name: Some("First".into()),
                    },
                    ZustaendigeBehoerdeUpdate {
                        kennung: Some("new-2".into()),
                        name: Some("Second".into()),
                    },
                ]),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert!(parsed.zusatzinformationen.is_some());
    }

    #[test]
    fn test_zusatzinfo_replace_with_empty() {
        let base = load_quality_report();
        let with_zi = transform_xml(
            &base,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth".into()),
                    name: Some("Auth".into()),
                }]),
                ..Default::default()
            },
        );
        // Replace with empty content
        let result = transform_xml(
            &with_zi,
            &TransformOptions {
                zusatzinformationen: Some(&[]),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert!(
            parsed.zusatzinformationen.is_some(),
            "zusatzinfo element should remain"
        );
    }

    #[test]
    fn test_insert_leser() {
        let xml = sample_xml_no_leser_no_autor();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                ..Default::default()
            },
        );
        let leser_pos = match result.find("psw:inserted") {
            Some(p) => p,
            None => panic!("psw:inserted not found"),
        };
        let dvdv_pos = match result.find("dvdvDienstkennung") {
            Some(p) => p,
            None => panic!("dvdv not found"),
        };
        assert!(leser_pos < dvdv_pos);
    }

    #[test]
    fn test_insert_autor() {
        let xml = sample_xml_no_leser_no_autor();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:l".into()),
                    name: Some("Leser".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:a".into()),
                    name: Some("Autor".into()),
                }),
                ..Default::default()
            },
        );
        let leser_pos = match result.find("psw:l") {
            Some(p) => p,
            None => panic!("psw:l not found"),
        };
        let autor_pos = match result.find("psw:a") {
            Some(p) => p,
            None => panic!("psw:a not found"),
        };
        assert!(leser_pos < autor_pos);
    }

    #[test]
    fn test_insert_zusatzinformationen() {
        let xml = load_quality_report();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("new-auth".into()),
                    name: Some("New Authority".into()),
                }]),
                ..Default::default()
            },
        );
        let parsed = assert_raxb_roundtrip(&result);
        assert!(
            parsed.zusatzinformationen.is_some(),
            "zusatzinfo must be present"
        );
    }

    #[test]
    fn test_custom_prefix_raxb_roundtrip() {
        // sample_xml_custom_prefix uses "xw:" prefix; verify transform preserves
        // raxb parseability and field values
        let xml = sample_xml_custom_prefix();
        assert!(xml.contains("xw:"), "fixture must use custom xw: prefix");

        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:custom".into()),
                    name: Some("Custom".into()),
                }),
                zusatzinformationen: Some(&[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Updated".into()),
                }]),
                ..Default::default()
            },
        );

        // raxb must parse output directly (no prefix normalization)
        let parsed = assert_raxb_roundtrip(&result);
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:custom");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Custom");
        assert!(
            parsed.zusatzinformationen.is_some(),
            "zusatzinfo must be present"
        );
        // Verify output uses xw: prefix throughout (not xwas:)
        assert!(
            result.contains("xw:zusatzinformationen"),
            "should use xw: prefix for zusatzinfo"
        );
        assert!(
            result.contains("xw:zustaendigeBehoerde"),
            "should use xw: prefix for zustaendigeBehoerde"
        );
        assert!(
            !result.contains("xwas:zusatzinformationen"),
            "output should NOT use xwas: prefix"
        );
        // Verify authority content values via XML text
        assert!(
            result.contains("xw:kennung>auth-001"),
            "authority kennung value"
        );
        assert!(result.contains("xw:name>Updated"), "authority name value");
    }

    #[test]
    fn test_raxb_roundtrip_noop() {
        assert_raxb_roundtrip(&load_quality_report());
    }

    #[test]
    fn test_comment_preservation() {
        let result = transform_xml(&sample_xml(), &TransformOptions::default());
        assert!(result.contains("<!-- root comment -->"));
    }

    #[test]
    fn test_whitespace_preservation() {
        let result = transform_xml(&sample_xml(), &TransformOptions::default());
        assert!(result.contains("  <nachrichtenkopf.g2g>"));
        assert!(result.contains("    <identifikation.nachricht>"));
    }

    #[test]
    fn test_signature_roundtrip() {
        let result = transform_xml(&sample_xml(), &TransformOptions::default());
        assert!(result.contains("ds:Signature"));
        assert!(result.contains("ds:SignedInfo"));
        assert!(result.contains("ds:DigestValue"));
        assert!(result.contains("ds:SignatureValue"));
        assert!(result.contains("ds:X509Data"));
    }

    #[test]
    fn test_raxb_roundtrip_quality_report() {
        let dir = match std::env::current_dir() {
            Ok(d) => d,
            Err(e) => panic!("{e}"),
        };
        let path = dir.join("tests/quality_report_minimal.xml");
        let xml = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => panic!("{e}"),
        };
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:mutated".into()),
                    name: Some("Mutated Reader".into()),
                }),
                ..Default::default()
            },
        );
        let parsed: crate::model::transport::VorgangTransportieren2010 =
            match raxb::de::from_str(&result) {
                Ok(p) => p,
                Err(e) => panic!("raxb round-trip failed: {e:?}"),
            };
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:mutated");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Mutated Reader");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110");
    }
}
