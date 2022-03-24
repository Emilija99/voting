use std::convert::TryInto;

use cosmwasm_std::{
    to_binary, to_vec, Api, Binary, Coin, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage, Uint128, WasmMsg, Decimal,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::proposal::Proposal;
use crate::state::{config,  State};
use crate::voter::{Vote, Voter};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    deps.storage.set(b"total", &to_vec(&0)?);
    let state = State {
        min_deposit: msg.min_deposit,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::CreateProposal {
            title,
            description,
            messages,
            quorum,
            threshold,
            expires,
        } => try_create(
            deps,
            env,
            title,
            description,
            messages,
            quorum,
            threshold,
            expires,
        ),
        HandleMsg::Vote { vote, proposal_id } => try_vote(deps, env, vote, proposal_id),
        HandleMsg::CalculateResults { proposal_id } => try_calculate_results(deps, env, proposal_id),
    }
}

pub fn try_vote<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    vote: Vote,
    proposal_id: u64,
) -> StdResult<HandleResponse> {
    let mut proposal = Proposal::get(deps, proposal_id)?;
    if proposal.expired(env.block.time) {
        return Err(StdError::generic_err("Proposal expired"));
    }
    if proposal.user_already_voted(&deps.api.canonical_address(&env.message.sender)?) {
        return Err(StdError::generic_err("User already voted"));
    }
    proposal.add_voter(Voter::new(
        calculate_amount(env.message.sent_funds),
        deps.api.canonical_address(&env.message.sender)?,
        vote,
    ));
    Proposal::update(deps, &proposal)?;
    Ok(HandleResponse::default())
}

pub fn try_create<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    title: String,
    description: String,
    messages: Option<Vec<WasmMsg>>,
    quorum: Uint128,
    threshold: Decimal,
    expires: u64,
) -> StdResult<HandleResponse> {
    if calculate_amount(env.message.sent_funds) < State::get_min_deposit(deps)? {
        return Err(StdError::generic_err("Deposit not big enough"));
    }
    Proposal::create(
        deps,
        title,
        description,
        messages,
        quorum,
        threshold,
        expires,
    )?;
    Ok(HandleResponse::default())
}

pub fn try_calculate_results<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    proposal_id: u64,
) -> StdResult<HandleResponse> {
    let mut proposal = Proposal::get(deps, proposal_id)?;
    if proposal.expired(env.block.time) == false {
        return Err(StdError::generic_err("Voting still in progress"));
    }
    if proposal.valid() == false {
        return Err(StdError::generic_err("Proposal not valid"));
    }

    if proposal.results_calculated {
        return Err(StdError::generic_err(
            "Results have already been calculated",
        ));
    }
    proposal.results_calculated = true;
    Proposal::update(deps, &proposal)?;
    let mut messages = proposal.bank_messages(env.contract.address, deps)?;
    if proposal.passed() && proposal.messages != None {
        messages.append(&mut proposal.proposal_messages())
    }

    Ok(HandleResponse {
        messages,
        log: vec![],
        data: None,
    })
}

pub fn calculate_amount(coins: Vec<Coin>) -> Uint128 {
    coins
        .iter()
        .filter(|coin| coin.denom.eq("uscrt"))
        .map(|coin| coin.amount)
        .reduce(|a, b| a + b)
        .unwrap()
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Proposals {
            page_num,
            page_size,
        } => to_binary(&query_proposals(deps, page_num, page_size)?),
    }
}

fn query_proposals<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    page_num: u64,
    page_size: u64,
) -> StdResult<Vec<Proposal>> {
    Ok(Proposal::get_proposals(
        deps,
        page_num.try_into().unwrap(),
        page_size.try_into().unwrap(),
    )?)
}

/*#[cfg(test)]
mod tests{
    use cosmwasm_std::Decimal;

    #[test]
    fn test1(){
        let dec1=Decimal::from_ratio(1 as u128,4 as u128);
        println!("{}",dec1);
    }
}*/

/*#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{coins, from_binary, StdError};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(20, &[]);

        /*let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = init(&mut deps, env, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);*/
    }

    /*#[test]
    fn increment() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // anyone can increment
        let env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Increment {};
        let _res = handle(&mut deps, env, msg).unwrap();

        // should increase counter by 1
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies(20, &coins(2, "token"));

        let msg = InitMsg { count: 17 };
        let env = mock_env("creator", &coins(2, "token"));
        let _res = init(&mut deps, env, msg).unwrap();

        // not anyone can reset
        let unauth_env = mock_env("anyone", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let res = handle(&mut deps, unauth_env, msg);
        match res {
            Err(StdError::Unauthorized { .. }) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_env = mock_env("creator", &coins(2, "token"));
        let msg = HandleMsg::Reset { count: 5 };
        let _res = handle(&mut deps, auth_env, msg).unwrap();

        // should now be 5
        let res = query(&deps, QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }*/
}
*/
