use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cosmwasm_std::Binary;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveNft {
        sender: String,
        token_id: String,
        msg: Binary,
    },
    StartRaffle {
        ticket_price: u32,
        total_ticket_count: u32,
        expected_participants_count: u32,
        nft_contract_addr: Addr,
        nft_token_id: String,
    },
    EnterRaffle {},
    TransferTokensToCollectionWallet {
        amount: u128,
        denom: String,
        collection_wallet_address: String,
    },
    SelectWinnerAndTransferNFTtoWinner { },
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
    pub expected_participants_count: u32,
    pub raffle_status: i32,
    pub nft_contract_addr: Option<Addr>,
    pub nft_token_id: String,
    pub owner: Addr,
}
