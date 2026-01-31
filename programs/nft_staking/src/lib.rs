use anchor_lang::prelude::*;

declare_id!("2S4tV2QWuerwZQjcj3fwyCySPFEPKZatJsLHvdvsHfnb");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn stake(ctx: Context<Stake>, token_id: u64) -> Result<()> {
        let stake = &mut ctx.accounts.stake_data;
        stake.owner = ctx.accounts.user.key();
        stake.token_id = token_id;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        let stake = &ctx.accounts.stake_data;
        require!(
            stake.owner == ctx.accounts.user.key(),
            StakingError::NotOwner
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    pub stake_data: Account<'info, StakeData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub stake_data: Account<'info, StakeData>,
    pub user: Signer<'info>,
}

#[account]
pub struct StakeData {
    pub owner: Pubkey,
    pub token_id: u64,
}

#[error_code]
pub enum StakingError {
    #[msg("Not NFT owner")]
    NotOwner,
}
