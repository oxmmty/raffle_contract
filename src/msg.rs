use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    StartRaffle {},
    EnterRaffle {},
    TransferTokensToCollectionWallet {
        amount: u128,
        denom: String,
        collection_wallet_address: String,
    },
    SelectWinner {},
    TransferNFTtoWinner {
        winner_addr: String,
        nft_contract_addr: String,
        token_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetRaffle {}
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RaffleResponse {
    pub ticket_price: u32,
    pub sold_ticket_count: u32,
    pub total_ticket_count: u32,
    pub raffle_status: i32,
    pub owner: Addr,
}
