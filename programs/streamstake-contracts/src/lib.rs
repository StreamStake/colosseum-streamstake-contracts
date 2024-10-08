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
        let state = &mut ctx.accounts.state;
        state.owner = *ctx.accounts.owner.key;
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

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_account: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferSol<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program:Program<'info, System>
}