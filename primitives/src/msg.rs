use std::result;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOwnerResponse)]
    GetOwner {},
    
    #[returns(GetIntegerResponse)]
    Integer {},
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: Addr,
}

#[cw_serde]
pub struct GetIntegerResponse {
    pub works: bool,
}
