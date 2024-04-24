use anchor_lang::prelude::*;

// total 65
#[account]
pub struct Locker {
    pub admin: Pubkey,    //32
    pub operator: Pubkey, //32
    pub unp: Pubkey,      // 32
    pub bump: u8,         //1
}

impl Locker {
    pub fn init(&mut self, admin: Pubkey, operator: Pubkey, unp: Pubkey, bump: u8) -> Result<()> {
        self.admin = admin;
        self.operator = operator;
        self.unp = unp;
        self.bump = bump;

        Ok(())
    }
}
