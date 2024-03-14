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

    #[error("Wrong Game Id")]
    WrongGameId {},

    #[error("Raffle already ended")]
    RaffleEnded {},

    #[error("Raffle Time Over")]
    RaffleTimeOver {},

    #[error("Raffle not started yet")]
    RaffleNotActive {},
    
    #[error("All raffle tickets was sold.")]
    RaffleSoldOut {},

    #[error("It is not the end time of the game")]
    CantFinishGame {},

    #[error("Can not access prize NFT")]
    CantAccessPrize {},

    #[error("Incorrect Funds")]
    IncorrectFunds {},

    #[error("Must send exactly {ticket_price} SEI to enter the raffle")]
    PayError { ticket_price: u32 },

    #[error("No participants in the raffle")]
    NoParticipants {},
    
    #[error("Missing NFT contract address")]
    MissingNftContractAddr {}, 

    #[error("Can not transfer tokens until raffle is finished")]
    CantTransferTokens {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
