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

    // ----------------------------------------------
    // voting related instructions

    // creating vote of a spesific team
    // @param team_account_address : address of the team account, used to create pda
    pub fn init_vote(ctx: Context<InitializeVote>, team_account_address: Pubkey) -> Result<()> {
        let vote = &mut ctx.accounts.vote_account;

        vote.bump = *ctx
            .bumps
            .get("vote_account")
            .ok_or(ErrorCode::InvalidBumpSeeds)?;

        // assigning required parameters to the vote
        vote.team = team_account_address;
        vote.yes = 0;
        vote.no = 0;

        msg!("Vote created");

        Ok(())
    }

    // members voting
    // @param team_account_address: address of the team account, used to to find the vote account
    // @param vote: vote to cast
    pub fn vote(
        ctx: Context<Vote>,
        _team_account_address: Pubkey,
        vote_type: VoteType,
    ) -> Result<()> {
        let vote = &mut ctx.accounts.vote_account;

        // checking if the vote is already closed
        require!(vote.yes + vote.no < 5, ErrorCode::VoteClosedError);

        // checking if the signer has already voted
        require!(
            !vote.voters.contains(ctx.accounts.signer.key),
            ErrorCode::AlreadyVotedError
        );

        // adding the voter to the list of voters
        vote.voters.push(*ctx.accounts.signer.key);

        // casting the vote
        match vote_type {
            VoteType::Yes => vote.yes += 1,
            VoteType::No => vote.no += 1,
        }

        msg!("Vote casted");

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

// Team account struct
#[account]
pub struct TeamAccount {
    pub captain: Pubkey,
    pub bump: u8,
    pub name: String,
    pub members: Vec<Pubkey>,
    pub id: u64,
}

impl TeamAccount {
    const LEN: usize = 8 // discriminator 
    + 32 // captain pubkey 
    + 1 // bump 
    + 32 // name
    + 5 * 32 // members vector 
    + 8; // id
}

// ----------------------------------------------
// voting related instructions and accounts

// derive macro for initialize vote instruction
#[derive(Accounts)]
#[instruction(_team_account_address: Pubkey)]
pub struct InitializeVote<'info> {
    #[account(init, payer = signer, space= VoteAccount::LEN, seeds=[_team_account_address.as_ref()], bump)]
    pub vote_account: Account<'info, VoteAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// derive macro for vote instruction
#[derive(Accounts)]
#[instruction(_team_account_address: Pubkey)]
pub struct Vote<'info> {
    #[account(mut, seeds=[_team_account_address.as_ref()], bump = vote_account.bump)]
    pub vote_account: Account<'info, VoteAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// vote account struct
#[account]
pub struct VoteAccount {
    pub team: Pubkey,
    pub bump: u8,
    pub yes: u8,
    pub no: u8,
    pub id: u8,
    pub voters: Vec<Pubkey>,
}

impl VoteAccount {
    const LEN: usize = 8 // discriminator 
    + 32 // team pubkey 
    + 1 // bump 
    + 1 // votes yes
    + 1 // votes no
    + 1 // id
    + 5 * 32; // voters vector
}

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
    #[msg("You have already voted")]
    AlreadyVotedError,
    #[msg("Voting is closed for this team")]
    VoteClosedError,
    #[msg("Invalid vote")]
    InvalidVoteError,
}
