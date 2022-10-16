use anchor_lang::prelude::*;
use team::*;

pub mod team;

declare_id!("FuQvo5fjJ2A3P3DXgSWsYX8Hsawd2Qg7LwohfSKhEBpu");

#[program]
pub mod team_dao {

    use super::*;

    use team::adding_member::{adding_member, AddMember};
    use team::can_join::{can_join, CanJoinTournament};
    use team::claiming_reward::{claiming_reward, ClaimReward};
    use team::creating_team::{creating_team, CreateTeam};
    use team::handle_distribute_proposal::{
        handle_distribute_proposal, DistributionProposalHandler,
    };
    use team::initing_percentage_proposal::{initing_percentage_proposal, InitPercentageProposal};
    use team::initing_tournament::{initing_tournament, InitTournament};
    use team::leaving_team::{leaving_team, LeaveTeam};
    use team::leaving_tournament::{leaving_tournament, LeaveTournament};
    use team::removing_member::{removing_member, RemoveMember};
    use team::transfering_captain::{transfering_captain, TransferCaptain};
    use team::voting_for_tournament::{voting_for_tournament, VoteForTournament};
    use team::VoteType;

    // ----------------------------------------------

    // instructions that can be called by captain

    // creating team
    // @param team_name: name of the team, used to create pda
    // @param team_id: id of the team, used to create pda
    pub fn create_team(ctx: Context<CreateTeam>, team_name: String, team_id: u64) -> Result<()> {
        return creating_team(ctx, team_name, team_id);
    }

    // adding member to team
    // @param team_name: name of the team, used in pda
    // @param team_id: id of the team, used in pda
    // @param member: member's public key to add to team
    pub fn add_member(
        ctx: Context<AddMember>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return adding_member(ctx, _team_name, _team_id, member);
    }

    // removing member from team
    // @param team_name: name of the team, used in pda
    // @param team_id: id of the team, used in pda
    // @param member: member's public key to remove from team
    pub fn remove_member(
        ctx: Context<RemoveMember>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return removing_member(ctx, _team_name, _team_id, member);
    }

    // transferring captain role to another member
    // @param team_name: name of the team, used in pda
    // @param team_id: id of the team, used in pda
    // @param member: member's public key to transfer captain role to
    pub fn transfer_captain(
        ctx: Context<TransferCaptain>,
        _team_name: String,
        _team_id: u64,
        member: Pubkey,
    ) -> Result<()> {
        return transfering_captain(ctx, _team_name, _team_id, member);
    }

    // ----------------------------------------------
    // instructions that can be called by anyone by players in the team

    // leaving team
    // @param team_name: name of the team, used in pda
    // @param team_id: id of the team, used in pda
    pub fn leave_team(ctx: Context<LeaveTeam>, _team_name: String, _team_id: u64) -> Result<()> {
        return leaving_team(ctx, _team_name, _team_id);
    }

    // init tournament
    // @param tournament_name: name of the tournament, used in pda
    // @param tournament_id: id of the tournament, used in pda
    // @param tournament_prize: prize of the tournament
    pub fn init_tournament(
        ctx: Context<InitTournament>,
        _team_name: String,
        _team_id: u64,
        tournament_address: Pubkey,
        tournament_prize: u64,
    ) -> Result<()> {
        return initing_tournament(
            ctx,
            _team_name,
            _team_id,
            tournament_address,
            tournament_prize,
        );
    }

    // vote for tournament
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    // @param tournament_address : tournament address"
    pub fn vote_for_tournament(
        ctx: Context<VoteForTournament>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return voting_for_tournament(ctx, _team_name, _team_id, vote_type);
    }

    // leave a tournament
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn leave_tournament(
        ctx: Context<LeaveTournament>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return leaving_tournament(ctx, _team_name, _team_id, vote_type);
    }

    // init percentage proposal
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn init_percentage_proposal(
        ctx: Context<InitPercentageProposal>,
        _team_name: String,
        _team_id: u64,
        percentages: Vec<u8>,
    ) -> Result<()> {
        return initing_percentage_proposal(ctx, _team_name, _team_id, percentages);
    }

    // reward distribution proposal handler
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn distribution_proposal_handler(
        ctx: Context<DistributionProposalHandler>,
        _team_name: String,
        _team_id: u64,
        vote_type: VoteType,
    ) -> Result<()> {
        return handle_distribute_proposal(ctx, _team_name, _team_id, vote_type);
    }
    // two functions above will basically be used to vote for the distribution of the rewards
    // the function below will use the logic to decide if a team can join the tournament or not

    // can join the tournament, we will use this function to decide if a team can join the tournament or not
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn can_join_tournament(
        ctx: Context<CanJoinTournament>,
        _team_name: String,
        _team_id: u64,
    ) -> Result<()> {
        return can_join(ctx, _team_name, _team_id);
    }

    // distribute rewards
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        _team_name: String,
        _team_id: u64,
        reward: u64,
    ) -> Result<()> {
        return claiming_reward(ctx, _team_name, _team_id, reward);
    }
}
