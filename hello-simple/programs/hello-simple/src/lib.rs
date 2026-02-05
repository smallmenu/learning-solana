use anchor_lang::prelude::*;

declare_id!("9PEM6jn8AbpUBycNL1dzWnmEewYW6QAXuWANVB1eyd9r");

#[program]
pub mod hello_simple {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
