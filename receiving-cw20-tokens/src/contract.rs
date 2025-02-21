#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, QueryResponse, StdError};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::*;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:receiving-cw20-tokens";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

// store admin and whitelist token (symbol & token contract address) to blockchain
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    let admin = info.sender.to_string();
    let cw20_whitelist: Vec<(String, Addr)> = vec![(
        msg.token_symbol,
        msg.token_contract_address,
    )];

    CONFIG.save(
        deps.storage,
        &Config {
            admin: deps.api.addr_validate(&admin).unwrap(),
            cw20_wl: cw20_whitelist,
        }
    )?;

    Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("admin", admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(receive_msg) => execute::execute_receive(deps, env, info, receive_msg),
    }
}

pub mod execute {
    use cosmwasm_std::from_json;
    use super::*;
    use cw20::{Cw20ReceiveMsg, Cw20CoinVerified, Balance};

    use crate::msg::ReceiveMsg;

    pub fn execute_receive(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        wrapper: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        
        let msg: ReceiveMsg = from_json(&wrapper.msg).unwrap(); // msg included in Send {contract, amount, **msg**} execute on the cw20 contract
        let user_wallet = deps.api.addr_validate(&wrapper.sender).unwrap();

        let balance = Balance::Cw20(Cw20CoinVerified {
            address: info.sender.clone(), // cw20 contract this message was sent from
            amount: wrapper.amount,
        });

        let config = CONFIG.load(deps.storage)?;
        // @todo: check if the token is in the whitelist

        match msg {
            ReceiveMsg::AnExecuteMsg {} => {
                execute_do_somthing(deps, &user_wallet, &info.sender, balance)
            }
        }
    }

    pub fn execute_do_somthing(
        _deps: DepsMut,
        _user_wallet: &Addr,
        _cw20_contract_addr: &Addr,
        _balance: Balance,
    ) -> Result<Response, ContractError> {
        Ok(Response::default())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetAdmin {} => query::get_admin(deps),
    }
}

pub mod query {
    use super::*;
    use crate::msg::AdminResponse;

    pub fn get_admin(deps: Deps) -> Result<QueryResponse, StdError> {
        let config = CONFIG.load(deps.storage)?;

        let admin = config.admin.to_string();
        to_json_binary(&AdminResponse { admin })
    }
}

#[cfg(test)]
mod tests {
    
}
