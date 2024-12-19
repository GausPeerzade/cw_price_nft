use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
    pub price: Uint128,
    pub uri: String,
    pub cw721_id: u64,
    pub name: String,
    pub symbol: String,
    pub denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Mint {},
    Claim {},
    Pause {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
