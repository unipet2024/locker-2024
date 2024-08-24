use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{Claim, ClaimType, LockerErrors, UserClaimEvent};

use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};

#[derive(Accounts)]
#[instruction(
    claim_type:  ClaimType,
)]
pub struct UserClaim<'info> {
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = claim_account
    )]
    pub claim_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [claim_type.get_seeds()],
        bump=claim_account.bump,
    )]
    pub claim_account: Account<'info, Claim>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl UserClaim<'_> {
    fn validate(&self) -> Result<()> {
        let current = Clock::get()?.unix_timestamp as u64;

        require_gte!(self.claim_account.start_time, 0, LockerErrors::StillLock);
        require_gte!(
            current,
            self.claim_account.start_time,
            LockerErrors::StillLock
        );

        Ok(())
    }

    #[access_control(ctx.accounts.validate())]
    pub fn user_claim_handle(ctx: Context<Self>, claim_type: ClaimType) -> Result<()> {
        let claim_account = &mut ctx.accounts.claim_account;
        let user = &mut ctx.accounts.user;

        let current = Clock::get().unwrap().unix_timestamp as u64;
        msg!("Current: {:}", current);

        let (index, amount) = claim_account.get_claim_info(user.key(), current);
        require_gt!(amount, 0, LockerErrors::NothingToClaim);

        //transfer UNP from locker to user
        // let token_mint = &mut ctx.accounts.token_mint;
        // msg!(
        //     "Tranfer {:} {:} to {:}",
        //     amount,
        //     token_mint.key(),
        //     user.key()
        // );

        let seeds: &[&[u8]] = &[claim_type.get_seeds(), &[claim_account.bump]];
        let signer = &[&seeds[..]];

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    authority: claim_account.to_account_info(),
                    from: ctx.accounts.claim_ata.to_account_info(),
                    to: ctx.accounts.user_ata.to_account_info(),
                },
            )
            .with_signer(signer),
            amount,
        )?;

        //update claim account
        claim_account.claim_data[index].released += amount;

        emit!(UserClaimEvent {
            claim_type,
            claimer: user.key(),
            amount: amount,
            time: current
        });

        Ok(())
    }
}
