pub mod bridge_types;
///0x3270934BF219CBD66697DE72fB218A2cC44bBfe9 => Mumbai
// 0x17E9C819Ea0fd3793a41248fA0724a35CD7Ff8a7 => Binance
// 0x7BFD4157F0Bbf52d96e278504275a12FB01529Cc => Goerli
// bd3sg-teaaa-aaaaa-qaaba-cai rpc evm canister
pub mod ecdsa;
pub mod eth_rpc_data;
pub mod functions;
pub mod transaction;
pub mod utils;
// use candid::Principal;
// use ic_cdk::api::call::{call, CallResult};
use cketh_common::eth_rpc::ProviderError;
use cketh_common::eth_rpc::RpcError;
use cketh_common::eth_rpc_client::providers::RpcApi;
use cketh_common::eth_rpc_client::providers::RpcService;

use ethers_core::abi::Token;
use primitive_types::U256;

use bridge_types::STATE;
#[ic_cdk::init]
async fn init(network_details: Vec<bridge_types::NetworkDetails>, rpc_canister: String) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.config.rpc_canister = rpc_canister;
        state.config.network_details_list = network_details.clone();

        for network in network_details.iter() {
            let nonce = bridge_types::Nonce {
                chain_id: network.chain_id,
                nonce: 0,
            };
            state.canister_wallet.nonces.push(nonce);
        }
    });

    //candid::types::principal::Principal::from_text(rpc_canister).unwrap();

    // let _addr =
    //     functions::create_address(Principal::from_text(ic_cdk::caller().to_text()).unwrap()).await;
}

#[ic_cdk::update]
async fn set_rpc_canister(canister: String) -> String {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.config.rpc_canister = canister;
    });
    return "Successfull".to_owned();
}

#[ic_cdk::update]
pub async fn get_gas_price(network_name: String) -> Result<String, String> {
    let state = STATE.with(|s| s.borrow().clone());
    let evm_rpc =
        candid::types::principal::Principal::from_text(state.config.rpc_canister).unwrap();
    let rpc_url = state
        .config
        .network_details_list
        .iter()
        .find(|x| x.network_name == network_name);
    let rpc_url = match rpc_url {
        Some(url) => url.rpc_url.clone(),
        None => {
            return Err("Network not found".to_string());
        }
    };

    let params = (
        &RpcService::Custom(RpcApi {
            url: rpc_url,
            headers: None,
        }), // Ethereum mainnet
        "{\"jsonrpc\":\"2.0\",\"method\":\"eth_gasPrice\",\"params\":null,\"id\":1}".to_string(),
        1000 as u64,
    );
    // let canister_principal = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();
    //let evm_rpc = candid::types::principal::Principal::from_text(evm_rpc).unwrap();
    let (cycles_result,): (Result<u128, RpcError>,) =
        ic_cdk::api::call::call(evm_rpc, "requestCost", params.clone())
            .await
            .unwrap();
    let cycles = cycles_result
        .unwrap_or_else(|e| ic_cdk::trap(&format!("error in `request_cost`: {:?}", e)));

    // Call without sending cycles
    let (result_without_cycles,): (Result<String, RpcError>,) =
        ic_cdk::api::call::call(evm_rpc, "request", params.clone())
            .await
            .unwrap();
    match result_without_cycles {
        Ok(s) => ic_cdk::trap(&format!("response from `request` without cycles: {:?}", s)),
        Err(RpcError::ProviderError(ProviderError::TooFewCycles { expected, .. })) => {
            assert_eq!(expected, cycles)
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` without cycles: {:?}", err)),
    }

    // Call with expected number of cycles
    let (result,): (Result<String, RpcError>,) =
        ic_cdk::api::call::call_with_payment128(evm_rpc, "request", params, cycles)
            .await
            .unwrap();
    match result {
        Ok(response) => {
            return Ok(response.to_string());
        }
        Err(_) => Err("RPC error occured".to_string()),
    }
}

#[ic_cdk::update]
async fn create_adress_for_owner() -> String {
    // let principal_id = ic_cdk::caller();
    let state = STATE.with(|s| s.borrow().clone());
    // TODO: Only owner should be able to call this function and it should called once inside init (if possible)
    // if state.config.owner != principal_id {
    //     return "Only owner can call".to_string();
    // }

    let addr = functions::create_address(state.config.owner).await;
    if let Ok(res) = addr {
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            // state.config.owner_public_key = res.address.to_bytes().clone();
            state.canister_wallet.public_key_str = res.address.clone();
            state.canister_wallet.public_key = res.public_key.clone();
        });
        return res.address;
    }
    // let (first_result, second_result): (first_result_type, second_result_type) =
    //     ic_cdkapi::call::call(canister_id, "method", (first_arg, second_arg)).await?;

    return "Error".to_owned();
}

#[ic_cdk::update]
async fn get_canister_address() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    // ic_cdk::print(&state.owner);
    // let principal_id = Principal::from_text(&state.owner).unwrap();
    // let addr = functions::get_address(state.config.owner).await;
    let addr = state.canister_wallet.public_key_str.clone();
    return addr;
}

async fn send_transaction(chain_id: u64, message: Vec<u8>) -> Result<String, String> {
    let data_to_sign = eth_rpc_data::get_data_with_arguments("icpCall", &[Token::Bytes(message)])
        .await
        .map_err(|e| format!("Error in get_data_with_arguments: {:?}", e))?;
    ic_cdk::println!(
        "Data to sign is : {:?}",
        eth_rpc_data::to_hex(&data_to_sign)
    );
    let signed_tx = functions::sign_custom_tx(eth_rpc_data::to_hex(&data_to_sign), chain_id).await;
    // ic_cdk::println!("Signed tx is : {:?}", signed_tx.unwra.clone());
    let data = eth_rpc_data::to_hex(&signed_tx.unwrap().sign_tx);
    ic_cdk::println!("Data is : {:?}", data.clone());
    let tx_result = rpc_call(data, None, chain_id).await;
    if tx_result.contains("error") {
        return Err(tx_result);
    }
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let nonce = state
            .canister_wallet
            .nonces
            .iter_mut()
            .find(|x| x.chain_id == chain_id)
            .unwrap();
        nonce.nonce += 1;
    });

    return Ok(tx_result);
}

#[ic_cdk::update]
async fn get_data_from_source(chain: u64) -> Result<String, String> {
    let state = STATE.with(|s| s.borrow().clone());
    let contract_address = state
        .config
        .network_details_list
        .iter()
        .find(|x| x.chain_id == chain)
        .unwrap()
        .contract_address
        .clone();
    let data = eth_rpc_data::get_data_with_arguments("getMessage", &[Token::Uint(U256::from(0))])
        .await
        .map_err(|e| format!("Error in get_data_with_arguments: {:?}", e))?;
    ic_cdk::println!("Data is : {:?}", data);
    let request_data = eth_rpc_data::to_hex(&data);
    let rpc_result = rpc_call(request_data, Some(contract_address), chain).await;
    return Ok(rpc_result);
}

#[ic_cdk::update]
async fn rpc_call(data: String, contract_address: Option<String>, chain: u64) -> String {
    // let state = STATE.with(|s| s.borrow().clone());
    ic_cdk::println!("Data in rpc is : {}", data);
    let state = STATE.with(|s| s.borrow().clone());
    ic_cdk::println!("EVM RPC is : {}", state.config.rpc_canister);
    let evm_rpc =
        candid::types::principal::Principal::from_text(state.config.rpc_canister.clone()).unwrap();

    let rpc_url = state
        .config
        .network_details_list
        .iter()
        .find(|x| x.chain_id == chain)
        .unwrap()
        .rpc_url
        .clone();
    ic_cdk::println!("RPC URL is : {}", rpc_url);
    let payload = match contract_address {
        Some(addr) => {
            format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[{{ \"to\":\"{}\", \"data\":\"{}\"}}, \"latest\"],\"id\":1}}", addr, data)
        }
        None => {
            format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\",\"params\":[\"{}\"],\"id\":1}}", data)
        }
    };
    let params = (
        &RpcService::Custom(RpcApi {
            url: rpc_url,
            headers: None,
        }), // Ethereum mainnet
        payload,
        1000 as u64,
    );

    // let evm_rpc = candid::types::principal::Principal::from_text(evm_rpc).unwrap();
    let (cycles_result,): (Result<u128, RpcError>,) =
        ic_cdk::api::call::call(evm_rpc, "requestCost", params.clone())
            .await
            .unwrap();
    ic_cdk::println!("Cycles result is : {:?}", cycles_result);
    let cycles = cycles_result
        .unwrap_or_else(|e| ic_cdk::trap(&format!("error in `request_cost`: {:?}", e)));

    // Call with expected number of cycles
    let (result,): (Result<String, RpcError>,) =
        ic_cdk::api::call::call_with_payment128(evm_rpc, "request", params, cycles)
            .await
            .unwrap();
    match result {
        Ok(response) => {
            ic_cdk::println!("Response is : {:?}", response.to_string());
            return response.to_string();
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` with cycles: {:?}", err)),
    }
}

#[ic_cdk::update]
async fn read_binance_write_mumbai() -> Result<String, String> {
    let data_in_mumbai = get_data_from_source(97)
        .await
        .map_err(|_e| format!("Could not get any data from mumbai"))?;
    let data_result = &data_in_mumbai[34..&data_in_mumbai.len() - 2];
    let vector_data =
        eth_rpc_data::from_hex(data_result).map_err(|e| format!("Error in from_hex: {:?}", e))?;
    let tx_result = send_transaction(80001, vector_data)
        .await
        .map_err(|e| format!("Error in send_transaction: {:?}", e))?;

    return Ok(tx_result);
}

#[ic_cdk::update]
async fn read_mumbai_write_binance() -> Result<String, String> {
    let data_in_mumbai = get_data_from_source(80001)
        .await
        .map_err(|_e| format!("Could not get any data from mumbai"))?;
    let data_result = &data_in_mumbai[34..&data_in_mumbai.len() - 2];
    let vector_data =
        eth_rpc_data::from_hex(data_result).map_err(|e| format!("Error in from_hex: {:?}", e))?;
    let tx_result = send_transaction(97, vector_data)
        .await
        .map_err(|e| format!("Error in send_transaction: {:?}", e))?;

    return Ok(tx_result);
}
