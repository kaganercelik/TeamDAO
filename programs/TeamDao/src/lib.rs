use anchor_lang::prelude::*;

declare_id!("DX9sn7m7pn3zQJP5B5oD5YQVQWxen9CX77u8rEqMFC41");

#[program]
pub mod team_dao {
    use super::*;

    // ----------------------------------------------

    // instructions that can be called by captain

    // creating team
    // @param team_name: name of the team, used to create pda
    // @param team_id: id of the team, used to create pda
    pub fn create_team(ctx: Context<CreateTeam>, team_name: String, team_id: u64) -> Result<()> {
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

        msg!("Team created");
        msg!("Team name: {}", team.name);
        msg!("Team captain: {}", team.captain);

        Ok(())
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

    // ----------------------------------------------
    // instructions that can be called by anyone by players in the team

    // leaving team
    // @param team_name: name of the team, used in pda
    // @param team_id: id of the team, used in pda
    pub fn leave_team(ctx: Context<LeaveTeam>, _team_name: String, _team_id: u64) -> Result<()> {
        let team = &mut ctx.accounts.team_account;

        // checking if the signer is in the team
        require!(
            team.members.contains(ctx.accounts.signer.key),
            ErrorCode::MemberNotInTeamError
        );

        if team.members.len() == 1 {
            // if the captain is the last member disband team
            // delete team
            team.name = "".to_string();
            team.captain = Pubkey::default();
            team.id = 0;
            team.members = vec![];
        }
        if team.captain == *ctx.accounts.signer.key {
            // transfer captain role to the second member in the team
            team.captain = team.members[1];
        }

        // deleting the member from team
        team.members.retain(|&x| x != *ctx.accounts.signer.key);

        msg!(
            "{} is successfully removed from the team {}",
            ctx.accounts.signer.key,
            team.name
        );

        Ok(())
    }

    // vote for tournament
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    // @param tournament_address : tournament address
    pub fn vote_for_tournament(
        ctx: Context<VoteForTournament>,
        _team_name: String,
        _team_id: u64,
        tournament_address: Pubkey,
        vote_type: VoteType,
    ) -> Result<()> {
        let team = &mut ctx.accounts.team_account;

        // checking if the team still has an active tournament
        require!(
            team.active_tournament == Pubkey::default(),
            ErrorCode::AlreadyActiveTournamentError
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
            team.active_tournament = tournament_address;
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

    // leave a tournament
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn leave_tournament(
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

    // init percentage proposal
    // @param _team_name : name of the team, used in pda
    // @param _team_id : id of the team, used in pda
    pub fn init_percentage_proposal(
        ctx: Context<InitPercentageProposal>,
        _team_name: String,
        _team_id: u64,
        percentages: Vec<u8>,
    ) -> Result<()> {
        let team = &mut ctx.accounts.team_account;
        // sum of the percentages vector
        let sum: u8 = percentages.iter().sum();
        // checking if the sum of percentages is equal to 100
        require!(sum == 100, ErrorCode::InvalidPercentageError);

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

        // checking if the captain is the signer
        require!(
            team.captain == *ctx.accounts.signer.key,
            ErrorCode::NotCaptainError
        );

        // setting the percentage proposal
        team.distribution_percentages = percentages;

        msg!(
            "{} is successfully proposed a percentage {:?}",
            team.name,
            team.distribution_percentages
        );

        Ok(())
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
                team.distribution_voted_players
                    .push(*ctx.accounts.signer.key);
                // incrementing yes votes
                team.distribution_yes_votes += 1;
            }
            VoteType::No => {
                // adding the player to voted players
                team.voted_players.push(*ctx.accounts.signer.key);
            }
        }

        Ok(())
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
        let team = &mut ctx.accounts.team_account;

        // checking if the team has 5 players to join the tournament
        require!(team.members.len() == 5, ErrorCode::NotEnoughPlayersError);

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

        // checking if voting result is yes, all players voted for tournament and distribution
        if team.voting_result == true && team.distribution_yes_votes > 2 {
            team.can_join_tournament = true;
        } else {
            team.can_join_tournament = false;
        }

        Ok(())
    }
}

// derive macros for member instructions

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

#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct RemoveMember<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
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

// leave team instruction
#[derive(Accounts)]
#[instruction(team_name: String, team_id: u64)]
pub struct LeaveTeam<'info> {
    #[account(mut, seeds=[team_name.as_bytes(), &team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
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

// init percentage proposal
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct InitPercentageProposal<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// vote for distribution
#[derive(Accounts)]
#[instruction(_team_name: String, _team_id: u64)]
pub struct DistributionProposalHandler<'info> {
    #[account(mut, seeds=[_team_name.as_bytes(), &_team_id.to_ne_bytes()], bump = team_account.bump)]
    pub team_account: Account<'info, TeamAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
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
    pub voting_result: bool,
    pub leave_votes: u8,
    pub leave_voted_players: Vec<Pubkey>,
    pub distribution_percentages: Vec<u8>,
    pub distribution_yes_votes: u8,
    pub distribution_voted_players: Vec<Pubkey>,
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
    + 1 // voting_result
    + 1 * 5 // reward_distribution_percentages vector
    + 1 // distribution_yes_votes
    + 5 * 32 // distribution_voted_players vector
    + 1; // can_join_tournament
} // 603 bytes < 10k

// ----------------------------------------------
// voting related instructions and accounts

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    Yes,
    No,
}

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
}
