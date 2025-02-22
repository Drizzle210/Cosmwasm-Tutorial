#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, StdError, QueryRequest, Addr};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::*;

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:token-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let total_supply=Uint128::zero();
    let token_info = TokenInfo {
        token_denom: msg.token_symbol,
        token_address: msg.token_contract_address,
    };

    TOTAL_SUPPLY.save(deps.storage, &total_supply)?;
    TOKEN_INFO.save(deps.storage, &token_info)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("total_supply", total_supply))

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {amount} => execute::execute_deposit(deps, env, info, amount),
        ExecuteMsg::Withdraw {shares} => execute::execute_withdraw(deps, env, info, shares),
    }
}

pub mod execute {
    use super::*;
    use cosmwasm_std::{CosmosMsg, WasmMsg, WasmQuery};
    
    pub fn execute_deposit(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let token_info = TOKEN_INFO.load(deps.storage)?;
        let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;
        let mut shares = Uint128::zero();
        let mut balance= BALANCE_OF.load(deps.storage, info.sender.clone()).unwrap_or(Uint128::zero());
        let balance_of = get_token_balance_of(&deps, info.sender.clone(), token_info.token_address.clone())?;

        if total_supply.is_zero() {
            shares += amount;
        } else {
            shares+=amount.checked_mul(total_supply).map_err(StdError::overflow)?.checked_div(balance_of).map_err(StdError::divide_by_zero)?
        };

        give_allowance(env.clone(), info.clone(), amount, token_info.token_address.clone())?;
        total_supply += shares;
        TOTAL_SUPPLY.save(deps.storage, &total_supply)?;
        balance += shares;
        BALANCE_OF.save(deps.storage, info.sender.clone(), &balance)?;

        let transfer_from_msg = cw20::Cw20ExecuteMsg::TransferFrom {
            owner: info.sender.to_string(),
            recipient: env.contract.address.to_string(),
            amount,
        };
        let msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: token_info.token_address.to_string(),
            msg: to_json_binary(&transfer_from_msg)?,
            funds: info.funds,
        });

        Ok(Response::new().add_attribute("action", "deposit").add_message(msg))
    }

    pub fn execute_withdraw(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        shares: Uint128,
    ) -> Result<Response, ContractError> {
        let token_info = TOKEN_INFO.load(deps.storage)?;
        let mut total_supply = TOTAL_SUPPLY.load(deps.storage)?;
        let mut balance = BALANCE_OF.load(deps.storage, info.sender.clone()).unwrap_or(Uint128::zero());
        let balance_of = get_token_balance_of(&deps, info.sender.clone(), token_info.token_address.clone())?;
        let amount=shares.checked_mul(balance_of).map_err(StdError::overflow)?.checked_div(total_supply).map_err(StdError::divide_by_zero)?;
        total_supply -= shares;
        TOTAL_SUPPLY.save(deps.storage, &total_supply)?;
        balance -= shares;
        BALANCE_OF.save(deps.storage, info.sender.clone(), &balance)?;

        let transfer_msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: info.sender.to_string(),
            amount,
        };
        let msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: token_info.token_address.to_string(),
            msg: to_json_binary(&transfer_msg)?,
            funds: info.funds,
        });

        Ok(Response::new().add_attribute("action", "withdraw").add_message(msg))
    }

    pub fn get_token_balance_of(
        deps: &DepsMut,
        user_address: Addr,
        cw20_contract_addr: Addr,
    ) -> Result<Uint128, ContractError> {
        let query_msg = cw20::Cw20QueryMsg::Balance {
            address: user_address.to_string(),
        };
        let msg: cw20::BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart { 
            contract_addr: cw20_contract_addr.to_string(), 
            msg: to_json_binary(&query_msg)? 
        }))?;
        Ok(msg.balance)
    }

    pub fn give_allowance(
        env: Env,
        info: MessageInfo,
        amount: Uint128,
        cw20_contract_addr: Addr,
    ) -> Result<Response, ContractError> {
        let allowance_msg = cw20::Cw20ExecuteMsg::IncreaseAllowance {
            spender: env.contract.address.to_string(),
            amount,
            expires: None,
        };
        let msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: cw20_contract_addr.to_string(),
            msg: to_json_binary(&allowance_msg)?,
            funds: vec![],
        });

        Ok(Response::new().add_attribute("action", "give_allowance").add_message(msg))
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTotalSupply {  } => query::get_total_supply(deps),
        QueryMsg::GetBalanceOf { address } => query::get_balance_of(deps, address),
    }
}

pub mod query {
    use super::*;
    pub fn get_total_supply(deps: Deps) -> StdResult<Binary> {
        let total_supply = TOTAL_SUPPLY.load(deps.storage)?;
        to_json_binary(&total_supply)
    }

    pub fn get_balance_of(deps: Deps, address: Addr) -> StdResult<Binary> {
        let balance = BALANCE_OF.load(deps.storage, address)?;
        to_json_binary(&balance)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Uint128};
    use cw_multi_test::{App, BankKeeper, ContractWrapper, Executor, IntoAddr};

    use cw20::{Cw20Coin, Cw20ExecuteMsg, Cw20QueryMsg};
    use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;


    use super::*;

    fn setup_cw20_contract(app: &mut App, owner: Addr) -> Addr {
        let cw20_code = ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        );
        let cw20_code_id = app.store_code(Box::new(cw20_code));

        let cw20_addr = app
            .instantiate_contract(
                cw20_code_id,
                owner.clone(),
                &Cw20InstantiateMsg {
                    name: "Test Token".to_string(),
                    symbol: "TTK".to_string(),
                    decimals: 6,
                    initial_balances: vec![Cw20Coin {
                        address: owner.to_string(),
                        amount: Uint128::new(1000000),
                    }],
                    mint: None,
                    marketing: None,
                },
                &[],
                "CW20 Test Token",
                None,
            )
            .unwrap();
        cw20_addr
    }

    fn setup_vault_contract(app: &mut App, owner: Addr, cw20_addr: Addr) -> Addr {
        let vault_code = ContractWrapper::new(execute, instantiate, query);
        let vault_code_id = app.store_code(Box::new(vault_code));

        let vault_addr = app
            .instantiate_contract(
                vault_code_id,
                owner.clone(),
                &InstantiateMsg {
                    token_symbol: "TTK".to_string(),
                    token_contract_address: cw20_addr.clone(),
                },
                &[],
                "Token Vault",
                None,
            )
            .unwrap();
        vault_addr
    }

    #[test]
    fn test_deposit_cw20_token() {
        let mut app = App::default();
        let owner = app.api().addr_make("owner");

        // Step 1: Setup CW20 Token Contract
        let cw20_addr = setup_cw20_contract(&mut app, owner.clone());

        // Step 2: Setup Vault Contract
        let vault_addr = setup_vault_contract(&mut app, owner.clone(), cw20_addr.clone());

        // Step 3: Approve Vault Contract to Spend Tokens
        app.execute_contract(
            owner.clone(),
            cw20_addr.clone(),
            &Cw20ExecuteMsg::IncreaseAllowance {
                spender: vault_addr.to_string(),
                amount: Uint128::new(500),
                expires: None,
            },
            &[],
        )
        .unwrap();

        // Step 4: Deposit Tokens into Vault
        let response = app
            .execute_contract(
                owner.clone(),
                vault_addr.clone(),
                &ExecuteMsg::Deposit {
                    amount: Uint128::new(500),
                },
                &[],
            )
            .unwrap();

        assert!(response.events.iter().any(|e| e.ty == "wasm" && e.attributes.iter().any(|attr| attr.key == "action" && attr.value == "deposit")));

        // Step 5: Query Vault Balance
        let vault_balance: cw20::BalanceResponse = app
            .wrap()
            .query_wasm_smart(
                &cw20_addr,
                &Cw20QueryMsg::Balance {
                    address: vault_addr.to_string(),
                },
            )
            .unwrap();

        assert_eq!(vault_balance.balance, Uint128::new(500));

        // Step 6: Query User Shares in Vault
        let user_shares: Uint128 = app
            .wrap()
            .query_wasm_smart(
                &vault_addr,
                &QueryMsg::GetBalanceOf {
                    address: owner.clone(),
                },
            )
            .unwrap();

        assert_eq!(user_shares, Uint128::new(500));
    }
}

