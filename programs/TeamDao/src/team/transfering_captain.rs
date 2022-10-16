use super::errors::ErrorCode;
use super::*;

pub fn transfering_captain(
    ctx: Context<TransferCaptain>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the signer is captain
    require!(
        team.captain == *ctx.accounts.signer.key,
        ErrorCode::NotCaptainError
    );
    // checking if the member is in the team
    require!(
        team.members.contains(&member),
        ErrorCode::MemberNotInTeamError
    );

    // transferring captain role
    team.captain = member;

    msg!(
        "Captain role is successfully transferred to {} in the team {}",
        member,
        team.name
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct TransferCaptain<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
