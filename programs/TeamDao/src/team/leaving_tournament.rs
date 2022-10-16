use super::errors::ErrorCode;
use super::*;

pub fn leaving_tournament(
    ctx: Context<LeaveTournament>,
    _team_name: String,
    _team_id: u64,
    vote_type: VoteType,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the team has an active tournament
    require!(
        team.active_tournament != Pubkey::default(),
        ErrorCode::NoActiveTournamentError
    );

    // checking if the signer is in the team
    require!(
        team.members.contains(ctx.accounts.signer.key),
        ErrorCode::MemberNotInTeamError
    );

    // checking if the tournament is not already voted
    require!(
        !team.voted_players.contains(ctx.accounts.signer.key),
        ErrorCode::AlreadyVotedError
    );

    // checking the vote type
    match vote_type {
        VoteType::Yes => {
            // adding the player to voted players
            team.leave_voted_players.push(*ctx.accounts.signer.key);
            // incrementing yes votes
            team.leave_votes += 1;
        }
        VoteType::No => {
            // adding the player to voted players
            team.voted_players.push(*ctx.accounts.signer.key);
        }
    }

    if team.leave_votes > 2 {
        // if yes votes are more than half of the team members
        // remove the tournament from the team's active tournament
        team.active_tournament = Pubkey::default();
        // reset yes votes
        team.leave_votes = 0;
        // reset voted players
        team.leave_voted_players = vec![];
        // reset voted_players
        team.voted_players = vec![];
        // reset voting result
        team.voting_result = false;
        // reset yes votes
        team.yes_votes = 0;

        msg!(
            "{} is successfully left the tournament {}",
            team.name,
            team.name
        );
    }

    Ok(())
}

// vote for leaving the tournament
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct LeaveTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
