use anchor_lang::prelude::*;

declare_id!("5BxFJmMoRoF4PPeHqw8RCwv4QJuPeQYohYbnYcorGr7B");

#[program]
pub mod streamstake_contracts {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
