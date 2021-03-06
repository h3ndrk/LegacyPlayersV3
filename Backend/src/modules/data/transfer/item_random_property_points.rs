use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemRandomPropertyPoints;
use crate::modules::data::tools::RetrieveItemRandomPropertyPoints;

#[openapi]
#[get("/item_random_property_points/<expansion_id>/<item_level>")]
pub fn get_item_random_property_points(me: State<Data>, expansion_id: u8, item_level: u16) -> Option<Json<ItemRandomPropertyPoints>> {
  me.get_item_random_property_points(expansion_id, item_level)
    .and_then(|result| Some(Json(result)))
}