use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("A team can contain maximum 5 members")]
    TeamCapacityFullError,
    #[msg("Invalid bump seeds")]
    InvalidBumpSeeds,
    #[msg("A team must contain at least 2 members to be able to remove a member")]
    TeamCapacityLowError,
    #[msg("Only captain can call this function")]
    NotCaptainError,
    #[msg("Member is not in the team")]
    MemberNotInTeamError,
    #[msg("Member is already in the team")]
    MemberAlreadyInTeamError,
    #[msg("Captain cannot leave the team unless he transfers the captain role to another member")]
    CaptainCannotLeaveTeamError,
    #[msg("Member is already voted for the tournament")]
    AlreadyVotedError,
    #[msg("The team has an active tournament and cannot vote for another tournament, leave the current one first")]
    AlreadyActiveTournamentError,
    #[msg("The team has no active tournament")]
    NoActiveTournamentError,
    #[msg("A team must contain 5 players to join a tournament")]
    NotEnoughPlayersError,
    #[msg("The sum of percentages must be equal to 100")]
    InvalidPercentageError,
    #[msg("Invalid member for that reward")]
    InvalidRewardError,
}
