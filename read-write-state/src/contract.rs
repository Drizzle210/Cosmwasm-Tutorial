#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE, NAMES};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:read-write-state";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    STATE.save(deps.storage, &state)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Write{} => execute::write(deps)
    }
}

pub mod execute {
    use super::*;
    pub fn write(deps: DepsMut) -> Result<Response, ContractError> {
        let mut state = STATE.load(deps.storage)?;
        state.count = 5;
        STATE.save(deps.storage, &state)?;
        NAMES.save(deps.storage, "FAMILY NAME KEY".to_string(), &"FAMILY NAME VALUE".to_string())?;
        Ok(Response::new().add_attribute("action", "write"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_json_binary(&query::count(deps)?),
        QueryMsg::GetFamilyName { first_name } => to_json_binary(&query::name(deps, first_name)?),
    }
}

pub mod query {
    use crate::msg::{GetCountResponse, GetNameResponse};

    use super::*;
    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }

    pub fn name(deps: Deps, first_name: String) -> StdResult<GetNameResponse> {
        let family_name = NAMES.load(deps.storage, first_name)?;
        Ok(GetNameResponse { family_name })
    }
}

#[cfg(test)]
mod tests {}
