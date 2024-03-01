use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    #[error("Raffle already started or completed")]
    RaffleStarted {},

    #[error("Must send exactly {ticket_price} SEI to enter the raffle")]
    PayError { ticket_price: u32 },

    #[error("No participants in the raffle")]
    NoParticipants {},
    
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
