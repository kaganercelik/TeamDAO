use super::errors::ErrorCode;
use super::*;

pub fn can_join(ctx: Context<CanJoinTournament>, _team_name: String, _team_id: u64) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the team has 5 players to join the tournament
    require!(team.members.len() == 5, ErrorCode::NotEnoughPlayersError);

    // checking if the team has an active tournament
    require!(
        team.active_tournament != Pubkey::default(),
        ErrorCode::NoActiveTournamentError
    );

    // checking if voting result is yes, all players voted for tournament and distribution
    if team.voting_result == true && team.distribution_yes_votes > 2 {
        team.can_join_tournament = true;
    } else {
        team.can_join_tournament = false;
    }

    Ok(())
}
// can join tournament
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct CanJoinTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
