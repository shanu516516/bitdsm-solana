// programs/bitdsm/src/errors.rs

#[error_code]
pub enum BitDSMError {
    #[msg("Invalid Bitcoin public key length")]
    InvalidBtcPublicKeyLength,
    #[msg("Operator not registered")]
    OperatorNotRegistered,
    #[msg("Insufficient stake weight")]
    InsufficientStakeWeight,
}
