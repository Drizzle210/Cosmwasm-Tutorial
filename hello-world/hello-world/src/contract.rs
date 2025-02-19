#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, HelloWorldResponse};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hello-world";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
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
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::HelloWorld {} => to_json_binary(&query_hello_world()?),
    }
}

pub fn query_hello_world() -> StdResult<HelloWorldResponse> {
    let hello_message = HelloWorldResponse  {
        hello_world_message: "Hello, world!".to_string(),
    };
    Ok(hello_message)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor, IntoAddr};

    use super::*;
    #[test]
    fn basic_hello() {
        let mut app = App::default();
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let owner = "owner".into_addr();

        let addr = app
            .instantiate_contract(
                code_id,
                owner.clone(),
                &InstantiateMsg {},
                &vec![],
                "Contract",
                None
            ).unwrap();

        let resp: HelloWorldResponse = app
            .wrap()
            .query_wasm_smart(&addr, &QueryMsg::HelloWorld {  })
            .unwrap();
        assert_eq!(
            resp, 
            HelloWorldResponse {
                hello_world_message: "Hello, world!".to_string(),
            }
        )
    }
}
