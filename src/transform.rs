//! Streaming XML transform for XWasser messages using quick-xml.
//!
//! Mutates `<leser>` inside `nachrichtenkopf.g2g` and `<zustaendigeBehoerde>`
//! elements inside `<zusatzinformationen>` in a single pass, preserving all
//! comments, processing instructions, whitespace text nodes, and attribute order.
//!
//! A no-op transform (no reader, no authorities) produces output that is
//! byte-identical to the input, keeping XML digital signatures valid.

use raxb::quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use raxb::quick_xml::Reader;
use raxb::quick_xml::Writer;

/// Update parameters for the `<leser>` element inside `nachrichtenkopf.g2g`.
/// All fields are optional; `None` leaves the existing value unchanged.
#[derive(Debug, Clone, Default)]
pub struct ReaderUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

/// Update parameters for a single `<zustaendigeBehoerde>` element inside
/// `<zusatzinformationen>`. `kennung` is used for matching existing elements;
/// `name` is updated for the matched/replaced element.
#[derive(Debug, Clone, Default)]
pub struct AuthorityUpdate {
    pub kennung: Option<String>,
    pub name: Option<String>,
}

// ---------------------------------------------------------------------------
// Core streaming transform
// ---------------------------------------------------------------------------

/// Run the XML transform in a single streaming pass.
pub fn transform_xml(
    xml: &str,
    reader: Option<&ReaderUpdate>,
    authorities: &[AuthorityUpdate],
) -> String {
    let mut rdr = Reader::from_str(xml);
    rdr.config_mut().trim_text(false);
    // Keep expand_empty_elements = false (default) so that self-closing
    // tags like <code/> stay as Empty events and are reproduced verbatim.
    rdr.config_mut().allow_unmatched_ends = true;

    let mut writer = Writer::new(Vec::<u8>::new());

    let has_authority_updates =
        authorities.iter().any(|a| a.kennung.is_some() || a.name.is_some());

    let cfg = TransformCfg {
        reader,
        authorities,
        has_authority_updates,
    };

    let mut state = TransformState::default();
    let mut buf = Vec::new();

    loop {
        match rdr.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                state.depth += 1;
                let name = e.name().as_ref().to_vec();
                handle_start(&mut state, &cfg, &mut writer, &name, &e);
            }

            Ok(Event::End(e)) => {
                let name = e.name().as_ref().to_vec();
                handle_end(&mut state, &cfg, &mut writer, &name, &e);
                state.depth = state.depth.saturating_sub(1);
            }

            Ok(Event::Empty(e)) => {
                let name = e.name().as_ref().to_vec();
                handle_empty(&mut state, &cfg, &mut writer, &name, &e);
            }

            Ok(Event::Text(e)) => {
                let owned = Event::Text(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok(Event::CData(e)) => {
                let owned = Event::CData(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok(Event::Comment(e)) => {
                let owned = Event::Comment(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok(Event::PI(e)) => {
                let owned = Event::PI(e.clone().into_owned());
                handle_generic(&mut state, &mut writer, owned);
            }

            Ok(Event::Decl(e)) => {
                write_event(&mut writer, Event::Decl(e.clone().into_owned()));
            }

            Ok(Event::DocType(e)) => {
                write_event(&mut writer, Event::DocType(e.clone().into_owned()));
            }

            Ok(Event::Eof) => break,

            Err(_) => break,
        }
        buf.clear();
    }

    String::from_utf8(writer.into_inner()).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Configuration and state
// ---------------------------------------------------------------------------

struct TransformCfg<'a> {
    reader: Option<&'a ReaderUpdate>,
    authorities: &'a [AuthorityUpdate],
    has_authority_updates: bool,
}

#[derive(Default)]
struct TransformState {
    depth: usize,

    // Root element tracking
    root_depth: usize,

    // nachrichtenkopf.g2g tracking
    nk_depth: usize,
    seen_leser: bool,
    should_insert_leser: bool,

    // Buffering for leser mutation
    in_leser: bool,
    leser_buf: Vec<Event<'static>>,

    // zusatzinformationen tracking
    zi_depth: usize,
    seen_zi: bool,

    // zustaendigeBehoerde buffering
    in_zb: bool,
    zb_buf: Vec<Event<'static>>,

    // vorgang tracking (for insert-after logic)
    vorgang_depth: usize,
}

// ---------------------------------------------------------------------------
// Event handlers
// ---------------------------------------------------------------------------

fn handle_start(
    state: &mut TransformState,
    cfg: &TransformCfg,
    writer: &mut Writer<Vec<u8>>,
    name: &[u8],
    e: &BytesStart<'_>,
) {
    // General buffering for nested content inside leser / zustaendigeBehoerde
    if state.in_leser {
        state.leser_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }
    if state.in_zb {
        state.zb_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }

    // Track root element depth
    if name == b"xwas:vorgang.transportieren.2010" && state.root_depth == 0 {
        state.root_depth = state.depth;
    }

    // Track parent depth
    if name == b"nachrichtenkopf.g2g" {
        state.nk_depth = state.depth;
        state.seen_leser = false;
        state.should_insert_leser = false;
    }
    if name == b"xwas:zusatzinformationen" {
        state.zi_depth = state.depth;
        state.seen_zi = true;
    }
    if name == b"xwas:vorgang" && state.vorgang_depth == 0 {
        state.vorgang_depth = state.depth;
    }

    // Insert missing leser before autor (second-child position)
    if state.should_insert_leser && name == b"autor" {
        if let Some(r) = cfg.reader {
            insert_reader_element(writer, r);
        }
        state.should_insert_leser = false;
    }

    // --- leser element itself ---
    if state.nk_depth > 0 && name == b"leser" {
        state.seen_leser = true;
        if cfg.reader.is_some() {
            state.in_leser = true;
            state.leser_buf.clear();
            state.leser_buf.push(Event::Start(e.clone().into_owned()));
            return;
        }
    }

    // --- zustaendigeBehoerde element itself ---
    if state.zi_depth > 0 && name == b"xwas:zustaendigeBehoerde" && cfg.has_authority_updates {
        state.in_zb = true;
        state.zb_buf.clear();
        state.zb_buf.push(Event::Start(e.clone().into_owned()));
        return;
    }

    write_event(writer, Event::Start(e.clone().into_owned()));
}

fn handle_end(
    state: &mut TransformState,
    cfg: &TransformCfg,
    writer: &mut Writer<Vec<u8>>,
    name: &[u8],
    e: &BytesEnd<'_>,
) {
    // Track identifikation.nachricht close -> schedule leser insertion after it
    if state.nk_depth > 0 && name == b"identifikation.nachricht"
        && cfg.reader.is_some() && !state.seen_leser {
            state.should_insert_leser = true;
        }

    // --- closing leser end ---
    if state.in_leser && name == b"leser" {
        state.in_leser = false;
        if let Some(r) = cfg.reader
            && !state.leser_buf.is_empty()
        {
            emit_mutated_reader(writer, &state.leser_buf, r);
            write_event(writer, Event::End(BytesEnd::new("leser")));
            state.leser_buf.clear();
            return;
        }
    }

    // --- closing zustaendigeBehoerde end ---
    if state.in_zb && name == b"xwas:zustaendigeBehoerde" {
        state.in_zb = false;
        if !state.zb_buf.is_empty() {
            emit_mutated_authority(writer, &state.zb_buf, cfg.authorities);
            state.zb_buf.clear();
            return;
        }
    }

    // Buffer nested end events
    if state.in_leser {
        state.leser_buf.push(Event::End(e.clone().into_owned()));
        return;
    }
    if state.in_zb {
        state.zb_buf.push(Event::End(e.clone().into_owned()));
        return;
    }

    // --- closing nachrichtenkopf.g2g (insert leser if still missing, no autor) ---
    if state.nk_depth > 0 && name == b"nachrichtenkopf.g2g" {
        if state.should_insert_leser {
            if let Some(r) = cfg.reader {
                insert_reader_element(writer, r);
            }
            state.should_insert_leser = false;
        }
        state.nk_depth = 0;
    }

    // --- closing zusatzinformationen ---
    if name == b"xwas:zusatzinformationen" {
        state.zi_depth = 0;
    }

    // --- closing root (insert zusatzinformationen if still missing) ---
    if state.root_depth > 0
        && name == b"xwas:vorgang.transportieren.2010"
        && state.depth == state.root_depth
    {
        if !state.seen_zi && cfg.has_authority_updates {
            insert_zusatzinformationen_element(writer, cfg.authorities);
        }
        state.root_depth = 0;
    }

    write_event(writer, Event::End(e.clone().into_owned()));
}

fn handle_empty(
    state: &mut TransformState,
    cfg: &TransformCfg,
    writer: &mut Writer<Vec<u8>>,
    _name: &[u8],
    e: &BytesStart<'_>,
) {
    if state.in_leser {
        state.leser_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    if state.in_zb {
        state.zb_buf.push(Event::Empty(e.clone().into_owned()));
        return;
    }
    let _ = cfg;
    write_event(writer, Event::Empty(e.clone().into_owned()));
}

fn handle_generic(state: &mut TransformState, writer: &mut Writer<Vec<u8>>, event: Event<'static>) {
    if state.in_leser {
        state.leser_buf.push(event);
        return;
    }
    if state.in_zb {
        state.zb_buf.push(event);
        return;
    }
    write_event(writer, event);
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
// Emit mutated <leser> from buffered events
// ---------------------------------------------------------------------------

fn emit_mutated_reader<W: std::io::Write>(
    writer: &mut Writer<W>,
    buffered: &[Event<'static>],
    update: &ReaderUpdate,
) {
    let mut inside_kennung = false;
    let mut inside_name = false;
    let has_kennung_update = update.kennung.is_some();
    let has_name_update = update.name.is_some();

    for ev in buffered {
        match ev {
            Event::Start(e) => {
                let en = e.name().as_ref().to_vec();
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
                let en = e.name().as_ref().to_vec();
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
                // Skip original text for kennung/name if we already wrote replacement
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

fn emit_mutated_authority<W: std::io::Write>(
    writer: &mut Writer<W>,
    buffered: &[Event<'static>],
    authorities: &[AuthorityUpdate],
) {
    let current_kennung = extract_kennung_from_zb_buf(buffered);

    if let Some(ref cur) = current_kennung
        && let Some(matching) = authorities.iter().find(|a| a.kennung.as_deref() == Some(cur))
    {
        // Write start tag from first buffered event
        if let Some(Event::Start(first)) = buffered.first() {
            write_event(writer, Event::Start(first.clone()));
        }
        // Write updated content: kennung and name only
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
        write_event(writer, Event::End(BytesEnd::new("xwas:zustaendigeBehoerde")));
        return;
    }

    // No match – passthrough original
    for ev in buffered {
        write_event(writer, ev.clone());
    }
}

fn extract_kennung_from_zb_buf(buffered: &[Event<'static>]) -> Option<String> {
    let mut in_kennung = false;
    for ev in buffered {
        match ev {
            Event::Start(e) if e.name().as_ref() == b"xwas:kennung" => in_kennung = true,
            Event::Text(t) if in_kennung => {
                return Some(String::from_utf8_lossy(t.as_ref()).to_string());
            }
            Event::End(e) if e.name().as_ref() == b"xwas:kennung" => in_kennung = false,
            _ => {}
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Insert a new <leser> as second child of nachrichtenkopf.g2g
// ---------------------------------------------------------------------------

fn insert_reader_element<W: std::io::Write>(writer: &mut Writer<W>, update: &ReaderUpdate) {
    let kennung = update.kennung.as_deref().unwrap_or("");
    let name = update.name.as_deref().unwrap_or("");

    write_text_bytes(writer, b"\n");
    write_text_bytes(writer, b"    ");
    write_event(writer, Event::Start(BytesStart::new("leser")));
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
    write_event(writer, Event::End(BytesEnd::new("leser")));
}

// ---------------------------------------------------------------------------
// Insert a new <xwas:zusatzinformationen> at the end of the root element
// (before </xwas:vorgang.transportieren.2010>)
// ---------------------------------------------------------------------------

fn insert_zusatzinformationen_element<W: std::io::Write>(
    writer: &mut Writer<W>,
    authorities: &[AuthorityUpdate],
) {
    let non_empty: Vec<&AuthorityUpdate> = authorities
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

    fn sample_xml_no_leser() -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0" produkt="SHAPTH CLI" produkthersteller="H &amp; D GmbH" produktversion="0.800.0" standard="XWasser" test="true" version="1.0.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht>
      <nachrichtenUUID>693c64d6-456f-4d14-abe7-fe9681c74aae</nachrichtenUUID>
    </identifikation.nachricht>
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

    fn sample_xml_with_zi_extra_children() -> String {
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
      <xwas:kennung>auth-with-extra</xwas:kennung>
      <xwas:name>With Extras</xwas:name>
      <xwas:kommentar>some comment</xwas:kommentar>
    </xwas:zustaendigeBehoerde>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    fn sample_xml_with_zi_self_closing_name() -> String {
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
      <xwas:kennung>auth-selfclose</xwas:kennung>
      <xwas:name/>
    </xwas:zustaendigeBehoerde>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>"#
        .to_string()
    }

    // ---- tests ----

    #[test]
    fn test_noop_is_byte_identical() {
        let xml = sample_xml();
        let result = transform_xml(&xml, None, &[]);
        assert_eq!(result, xml, "no-op transform must produce byte-identical output");
    }

    #[test]
    fn test_transform_reader_mutation() {
        let xml = sample_xml();
        let result = transform_xml(
            &xml,
            Some(&ReaderUpdate {
                kennung: Some("psw:99999999".into()),
                name: Some("NewReader".into()),
            }),
            &[],
        );

        assert!(
            result.contains("<kennung>psw:99999999</kennung>"),
            "reader kennung not updated in:\n{result}"
        );
        assert!(
            result.contains("<name>NewReader</name>"),
            "reader name not updated in:\n{result}"
        );
        assert!(
            result.contains("<kennung>psw:01003110</kennung>"),
            "autor kennung missing"
        );
        assert!(result.contains("<name>Author</name>"), "autor name missing");

        // Content assertions (schema model round-trip tested separately
        // with full-quality XML documents)
    }

    #[test]
    fn test_transform_authorities_mutation() {
        let xml = sample_xml_with_zi();
        let result = transform_xml(
            &xml,
            None,
            &[AuthorityUpdate {
                kennung: Some("auth-001".into()),
                name: Some("Updated Authority".into()),
            }],
        );

        assert!(result.contains("<xwas:kennung>auth-001</xwas:kennung>"), "auth kennung missing");
        assert!(
            result.contains("<xwas:name>Updated Authority</xwas:name>"),
            "auth name not updated"
        );

    }

    #[test]
    fn test_transform_authorities_mutation_extra_children() {
        let xml = sample_xml_with_zi_extra_children();
        let result = transform_xml(
            &xml,
            None,
            &[AuthorityUpdate {
                kennung: Some("auth-with-extra".into()),
                name: Some("Replaced".into()),
            }],
        );

        assert!(result.contains("<xwas:kennung>auth-with-extra</xwas:kennung>"));
        assert!(result.contains("<xwas:name>Replaced</xwas:name>"));
        // Extra children should NOT survive — element is replaced entirely
        assert!(!result.contains("some comment"), "extra children must be dropped on replacement");
    }

    #[test]
    fn test_transform_authorities_mutation_self_closing_name() {
        let xml = sample_xml_with_zi_self_closing_name();
        let result = transform_xml(
            &xml,
            None,
            &[AuthorityUpdate {
                kennung: Some("auth-selfclose".into()),
                name: Some("Now Has Name".into()),
            }],
        );

        assert!(result.contains("<xwas:kennung>auth-selfclose</xwas:kennung>"));
        assert!(result.contains("<xwas:name>Now Has Name</xwas:name>"));
    }

    #[test]
    fn test_transform_no_authorities_noop() {
        let xml = sample_xml_with_zi();
        let result = transform_xml(&xml, None, &[]);
        assert!(result.contains("<xwas:kennung>auth-001</xwas:kennung>"));
        assert!(result.contains("<xwas:name>Existing Authority</xwas:name>"));
    }

    #[test]
    fn test_transform_multiple_authorities() {
        let xml = sample_xml_with_zi();
        let result = transform_xml(
            &xml,
            None,
            &[
                AuthorityUpdate {
                    kennung: Some("auth-001".into()),
                    name: Some("Updated First".into()),
                },
                AuthorityUpdate {
                    kennung: Some("auth-002".into()),
                    name: Some("Second New".into()),
                },
            ],
        );

        // auth-001 should be updated in-place
        assert!(result.contains("<xwas:kennung>auth-001</xwas:kennung>"));
        assert!(result.contains("<xwas:name>Updated First</xwas:name>"));
        // auth-002 has no match in existing zusatzinformationen,
        // so it should NOT appear (no insertion when ZI already exists)
        assert!(!result.contains("auth-002"), "unmatched authority should not appear when ZI exists");
    }

    #[test]
    fn test_transform_insert_reader() {
        let xml = sample_xml_no_leser();
        let result = transform_xml(
            &xml,
            Some(&ReaderUpdate {
                kennung: Some("psw:inserted".into()),
                name: Some("Inserted Reader".into()),
            }),
            &[],
        );

        assert!(
            result.contains("<kennung>psw:inserted</kennung>"),
            "inserted kennung missing"
        );
        assert!(
            result.contains("<name>Inserted Reader</name>"),
            "inserted name missing"
        );

        let leser_pos = result.find("<kennung>psw:inserted</kennung>").unwrap();
        let autor_pos = result.find("<kennung>psw:01003110</kennung>").unwrap();
        assert!(leser_pos < autor_pos, "leser must come before autor");


    }

    #[test]
    fn test_transform_insert_zusatzinformationen() {
        let xml = sample_xml_no_zi();
        let result = transform_xml(
            &xml,
            None,
            &[AuthorityUpdate {
                kennung: Some("new-auth".into()),
                name: Some("New Authority".into()),
            }],
        );

        assert!(result.contains("xwas:zusatzinformationen"), "zusatzinformationen missing");
        assert!(result.contains("<xwas:kennung>new-auth</xwas:kennung>"), "auth kennung missing");
        assert!(
            result.contains("<xwas:name>New Authority</xwas:name>"),
            "auth name missing"
        );

    }

    #[test]
    fn test_transform_insert_zusatzinformationen_multiple() {
        let xml = sample_xml_no_zi();
        let result = transform_xml(
            &xml,
            None,
            &[
                AuthorityUpdate {
                    kennung: Some("first-auth".into()),
                    name: Some("First Authority".into()),
                },
                AuthorityUpdate {
                    kennung: Some("second-auth".into()),
                    name: Some("Second Authority".into()),
                },
            ],
        );

        assert!(result.contains("xwas:zusatzinformationen"), "zusatzinformationen missing");
        assert!(result.contains("<xwas:kennung>first-auth</xwas:kennung>"));
        assert!(result.contains("<xwas:kennung>second-auth</xwas:kennung>"));
        assert!(result.contains("<xwas:name>First Authority</xwas:name>"));
        assert!(result.contains("<xwas:name>Second Authority</xwas:name>"));

    }

    #[test]
    fn test_transform_comment_preservation() {
        let xml = sample_xml();
        let result = transform_xml(&xml, None, &[]);

        assert!(result.contains("<!-- root comment -->"));
    }

    #[test]
    fn test_transform_whitespace_preservation() {
        let xml = sample_xml();
        let result = transform_xml(&xml, None, &[]);

        assert!(result.contains("  <nachrichtenkopf.g2g>"));
        assert!(result.contains("    <identifikation.nachricht>"));
    }

    #[test]
    fn test_transform_raxb_roundtrip_quality_report() {
        // Load a full quality report XML that raxb can parse
        let path = std::env::current_dir()
            .unwrap()
            .join("tests/quality_report_minimal.xml");
        let xml = std::fs::read_to_string(path).unwrap();

        // Mutate reader
        let result = transform_xml(
            &xml,
            Some(&ReaderUpdate {
                kennung: Some("psw:mutated".into()),
                name: Some("Mutated Reader".into()),
            }),
            &[],
        );

        // raxb must be able to parse the result
        let parsed: Result<crate::model::transport::VorgangTransportieren2010, _> =
            raxb::de::from_str(&result);
        assert!(
            parsed.is_ok(),
            "raxb round-trip failed: {:?}",
            parsed.err()
        );
        let parsed = parsed.unwrap();
        assert_eq!(
            parsed.nachrichtenkopf_g2g.leser.kennung, "psw:mutated",
            "leser kennung should be updated"
        );
        assert_eq!(
            parsed.nachrichtenkopf_g2g.leser.name, "Mutated Reader",
            "leser name should be updated"
        );
        // autor unchanged
        assert_eq!(
            parsed.nachrichtenkopf_g2g.autor.kennung, "psw:01003110",
            "autor kennung should stay unchanged"
        );
        // No authorities provided — should still parse
        assert!(parsed.zusatzinformationen.is_none());
    }

    #[test]
    fn test_transform_signature_roundtrip() {
        let xml = sample_xml();
        let result = transform_xml(&xml, None, &[]);

        assert!(result.contains("ds:Signature"), "ds:Signature missing");
        assert!(result.contains("ds:SignedInfo"), "ds:SignedInfo missing");
        assert!(result.contains("ds:DigestValue"), "ds:DigestValue missing");
        assert!(
            result.contains("ds:SignatureValue"),
            "ds:SignatureValue missing"
        );
        assert!(result.contains("ds:X509Data"), "ds:X509Data missing");
    }
}
