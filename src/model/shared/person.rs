#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

use crate::model::codes::deserialize_list_of_codes;
use crate::model::codes::deserialize_optional_code;
use xoev_xwasser_derive::XWasserValidate;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::{
    TNS,
    model::codes::{
        CodeAuskunftssperreType, CodeAusweisdokumenteType, CodeFamilienstandBeendigungsgrundType,
        CodeFamilienstandType, CodeGeschlechtType, CodeNamensartType, CodePersonenrolleType,
        CodeRechtsformenType, CodeVertretungsartType,
    },
};

use super::{
    anschrift::AnschriftType,
    behoerde::BehoerdeType,
    kommunikation::KommunikationType,
    misc::IdentifikationType,
    sprache::SpracheType,
    staat::{StaatType, StaatsangehoerigkeitType},
    zeitraum::ZeitraumType, // AnschriftType, AuskunftssperreType, DoktorgradType, GeburtType, NameNatuerlichePersonType, VertreterBevollmaechtigterType
};

/// Der AllgemeineName dient der Darstellung von Vor- und Nachnamen und fasst deren
/// gemeinsame Eigenschaften zusammen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AllgemeinerNameType {
    #[xml(ns = b"xwas", name = b"name", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub name: Option<String>,
    #[xml(ns = b"xwas", name = b"nichtVorhanden", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub nicht_vorhanden: Option<bool>,
    #[xml(ns = b"xwas", name = b"namensart", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub namensart: Option<CodeNamensartType>,
    #[xml(ns = b"xwas", name = b"alternativeRepraesentation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub alternative_repraesentation: Vec<AlternativeRepraesentationType>,
}

/// Die "AlternativeRepraesentation" beinhaltet das mit ihm verbundene Objekt in einer
/// alternativen Form, die einer festgelegten Konvention folgt. Das Element kann Inhalte
/// anderer Elemente des verbundenen Objekts beinhalten. Die in der Komponente
/// "AlternativeRepraesentation" übermittelten Informationen müssen redundant zu den
/// anderen Elementen des mit ihm verbundenen Objekts sein. Eine
/// "AlternativeRepraesentation" kann auch eine multimediale Abbildung des Objektes
/// darstellen. Hierzu zählen beispielsweise Logos oder Bilder. Beispiel: Ein Beispiel
/// für die Verwendung einer alternativen Repraesentation ist die Übermittlung von Namen.
/// Der Name "Andrè Müller" würde nach ICAO-Standard, in dem keine Umlaute erlaubt sind,
/// daher alternativ als "ANDRE MUELLER" übertragen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AlternativeRepraesentationType {
    #[xml(ns = b"xwas", name = b"repraesentation", ty = "child")]
    pub repraesentation: String,
    #[xml(ns = b"xwas", name = b"algorithmus", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub algorithmus: Option<String>,
    #[xml(ns = b"xwas", name = b"hinweis", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub hinweis: Option<String>,
}

/// Der Name einer Person ist eine Benennung dieser Person, die dazu dient, diese Person
/// von anderen Personen zu unterscheiden.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct NameNatuerlichePersonType {
    #[xml(ns = b"xwas", name = b"titel", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub titel: Option<String>,
    #[xml(ns = b"xwas", name = b"anrede", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anrede: Vec<String>,
    #[xml(ns = b"xwas", name = b"namenssuffix", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub namenssuffix: Vec<String>,
    #[xml(ns = b"xwas", name = b"familienname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub familienname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"ehename", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ehename: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"lebenspartnerschaftsname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub lebenspartnerschaftsname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"geburtsname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub geburtsname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"fruehererFamilienname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub frueherer_familienname: Vec<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"vorname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub vorname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"rufname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub rufname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"fruehererVorname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub frueherer_vorname: Vec<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"alternativeRepraesentation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub alternative_repraesentation: Vec<AlternativeRepraesentationType>,
    #[xml(ns = b"xwas", name = b"ordensname", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ordensname: Option<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"kuenstlername", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kuenstlername: Vec<AllgemeinerNameType>,
    #[xml(ns = b"xwas", name = b"weitererName", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub weiterer_name: Vec<AllgemeinerNameType>,
}

/// Die Auskunftssperre beschränkt die Weitergabe von Informationen an Dritte.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AuskunftssperreType {
    #[xml(ns = b"xwas", name = b"grund", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub grund: Option<CodeAuskunftssperreType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
}

/// Unter "Geburt" werden geburtsbezogene Informationen zusammengefasst.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct GeburtType {
    #[xml(ns = b"xwas", name = b"datum", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub datum: Option<String>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"geburtsort", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub geburtsort: Option<AnschriftType>,
}

/// Dieser Datentyp erlaubt die Angabe von Doktorgraden. Es sind nur diejenigen
/// Doktorgrade anzugeben, die in Pässe eingetragen werden dürfen. Sind mehrere
/// Doktorgrade anzugeben, so sind sie durch ein Leerzeichen zu trennen. Zulässig sind
/// derzeit: „DR.“, „Dr.“, „DR.HC.“, „Dr.hc.“, „Dr.EH.“ und „Dr.eh.“.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct DoktorgradType {
    // hier gibt es eine striction mit max length 120, wie umsetzen ?
    #[xml(ns = b"xwas", name = b"bezeichnung", ty = "child")]
    pub bezeichnung: String,
}

/// Mit diesem Datentyp wird ein gesetzlicher Vertreter oder ein Bevollmächtigter einer
/// nichtnatürlichen Person abgebildet.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct VertreterBevollmaechtigterType {
    #[xml(ns = b"xwas", name = b"vertreterBevollmaechtigterID", ty = "child")]
    pub vertreter_bevollmaechtigter_id: String,
    #[xml(ns = b"xwas", name = b"artVertreter", ty = "child")]
    pub art_vertreter: CodeVertretungsartType,
}

/// Hier werden Angaben zum Familienstand einer natürlichen Person zusammengefasst.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct FamilienstandType {
    #[xml(ns = b"xwas", name = b"familienstand", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub familienstand: Option<CodeFamilienstandType>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"grund", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub grund: Option<CodeFamilienstandBeendigungsgrundType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"behoerde", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub behoerde: Option<BehoerdeType>,
}

/// Ein Ausweis ist eine öffentliche oder private Urkunde, die die Identität des Inhabers
/// schriftlich und offiziell darstellt. Er enthält meist persönliche Daten.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct AusweisdokumentType {
    #[xml(ns = b"xwas", name = b"ausweisart", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ausweisart: Option<CodeAusweisdokumenteType>,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
    #[xml(ns = b"xwas", name = b"ausweisID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ausweis_id: Option<IdentifikationType>,
    // TODO: check BehoerdeType
    #[xml(ns = b"xwas", name = b"ausstellendeBehoerde", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ausstellende_behoerde: Option<BehoerdeType>,
    #[xml(ns = b"xwas", name = b"ausstellenderStaat", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub ausstellender_staat: Option<StaatType>,
}

/// Dieses Objekt umfasst die allgemeinen Angaben zu einer juristischen Person des
/// privaten oder öffentlichen Rechts. Unter juristischen Personen werden sowohl die
/// Körperschaften des Privatrechts (Gesellschaft mit beschränkter Haftung,
/// Aktiengesellschaft, Kommanditgesellschaft auf Aktien, eingetragener Verein,
/// Genossenschaft) als auch öffentlich-rechtliche Körperschaften (Gebietskörperschaften,
/// Selbstverwaltungskörperschaften, sonstige Körperschaften des öffentlichen Rechts)
/// erfasst. Die konkrete Rechtsform der juristischen Person kann mittels einer Codeliste
/// angegeben werden.
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct JuristischePersonType {
    #[xml(
        ns = b"xwas",
        name = b"bundeseinheitlicheWirtschaftsnummer",
        ty = "child"
    )]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub bundeseinheitliche_wirtschaftsnummer: Option<String>,
    #[xml(ns = b"xwas", name = b"rechtsform", ty = "child")]
    #[serde(deserialize_with = "deserialize_optional_code")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub rechtsform: Option<CodeRechtsformenType>,
    #[xml(ns = b"xwas", name = b"eingetragenerName", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub eingetragener_name: Option<String>,
    #[xml(ns = b"xwas", name = b"eintragung", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub eintragung: Option<String>,
    #[xml(ns = b"xwas", name = b"geschaeftsbezeichnung", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub geschaeftsbezeichnung: Option<String>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"sitz", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub sitz: Option<String>,
    #[xml(ns = b"xwas", name = b"effektiverVerwaltungssitz", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub effektiver_verwaltungssitz: Option<String>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"vertreterBevollmaechtigter", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub vertreter_bevollmaechtigter: Vec<VertreterBevollmaechtigterType>,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}

/// Die Komponente "Geschlecht" dient der Repräsentation des biologischen Geschlechts.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct GeschlechtType {
    #[xml(ns = b"xwas", name = b"geschlecht", ty = "child")]
    pub geschlecht: CodeGeschlechtType,
    #[xml(ns = b"xwas", name = b"gueltigkeit", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub gueltigkeit: Option<ZeitraumType>,
}

/// Eine natürliche Person ist der Mensch in seiner Rolle als Rechtssubjekt, d. h. als
/// Träger von Rechten und Pflichten. Mit der Vollendung seiner Geburt wird ein Mensch
/// rechtsfähig und damit zu einer natürlichen Person (§ 1 BGB). Der Mensch verliert
/// seine Rechtsfähigkeit mit dem Tod. Rechtssubjekte, die keine natürlichen Personen
/// sind, nennt man juristische Personen.
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct NatuerlichePersonType {
    #[xml(ns = b"xwas", name = b"auskunftssperre", ty = "child")]
    pub auskunftssperre: Vec<AuskunftssperreType>,
    #[xml(ns = b"xwas", name = b"nameNatuerlichePerson", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub name_natuerliche_person: Option<NameNatuerlichePersonType>,
    #[xml(ns = b"xwas", name = b"familienstand", ty = "child")]
    pub familienstand: Vec<FamilienstandType>,
    #[xml(ns = b"xwas", name = b"geburt", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub geburt: Option<GeburtType>,
    #[xml(ns = b"xwas", name = b"doktorgrad", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub doktorgrad: Option<String>,
    #[xml(ns = b"xwas", name = b"staatsangehoerigkeit", ty = "child")]
    pub staatsangehoerigkeit: Vec<StaatsangehoerigkeitType>,
    #[xml(ns = b"xwas", name = b"ausweisdokument", ty = "child")]
    pub ausweisdokument: Vec<AusweisdokumentType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"geschlecht", ty = "child")]
    pub geschlecht: Vec<GeschlechtType>,
    #[xml(ns = b"xwas", name = b"identifikationsnummer", ty = "child")]
    pub identifikationsnummer: Vec<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"muttersprache", ty = "child")]
    pub muttersprache: Vec<SpracheType>,
    #[xml(ns = b"xwas", name = b"fremdsprache", ty = "child")]
    pub fremdsprache: Vec<SpracheType>,
    #[xml(ns = b"xwas", name = b"vertreterBevollmaechtigter", ty = "child")]
    pub vertreter_bevollmaechtigter: Vec<VertreterBevollmaechtigterType>,
    #[xml(name = b"id", ty = "attr")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub id: Option<String>,
}

// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(
    Clone, Default, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize,
)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "t", content = "c")]
#[xml(tns(b"xwas", TNS))]
pub enum Person {
    #[xml(ns = b"xwas", name = b"natuerlichePerson")]
    NatuerlichePerson(NatuerlichePersonType),
    #[xml(ns = b"xwas", name = b"juristischePerson", ty = "child")]
    JuristischePerson(JuristischePersonType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}

/// Mit diesem Datentyp wird eine allgemeine Person, natürlich oder nichtnatürlich,
/// spezifiziert
/// dieser Datentype wird bisher nirgends verwendet
#[derive(Clone, Debug, XmlSerialize, XmlDeserialize, XWasserValidate, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(b"xwas", TNS))]
pub struct PersonType {
    #[xml(ns = b"xwas", name = b"personRolle", ty = "child")]
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_list_of_codes")]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub person_rolle: Vec<CodePersonenrolleType>,
    #[xml(ns = b"xwas", name = b"dokumentReferenzID", ty = "child")]
    #[serde(default)]
    #[cfg_attr(feature = "wasm", tsify(optional))]
    pub dokument_referenz_id: Vec<String>,
    #[xml(ns = b"xwas", name = b"person", ty = "child")]
    pub person: Person,
    #[xml(name = b"id", ty = "attr")]
    pub id: String,
}
