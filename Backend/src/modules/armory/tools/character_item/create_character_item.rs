use mysql_connection::tools::Execute;

use crate::modules::armory::Armory;
use crate::modules::armory::domain_value::CharacterItem;
use crate::modules::armory::dto::{ArmoryFailure, CharacterItemDto};
use crate::modules::armory::tools::GetCharacterItem;

pub trait CreateCharacterItem {
  fn create_character_item(&self, character_item: CharacterItemDto) -> Result<CharacterItem, ArmoryFailure>;
}

impl CreateCharacterItem for Armory {
  fn create_character_item(&self, character_item: CharacterItemDto) -> Result<CharacterItem, ArmoryFailure> {
    // If it already exists, return this one
    let existing_item = self.get_character_item_by_value(character_item.clone());
    if existing_item.is_ok() {
      return existing_item;
    }

    let params = params!(
      "item_id" => character_item.item_id,
      "random_property_id" => character_item.random_property_id,
      "enchant_id" => character_item.enchant_id,
      "gem_id1" => character_item.gem_ids.get(0).cloned(),
      "gem_id2" => character_item.gem_ids.get(1).cloned(),
      "gem_id3" => character_item.gem_ids.get(2).cloned(),
      "gem_id4" => character_item.gem_ids.get(3).cloned()
    );

    // It may happen that another thread is inserting the same item
    // Therefore the insert will fail due to the unique constraint
    // So in any case, attempt to retrieve the character item
    self.db_main.execute_wparams("INSERT INTO armory_item (`item_id`, `random_property_id`, `enchant_id`, `gem_id1`, `gem_id2`, `gem_id3`, `gem_id4`) VALUES (:item_id, :random_property_id, :enchant_id, :gem_id1, :gem_id2, :gem_id3, :gem_id4)", params.clone());
    let char_item = self.get_character_item_by_value(character_item);
    if char_item.is_ok() {
      return Ok(char_item.unwrap());
    }
    Err(ArmoryFailure::Database("create_character_item".to_owned()))
  }
}