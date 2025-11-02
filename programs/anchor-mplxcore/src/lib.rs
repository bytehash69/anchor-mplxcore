use anchor_lang::prelude::*;

declare_id!("86BV8L9Vf1a2idboxzewZ7K1ybMrTDHip8sZNQUkm3dd");

#[program]
pub mod anchor_mplxcore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
