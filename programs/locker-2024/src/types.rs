use anchor_lang::prelude::*;

use crate::constants::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum ClaimType {
    SeedRound,
    PrivateRound,
    PublisSale,
    FoundingTeam,
    Advisors,
    // Liquidity,
    Treasury,
    Ecosystem,
}
impl ClaimType {
    pub fn get_seeds(&self) -> &[u8] {
        match *self {
            ClaimType::SeedRound => SEED_ROUND_ACCOUNT,
            ClaimType::PrivateRound => PRIVATE_ROUND_ACCOUNT,
            ClaimType::PublisSale => PUBLIC_SALE_ACCOUNT,
            ClaimType::FoundingTeam => FOUNDING_TEAM_ACCOUNT,
            ClaimType::Advisors => ADVISORS_ACCOUNT,
            ClaimType::Treasury => TREASURY_ACCOUNT,
            ClaimType::Ecosystem => ECOSYSTEM_ACCOUNT,
        }
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub struct ClaimData {
    pub claimer: Pubkey, //32
    pub lock: u64,       //8
    pub released: u64,   //8
}
