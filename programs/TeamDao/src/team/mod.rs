use anchor_lang::prelude::*;

pub mod adding_member;
pub mod can_join;
pub mod claiming_reward;
pub mod creating_team;
pub mod errors;
pub mod handle_distribute_proposal;
pub mod initing_percentage_proposal;
pub mod initing_tournament;
pub mod leaving_team;
pub mod leaving_tournament;
pub mod removing_member;
pub mod transfering_captain;
pub mod voting_for_tournament;

pub use adding_member::*;
pub use can_join::*;
pub use claiming_reward::*;
pub use creating_team::*;
pub use errors::ErrorCode;
pub use handle_distribute_proposal::*;
pub use initing_percentage_proposal::*;
pub use initing_tournament::*;
pub use leaving_team::*;
pub use leaving_tournament::*;
pub use removing_member::*;
pub use transfering_captain::*;
pub use voting_for_tournament::*;

// Team account struct
#[account]
pub struct TeamAccount {
    pub captain: Pubkey,
    pub bump: u8,
    pub name: String,
    pub members: Vec<Pubkey>,
    pub id: u64,
    pub is_initialized: bool,
    pub yes_votes: u8,
    pub voted_players: Vec<Pubkey>,
    pub active_tournament: Pubkey,
    pub prize: u64,
    pub voting_result: bool,
    pub leave_votes: u8,
    pub leave_voted_players: Vec<Pubkey>,
    pub distribution_percentages: Vec<u8>,
    pub distribution_yes_votes: u8,
    pub distribution_voted_players: Vec<Pubkey>,
    pub distribution_voting_result: bool,
    pub can_join_tournament: bool,
}

impl TeamAccount {
    const LEN: usize = 8 // discriminator 
    + 32 // captain pubkey 
    + 1 // bump 
    + 32 // name
    + 5 * 32 // members vector 
    + 8 // id
    + 1 // is_initialized
    + 1 // yes_votes
    + 5 * 32 // voted_players vector
    + 32 // active_tournament
    + 8 // tournament_prize
    + 1 // voting_result
    + 1 * 5 // reward_distribution_percentages vector
    + 1 // distribution_yes_votes
    + 5 * 32 // distribution_voted_players vector
    + 1 // distribution_voting_result
    + 1; // can_join_tournament
} // 612 bytes < 10k

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    Yes,
    No,
}
