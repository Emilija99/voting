use cosmwasm_std::{Uint128, CanonicalAddr};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Vote{
    Yes,
    No,
    Abstain
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Voter {
    pub voting_power: Uint128,
    pub address: CanonicalAddr,
    pub vote:Vote
}

impl Voter{
    pub fn new(amount:Uint128,address:CanonicalAddr,vote:Vote)->Self{
        Self{
            voting_power: amount,
            address,
            vote,
        }
    }
}