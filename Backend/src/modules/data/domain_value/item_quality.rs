#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ItemQuality {
  pub id: u8,
  pub localization_id: u32,
  pub color: String
}