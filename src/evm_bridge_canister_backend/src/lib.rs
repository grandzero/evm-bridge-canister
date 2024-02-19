use candid::Principal;

pub mod ecdsa;
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
async fn init() {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.owner = ic_cdk::caller().to_text();
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
        "{\"jsonrpc\":\"2.0\",\"method\":\"eth_gasPrice\",\"params\":null,\"id\":1}".to_string(),
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
async fn greet(_name: String) -> String {
    let principal_id = ic_cdk::caller();
    let principal_id = Principal::from_text(principal_id.to_text()).unwrap();
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        state.owner = ic_cdk::caller().to_text();
    });
    let addr = functions::create_address(principal_id).await;
    if let Ok(res) = addr {
        return res.address;
    }
    // let (first_result, second_result): (first_result_type, second_result_type) =
    //     ic_cdkapi::call::call(canister_id, "method", (first_arg, second_arg)).await?;

    return "Error".to_owned();
}
#[ic_cdk::update]
async fn get_canister_address() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    ic_cdk::print(&state.owner);
    let principal_id = Principal::from_text(state.owner).unwrap();
    let addr = functions::get_address(principal_id).await;
    return addr;
}

#[ic_cdk::update]
async fn sign_example_tx() -> String {
    let state = STATE.with(|s| s.borrow().clone());
    // let principal_id = ic_cdk::caller();
    ic_cdk::print(&state.owner);
    let res = functions::sign_custom_tx("0x".to_owned()).await;
    if let Ok(res) = res {
        return format!("{:?}", res);
    }

    return "Error".to_owned();
}
