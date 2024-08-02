#![allow(non_snake_case, dead_code)]

use raxb::{XmlDeserialize, XmlSerialize};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

use crate::model::codes::{
    CodeArtObjektType, CodeBetriebszustandType, CodeRahmenTrinkwasserbereitstellungType,
};

use super::{
    anschrift::AnschriftType, behoerde::ZustaendigeBehoerdeType, misc::GeokoordinatenShapthType,
    organisation::OrganisationType, person::NatuerlichePersonType, probe::ProbennahmestelleType,
};

/// Informationen zu einem Auftraggeber [Ergänzende Angaben zu den jeweiligen
/// Informationen aus den Registern von Betreibern/Behörden].
// TODO: implement Box<T>, Arc<T>, Rc<T> for raxb
#[allow(clippy::large_enum_variant)]
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "t", content = "c")]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub enum ArtDerPerson {
    #[xml(ns = b"xwas", name = b"organisation", ty = "child")]
    Organisation(OrganisationType),
    #[xml(ns = b"xwas", name = b"natuerlichePerson")]
    NatuerlichePerson(NatuerlichePersonType),
    #[xml(ns = b"xwas", name = b"zustaendigeBehoerde", ty = "child")]
    ZustaendigeBehoerde(ZustaendigeBehoerdeType),
    #[default]
    #[xml(ns = b"xwas", name = b"unknown")]
    None,
}

/// Klasse zum Transport von Informationen zu einem Betreiber einer WVA [Soweit möglich
/// in Register zu pflegen].
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct BetreiberType {
    #[xml(ns = b"xwas", name = b"betreiberID", ty = "child")]
    pub betreiber_id: String,
    #[xml(ns = b"xwas", name = b"artDerPerson", ty = "child")]
    pub art_der_person: ArtDerPerson, // enum .. heisst eigentlich ArtDerPerson
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
}

/// Klasse für den Transport von Informationen zu einem Objekt.
#[derive(Default, Debug, XmlSerialize, XmlDeserialize, Serialize, Deserialize)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[xml(tns(
    b"xwas",
    b"https://gitlab.opencode.de/akdb/xoev/xwasser/-/raw/main/V0_5_0"
))]
pub struct ObjektType {
    #[xml(ns = b"xwas", name = b"objektID", ty = "child")]
    pub objekt_id: String,
    #[xml(ns = b"xwas", name = b"wasserversorgungsgebiet", ty = "child")]
    pub wasserversorgungsgebiet: Option<String>,
    #[xml(ns = b"xwas", name = b"anschriftObjekt", ty = "child")]
    #[serde(default)]
    pub anschrift_objekt: Vec<AnschriftType>,
    #[xml(ns = b"xwas", name = b"artObjekt", ty = "child")]
    pub art_objekt: CodeArtObjektType,
    #[xml(ns = b"xwas", name = b"nameObjekt", ty = "child")]
    pub name_objekt: String,
    #[xml(ns = b"xwas", name = b"betriebszustandDesObjekts", ty = "child")]
    pub betriebszustand_des_objekts: Option<CodeBetriebszustandType>,
    #[xml(ns = b"xwas", name = b"datumInBetriebnahme", ty = "child")]
    pub datum_in_betriebnahme: Option<String>,
    #[xml(ns = b"xwas", name = b"datumAusserBetriebnahme", ty = "child")]
    pub datum_ausser_betriebnahme: Option<String>,
    #[xml(
        ns = b"xwas",
        name = b"rahmenDerTrinkwasserbereitstellung",
        ty = "child"
    )]
    #[serde(default)]
    pub rahmen_der_trinkwasserbereitstellung: Vec<CodeRahmenTrinkwasserbereitstellungType>,
    #[xml(ns = b"xwas", name = b"geokoordinatenObjekt", ty = "child")]
    pub geokoordinaten_objekt: GeokoordinatenShapthType,
    #[xml(ns = b"xwas", name = b"altID", ty = "child")]
    pub alt_id: Option<String>,
    #[xml(ns = b"xwas", name = b"kommentar", ty = "child")]
    pub kommentar: Option<String>,
    #[xml(ns = b"xwas", name = b"betreiber", ty = "child")]
    #[serde(default)]
    pub betreiber: Vec<BetreiberType>,
    #[xml(ns = b"xwas", name = b"objekt_probennahmestelle", ty = "child")]
    #[serde(default)]
    pub objekt_probennahmestelle: Vec<ProbennahmestelleType>,
    #[xml(ns = b"xwas", name = b"id", ty = "attr")]
    pub id: String,
}
