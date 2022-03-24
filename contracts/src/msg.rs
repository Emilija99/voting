use cosmwasm_std::{Uint128, WasmMsg, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::voter::Vote;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub min_deposit: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    CreateProposal {
        title: String,
        description: String,
        messages: Option<Vec<WasmMsg>>,
        quorum: Uint128,
        threshold: Decimal,
        expires: u64,
    },
    Vote{
        vote:Vote,
        proposal_id:u64
    },
    CalculateResults{
        proposal_id:u64
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Proposals{
        page_num:u64,
        page_size:u64
    }
}


