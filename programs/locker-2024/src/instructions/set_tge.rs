use anchor_lang::prelude::*;

use crate::{
    AuthRole, AuthorityRole, Claim, Locker, LockerErrors, ADVISORS_ACCOUNT, ECOSYSTEM_ACCOUNT,
    FOUNDING_TEAM_ACCOUNT, LOCKER_ACCOUNT, OPERATOR_ROLE, PRIVATE_ROUND_ACCOUNT,
    PUBLIC_SALE_ACCOUNT, SEED_ROUND_ACCOUNT, TREASURY_ACCOUNT,
};

#[derive(Accounts)]
#[instruction(tge: u64)]
pub struct SetTge<'info> {
    #[account(
        seeds = [LOCKER_ACCOUNT],
        constraint = locker.operator == operator_account.key() @ LockerErrors::OperatorAccountInvalid,
        bump=locker.bump
    )]
    pub locker: Box<Account<'info, Locker>>,
    #[account(
        seeds = [OPERATOR_ROLE],
        bump=operator_account.bump,
        constraint = operator_account.is_authority(operator.key) == true @ LockerErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ LockerErrors::OnlyOperator,
        constraint = operator_account.status == true @ LockerErrors::OnlyOperator,
    )]
    pub operator_account: Account<'info, AuthorityRole>,

    #[account(
        mut,
        seeds = [SEED_ROUND_ACCOUNT],
        bump=seed_round_account.bump,
    )]
    pub seed_round_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [PRIVATE_ROUND_ACCOUNT],
        bump=private_round_account.bump,
    )]
    pub private_round_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [PUBLIC_SALE_ACCOUNT],
        bump=public_sale_account.bump,
    )]
    pub public_sale_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [FOUNDING_TEAM_ACCOUNT],
        bump=founding_team_account.bump,
    )]
    pub founding_team_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [ADVISORS_ACCOUNT],
        bump=advisors_account.bump,
    )]
    pub advisors_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [TREASURY_ACCOUNT],
        bump=treasury_account.bump,
    )]
    pub treasury_account: Account<'info, Claim>,

    #[account(
        mut,
        seeds = [ECOSYSTEM_ACCOUNT],
        bump=ecosystem_account.bump,
    )]
    pub ecosystem_account: Account<'info, Claim>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn set_tge_handle(ctx: Context<SetTge>, tge: u64) -> Result<()> {
    // SET SEED ROUND CLAIM

    let seed_round_account = &mut ctx.accounts.seed_round_account;
    seed_round_account.set_tge(tge)?;

    //SET PRIVATE ROUND CLAIM
    let private_round_account = &mut ctx.accounts.private_round_account;
    private_round_account.set_tge(tge)?;

    //SET PUBLIC SALE CLAIM
    let public_sale_account = &mut ctx.accounts.public_sale_account;
    public_sale_account.set_tge(tge)?;

    //SET FOUNDING CLAIM
    let founding_team_account = &mut ctx.accounts.founding_team_account;
    founding_team_account.set_tge(tge)?;

    //SET ADVISORS CLAIM
    let advisors_account = &mut ctx.accounts.advisors_account;
    advisors_account.set_tge(tge)?;

    //SET TREASURY CLAIM
    let treasury_account = &mut ctx.accounts.treasury_account;
    treasury_account.set_tge(tge)?;

    //SET ECOSYSTEM CLAIM
    let ecosystem_account = &mut ctx.accounts.ecosystem_account;
    ecosystem_account.set_tge(tge)?;

    Ok(())
}
