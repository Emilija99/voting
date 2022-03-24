use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{ Storage, Uint128, Api, Querier, Extern, StdError};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
  
    pub min_deposit:Uint128
}

impl State{
    pub fn get_min_deposit<S: Storage, A: Api, Q: Querier>(deps: & Extern<S, A, Q>)->Result<Uint128,StdError>{
        Ok(config_read(&deps.storage).load()?.min_deposit)

    }
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
