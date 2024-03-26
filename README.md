# Pendulum-Solidity Wrappers

A collection of contracts written in Solidity and compiled to WASM using Solang compiler for deployment on the
Contracts pallet.

These contracts implement well know interfaces (`IERC20`, `IPriceOracleGetter`), but the novel feature
is the ability to call Pendulum's chain extensions which allows them to interact with 
the logic of the pallets. Notably, fetching latest price of assets from the chain and interactions with the tokens pallet.

Please see 'docs' for a more information.

# Compiling the solidity contract

### Install solang

```bash
brew install hyperledger/hyperledger/solang
```

### Compile the contract

```bash
solang compile --target polkadot {path/to/contract}.sol
```

### Deploying the contract

As a result you will get a file called `{contract}.contract`.
You can now deploy this contract to a parachain using Contracts UI.





