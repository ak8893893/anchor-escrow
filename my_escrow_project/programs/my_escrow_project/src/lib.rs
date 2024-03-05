use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Mint};


declare_id!("7sQMvJeWH3JCyrHLfxWXg678NSKYonm1xCoDB83D5GdH");

#[account]
pub struct Escrow {
    pub initializer: Pubkey,
    pub temp_token_account: Pubkey,
    pub initializer_token_to_receive_account: Pubkey,
    pub expected_amount: u64,
}

#[program]
pub mod my_escrow_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = initializer, space = 8 + 8 + 32 + 32 + 32)]
    pub escrow_account: Account<'info, Escrow>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self, expected_amount: u64) -> ProgramResult {
        let escrow = &mut self.escrow_account;
        escrow.initializer = *self.initializer.key;
        escrow.expected_amount = expected_amount;
        Ok(())
    }
}

