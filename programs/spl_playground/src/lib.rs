use anchor_lang::prelude::*;

declare_id!("HS6fgmD8usW9sBgWBfaZqYzMqW8Gb38Bg58X1NKSe6m4");

#[program]
pub mod spl_playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
