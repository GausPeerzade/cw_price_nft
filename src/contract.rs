#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coin, to_json_binary, Addr, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Reply, ReplyOn, Response, StdResult, SubMsg, Uint128, WasmMsg,
};

use cw0::parse_reply_instantiate_data;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{DENOM, ID_COUNTER, NFTADDR, OWNER, PRICE, STATUS, URI};

use cosmwasm_std::Empty;

use cw721_base::ExecuteMsg::Mint;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use cw721_base::{msg, Cw721Contract};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:fractit_nft";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    OWNER.save(deps.storage, &msg.owner)?;
    PRICE.save(deps.storage, &msg.price)?;
    STATUS.save(deps.storage, &false)?;
    URI.save(deps.storage, &msg.uri)?;
    DENOM.save(deps.storage, &msg.denom)?;
    NFTADDR.save(deps.storage, &None)?;

    let nft_msg: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            code_id: msg.cw721_id,
            msg: to_json_binary(&Cw721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
            admin: None,
            label: String::from("Instantiate fixed price NFT contract"),
        }
        .into(),
        id: INSTANTIATE_TOKEN_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new().add_submessages(nft_msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let nft_address = NFTADDR.load(deps.storage)?;

    if nft_address.is_some() {
        return Err(ContractError::AlreadySet {});
    }

    if msg.id != INSTANTIATE_TOKEN_REPLY_ID {
        return Err(ContractError::InvalidReplyId {});
    }

    let reply = parse_reply_instantiate_data(msg).unwrap();
    let address = Addr::unchecked(reply.contract_address).into();

    NFTADDR.save(deps.storage, &Some(address))?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {} => unimplemented!(),
        ExecuteMsg::Claim {} => unimplemented!(),
        ExecuteMsg::Pause {} => unimplemented!(),
    }
}

fn execute_mint(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let price = PRICE.load(deps.storage)?;
    let denom = DENOM.load(deps.storage)?;
    let uri = URI.load(deps.storage)?;
    let counter = ID_COUNTER.load(deps.storage).unwrap_or_default();
    let contract_address = NFTADDR.load(deps.storage)?.unwrap();
    let user = info.sender;

    let mint_sent = amount_sent(info.funds, denom);

    if mint_sent < price {
        return Err(ContractError::InvalidMintAmount {});
    }

    let mint_msg: cw721_base::ExecuteMsg<Empty, Empty> = cw721_base::ExecuteMsg::Mint {
        token_id: counter.to_string(),
        owner: user.clone().into_string(),
        token_uri: Some(uri),
        extension: Empty {},
    };

    let msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: contract_address.to_string(),
        msg: to_json_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    ID_COUNTER.save(deps.storage, &(counter + Uint128::one()))?;
    Ok(Response::new().add_message(msg))
}

fn amount_sent(sent_funds: Vec<Coin>, denom: String) -> Uint128 {
    let amount = sent_funds
        .iter()
        .find(|coin| coin.denom == denom)
        .map(|coin| coin.amount)
        .unwrap_or(Uint128::zero());
    return amount;
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coin, coins, Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use super::*;

    fn protocol_contract() -> Box<dyn Contract<Empty>> {
        fn protocol_contract() -> Box<dyn Contract<Empty>> {
            let contract: ContractWrapper<Empty> =
                ContractWrapper::new(execute, instantiate, query);
            Box::new(contract)
        }
    }

    fn nft_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
}
