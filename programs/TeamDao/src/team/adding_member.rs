use super::errors::ErrorCode;
use super::*;

pub fn adding_member(
    ctx: Context<AddMember>,
    _team_name: String,
    _team_id: u64,
    member: Pubkey,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the team already has 5 players if so, return error
    require!(team.members.len() < 5, ErrorCode::TeamCapacityFullError);
    // checking if the member is already in the team, if so, return error
    require!(
        !team.members.contains(&member),
        ErrorCode::MemberAlreadyInTeamError
    );
    // checkin if the signer is the captain
    require!(
        team.captain == *ctx.accounts.signer.key,
        ErrorCode::NotCaptainError
    );

    // adding member to the team
    team.members.push(member);

    msg!("{} is successfully added to the team {}", member, team.name);

    Ok(())
}

// derive macro for adding member instruction
#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct AddMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
