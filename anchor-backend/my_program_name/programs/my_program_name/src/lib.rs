use anchor_lang::prelude::*;

declare_id!("D4QhG7nNFPFgrtzhXqz46YGc9jujV93mazdYhMPSyoEP");

#[program]
pub mod my_program_name {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
