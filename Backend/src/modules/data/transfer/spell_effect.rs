use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::SpellEffect;
use crate::modules::data::tools::RetrieveSpellEffect;

#[openapi]
#[get("/spell_effect/<expansion_id>/<spell_id>")]
pub fn get_spell_effects(me: State<Data>, expansion_id: u8, spell_id: u32) -> Option<Json<Vec<SpellEffect>>> {
  me.get_spell_effects(expansion_id, spell_id)
    .and_then(|result| Some(Json(result)))
}