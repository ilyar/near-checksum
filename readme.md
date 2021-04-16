<div align="center">
  <h1><code>near-checksum</code></h1>
  <p>
    <strong>Implementation of a checksum any data</strong>
  </p>
</div>

## Develop

```shell
make fmt
make qa
make build
```

## Note init

https://docs.near.org/docs/tutorials/contracts/intro-to-rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
mkdir near-checksum  && cd $_
git init
cargo init --edition 2018 --lib --vcs git
```
