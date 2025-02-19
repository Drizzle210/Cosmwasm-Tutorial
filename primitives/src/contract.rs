#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse};
use crate::state::{State, STATE};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:primitives";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone()
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_json_binary(&query::owner(deps)?),
        QueryMsg::Integer {} => to_json_binary(&query::integers(deps)?)
    }
}

pub mod query {
    use cosmwasm_std::{Uint128, Uint64};

    use super::*;

    use crate::msg::{GetIntegerResponse, GetOwnerResponse};

    pub fn owner(deps: Deps) -> StdResult<GetOwnerResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetOwnerResponse {
            owner: state.owner
        })
    }

    pub fn integers(deps: Deps) -> StdResult<GetIntegerResponse> {
        let _ex = Uint64::from(2u64);
        let _def = Uint128::default();
        let max_u128 = Uint128::from(340_282_366_920_938_463_463_374_607_431_768_211_455u128);
        let max_u128_primitive = Uint128::MAX;
        assert_eq!(max_u128, max_u128_primitive);

        Ok(GetIntegerResponse {
            works: true
        })
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor, IntoAddr};
 
    use super::*;
    #[test]
    fn test_owner() {
        let mut app = App::default();
 
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        let owner = "test_address".into_addr();
 
        let addr = app
            .instantiate_contract(
                code_id,
                owner.clone(),
                &InstantiateMsg {},
                &[],
                "Contract",
                None,
            )
            .unwrap();

        let resp: GetOwnerResponse = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::GetOwner {})
            .unwrap();

        assert_eq!(resp, GetOwnerResponse {
            owner: owner.clone()
        })
    
    }
}
