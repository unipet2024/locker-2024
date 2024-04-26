use anchor_lang::prelude::*;

use crate::{
    AuthRole, AuthorityRole, Claim, ClaimType, Locker, ADMIN_ROLE, ADVISORS_ACCOUNT,
    ECOSYSTEM_ACCOUNT, FOUNDING_TEAM_ACCOUNT, LOCKER_ACCOUNT, OPERATOR_ROLE, PRIVATE_ROUND_ACCOUNT,
    PUBLIC_SALE_ACCOUNT, SEED_ROUND_ACCOUNT, TREASURY_ACCOUNT,
};

#[derive(Accounts)]
#[instruction(unp: Pubkey)]
pub struct InitLocker<'info> {
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 97,
        seeds = [LOCKER_ACCOUNT],
        bump
    )]
    pub locker: Box<Account<'info, Locker>>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE],
        bump,
    )]
    pub admin_account: Box<Account<'info, AuthorityRole>>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [OPERATOR_ROLE],
        bump,
    )]
    pub operator_account: Box<Account<'info, AuthorityRole>>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [SEED_ROUND_ACCOUNT],
        bump,
    )]
    pub seed_round_account: Box<Account<'info, Claim>>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [PRIVATE_ROUND_ACCOUNT],
        bump,
    )]
    pub private_round_account: Account<'info, Claim>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [PUBLIC_SALE_ACCOUNT],
        bump,
    )]
    pub public_sale_account: Account<'info, Claim>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [FOUNDING_TEAM_ACCOUNT],
        bump,
    )]
    pub founding_team_account: Account<'info, Claim>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [ADVISORS_ACCOUNT],
        bump,
    )]
    pub advisors_account: Account<'info, Claim>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [TREASURY_ACCOUNT],
        bump,
    )]
    pub treasury_account: Account<'info, Claim>,

    #[account(
        init,
        space = 8 + 42 + 48,
        payer = authority,
        seeds = [ECOSYSTEM_ACCOUNT],
        bump,
    )]
    pub ecosystem_account: Account<'info, Claim>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_handle(ctx: Context<InitLocker>, unp: Pubkey) -> Result<()> {
    let locker = &mut ctx.accounts.locker;
    let admin_account = &mut ctx.accounts.admin_account;
    let operator_account = &mut ctx.accounts.operator_account;

    msg!("UNP: {:}", unp);

    locker.init(
        admin_account.key(),
        operator_account.key(),
        unp,
        ctx.bumps.locker,
    )?;

    //SET ADMIN
    let authorities = vec![ctx.accounts.authority.key()];
    admin_account.initialize(&authorities, ctx.bumps.admin_account, AuthRole::Admin)?;

    //SET OPERATOR
    operator_account.initialize(&authorities, ctx.bumps.operator_account, AuthRole::Operator)?;

    // // SET SEED ROUND CLAIM
    // let seed_round_account = &mut ctx.accounts.seed_round_account;
    // seed_round_account.init(
    //     ClaimType::SeedRound,
    //     2 * 86400 * 30,
    //     18 * 86400 * 30,
    //     750,
    //     ctx.bumps.seed_round_account,
    // )?;

    // //SET PRIVATE ROUND CLAIM
    // let private_round_account = &mut ctx.accounts.private_round_account;
    // private_round_account.init(
    //     ClaimType::PrivateRound,
    //     2 * 86400 * 30,
    //     12 * 86400 * 30,
    //     1250,
    //     ctx.bumps.private_round_account,
    // )?;

    // //SET PUBLIC SALE CLAIM
    // let public_sale_account = &mut ctx.accounts.public_sale_account;
    // public_sale_account.init(
    //     ClaimType::PrivateRound,
    //     2 * 86400 * 30,
    //     8 * 86400 * 30,
    //     1500,
    //     ctx.bumps.public_sale_account,
    // )?;

    // //SET FOUNDING CLAIM
    // let founding_team_account = &mut ctx.accounts.founding_team_account;
    // founding_team_account.init(
    //     ClaimType::PrivateRound,
    //     12 * 86400 * 30,
    //     36 * 86400 * 30,
    //     0,
    //     ctx.bumps.founding_team_account,
    // )?;

    // //SET ADVISORS CLAIM
    // let advisors_account = &mut ctx.accounts.advisors_account;
    // advisors_account.init(
    //     ClaimType::PrivateRound,
    //     3 * 86400 * 30,
    //     18 * 86400 * 30,
    //     0,
    //     ctx.bumps.advisors_account,
    // )?;

    // //SET TREASURY CLAIM
    // let treasury_account = &mut ctx.accounts.treasury_account;
    // treasury_account.init(
    //     ClaimType::PrivateRound,
    //     12 * 86400 * 30,
    //     48 * 86400 * 30,
    //     0,
    //     ctx.bumps.treasury_account,
    // )?;

    // //SET ECOSYSTEM CLAIM
    // let ecosystem_account = &mut ctx.accounts.ecosystem_account;
    // ecosystem_account.init(
    //     ClaimType::PrivateRound,
    //     0,
    //     38 * 86400 * 30,
    //     800,
    //     ctx.bumps.ecosystem_account,
    // )?;

    /*  TESTING */
    // SET SEED ROUND CLAIM
    let seed_round_account = &mut ctx.accounts.seed_round_account;
    seed_round_account.init(
        ClaimType::SeedRound,
        2,
        18,
        750,
        ctx.bumps.seed_round_account,
    )?;

    //SET PRIVATE ROUND CLAIM
    let private_round_account = &mut ctx.accounts.private_round_account;
    private_round_account.init(
        ClaimType::PrivateRound,
        2,
        12,
        1250,
        ctx.bumps.private_round_account,
    )?;

    //SET PUBLIC SALE CLAIM
    let public_sale_account = &mut ctx.accounts.public_sale_account;
    public_sale_account.init(
        ClaimType::PrivateRound,
        2,
        8,
        1500,
        ctx.bumps.public_sale_account,
    )?;

    //SET FOUNDING CLAIM
    let founding_team_account = &mut ctx.accounts.founding_team_account;
    founding_team_account.init(
        ClaimType::PrivateRound,
        12,
        36,
        0,
        ctx.bumps.founding_team_account,
    )?;

    //SET ADVISORS CLAIM
    let advisors_account = &mut ctx.accounts.advisors_account;
    advisors_account.init(
        ClaimType::PrivateRound,
        3,
        18,
        0,
        ctx.bumps.advisors_account,
    )?;

    //SET TREASURY CLAIM
    let treasury_account = &mut ctx.accounts.treasury_account;
    treasury_account.init(
        ClaimType::PrivateRound,
        12,
        48,
        0,
        ctx.bumps.treasury_account,
    )?;

    //SET ECOSYSTEM CLAIM
    let ecosystem_account = &mut ctx.accounts.ecosystem_account;
    ecosystem_account.init(
        ClaimType::PrivateRound,
        0,
        38,
        800,
        ctx.bumps.ecosystem_account,
    )?;

    Ok(())
}
