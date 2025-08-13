use serde::Deserialize;

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

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Lead {
    pub id: u64,
    pub name: String,
    pub created_at: i64,
    pub custom_fields_values: Vec<CustomField>,
    pub _embedded: LeadEmbedded,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct CustomField {
    pub field_id: u64,
    pub field_name: String,
    pub values: Vec<Val>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LeadEmbedded {
    pub contacts: Vec<ContactSummary>,
}

#[derive(Deserialize, Debug, Clone)]
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
}

impl From<FlexibleType> for i32 {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(str_value) => str_value.parse().unwrap_or_default(),
            FlexibleType::Int(int_value) => int_value as i32,
            FlexibleType::Bool(_) => 0,
        }
    }
}

impl From<FlexibleType> for String {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(str_value) => str_value,
            FlexibleType::Int(_) => "".to_string(),
            FlexibleType::Bool(_) => "".to_string(),
        }
    }
}

impl From<FlexibleType> for bool {
    fn from(value: FlexibleType) -> Self {
        match value {
            FlexibleType::Str(_) => false,
            FlexibleType::Int(_) => false,
            FlexibleType::Bool(val) => val,
        }
    }
}

#[derive(Debug)]
pub struct DealWithContacts {
    pub deal_id: u64,
    pub contacts: Vec<ContactInfo>,
}

#[derive(Debug, Deserialize)]
pub struct RawContact {
    pub id: u64,
    pub custom_fields_values: Vec<CustomField>,
}

#[derive(Debug)]
pub struct ContactInfo {
    pub is_main: bool,
    pub info: Contact,
}

#[derive(Debug)]
pub struct Contact {
    pub id: u64,
    pub owner: bool,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
}

impl From<RawContact> for Contact {
    fn from(raw: RawContact) -> Self {
        let owner = raw.val_to_owner();
        let first_name = raw.val_to_first_name();
        let middle_name = raw.val_to_middle_name();
        let last_name = raw.val_to_last_name();
        let phone = raw.val_to_phone();
        let email = raw.val_to_email();

        Self {
            id: raw.id,
            owner,
            first_name,
            middle_name,
            last_name,
            phone,
            email,
        }
    }
}

impl RawContact {
    fn val_to_owner(&self) -> bool {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 763165)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
    fn val_to_first_name(&self) -> String {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 754911)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
    fn val_to_last_name(&self) -> String {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 754909)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
    fn val_to_middle_name(&self) -> String {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 754913)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
    fn val_to_phone(&self) -> String {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 194059)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
    fn val_to_email(&self) -> String {
        self.custom_fields_values
            .iter()
            .find(|f| f.field_id == 194061)
            .unwrap()
            .values[0]
            .value
            .clone()
            .into()
    }
}
