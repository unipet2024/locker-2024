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

declare_id!("8oUvgoQeCqB2ADoEBACvGtBz3w2jvVbzHXFSw72GrjEq");

#[program]
pub mod locker_2024 {
    use super::*;

    pub fn init(ctx: Context<InitLocker>, unp: Pubkey) -> Result<()> {
        init_locker::init_handle(ctx, unp)
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
        set_claim::set_claim_handle(
            ctx,
            seed_round_pubkeys,
            seed_round_locks,
            private_round_pubkeys,
            private_round_locks,
            public_sale_pubkeys,
            public_sale_locks,
            founding_team_pubkeys,
            founding_team_locks,
            advisors_pubkeys,
            advisors_locks,
            treasury_pubkeys,
            treasury_locks,
            ecosystem_pubkeys,
            ecosystem_locks,
        )
    }

    pub fn set_tge(ctx: Context<SetTge>, tge: u64) -> Result<()> {
        set_tge::set_tge_handle(ctx, tge)
    }

    pub fn user_claim(ctx: Context<UserClaim>, claim_type: ClaimType) -> Result<()> {
        user_claim::user_claim_handle(ctx, claim_type)
    }
}
