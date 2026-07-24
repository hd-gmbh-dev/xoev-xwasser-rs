import { describe, it, expect } from "vitest";
import { transform_vorgang_transportieren_2010 } from "../pkg/xoev_xwasser";

function sampleXml(): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</xwas:vorgang.transportieren.2010>`;
}

function sampleXmlNoLeserNoAutor(): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</xwas:vorgang.transportieren.2010>`;
}

function sampleXmlNoZi(): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
</xwas:vorgang.transportieren.2010>`;
}

function sampleXmlWithZi(): string {
  return `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
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
    <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>`;
}

describe("transform_vorgang_transportieren_2010 via wasm", () => {
  it("no-op transform produces byte-identical output", () => {
    const xml = sampleXml();
    expect(transform_vorgang_transportieren_2010(xml)).toBe(xml);
    expect(transform_vorgang_transportieren_2010(xml, undefined)).toBe(xml);
    expect(transform_vorgang_transportieren_2010(xml, null)).toBe(xml);
    expect(transform_vorgang_transportieren_2010(xml, {})).toBe(xml);
  });

  it("transforms leser element in-place", () => {
    const result = transform_vorgang_transportieren_2010(sampleXml(), {
      nachrichtenkopf_g2g: { leser: { kennung: "psw:99999999", name: "NewReader" } },
    });
    expect(result).toContain("<kennung>psw:99999999</kennung>");
    expect(result).toContain("<name>NewReader</name>");
    // autor unchanged
    expect(result).toContain("<kennung>psw:01003110</kennung>");
    expect(result).toContain("<name>Author</name>");
  });

  it("transforms autor element in-place", () => {
    const result = transform_vorgang_transportieren_2010(sampleXml(), {
      nachrichtenkopf_g2g: { autor: { kennung: "psw:autor123", name: "Updated Autor" } },
    });
    // leser unchanged
    expect(result).toContain("<kennung>psw:11113110</kennung>");
    expect(result).toContain("<name>Reader</name>");
    expect(result).toContain("<kennung>psw:autor123</kennung>");
    expect(result).toContain("<name>Updated Autor</name>");
  });

  it("transforms both leser and autor in-place", () => {
    const result = transform_vorgang_transportieren_2010(sampleXml(), {
      nachrichtenkopf_g2g: {
        leser: { kennung: "psw:L", name: "Leser" },
        autor: { kennung: "psw:A", name: "Autor" },
      },
    });
    expect(result).toContain("<kennung>psw:L</kennung>");
    expect(result).toContain("<kennung>psw:A</kennung>");
  });

  it("transforms authorities elements in-place", () => {
    const result = transform_vorgang_transportieren_2010(sampleXmlWithZi(), {
      zusatzinformationen: { zustaendige_behoerde_id: ["auth-001"] },
    });
    expect(result).toContain("zustaendigeBehoerdeID>auth-001");
  });

  it("inserts missing leser as second child of nachrichtenkopf.g2g", () => {
    const result = transform_vorgang_transportieren_2010(sampleXmlNoLeserNoAutor(), {
      nachrichtenkopf_g2g: {
        leser: { kennung: "psw:inserted", name: "Inserted Reader" },
        autor: { kennung: "psw:newauthor", name: "New Autor" },
      },
    });
    expect(result).toContain("<kennung>psw:inserted</kennung>");
    expect(result).toContain("<kennung>psw:newauthor</kennung>");
    const leserPos = result.indexOf("psw:inserted");
    const autorPos = result.indexOf("psw:newauthor");
    expect(leserPos).toBeLessThan(autorPos);
  });

  it("inserts missing zusatzinformationen", () => {
    const result = transform_vorgang_transportieren_2010(sampleXmlNoZi(), {
      zusatzinformationen: { zustaendige_behoerde_id: ["new-auth"] },
    });
    expect(result).toContain("xwas:zusatzinformationen");
    expect(result).toMatch(/xwas:zusatzinformationen/);
  });

  it("inserts multiple authorities when zusatzinformationen missing", () => {
    const result = transform_vorgang_transportieren_2010(sampleXmlNoZi(), {
      zusatzinformationen: { zustaendige_behoerde_id: ["id1", "id2"] },
    });
    expect(result).toMatch(/xwas:zusatzinformationen/);
  });

  it("replaces zustaendigeBehoerdeID while preserving other fields", () => {
    const ns =
      'xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0"';
    const xml = `<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 ${ns}>
  <nachrichtenkopf.g2g>
    <identifikation.nachricht><nachrichtenUUID>t</nachrichtenUUID></identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>t</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
  <xwas:zusatzinformationen>
    <xwas:zustaendigeBehoerdeID>Old</xwas:zustaendigeBehoerdeID>
    <xwas:kommentar>should be preserved</xwas:kommentar>
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>`;
    const result = transform_vorgang_transportieren_2010(xml, {
      zusatzinformationen: { zustaendige_behoerde_id: ["auth-1"] },
    });
    expect(result).toMatch(/xwas:zusatzinformationen/);
    // Old ID replaced
    expect(result).not.toContain("Old");
    // New ID present
    expect(result).toContain("auth-1");
    // kommentar preserved
    expect(result).toContain("should be preserved");
  });

  it("preserves comments verbatim through round-trip", () => {
    const xml = sampleXml();
    expect(transform_vorgang_transportieren_2010(xml)).toContain("<!-- root comment -->");
  });

  it("preserves comments through leser mutation", () => {
    const ns =
      'xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0"';
    const xml = `<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 ${ns}>
  <nachrichtenkopf.g2g>
    <identifikation.nachricht><nachrichtenUUID>id</nachrichtenUUID></identifikation.nachricht>
    <leser>
      <!-- inside leser -->
      <verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst>
      <!-- before kennung -->
      <kennung>old</kennung>
      <!-- before name -->
      <name>Old</name>
    </leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
</xwas:vorgang.transportieren.2010>`;
    const result = transform_vorgang_transportieren_2010(xml, {
      nachrichtenkopf_g2g: { leser: { kennung: "psw:new", name: "New" } },
    });
    expect(result).toContain("<!-- inside leser -->");
    expect(result).toContain("<!-- before kennung -->");
    expect(result).toContain("<!-- before name -->");
    expect(result).toContain("<kennung>psw:new</kennung>");
  });

  it("preserves comments outside replaced authority element", () => {
    const ns =
      'xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0"';
    const xml = `<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<xwas:vorgang.transportieren.2010 xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0 ../schemas/V1_0_0/xwasser.xsd" ${ns} produkt="t" produkthersteller="t" produktversion="t" standard="XWasser" test="false" version="1.0.0">
  <nachrichtenkopf.g2g>
    <identifikation.nachricht><nachrichtenUUID>id</nachrichtenUUID></identifikation.nachricht>
    <leser><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>r</kennung><name>R</name></leser>
    <autor><verzeichnisdienst listVersionID=""><code></code></verzeichnisdienst><kennung>a</kennung><name>A</name></autor>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
  <xwas:zusatzinformationen>
    <!-- zusatzinfo comment -->
    <xwas:zustaendigeBehoerdeID>auth-001</xwas:zustaendigeBehoerdeID>
    <xwas:zustaendigeBehoerdeID>Old</xwas:zustaendigeBehoerdeID>
    <!-- after authority -->
  </xwas:zusatzinformationen>
</xwas:vorgang.transportieren.2010>`;
    const result = transform_vorgang_transportieren_2010(xml, {
      zusatzinformationen: { zustaendige_behoerde_id: ["auth-001"] },
    });
    expect(result).toMatch(/xwas:zusatzinformationen/);
    expect(result).not.toContain("Old");
  });

  it("preserves comments when inserting missing leser", () => {
    const ns =
      'xmlns:xwas="https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V1_0_0"';
    const xml = `<?xml version="1.0" encoding="UTF-8"?>
<xwas:vorgang.transportieren.2010 ${ns}>
  <nachrichtenkopf.g2g>
    <!-- existing comment -->
    <identifikation.nachricht><nachrichtenUUID>id</nachrichtenUUID></identifikation.nachricht>
    <!-- after ident -->
    <dvdvDienstkennung>s</dvdvDienstkennung>
  </nachrichtenkopf.g2g>
  <xwas:vorgang><xwas:identifikationVorgang><xwas:vorgangsID>id</xwas:vorgangsID></xwas:identifikationVorgang></xwas:vorgang>
</xwas:vorgang.transportieren.2010>`;
    const result = transform_vorgang_transportieren_2010(xml, {
      nachrichtenkopf_g2g: { leser: { kennung: "psw:l", name: "Leser" } },
    });
    expect(result).toContain("<!-- existing comment -->");
    expect(result).toContain("<!-- after ident -->");
    expect(result).toContain("<kennung>psw:l</kennung>");
  });

  it("preserves whitespace text nodes verbatim through round-trip", () => {
    const xml = sampleXml();
    const result = transform_vorgang_transportieren_2010(xml);
    expect(result).toContain("  <nachrichtenkopf.g2g>");
    expect(result).toContain("    <identifikation.nachricht>");
  });

  it("ds:Signature remains valid after no-op transform", () => {
    const result = transform_vorgang_transportieren_2010(sampleXml());
    expect(result).toContain("ds:Signature");
    expect(result).toContain("ds:SignedInfo");
    expect(result).toContain("ds:DigestValue");
    expect(result).toContain("ds:SignatureValue");
    expect(result).toContain("ds:X509Data");
  });

  it("undefined zusatzinformationen keeps existing block unchanged", () => {
    const result = transform_vorgang_transportieren_2010(sampleXmlWithZi(), {
      nachrichtenkopf_g2g: { leser: { kennung: "psw:new", name: "New" } },
    });
    expect(result).toContain("<kennung>psw:new</kennung>");
    expect(result).toContain("auth-001");
  });
});
