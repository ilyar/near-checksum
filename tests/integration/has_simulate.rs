use crate::utils::init;
use near_sdk::serde_json::json;

#[test]
fn default() {
    let (root, contract, _alice) = init();

    let actual: bool = root
        .view(
            contract.account_id(),
            "has",
            &json!({
                "hash": "foo".to_string(),
            })
            .to_string()
            .into_bytes(),
        )
        .unwrap_json();

    assert_eq!(false, actual);
}
