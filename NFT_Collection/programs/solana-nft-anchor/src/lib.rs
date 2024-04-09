
use anchor_lang::prelude::*;

declare_id!("DSnujzVSNxL9RRwt1caKZWthi9WDqa7TQLJqrx1rYhbj"); //the program id above will be different to mine

#[program]
pub mod solana_nft_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitNFT<'info> {
    /// CHECK: ok, we are passing in this account ourselves
    #[account(mut, signer)]
    signer: AccountInfo<'info>
}