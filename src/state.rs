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
    pub count: u32,
    pub owner: Addr
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GameState {
    pub ticket_price: u32,
    pub sold_ticket_count: u32,
    pub total_ticket_count: u32,
    pub raffle_status: u8,
    pub nft_contract_addr: Option<Addr>,
    pub nft_token_id: String,
    pub owner: Addr,
    pub collection_wallet: Addr, // Collection wallet address to send tokens after the game finished
    pub end_time: u64,
}

pub const GLOBALSTATE: Item<GlobalState> = Item::new("global_state");
pub const GAMESTATE: Map<u32, GameState> = Map::new("game_state");
pub const TICKET_STATUS: Map<(u32, u32), Addr> = Map::new("ticket_status");