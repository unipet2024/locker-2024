use anchor_lang::prelude::*;

use crate::{
    AuthRole, AuthorityRole, Claim, Locker, LockerErrors, ADVISORS_ACCOUNT, ECOSYSTEM_ACCOUNT,
    FOUNDING_TEAM_ACCOUNT, LOCKER_ACCOUNT, OPERATOR_ROLE, PRIVATE_ROUND_ACCOUNT,
    PUBLIC_SALE_ACCOUNT, SEED_ROUND_ACCOUNT, TREASURY_ACCOUNT,
};

#[derive(Accounts)]
#[instruction(
    seed_round_pubkeys: Vec<Pubkey>,
    seed_round_locks: Vec<u64>,

    private_round_pubkeys: Vec<Pubkey>,
    private_round_locks: Vec<u64>,

    public_sale_pubkeys: Vec<Pubkey>,
    public_sale_locks: Vec<u64>,

    founding_team_pubkeys: Vec<Pubkey>,
    founding_team_locks: Vec<u64>,

    advisors_pubkeys: Vec<Pubkey>,
    advisors_locks: Vec<u64>,

    treasury_pubkeys: Vec<Pubkey>,
    treasury_locks: Vec<u64>,

    ecosystem_pubkeys: Vec<Pubkey>,
    ecosystem_locks: Vec<u64>
)]
pub struct SetClaim<'info> {
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
        realloc = 8 + 42 + 48 * seed_round_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [SEED_ROUND_ACCOUNT],
        bump=seed_round_account.bump,
    )]
    pub seed_round_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * private_round_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [PRIVATE_ROUND_ACCOUNT],
        bump=private_round_account.bump,
    )]
    pub private_round_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * public_sale_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [PUBLIC_SALE_ACCOUNT],
        bump=public_sale_account.bump,
    )]
    pub public_sale_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * founding_team_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [FOUNDING_TEAM_ACCOUNT],
        bump=founding_team_account.bump,
    )]
    pub founding_team_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * advisors_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [ADVISORS_ACCOUNT],
        bump=advisors_account.bump,
    )]
    pub advisors_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * treasury_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [TREASURY_ACCOUNT],
        bump=treasury_account.bump,
    )]
    pub treasury_account: Account<'info, Claim>,

    #[account(
        mut,
        realloc = 8 + 42 + 48 * ecosystem_pubkeys.len(),
        realloc::zero = true,
        realloc::payer=operator,
        seeds = [ECOSYSTEM_ACCOUNT],
        bump=ecosystem_account.bump,
    )]
    pub ecosystem_account: Account<'info, Claim>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn set_claim_handle(
    ctx: Context<SetClaim>,
    seed_round_pubkeys: Vec<Pubkey>,
    seed_round_locks: Vec<u64>,
    private_round_pubkeys: Vec<Pubkey>,
    private_round_locks: Vec<u64>,
    public_sale_pubkeys: Vec<Pubkey>,
    public_sale_locks: Vec<u64>,
    founding_team_pubkeys: Vec<Pubkey>,
    founding_team_locks: Vec<u64>,
    advisors_pubkeys: Vec<Pubkey>,
    advisors_locks: Vec<u64>,
    treasury_pubkeys: Vec<Pubkey>,
    treasury_locks: Vec<u64>,
    ecosystem_pubkeys: Vec<Pubkey>,
    ecosystem_locks: Vec<u64>,
) -> Result<()> {
    // SET SEED ROUND CLAIM
    
    let seed_round_account = &mut ctx.accounts.seed_round_account;
    seed_round_account.set_claims(seed_round_pubkeys, seed_round_locks)?;

    //SET PRIVATE ROUND CLAIM
    let private_round_account = &mut ctx.accounts.private_round_account;
    private_round_account.set_claims(private_round_pubkeys, private_round_locks)?;

    //SET PUBLIC SALE CLAIM
    let public_sale_account = &mut ctx.accounts.public_sale_account;
    public_sale_account.set_claims(public_sale_pubkeys, public_sale_locks)?;

    //SET FOUNDING CLAIM
    let founding_team_account = &mut ctx.accounts.founding_team_account;
    founding_team_account.set_claims(founding_team_pubkeys, founding_team_locks)?;

    //SET ADVISORS CLAIM
    let advisors_account = &mut ctx.accounts.advisors_account;
    advisors_account.set_claims(advisors_pubkeys, advisors_locks)?;

    //SET TREASURY CLAIM
    let treasury_account = &mut ctx.accounts.treasury_account;
    treasury_account.set_claims(treasury_pubkeys, treasury_locks)?;

    //SET ECOSYSTEM CLAIM
    let ecosystem_account = &mut ctx.accounts.ecosystem_account;
    ecosystem_account.set_claims(ecosystem_pubkeys, ecosystem_locks)?;

    Ok(())
}
