use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    Claim, ClaimType, Locker, LockerErrors, UserClaimEvent, ADVISORS_ACCOUNT, ECOSYSTEM_ACCOUNT,
    FOUNDING_TEAM_ACCOUNT, LOCKER_ACCOUNT, PRIVATE_ROUND_ACCOUNT, PUBLIC_SALE_ACCOUNT,
    SEED_ROUND_ACCOUNT, TREASURY_ACCOUNT,
};

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
        seeds = [LOCKER_ACCOUNT],
        bump=locker.bump
    )]
    pub locker: Box<Account<'info, Locker>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = locker
    )]
    pub locker_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            match claim_type {
                ClaimType::SeedRound => SEED_ROUND_ACCOUNT,
                ClaimType::PrivateRound => PRIVATE_ROUND_ACCOUNT,
                ClaimType::PublisSale => PUBLIC_SALE_ACCOUNT,
                ClaimType::FoundingTeam => FOUNDING_TEAM_ACCOUNT,
                ClaimType::Advisors => ADVISORS_ACCOUNT,
                ClaimType::Treasury =>TREASURY_ACCOUNT,
                ClaimType::Ecosystem =>ECOSYSTEM_ACCOUNT
            }
        ],
        bump=claim_account.bump,
    )]
    pub claim_account: Account<'info, Claim>,

    #[account(
        init_if_needed,
        payer=claimer,
        associated_token::mint = token_mint,
        associated_token::authority = claimer
    )]
    pub claim_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub claimer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub token_mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn user_claim_handle(ctx: Context<UserClaim>, claim_type: ClaimType) -> Result<()> {
    let locker = &mut ctx.accounts.locker;
    // let locker_ata = &mut ctx.accounts.locker_ata;
    let claim_account = &mut ctx.accounts.claim_account;
    let claimer = &mut ctx.accounts.claimer;

    let current = Clock::get()?.unix_timestamp as u64;
    require_gte!(current, claim_account.start_time, LockerErrors::StillLock);

    let (index, check) = claim_account.get_claim_index(claimer.key());
    require_eq!(check, false, LockerErrors::NotAllowToClaim);

    let amount = claim_account.get_claim_amount(claimer.key(), current);
    require_gt!(amount, 0, LockerErrors::NothingToClaim);

    //transfer UNP from locker to claimer
    let token_mint = &mut ctx.accounts.token_mint;
    msg!(
        "Tranfer {:} {:} to {:}",
        amount,
        token_mint.key(),
        claimer.key()
    );

    let seeds: &[&[u8]] = &[LOCKER_ACCOUNT, &[locker.bump]];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: locker.to_account_info(),
                from: ctx.accounts.locker_ata.to_account_info(),
                to: ctx.accounts.claim_ata.to_account_info(),
            },
        )
        .with_signer(signer),
        amount,
    )?;

    //update claim account
    claim_account.claim_data[index].released += amount;

    emit!(UserClaimEvent {
        claim_type,
        claimer: claimer.key(),
        amount: amount,
        time: current
    });

    Ok(())
}
