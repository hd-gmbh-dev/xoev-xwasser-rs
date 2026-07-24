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
///
/// The structure mirrors the XML document layout: the header group
/// (`nachrichtenkopf.g2g`) holds fields that land inside `<nachrichtenkopf.g2g>`,
/// and the `zusatzinformationen` group holds fields that land inside
/// `<zusatzinformationen>`. Keeping the groups separate makes the field
/// destination explicit and lets the `zusatzinformationen` group grow
/// (e.g. a future `kommentar` update) without touching the header.
#[derive(Debug, Clone, Default)]
pub struct TransformOptions<'a> {
    /// Updates targeting `<nachrichtenkopf.g2g>` (`leser`, `autor`,
    /// `nachrichtenUUID`).
    pub nachrichtenkopf_g2g: Option<NachrichtenkopfG2gOptions<'a>>,
    /// Updates targeting `<zusatzinformationen>`.
    pub zusatzinformationen: Option<ZusatzinformationenOptions<'a>>,
}

/// Options for the `<nachrichtenkopf.g2g>` header block.
#[derive(Debug, Clone, Default)]
pub struct NachrichtenkopfG2gOptions<'a> {
    /// Optional update for the `<leser>` element.
    pub leser: Option<ElementUpdate>,
    /// Optional update for the `<autor>` element.
    pub autor: Option<ElementUpdate>,
    /// Optional update for the `<nachrichtenUUID>` element.
    /// - `None`: no change / do not insert if missing.
    /// - `Some(uuid)`: replace existing or insert if missing.
    pub nachrichten_uuid: Option<&'a str>,
}

/// Options for the `<zusatzinformationen>` extra-info block.
///
/// Currently only the `<zustaendigeBehoerdeID>` entries are supported.
/// The struct is laid out so further fields (e.g. `kommentar`,
/// `wasserversorgungsgebiet_id`) can be added later without reshaping the
/// top-level [`TransformOptions`].
#[derive(Debug, Clone, Default)]
pub struct ZusatzinformationenOptions<'a> {
    /// Replacement `<zustaendigeBehoerdeID>` entries.
    /// - `None`: no change / do not insert if missing.
    /// - `Some(&[])`: replace existing with empty block.
    /// - `Some(&["id1", "id2"])`: replace full content with given entries.
    pub zustaendige_behoerde_id: Option<&'a [String]>,
}

/// Update parameters for an element inside `nachrichtenkopf.g2g`
/// that has `<kennung>` and `<name>` children (e.g. `<leser>`, `<autor>`).
#[derive(Debug, Clone, Default)]
pub struct ElementUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

/// Flattened view of [`TransformOptions`] used internally by the handlers so
/// they stay decoupled from the public grouped struct shape. All references
/// borrow from the original `TransformOptions`.
struct ResolvedOptions<'a> {
    leser: Option<&'a ElementUpdate>,
    autor: Option<&'a ElementUpdate>,
    nachrichten_uuid: Option<&'a str>,
    zusatzinfo_ids: Option<&'a [String]>,
}

fn resolve_options<'a>(options: &'a TransformOptions<'a>) -> ResolvedOptions<'a> {
    let header = options.nachrichtenkopf_g2g.as_ref();
    ResolvedOptions {
        leser: header.and_then(|h| h.leser.as_ref()),
        autor: header.and_then(|h| h.autor.as_ref()),
        nachrichten_uuid: header.and_then(|h| h.nachrichten_uuid),
        zusatzinfo_ids: options
            .zusatzinformationen
            .as_ref()
            .and_then(|z| z.zustaendige_behoerde_id),
    }
}

// ---------------------------------------------------------------------------
// Core streaming transform
// ---------------------------------------------------------------------------

/// Run the XML transform in a single streaming pass with the internal options struct.
pub fn transform_vorgang_transportieren_2010(xml: &str, options: &TransformOptions) -> String {
    transform_vorgang_transportieren_2010_impl(xml, options)
}

/// Convenience wrapper accepting `<zustaendigeBehoerdeID>` strings.
pub fn transform_vorgang_transportieren_2010_with_ids(
    xml: &str,
    leser: Option<&ElementUpdate>,
    autor: Option<&ElementUpdate>,
    zusatzinfo_ids: Option<&[String]>,
) -> String {
    transform_vorgang_transportieren_2010_impl(
        xml,
        &TransformOptions {
            nachrichtenkopf_g2g: Some(NachrichtenkopfG2gOptions {
                leser: leser.cloned(),
                autor: autor.cloned(),
                nachrichten_uuid: None,
            }),
            zusatzinformationen: zusatzinfo_ids.map(|ids| ZusatzinformationenOptions {
                zustaendige_behoerde_id: Some(ids),
            }),
        },
    )
}

pub fn transform_vorgang_transportieren_2010_impl(xml: &str, options: &TransformOptions) -> String {
    let mut rdr = NsReader::from_str(xml);
    rdr.config_mut().trim_text(false);
    rdr.config_mut().allow_unmatched_ends = true;

    let mut writer = Writer::new(Vec::<u8>::new());
    let resolved = resolve_options(options);
    let has_zusatzinfo_updates = resolved.zusatzinfo_ids.is_some();
    let has_leser = resolved.leser.is_some();
    let has_autor = resolved.autor.is_some();

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
                    &resolved,
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
                    &resolved,
                    &mut writer,
                    ns,
                    &lok,
                    &e,
                );
                state.depth = state.depth.saturating_sub(1);

                // Optimization: the last mutation point in document order has
                // been processed (</zusatzinformationen> replace,
                // </nachrichtenkopf.g2g> when no zusatzinfo update, or </root>
                // insert/fallback). Dump the remaining input verbatim instead
                // of parsing it (skips re-parsing <vorgang>, the <ds:Signature>
                // block, and trailing whitespace).
                if state.dump_tail {
                    let pos = rdr.buffer_position() as usize;
                    // pending_ws is empty here (the close tag was just
                    // written), but flush defensively in case of odd input.
                    state.flush_ws(&mut writer);
                    writer
                        .get_mut()
                        .extend_from_slice(xml.as_bytes()[pos..].as_ref());
                    break;
                }
            }

            Ok((_ns, Event::Empty(e))) => {
                handle_empty(&mut state, &mut writer, &e);
            }

            Ok((_ns, Event::Text(e))) => {
                let owned = Event::Text(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned, &resolved);
            }

            Ok((_ns, Event::CData(e))) => {
                let owned = Event::CData(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned, &resolved);
            }

            Ok((_ns, Event::Comment(e))) => {
                let owned = Event::Comment(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned, &resolved);
            }

            Ok((_ns, Event::PI(e))) => {
                let owned = Event::PI(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned, &resolved);
            }

            Ok((_ns, Event::Decl(e))) => {
                state.flush_ws(&mut writer);
                write_event(&mut writer, Event::Decl(e.clone().into_owned()));
            }

            Ok((_ns, Event::DocType(e))) => {
                state.flush_ws(&mut writer);
                write_event(&mut writer, Event::DocType(e.clone().into_owned()));
            }

            Ok((_ns, Event::Eof)) => {
                // Flush any trailing whitespace held back by pending_ws so the
                // no-op transform stays byte-identical (incl. trailing newline).
                state.flush_ws(&mut writer);
                break;
            }

            // On a parse error, do not emit a truncated/invalid document;
            // return the original input unchanged so callers can detect
            // and handle the malformed input themselves.
            Err(_) => return xml.to_string(),
        }
        buf.clear();
    }

    String::from_utf8(writer.into_inner()).unwrap_or_else(|e| {
        // If the writer produced invalid UTF-8, return the raw bytes as lossy
        String::from_utf8_lossy(e.as_bytes()).into_owned()
    })
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
    seen_nachrichten_uuid: bool,
    in_nachrichten_uuid: bool,
    nachrichten_uuid_buf: Vec<Event<'static>>,

    // Buffering for g2g element (leser/author) mutation
    in_g2g_element: bool,
    g2g_buf: Vec<Event<'static>>,
    g2g_element_name: Vec<u8>,

    // Measured indentation from surrounding XML
    nk_child_indent: Vec<u8>,
    root_child_indent: Vec<u8>,

    // Indentation unit (one level of indentation) measured from the source XML.
    // Falls back to 2 spaces when no indentation is detected.
    indent_unit: Vec<u8>,

    // XWasser prefix from root element (e.g. "xwas" or "xw")
    root_ns_prefix: Vec<u8>,

    // zusatzinformationen tracking
    seen_zi: bool,
    in_zi: bool,
    zi_buf: Vec<Event<'static>>,

    // Set once the last mutation point in document order has been processed.
    // All mutations target <nachrichtenkopf.g2g> (nachrichtenUUID, leser, autor)
    // and/or <zusatzinformationen>, which precede the rest of the document. So
    // once this is set, the main loop dumps the remaining input verbatim
    // instead of parsing it (skips re-parsing <vorgang>, <ds:Signature>, etc.).
    // Triggered at: </zusatzinformationen> (replace), </nachrichtenkopf.g2g>
    // (no zusatzinfo update), or </root> (insert-missing / fallback).
    dump_tail: bool,

    // Pending whitespace text node (held back so insertion helpers can own
    // the surrounding whitespace instead of duplicating or dropping it).
    // Flushed (written) immediately before the next non-whitespace event
    // unless an insertion consumes it.
    pending_ws: Option<Vec<u8>>,
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

    /// Write any held-back whitespace text node to the writer and clear it.
    /// Called before writing a non-whitespace event (or by an insertion
    /// helper that wants to re-emit the original trailing whitespace).
    fn flush_ws<W: std::io::Write>(&mut self, writer: &mut Writer<W>) {
        if let Some(ws) = self.pending_ws.take() {
            write_text_bytes(writer, &ws);
        }
    }

    /// One level of indentation, measured from the source XML.
    /// Falls back to 2 spaces when no indentation is detected.
    fn indent_unit(&self) -> &[u8] {
        if self.indent_unit.is_empty() {
            b"  "
        } else {
            &self.indent_unit
        }
    }

    /// Compute `indent + n * indent_unit` for nested levels.
    fn nested_indent(&self, base: &[u8], levels: usize) -> Vec<u8> {
        let mut result = base.to_vec();
        for _ in 0..levels {
            result.extend_from_slice(self.indent_unit());
        }
        result
    }

    /// Measure the indentation unit from a whitespace text node.
    /// The unit is the entire indentation string of the text node.
    /// For example, if the text is "    " (4 spaces), the unit is "    " (4 spaces).
    /// This is the indentation of one level of nesting.
    fn measure_indent_unit(&mut self, text: &[u8]) {
        if !self.indent_unit.is_empty() {
            return;
        }
        // Strip newlines and carriage returns to get just the indentation
        let indent: Vec<u8> = text
            .iter()
            .filter(|b| **b == b' ' || **b == b'\t')
            .copied()
            .collect();
        if indent.is_empty() {
            return;
        }
        self.indent_unit = indent;
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
    options: &ResolvedOptions,
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

    // Track nachrichtenUUID element for replacement/insertion
    if state.nk_depth > 0 && lok == b"nachrichtenUUID" && options.nachrichten_uuid.is_some() {
        state.in_nachrichten_uuid = true;
        state.nachrichten_uuid_buf.clear();
    }

    if state.in_nachrichten_uuid {
        state
            .nachrichten_uuid_buf
            .push(Event::Start(e.clone().into_owned()));
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
        state.seen_zi = true;
        if has_zusatzinfo_updates {
            // Write the preceding whitespace before we start buffering,
            // so write_zusatzinfo_content does not need to re-emit it.
            state.flush_ws(writer);
            state.in_zi = true;
            state.zi_buf.clear();
            state.zi_buf.push(Event::Start(e.clone().into_owned()));
            return;
        }
    }

    if state.should_insert_leser && lok != b"nachrichtenkopf.g2g" && lok != b"leser" {
        if let Some(r) = options.leser {
            let trailing = state.pending_ws.take();
            insert_g2g_element(writer, "leser", r, state, trailing.as_deref());
        }
        state.should_insert_leser = false;
        if has_autor && !state.seen_autor {
            state.should_insert_autor = true;
        }
    }
    if state.should_insert_autor
        && lok != b"nachrichtenkopf.g2g"
        && lok != b"leser"
        && lok != b"autor"
    {
        if let Some(r) = options.autor {
            let trailing = state.pending_ws.take();
            insert_g2g_element(writer, "autor", r, state, trailing.as_deref());
        }
        state.should_insert_autor = false;
    }

    if state.nk_depth > 0 && lok == b"leser" {
        state.seen_leser = true;
        state.should_insert_leser = false;
        if has_leser {
            state.flush_ws(writer);
            start_g2g_element_buf(state, e, b"leser");
            return;
        }
    }

    if state.nk_depth > 0 && lok == b"autor" {
        state.seen_autor = true;
        state.should_insert_autor = false;
        if has_autor {
            state.flush_ws(writer);
            start_g2g_element_buf(state, e, b"autor");
            return;
        }
    }

    // Insert missing <zusatzinformationen> right before <ds:Signature> —
    // its correct schema position is after <vorgang> and before the
    // signature (XSD sequence: vorgang, zusatzinformationen, ds:Signature).
    // By the time <ds:Signature> appears, seen_zi reliably tells us whether
    // <zusatzinformationen> already preceded it, so we only insert when it
    // is truly absent. After inserting, the rest (the signature block and
    // </root>) is dumped verbatim via dump_tail.
    if lok == b"Signature"
        && state.root_depth > 0
        && state.depth == state.root_depth + 1
        && !state.seen_zi
        && options.zusatzinfo_ids.is_some_and(|a| !a.is_empty())
    {
        let trailing = state.pending_ws.take();
        insert_zusatzinformationen_element(
            writer,
            options.zusatzinfo_ids.unwrap_or(&[]),
            if state.root_ns_prefix.is_empty() {
                b"xwas"
            } else {
                &state.root_ns_prefix
            },
            state,
            trailing.as_deref(),
        );
        // Re-emit the <ds:Signature> start tag; the main loop dumps the
        // remainder (children, </ds:Signature>, </root>) verbatim.
        write_event(writer, Event::Start(e.clone().into_owned()));
        state.dump_tail = true;
        return;
    }

    state.flush_ws(writer);
    write_event(writer, Event::Start(e.clone().into_owned()));
}

#[allow(clippy::too_many_arguments)]
fn handle_end(
    state: &mut TransformState,
    has_leser: bool,
    has_autor: bool,
    has_zusatzinfo_updates: bool,
    options: &ResolvedOptions,
    writer: &mut Writer<Vec<u8>>,
    ns: ResolveResult,
    lok: &[u8],
    e: &BytesEnd<'_>,
) {
    // The root-level </zusatzinformationen> is the last mutation point when
    // zusatzinfo is being replaced. Once it closes, the main loop dumps the
    // remaining input verbatim.
    if lok == b"zusatzinformationen"
        && ns_is_xwas(&ns)
        && state.root_depth > 0
        && state.depth == state.root_depth + 1
    {
        state.dump_tail = true;
    }

    if state.in_g2g_element && lok == state.g2g_element_name {
        state.in_g2g_element = false;
        let r = if state.g2g_element_name == b"leser" {
            options.leser
        } else {
            options.autor
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

    // Handle nachrichtenUUID end: replace text content if update is provided
    if state.in_nachrichten_uuid && lok == b"nachrichtenUUID" {
        state.in_nachrichten_uuid = false;
        state.seen_nachrichten_uuid = true;
        if let Some(uuid) = options.nachrichten_uuid {
            // Flush the preceding whitespace (the indent before
            // <nachrichtenUUID>) so the replacement stays on its own line
            // instead of being glued to the previous sibling's start tag.
            state.flush_ws(writer);
            // Re-emit the start tag with new text content
            for ev in &state.nachrichten_uuid_buf {
                if let Event::Start(s) = ev {
                    write_event(writer, Event::Start(s.clone()));
                }
            }
            write_event(writer, Event::Text(BytesText::new(uuid)));
            write_event(writer, Event::End(BytesEnd::new("nachrichtenUUID")));
            state.nachrichten_uuid_buf.clear();
            return;
        }
    }

    if state.in_zi && lok == b"zusatzinformationen" {
        state.in_zi = false;
        if !state.zi_buf.is_empty() {
            write_zusatzinfo_content(
                writer,
                &state.zi_buf,
                options.zusatzinfo_ids,
                &state.root_ns_prefix,
                state,
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
    // Insert nachrichtenUUID if missing and update is provided
    if state.nk_depth > 0
        && lok == b"identifikation.nachricht"
        && !state.seen_nachrichten_uuid
        && let Some(uuid) = options.nachrichten_uuid
    {
        let indent = state.nested_indent(&state.nk_child_indent, 1);
        write_text_bytes(writer, &indent);
        write_event(writer, Event::Start(BytesStart::new("nachrichtenUUID")));
        write_text_bytes(writer, uuid.as_bytes());
        write_event(writer, Event::End(BytesEnd::new("nachrichtenUUID")));
        // Re-emit any held-back inner whitespace as the closing-tag indent,
        // so </identifikation.nachricht> lands on its own line.
        state.flush_ws(writer);
        state.seen_nachrichten_uuid = true;
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
            if let Some(r) = options.leser {
                let trailing = state.pending_ws.take();
                insert_g2g_element(writer, "leser", r, state, trailing.as_deref());
            }
            state.should_insert_leser = false;
            if has_autor && !state.seen_autor {
                state.should_insert_autor = true;
            }
        }
        if state.should_insert_autor {
            if let Some(r) = options.autor {
                let trailing = state.pending_ws.take();
                insert_g2g_element(writer, "autor", r, state, trailing.as_deref());
            }
            state.should_insert_autor = false;
        }
        state.nk_depth = 0;
        // When no zusatzinfo update is requested, every mutation is done by
        // the time </nachrichtenkopf.g2g> closes — dump the rest (<vorgang>,
        // <zusatzinformationen>, <ds:Signature>, </root>) verbatim.
        if !has_zusatzinfo_updates {
            state.dump_tail = true;
        }
    }

    if state.root_depth > 0
        && lok == b"vorgang.transportieren.2010"
        && state.depth == state.root_depth
    {
        // Fallback for malformed input with no <vorgang> element: insert a
        // missing zusatzinfo at </root> (best effort) and dump the trailing
        // rest. Well-formed input never reaches here — the </vorgang> block
        // above handles the insert at the correct position and returns early.
        let should_insert = options.zusatzinfo_ids.is_some_and(|a| !a.is_empty());
        if !state.seen_zi && should_insert {
            let trailing = state.pending_ws.take();
            insert_zusatzinformationen_element(
                writer,
                options.zusatzinfo_ids.unwrap_or(&[]),
                if state.root_ns_prefix.is_empty() {
                    b"xwas"
                } else {
                    &state.root_ns_prefix
                },
                state,
                trailing.as_deref(),
            );
        }
        state.root_depth = 0;
        // Last mutation point for the insert-missing path (or any case that
        // reaches the root close): the rest is just trailing content after
        // </root>, dump it verbatim.
        state.dump_tail = true;
    }

    state.flush_ws(writer);
    write_event(writer, Event::End(e.clone().into_owned()));
}

fn handle_empty(state: &mut TransformState, writer: &mut Writer<Vec<u8>>, e: &BytesStart<'_>) {
    if state.in_g2g_element {
        state.g2g_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    if state.in_zi {
        state.zi_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    state.flush_ws(writer);
    write_event(writer, Event::Empty(e.clone().into_owned()));
}

fn handle_generic(
    state: &mut TransformState,
    writer: &mut Writer<Vec<u8>>,
    event: Event<'static>,
    options: &ResolvedOptions,
) {
    if state.in_g2g_element {
        state.g2g_buf.push(event);
        return;
    }
    if state.in_zi {
        state.zi_buf.push(event);
        return;
    }

    // Skip text content inside nachrichtenUUID (we'll re-emit with new value)
    if state.in_nachrichten_uuid
        && options.nachrichten_uuid.is_some()
        && matches!(event, Event::Text(_))
    {
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
                state.measure_indent_unit(bytes);
            }
            if state.root_depth > 0
                && state.depth == state.root_depth
                && state.root_child_indent.is_empty()
            {
                state.root_child_indent = bytes.to_vec();
                state.measure_indent_unit(bytes);
            }
            // Hold the whitespace back instead of writing it eagerly.
            // It is flushed right before the next non-whitespace event
            // (see the else branch) or consumed by an insertion helper.
            // Accumulate consecutive whitespace events in case the reader
            // splits a text node (e.g. around entity references).
            match state.pending_ws.take() {
                Some(mut v) => {
                    v.extend_from_slice(bytes);
                    state.pending_ws = Some(v);
                }
                None => state.pending_ws = Some(bytes.to_vec()),
            }
            return;
        }
    }

    state.flush_ws(writer);
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
        let prefix_str = std::str::from_utf8(prefix).unwrap_or("xwas");
        format!("{prefix_str}:{local}")
    }
}

/// Emit the `<zusatzinformationen>` content, replaying buffered events but
/// replacing `<zustaendigeBehoerdeID>` elements with the new IDs.
///
/// This preserves all other content inside `<zusatzinformationen>` — comments,
/// `<wasserversorgungsgebietID>`, `<kommentar>`, whitespace text nodes, etc.
/// — while only swapping out the authority ID entries.
///
/// The buffer includes the `<zusatzinformationen>` start tag but not the end
/// tag (the end tag is handled by the caller's early return).
fn write_zusatzinfo_content<W: std::io::Write>(
    writer: &mut Writer<W>,
    buffered: &[Event<'static>],
    updates: Option<&[String]>,
    prefix: &[u8],
    state: &TransformState,
) {
    let has_updates = updates.is_some_and(|v| !v.is_empty());
    let zbid_local = b"zustaendigeBehoerdeID";
    let mut skip_zbid = false;
    let mut zi_end_name: Vec<u8> = Vec::new();
    let mut events = Vec::new();
    let mut text_buffer = Vec::new();
    let mut additional_events = Vec::new();

    // Insert new zustaendigeBehoerdeID entries at the end (before the closing tag)
    let zbid = qn_str(prefix, "zustaendigeBehoerdeID");
    // root_child_indent includes leading newline (e.g. "\n  "), so
    // nested_indent gives "\n    " for 2-space XML. We use it directly
    // as the text before each ID and the closing tag.
    let id_indent = state.nested_indent(state.root_child_indent(), 1);
    let close_indent = state.root_child_indent();
    let id_text = String::from_utf8_lossy(&id_indent).to_string();
    let close_text = String::from_utf8_lossy(close_indent).to_string();
    if let Some(entries) = updates {
        for id in entries {
            additional_events.push(Event::Text(BytesText::new(&id_text)));
            additional_events.push(Event::Start(BytesStart::new(&zbid)));
            additional_events.push(Event::Text(BytesText::new(id)));
            additional_events.push(Event::End(BytesEnd::new(&zbid)));
        }
    }
    for ev in buffered {
        match ev {
            Event::Start(e) => {
                if e.local_name().as_ref() == zbid_local {
                    if has_updates && !text_buffer.is_empty() {
                        text_buffer.clear();
                    }
                    skip_zbid = true;
                    continue;
                }
                text_buffer.reverse();
                while let Some(e) = text_buffer.pop() {
                    events.push(e);
                }
                events.push(ev.clone());
                if zi_end_name.is_empty() && e.local_name().as_ref() == b"zusatzinformationen" {
                    zi_end_name = e.name().as_ref().to_vec();
                    additional_events.reverse();
                    while let Some(e) = additional_events.pop() {
                        events.push(e);
                    }
                }
            }
            Event::End(e) => {
                if e.local_name().as_ref() == zbid_local && skip_zbid {
                    if has_updates && !text_buffer.is_empty() {
                        text_buffer.clear();
                    }
                    skip_zbid = false;
                    continue;
                }
                text_buffer.reverse();
                while let Some(e) = text_buffer.pop() {
                    events.push(e);
                }
                events.push(ev.clone());
            }
            Event::Empty(_) => {
                events.push(ev.clone());
            }
            Event::Text(_) => {
                if skip_zbid {
                    continue;
                }
                text_buffer.push(ev.clone());
            }
            _ => {
                events.push(ev.clone());
            }
        }
    }

    // Flush any remaining buffered text (but clear it to avoid double newlines,
    // since close_text already provides the correct indent)
    text_buffer.clear();

    events.push(Event::Text(BytesText::new(&close_text)));
    events.push(Event::End(BytesEnd::new(
        std::str::from_utf8(&zi_end_name).unwrap_or("zusatzinformationen"),
    )));
    // The whitespace preceding <zusatzinformationen> was already flushed by
    // handle_start before the start tag was buffered, so the first event here
    // is the start tag itself — no leading indent needed.
    for ev in events {
        write_event(writer, ev);
    }
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
    state: &TransformState,
    trailing_ws: Option<&[u8]>,
) {
    let kennung = update.kennung.as_deref().unwrap_or("");
    let name = update.name.as_deref().unwrap_or("");

    // The measured indents already carry the leading line ending from the
    // source XML (e.g. "\n    " or "\r\n    "), so each line is written as a
    // single text node and we must NOT prepend an extra "\n".
    let indent = state.g2g_child_indent();
    let sub = state.nested_indent(indent, 1);
    let subsub = state.nested_indent(indent, 2);

    write_text_bytes(writer, indent);
    write_event(writer, Event::Start(BytesStart::new(element_name)));
    write_text_bytes(writer, &sub);
    let mut vd = BytesStart::new("verzeichnisdienst");
    vd.push_attribute(("listVersionID", ""));
    write_event(writer, Event::Start(vd));
    write_text_bytes(writer, &subsub);
    write_event(writer, Event::Start(BytesStart::new("code")));
    write_event(writer, Event::End(BytesEnd::new("code")));
    write_text_bytes(writer, &sub);
    write_event(writer, Event::End(BytesEnd::new("verzeichnisdienst")));
    write_text_bytes(writer, &sub);
    write_event(writer, Event::Start(BytesStart::new("kennung")));
    write_text_bytes(writer, kennung.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("kennung")));
    write_text_bytes(writer, &sub);
    write_event(writer, Event::Start(BytesStart::new("name")));
    write_text_bytes(writer, name.as_bytes());
    write_event(writer, Event::End(BytesEnd::new("name")));
    write_text_bytes(writer, indent);
    write_event(writer, Event::End(BytesEnd::new(element_name)));
    // Re-emit the original trailing whitespace (the indent of the following
    // sibling or the parent's closing tag) so the next tag is not glued onto
    // the same line as this element's closing tag.
    if let Some(ws) = trailing_ws {
        write_text_bytes(writer, ws);
    }
}

// ---------------------------------------------------------------------------
// Insert a new `<xwas:zusatzinformationen>` at the end of the root element
// ---------------------------------------------------------------------------

fn insert_zusatzinformationen_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    zusatzinformationen: &[String],
    prefix: &[u8],
    state: &TransformState,
    trailing_ws: Option<&[u8]>,
) {
    let non_empty: Vec<&String> = zusatzinformationen
        .iter()
        .filter(|id| !id.is_empty())
        .collect();

    if non_empty.is_empty() {
        return;
    }

    let indent = state.root_child_indent();
    let sub = state.nested_indent(indent, 1);

    let zi = qn_str(prefix, "zusatzinformationen");
    let zbid = qn_str(prefix, "zustaendigeBehoerdeID");

    // indent includes leading newline (e.g. "\n  "), so we use it directly
    write_text_bytes(writer, indent);
    write_event(writer, Event::Start(BytesStart::new(&zi)));

    for id in &non_empty {
        // sub includes leading newline (e.g. "\n    ")
        write_text_bytes(writer, &sub);
        write_event(writer, Event::Start(BytesStart::new(&zbid)));
        write_text_bytes(writer, id.as_bytes());
        write_event(writer, Event::End(BytesEnd::new(&zbid)));
    }

    write_text_bytes(writer, indent);
    write_event(writer, Event::End(BytesEnd::new(&zi)));
    // Re-emit the original trailing whitespace (the line ending before
    // </root>) so the root closing tag is not glued to </zusatzinformationen>.
    if let Some(ws) = trailing_ws {
        write_text_bytes(writer, ws);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests;
