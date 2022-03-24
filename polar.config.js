const accounts = [
  {
    name: 'a',
    address: 'secret14a4la45v5l9gnspmq7wt5tcwy5dllu20ly4ngl',
    mnemonic: 'patient close fox banner sauce path soul float napkin wink gloom response oil envelope attack change wrong pumpkin roof author hedgehog load hair girl'
  },
  {
    name: 'b',
    address: 'secret1t3jchcxdnjuuehrt5889es5f7vpwwdjafcx4gz',
    mnemonic: 'obey section school behind thrive holiday make rural faith curtain flight satisfy next claim style casino skull dad worth analyst bench wood include bonus'
  }
];

const networks = {
  default: {
    endpoint: "http://192.168.1.95:1337/"
  },
  localnet: {
    endpoint: 'http://192.168.1.95:1337/',
    accounts: accounts,
  },
  development: {
    endpoint: 'tcp://0.0.0.0:26656',
    chainId: 'secretdev-1',
    types: {}
  },
 
};

module.exports = {
  networks: {
    default: networks.localnet,
    localnet: networks.localnet,
    development: networks.development
  },
  mocha: {
    timeout: 60000
  },
  rust: {
    version: "1.55.0",
  }
};