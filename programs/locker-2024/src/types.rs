use anchor_lang::prelude::*;

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
    Ecosystem
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub struct ClaimData {
    pub claimer: Pubkey, //32
    pub lock: u64,       //8
    pub released: u64,   //8
}
