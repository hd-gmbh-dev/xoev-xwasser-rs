#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify_next::Tsify;

#[cfg(feature = "builder")]
use typed_builder::TypedBuilder;

use crate::model::codes::{
    CodeAgsType, CodeBehoerdenkennungType, CodeBezirkType, CodeBundeslandType, CodeKreisType,
    CodeLaenderkennzeichenType, CodePraefixType, CodeRegionalschluesselType,
};

use super::{
    anschrift::AnschriftType,
    kommunikation::KommunikationType,
    misc::IdentifikationType,
    organisation::{NameOrganisationType, OrganisationseinheitType},
    staat::StaatType,
    xoev::XWasserXoevCode,
};

/// Klasse für den Transport von Informationen zu einer Zuständigen Behörde [Ergänzende
/// Angaben zu einer im Register Behörden gepflegte Behörde].
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct ZustaendigeBehoerdeType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"typ", ty = "child")]
    pub typ: Option<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"behoerdenkennung", ty = "child")]
    pub behoerdenkennung: Option<BehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenidentifikation", ty = "child")]
    pub behoerdenidentifikation: Option<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenname", ty = "child")]
    #[serde(default)]
    pub behoerdenname: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"nachgeordneteBehoerde", ty = "child")]
    #[serde(default)]
    pub nachgeordnete_behoerde: Vec<BehoerdeType>,
    #[xml(
        ns = b"xwas",
        name = b"verwaltungspolitischeZustaendigkeit",
        ty = "child"
    )]
    #[serde(default)]
    pub verwaltungspolitische_zustaendigkeit: Vec<VerwaltungspolitischeKodierungType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"organisationsstruktur", ty = "child")]
    #[serde(default)]
    pub organisationsstruktur: Vec<OrganisationseinheitType>,
    #[xml(ns = b"xwas", name = b"anlageNachTrinkwVID", ty = "child")]
    #[serde(default)]
    pub anlage_nach_trinkw_vid: Vec<String>,
    #[xml(ns = b"xwas", name = b"laenderkuerzel", ty = "child")]
    pub laenderkuerzel: CodeLaenderkennzeichenType,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}

/// Die Behoerdenkennung fasst die Elemente zusammen, über die eine Behörde identifiziert
/// werden kann. Die "Behoerdenkennung" ist prioritär zur Übermittlung der im DVDV
/// verzeichneten Behördenschlüssel vorgesehen, kann aber auch für andere
/// Behördenkennungen, bspw. die BKZ der Justizverwaltung eingesetzt werden. Eine
/// Behördenkennung im DVDV besteht aus einem Präfix und der eigentlichen Kennung. Die
/// Codelisten für die Präfixe sowie die Kennungen pro Präfix werden durch die
/// koordinierende Stelle für das DVDV verwaltet. Anmerkung: Beispiel für die
/// Übermittlung einer Behördenkennung des DVDV: Bei einer Identifikation von Behörden
/// auf kommunaler Ebene anhand des amtlichen Gemeindeschlüssels (AGS) der Gemeinde, für
/// die die Behörde zuständig ist, lautet der Präfix "ags:", die Kennung ist dann der AGS
/// der jeweiligen Gemeinde.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct BehoerdenkennungType {
    #[xml(ns = b"xwas", name = b"kennung", ty = "child")]
    #[serde(default)]
    pub kennung: Vec<CodeBehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"praefix", ty = "child")]
    #[serde(default)]
    pub praefix: Vec<CodePraefixType>,
}

/// Eine Behörde ist ein Organ eines Verwaltungsträgers, das gegenüber dem
/// Verwaltungsträger berechtigt ist, mit Außenwirkung Aufgaben öffentlichen Handelns
/// (insbes. der Erlass von Verwaltungsakten und das Schließen öffentlich-rechtlicher
/// Verträge) wahrzunehmen.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct BehoerdeType {
    #[xml(ns = b"xwas", name = b"id", ty = "child")]
    pub id: Option<String>,
    #[xml(ns = b"xwas", name = b"typ", ty = "child")]
    pub typ: Option<XWasserXoevCode>,
    #[xml(ns = b"xwas", name = b"zusatz", ty = "child")]
    pub zusatz: Option<String>,
    #[xml(ns = b"xwas", name = b"behoerdenkennung", ty = "child")]
    pub behoerdenkennung: Option<BehoerdenkennungType>,
    #[xml(ns = b"xwas", name = b"kommunikation", ty = "child")]
    #[serde(default)]
    pub kommunikation: Vec<KommunikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenidentifikation", ty = "child")]
    pub behoerdenidentifikation: Option<IdentifikationType>,
    #[xml(ns = b"xwas", name = b"behoerdenname", ty = "child")]
    #[serde(default)]
    pub behoerdenname: Option<NameOrganisationType>,
    #[xml(ns = b"xwas", name = b"nachgeordneteBehoerde", ty = "child")]
    #[serde(default)]
    pub nachgeordnete_behoerde: Vec<BehoerdeType>,
    #[xml(
        ns = b"xwas",
        name = b"verwaltungspolitischeZustaendigkeit",
        ty = "child"
    )]
    #[serde(default)]
    pub verwaltungspolitische_zustaendigkeit: Vec<VerwaltungspolitischeKodierungType>,
    #[xml(ns = b"xwas", name = b"anschrift", ty = "child")]
    #[serde(default)]
    pub anschrift: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"organisationsstruktur", ty = "child")]
    #[serde(default)]
    pub organisationsstruktur: Vec<OrganisationseinheitType>,
}

/// Die Komponente "VerwaltungspolitischeKodierung" beinhaltet Information, die eine
/// verwaltungspolitisch eindeutige Zuordnung ermöglichen.
#[derive(Clone, Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "builder", derive(TypedBuilder))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/develop/V0_6_0/"
))]
pub struct VerwaltungspolitischeKodierungType {
    #[xml(ns = b"xwas", name = b"kreis", ty = "child")]
    pub kreis: Option<CodeKreisType>,
    #[xml(ns = b"xwas", name = b"bezirk", ty = "child")]
    pub bezirk: Option<CodeBezirkType>,
    #[xml(ns = b"xwas", name = b"bundesland", ty = "child")]
    pub bundesland: Option<CodeBundeslandType>,
    #[xml(ns = b"xwas", name = b"gemeindeschluessel", ty = "child")]
    pub gemeindeschluessel: Option<CodeAgsType>,
    #[xml(ns = b"xwas", name = b"regionalschluessel", ty = "child")]
    pub regionalschluessel: Option<CodeRegionalschluesselType>,
    #[xml(ns = b"xwas", name = b"nation", ty = "child")]
    pub nation: Option<StaatType>,
}
