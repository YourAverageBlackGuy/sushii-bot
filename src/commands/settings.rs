use serenity::framework::standard::CommandError;
use serenity::utils::parse_channel;
use serenity::utils::parse_role;
use serenity::model::guild::Role;

use serde_json;

use std::env;
use std::fmt::Write;
use database;
use utils::config::*;

command!(prefix(ctx, msg, args) {
    let mut data = ctx.data.lock();
    let pool = data.get_mut::<database::ConnectionPool>().unwrap();

    // check for MANAGE_SERVER permissions

    if let Some(guild) = msg.guild() {
        let guild = guild.read();

        let pref = match args.single::<String>() {
            Ok(val) => val,
            Err(_) => {
                // no prefix argument, set the prefix
                match pool.get_prefix(guild.id.0) {
                    Some(pref) => {
                        let _ = msg.channel_id.say(&get_msg!("info/prefix_current", &pref));
                        return Ok(());
                    },
                    None => {
                        let pref = env::var("DEFAULT_PREFIX").expect(&get_msg!("error/prefix_no_default"));
                        let _ = msg.channel_id.say(get_msg!("info/prefix_current", &pref));
                        return Ok(());
                    }
                }
            },
        };

        let has_manage_guild = guild.member_permissions(msg.author.id).manage_guild();

        if has_manage_guild {
            let success = pool.set_prefix(guild.id.0, &pref);

            if success {
                let _ = msg.channel_id.say(get_msg!("info/prefix_set", &pref));
            } else {
                let _ = msg.channel_id.say(get_msg!("info/prefix_existing", &pref));
            }
        } else {
            return Err(CommandError::from("error/prefix_no_perms"));
        }
        
    } else {
        // no guild found, probably in DMs
        let pref = env::var("DEFAULT_PREFIX").expect(&get_msg!("error/prefix_no_default"));
        let _ = msg.channel_id.say(get_msg!("info/prefix_default", &pref));
    }
});

command!(joinmsg(ctx, msg, args) {
    let mut data = ctx.data.lock();
    let pool = data.get_mut::<database::ConnectionPool>().unwrap();

    let message = args.full().to_owned();

    if let Some(guild_id) = msg.guild_id() {
        let guild_id = guild_id.0;
        let config = pool.get_guild_config(guild_id);

        // no message given, just print out the current message
        if args.len() == 0 {
            if let Some(current_message) = config.join_msg {
                let s = get_msg!("info/join_message_current", current_message);
                let _ = msg.channel_id.say(&s);
            } else {
                let _ = msg.channel_id.say(get_msg!("info/join_message_none"));
            }
        } else {
            let mut config = config;

            if message == "off" {
                config.join_msg = None;

                let _ = msg.channel_id.say(get_msg!("info/join_message_disable"));
            } else {
                config.join_msg = Some(message.to_owned());

                let s = get_msg!("info/join_message_set", message);
                let _ = msg.channel_id.say(&s);
            }

            pool.save_guild_config(&config);
        }
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(leavemsg(ctx, msg, args) {
    let mut data = ctx.data.lock();
    let pool = data.get_mut::<database::ConnectionPool>().unwrap();

    let message = args.full().to_owned();

    if let Some(guild_id) = msg.guild_id() {
        let guild_id = guild_id.0;
        let config = pool.get_guild_config(guild_id);

        // no message given, just print out the current message
        if args.len() == 0 {
            if let Some(current_message) = config.leave_msg {
                let s = get_msg!("info/leave_message_current", current_message);
                let _ = msg.channel_id.say(&s);
            } else {
                let _ = msg.channel_id.say(get_msg!("info/leave_message_none"));
            }
        } else {
            let mut config = config;

            if message == "off" {
                config.leave_msg = None;

                let _ = msg.channel_id.say(get_msg!("info/leave_message_disable"));
            } else {
                config.leave_msg = Some(message.to_owned());

                let s = get_msg!("info/leave_message_set", message);
                let _ = msg.channel_id.say(&s);
            }

            pool.save_guild_config(&config);
        }
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(modlog(ctx, msg, args) {
    let channel = match args.single::<String>() {
        Ok(val) => parse_channel(&val).unwrap_or(0),
        Err(_) => return Err(CommandError::from(get_msg!("error/no_channel_given"))),
    };

    if channel == 0 {
        return Err(CommandError::from(get_msg!("error/invalid_channel")));
    }

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);

        config.log_mod = Some(channel as i64);

        pool.save_guild_config(&config);

        let s = get_msg!("info/mod_log_set", channel);
        let _ = msg.channel_id.say(&s);
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(msglog(ctx, msg, args) {
    let channel = match args.single::<String>() {
        Ok(val) => parse_channel(&val).unwrap_or(0),
        Err(_) => return Err(CommandError::from(get_msg!("error/no_channel_given"))),
    };

    if channel == 0 {
        return Err(CommandError::from(get_msg!("error/invalid_channel")));
    }

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);

        config.log_msg = Some(channel as i64);

        pool.save_guild_config(&config);

        let s = get_msg!("info/message_log_set", channel);
        let _ = msg.channel_id.say(&s);
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(memberlog(ctx, msg, args) {
    let channel = match args.single::<String>() {
        Ok(val) => parse_channel(&val).unwrap_or(0),
        Err(_) => return Err(CommandError::from(get_msg!("error/no_channel_given"))),
    };

    if channel == 0 {
        return Err(CommandError::from(get_msg!("error/invalid_channel")));
    }

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);

        config.log_member = Some(channel as i64);

        pool.save_guild_config(&config);

        let s = get_msg!("info/member_log_set", channel);
        let _ = msg.channel_id.say(&s);
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(inviteguard(ctx, msg, args) {
    let status_str = match args.single::<String>() {
        Ok(val) => val,
        Err(_) => return Err(CommandError::from(get_msg!("error/invalid_option_enable_disable"))),
    };

    let mut status;
    let mut s;

    if status_str == "enable" {
        status = true;
        s = "Invite guard has been enabled.";
    } else if status_str == "disable" {
        status = false;
        s = "Invite guard has been disabled.";
    } else {
        return Err(CommandError::from(get_msg!("error/invalid_option_enable_disable")));
    }

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);

        config.invite_guard = Some(status);

        pool.save_guild_config(&config);

        let _ = msg.channel_id.say(&s);
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

fn validate_roles_config(cfg: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut s = String::new();
    for (cat_name, cat_data) in cfg.iter() {
        // check if there's a roles field
        if let Some(lim) = cat_data.get("limit") {
            if !lim.is_number() {
                let _ = write!(s, "Category limit for `{}` has to be a number\n", cat_name);
            }
        } else {
            let _ = write!(s, "Missing category limit for `{}`, set to 0 to disable\n", cat_name);
        }
        // check if there is a roles field
        if let Some(roles) = cat_data.get("roles") {
            // check if roles is an object
            if let Some(obj) = roles.as_object() {
                // check if roles object is empty
                if obj.is_empty() {
                    let _ = write!(s, "Roles for `{}` cannot be empty\n", cat_name); 
                }
                // check if each role has correct properties
                for (role_name, role_data) in obj.iter() {
                    let role_fields = ["search", "primary", "secondary"];

                    // check for each property
                    for role_field in role_fields.iter() {
                        if let Some(val) = role_data.get(role_field) {
                            if role_field == &"search" {
                                if !val.is_string() {
                                    let _ = write!(s, "Field `{}` for role `{}` in category `{}` must be a string (Supports RegEx)\n", 
                                        role_field, role_name, cat_name);
                                }
                            } else {
                                if !val.is_u64() {
                                    let _ = write!(s, "Field `{}` for role `{}` in category `{}` has to be a number (Role ID)\n", 
                                        role_field, role_name, cat_name);
                                }
                            }
                        } else {
                            let _ = write!(s, "Role `{}` in category `{}` is missing field `{}`\n", 
                                role_name, cat_name, role_field);
                        }
                    }
                }
            } else {
                let _ = write!(s, "Roles in category `{}` are not configured properly as an object\n", cat_name);
            }
        } else {
            let _ = write!(s, "Missing roles for category `{}`\n", cat_name);
        }
    }

    return s;
}

command!(roles_set(ctx, msg, args) {
    let mut raw_json = args.full().to_owned();

    // check if it starts with a code block
    if raw_json.starts_with("```") && raw_json.ends_with("```") {
        // remove code block from string
        raw_json = raw_json.replace("```json", "");
        raw_json = raw_json.replacen("```", "", 2);
    }

    if raw_json.is_empty() && msg.attachments.len() > 0 {
        let bytes = match msg.attachments[0].download() {
            Ok(content) => content,
            Err(e) => return Err(CommandError::from(e)),
        };

        raw_json = match String::from_utf8(bytes) {
            Ok(content) => content,
            Err(e) => return Err(CommandError::from(e)),
        };
    } else if raw_json.is_empty() && msg.attachments.is_empty() {
        // no message or attachment 
        return Err(CommandError::from(get_msg!("error/no_config_given")));
    }

    let role_config: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(&raw_json) {
        Ok(val) => val,
        Err(e) => return Err(CommandError::from(e)),
    };

    let validated = validate_roles_config(&role_config);
    if !validated.is_empty() {
        return Err(CommandError::from(validated));
    };

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);        
        config.role_config = Some(serde_json::Value::from(role_config));

        pool.save_guild_config(&config);

        let _ = msg.channel_id.say(get_msg!("info/role_config_set"));
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(roles_channel(ctx, msg, args) {
    let channel = match args.single::<String>() {
        Ok(val) => parse_channel(&val).unwrap_or(0),
        Err(_) => return Err(CommandError::from(get_msg!("error/no_channel_given"))),
    };

    if channel == 0 {
        return Err(CommandError::from(get_msg!("error/invalid_channel")));
    }

    if let Some(guild_id) = msg.guild_id() {
        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild_id.0);

        config.role_channel = Some(channel as i64);

        pool.save_guild_config(&config);

        let s = get_msg!("info/role_channel_set", channel);
        let _ = msg.channel_id.say(&s);
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});

command!(roles_get(ctx, msg, _args) {
    if let Some(guild_id) = msg.guild_id() {
        let config = get_config_from_context(&ctx, guild_id.0);

        if let Some(role_config) = config.role_config {
            let roles_pretty = match serde_json::to_string_pretty(&role_config) {
                Ok(val) => val,
                Err(e) => return Err(CommandError::from(e)),
            };

            let s = format!("```json\n{}\n```", roles_pretty);
            let _ = msg.channel_id.say(&s);
        } else {
            return Err(CommandError::from(get_msg!("error/no_role_config")))
        }
    }
});


command!(mute_role(ctx, msg, args) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read();

        let role = match args.single::<String>() {
            Ok(val) => val,
            Err(e) => return Err(CommandError::from(e)),
        };

        let role_id = parse_role(&role)
            .or(guild.roles.values().find(|&x| x.name == role).map(|x| x.id.0));

        if let Some(id) = role_id {
            let pool = get_pool(&ctx);

            let mut config = pool.get_guild_config(guild.id.0);
            config.mute_role = Some(id as i64);

            pool.save_guild_config(&config);

            let s = get_msg!("info/mute_role_set", id);
            let _ = msg.channel_id.say(&s);
        } else {
            return Err(CommandError::from(get_msg!("error/invalid_role")));
        }
    }    
});

command!(max_mentions(ctx, msg, args) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read();

        let max_mention = match args.single::<i32>() {
            Ok(val) => val,
            Err(_) => return Err(CommandError::from(get_msg!("error/required_number"))),
        };

        let pool = get_pool(&ctx);

        let mut config = pool.get_guild_config(guild.id.0);
        config.max_mention = max_mention;

        pool.save_guild_config(&config);

        let s = get_msg!("info/max_mention_set", max_mention);
        let _ = msg.channel_id.say(&s);
    }    
});

command!(list_ids(_ctx, msg, _args) {
    if let Some(guild) = msg.guild() {
        let guild = guild.read();

        let mut roles_text = String::new();

        let mut roles = guild.roles.values().collect::<Vec<&Role>>();
        roles.sort_by(|&a, &b| b.position.cmp(&a.position));

        for role in roles.iter() {
            let _ = write!(roles_text, "[{:02}] {} - {}\n", role.position, role.id.0, role.name);
        }

        // check if over limit, send a text file instead
        if roles_text.len() >= 2000 {
            let files = vec![(roles_text.as_bytes(), "roles.txt")];
            
            let _ = msg.channel_id.send_files(files, |m| m.content(get_msg!("info/list_ids_attached")));
        } else {
            let s = format!("Server roles:\n```ruby\n{}```", roles_text);

            let _ = msg.channel_id.say(&s);
        }
    } else {
        return Err(CommandError::from(get_msg!("error/no_guild")));
    }
});