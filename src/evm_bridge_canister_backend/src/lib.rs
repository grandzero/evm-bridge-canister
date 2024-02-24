///0x1d5989e6f450BBd9385Ce98922C0218af4c9aE97 => Mumbai
// 0x4FF62fC53b4fCD08428aE79a59B36A5Ba9235817 => Binance
// 0x7BFD4157F0Bbf52d96e278504275a12FB01529Cc => Goerli
// bd3sg-teaaa-aaaaa-qaaba-cai rpc evm canister
pub mod ecdsa;
pub mod eth_rpc_data;
pub mod functions;
pub mod state;
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
use state::ChainSelection;
use state::RPCEndpoints;
use state::STATE;
#[ic_cdk::init]
async fn init(endpoints: Vec<RPCEndpoints>, rpc_canister: String) {
    STATE.with(|s| {
        // TODO: Remove hardcoded initiaalization values
        let mut state = s.borrow_mut();
        state.config.owner = ic_cdk::caller();
        state.config.rpc_url =
            "https://polygon-mumbai.g.alchemy.com/v2/4_fdLBh3p_OwpbYRHzua1BFsJFI4-eNr".to_string();
        state.config.rpc_endpoints = endpoints;
        state.config.rpc_canister = rpc_canister;
        state.config.mumbai_contract = "0x1d5989e6f450BBd9385Ce98922C0218af4c9aE97".to_string();
        state.config.binance_contract = "0x4FF62fC53b4fCD08428aE79a59B36A5Ba9235817".to_string();
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
pub async fn get_gas_price() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    let evm_rpc =
        candid::types::principal::Principal::from_text(state.config.rpc_canister).unwrap();
    let rpc_url = state.config.rpc_url;

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
            // Check response structure around gas price
            // assert_eq!(
            //     &response[..36],
            //     "{\"id\":1,\"jsonrpc\":\"2.0\",\"result\":\"0x"
            // );
            // assert_eq!(&response[response.len() - 2..], "\"}");
            //let _transaction_hash = &response[36..response.len()];

            return response.to_string();
        }
        Err(err) => ic_cdk::trap(&format!("error in `request` with cycles: {:?}", err)),
    }
}

#[ic_cdk::update]
async fn create_adress_for_owner() -> String {
    let principal_id = ic_cdk::caller();
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
async fn send_transaction_from_rpc(
    signed_data: String,
    chain: ChainSelection,
    evm_rpc: String,
) -> String {
    let state = STATE.with(|s| s.borrow().clone());
    // TODO: Remove hardcoded chain selection, remove copy paste code
    let rpc_url = match chain {
        ChainSelection::Mumbai => state.config.rpc_endpoints[0].rpc_url.clone(),
        ChainSelection::Binance => state.config.rpc_endpoints[1].rpc_url.clone(),
    };
    ic_cdk::println!("RPC URL in send is : {}", rpc_url);
    let data =  format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_sendRawTransaction\",\"params\":[\"{}\"],\"id\":1}}",  signed_data);
    ic_cdk::println!("Data is : {}", data);

    let params = (
        &RpcService::Custom(RpcApi {
            url: rpc_url,
            headers: None,
        }),
        data, // Ethereum mainnet
        1000 as u64,
    );

    let evm_rpc = candid::Principal::from_text(evm_rpc).unwrap();
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

async fn send_view_rpc_request(
    data: String,
    contract_address: String,
    chain: ChainSelection,
) -> String {
    // let state = STATE.with(|s| s.borrow().clone());
    let state = STATE.with(|s| s.borrow().clone());
    ic_cdk::println!("EVM RPC is : {}", state.config.rpc_canister);
    let evm_rpc =
        candid::types::principal::Principal::from_text(state.config.rpc_canister.clone()).unwrap();

    let rpc_url = match chain {
        ChainSelection::Mumbai => state.config.rpc_endpoints[0].rpc_url.clone(),
        ChainSelection::Binance => state.config.rpc_endpoints[1].rpc_url.clone(),
    };
    ic_cdk::println!("RPC URL is : {}", rpc_url);
    let params = (
        &RpcService::Custom(RpcApi {
            url: rpc_url,
            headers: None,
        }), // Ethereum mainnet
        format!("{{\"jsonrpc\":\"2.0\",\"method\":\"eth_call\",\"params\":[{{ \"to\":\"{}\", \"data\":\"{}\"}}, \"latest\"],\"id\":1}}", contract_address, data),
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
async fn send_transaction(chain: ChainSelection, message: Vec<u8>) -> Result<String, String> {
    let mut state = STATE.with(|s| s.borrow_mut().clone());
    let chain_id = match chain {
        ChainSelection::Mumbai => 80001,
        ChainSelection::Binance => 97,
    };
    let data_to_sign = eth_rpc_data::get_data_with_arguments("icpCall", &[Token::Bytes(message)])
        .await
        .map_err(|e| format!("Error in get_data_with_arguments: {:?}", e))?;
    ic_cdk::println!(
        "Data to sign is : {:?}",
        eth_rpc_data::to_hex(&data_to_sign)
    );
    let signed_tx = functions::sign_custom_tx(eth_rpc_data::to_hex(&data_to_sign), chain_id).await;
    let tx_result = send_transaction_from_rpc(
        eth_rpc_data::to_hex(&signed_tx.unwrap().sign_tx),
        chain,
        state.config.rpc_canister,
    )
    .await;
    state.nonce = state.nonce + 1;
    return Ok(tx_result);
}

#[ic_cdk::update]
async fn get_data_from_source(
    chain: ChainSelection,
    contract_address: String,
) -> Result<String, String> {
    let data = eth_rpc_data::get_data_with_arguments("getMessage", &[Token::Uint(U256::from(0))])
        .await
        .map_err(|e| format!("Error in get_data_with_arguments: {:?}", e))?;
    ic_cdk::println!("Data is : {:?}", data);
    let request_data = eth_rpc_data::to_hex(&data);
    let rpc_result = send_view_rpc_request(request_data, contract_address, chain).await;
    return Ok(rpc_result);
}

#[ic_cdk::update]
async fn read_binance_write_mumbai(contract_address: String) -> Result<String, String> {
    let data_in_mumbai = get_data_from_source(ChainSelection::Binance, contract_address.clone())
        .await
        .map_err(|e| format!("Could not get any data from mumbai"))?;
    let data_result = &data_in_mumbai[34..&data_in_mumbai.len() - 2];
    let vector_data =
        eth_rpc_data::from_hex(data_result).map_err(|e| format!("Error in from_hex: {:?}", e))?;
    let tx_result = send_transaction(ChainSelection::Mumbai, vector_data)
        .await
        .map_err(|e| format!("Error in send_transaction: {:?}", e))?;

    return Ok(tx_result);
}

// #[ic_cdk::update]
// async fn read_mumbai_write_binance(contract_address: String) -> Result<String, String> {
//     let data_in_mumbai = get_data_from_source(ChainSelection::Mumbai, contract_address.clone())
//         .await
//         .map_err(|e| format!("Could not get any data from mumbai"))?;
//     let data_result = &data_in_mumbai[34..&data_in_mumbai.len() - 2];
//     let vector_data =
//         eth_rpc_data::from_hex(data_result).map_err(|e| format!("Error in from_hex: {:?}", e))?;
//     let tx_result = send_transaction(ChainSelection::Binance, vector_data)
//         .await
//         .map_err(|e| format!("Error in send_transaction: {:?}", e))?;

//     return Ok(tx_result);
// }
//https://eth-sepolia.g.alchemy.com/v2/9_z81E1WLPhEG17yVTfdmBuOUrnh-14C
