use super::errors::ErrorCode;
use super::*;

pub fn voting_for_tournament(
    ctx: Context<VoteForTournament>,
    _team_name: String,
    _team_id: u64,
    vote_type: VoteType,
) -> Result<()> {
    let team = &mut ctx.accounts.team_account;

    // checking if the team still has an active tournament
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

    // checking vote type
    match vote_type {
        VoteType::Yes => {
            // adding the player to voted players
            team.voted_players.push(*ctx.accounts.signer.key);
            // incrementing yes votes
            team.yes_votes += 1;
        }
        VoteType::No => {
            // adding the player to voted players
            team.voted_players.push(*ctx.accounts.signer.key);
        }
    }

    // checking if the vote is successful
    if team.yes_votes > 2 {
        // if yes votes are more than half of the team members
        // add the tournament to the team's active tournament
        // reset yes votes
        team.yes_votes = 0;
        // reset voted players
        team.voted_players = vec![];

        team.voting_result = true;
    }

    msg!(
        "{} is successfully voted for the tournament {}",
        team.name,
        team.name
    );

    Ok(())
}

// vote for tournament instruction
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct VoteForTournament<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    // #[account(mut)]
    // pub tournament_account: Account<'info, TeamAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
