use crate::utils::fixture;
use crate::utils::init;
use near_sdk::serde_json::json;
use near_sdk_sim::DEFAULT_GAS;

#[test]
fn default() {
    let (root, contract, _alice) = init();
    let (data, hash) = fixture();

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
    assert_eq!(hash, result.unwrap_json::<String>());

    let actual: bool = root
        .view(
            contract.account_id(),
            "has",
            &json!({
                "hash": hash,
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();
    assert_eq!(true, actual);
}
