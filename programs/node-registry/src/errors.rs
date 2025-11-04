use anchor_lang::prelude::*;

#[error_code]
pub enum NodeRegistryError {
    #[msg("Node ID is too long (max 64 characters)")]
    NodeIdTooLong,

    #[msg("GPU specs hash is invalid")]
    InvalidGpuSpecsHash,

    #[msg("Location string is too long (max 64 characters)")]
    LocationTooLong,

    #[msg("Node is not online")]
    NodeNotOnline,

    #[msg("Node is already registered")]
    NodeAlreadyRegistered,

    #[msg("Insufficient stake amount")]
    InsufficientStake,

    #[msg("Unauthorized: not node owner")]
    UnauthorizedNotOwner,

    #[msg("Node is suspended and cannot perform this action")]
    NodeSuspended,

    #[msg("Heartbeat interval too short (minimum 30 seconds)")]
    HeartbeatTooFrequent,
}
