use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ProfitRecord {
    pub status: String,
    pub data: Vec<ProfitData>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProfitData {
    pub number: String,
    #[serde(rename = "propertyType")]
    pub property_type: String,
    #[serde(rename = "houseName")]
    pub house_name: String,
    pub attributes: Attrs,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Attrs {
    pub facing: Option<String>,
}