use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("5BxFJmMoRoF4PPeHqw8RCwv4QJuPeQYohYbnYcorGr7B");

#[program]
pub mod streamstake_contracts {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter account created");
        msg!("Current count: { }", counter.count);
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub deposits: HashMap<Pubkey, u64>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 10000)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
