# Create Rust Smart Contract

1. function that receives not a null array (Vec<u8>)
2. Calculate sha256 hash
3. Check is hash exist in the storage
    1. If exist - fail execution
    2. if not exist - save to storage
4. Return hash as String
5. Cover with unit tests
6. Cover with integration tests

## Tools

* near-cli
* near-sdk-rs
* near-sdk-sim
