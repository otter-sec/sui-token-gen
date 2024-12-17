# Sui Token CLI Tool

A **Rust-based CLI tool** for generating and verifying Sui token smart contracts effortlessly.

---

## Features
- **Create Command**: Interactively generates a Sui token contract with customizable parameters.
- **Verify Command**: Ensures the validity of Sui token contracts, either from a remote repository URL or a local file path.

---

## Commands
### 1. create
The create command prompts the user to enter the following parameters:

- Decimals: Number of decimal places for the token.
- Name: Name of the token.
- Symbol: Token symbol (e.g., "SUI").
- Description (optional): A brief description of the token.
- Is Frozen: Indicates whether the token is frozen at creation.
- Environment: Choose from devnet, testnet, or mainnet (devnet is default).

Upon successful execution, the command generates a Sui token smart contract code in the current project directory.

### usage:
```
cargo run create
```
### Output
```console
foo@bar:~$ cargo run create
? Decimals:  8
? Symbol:  SUI
? Name:  Sui token
? Description:  Fake sui token
? Frozen metadata? Yes
? Select environment: devnet
Creating contract...
SUCCESS: Contract has been generated!
Token Details:
  Name: Sui token
  Symbol: SUI
  Decimals: 8
  Environment: devnet
  Description: Fake sui token
  Frozen: Yes
foo@bar:~$ 
```
### 2. verify
The verify command checks whether a Sui token smart contract was generated using this CLI tool.

It supports:

Remote Repository: By providing a GitHub/GitLab repository URL.\
Local Path: By providing the file system path to the contract directory.\
If the provided contract is invalid or not generated by this tool, an error is thrown.

Flags:\
--url: Validate a remote repository.\
--path: Validate a local directory path.

### Example usage:
URL
```
cargo run verify --url https://github.com/meumar-osec/test-sui-token
```
### Output
```console
foo@bar:~$ cargo run verify --url https://github.com/meumar-osec/test-sui-token
foo@bar:~$ SUCCESS: Verified successfully from url: https://github.com/meumar-osec/test-sui-token
```
Path
```
cargo run verify --path /Users/developer/Desktop/sui/sui-token
```
### Output
```console
foo@bar:~$ cargo run verify --path /Users/developer/Desktop/sui/sui-token
foo@bar:~$ SUCCESS: Verified successfully from path: /Users/developer/Desktop/sui/sui-token
```
