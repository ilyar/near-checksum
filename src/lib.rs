use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::{env, near_bindgen, setup_alloc, BorshStorageKey};
setup_alloc!();

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Checksum,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Checksum {
    checksum_set: LookupSet<Vec<u8>>,
}

impl Default for Checksum {
    fn default() -> Self {
        Self {
            checksum_set: LookupSet::new(StorageKey::Checksum),
        }
    }
}

#[near_bindgen]
impl Checksum {
    pub fn add(&mut self, data: Vec<u8>) -> bool {
        let checksum = env::sha256(&data);
        if self.checksum_set.contains(&checksum) {
            env::panic(b"For given data checksum exist")
        }
        self.checksum_set.insert(&checksum)
    }

    pub fn contains(&self, checksum: String) -> bool {
        self.checksum_set.contains(&checksum.as_bytes().to_vec())
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use std::convert::TryInto;

    fn context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("alice".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn add() {
        let context = context(false);
        testing_env!(context);
        let mut contract = Checksum::default();
        assert!(contract.add([102, 111, 111].to_vec()));
        assert!(!contract.contains(String::from(
            "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"
        )));
    }

    #[test]
    #[should_panic(expected = "For given data checksum exist")]
    fn add_fail() {
        testing_env!(context(false));
        let mut contract = Checksum::default();
        assert!(contract.add([98, 97, 114].to_vec()));
        assert!(contract.add([98, 97, 114].to_vec()));
    }

    #[test]
    fn contains() {
        testing_env!(context(true));
        let contract = Checksum::default();
        assert!(!contract.contains(String::from(
            "b5bb9d8014a0f9b1d61e21e796d78dccdf1352f23cd32812f4850b878ae4944c"
        )));
    }
}
