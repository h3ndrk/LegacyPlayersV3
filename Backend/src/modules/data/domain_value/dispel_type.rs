#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct DispelType {
  pub id: u8,
  pub localization_id: u32,
  pub color: String
}