const accounts = [
  {
    name: 'a',
    address: 'secret1d803jjq909ynkw23um3s6vzeqy8esy8pkgxtaw',
    mnemonic: 'skate find noodle barely depth pair uncle alert mandate advice video actor undo notice dumb frame bright foil recycle wagon rate pelican beef choice'
  },
  {
    name: 'b',
    address: 'secret1vh30r7hudlxqan2zlss35420rqh8jwfxamnkj6',
    mnemonic: 'say type rug ancient false timber recipe shaft grab pill cruel seat settle pumpkin mixed horse omit frozen cupboard journey bike use milk when'
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