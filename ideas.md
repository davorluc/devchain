# Ideas for devchain

## Core blockchain ideas
[] Add digital signatures for transactions (e.g. Ed25519).
[] Move from a simple account balance model to a UTXO model.
[x] Add transaction fees and sort the mempool by fee rate.
[] Add a Merkle root to each block.
[] Add difficulty adjustment instead of a fixed proof-of-work target.
[] Add block rewards and halving rules.
[] Implement chain validation and block verification as proper public APIs.

## Rust learning ideas
[] Add unit tests for block hashing, chain validation, and mempool cleanup.
[] Add property-based tests for hash stability and transaction serialization.
[] Refactor persistence into a separate module with cleaner error handling.
[] Replace `panic!` paths with `Result`-based error types.
[] Add benchmarks for hashing and block creation.
[] Add a small CLI with commands like `mine`, `validate`, `show-chain`, and `send`.

## Portfolio[]friendly features
[] Build a simple blockchain explorer output in the terminal.
[] Add a network simulator with multiple nodes and fork resolution.
[] Save and load the chain from disk using a stable format.
[] Add colored logging and better debug output for mining steps.
[] Add a README with architecture diagrams and example runs.
[] Create a mini dashboard or web UI that shows chain height, mempool size, and latest block hash.

## Stretch goals
[] Simulate reorgs and competing chains.
[] Add wallet key generation and address creation.
[] Add peer-to-peer syncing between nodes.
[] Add a configurable difficulty and mining interval.
[] Add block and transaction serialization with versioning.
