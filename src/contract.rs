#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, CosmosMsg, WasmMsg, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, BankMsg, coin};
use cw2::set_contract_version;
use cw721::Cw721ExecuteMsg;

use crate::error::ContractError;
use crate::msg::{RaffleResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, TICKET_STATUS};

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
    let sender = info.sender.clone();
    let state = State {
        ticket_price: 0,
        sold_ticket_count: 0,
        total_ticket_count: 0,
        raffle_status: 0, // Assuming 0 represents 'not started'
        owner: info.sender,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", sender.to_string())
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StartRaffle {} => try_start_raffle(deps, env, info),
        ExecuteMsg::EnterRaffle {} => try_enter_raffle(deps, env, info),
        ExecuteMsg::TransferTokensToCollectionWallet { amount, denom, collection_wallet_address } => try_transfer_tokens_to_collection_wallet(deps, env, info, amount, denom, collection_wallet_address),
        ExecuteMsg::SelectWinner {} => try_select_winner(deps, env, info),
        ExecuteMsg::TransferNFTtoWinner { winner_addr, nft_contract_addr, token_id } => try_transfer_nft_to_winner(deps, env, info, winner_addr, nft_contract_addr, token_id),
    }
}

fn try_start_raffle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    // Check
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }

    if state.raffle_status != 0 {
        return Err(ContractError::RaffleStarted {  });
    }
    
    // Assuming 1 represents 'active'
    state.raffle_status = 1;
    state.sold_ticket_count = 0; // Reset sold ticket count if necessary
    
    STATE.save(deps.storage, &state)?;
    
    Ok(Response::new().add_attribute("method", "start_raffle").add_attribute("status", "active"))
}
fn try_enter_raffle(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    // Define the exact amount expected per entry in the smallest unit, e.g., 1 SEI = 1,000,000 uSEI

    // Sum up the amount of SEI sent with the transaction
    let mut sent_amount = 0u128;
    let state = STATE.load(deps.storage)?;
    let expected_amount_u128 = state.ticket_price as u128;
    let participant_address = info.sender;
    for coin in info.funds.iter() {
        if coin.denom == "usei" { // Adjust the denomination based on how SEI is represented
            sent_amount += coin.amount.u128();
        }
    }

    // Check if the sent amount is exactly what's expected
    if sent_amount < expected_amount_u128 {
        return Err(ContractError::PayError { ticket_price: state.ticket_price });
    }

    // Add the sender to the list of participants
    TICKET_STATUS.save(deps.storage, state.sold_ticket_count, &participant_address)?;

    Ok(Response::new().add_attribute("action", "enter_raffle"))
}

fn try_transfer_tokens_to_collection_wallet(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: u128, // Amount of tokens to transfer
    denom: String, // Token denomination, e.g., "usei" for micro SEI tokens
    collection_wallet_address: String, // Address of the collection wallet
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let collection_wallet = collection_wallet_address.clone();
    // Authorization check: Ensure the caller is the owner
    if info.sender != state.owner {
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

fn try_select_winner(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // Load the owner address and ensure only the owner can select a winner
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }

    // Load the number of participants
    let state: State = STATE.load(deps.storage)?;
    if state.sold_ticket_count == 0 {
        return Err(ContractError::NoParticipants {  });
    }

    // Use the current block timestamp and height as a seed for randomness
    let seed = env.block.time.nanos() + env.block.height;
    let mod_number = state.total_ticket_count as u64;
    let winner_index = seed % mod_number;

    // Retrieve the winner's address
    let winner_address = TICKET_STATUS.load(deps.storage, winner_index as u32);

    let winner_address_string = match winner_address {
        Ok(addr) => addr.to_string(), // If Ok, convert the Addr to String
        Err(_) => "Error retrieving address".to_string(), // Handle the error case
    };
    Ok(Response::new()
        .add_attribute("action", "select_winner")
        .add_attribute("winner", winner_address_string))
}

fn try_transfer_nft_to_winner(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    winner_addr: String, // Address of the winner
    nft_contract_addr: String, // cw721 NFT contract address
    token_id: String, // ID of the NFT to transfer
) -> Result<Response, ContractError> {
    let sender = info.sender;
    let state = STATE.load(deps.storage)?;
    // Additional Chekc if the winner is nothing.

    // Optional: Check if the sender is authorized (e.g., the contract owner or the raffle contract itself)
    if sender != state.owner {
        return Err(ContractError::Unauthorized {  });
    }
    // Construct the cw721 transfer message
    let transfer_msg = Cw721ExecuteMsg::TransferNft {
        recipient: winner_addr.clone(),
        token_id: token_id.clone(),
    };

    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: nft_contract_addr,
        msg: to_binary(&transfer_msg)?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "transfer_nft")
        .add_attribute("recipient", winner_addr)
        .add_attribute("token_id", token_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRaffle {} => to_binary(&query_raffle(deps)?),
    }
}

fn query_raffle(deps: Deps) -> StdResult<RaffleResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(RaffleResponse { 
        ticket_price: state.ticket_price,
        sold_ticket_count: state.sold_ticket_count,
        total_ticket_count: state.total_ticket_count,
        raffle_status: state.raffle_status,
        owner: state.owner
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetRaffle {}).unwrap();
        let value: RaffleResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.raffle_status);
    }

}
