# `fluxx`

Official Rust SDK for the [Fluxx](https://fluxx.dev) control plane API.

Targets stable Rust (MSRV 1.75).

## Install

```toml
[dependencies]
fluxx = "0.1"
```

## Quickstart

```rust
use fluxx::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("flx_live_…");
    // let fleet = client.fleet().list().await?;
    Ok(())
}
```

> **Status:** scaffold. The client surface is being generated from
> `https://api.fluxx.dev/openapi.json`.

## License

Apache-2.0
