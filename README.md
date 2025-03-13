# Sui Token CLI Tool

A **Rust-based Command-Line Interface (CLI) tool** designed to simplify the process of generating and verifying Sui token smart contracts. By default, it uses the RPC service provided by [Osec](http://5.161.90.244:5001). If you prefer using a different RPC endpoint, specify it using the `--rpc` flag.

---

## Installation

Install via [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install sui-token-gen
```

For additional documentation, visit the [sui-token-gen page on docs.rs](https://docs.rs/crate/sui-token-gen/latest).

### Features & Commands

1. **Create (`sui-token-gen create`)**  
   - Interactively generate a Sui token contract by specifying:
     - Decimals
     - Name
     - Symbol
     - Description (optional)
     - Is Frozen
     - Environment (devnet by default)
   - Produces the smart contract, a TOML configuration file, and a test file in the current directory.
   - Example:

     ```console
     ? Name:  Sui token
     ? Symbol:  SUI
     ? Decimals:  8
     ? Description:  Fake sui token
     ? Frozen metadata? Yes
     ? Select environment: devnet
     Creating contract...
     SUCCESS: Contract has been generated at: /foo/bar/suitoken
     Token Details:
       Name: Sui token
       Symbol: SUI
       Decimals: 8
       Description: Fake sui token
       Frozen: Yes
       Environment: devnet
     ```

   - **Tip**: Add `--rpc <URL>` to override the default RPC endpoint.

2. **Verify (`sui-token-gen verify`)**  
   - Checks if a Sui token contract was created by this tool.
   - Provide exactly one of the following flags:
     - `--url` : Remote Git repository URL (e.g., GitHub, GitLab)
     - `--path`: Local directory path
     - `--address`: Deployed contract address
   - **Example**:

     ```bash
     # Verify by URL
     sui-token-gen verify --url https://github.com/meumar-osec/test-sui-token
     
     # Verify by local path
     sui-token-gen verify --path /Users/developer/Desktop/sui/sui-token
     
     # Verify by address
     sui-token-gen verify --address 0x1234abcd5678ef90
     ```

   - If the token is invalid or not generated by this tool, an error is thrown.

## REST APIs

For direct interaction with the underlying RPC service, refer to [http://5.161.90.244:5001](http://5.161.90.244:5001/). Comprehensive parameter and response details can be found at that endpoint.
