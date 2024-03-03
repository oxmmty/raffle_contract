use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub ticket_price: u32,
    pub sold_ticket_count: u32,
    pub total_ticket_count: u32,
    pub raffle_status: i32,
    pub nft_contract_addr: Option<Addr>,
    pub nft_token_id: String,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EmptyExtension {}

pub const STATE: Item<State> = Item::new("state");
pub const TICKET_STATUS: Map<u32, Addr> = Map::new("ticket_status");