#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::HumanAddr;
use cosmwasm_std::{attr, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, InitResponse, HandleResponse, MigrateResponse, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CountResponse, OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:my-first-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<InitResponse, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(InitResponse {
        attributes: vec![
            attr("method", "instantiate"),
            attr("owner", info.sender),
            attr("count", msg.count.to_string()),
        ],
        messages: vec![],
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::ChangeOwner{new_owner} => try_change_owner(deps, info, new_owner),
    }
}

pub fn try_change_owner(deps: DepsMut, info : MessageInfo, new_owner : HumanAddr) -> Result<HandleResponse,ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_,ContractError>{
        if info.sender != state.owner{
            return Err(ContractError::Unauthorized{});
        }
        state.owner = new_owner;
        Ok(state)
    })?;
    Ok(HandleResponse{
        attributes: vec![
            attr("method", "try_change_owner"),
        ],
        messages: vec![],
        data: None,
    })
}

pub fn try_increment(deps: DepsMut) -> Result<HandleResponse, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(HandleResponse{
        attributes: vec![
            attr("method", "try_increment"),
        ],
        messages: vec![],
        data: None,
    })

}

pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<HandleResponse, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;

    Ok(HandleResponse{
        attributes: vec![
            attr("method", "reset"),
        ],
        messages: vec![],
        data: None,
    })
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse {owner: state.owner})
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

