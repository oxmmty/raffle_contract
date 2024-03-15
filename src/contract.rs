#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coin, to_json_binary, Addr, BankMsg, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, BankQuery, QuerierWrapper, QueryRequest, Response, StdError, StdResult, WasmMsg, WasmQuery};
use cw2::set_contract_version;
use cw721::Cw721ExecuteMsg;
use sha2::{Sha256, Digest};

use cw721::{Cw721QueryMsg, OwnerOfResponse}; 
// use cosmwasm_std::{to_json_binary, Addr, QuerierWrapper, StdResult, WasmQuery, QueryRequest};

use crate::error::ContractError;
use crate::msg::{GlobalResponse, GameResponse, WalletTicketResponse, AllGamesResponse, BalanceResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{GlobalState, GameState, GameStatus, GAME_STATE, GLOBAL_STATE, TICKET_STATUS, WALLET_TICKETS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:raffle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let sender_str = info.sender.clone().to_string();
    let data_to_hash = format!("{}{}", sender_str, "sei1j7ah3st8qjr792qjwtnjmj65rqhpedjqf9dnsddj");
    let mut hasher = Sha256::new();
    hasher.update(data_to_hash.as_bytes());
    let result_hash = hasher.finalize();
    let hex_encoded_hash = hex::encode(result_hash);

    // Compare the generated hash with `msg.authkey`
    if hex_encoded_hash != msg.authkey {
        return Err(ContractError::Unauthorized {});
    }

    let global_state: GlobalState = GlobalState {
        count: 0,
        owner: msg.owner.clone()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    GLOBAL_STATE.save(deps.storage, &global_state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ReceiveNft { sender, token_id, msg } => try_receive_nft(deps, env, info, sender, token_id, msg),
        ExecuteMsg::StartRaffle { ticket_price, total_ticket_count, nft_contract_addr, nft_token_id, collection_wallet, end_time } => 
            try_start_raffle(deps, env, info, ticket_price, total_ticket_count, nft_contract_addr, nft_token_id, collection_wallet, end_time),
        ExecuteMsg::EnterRaffle { game_id } => try_enter_raffle(deps, env, info, game_id),
        ExecuteMsg::TransferTokensToCollectionWallet { amount, denom, collection_wallet_address } => try_transfer_tokens_to_collection_wallet(deps, env, info, amount, denom, collection_wallet_address),
        ExecuteMsg::SelectWinnerAndTransferNFTtoWinner { game_id } => try_select_winner_and_transfer_nft_to_winner(deps, env, info, game_id),
    }
}

// Pseudo-code for CW721 receiver function
pub fn try_receive_nft(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    // Parameters might include the sender address, token ID, and any additional data
    _sender: String,
    token_id: String,
    _msg: Binary,
) -> Result<Response, ContractError> {

    // Logic to handle the received NFT, such as setting it as the prize for the raffle

    // Additional logic as necessary, for example, parsing `msg` for any specific instructions

    Ok(Response::new().add_attribute("action", "receive_nft").add_attribute("token_id", token_id))
}

// Function to get the current status of a game
pub fn get_game_status(raffle_status: u8, end_time: u64, cur_time: u64) -> StdResult<GameStatus> {

    if raffle_status == 0 {
        Ok(GameStatus::Ended)
    } 
    else if cur_time * 1000 >= end_time {
        
        Ok(GameStatus::TimeOver)
    }
    else {
        Ok(GameStatus::Active)
    }
}

fn can_transfer_nft(querier: &QuerierWrapper, nft_contract_addr: Addr, nft_token_id: String, operator: Addr) -> StdResult<bool> {
    // Adjusted query to fetch ownership information
    let owner_response: OwnerOfResponse = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_contract_addr.into_string(),
        msg: to_json_binary(&Cw721QueryMsg::OwnerOf {
            token_id: nft_token_id,
            // Include field for including expired items or not, based on your contract's requirements
            include_expired: None, // This parameter depends on your CW721 version's API
        })?,
    }))?;

    // Check if the contract is the owner or has been approved
    Ok(owner_response.owner == operator || owner_response.approvals.iter().any(|approval| approval.spender == operator))
}

fn try_start_raffle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    ticket_price: u64,
    total_ticket_count: u64,
    nft_contract_addr: Addr,
    nft_token_id: String,
    collection_wallet: Addr,
    end_time: u64
) -> Result<Response, ContractError> {
    let mut global_state = GLOBAL_STATE.load(deps.storage)?;
    // Check
    if info.sender != global_state.owner {
        return Err(ContractError::Unauthorized {  });
    }
    
    if !can_transfer_nft(&deps.querier, nft_contract_addr.clone(), nft_token_id.clone(), env.contract.address)? {
        return Err(ContractError::CantAccessPrize {});
    }
    
    let count_tmp = global_state.count.clone() + 1;
    global_state.count += 1;

    // Assuming 1 represents 'active'
    let game_state: GameState = GameState {
        raffle_status: 1,
        sold_ticket_count: 0,
        ticket_price: ticket_price,
        total_ticket_count: total_ticket_count,
        nft_contract_addr: nft_contract_addr,
        nft_token_id: nft_token_id,
        owner: info.sender.clone(),
        collection_wallet: collection_wallet,
        end_time: end_time,
    };

    GLOBAL_STATE.save(deps.storage, &global_state)?;
    GAME_STATE.save(deps.storage, count_tmp.clone() , &game_state)?;
    
    Ok(Response::new().add_attribute("method", "start_raffle").add_attribute("status", "active").add_attribute("game_id", count_tmp.to_string()))
}

fn try_enter_raffle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    game_id: u64,
) -> Result<Response, ContractError> {

    match GAME_STATE.load(deps.storage, game_id.clone()) {
        Ok(mut game_state) => {
            if game_state.raffle_status.clone() == 0 {
                return Err(ContractError::RaffleEnded {});
            }
            if game_state.end_time <= env.block.time.seconds() * 1000 {
                return Err(ContractError::RaffleTimeOver {  });
            }

            if game_state.sold_ticket_count >= game_state.total_ticket_count {
                return Err(ContractError::RaffleSoldOut {});
            }

            // Simulate ticket purchase by verifying sent funds match the ticket price
            let ticket_price = game_state.ticket_price as u128;
            let sent_funds = info.funds.iter().find(|coin| coin.denom == "usei").map_or(0u128, |coin| coin.amount.u128());
            if sent_funds.clone() < ticket_price.clone() {
                return Err(ContractError::IncorrectFunds {});
            }
            let purchase_ticket_count = sent_funds.clone() / ticket_price.clone();
            let real_purchase_ticket_count = std::cmp::min(purchase_ticket_count, game_state.total_ticket_count.clone() as u128 - game_state.sold_ticket_count.clone() as u128);
            let start_ticket_number = game_state.sold_ticket_count.clone();
            let key = (game_id.clone(), info.sender.clone());

            // Retrieve the current list of tickets for the wallet and game ID, if it exists
            let mut tickets = WALLET_TICKETS.load(deps.storage, key.clone()).unwrap_or_else(|_| Vec::new());
            // Increment the sold_ticket_count and save the participant's address
            for i in 0..real_purchase_ticket_count{
                TICKET_STATUS.save(deps.storage, (game_id.clone(), start_ticket_number.clone() + i as u64) , &info.sender.clone())?;
                tickets.push(start_ticket_number.clone() + 1 + i as u64);
            }
            // Save the updated list back to storage
            WALLET_TICKETS.save(deps.storage, key, &tickets)?;
            game_state.sold_ticket_count += real_purchase_ticket_count.clone() as u64;
            GAME_STATE.save(deps.storage, game_id , &game_state)?;

            let refund_amount = sent_funds.clone() - ticket_price * real_purchase_ticket_count.clone();

            if refund_amount > 0 {
                let send_msg = BankMsg::Send {
                    to_address: info.sender.into_string(),
                    amount: vec![coin(refund_amount, "usei")]
                };
                Ok(Response::new().add_attribute("action", "enter_raffle")
                    .add_attribute("start_ticket_number", (start_ticket_number + 1).to_string())
                    .add_attribute("purchase_ticket_count", real_purchase_ticket_count.to_string())
                    .add_message(send_msg)
                )                
            }
            else{
                Ok(Response::new().add_attribute("action", "enter_raffle")
                    .add_attribute("start_ticket_number", (start_ticket_number + 1).to_string())
                    .add_attribute("purchase_ticket_count", real_purchase_ticket_count.to_string()))
            }
        },
        Err(_) => {
            return Err(ContractError::WrongGameId {});
        }
    }
}

fn try_transfer_tokens_to_collection_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u128, // Amount of tokens to transfer
    denom: String, // Token denomination, e.g., "usei" for micro SEI tokens
    collection_wallet_address: String, // Address of the collection wallet
) -> Result<Response, ContractError> {
    let global_state = GLOBAL_STATE.load(deps.storage)?;
    let collection_wallet = collection_wallet_address.clone();
    // Authorization check: Ensure the caller is the owner
    if info.sender != global_state.owner {
        return Err(ContractError::Unauthorized {  });
    }

    // Create the message to transfer tokens
    let send_msg = BankMsg::Send {
        to_address: collection_wallet_address,
        amount: vec![coin(amount, denom)],
    };

    // Create and return the response that sends the tokens
    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", "transfer_tokens")
        .add_attribute("amount", amount.to_string())
        .add_attribute("to", collection_wallet))
}

fn try_select_winner_and_transfer_nft_to_winner(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    game_id: u64
) -> Result<Response, ContractError> {
    match GAME_STATE.load(deps.storage, game_id.clone()) {
        Ok(mut game_state) => {
            if game_state.raffle_status.clone() == 0 {
                return Err(ContractError::RaffleEnded {});
            }
            if game_state.end_time > env.block.time.seconds() * 1000 {
                return Err(ContractError::CantFinishGame {});
            }

            let mod_number = game_state.total_ticket_count as u64;
            let sold_count = game_state.sold_ticket_count as u64;
            let seed_assist = sold_count % mod_number.clone() * (env.block.time.nanos() / 1024 / mod_number.clone() + env.block.height.clone() % mod_number.clone() * 256 % mod_number.clone() + 1) % mod_number.clone();
            let seed = (env.block.time.nanos() % mod_number + env.block.height + seed_assist) % mod_number;
            let winner_index = seed % mod_number;

            

            // Check if the winner's ticket was actually sold
            match TICKET_STATUS.load(deps.storage, (game_id.clone(), winner_index.clone() as u64)) {
                Ok(winner_ticket) => {

                    let transfer_msg = Cw721ExecuteMsg::TransferNft {
                        recipient: winner_ticket.clone().into_string(),
                        token_id: game_state.nft_token_id.clone(),
                    };
        
                    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: game_state.nft_contract_addr.clone().into_string(),
                        msg: to_json_binary(&transfer_msg)?,
                        funds: vec![],
                    });
        
                    // Update the state before returning the response
                    game_state.raffle_status = 0; // End the raffle by setting the status to 0
                    GAME_STATE.save(deps.storage, game_id.clone(), &game_state)?;
        
                    // Return a response with the winner information and the transfer message
                    Ok(Response::new()
                        .add_message(msg)
                        .add_attribute("action", "select_winner_and_transfer_nft")
                        .add_attribute("game_id", game_id.to_string())
                        .add_attribute("winner_ticket", (winner_index + 1).to_string())
                        .add_attribute("winner", winner_ticket.into_string())
                        .add_attribute("nft_contract_addr", game_state.nft_contract_addr.into_string())
                        .add_attribute("token_id", game_state.nft_token_id))                    
                },
                Err(_) => {
                    // If the ticket wasn't sold, simply end the raffle with transferring the NFT to collection wallet.
                    let transfer_msg = Cw721ExecuteMsg::TransferNft {
                        recipient: game_state.collection_wallet.clone().into_string(),
                        token_id: game_state.nft_token_id.clone(),
                    };
        
                    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: game_state.nft_contract_addr.clone().into_string(),
                        msg: to_json_binary(&transfer_msg)?,
                        funds: vec![],
                    });

                    game_state.raffle_status = 0; // End the raffle
                    GAME_STATE.save(deps.storage, game_id.clone(), &game_state)?;

                    Ok(Response::new()
                        .add_message(msg)
                        .add_attribute("action", "select_winner")
                        .add_attribute("game_id", game_id.to_string())
                        .add_attribute("winner_ticket", (winner_index + 1).to_string())
                        .add_attribute("status", "Winner ticket was not sold"))
                }
            }
        },
        Err(_) => {
            return Err(ContractError::WrongGameId {});
        }
    }
    
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetGlobalInfo {} => to_json_binary(&query_global_info(deps)?),
        QueryMsg::GetGameInfo { game_id } => to_json_binary(&query_game_info(deps, game_id)?),
        QueryMsg::GetTicketsForWallet { game_id, wallet_addr } => to_json_binary(&query_tickets_for_wallet(deps, game_id, wallet_addr)?),
        QueryMsg::GetAllGames {} => to_json_binary(&query_all_games(deps)?),
        QueryMsg::GetBalance {} => to_json_binary(&query_sei_balance(deps, env)?),
    }
}

fn query_global_info(deps: Deps) -> StdResult<GlobalResponse> {
    let global_state = GLOBAL_STATE.load(deps.storage)?;

    Ok(GlobalResponse { 
        raffle_count: global_state.count,
        owner: global_state.owner
    })
}

fn query_game_info(deps: Deps, game_id: u64) -> StdResult<GameResponse> {
    let game_state = GAME_STATE.load(deps.storage, game_id)
        .map_err(|_| StdError::generic_err("Game with provided ID does not exist"))?;

    Ok(GameResponse { 
        ticket_price: game_state.ticket_price,
        sold_ticket_count: game_state.sold_ticket_count,
        total_ticket_count: game_state.total_ticket_count,
        raffle_status: game_state.raffle_status,
        nft_contract_addr: game_state.nft_contract_addr,
        nft_token_id: game_state.nft_token_id,
        owner: game_state.owner,
        collection_wallet: game_state.collection_wallet,
        end_time: game_state.end_time,
    })
}

fn query_tickets_for_wallet(
    deps: Deps,
    game_id: u64,
    wallet_addr: Addr,
) -> StdResult<WalletTicketResponse> {
    let key = (game_id, wallet_addr);

    // Directly retrieve the list of ticket numbers for the wallet and game ID
    let tickets = WALLET_TICKETS.load(deps.storage, key).unwrap_or_else(|_| Vec::new());

    Ok(WalletTicketResponse{
        tickets: tickets
    })
}

pub fn query_all_games(deps: Deps) -> StdResult<AllGamesResponse> {
    let all_games = GAME_STATE.range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|item| item.ok()) // Filter out any errors
        .map(|(_, game_state)| game_state) // We're interested in the GameState value
        .collect::<Vec<GameState>>();

    Ok(AllGamesResponse {
        games: all_games,
    })
}

pub fn query_sei_balance(deps: Deps, env: Env) -> StdResult<BalanceResponse> {
    let sei_denom = "usei";

    let query = QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: sei_denom.to_string(),
    });

    // Execute the query
    let res: cosmwasm_std::BalanceResponse = deps.querier.query(&query)?;

    // Find the SEI token in the response
    let sei_balance = res.amount; // Assuming `amount` is the field you're interested in.

    // Construct your BalanceResponse, assuming it expects a Coin
    Ok(BalanceResponse { balance: sei_balance })
}