const { Contract, getAccountByName, getLogs } = require("secret-polar");

async function deploy_contract(contract,contract_owner){

  const deploy_response=await contract.deploy(contract_owner,{ // custom fees
    amount: [{ amount: "750000", denom: "uscrt" }],
    gas: "3000000",
  });
  console.log(deploy_response);
}

async function instantiate_contract(contract,contract_owner,msg,label){
  const contract_info=await contract.instantiate(msg,label,contract_owner);
  console.log(contract_info);
}

async function run () {
  const contract_owner = getAccountByName("a");
  const contract = new Contract("voting");
  await contract.parseSchema();
  await deploy_contract(contract,contract_owner);
  await instantiate_contract(contract,contract_owner,{"min_deposit": "1000"},"contract1");

  const transferAmount = [{"denom": "uscrt", "amount": "15000000"}] // 15 SCRT
  const customFees = { // custom fees
    amount: [{ amount: "750000", denom: "uscrt" }],
    gas: "3000000",
  }
  const proposal_msg={title:"prop1",description:"abcd",quorum:"3000",threshold:"0.51",expires:Math.floor(Date.now()/1000+3600)};
  const response=await contract.tx.create_proposal({account:contract_owner,transferAmount: transferAmount, customFees: customFees},proposal_msg);
  console.log(response);

  //query proposals
  const proposals=await contract.query.proposals({page_num:1,page_size:5});
  console.log(proposals);

  //vote
  const transferAmount1 = [{"denom": "uscrt", "amount": "1200"}];
  const vote_msg={vote:"Yes",proposal_id:1};
  const r1=await contract.tx.vote({account:contract_owner,transferAmount:transferAmount1,customFees:customFees},vote_msg);
  console.log(r1);

  //query proposals
  const proposals1=await contract.query.proposals({page_num:1,page_size:5});
  console.log(proposals1[0].voters[0]);
  console.log(proposals1[0].deposit);

 /* const deploy_response = await contract.deploy(
    contract_owner,
    { // custom fees
      amount: [{ amount: "750000", denom: "uscrt" }],
      gas: "3000000",
    }
  );
  console.log(deploy_response);

  const contract_info = await contract.instantiate({"min_deposit": "1000"}, "deploy test", contract_owner);
  console.log(contract_info);*/

  // use below line if contract initiation done using another contract
  // const contract_addr = "secret76597235472354792347952394";
  // contract.instantiatedWithAddress(contract_addr);

  /*const inc_response = await contract.tx.increment({account: contract_owner});
  console.log(inc_response);
  // to get logs as a key:value object
  // console.log(getLogs(inc_response));

  const response = await contract.query.get_count();
  console.log(response);

  const transferAmount = [{"denom": "uscrt", "amount": "15000000"}] // 15 SCRT
  const customFees = { // custom fees
    amount: [{ amount: "750000", denom: "uscrt" }],
    gas: "3000000",
  }
  const ex_response = await contract.tx.increment(
    {account: contract_owner, transferAmount: transferAmount}
  );
  // const ex_response = await contract.tx.increment(
  //   {account: contract_owner, transferAmount: transferAmount, customFees: customFees}
  // );
  console.log(ex_response);*/
}

module.exports = { default: run };
