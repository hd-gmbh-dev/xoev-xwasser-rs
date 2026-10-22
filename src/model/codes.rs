#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use xoev_xwasser_derive::xoev_xwasser_code;

use raxb::de::XmlDeserializeError;
use std::str::FromStr;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct ConstStr;

impl FromStr for ConstStr {
    type Err = XmlDeserializeError;
    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(Self)
    }
}

// #[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
// #[cfg_attr(feature = "wasm", derive(Tsify))]
// #[xml(tns(
//     b"xwas",
//     b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/"
// ))]
// pub struct CodeAufbereitungsstoffDesinfektionsverfahrenType {
//     #[xml(name = b"code", ty = "child")]
//     pub code: String,
//     #[xml(name = b"name", ty = "child")]
//     pub name: Option<String>,
//     #[xml(
//         name = b"listURI",
//         ty = "attr",
//         value = "urn:xoev-de:xwasser:codeliste:aufbereitungsstoffe-desinfektionsverfahren"
//     )]
//     list_uri: ConstStr,
//     #[xml(name = b"listVersionID", ty = "attr")]
//     list_version_id: String,
// }

#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/"
))]
pub struct CodeBehoerdenkennungType {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    pub list_uri: String,
    #[xml(name = b"listVersionID", ty = "attr")]
    pub list_version_id: String,
}

#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_9_2/"
))]
pub struct CodePersonenrolleType {
    #[xml(name = b"code", ty = "child")]
    pub code: String,
    #[xml(name = b"name", ty = "child")]
    pub name: Option<String>,
    #[xml(name = b"listURI", ty = "attr")]
    pub list_uri: String,
    #[xml(name = b"listVersionID", ty = "attr")]
    pub list_version_id: String,
}

/// Type name: Code.Kommunikation.KanalType
/// Eine Liste der Kommunikationsmedien und -kanäle, über die man eine Person oder Institution erreichen kann.
#[xoev_xwasser_code("urn:de:xoev:codeliste:erreichbarkeit", fixed = "3")]
pub struct CodeKommunikationKanalType;

/// Type name: Code.VerzeichnisdienstType
/// Liste der Verzeichnisdienste, in die Behörden / öffentliche Stellen eingetragen sein können
#[xoev_xwasser_code("urn:xoev-de:kosit:codeliste:verzeichnisdienst")]
pub struct CodeVerzeichnisdienstType;

/// Type name: Code.NachrichtentypType
/// Die Liste von eindeutigen Bezeichner für Nachrichtentypen von XWasser.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:nachrichtentyp", fixed = "1", validate)]
pub struct CodeNachrichtentypType;

/// Type name: Code.DatentypType
/// Die Werteliste der W3C-Datentypen (http://www.w3.org/TR/xmlschema, W3C Recommendation 5 April 2012) ergänzt um anySimpleType als Metadatentyp.
#[xoev_xwasser_code("urn:xoev-de:xdomea:codeliste:datentyp", fixed = "1.1")]
pub struct CodeDatentypType;

/// Type name: Code.AGSType
/// Diese Codeliste stellt alle Gemeinden Deutschlands durch den Amtlichen Gemeindeschlüssel (AGS) dar, wie im Gemeindeverzeichnis des Statistischen Bundesamtes enthalten. Darüber hinaus enthält die Codeliste für die Stadtstaaten Hamburg, Bremen und Berlin Einträge für Stadt-/Ortsteile bzw. Stadtbezirke. Diese Einträge sind mit einem entsprechenden Hinweis versehen.
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:ags",
    current = "2026-09-30",
    validate
)]
pub struct CodeAgsType;

/// Type name: Code.AbhilfemassnahmeType
/// Mit dieser Codeliste werden im Kontext von XWasser die Abhilfemaßnahmen definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:abhilfemassnahmen", fixed = "1")]
pub struct CodeAbhilfemassnahmeType;

/// Type name: Code.AmtsspracheEUType
/// Diese Liste beinhaltet die Amtssprachen der Europäischen Union.
#[xoev_xwasser_code("urn:xoev-de:xfahrtenschreiber:codeliste:sprache-eu", fixed = "2")]
pub struct CodeAmtsspracheEuType;

/// Type name: Code.AnlassUntersuchungType
/// Mit dieser Codeliste wird im Kontext von XWasser der Anlasses einer Wasseruntersuchung definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:untersuchung-anlass", current = "3")]
pub struct CodeAnlassUntersuchungType;

/// Type name: Code.AnschrifttypType
/// Codeliste für Anschriftentypen. Diese dienen zur näheren Bestimmung der Art von Anschriften.
#[xoev_xwasser_code("urn:xoev-de:fim:codeliste:xzufi.anschrifttyp", fixed = "2.0")]
pub struct CodeAnschrifttypType;

/// Type name: Code.ArtEntnahmearmaturType
/// In dieser Codeliste werden Arten von Entnahmearmaturen definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-entnahmearmatur", current = "1")]
pub struct CodeArtEntnahmearmaturType;

/// Type name: Code.ArtGesetzlicherVertreterType
/// Diese Codeliste definiert Arten der gesetzlichen Vertreter.
#[xoev_xwasser_code(
    "urn:xoev-de:xunternehmen:codeliste:artgesetzlichervertreter",
    current = "1",
    validate
)]
pub struct CodeArtGesetzlicherVertreterType;

/// Type name: Code.ArtObjektType
/// Diese Codeliste definiert, welcher Art ein Objekt ist.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-objekt", current = "3")]
pub struct CodeArtObjektType;

/// Type name: Code.ArtProbennahmestelleEUType
/// Mit dieser Codeliste wird im Rahman von XWasser die Art der Probennahmestelle gem. den neuen EU Vorgaben spezifiziert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-probennahmestelle-eu", fixed = "1")]
pub struct CodeArtProbennahmestelleEuType;

/// Type name: Code.ArtProbennahmestelleType
/// Mit dieser Codeliste wird im Kontext von XWasser die Art einer Probennahmestelle definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-probennahmestelle", current = "2")]
pub struct CodeArtProbennahmestelleType;

/// Type name: Code.ArtTrinkwasseranlageType
/// Mit dieser Codeliste wird im Rahmen von XWassser die Art einee Trinkwasseranlage gem. TrinkwV beschrieben.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-trinkwasseranlage", fixed = "1")]
pub struct CodeArtTrinkwasseranlageType;

/// Type name: Code.ArtWasserressourceType
/// Mit Hilfe dieser Codeliste wird die Art der Wasserressource angegeben, die in einem Wasserversorgungsgebiete vorliegt.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:art-wasserressource", fixed = "1")]
pub struct CodeArtWasserressourceType;

/// Type name: Code.AufbereitungsstoffDesinfektionsverfahrenType
/// Diese Codeliste dient im Kontext von XWasser zur Definition der Aufbereitungsstoffe und der Desinfektionsverfahren.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:aufbereitungsstoffe-desinfektionsverfahren",
    current = "3",
    validate
)]
pub struct CodeAufbereitungsstoffDesinfektionsverfahrenType;

/// Type name: Code.AuftraggeberartType
/// Mit dieser Codeliste wird im Kontext von XWasser die Art des Auftraggebers definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:auftraggeberart", fixed = "1")]
pub struct CodeAuftraggeberartType;

/// Type name: Code.AuskunftssperreType
/// Mit dieser Schlüsseltabelle werden die Gründe für Auskunfts- oder Übermittlungssperren beschrieben. Siehe Blatt 1801 und Anlage 1 Schlüsseltabelle Auskunfts- und Übermittlungssperren des DSMeld.
#[xoev_xwasser_code("urn:de:dsmeld:schluesseltabelle:auskunftssperre", fixed = "5")]
pub struct CodeAuskunftssperreType;

/// Type name: Code.AusweisdokumenteType
/// Mit dieser Schlüsseltabelle werden die Schlüssel für die Art eines Ausweisdokumentes abgebildet (siehe Anlage 3 des DSMeld).
#[xoev_xwasser_code(
    "urn:de:dsmeld:schluesseltabelle:pass.und.ausweisdokumente",
    current = "5",
    validate
)]
pub struct CodeAusweisdokumenteType;

/// Type name: Code.BetriebszustandType
/// Diese Codeliste dient zur Beschreibung des Betriebszustands eines Objekts.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:betriebszustand", fixed = "1")]
pub struct CodeBetriebszustandType;

/// Type name: Code.BewertungUntersuchungswertType
/// Diese Codeliste dient im Kontext von XWasser zur Bewertung des Untersuchungswerts eines Parameters.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:bewertung-untersuchungswert",
    fixed = "1"
)]
pub struct CodeBewertungUntersuchungswertType;

/// Type name: Code.BezirkType
/// Die Bundesrepublik Deutschland ist ein Zusammenschluss von Bundesländern. Die meisten Bundesländer umfassen mehrere Kreise und Bezirke bzw. Regierungsbezirke. Diese Codeliste stellt alle Bezirke nach dem Gemeindeverzeichnis des Statistischen Bundesamtes dar. Dieser Code ist auch Bestandteil des Amtlichen Gemeindeschlüssels (AGS).
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:bezirk",
    current = "2019-07-31",
    validate
)]
pub struct CodeBezirkType;

/// Type name: Code.BundeslandType
/// Die Bundesrepublik Deutschland ist ein Zusammenschluss von Bundesländern. Die meisten Bundesländer umfassen mehrere Kreise und Bezirke.Diese Codeliste stellt die deutschen Bundesländer nach dem Gemeindeverzeichnis des Statistischen Bundesamtes dar. Dieser Code ist auch Bestandteil des Amtlichen Gemeindeschlüssels (AGS).
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:bundesland",
    fixed = "2010-04-01"
)]
pub struct CodeBundeslandType;

/// Type name: Code.DesinfektionsartType
/// Mit dieser Codeliste wird im Kontext von XWasser definiert, auf welche Art eine dauerhafte Desinfektion durchgeführt wird.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:desinfektionsart", current = "2")]
pub struct CodeDesinfektionsartType;

/// Type name: Code.DokumenttypType
/// Diese Codeliste dient innerhalb von XWasser zur Festlegung des fachlichen Typs eines Dokumentes.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:dokumenttyp", fixed = "1")]
pub struct CodeDokumenttypType;

/// Type name: Code.ErlaeuterungWasserabgabemengeType
/// Diese Codeliste wird im Kontext von XWasser zur Erläuterung der Wasserabgabemenge verwendet.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:erlaeuterung-wasserabgabemenge",
    fixed = "1"
)]
pub struct CodeErlaeuterungWasserabgabemengeType;

/// Type name: Code.FamilienstandBeendigungsgrundType
/// Mit dieser Schlüsseltabelle werden die rechtlichen Gründe der Beendigung oder Nichtigkeit der letzten Ehe oder der letzten Lebenspartnerschaft abgebildet. Siehe Blatt 1405 des DSMeld.
#[xoev_xwasser_code(
    "urn:de:dsmeld:schluesseltabelle:familienstand.beendigungsgrund",
    fixed = "3"
)]
pub struct CodeFamilienstandBeendigungsgrundType;

/// Type name: Code.FamilienstandType
/// Mit dieser Schlüsseltabelle wird der personenstandsrechtliche Familienstand einer Person abgebildet. Siehe Blatt 1401 des DSMeld.
#[xoev_xwasser_code("urn:de:dsmeld:schluesseltabelle:familienstand", fixed = "2")]
pub struct CodeFamilienstandType;

/// Type name: Code.FlockungType
/// Mit dieser Codeliste wird im Kontext von XWasser die Art der Flockung definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:flockung", fixed = "1")]
pub struct CodeFlockungType;

/// Type name: Code.GesamtbewertungType
/// Diese Codeliste dient im Kontext von XWasser der Einschätzung, ob in einem Prüfbericht alle Werte in Ordnung waren oder es Auffälligkeiten/Mängel/Grenzwertüberschreitungen/… von mindestens einem Parameter gab.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:gesamtbewertung", fixed = "2")]
pub struct CodeGesamtbewertungType;

/// Type name: Code.GeschlechtType
/// Diese Codeliste umfasst die im Standard XInneres zur Datenübermittlung genutzten Codes für Geschlechtsangaben gemäß §§ 22 und 45 b PStG.
#[xoev_xwasser_code("urn:xoev-de:xinneres:codeliste:geschlecht", fixed = "1")]
pub struct CodeGeschlechtType;

/// Type name: Code.GrundAusnahmeregelungType
/// Mit dieser Codeliste werden im Kontext von XWasser die Gründen für eine Ausnahmeregelungen gem. den neuen Vorgaben für das EU Berichtsformat festgelegt.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:grund-ausnahmeregelung", fixed = "1")]
pub struct CodeGrundAusnahmeregelungType;

/// Type name: Code.GrundSchliessungWasserversorgungsgebietType
/// Diese Codeliste beinhaltet die Gründe für die Schließung eines Wasserversorgungsgebiets.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:grund-schliessung-wasserversorgungsgebiet",
    fixed = "1"
)]
pub struct CodeGrundSchliessungWasserversorgungsgebietType;

/// Type name: Code.IncidentCategoryType
/// Diese Codeliste definiert die Kategorien eines Vorfalls gem. den Vorgaben für das EU Berichtsformat.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:incident-category", fixed = "1")]
pub struct CodeIncidentCategoryType;

/// Type name: Code.IncidentExceedanceCauseType
/// Mit dieser Codeliste wird im Rahmen von XWasser die Ursache eines Überschreitungsvorfalls gem. den Vorgaben für das EU Berichtsformat definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:incident-exceedance-cause", fixed = "1")]
pub struct CodeIncidentExceedanceCauseType;

/// Type name: Code.KategorieProbennahmestelleType
/// Mit dieser Codeliste wird im Kontext von XWasser die Kategorie der Probennahmestelle definiert.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:kategorie-probennahmestelle",
    fixed = "1"
)]
pub struct CodeKategorieProbennahmestelleType;

/// Type name: Code.KommunikationType
/// Eine Liste der Kommunikationsmedien und -kanäle, über die man eine Person oder Institution erreichen kann.
#[xoev_xwasser_code("urn:de:xoev:codeliste:erreichbarkeit", fixed = "3")]
pub struct CodeKommunikationType;

/// Type name: Code.KreisType
/// Die Bundesrepublik Deutschland ist ein Zusammenschluss von Bundesländern. Die meisten Bundesländer umfassen mehrere Kreise und Bezirke.Diese Codeliste stellt die Kreise der deutschen Bundesländer nach dem Gemeindeverzeichnis des Statistischen Bundesamtes dar. Dieser Code ist auch Bestandteil des Amtlichen Gemeindeschlüssels (AGS).
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:kreis",
    current = "2026-01-31",
    validate
)]
pub struct CodeKreisType;

/// Type name: Code.LaenderkennzeichenType
/// Diese Codeliste wird im Rahmen von XWasser zur Kennzeichnung des Bundeslands, des Bundesministeriums der Verteidigung bzw. des Eisenbahnbundesamts verwendet.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:laenderkennzeichen", fixed = "1")]
pub struct CodeLaenderkennzeichenType;

/// Type name: Code.MassnahmeType
/// Mit dieser Codeliste werden im Kontext von XWasser die Maßnahmen definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:massnahmen", fixed = "1")]
pub struct CodeMassnahmeType;

/// Type name: Code.MediumType
/// Diese Codeliste dient im Kontext von XWasser der Angabe des Mediums einer Probennahme.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:medium", current = "3")]
pub struct CodeMediumType;

/// Type name: Code.MesswertergaenzungType
/// Diese Codeliste wird im Rahmen von XWasser für die Ergänzung von Messwertangaben verwendet.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:messwertergaenzung", fixed = "2")]
pub struct CodeMesswertergaenzungType;

/// Type name: Code.NachweisartType
/// Mit dieser Codeliste wird im Kontext von XWasser für bestimmte Parameter (z. B. Acrylamid, Epichlorhydrin oder Vinylchlorid) definiert, wie der Nachweis erbracht wird.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:nachweisart", fixed = "1")]
pub struct CodeNachweisartType;

/// Type name: Code.NamensartType
/// Liste ausländischer Namensformen
#[xoev_xwasser_code("urn:xpersonenstand:schluesseltabelle:namensart", fixed = "1")]
pub struct CodeNamensartType;

/// Type name: Code.ParameterauspraegungType
/// Diese Codeliste definiert im Rahmen von XWasser die speziellen Ausprägungen von qualitativen Parametern.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:parameterauspraegung", current = "5")]
pub struct CodeParameterauspraegungType;

/// Type name: Code.PraefixType
/// Diese Codeliste enthält alle Präfixe, welche im DVDV zum Einsatz kommen.
#[xoev_xwasser_code(
    "urn:xoev-de:bund:bmi:bit:codeliste:dvdv.praefix",
    current = "51",
    validate
)]
pub struct CodePraefixType;

/// Type name: Code.ProbenbewertungType
/// Mit dieser Codeliste wird im Kontext von XWasser definiert, ob in einer Wasserprobe alle Werte in Ordnung waren oder es Auffälligkeiten/Mängel/Grenzwertüberschreitungen/... von mindestens einem Parameter gab.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:probenbewertung", fixed = "1")]
pub struct CodeProbenbewertungType;

/// Type name: Code.ProbenentnahmegeraetType
/// Mit dieser Codeliste wird im Kontext von XWasser das Gerät definiert, mit dem eine Wasserprobe entnommen wird.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:probenentnahmegeraet", current = "1")]
pub struct CodeProbenentnahmegeraetType;

/// Type name: Code.ProbengefaessType
/// Mit dieser Codeliste wird im Kontext von XWasser das Gerät angegeben, mit dem eine Wasserprobe transportiert/gelagert wird.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:probengefaess", current = "1")]
pub struct CodeProbengefaessType;

/// Type name: Code.ProbennahmeverfahrenType
/// Diese Codeliste dient im Kontext von XWasser zur Angabe des Verfahrens mit dem eine Probe entnommen wird
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:probennahmeverfahren", current = "4")]
pub struct CodeProbennahmeverfahrenType;

/// Type name: Code.ProbennahmezeitraumType
/// Mit dieser Codeliste wird der Probennahmezeitraum definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:probennahmezeitraum", fixed = "1")]
pub struct CodeProbennahmezeitraumType;

/// Type name: Code.RahmenTrinkwasserbereitstellungType
/// Diese Codeliste definiert im Kontext von XWasser den Rahmen der Trinkwasserbereitstellung von einem Objekt.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:rahmen-trinkwasserbereitstellung",
    fixed = "1"
)]
pub struct CodeRahmenTrinkwasserbereitstellungType;

/// Type name: Code.RechtsformenType
/// Die Codeliste urn:xoev-de:xunternehmen:codeliste:rechtsformen bildet ab Version 2 fachbereichsübergreifend Anforderungen an die Codierung von Rechtsforminformationen ab und ermöglicht deren bereichsübergreifend einheitliche Codierung und Übermittlung. Aktuell werden die Anforderungen aus dem Grundinformationsdienst GINSTER der Steuerverwaltung und damit auch der Codierung im ELSTER-Unternehmenskonto, aus dem Registerportal der Handels-, Genossenschafts-, Partnerschafts- und Vereinsregister, dem Standard XGewerbeanzeige / XGewerbeordnung und damit auch dem Unternehmerverzeichnis der gesetzlichen Unfallversicherung umgesetzt. Auf diese Weise können beispielweise Rechtsforminformationen ohne semantische Brüche aus dem ELSTER-Unternehmenskonto über einen Online-Dienst zur Gewerbeanmeldung an die zuständige Gewerbebehörde übermittelt werden und dort mit Daten aus dem Registerportal der Justiz und (perspektivisch) mit dem Basisregister gemäß UBRegG zusammengeführt werden. Seit Version 2 sind die Codes sechsstellig und hierarchisch gegliedert, um unterschiedliche Granularitäten der Rechtsformdifferenzierung zu unterstützen und Codes einfacher zueinander in Bezug setzen zu können. Die Version 2 der Rechtsformcodierung ist vollständig rückwärtskompatibel zur Version 1 in dem Sinne, dass eine Eins-zu-eins-Umschlüsselung der bisherigen Codes erfolgen kann.
#[xoev_xwasser_code(
    "urn:xoev-de:xunternehmen:codeliste:rechtsformen",
    current = "2.4",
    validate
)]
pub struct CodeRechtsformenType;

/// Type name: Code.RegionalschluesselType
/// Diese Codeliste stellt alle Gemeinden Deutschlands durch den Amtlichen Regionalschlüssel (ARS) dar, wie im Gemeindeverzeichnis des Statistischen Bundesamtes enthalten. Darüber hinaus enthält die Codeliste für die Stadtstaaten Hamburg, Bremen und Berlin Einträge für Stadt-/Ortsteile bzw. Stadtbezirke. Diese Einträge sind mit einem entsprechenden Hinweis versehen.
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:rs",
    current = "2026-09-30",
    validate
)]
pub struct CodeRegionalschluesselType;

/// Type name: Code.SHAPTH-Parameter-EinheitType
/// Mit dieser Codeliste wird im Kontext von XWassser die Einheit eines SHAPTH Parameters angegeben,
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:shapth-parameter-einheit",
    current = "2"
)]
pub struct CodeShapthParameterEinheitType;

/// Type name: Code.SHAPTH-ParameterType
/// Mit dieser Codeliste werden die Parameter definiert, die via SHAPTH übermittelt werden können.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:shapth-parameter",
    current = "5",
    validate
)]
pub struct CodeShapthParameterType;

/// Type name: Code.SpracheType
/// Der Sprachenkatalog basiert auf der Norm ISO 639 und enthält sowohl Werte der Teilnorm 2 als auch der Teilnorm 3. Er dient unterschiedlichen Verwendungszwecken und wird daher bedarfsgereicht angepasst /erweitert. Es ist die jeweils aktuelle Version zu verwenden, die im XRepository veröffentlicht ist.
#[xoev_xwasser_code(
    "urn:xoev-de:xauslaender:codeliste:sprachenkatalog",
    current = "11",
    validate
)]
pub struct CodeSpracheType;

/// Type name: Code.StaatType
/// Die Codeliste Staat (eigenständige, von Deutschland diplomatisch anerkannte derzeitige Staaten). Tabelle von Staaten und Staatsangehörigkeiten. Enthalten sind alle Staaten im vollen politischen Sinne. Entspricht inhaltlich dem "Verzeichnis der Staatennamen für den amtlichen Gebrauch" des Auswärtigen Amtes. Enthalten sind z.B. Einträge für: Frankreich, Italien, Vereinigtes Königreich; nicht aber für: Französisch-Guayana, die britischen Jungferninseln oder Jersey. Neben den amtlichen Bezeichnungen (Kurzform und Vollform) und der Staatsangehörigkeit als Adjektiv/Adverb ist auch der jeweilige Suchbegriff aus dem Länderverzeichnis des Auswärtigen Amtes aufgeführt (der Suchbegriff ist eine griffige Bezeichnung des Staates). Die Angaben umfassen zudem den numerischen Destatis-Code, den 2- und 3-stelligen alphabetischen ISO 3166-1 Code sowie ggf. das Datum der Selbständigkeit bzw. Gründung nach den Angaben des Auswärtigen Amtes.
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:staat",
    current = "2026-08-05",
    validate
)]
pub struct CodeStaatType;

/// Type name: Code.StaatsangehoerigkeitType
/// Codeliste Staatsangehörigkeit. Tabelle von Staaten und Staatsangehörigkeiten. Enthält alle Einträge der Codeliste Staat (eigenständige, von Deutschland diplomatisch anerkannte derzeitige Staaten) und zusätzlich Einträge für ehemalige Staaten sowie Ersatzwerte. Die Liste enthält auch einen Eintrag für die Palästinensischen Gebiete. Enthalten sind alle Staaten im vollen politischen Sinne, z.B. Frankreich, Italien, Vereinigtes Königreich, nicht aber: britische Jungferninseln, Jersey sowie alle früheren Staaten seit 1970, z.B. Sowjetunion. Neben den amtlichen Bezeichnungen (Kurzform und Vollform) und der Staatsangehörigkeit als Adjektiv/Adverb ist auch der jeweilige ist auch der jeweilige Suchbegriff aus dem Länderverzeichnis des Auswärtigen Amtes aufgeführt (der Suchbegriff ist eine griffige Bezeichnung des Staates). Die Angaben umfassen zudem den numerische Destatis-Code für Staatsangehörigkeit und Staat den 2- und 3-stelligen alphabetischen ISO 3166-1 Code sowie ggf. das Datum der Selbständigkeit bzw. Gründungsdatum nach den Angaben des Auswärtigen Amtes und das Auflösungs- bzw. Enddatum als berechnete Angabe aus dem Gründungsdatum des/der Nachfolgestaaten.
#[xoev_xwasser_code(
    "urn:de:bund:destatis:bevoelkerungsstatistik:schluessel:staatsangehoerigkeit",
    current = "2026-08-05",
    validate
)]
pub struct CodeStaatsangehoerigkeitType;

/// Type name: Code.StatusFachlichType
/// Mit dieser Codeliste wird der fachliche Status definiert.
#[xoev_xwasser_code("urn:xoev-de:xewaffe:codeliste:statusfachlich", fixed = "1")]
pub struct CodeStatusFachlichType;

/// Type name: Code.StatusTechnischType
/// Mit dieser Codeliste wird der technische Status definiert.
#[xoev_xwasser_code("urn:xoev-de:xewaffe:codeliste:statustechnisch", fixed = "1")]
pub struct CodeStatusTechnischType;

/// Type name: Code.StatusUntersuchungsplanType
/// Mit dieser Codeliste wird im Kontext von XWasser der Status des Untersuchungsplans definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:status-untersuchungsplan", fixed = "2")]
pub struct CodeStatusUntersuchungsplanType;

/// Type name: Code.UeberschreitungsursacheType
/// Beschreibung der Codeliste.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:ueberschreitungsursache", fixed = "1")]
pub struct CodeUeberschreitungsursacheType;

/// Type name: Code.UeberwachungAufbereitungType
/// Mit dieser Codeliste wird im Kontext von XWasser definiert, wie die Überwachung der Trinkwasseraufbereitung erfolgt.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:ueberwachung-aufbereitung", fixed = "2")]
pub struct CodeUeberwachungAufbereitungType;

/// Type name: Code.UnterkategorieProbennahmestelleType
/// In dieser Codeliste werden Unterkategorien von Probennahmestellen definiert.
#[xoev_xwasser_code(
    "urn:xoev-de:xwasser:codeliste:unterkategorie-probennahmestelle",
    current = "1"
)]
pub struct CodeUnterkategorieProbennahmestelleType;

/// Type name: Code.UntersuchungsstelleType
/// Beschreibung der Codeliste. Diese Codeliste definiert alle im Kontext von XWasser zugelassenen Untersuchungsstellen (Diese Codeliste soll langfristig über das Register für zugelassene Untersuchungsstellen abgelöst werden).
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:untersuchungsstelle", fixed = "1")]
pub struct CodeUntersuchungsstelleType;

/// Type name: Code.UntersuchungsverfahrenType
/// Mit dieser Codeliste wird im Kontext von XWasser das Untersuchungsverfahren von Parametern im Labor festgelegt.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:untersuchungsverfahren", current = "3")]
pub struct CodeUntersuchungsverfahrenType;

/// Type name: Code.VertretungsartType
/// Mit dieser Schlüsselliste wird Rolle einer Person definiert, in der sie eine nichtnatürliche Person vertritt.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:vertretungsart", fixed = "1")]
pub struct CodeVertretungsartType;

/// Type name: Code.VorfallkategorieType
/// Mit dieser Codeliste wird im Kontext von XWassser dier Kategorie eines Vorfalls definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:vorfallkategorie", fixed = "1")]
pub struct CodeVorfallkategorieType;

/// Type name: Code.WVAType
/// Mit dieser Codeliste wird im Kontext von XWasser die Art der Wassserversorgungsanlage definiert.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:wva", fixed = "1")]
pub struct CodeWvaType;

/// Type name: Code.WaehrungType
/// Die Codeliste basiert auf dem Standard "Currency codes" der International Organization for Standardization (ISO). Die vorliegende Version der Codeliste enthält alle Codelisteneinträge, die mit einem listenweit eindeutigen Eintrag in den Feldern "Currency" und "Alphabetic Code" geführt werden. Sie ist konform zur EN16931-1 und zu dem darauf basierenden Standard XRechnung sowie zum Standard XBestellung und der zugrundeliegenden Peppol BIS Order only.
#[xoev_xwasser_code("urn:xoev-de:kosit:codeliste:currency-codes", fixed = "3")]
pub struct CodeWaehrungType;

/// Type name: Code.WasserversorgungsgebietType
/// Diese Codeliste enthält die Namen der Wasserversorgungsgebiete (Die Codeliste soll langfristig über das Register für Wasserversorgungegebiete abgelöst werden).
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:wasserversorgungsgebiet", validate)]
pub struct CodeWasserversorgungsgebietType;

/// Type name: Code.RueckweisungsgrundType
/// Die Codeliste urn:xoev-de:xgewerbeordnung:codeliste:rueckweisungsgruende führt mögliche Gründe für eine Rücksendung einer Nachricht an den Absender auf. Mit dem Präfix des jeweiligen Schlüssels wird folgende Systematik festgelegt: T (Transportproblem), X (formales Problem mit XML), V (Versionsproblem), S (nicht spezifikationskonform).
#[xoev_xwasser_code(
    "urn:xoev-de:xgewerbeordnung:codeliste:rueckweisungsgruende",
    fixed = "1"
)]
pub struct CodeRueckweisungsgrundType;

/// Type name: Code.FormatAlternativeIDType
/// Die Codeliste gibt ID-Formate an, welche parallel zu SHAPTH bestehen können. Beispielsweise aus den Landesschnittstellen wie SEBAM, oder TEIS/ZTEIS, welche durch SHAPTH ersetzt werden.
#[xoev_xwasser_code("urn:xoev-de:xwasser:codeliste:format-alternative-id", current = "1")]
pub struct CodeFormatAlternativeIDType;
