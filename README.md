# Geyser gRPC Pubkey Limit Test

This project tests the pubkey limit functionality of Solana Geyser gRPC endpoints. It verifies whether a Solana node supports more than 50 pubkeys per subscription request.

## Purpose

The script tests if your Solana gRPC node enforces the 50 pubkey limit that was previously common in some implementations. It attempts to subscribe to:

- 56 pubkeys (to test if the node supports more than 50)
- Exactly 50 pubkeys
- Exactly 51 pubkeys

## Prerequisites

- Rust (latest stable version)
- A running Solana gRPC endpoint (default: `http://localhost:10000`)

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd geyser-grpc-test

# Build the project
cargo build
```

## Usage

1. **Update the endpoint** (if needed):
   Edit `src/main.rs` and change the endpoint URL:

   ```rust
   let endpoint = "http://your-node-address:port";
   ```

2. **Run the test**:

   ```bash
   cargo run
   ```

## Expected Output

If your node supports more than 50 pubkeys:

```
✓ Subscription successful! Your node supports more than 50 pubkeys per subscription.
✓ 50 pubkeys subscription successful - node supports at least 50 pubkeys.
✓ 51 pubkeys subscription successful - node supports more than 50 pubkeys.
```

If your node still enforces the 50 pubkey limit:

```
✗ Subscription failed with error: Max amount of Pubkeys reached, only 50 allowed
```

## Configuration

- **Default endpoint**: `http://localhost:10000`
- **Test pubkeys**: 56 real Solana program and token addresses
- **Commitment level**: Confirmed

## Dependencies

- `tokio` - Async runtime
- `tonic` - gRPC client
- `yellowstone-grpc-proto` - Solana Geyser protobuf definitions
- `yellowstone-grpc-client` - Geyser client library
- `solana-sdk` - Solana SDK for pubkey parsing
- `futures` - Async stream support

## Troubleshooting

- **Connection failed**: Ensure your Solana gRPC endpoint is running and accessible
- **Compilation errors**: Make sure you have Rust installed and all dependencies are available
- **Permission denied**: Check if the endpoint requires authentication

## License

[Add your license information here]
