use anchor_lang::{prelude::*, solana_program::vote::error};
use std::collections::HashMap;

declare_id!("5BxFJmMoRoF4PPeHqw8RCwv4QJuPeQYohYbnYcorGr7B");

#[program]
pub mod streamstake_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount:u64) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let state = &mut ctx.accounts.state;

        **user.to_account_info().try_borrow_mut_lamports()? -= amount;
        **state.to_account_info().try_borrow_mut_lamports()? += amount;
        
        let user_key = ctx.accounts.user.key();
        let mut found = false;
        for deposit in &mut state.deposits {
            if deposit.0 == user_key {
                deposit.1 += amount; // Update the deposit amount if the user already exists
                found = true;
                break;
            }
        }
        if !found {
            state.deposits.push((user_key, amount)); // Add a new deposit entry if the user is new
        }

        msg!("User {} deposited {}", ctx.accounts.user.key(), amount);
        Ok(())
    }

    pub fn transfer_sol(ctx: Context<TransferSol>, amount: u64, to: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;

        // Ensure only the owner can transfer funds
        require!(state.owner == *ctx.accounts.owner.key, ErrorCode::OnlyOwner);

        // Transfer SOL from contract to recipient
        **state.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.to.to_account_info().try_borrow_mut_lamports()? += amount;

        msg!("Transferred {} lamports to {}", amount, to);
        Ok(())
    }
}

#[account]
pub struct State {
    pub owner: Pubkey,
    pub deposits: Vec<(Pubkey, u64)>,
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

#[error_code]
pub enum ErrorCode {
    #[msg(Only owner can perform this action)]
    OnlyOwner,
}