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
}
