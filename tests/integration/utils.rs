near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    CONTRACT_WASM_BYTES => "build/checksum.wasm",
}

use near_sdk_sim::init_simulator;
use near_sdk_sim::to_yocto;
use near_sdk_sim::UserAccount;
use near_sdk_sim::STORAGE_AMOUNT;

const CONTRACT_ID: &str = "contract";

pub fn fixture() -> (Vec<u8>, String) {
    (
        vec![102, 111, 111],
        String::from("2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"),
    )
}

pub fn init() -> (UserAccount, UserAccount, UserAccount) {
    // Use `None` for default genesis configuration; more info below
    let root = init_simulator(None);

    let contract = root.deploy(
        &CONTRACT_WASM_BYTES,
        CONTRACT_ID.to_string(),
        STORAGE_AMOUNT, // attached deposit
    );

    let alice = root.create_user(
        "alice".to_string(),
        to_yocto("100"), // initial balance
    );

    (root, contract, alice)
}
