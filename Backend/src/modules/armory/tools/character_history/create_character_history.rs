use mysql_connection::tools::Execute;

use crate::modules::armory::{
    domain_value::HistoryMoment,
    dto::{ArmoryFailure, CharacterHistoryDto},
    material::CharacterHistory,
    tools::{CreateCharacterFacial, CreateCharacterInfo, CreateGuild, GetCharacter, GetCharacterHistory, SetGuildRank},
    Armory,
};
use crate::modules::armory::tools::SetArenaTeam;
use crate::modules::armory::domain_value::ArenaTeamSizeType;

pub trait CreateCharacterHistory {
    fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto, character_uid: u64) -> Result<CharacterHistory, ArmoryFailure>;
}

impl CreateCharacterHistory for Armory {
    // Assumption: It has been checked that the previous value is not the same
    // Assumption: Character exists
    fn create_character_history(&self, server_id: u32, character_history_dto: CharacterHistoryDto, character_uid: u64) -> Result<CharacterHistory, ArmoryFailure> {
        let character_id = self.get_character_id_by_uid(server_id, character_uid).unwrap();
        let mut guild_id = None;
        if let Some(char_guild_dto) = character_history_dto.character_guild.as_ref() {
            let guild = self.create_guild(server_id, char_guild_dto.guild.to_owned());
            if guild.is_err() {
                return Err(guild.err().unwrap());
            }
            guild_id = Some(guild.unwrap().id);
            if let Err(e) = self.set_guild_rank(*guild_id.as_ref().unwrap(), char_guild_dto.rank.clone()) {
                return Err(e);
            }
        }
        let character_info_res = self.create_character_info(character_history_dto.character_info.to_owned());
        if character_info_res.is_err() {
            return Err(character_info_res.err().unwrap());
        }
        let character_info = character_info_res.unwrap();

        let facial = if character_history_dto.facial.is_some() {
            let facial_res = self.create_character_facial(character_history_dto.facial.as_ref().unwrap().clone());
            if facial_res.is_err() {
                return Err(facial_res.err().unwrap());
            }
            facial_res.ok()
        } else {
            None
        };

        let mut arena_teams = Vec::new();
        for team in &character_history_dto.arena_teams {
            arena_teams.push(self.set_arena_team(server_id, team.clone())?);
        }
        let arena2 = arena_teams.iter_mut().find(|team| team.size_type == ArenaTeamSizeType::Size2v2).map(|team| team.id);
        let arena3 = arena_teams.iter().find(|team| team.size_type == ArenaTeamSizeType::Size3v3).map(|team| team.id);
        let arena5 = arena_teams.iter().find(|team| team.size_type == ArenaTeamSizeType::Size5v5).map(|team| team.id);

        let params = params!(
          "character_id" => character_id,
          "character_info_id" => character_info.id,
          "character_name" => character_history_dto.character_name.clone(),
          "title" => character_history_dto.character_title,
          "guild_id" => guild_id,
          "guild_rank" => character_history_dto.character_guild.as_ref().map(|chr_guild_dto| chr_guild_dto.rank.index),
          "prof_skill_points1" => character_history_dto.profession_skill_points1,
          "prof_skill_points2" => character_history_dto.profession_skill_points2,
          "facial" => facial.as_ref().map(|chr_facial| chr_facial.id),
          "arena2" => arena2,
          "arena3" => arena3,
          "arena5" => arena5
        );
        self.db_main.execute_wparams(
            "INSERT INTO armory_character_history (`character_id`, `character_info_id`, `character_name`, `title`, \
            `guild_id`, `guild_rank`, `prof_skill_points1`, `prof_skill_points2`, `facial`, `arena2`, `arena3`, `arena5`, `timestamp`) VALUES (:character_id, \
             :character_info_id, :character_name, :title, :guild_id, :guild_rank, :prof_skill_points1, :prof_skill_points2, :facial, :arena2, :arena3, :arena5, UNIX_TIMESTAMP())",
            params,
        );
        if let Ok(character_history_res) = self.get_character_history_by_value(character_id, character_history_dto) {
            let mut characters = self.characters.write().unwrap();
            let mut character = characters.get_mut(&character_id).unwrap();
            character.last_update = Some(character_history_res.clone());
            character.history_moments.push(HistoryMoment {
                id: character_history_res.id,
                timestamp: character_history_res.timestamp,
            });
            return Ok(character_history_res);
        }

        Err(ArmoryFailure::Database("create_character_history".to_owned()))
    }
}
