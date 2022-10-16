use super::errors::ErrorCode;
use super::*;

pub fn creating_team(ctx: Context<CreateTeam>, team_name: String, team_id: u64) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    team.bump = *ctx
        .bumps
        .get("team_account")
        .ok_or(ErrorCode::InvalidBumpSeeds)?;

    // assigning required parameters to the team
    team.name = team_name;
    team.captain = *ctx.accounts.signer.key;
    team.id = team_id;
    team.members.push(*ctx.accounts.signer.key);
    team.can_join_tournament = false;
    team.distribution_voting_result = false;

    msg!("Team created");
    msg!("Team name: {}", team.name);
    msg!("Team captain: {}", team.captain);

    Ok(())
}

// derive macro for create team instruction
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct CreateTeam<'info> {
    #[account(init, payer = signer, space = TeamAccount::LEN, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
