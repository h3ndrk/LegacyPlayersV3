use crate::modules::tooltip::material::GuildTooltip;
use crate::modules::armory::Armory;
use crate::modules::tooltip::dto::TooltipFailure;
use crate::modules::armory::tools::GetGuild;
use crate::modules::tooltip::Tooltip;

pub trait RetrieveGuildTooltip {
    fn get_guild(&self, armory: &Armory, guild_id: u32) -> Result<GuildTooltip, TooltipFailure>;
}

impl RetrieveGuildTooltip for Tooltip {
    fn get_guild(&self, armory: &Armory, guild_id: u32) -> Result<GuildTooltip, TooltipFailure> {
        let guild = armory.get_guild(guild_id);
        if guild.is_none() {
            return Err(TooltipFailure::InvalidInput);
        }
        let guild = guild.unwrap();

        let characters = armory.characters.read().unwrap();
        let num_member = characters.iter()
            .filter(|(_, character)| character.last_update.is_some())
            .filter(|(_, character)| character.last_update.as_ref().unwrap().character_guild.is_some())
            .filter(|(_, character)| character.last_update.as_ref().unwrap().character_guild.as_ref().unwrap().guild_id == guild_id)
            .count();

        Ok(GuildTooltip {
            guild_id,
            guild_name: guild.name,
            num_member,
        })
    }
}