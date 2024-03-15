use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum GameStatus {
    Active,
    Ended,
    TimeOver,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalState {
    pub count: u64,
    pub owner: Addr
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub ticket_price: u64,
    pub refund_price: u64,
    pub sold_ticket_count: u64,
    pub total_ticket_count: u64,
    pub total_seigma_amounts: u128,
    pub raffle_status: u8,
    pub nft_contract_addr: Addr,
    pub nft_token_id: String,
    pub owner: Addr,
    pub collection_wallet: Addr, // Collection wallet address to send tokens after the game finished
    pub end_time: u64,
}

pub const GLOBAL_STATE: Item<GlobalState> = Item::new("global_state");
pub const GAME_STATE: Map<u64, GameState> = Map::new("game_state");
pub const TICKET_STATUS: Map<(u64, u64), Addr> = Map::new("ticket_status");
pub const WALLET_TICKETS: Map<(u64, Addr), Vec<u64>> = Map::new("wallet_tickets");
