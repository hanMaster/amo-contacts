use crate::profit::ProfitData;
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug, Clone)]
pub struct Leads {
    pub _links: Links,
    pub _embedded: Embedded,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Links {
    pub next: Option<Link>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Link {
    pub href: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Embedded {
    pub leads: Vec<Lead>,
}
fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Deserialize, Debug, Clone)]
pub struct Lead {
    pub id: u64,
    pub name: String,
    pub created_at: i64,
    #[serde(deserialize_with = "null_to_default")]
    pub custom_fields_values: Vec<CustomField>,
    pub _embedded: LeadEmbedded,
}

impl Lead {
    pub fn get_deal_type(&self) -> String {
        self.val_to_str("Тип договора")
    }
    pub fn val_to_str(&self, field_name: &str) -> String {
        let field_opt = self
            .custom_fields_values
            .iter()
            .find(|f| f.field_name == field_name);
        match field_opt {
            None => "".to_string(),
            Some(f) => f.values[0].value.clone().into(),
        }
    }
}

impl Display for Lead {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let project = self.val_to_str("ЖК");
        let house = self.val_to_str("Дом");
        let num = self.val_to_str("Номер помещения");
        write!(f, "https://dnscity.amocrm.ru/leads/detail/{}, ЖК: {}, {}, помещение № {}", self.id, project, house, num)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CustomField {
    // pub field_id: u64,
    pub field_name: String,
    pub values: Vec<Val>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LeadEmbedded {
    pub contacts: Vec<ContactSummary>,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct ContactSummary {
    pub id: i64,
    pub is_main: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactSelfLink {
    #[serde(rename = "self")]
    pub _self: Link,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Val {
    pub value: FlexibleType,
    pub enum_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum FlexibleType {
    Str(String),
    Int(i64),
    Bool(bool),
    Struct(FileInfo),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FileInfo {
    pub file_uuid: String,
}

impl From<FlexibleType> for i32 {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(str_value) => str_value.parse().unwrap_or_default(),
            FlexibleType::Int(int_value) => int_value as i32,
            FlexibleType::Bool(_) => 0,
            FlexibleType::Struct(_) => 0,
        }
    }
}

impl From<FlexibleType> for String {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(str_value) => str_value,
            FlexibleType::Int(_) => "".to_string(),
            FlexibleType::Bool(_) => "".to_string(),
            FlexibleType::Struct(_) => "".to_string(),
        }
    }
}

impl From<FlexibleType> for bool {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(_) => false,
            FlexibleType::Int(_) => false,
            FlexibleType::Bool(val) => val,
            FlexibleType::Struct(_) => false,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawContact {
    pub custom_fields_values: Vec<CustomField>,
}

#[derive(Debug, Clone)]
pub struct ContactInfo {
    pub is_main: bool,
    pub info: Contact,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub owner: bool,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub doc_type: String,
    pub doc_serial: String,
    pub doc_number: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ContactForExport {
    #[serde(rename = "Name")]
    pub first_name: String,
    #[serde(rename = "Surname")]
    pub last_name: String,
    #[serde(rename = "Patronymic")]
    pub middle_name: String,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Clientid")]
    pub client_id: i64,
}

impl From<RawContact> for Contact {
    fn from(raw: RawContact) -> Self {
        let owner = raw.val_to_owner();
        let first_name = raw.val_to_str("Имя");
        let middle_name = raw.val_to_str("Отчество");
        let last_name = raw.val_to_str("Фамилия");
        let phone = raw.val_to_str("Телефон");
        let email = raw.val_to_str("Email");
        let doc_type = raw.val_to_str("Тип документа");
        let doc_serial = raw.val_to_str("Серия паспорта");
        let doc_number = raw.val_to_str("Номер паспорта");

        Self {
            owner,
            first_name,
            middle_name,
            last_name,
            phone,
            email,
            doc_type,
            doc_serial,
            doc_number,
        }
    }
}

impl RawContact {
    pub fn val_to_owner(&self) -> bool {
        let field_opt = self
            .custom_fields_values
            .iter()
            .find(|f| f.field_name == "Собственник");
        match field_opt {
            Some(f) => f.values[0].value.clone().into(),
            None => false,
        }
    }
    pub fn val_to_str(&self, field_name: &str) -> String {
        let field_opt = self
            .custom_fields_values
            .iter()
            .find(|f| f.field_name == field_name);
        match field_opt {
            None => "".to_string(),
            Some(f) => f.values[0].value.clone().into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawData {
    pub deal_type: String,
    pub profit_data: ProfitData,
    pub contacts: Vec<ContactSummary>,
}

#[derive(Debug, Clone)]
pub struct VecRawData {
    pub rows: Vec<RawData>,
}

#[derive(Debug, Clone)]
pub struct RawDataFlat {
    pub deal_type: String,
    pub profit_data: ProfitData,
    pub contact: ContactSummary,
}

#[derive(Debug, Clone)]
pub struct ProfitWithContact {
    pub deal_type: String,
    pub profit_data: ProfitData,
    pub contact: ContactInfo,
}

impl From<VecRawData> for Vec<RawDataFlat> {
    fn from(value: VecRawData) -> Self {
        let mut res = Vec::with_capacity(value.rows.len());
        for row in value.rows {
            let pd = row.profit_data;
            for contact in row.contacts {
                let d = RawDataFlat {
                    deal_type: row.deal_type.clone(),
                    profit_data: pd.clone(),
                    contact,
                };
                res.push(d);
            }
        }
        res
    }
}

#[derive(Deserialize, Debug)]
pub struct RawContacts {
    pub _links: Links,
    pub _embedded: EmbeddedContacts,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddedContacts {
    pub contacts: Vec<RawContact>,
}