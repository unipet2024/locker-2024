use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use types::*;

declare_id!("Ay5kbaQ1YojYTvoeKExWvCeyA77fNKtVuxSqbbctEUkq");

#[program]
pub mod locker_2024 {
    use super::*;

    pub fn init(ctx: Context<InitLocker>) -> Result<()> {
        InitLocker::init_handle(ctx)
    }

    pub fn set_authority(
        ctx: Context<AdminInstruction>,
        role: AuthRole,
        operators: Vec<Pubkey>,
    ) -> Result<()> {
        admin_instruction::set_authority_handler(ctx, role, operators)
    }

    pub fn set_claim(
        ctx: Context<SetClaim>,
        claim_type: ClaimType,
        full_lock: u64,
        vesting: u64,
        tge_release: u16,
        token: Pubkey,
        claimers: Vec<Pubkey>,
        locks: Vec<u64>,
    ) -> Result<()> {
        SetClaim::set_claim_handle(
            ctx,
            claim_type,
            full_lock,
            vesting,
            tge_release,
            token,
            claimers,
            locks,
        )
    }

    pub fn set_tge(ctx: Context<SetTge>, claim_type: ClaimType, tge: u64) -> Result<()> {
        SetTge::set_tge_handle(ctx, claim_type, tge)
    }

    pub fn user_claim(ctx: Context<UserClaim>, claim_type: ClaimType) -> Result<()> {
        UserClaim::user_claim_handle(ctx, claim_type)
    }
}
