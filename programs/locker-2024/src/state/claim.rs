use anchor_lang::prelude::*;

use crate::{ClaimData, ClaimType, LockerErrors};

// total 97
#[account]
pub struct Claim {
    pub claim_type: ClaimType,      //1
    pub full_lock: u64,       //8
    pub vesting: u64,         //8
    pub tge_release: u16,           //2 2 decimals
    pub start_time: u64,            //8
    pub end_time: u64,              //8
    pub token: Pubkey,              //32
    pub claim_data: Vec<ClaimData>, // 4 + 48*n
    pub bump: u8,                   //1
}

impl Claim {
    pub fn size(claim_size: usize) -> usize {
        return 8 + 
        1 + // claim_type
        8 + // full_lock
        8 + // vesting
        2 + // tge_release
        8 + // start_time
        8 + // end_time
        32 + //token
        4 +  claim_size * 48 +// claim_data
        1; //bump
    }
    pub fn set_claims(
        &mut self,
        claim_type: ClaimType,
        full_lock: u64,
        vesting: u64,
        tge_release: u16,
        token: Pubkey,
        claimers: &Vec<Pubkey>,
        locks: &Vec<u64>,
        bump: u8,
    ) -> Result<()> {
        require_eq!(claimers.len(), locks.len(), LockerErrors::InputInvalid);

        self.claim_type = claim_type;
        self.full_lock = full_lock;
        self.vesting = vesting;
        self.token = token;
        self.tge_release = tge_release;
        self.bump = bump;

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
        self.start_time = tge + self.full_lock;
        self.end_time = self.start_time + self.vesting as u64;

        // set tge amount
        // for claim_data in self.claim_data.iter_mut() {
        //     // claim_data.released = claim_data.lock * self.tge_release / 10000; // TGE RELEASE 2 DECIMALS
        //     claim_data.lock -= claim_data.lock * (self.tge_release as u64) / 10000;
        // }

        Ok(())
    }

    pub fn get_claim_info(&self, claimer: Pubkey, timer: u64) -> (usize, u64) {
        if claimer == Pubkey::default() {
            return (0,0)
        }

        for (index, claim_data) in self.claim_data.iter().enumerate() {
            if claim_data.claimer == claimer  {
                return (index, self.get_claim_amount(index, timer));
            }
        }
        (0, 0)
    }

    pub fn released_times(&self, timer: u64) -> u64 {
        let mut target_now = self.end_time;
        if timer < self.end_time {
            target_now = timer;
        }

        let released_time = target_now - self.start_time;
        released_time
    }

    pub fn get_claim_amount_tge(&self, index: usize) -> u64{
        return self.claim_data[index].lock * (self.tge_release as u64) /10000 ;
    }

    pub fn get_claim_amount(&self, index: usize, timer: u64) -> u64 {
        let claim_data = self.claim_data[index];

        if timer < self.start_time {
            return 0;
        } else if timer > self.end_time {
            return claim_data.lock - claim_data.released;
        } else {
            let amount_tge = self.get_claim_amount_tge(index);

            let total_vesting_time = self.end_time - self.start_time;
            let released_time = self.released_times(timer);
            msg!("released_time: {:}", released_time);
            msg!("amount_tge: {:}", amount_tge);
            msg!("released: {:}", claim_data.released);


            return (((claim_data.lock - amount_tge) * released_time) / total_vesting_time) 
                + amount_tge - claim_data.released;
        }
    }
}
