use serde::{Deserialize, Serialize};

use crate::frontmatter::is_valid_name;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub en: String,
    pub pt: String,
}

/// An area of interest from `catalog/areas.json`: a niche or a channel a base agent
/// gets specialized for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: String,
    pub kind: AreaKind,
    pub label: Label,
    pub hint: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AreaKind {
    Niche,
    Channel,
}

pub fn parse_areas(json: &str) -> Result<Vec<Area>, String> {
    let areas: Vec<Area> =
        serde_json::from_str(json).map_err(|e| format!("catalog/areas.json: {e}"))?;
    let mut ids = std::collections::HashSet::new();
    for area in &areas {
        if !is_valid_name(&area.id) || !ids.insert(area.id.as_str()) {
            return Err(format!("catalog/areas.json: invalid or duplicate id '{}'", area.id));
        }
    }
    Ok(areas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_catalog_is_valid() {
        let areas = parse_areas(include_str!("../../../catalog/areas.json")).unwrap();
        assert!(areas.iter().any(|a| a.kind == AreaKind::Niche));
        assert!(areas.iter().any(|a| a.kind == AreaKind::Channel));
        assert!(areas.iter().all(|a| !a.label.pt.is_empty() && !a.hint.is_empty()));
    }

    #[test]
    fn rejects_duplicate_ids() {
        let one = r#"{"id":"a","kind":"niche","label":{"en":"A","pt":"A"},"hint":"h"}"#;
        assert!(parse_areas(&format!("[{one},{one}]")).is_err());
    }
}
