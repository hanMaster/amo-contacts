#[derive(Debug, Clone, Default)]
pub struct ProfitData {
    pub deal_id: u64,
    pub project: String,
    pub house: i32,
    pub object_type: String,
    pub object: i32,
}

pub fn get_ru_object_type(profitbase_type: &str) -> &'static str {
    match profitbase_type {
        "property" => "Квартира",
        "pantry" => "Кладовка",
        "parking" => "Машиноместо",
        _ => ""
    }
}
