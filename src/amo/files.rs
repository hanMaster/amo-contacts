use super::Result;
use std::collections::HashSet;

pub fn get_profit_ids() -> Result<HashSet<String>> {
    let mut profit_ids = HashSet::new();
    let content = std::fs::read_to_string("1.txt")?;

    for line in content.lines() {
        let line = line.trim();
        if !line.is_empty() && line.len() == 8 {
            profit_ids.insert(line.to_string());
        }
    }
    Ok(profit_ids)
}