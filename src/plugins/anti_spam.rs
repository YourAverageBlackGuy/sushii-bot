use serenity::model::channel::Message;
use serenity::prelude::Context;
use serenity::CACHE;

use database::ConnectionPool;

pub fn on_message(_ctx: &Context, pool: &ConnectionPool, msg: &Message) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read();

        let current_user_id = CACHE.read().user.id;

        // return if bot doesn't have role perms
        if !guild.member_permissions(current_user_id).manage_roles() {
            return;
        }

        // return if bot sent the message, not sure why this would happen
        if msg.author.id == current_user_id {
            return;
        }

        // allow those with perms to bypass
        if guild.member_permissions(msg.author.id).manage_guild() {
            return;
        }

        // get the config
        let config = check_res!(pool.get_guild_config(guild.id.0));

        if msg.mentions.len() > config.max_mention as usize {
            // get the member
            let mut member = match guild.member(msg.author.id) {
                Ok(val) => val,
                Err(e) => {
                    error!("Error while fetching member: {}", e);
                    return;
                }
            };

            // get the mute role
            let mute_role = match config.mute_role {
                Some(val) => val,
                None => return,
            };

            // create a pending case with mute, reason, no exexcutor (defaults to bot)
            let case_id = check_res!(pool.add_mod_action(
                "mute",
                guild.id.0,
                &msg.author,
                Some("Automated Mute: User exceeded mention limit. (10)"),
                true,
                None,
            )).case_id;

            // add the mute role
            if let Err(e) = member.add_role(mute_role as u64) {
                error!(
                    "Error while adding auto mute role exceeding mention limit: {}",
                    e
                );

                // remove pending action if mute failed
                pool.remove_mod_action(guild.id.0, &msg.author, case_id);
            }
        }
    }
}
