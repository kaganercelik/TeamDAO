use super::errors::ErrorCode;
use super::*;

pub fn removing_member(
    ctx: Context<RemoveMember>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the team has at least 2 players if not, return error
    require!(team.members.len() > 1, ErrorCode::TeamCapacityLowError);
    // checkinf if the caller is the captain of the team
    require!(team.captain != member, ErrorCode::NotCaptainError);
    // checking it the member is in the team
    require!(
        team.members.contains(&member),
        ErrorCode::MemberNotInTeamError
    );

    // checking if the member is in the team
    require!(
        team.members.contains(&member),
        ErrorCode::MemberNotInTeamError
    );

    // removing member from team
    team.members.retain(|&x| x != member);

    msg!(
        "{} is successfully removed from the team {}",
        member,
        team.name
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct RemoveMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
