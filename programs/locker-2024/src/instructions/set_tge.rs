use anchor_lang::prelude::*;

use crate::{
    AuthRole, AuthorityRole, Claim, ClaimType, Locker, LockerErrors, SetTgeEvent, LOCKER_ACCOUNT,
    OPERATOR_ROLE,
};

#[derive(Accounts)]
#[instruction(claim_type:ClaimType,tge: u64)]
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
        seeds = [claim_type.get_seeds()],
        bump,
    )]
    pub claim_account: Account<'info, Claim>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl SetTge<'_> {
    fn validate(&self, tge: u64) -> Result<()> {
        require_eq!(
            self.claim_account.start_time,
            0,
            LockerErrors::TgeSetupAlready
        );
        require_gt!(tge, 0, LockerErrors::InputInvalid);

        Ok(())
    }

    #[access_control(ctx.accounts.validate(tge))]
    pub fn set_tge_handle(ctx: Context<Self>, claim_type: ClaimType, tge: u64) -> Result<()> {
        let claim_account = &mut ctx.accounts.claim_account;

        claim_account.set_tge(tge)?;
        emit!(SetTgeEvent {
            claim_type,
            operator: ctx.accounts.operator.key(),
            tge,
            time: Clock::get().unwrap().unix_timestamp
        });

        Ok(())
    }
}
