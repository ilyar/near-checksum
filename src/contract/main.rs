use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::{env, near_bindgen, setup_alloc, BorshStorageKey};

setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Hash,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Checksum {
    hash_set: LookupSet<Vec<u8>>,
}

impl Default for Checksum {
    fn default() -> Self {
        Self {
            hash_set: LookupSet::new(StorageKey::Hash),
        }
    }
}

#[near_bindgen]
impl Checksum {
    pub fn add(&mut self, data: Vec<u8>) -> String {
        let checksum = env::sha256(&data);
        assert!(
            !self.hash_set.contains(&checksum),
            "For given data checksum exist"
        );
        if !self.hash_set.insert(&checksum) {
            env::panic(b"Failed to save checksum");
        }
        checksum
            .iter()
            .map(|x| format!("{:x}", *x))
            .collect::<String>()
    }

    pub fn has(&self, hash: String) -> bool {
        if hash.len() != 64 {
            return false;
        }
        self.hash_set.contains(
            &(0..hash.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hash[i..i + 2], 16).unwrap_or_default())
                .collect(),
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod unit {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;

    fn context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(accounts(0));
        builder
    }

    #[test]
    fn add() {
        testing_env!(context().build());
        let mut contract = Checksum::default();
        let hash = String::from("2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae");
        assert_eq!(hash, contract.add([102, 111, 111].to_vec()));
        assert!(contract.has(hash));
    }

    #[test]
    #[should_panic(expected = "For given data checksum exist")]
    fn add_fail_on_exist() {
        testing_env!(context().build());
        let mut contract = Checksum::default();
        contract.add([98, 97, 114].to_vec());
        contract.add([98, 97, 114].to_vec());
    }

    #[test]
    #[ignore] // TODO setup context with empty storage
    #[should_panic(expected = "Failed to save checksum")]
    fn add_fail_on_storage() {
        testing_env!(context().build());
        let mut contract = Checksum::default();
        contract.add([98, 97, 114].to_vec());
    }

    #[test]
    fn has() {
        testing_env!(context().is_view(true).build());
        let contract = Checksum::default();
        assert!(!contract.has(String::from(
            "2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae"
        )));
        assert!(!contract.has(String::from("bad hash")));
    }
}
