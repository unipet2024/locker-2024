use anchor_lang::prelude::*;

use crate::{AuthRole, ClaimType};

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub operators: Vec<Pubkey>,
    pub time: i64,
}

#[event]
pub struct SetTgeEvent {
    pub operator: Pubkey,
    pub tge: u64,
    pub time: i64,
}

#[event]
pub struct UserClaimEvent {
    pub claim_type: ClaimType,
    pub claimer: Pubkey,
    pub amount: u64,
    pub time: u64,
}
