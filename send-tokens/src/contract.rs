#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:send-tokens";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new()
        .add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendTokens { amount, denom, to } => {
            execute::send_tokens(deps, amount, denom, to)
        }
    }
}

pub mod execute {
    use cosmwasm_std::{Addr, BankMsg, Uint128, Coin};
    use super::*;

    pub fn send_tokens(_deps: DepsMut, amount: Uint128, denom: String, to: Addr) -> Result<Response, ContractError> {
        Ok(Response::new().add_attribute("action", "increment")
        .add_message(BankMsg::Send {
            to_address: to.into_string(),
            amount: vec![Coin {
                denom,
                amount,
            }],
        }))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, coins, Uint128};
    use cw_multi_test::{App, ContractWrapper, Executor, IntoAddr};
    use super::*;

    #[test]
    pub fn test_send_tokens() {
        let owner = "owner".into_addr();
        let user = "user".into_addr();
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &owner, coins(12, "eth"))
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(code_id, owner.clone(), &InstantiateMsg {}, &[], "Contract", None)
            .unwrap();
        
        app.execute_contract(
            owner.clone(),
            addr.clone(),
            &ExecuteMsg::SendTokens {
                amount: Uint128::new(7),
                denom: "eth".to_string(),
                to: user.clone(),
            },
            &coins(7, "eth")
        ).unwrap();

        assert_eq!(
            app.wrap()
                .query_balance(owner.as_str(), "eth")
                .unwrap()
                .amount
                .u128(),
            5
        );

        assert_eq!(
            app.wrap()
                .query_balance(user.as_str(), "eth")
                .unwrap()
                .amount
                .u128(),
            7
        );
    }
}
