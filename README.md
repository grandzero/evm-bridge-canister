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
- **Sign Example Transaction**: Sign a sample transaction for demonstration purposes.
- **Interact with EVM Contracts**: Send RPC requests to interact with contracts deployed on EVM-compatible networks.
- **Get Data From Contract** : Get’s contract data using evm_rpc canister. Contract example is included under contract folder. Expects the contract address and principal of evm_rpc

## **Development**

To start local development:

```bash
# Start the local replica
dfx start --background

# Deploy your canisters to the local replica
dfx deploy evm_rpc --argument '(record { nodesInSubnet = 28 })'
dfx deploy evm_bridge_canister_backend --argument '("https://polygon-mumbai-pokt.nodies.app")'
```

### **Testing**

Tests will be implemented

## **Deployment**

To deploy your canister to the Internet Computer, follow the standard **`dfx`** deployment process. Ensure you have configured your **`dfx.json`** file with the appropriate network settings.

## **Contributing**

Contributions are welcome! If you have suggestions for improvements or encounter any issues, please open an issue or submit a pull request.

## **License**

This project is licensed under the MIT License - see the LICENSE file for details.
