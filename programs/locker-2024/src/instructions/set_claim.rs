use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{AuthRole, AuthorityRole, Claim, ClaimType, Locker, LockerErrors};

use crate::constants::*;

use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};

#[derive(Accounts)]
#[instruction(
    claim_type: ClaimType,
    full_lock: u64,
    vesting: u64,
    tge_release: u16, // 2 decimal
    token: Pubkey,
    claimers: Vec<Pubkey>,
    locks: Vec<u64>,
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
        init,
        payer=operator,
        associated_token::mint = mint,
        associated_token::authority = claim_account
    )]
    pub claim_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        space = 8 + Claim::size(claimers.len()),
        payer = operator,
        seeds = [claim_type.get_seeds()],
        bump,
    )]
    pub claim_account: Account<'info, Claim>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = operator
    )]
    pub operator_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl SetClaim<'_> {
    fn validate(&self, token: Pubkey, claimers: &Vec<Pubkey>, locks: &Vec<u64>) -> Result<()> {
        require_keys_neq!(token, Pubkey::default(), LockerErrors::AddressZero);
        require_eq!(claimers.len(), locks.len(), LockerErrors::InputInvalid);

        for (index, claimer) in claimers.iter().enumerate() {
            require_keys_neq!(*claimer, Pubkey::default(), LockerErrors::AddressZero);
            require_gt!(locks[index], 0, LockerErrors::InputInvalid);
        }

        Ok(())
    }

    #[access_control(ctx.accounts.validate(token, &claimers, &locks))]
    pub fn set_claim_handle(
        ctx: Context<Self>,
        claim_type: ClaimType,
        full_lock: u64,
        vesting: u64,
        tge_release: u16,
        token: Pubkey,
        claimers: Vec<Pubkey>,
        locks: Vec<u64>,
    ) -> Result<()> {
        // SET SEED ROUND CLAIM

        let claim_account = &mut ctx.accounts.claim_account;
        claim_account.set_claims(
            claim_type,
            full_lock,
            vesting,
            tge_release,
            token,
            &claimers,
            &locks,
            ctx.bumps.claim_account,
        )?;

        let mut amount: u64 = 0;
        for lock in locks.iter() {
            amount = amount.checked_add(*lock).unwrap();
        }
        //transfer token from operator to claim
        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                SplTransfer {
                    authority: ctx.accounts.operator.to_account_info(),
                    from: ctx.accounts.operator_ata.to_account_info(),
                    to: ctx.accounts.claim_ata.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}
