use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub cw20_wl: Vec<(String, Addr)>,
}

pub const CONFIG: Item<Config> = Item::new("config");
