//! Streaming XML transform for XWasser messages using quick-xml.
//!
//! Mutates `<leser>` and `<autor>` inside `nachrichtenkopf.g2g` and
//! `<zustaendigeBehoerde>` elements inside `<zusatzinformationen>` in a single
//! pass, preserving all comments, processing instructions, whitespace text
//! nodes, and attribute order.
//!
//! Element matching uses resolved namespace URIs, not prefixes — so the
//! transform works regardless of which prefix the source XML uses for the
//! XWasser namespace.
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
    /// Zero or more updates for `<zustaendigeBehoerde>` elements inside
    /// `<zusatzinformationen>`. Only those with a matching `<kennung>` in
    /// the source are replaced; unmatched entries are ignored unless
    /// `<zusatzinformationen>` is entirely missing, in which case all
    /// entries are inserted.
    pub zusatzinformationen: &'a [ZustaendigeBehoerdeUpdate],
}

/// Update parameters for an element inside `nachrichtenkopf.g2g`
/// that has `<kennung>` and `<name>` children (e.g. `<leser>`, `<autor>`).
#[derive(Debug, Clone, Default)]
pub struct ElementUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

/// Update parameters for a single `<zustaendigeBehoerde>` element inside
/// `<zusatzinformationen>`. `kennung` is used for matching existing elements;
/// `name` is updated for the matched/replaced element.
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

    let has_zusatzinfo_updates = options
        .zusatzinformationen
        .iter()
        .any(|a| a.kennung.is_some() || a.name.is_some());
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

/// Returns `true` when the resolved namespace is the XWasser namespace.
fn ns_is_xwas(ns: &ResolveResult) -> bool {
    matches!(ns, ResolveResult::Bound(b) if *b == XWAS_NS)
}

/// Returns `true` when the resolved namespace is explicitly *not* the XWasser
/// namespace.
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

    // zusatzinformationen tracking
    zi_depth: usize,
    seen_zi: bool,

    // zustaendigeBehoerde buffering
    in_zb: bool,
    zb_buf: Vec<Event<'static>>,
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
    // General buffering for nested content inside g2g elements / zustaendigeBehoerde
    if state.in_g2g_element {
        state.g2g_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }
    if state.in_zb {
        state.zb_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }

    // Track root element (unqualified)
    if lok == b"vorgang.transportieren.2010" && state.root_depth == 0 {
        state.root_depth = state.depth;
    }

    // Track parent depth — nachrichtenkopf.g2g is unqualified
    if lok == b"nachrichtenkopf.g2g" && !ns_is_foreign(&ns) {
        state.nk_depth = state.depth;
        state.seen_leser = false;
        state.seen_autor = false;
        state.should_insert_leser = false;
        state.should_insert_autor = false;
    }

    // Track zusatzinformationen by namespace + local name
    if lok == b"zusatzinformationen" && ns_is_xwas(&ns) {
        state.zi_depth = state.depth;
        state.seen_zi = true;
    }

    // Insert missing leser before autor (second-child position)
    if state.should_insert_leser && lok != b"nachrichtenkopf.g2g" {
        if let Some(r) = &options.leser {
            insert_g2g_element(writer, "leser", r, 2);
        }
        state.should_insert_leser = false;
        if has_autor && !state.seen_autor {
            state.should_insert_autor = true;
        }
    }
    // Insert missing autor after leser or before any later child
    if state.should_insert_autor && lok != b"nachrichtenkopf.g2g" && lok != b"leser" {
        if let Some(r) = &options.autor {
            insert_g2g_element(writer, "autor", r, 2);
        }
        state.should_insert_autor = false;
    }

    // --- leser element (unqualified) ---
    if state.nk_depth > 0 && lok == b"leser" {
        state.seen_leser = true;
        if has_leser {
            start_g2g_element_buf(state, e, b"leser");
            return;
        }
    }

    // --- autor element (unqualified) ---
    if state.nk_depth > 0 && lok == b"autor" {
        state.seen_autor = true;
        if has_autor {
            start_g2g_element_buf(state, e, b"autor");
            return;
        }
    }

    // --- zustaendigeBehoerde (xwas namespace) ---
    if state.zi_depth > 0
        && lok == b"zustaendigeBehoerde"
        && ns_is_xwas(&ns)
        && has_zusatzinfo_updates
    {
        state.in_zb = true;
        state.zb_buf.clear();
        state.zb_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }

    write_event(writer, Event::Start(e.clone().into_owned()));
}

#[allow(clippy::too_many_arguments)]
fn handle_end(
    state: &mut TransformState,
    has_leser: bool,
    has_autor: bool,
    has_zusatzinfo_updates: bool,
    options: &TransformOptions,
    writer: &mut Writer<Vec<u8>>,
    _ns: ResolveResult,
    lok: &[u8],
    e: &BytesEnd<'_>,
) {
    // --- closing g2g element (leser/author) ---
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

    // --- closing zustaendigeBehoerde ---
    if state.in_zb && lok == b"zustaendigeBehoerde" {
        state.in_zb = false;
        if !state.zb_buf.is_empty() {
            emit_mutated_zustaendige_behoerde(writer, &state.zb_buf, options.zusatzinformationen);
            state.zb_buf.clear();
            return;
        }
    }

    // Schedule leser/autor insertion after identifikation.nachricht (unqualified)
    if state.nk_depth > 0 && lok == b"identifikation.nachricht" {
        if has_leser && !state.seen_leser {
            state.should_insert_leser = true;
        }
        if has_autor && !state.seen_autor && !state.should_insert_leser {
            // autor scheduled only if leser already exists (handled via leser end)
        }
    }
    // Schedule autor insertion after leser end
    if state.nk_depth > 0 && lok == b"leser" && has_autor && !state.seen_autor {
        state.should_insert_autor = true;
    }

    // Buffer nested end events for g2g elements
    if state.in_g2g_element {
        state.g2g_buf.push(Event::End(e.clone().into_owned()));
        return;
    }
    if state.in_zb {
        state.zb_buf.push(Event::End(e.clone().into_owned()));
        return;
    }

    // --- closing nachrichtenkopf.g2g (insert missing elements) ---
    if state.nk_depth > 0 && lok == b"nachrichtenkopf.g2g" {
        if state.should_insert_leser {
            if let Some(r) = &options.leser {
                insert_g2g_element(writer, "leser", r, 2);
            }
            state.should_insert_leser = false;
            if has_autor && !state.seen_autor {
                state.should_insert_autor = true;
            }
        }
        if state.should_insert_autor {
            if let Some(r) = &options.autor {
                insert_g2g_element(writer, "autor", r, 2);
            }
            state.should_insert_autor = false;
        }
        state.nk_depth = 0;
    }

    // --- closing zusatzinformationen ---
    if lok == b"zusatzinformationen" && !ns_is_foreign(&ResolveResult::Bound(XWAS_NS)) {
        state.zi_depth = 0;
    }

    // --- closing root (insert zusatzinformationen if still missing) ---
    if state.root_depth > 0
        && lok == b"vorgang.transportieren.2010"
        && state.depth == state.root_depth
    {
        if !state.seen_zi && has_zusatzinfo_updates {
            insert_zusatzinformationen_element(writer, options.zusatzinformationen);
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
    if state.in_zb {
        state.zb_buf.push(Event::Empty(e.clone().into_owned()));
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
    if state.in_zb {
        state.zb_buf.push(event);
        return;
    }
    write_event(writer, event);
}

// ---------------------------------------------------------------------------
// Helper: start buffering a g2g element
// ---------------------------------------------------------------------------

fn start_g2g_element_buf(state: &mut TransformState, e: &BytesStart<'_>, element_name: &[u8]) {
    state.in_g2g_element = true;
    state.g2g_buf.clear();
    state.g2g_element_name = element_name.to_vec();
    state.g2g_buf.push(Event::Start(e.clone().into_owned()));
}

// ---------------------------------------------------------------------------
// Writer helpers
// ---------------------------------------------------------------------------

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
                    if has_kennung_update {
                        write_text_bytes(writer, update.kennung.as_deref().unwrap().as_bytes());
                        continue;
                    }
                } else if en == b"name" {
                    inside_name = true;
                    write_event(writer, Event::Start(e.clone()));
                    if has_name_update {
                        write_text_bytes(writer, update.name.as_deref().unwrap().as_bytes());
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
// Emit mutated <zustaendigeBehoerde> from buffered events
// ---------------------------------------------------------------------------

fn emit_mutated_zustaendige_behoerde<W: std::io::Write>(
    writer: &mut Writer<W>,
    buffered: &[Event<'static>],
    zusatzinformationen: &[ZustaendigeBehoerdeUpdate],
) {
    let current_kennung = extract_kennung_from_zb_buf(buffered);

    if let Some(ref cur) = current_kennung
        && let Some(matching) = zusatzinformationen
            .iter()
            .find(|a| a.kennung.as_deref() == Some(cur))
    {
        if let Some(Event::Start(first)) = buffered.first() {
            write_event(writer, Event::Start(first.clone()));
        }
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"        ");
        write_event(writer, Event::Start(BytesStart::new("xwas:kennung")));
        write_text_bytes(writer, matching.kennung.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new("xwas:kennung")));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"        ");
        write_event(writer, Event::Start(BytesStart::new("xwas:name")));
        write_text_bytes(writer, matching.name.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new("xwas:name")));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"      ");
        write_event(
            writer,
            Event::End(BytesEnd::new("xwas:zustaendigeBehoerde")),
        );
        return;
    }

    for ev in buffered {
        write_event(writer, ev.clone());
    }
}

fn extract_kennung_from_zb_buf(buffered: &[Event<'static>]) -> Option<String> {
    let mut in_kennung = false;
    for ev in buffered {
        match ev {
            Event::Start(e) if e.local_name().as_ref() == b"kennung" => in_kennung = true,
            Event::Text(t) if in_kennung => {
                return Some(String::from_utf8_lossy(t.as_ref()).to_string());
            }
            Event::End(e) if e.local_name().as_ref() == b"kennung" => in_kennung = false,
            _ => {}
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Insert a new g2g element (leser/autor) as Nth child
// ---------------------------------------------------------------------------

fn insert_g2g_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    element_name: &str,
    update: &ElementUpdate,
    _level: usize,
) {
    let kennung = update.kennung.as_deref().unwrap_or("");
    let name = update.name.as_deref().unwrap_or("");

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"    ");
    write_event(writer, Event::Start(BytesStart::new(element_name)));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"      ");
    write_event(
        writer,
        Event::Start(BytesStart::from_content(
            r#"verzeichnisdienst listVersionID="""#,
            4,
        )),
    );
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"        ");
    write_event(writer, Event::Start(BytesStart::new("code")));
    write_event(writer, Event::End(BytesEnd::new("code")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"      ");
    write_event(writer, Event::End(BytesEnd::new("verzeichnisdienst")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"      ");
    write_event(writer, Event::Start(BytesStart::new("kennung")));
    write_text_bytes(writer, kennung.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("kennung")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"      ");
    write_event(writer, Event::Start(BytesStart::new("name")));
    write_text_bytes(writer, name.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("name")));
    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"    ");
    write_event(writer, Event::End(BytesEnd::new(element_name)));
}

// ---------------------------------------------------------------------------
// Insert a new <xwas:zusatzinformationen> at the end of the root element
// (before </xwas:vorgang.transportieren.2010>)
// ---------------------------------------------------------------------------

fn insert_zusatzinformationen_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    zusatzinformationen: &[ZustaendigeBehoerdeUpdate],
) {
    let non_empty: Vec<&ZustaendigeBehoerdeUpdate> = zusatzinformationen
        .iter()
        .filter(|a| a.kennung.is_some() || a.name.is_some())
        .collect();

    if non_empty.is_empty() {
        return;
    }

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"  ");
    write_event(
        writer,
        Event::Start(BytesStart::new("xwas:zusatzinformationen")),
    );

    for auth in &non_empty {
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"    ");
        write_event(
            writer,
            Event::Start(BytesStart::new("xwas:zustaendigeBehoerde")),
        );
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"      ");
        write_event(writer, Event::Start(BytesStart::new("xwas:kennung")));
        write_text_bytes(writer, auth.kennung.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new("xwas:kennung")));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"      ");
        write_event(writer, Event::Start(BytesStart::new("xwas:name")));
        write_text_bytes(writer, auth.name.as_deref().unwrap_or("").as_bytes());
        write_event(writer, Event::End(BytesEnd::new("xwas:name")));
        write_text_bytes(writer, b"\n");
        write_text_bytes(writer, b"    ");
        write_event(
            writer,
            Event::End(BytesEnd::new("xwas:zustaendigeBehoerde")),
        );
    }

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"  ");
    write_event(
        writer,
        Event::End(BytesEnd::new("xwas:zusatzinformationen")),
    );
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
      <nachrichtentyp listURI="urn:xoev-de:xwasser:codeliste:nachrichtentyp" listVersionID="1">
        <code>2010</code>
      </nachrichtentyp>
      <erstellungszeitpunkt>2024-05-28T09:00:00</erstellungszeitpunkt>
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
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</xwas:vorgangsID>
    </xwas:identifikationVorgang>
    <xwas:vorgangType>
      <xwas:pruefbericht id="ID5e08e073-4e06-438d-9444-1275f6cbf061">
        <xwas:pruefberichtUUID>5e08e073-4e06-438d-9444-1275f6cbf061</xwas:pruefberichtUUID>
      </xwas:pruefbericht>
    </xwas:vorgangType>
  </xwas:vorgang>
  <ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#">
    <ds:SignedInfo>
      <ds:CanonicalizationMethod Algorithm="http://www.w3.org/2001/10/xml-exc-c14n#"/>
      <ds:SignatureMethod Algorithm="http://www.w3.org/2001/04/xmldsig-more#rsa-sha256"/>
      <ds:Reference>
        <ds:DigestMethod Algorithm="http://www.w3.org/2000/09/xmldsig#sha1"/>
        <ds:DigestValue></ds:DigestValue>
      </ds:Reference>
    </ds:SignedInfo>
    <ds:SignatureValue></ds:SignatureValue>
    <ds:KeyInfo>
      <ds:KeyName/>
      <ds:X509Data>
        <ds:X509Certificate></ds:X509Certificate>
      </ds:X509Data>
    </ds:KeyInfo>
  </ds:Signature>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_no_leser_no_autor() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="0.800.0" standard="XWasser" test="true" version="1.0.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>693c64d6-456f-4d14-abe7-fe9681c74aae</nachrichtenUUID>
    </identifikation.nachricht>
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang>
    <xwas:identifikationVorgang>
      <xwas:vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</xwas:vorgangsID>
    </xwas:identifikationVorgang>
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_no_zi() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
  </xwas:vorgang>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_with_zi() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
  </xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerde>
      <xwas:kennung>auth-001</xwas:kennung>
      <xwas:name>Existing Authority</xwas:name>
    </xwas:zustaendigeBehoerde>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_custom_prefix() -> String {
        // Uses a different prefix "xw" bound to the same XWasser namespace
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xw:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" xmlns:xw="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="0.800.0" standard="XWasser" test="true" version="1.0.0">
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
  <xw:vorgang>
    <xw:identifikationVorgang>
      <xw:vorgangsID>5e08e073-4e06-438d-9444-1275f6cbf061</xw:vorgangsID>
    </xw:identifikationVorgang>
  </xw:vorgang>
  <xw:zusatzinformationen>
    <xw:zustaendigeBehoerde>
      <xw:kennung>auth-001</xw:kennung>
      <xw:name>Existing Authority</xw:name>
    </xw:zustaendigeBehoerde>
  </xw:zusatzinformationen>
</xw:vorgang.transportieren.2010>"#
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
        let xml = sample_xml();
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
        assert!(result.contains("<kennung>psw:99999999</kennung>"));
        assert!(result.contains("<name>NewReader</name>"));
        assert!(result.contains("<kennung>psw:01003110</kennung>"));
        assert!(result.contains("<name>Author</name>"));
    }

    #[test]
    fn test_autor_mutation() {
        let xml = sample_xml();
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
        assert!(result.contains("<kennung>psw:11113110</kennung>"));
        assert!(result.contains("<name>Reader</name>"));
        assert!(result.contains("<kennung>psw:autor123</kennung>"));
        assert!(result.contains("<name>Updated Autor</name>"));
    }

    #[test]
    fn test_leser_and_autor_mutation() {
        let xml = sample_xml();
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
        assert!(result.contains("<kennung>psw:leser1</kennung>"));
        assert!(result.contains("<name>Leser1</name>"));
        assert!(result.contains("<kennung>psw:autor1</kennung>"));
        assert!(result.contains("<name>Autor1</name>"));
    }

    #[test]
    fn test_zusatzinformationen_mutation() {
        let xml = sample_xml_with_zi();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                zusatzinformationen: &[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Updated Authority".into()),
                }],
                ..Default::default()
            },
        );
        assert!(result.contains("<xwas:kennung>auth-001</xwas:kennung>"));
        assert!(result.contains("<xwas:name>Updated Authority</xwas:name>"));
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
        assert!(result.contains("<kennung>psw:inserted</kennung>"));
        assert!(result.contains("<name>Inserted Reader</name>"));
        let leser_pos = result.find("psw:inserted").unwrap();
        let dvdv_pos = result.find("dvdvDienstkennung").unwrap();
        assert!(
            leser_pos < dvdv_pos,
            "leser must appear before dvdvDienstkennung"
        );
    }

    #[test]
    fn test_insert_autor() {
        let xml = sample_xml_no_leser_no_autor();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:inserted".into()),
                    name: Some("Inserted Reader".into()),
                }),
                autor: Some(ElementUpdate {
                    kennung: Some("psw:newautor".into()),
                    name: Some("New Autor".into()),
                }),
                ..Default::default()
            },
        );
        assert!(result.contains("<kennung>psw:inserted</kennung>"));
        assert!(result.contains("<kennung>psw:newautor</kennung>"));
        assert!(result.contains("<name>New Autor</name>"));
        let leser_pos = result.find("psw:inserted").unwrap();
        let autor_pos = result.find("psw:newautor").unwrap();
        assert!(leser_pos < autor_pos, "leser must come before autor");
    }

    #[test]
    fn test_insert_zusatzinformationen() {
        let xml = sample_xml_no_zi();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                zusatzinformationen: &[ZustaendigeBehoerdeUpdate {
                    kennung: Some("new-auth".into()),
                    name: Some("New Authority".into()),
                }],
                ..Default::default()
            },
        );
        assert!(result.contains("xwas:zusatzinformationen"));
        assert!(result.contains("<xwas:kennung>new-auth</xwas:kennung>"));
        assert!(result.contains("<xwas:name>New Authority</xwas:name>"));
    }

    #[test]
    fn test_custom_namespace_prefix() {
        // Uses "xw:" prefix — the namespace-aware NsReader resolves URI so
        // matching works regardless of prefix
        let xml = sample_xml_custom_prefix();
        let result = transform_xml(
            &xml,
            &TransformOptions {
                leser: Some(ElementUpdate {
                    kennung: Some("psw:custom".into()),
                    name: Some("Custom".into()),
                }),
                zusatzinformationen: &[ZustaendigeBehoerdeUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Updated via custom prefix".into()),
                }],
                ..Default::default()
            },
        );
        // leser (unqualified) works fine
        assert!(result.contains("<kennung>psw:custom</kennung>"));
        assert!(result.contains("<name>Custom</name>"));
        // Authority replacement works because namespace resolves to XWAS_NS
        assert!(result.contains("xwas:kennung>auth-001"));
        assert!(result.contains("xwas:name>Updated via custom prefix"));
    }

    #[test]
    fn test_comment_preservation() {
        let xml = sample_xml();
        let result = transform_xml(&xml, &TransformOptions::default());
        assert!(result.contains("<!-- root comment -->"));
    }

    #[test]
    fn test_whitespace_preservation() {
        let xml = sample_xml();
        let result = transform_xml(&xml, &TransformOptions::default());
        assert!(result.contains("  <nachrichtenkopf.g2g>"));
        assert!(result.contains("    <identifikation.nachricht>"));
    }

    #[test]
    fn test_signature_roundtrip() {
        let xml = sample_xml();
        let result = transform_xml(&xml, &TransformOptions::default());
        assert!(result.contains("ds:Signature"));
        assert!(result.contains("ds:SignedInfo"));
        assert!(result.contains("ds:DigestValue"));
        assert!(result.contains("ds:SignatureValue"));
        assert!(result.contains("ds:X509Data"));
    }

    #[test]
    fn test_raxb_roundtrip_quality_report() {
        let path = std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_minimal.xml");
        let xml = std::fs::read_to_string(path).unwrap();

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

        let parsed: Result<crate::model::transport::VorgangTransportieren2010, _> =
            raxb::de::from_str(&result);
        assert!(parsed.is_ok(), "raxb round-trip failed: {:?}", parsed.err());
        let parsed = parsed.unwrap();
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.kennung, "psw:mutated");
        assert_eq!(parsed.nachrichtenkopf_g2g.leser.name, "Mutated Reader");
        assert_eq!(parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110");
    }
}
