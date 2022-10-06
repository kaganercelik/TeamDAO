use anchor_lang::prelude::*;

declare_id!("J6btWtBXncRisrhZMZaf2LBRW59D7XJZYYLz7HbMvm4v");

#[program]
pub mod team_dao {
    use super::*;

    // creating team
    // @param team_name: name of the team, used to create pda
    // @param team_id: id of the team, used to create pda
    pub fn create_team(ctx: Context<CreateTeam>, team_name: String, team_id: u64) -> Result<()> {
        let team = &mut ctx.accounts.team_account;

        team.bump = *ctx
            .bumps
            .get("team_account")
            .ok_or(ErrorCode::InvalidBumpSeeds)?;

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

        team.members.push(member);

        msg!("{} is successfully added to the team {}", member, team.name);

        Ok(())
    }
}

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

#[error_code]
pub enum ErrorCode {
    #[msg("A team can contain maximum 5 members")]
    TeamCapacityFullError,
    #[msg("Invalid bump seeds")]
    InvalidBumpSeeds,
}
