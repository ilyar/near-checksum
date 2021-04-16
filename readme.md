<div align="center">
  <h1><code>near-checksum</code></h1>
  <p>
    <strong>Implementation of a checksum any data on NEAR Protocol</strong>
  </p>
</div>

## Develop

```shell
make fmt
make qa
make build
make clean
```

## Deploy stage

```shell
make deploy-force
accountId=ilyar.testnet
contractName=$(cat neardev/dev-account)
near state $contractName
near call $contractName new --accountId $accountId
near delete $contractName $accountId
```

## Deploy testnet

```shell
make qa
make build
accountId=ilyar.testnet
contractName="checksum.$accountId"
near create-account --masterAccount $accountId $contractName 
near deploy --wasmFile build/checksum.wasm --accountId $contractName --initFunction new --initArgs '{}'
```

## Usage

```shell
near view $contractName has '{"hash": "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2"}'
near call $contractName add '{"data": [102, 111, 111, 98, 97, 114]}' --accountId $accountId
near view $contractName has '{"hash": "c3ab8ff13720e8ad9047dd39466b3c8974e592c2fa383d4a3960714caef0c4f2"}'
```

## Note init

https://docs.near.org/docs/tutorials/contracts/intro-to-rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
mkdir near-checksum  && cd $_
git init
cargo init --edition 2018 --lib --vcs git
```
