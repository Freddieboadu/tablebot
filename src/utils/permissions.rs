use crate::utils::table_utils::Settings;
use crate::Context;

pub async fn check_admin(ctx: Context<'_>, settings: &Settings) -> bool {
    let Some(member) = ctx.author_member().await else {
        return false;
    };

    // Check custom admin role first
    if let Some(role_id) = settings.admin_role_id {
        if member
            .roles
            .contains(&poise::serenity_prelude::RoleId::new(role_id))
        {
            return true;
        }
    }

    // Check MANAGE_GUILD via permissions field (populated in slash command interactions)
    if let Some(perms) = member.permissions {
        return perms.manage_guild();
    }

    false
}
