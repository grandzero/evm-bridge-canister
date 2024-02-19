use candid::Principal;
pub mod ecdsa;
pub mod functions;
pub mod state;
pub mod transaction;
pub mod utils;
use primitive_types::U256;
// use ic_cdk::api::call::{call, CallResult};
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
