use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use azure_data_cosmos::prelude::*;
use azure_core::Context;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amendment {
    pub uid: String,
    pub chronotag: String,
    pub legislature: String,
    pub identification: Identification,
    pub examen_ref: String,
    pub texte_legislatif_ref: String,
    pub tri_amendement: String,
    pub cardinalite_amdt_multiples: String,
    pub pointeur_fragment_texte: PointeurFragmentTexte,
    pub corps: Corps,
    pub cycle_de_vie: CycleDeVie,
    pub representations: Representations,
    pub article99: String,
    pub accord_gouvernement_depot_hors_delai: String,
    pub id: String,
    pub sort: String,
    pub date_depot: String,
    pub date_publication: String,
    pub date_sort: String,
    pub cosignataires: Vec<Value>,
    #[serde(rename = "cosignataires_libelle")]
    pub cosignataires_libelle: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identification {
    pub numero_long: String,
    pub numero_ordre_depot: String,
    pub prefixe_organe_examen: String,
    pub numero_rect: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointeurFragmentTexte {
    pub division: Division,
    pub amendement_standard: AmendementStandard,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Division {
    pub titre: String,
    pub article_designation_courte: String,
    pub article_designation: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "avant_A_Apres")]
    pub avant_a_apres: String,
    pub article_additionnel: String,
    pub chapitre_additionnel: String,
    pub url_division_texte_vise: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendementStandard {
    pub alinea: Alinea,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alinea {
    pub numero: String,
    pub alinea_designation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Corps {
    pub contenu_auteur: ContenuAuteur,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContenuAuteur {
    pub dispositif: String,
    pub expose_sommaire: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CycleDeVie {
    pub date_depot: String,
    pub date_publication: String,
    pub soumis_article40: String,
    pub etat_des_traitements: EtatDesTraitements,
    pub date_sort: String,
    pub sort: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EtatDesTraitements {
    pub etat: Etat,
    pub sous_etat: SousEtat,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Etat {
    pub code: String,
    pub libelle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SousEtat {
    pub code: String,
    pub libelle: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Representations {
    pub representation: Representation,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Representation {
    pub nom: String,
    pub type_mime: TypeMime,
    pub statut_representation: StatutRepresentation,
    pub contenu: Contenu,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeMime {
    #[serde(rename = "type")]
    pub type_field: String,
    pub sub_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatutRepresentation {
    pub verbatim: String,
    pub canonique: String,
    pub officielle: String,
    pub transcription: String,
    pub enregistrement: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contenu {
    #[serde(rename = "documentURI")]
    pub document_uri: String,
}

impl CosmosEntity for Amendment {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.cycle_de_vie.date_depot.to_owned()
    }
}