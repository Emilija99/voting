use std::convert::TryInto;

use cosmwasm_std::{
    from_slice, to_vec, Api, BankMsg, CanonicalAddr, Coin, CosmosMsg, Empty,  Extern,
     HumanAddr, Querier, StdError, StdResult, Storage, Uint128, WasmMsg,Decimal
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use schemars::JsonSchema;
use secret_toolkit::{
    serialization::Json,
    storage::{AppendStore, AppendStoreMut},
};
use serde::{Deserialize, Serialize};

use crate::voter::{Vote, Voter};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    id: u64,
    title: String,
    description: String,
    pub messages: Option<Vec<WasmMsg>>,
    quorum: Uint128,
    threshold: Decimal,
    expires: u64,
    voters: Vec<Voter>,
    pub results_calculated: bool,
}

impl Proposal {
    pub fn new(
        id: u64,
        title: String,
        description: String,
        messages: Option<Vec<WasmMsg>>,
        quorum: Uint128,
        threshold: Decimal,
        expires: u64,
    ) -> Self {
        return Self {
            title,
            description,
            messages,
            quorum,
            threshold,
            expires,
            voters: vec![],
            id,
            results_calculated: false,
        };
    }
    pub fn next_id<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
    ) -> Result<u64, StdError> {
        let total = from_slice::<u64>(&deps.storage.get(b"total").unwrap())?;
        deps.storage.set(b"total", &to_vec(&(total + 1))?);
        Ok(total + 1)
    }

    pub fn create<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        title: String,
        description: String,
        messages: Option<Vec<WasmMsg>>,
        quorum: Uint128,
        threshold: Decimal,
        expires: u64,
    ) -> Result<(), StdError> {
        let proposal = Self::new(
            Self::next_id(deps)?,
            title,
            description,
            messages,
            quorum,
            threshold,
            expires,
        );
        let mut store = PrefixedStorage::new(b"/proposals/", &mut deps.storage);
        let mut a_store = AppendStoreMut::<Proposal, _, _>::attach_or_create_with_serialization(
            &mut store, Json,
        )?;
        a_store.push(&proposal)?;
        Ok(())
    }
    pub fn get<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        id: u64,
    ) -> Result<Proposal, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/proposals/", &deps.storage);
        let a_store = AppendStore::<Proposal, _, _>::attach_with_serialization(&store, Json)
            .ok_or(StdError::generic_err("Proposals not created"))??;
        a_store
            .iter()
            .find(|p| p.as_ref().unwrap().id.eq(&id))
            .ok_or(StdError::generic_err("Proposal not found"))?
    }

    pub fn update<S: Storage, A: Api, Q: Querier>(
        deps: &mut Extern<S, A, Q>,
        proposal: &Proposal,
    ) -> Result<(), StdError> {
        let mut store = PrefixedStorage::new(b"/proposals/", &mut deps.storage);
        let mut a_store = AppendStoreMut::<Proposal, _, _>::attach_or_create_with_serialization(
            &mut store, Json,
        )?;
        let index = a_store
            .iter()
            .position(|p| p.unwrap().id.eq(&proposal.id))
            .ok_or(StdError::generic_err("Proposal not found"))?;
        a_store.set_at(index.try_into().unwrap(), proposal)?;
        Ok(())
    }

    pub fn get_proposals<S: Storage, A: Api, Q: Querier>(
        deps: &Extern<S, A, Q>,
        page_num: usize,
        page_size: usize,
    ) -> Result<Vec<Proposal>, StdError> {
        let store = ReadonlyPrefixedStorage::new(b"/proposals/", &deps.storage);
        let a_store = AppendStore::<Proposal, _, _>::attach_with_serialization(&store, Json)
            .ok_or(StdError::generic_err("Proposals not created"))??;
        Ok(a_store
            .iter()
            .map(|x| x.unwrap())
            .skip((page_num - 1) * page_size)
            .take(page_size)
            .collect::<Vec<Proposal>>())
    }

    pub fn user_already_voted(&self, address: &CanonicalAddr) -> bool {
        self.voters.iter().any(|v| v.address.eq(address))
    }

    pub fn add_voter(&mut self, voter: Voter) {
        self.voters.push(voter)
    }

    pub fn expired(&self, current_timestamp: u64) -> bool {
        self.expires < current_timestamp
    }
    pub fn total_amount(&self) -> Uint128 {
        self.voters
            .iter()
            .map(|v| v.voting_power)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn valid(&self) -> bool {
        self.total_amount() >= self.quorum
    }

    pub fn passed(&self) -> bool {
        let total_votes = self
            .voters
            .iter()
            .filter(|v| v.vote.ne(&Vote::Abstain))
            .map(|v| v.voting_power)
            .reduce(|a, b| a + b)
            .unwrap().u128();
        let yes_votes = self
            .voters
            .iter()
            .filter(|v| v.vote.eq(&Vote::Yes))
            .map(|v| v.voting_power)
            .reduce(|a, b| a + b)
            .unwrap()
            .u128();
      
        let d=Decimal::from_ratio(yes_votes, total_votes);
       
        if d>self.threshold {
            true
        } else {
            false
        }
    }
    pub fn bank_messages<S: Storage, A: Api, Q: Querier>(
        &self,
        contract_addr: HumanAddr,
        deps: &Extern<S, A, Q>,
    ) -> StdResult<Vec<CosmosMsg<Empty>>> {
        let mut messages: Vec<CosmosMsg<Empty>> = vec![];
        for voter in self.voters.iter() {
            let message: CosmosMsg<Empty> = CosmosMsg::Bank(BankMsg::Send {
                from_address: contract_addr.clone(),
                to_address: deps.api.human_address(&voter.address)?,
                amount: vec![Coin {
                    denom: "uscrt".to_string(),
                    amount: voter.voting_power,
                }],
            });
            messages.push(message);
        }
        Ok(messages)
    }
    pub fn proposal_messages(&self) -> Vec<CosmosMsg<Empty>> {
        self.messages
            .as_ref()
            .unwrap()
            .iter()
            .map(|msg| CosmosMsg::Wasm(msg.clone()))
            .collect()
    }
}
