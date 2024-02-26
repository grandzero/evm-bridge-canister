# **EVM Bridge Canister for Internet Computer**

Welcome to the EVM Bridge Canister project, a cutting-edge solution designed to seamlessly integrate Ethereum Virtual Machine (EVM) functionalities with the Internet Computer Protocol (ICP). This project leverages the capabilities of the EVM RPC Canister, enabling developers to interact with EVM-based networks directly from the Internet Computer.

Project currently includes only POC functions like signing raw transaction or getting data from contract using rpc. You can see TODO below.

## **Features**

- **EVM Interaction**: Perform essential EVM operations, such as retrieving gas prices and signing example transactions.
- **EVM RPC Canister Integration**: Utilize the newly announced [EVM RPC Canister](https://github.com/internet-computer-protocol/evm-rpc-canister) to facilitate communication with EVM networks.
- **Customizable and Expandable**: Easily extend the canister's functionalities to suit your specific needs.
- **Sign EVM Transactions: Project uses [ic-evm-sign](https://github.com/nikolas-con/ic-evm-sign/tree/master)** crates codes but updated for current ic-cdk and candid crates. In original repo versions are ic-cdk = “0.5.2” and candid=”0.7.14” but this projects uses lates improvements like sign_with_ecdsa function under management canister. Repo maintained for this project and updated version is not public for now.

## **Getting Started**

Before diving into the project, ensure you have the necessary tools installed on your system. This project requires the DFINITY Canister SDK (dfx) and Rust.

### **Prerequisites**

- DFINITY Canister SDK
- Rust
- Node.js (for managing frontend interactions, if applicable)

### **Installation**

1. Clone the repository to your local machine.
2. Navigate into the project directory:

   ```bash
   cd evm_bridge_canister/
   ```

3. Install the required dependencies:

   ```bash
   dfx deploy evm_rpc --argument '(record { nodesInSubnet = 28 })'
   dfx deploy evm_bridge_canister_backend --argument '("https://polygon-mumbai-pokt.nodies.app")'
   ```

## **Usage**

The EVM Bridge Canister allows you to perform various operations with ease. Here are some of the key functionalities:

- **Get Gas Price**: Retrieve the current gas price from the specified EVM network. (Using RPC url)
- **Create Address for Owner**: Generate a new address for the canister owner. This should be called once
- **Get Canister Address**: Obtain the address associated with the canister.
- **Get Data From Source**: This functions is used for retrieving data from contracts using evm_rpc canister. It requires contract addresses to be stored in state and chain_id as argument. Chain id can be 97(BSC Testnet) or Mumbai(80001) for now.
- **RPC Call**: Send RPC requests to interact with contracts deployed on EVM-compatible networks. Functions first parameter should be signed transaction or encoded view function call
- **Read Mumbai Write Binance** : This function retrieves data from mumbai contract, prepares function call data, signs the data with canisters eth wallet and then sends signed raw transaction using rpc with "eth_sendRawTransaction" method.

## **Development**

To start local development:

```bash
# Start the local replica
dfx start --background

# Deploy your canisters to the local replica
dfx deploy evm_rpc --argument '(record { nodesInSubnet = 28 })'
dfx deploy evm_bridge_canister_backend --argument '(vec {record {rpc_url="https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr"; network_name="mumbai"; contract_address="0x3270934BF219CBD66697DE72fB218A2cC44bBfe9"; chain_id=80001 }; record {rpc_url="https://data-seed-prebsc-1-s1.binance.org:8545"; network_name="binance"; contract_address="0x17E9C819Ea0fd3793a41248fA0724a35CD7Ff8a7"; chain_id=97;}}, "bkyz2-fmaaa-aaaaa-qaaaq-cai")'
```

### **Testing**

Tests will be implemented

## **Deployment**

To deploy your canister to the Internet Computer, follow the standard **`dfx`** deployment process. Ensure you have configured your **`dfx.json`** file with the appropriate network settings.

## **Contributing**

Contributions are welcome! If you have suggestions for improvements or encounter any issues, please open an issue or submit a pull request.

## **License**

This project is licensed under the MIT License - see the LICENSE file for details.

# **Usage**

This project is PoC implementation and test for Cross-Chain EVM Wallet with bridge functionality. It currenlt simulates receiving data from EVM contracts and writing data to EVM contracts. Current functions and implementations are not PRODUCTION READY. For demo purposes, only mumbai and binance testnet networks are used. To be able to test the canister below steps should be done :

1. Deploy evm_rpc canister : This canister is used for communication with EVM networks. More data can be found here : https://internetcomputer.org/docs/current/developer-docs/integrations/ethereum/evm-rpc
2. Deploy canister using deploy.sh : This file contains necessary initialization values like rpc url's, contract addresses etc.
3. Call set_rpc_canister : This function requires evm_rpc canister's id so that this canister will communicate using this principal with inter-canister calls.
4. Call create_address_for_owner : This function creates an eth wallet to be able to sign transactions and communicate with EVM networks. This function will be called only by owner of canister in the future but currently anyone can call for poc purposes. It should be called once
5. Test if all datas successfully created by calling get_canister_address and get_data_from_source functions : get_canister_address should return an evm address. This address is used for signing transactions. And get_data_from_source function requires a chain id which can be either 97 or 80001 for poc purposes. This will return currently saved data on testnet contracts
6. Fund evm address : To be able to send transaction, make sure canister address is funded in binance testnet network and polygon mumbai network. Small amounts like 0.1 MATIC and 0.1 tBNB should work for provided contracts.
7. Call read_mumbai_write_binance function : This function reads stored data from mumbai contract and writes it to binance testnet network contract. It requires binance wallet to be funded to be able to send transaction
8. Call read_binance_write_mumbai function : This function reads stored data from binance testnet contract and writes it to mumbai testnet network contract. It requires mumbai wallet to be funded to be able to send transaction

# **Warning**

This project consists and uses of multiple different projects codes from icp ecosystem. For rpc operations, it uses inter-canister calls with [evm_rpc](https://github.com/nikolas-con/ic-evm-sign/tree/master) canister. Since evm_rpc canister uses [ic-cketh-minter](https://github.com/dfinity/ic) types under dfinity/ic repo, this project also includes it as dependency. For preparing data to be signed, [icp-eth-starter](https://github.com/dfinity/icp-eth-starter) repo has been used and for signing transactions, [ic-evm-sign](https://github.com/nikolas-con/ic-evm-sign/tree/master) repo maintained for latest versions and used in this project. Project is currently under development and more features and fixes will be added. Current version is a simple PoC for getting data from EVM networks and writing data to EVM networks.

Project's codes can be used for many purposes :

1. Cross Chain Decentralized EVM Bridge
2. Cross Chain Decentralized EVM Wallet
3. True interoperable decentralized token (with upcoming ICRC-1 token standard) that supports EVM chains
4. Decentralized BTC <-> EVM Bridge (with upcoming BTC integration)
5. Decentralized bridge DAO (Created addresses requires funding and it can be funded by a DAO in return of cutting fee for bridge usage)

Example contract has been added under contracts folder.

Since project is created for ICP hackathon, it doesn't include some of the basic functionalities. After refactors and bug fixes, more flexible implementation will be added. Currently it calls one contract from each network but future architecture will allow users to provide their desired functions calls with desired datas using interfaces in solidity.

Canister uses "eth_sendRawTransaction" method for sending transactions using rpc and "eth_call" for getting data from contracts.

# ** Bugs and Refactor Notes **

- [x] Remove unused codes from rust files
- [x] Edit state for better architecture
- [x] Edit functions and remove copy paste code
- [x] Persistently fix the nonce issue and use state efficiently
- [x] Store only necessary data for wallets
- [ ] Created wallets should be funded before usage
- [ ] Error handling
- [ ] Security fixes (Creating address by owner, setting rpc canister)
- [ ] Key generation for different environments
- [ ] Serialize json rpc response
- [ ] Flexible function call across network
- [ ] Call different addresses securely

## To Do

- [x] Store canister eth address in state
- [x] Add more evm rpc urls
- [x] Add function for sending signed transaction
- [ ] Add token functions for mint and burn
- [ ] Refactor
- [ ] Add spawn timer for getting changes on evm contract
- [x] Add function to get latest changes on contract
- [ ] Store processed (cross chain transferred data unique hashes for preventing reply messages consume gas funds) datas in state
- [ ] Add cross chain token transfer mechanism with mint&burn approach
- [ ] Allow DAO structure to fund the canister for sending transactions so that dao investors can get fee from each operation
- [ ] Refactor error handling
- [ ] Improve security (currently uses owners principal but can be considered different approaches)
- [ ] Frontend implementation
