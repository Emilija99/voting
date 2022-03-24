const accounts = [
  {
    name: 'a',
    address: 'secret1855r2dssxpt5e96vdu69hfdl5egx74e4kw8h0f',
    mnemonic: 'current desk yard use solve teach afraid notable tomorrow flag remove throw explain price sell fetch own side cream annual agree ivory sock social'
  },
  {
    name: 'b',
    address: 'secret14pwqxvp5je72nqqct0gf404737scj0s7p4teu9',
    mnemonic: 'initial shine flock pen expose enhance stamp lab slice burden fantasy slush pet knee ranch avoid caution pretty cycle glad host record design immense'
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