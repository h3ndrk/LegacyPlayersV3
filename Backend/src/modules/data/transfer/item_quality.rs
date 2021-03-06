use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::ItemQuality;
use crate::modules::data::tools::RetrieveItemQuality;

#[openapi]
#[get("/item_quality/<id>")]
pub fn get_item_quality(me: State<Data>, id: u8) -> Option<Json<ItemQuality>>
{
  me.get_item_quality(id)
    .and_then(|item_quality| Some(Json(item_quality)))
}

#[openapi]
#[get("/item_quality")]
pub fn get_all_item_qualities(me: State<Data>) -> Json<Vec<ItemQuality>>
{
  Json(me.get_all_item_qualities())
}