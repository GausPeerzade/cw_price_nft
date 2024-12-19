use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const PRICE: Item<Uint128> = Item::new("price");
pub const STATUS: Item<bool> = Item::new("status");
pub const URI: Item<String> = Item::new("uri");
pub const NFTADDR: Item<Option<Addr>> = Item::new("nft_address");
pub const DENOM: Item<String> = Item::new("denom");
pub const ID_COUNTER: Item<Uint128> = Item::new("id_counter");
