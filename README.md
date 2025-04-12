# Solana Block Entries JSON-RPC Service

A lightweight JSON-RPC service that connects to a running Solana validator's ledger (RocksDB) and exposes an endpoint to retrieve block entries for a given slot.

## Features

- Reads from an existing Solana validator ledger (using secondary access)
- Exposes a `getBlockEntries` JSON-RPC method
- Returns block entries in the same format as Solana's BigTable schema

## JSON-RPC Method

### `getBlockEntries`

#### Request
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getBlockEntries",
  "params": [12345]
}
```

#### Response
```
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": [
    {
      "index": 0,
      "num_hashes": 1,
      "hash": "base58encodedhash",
      "num_transactions": 2,
      "starting_transaction_index": 0
    }
  ]
}
```

## Usage

### Build and Run (Locally)
```
cargo build --release
./target/release/solana-block-entries-service \
  --ledger-path /path/to/validator/ledger \
  --bind-addr 0.0.0.0 \
  --port 8080
```

### Docker
```
docker build -t solana-block-entries-service .

docker run --rm -p 8080:8080 \
  -v /path/to/validator/ledger:/solana/ledger \
  solana-block-entries-service \
  --ledger-path /solana/ledger
```

## Configuration

| Argument         | Environment Variable | Default                    | Description                         |
|------------------|----------------------|----------------------------|-------------------------------------|
| `--ledger-path`  | `LEDGER_PATH`        | `/solana/ledger` | Path to the Solana ledger directory |
| `--bind-addr`    | `BIND_ADDR`          | `0.0.0.0`                  | IP address to bind to               |
| `--port`         | `PORT`               | `8080`                     | HTTP server port                    |

> **Note:** The `ledger-path` should point to the top-level validator directory (e.g. `/solana/ledger`), **not** the `/rocksdb` subdirectory.

## License

MIT