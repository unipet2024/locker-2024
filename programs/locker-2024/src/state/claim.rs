use anchor_lang::prelude::*;

use crate::{ClaimData, ClaimType, LockerErrors};

// total 65
#[account]
pub struct Claim {
    pub claim_type: ClaimType,      //1
    pub full_lock_month: i8,        //1
    pub vesting_month: i8,          //1
    pub tge_release: u64,           //8
    pub start_time: u64,            //8
    pub end_time: u64,              //8
    pub claim_data: Vec<ClaimData>, // 4 + 48*n
    pub bump: u8,                   //1
}

impl Claim {
    pub fn init(
        &mut self,
        claim_type: ClaimType,
        full_lock_month: i8,
        vesting_month: i8,
        tge_release: u64,
        bump: u8,
    ) -> Result<()> {
        self.claim_type = claim_type;
        self.full_lock_month = full_lock_month;
        self.vesting_month = vesting_month;
        self.tge_release = tge_release;
        self.bump = bump;

        Ok(())
    }

    pub fn set_claims(&mut self, claimers: Vec<Pubkey>, locks: Vec<u64>) -> Result<()> {
        require_eq!(claimers.len(), locks.len(), LockerErrors::InputInvalid);
        self.claim_data = vec![];

        for i in 0..claimers.len() {
            require_keys_neq!(claimers[i], Pubkey::default(), LockerErrors::AddressZero);
            self.claim_data.push(ClaimData {
                claimer: claimers[i],
                lock: locks[i],
                released: 0,
            });
        }
        Ok(())
    }

    pub fn set_tge(&mut self, tge: u64) -> Result<()> {
        self.start_time = tge;
        self.end_time = tge + self.vesting_month as u64;

        // set tge amount
        for claim_data in self.claim_data.iter_mut() {
            // claim_data.released = claim_data.lock * self.tge_release / 10000; // TGE RELEASE 2 DECIMALS
            claim_data.lock -= claim_data.lock * self.tge_release / 10000;
        }

        Ok(())
    }

    pub fn get_claim_index(&self, claimer: Pubkey) -> (usize, bool) {
        for (index, claim_data) in self.claim_data.iter().enumerate() {
            if claim_data.claimer == claimer {
                return (index, true);
            }
        }
        (0, false)
    }

    pub fn released_times(&self, current: u64) -> u64 {
        let mut target_now = self.end_time;
        if current < self.end_time {
            target_now = current;
        }

        let released_time = target_now - self.start_time;
        released_time
    }

    pub fn get_claim_amount(&self, claimer: Pubkey, current: u64) -> u64 {
        let (index, check) = self.get_claim_index(claimer);

        if check == false {
            return 0;
        }

        let claim_data = self.claim_data[index];

        if current < self.start_time {
            return 0;
        } else if current > self.end_time {
            return claim_data.lock - claim_data.released;
        } else {
            let total_vesting_time = self.end_time - self.start_time;
            let released_time = self.released_times(current);

            return ((claim_data.lock * released_time) / total_vesting_time) - claim_data.released;
        }
    }
}
