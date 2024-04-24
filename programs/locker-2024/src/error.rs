use anchor_lang::prelude::*;

#[error_code]
pub enum LockerErrors {
    #[msg("Nothing to claim")]
    NothingToClaim,

    #[msg("Not allow to claim")]
    NotAllowToClaim,

    #[msg("Still lock")]
    StillLock,

    #[msg("Address 0")]
    AddressZero,

    #[msg("Input invalid")]
    InputInvalid,

    #[msg("Locker closed")]
    LockerClosed,

    #[msg("Admin account invalid")]
    AdminAccountInvalid,

    #[msg("Operator account invalid")]
    OperatorAccountInvalid,

    #[msg("Only admin")]
    OnlyAdmin,

    #[msg("Only Operator")]
    OnlyOperator,

    #[msg("Operator not change")]
    OperatorNotChange,
}

impl From<LockerErrors> for ProgramError {
    fn from(e: LockerErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}
