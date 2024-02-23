///0x98f37cF2fB4B66Fe7b509F8d09Daa7cc6F5CD6A2
// bd3sg-teaaa-aaaaa-qaaba-cai rpc evm canister
pub mod ecdsa;
pub mod eth_rpc_data;
pub mod functions;
pub mod state;
pub mod transaction;
pub mod utils;
// use ic_cdk::api::call::{call, CallResult};
use cketh_common::eth_rpc::ProviderError;
use cketh_common::eth_rpc::RpcError;
use cketh_common::eth_rpc_client::providers::RpcApi;
use cketh_common::eth_rpc_client::providers::RpcService;

use state::STATE;
#[ic_cdk::init]
async fn init(rpc_url: String, rpc_canister: String) {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.config.owner = ic_cdk::caller();
        state.config.rpc_url = rpc_url;
        state.config.rpc_canister =
            candid::types::principal::Principal::from_text(rpc_canister).unwrap();
    });
    // let _addr =
    //     functions::create_address(Principal::from_text(ic_cdk::caller().to_text()).unwrap()).await;
}

#[ic_cdk::update]
pub async fn get_gas_price(evm_rpc: String) -> String {
    let params = (
        &RpcService::Custom(RpcApi {
            url: "https://polygon-mumbai-pokt.nodies.app".to_string(),
            headers: None,
        }), // Ethereum mainnet
        "{\"jsonrpc\":\"2.0\",\"method\":\"eth_gasPrice\",\"params\":null,\"id\":80001}"
            .to_string(),
        1000 as u64,
    );
    // let canister_principal = Principal::from_text("br5f7-7uaaa-aaaaa-qaaca-cai").unwrap();
    let evm_rpc = candid::types::principal::Principal::from_text(evm_rpc).unwrap();
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
            // Check response structure around gas price
            // assert_eq!(
            //     &response[..36],
            //     "{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":\"0x"
            // );
            // assert_eq!(&response[response.len() - 2..], "\"}");
            return response.to_string();
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` with cycles: {:?}", err)),
    }
}

#[ic_cdk::update]
async fn create_adress_for_owner() -> String {
    let principal_id = ic_cdk::caller();
    let state = STATE.with(|s| s.borrow().clone());
    if state.config.owner != principal_id {
        return "Only owner can call".to_string();
    }

    let addr = functions::create_address(state.config.owner).await;
    if let Ok(res) = addr {
        STATE.with(|s| {
            let mut state = s.borrow_mut();
            // state.config.owner_public_key = res.address.to_bytes().clone();
            state.config.public_key_str = res.address.clone();
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
    let addr = state.config.public_key_str.clone();
    return addr;
}

#[ic_cdk::update]
async fn sign_example_tx() -> String {
    // let state = STATE.with(|s| s.borrow().clone());
    // let principal_id = ic_cdk::caller();
    // ic_cdk::print(&state.owner);
    let res = functions::sign_custom_tx("0x".to_owned(), 0).await;
    if let Ok(res) = res {
        return format!("{:?}", res);
    }

    return "Error".to_owned();
}

async fn send_transaction_from_rpc(
    signed_data: String,
    contract_address: String,
    evm_rpc: String,
) -> String {
    let state = STATE.with(|s| s.borrow().clone());
    // ic_cdk::println!("Data is : {}", signed_data);
    // ic_cdk::println!("Contract address is : {}", contract_address);
    // ic_cdk::println!("EVM RPC is : {}", evm_rpc);
    // ic_cdk::println!("Account : {}", state.config.public_key_str);

    let data =  format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\",\"params\":[\"{}\"],\"id\":1}}",  signed_data);
    ic_cdk::println!("Data is : {}", data);

    let params = (
        &RpcService::Custom(RpcApi {
            url: "https://polygon-mumbai-pokt.nodies.app".to_string(),
            headers: None,
        }),
        data, // Ethereum mainnet
        //   format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRaw\",\"params\":[{{\"from\":\"{}\", \"to\":\"{}\", \"data\":\"{}\"}}, \"latest\"],\"id\":1}}", state.config.public_key_str,contract_address, signed_data),
        1000 as u64,
    );

    let evm_rpc = candid::types::principal::Principal::from_text(evm_rpc).unwrap();
    let (cycles_result,): (Result<u128, RpcError>,) =
        ic_cdk::api::call::call(evm_rpc, "requestCost", params.clone())
            .await
            .unwrap();
    let cycles = cycles_result
        .unwrap_or_else(|e| ic_cdk::trap(&format!("error in `request_cost`: {:?}", e)));

    // Call with expected number of cycles
    let (result,): (Result<String, RpcError>,) =
        ic_cdk::api::call::call_with_payment128(evm_rpc, "request", params, cycles)
            .await
            .unwrap();
    match result {
        Ok(response) => {
            return response.to_string();
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` with cycles: {:?}", err)),
    }
}

async fn send_rpc_request(data: String, contract_address: String, evm_rpc: String) -> String {
    // let state = STATE.with(|s| s.borrow().clone());
    ic_cdk::println!("Data is : {}", data);
    ic_cdk::println!("Contract address is : {}", contract_address);
    ic_cdk::println!("EVM RPC is : {}", evm_rpc);
    ic_cdk::println!("RPC call data : {}", format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[{{\"from\":\"null\", \"to\":\"{}\", \"data\":\"{}\"}}, \"latest\"],\"id\":1}}", contract_address, data));
    let params = (
        &RpcService::Custom(RpcApi {
            url: "https://polygon-mumbai-pokt.nodies.app".to_string(),
            headers: None,
        }), // Ethereum mainnet
        format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[{{ \"to\":\"{}\", \"data\":\"{}\"}}, \"latest\"],\"id\":1}}", contract_address, data),
        1000 as u64,
    );

    let evm_rpc = candid::types::principal::Principal::from_text(evm_rpc).unwrap();
    let (cycles_result,): (Result<u128, RpcError>,) =
        ic_cdk::api::call::call(evm_rpc, "requestCost", params.clone())
            .await
            .unwrap();
    let cycles = cycles_result
        .unwrap_or_else(|e| ic_cdk::trap(&format!("error in `request_cost`: {:?}", e)));

    // Call with expected number of cycles
    let (result,): (Result<String, RpcError>,) =
        ic_cdk::api::call::call_with_payment128(evm_rpc, "request", params, cycles)
            .await
            .unwrap();
    match result {
        Ok(response) => {
            return response.to_string();
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` with cycles: {:?}", err)),
    }
}
#[ic_cdk::update]
async fn get_data_from_contract(contract_address: String, evm_rpc: String) -> String {
    // let state = STATE.with(|s| s.borrow().clone());

    let index = 0;
    let data = eth_rpc_data::get_data(contract_address.clone(), index).await;
    if let Ok(res) = data {
        let request_data = eth_rpc_data::to_hex(&res);
        ic_cdk::print(&request_data);
        let rpc_result = send_rpc_request(request_data, contract_address, evm_rpc).await;
        return rpc_result;
    }

    return "Error".to_owned();
}

#[ic_cdk::update]
async fn send_tx() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    // let principal_id = ic_cdk::caller();
    // ic_cdk::print(&state.owner);
    let data_to_sign = eth_rpc_data::prepare_data_to_sign("Hello".to_owned()).await;
    if let Ok(data) = data_to_sign {
        let hex_data = eth_rpc_data::to_hex(&data);
        ic_cdk::println!("Data is : {}", hex_data);
        let res = functions::sign_custom_tx(hex_data, state.nonce).await;
        if let Ok(res) = res {
            //return format!("{:?}", res.sign_tx);
            ic_cdk::println!("{:?}", res.sign_tx);
            let tx_result = send_transaction_from_rpc(
                eth_rpc_data::to_hex(&res.sign_tx),
                "0x98f37cF2fB4B66Fe7b509F8d09Daa7cc6F5CD6A2".to_owned(),
                "bd3sg-teaaa-aaaaa-qaaba-cai".to_owned(),
            )
            .await;
            return tx_result;
        }
        if let Err(e) = res {
            return format!("{:?}", e);
        }
    }

    // let res = functions::sign_custom_tx("0x".to_owned(), state.nonce).await;
    // if let Ok(res) = res {
    //     return format!("{:?}", res);
    // }
    // if let Err(e) = res {
    //     return format!("{:?}", e);
    // }
    return "Completed".to_owned();
}
