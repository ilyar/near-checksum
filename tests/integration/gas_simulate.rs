use crate::utils::{init, to_gas};
use near_sdk::serde_json::json;
use near_sdk_sim::{to_yocto, DEFAULT_GAS};
use rand::Rng;

#[test]
fn default() {
    let (root, contract, _alice) = init();
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..u32::pow(10, 4)).map(|_| rng.gen()).collect(); // of size 10Kb

    let result = root.call(
        contract.account_id(),
        "add",
        &json!({
            "data": data,
        })
        .to_string()
        .into_bytes(),
        DEFAULT_GAS,
        0, // deposit
    );
    // 10b   tokens: 0.0003Ⓝ gas:   3.1TeraGas Success
    // 100b  tokens: 0.0003Ⓝ gas:   3.5TeraGas Success
    // 1Kb   tokens: 0.0007Ⓝ gas:   7.1TeraGas Success
    // 10Kb  tokens: 0.0043Ⓝ gas:  43.9TeraGas Success
    // 100Kb tokens: 0.0203Ⓝ gas: 203.2TeraGas Failure
    println!(
        "burnt {:.05}Ⓝ {:.02}TeraGas",
        (result.tokens_burnt()) as f64 / 1e24,
        (result.gas_burnt()) as f64 / 1e12,
    );
    assert!(result.gas_burnt() <= to_gas("43.995"));
    assert!(result.tokens_burnt() <= to_yocto("0.00445"));
}
