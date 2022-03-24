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

async function calculate_results(contract,sender,customFees,proposal_id){
  const calc_message={proposal_id};
  const response=await contract.tx.calculate_results({account:sender,customFees},calc_message);
  console.log(response);
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
  //create proposal
  const proposal_msg={title:"prop1",description:"abcd",quorum:"3000",threshold:"0.51",expires:Math.floor(Date.now()/1000+36)};
  const response=await contract.tx.create_proposal({account:contract_owner,transferAmount: transferAmount, customFees: customFees},proposal_msg);
  console.log(response);

  //query proposals
  const proposals=await contract.query.proposals({page_num:1,page_size:5});
  console.log(proposals);

  //vote
  const transferAmount1 = [{"denom": "uscrt", "amount": "3500"}];
  const vote_msg={vote:"Yes",proposal_id:1};
  const r1=await contract.tx.vote({account:getAccountByName("b"),transferAmount:transferAmount1,customFees:customFees},vote_msg);
  console.log(r1);

  //query proposals
  const proposals1=await contract.query.proposals({page_num:1,page_size:5});
  console.log(proposals1[0].voters[0]);
  console.log(proposals1[0].deposit);
  
  //calculate results
  await new Promise(r => setTimeout(r, 36000));
  await calculate_results(contract,contract_owner,customFees,1);
  

  

 
}

module.exports = { default: run };
