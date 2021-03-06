use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemClass;

pub trait RetrieveItemClass {
  fn get_item_class(&self, id: u8) -> Option<ItemClass>;
  fn get_all_item_classes(&self) -> Vec<ItemClass>;
}

impl RetrieveItemClass for Data {
  fn get_item_class(&self, id: u8) -> Option<ItemClass> {
    self.item_classes.get(&id)
      .and_then(|item_class| Some(item_class.clone()))
  }

  fn get_all_item_classes(&self) -> Vec<ItemClass> {
    self.item_classes.iter().map(|(_, item_class)| item_class.clone()).collect()
  }
}
