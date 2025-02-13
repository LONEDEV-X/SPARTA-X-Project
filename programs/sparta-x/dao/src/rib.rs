use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[program]
pub mod sparta_x_dao {
    use super::*;

    pub fn create_proposal(ctx: Context<CreateProposal>, title: String, description: String) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        proposal.title = title;
        proposal.description = description;
        proposal.creator = *ctx.accounts.creator.key;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, approve: bool) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let voter = &ctx.accounts.voter;

        require!(proposal.creator != voter.key(), CustomError::CreatorCannotVote);
        
        if approve {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = creator, space = 8 + 256)]
    pub proposal: Account<'info, Proposal>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Signer<'info>,
}

#[account]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub creator: Pubkey,
    pub votes_for: u64,
    pub votes_against: u64,
}

#[error_code]
pub enum CustomError {
    #[msg("Creator cannot vote on their own proposal.")]
    CreatorCannotVote,
}
