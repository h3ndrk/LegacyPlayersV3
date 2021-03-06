use rocket::State;
use rocket_contrib::json::Json;

use crate::modules::data::Data;
use crate::modules::data::domain_value::{HeroClass, Localized};
use crate::modules::data::guard::Language;
use crate::modules::data::tools::{RetrieveHeroClass, RetrieveLocalization};

#[openapi]
#[get("/hero_class/<id>")]
pub fn get_hero_class(me: State<Data>, id: u8) -> Option<Json<HeroClass>>
{
  me.get_hero_class(id)
    .and_then(|hero_class| Some(Json(hero_class)))
}

#[openapi]
#[get("/hero_class")]
pub fn get_all_hero_classes(me: State<Data>) -> Json<Vec<HeroClass>>
{
  Json(me.get_all_hero_classes())
}

#[openapi]
#[get("/hero_class/localized/<id>")]
pub fn get_hero_class_localized(me: State<Data>, language: Language, id: u8) -> Option<Json<Localized<HeroClass>>>
{
  me.get_hero_class(id)
    .and_then(|hero_class| Some(Json(Localized {
      localization: me.get_localization(language.0, hero_class.localization_id).unwrap().content,
      base: hero_class.to_owned()
    })))
}

#[openapi]
#[get("/hero_class/localized")]
pub fn get_all_hero_classes_localized(me: State<Data>, language: Language) -> Json<Vec<Localized<HeroClass>>>
{
  Json(me.get_all_hero_classes().iter().map(|hero_class| Localized {
    localization: me.get_localization(language.0, hero_class.localization_id).unwrap().content,
    base: hero_class.to_owned(),
  }).collect())
}