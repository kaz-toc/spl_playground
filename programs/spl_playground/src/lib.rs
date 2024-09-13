use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, TokenAccount, Token};

declare_id!("HS6fgmD8usW9sBgWBfaZqYzMqW8Gb38Bg58X1NKSe6m4");

#[program]
pub mod spl_playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.is_transferable = true; // 初期状態ではトークン送信を許可
        Ok(())
    }

    pub fn set_transferability(ctx: Context<SetTransferability>, is_transferable: bool) -> Result<()> {
        let account = &mut ctx.accounts.my_account;
        account.is_transferable = is_transferable; // トークンの送信許可フラグを設定
        Ok(())
    }

    pub fn transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let account = &ctx.accounts.my_account;

        // トークンの送信が許可されているかをチェック
        if !account.is_transferable {
            return Err(ErrorCode::TransferNotAllowed.into());
        }

        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetTransferability<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    #[account(mut)]
    pub my_account: Account<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub is_transferable: bool, // トークンの送信可否を管理するフラグ
}

#[error_code]
pub enum ErrorCode {
    #[msg("Token transfers are not allowed.")]
    TransferNotAllowed,
}

