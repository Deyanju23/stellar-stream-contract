# StellarStream Contract

A continuous linear money-streaming protocol on Stellar, built with Soroban smart contracts.

## Overview

StellarStream enables real-time, continuous payment streams on the Stellar network. Funds are locked in a smart contract and linearly distributed to a recipient over a defined time window.

## Features

- **Linear Streaming**: Funds are distributed continuously over time using exact integer arithmetic.
- **Withdraw Anytime**: Recipients can withdraw accrued funds at any point during the stream.
- **Cancellable Streams**: Senders can optionally make streams cancellable, allowing early termination with fair distribution.
- **No Precision Loss**: All calculations use fixed-point integer arithmetic — no floating-point operations.

## Building

```bash
make build
```

## Testing

```bash
make test
```

## Linting

```bash
make clippy
```

## Architecture

```
contracts/
└── stream/         # Core streaming contract
    └── src/
        ├── lib.rs      # Contract interface and implementation
        ├── types.rs    # Data structures and storage keys
        ├── error.rs    # Contract error definitions
        ├── events.rs   # Event emission helpers
        ├── storage.rs  # Storage access and TTL management
        ├── admin.rs    # Admin initialization logic
        └── test.rs     # Comprehensive unit tests
```

## License

MIT
