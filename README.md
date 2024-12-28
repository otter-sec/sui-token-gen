# Sui Token CLI Tool

A **Rust-based CLI tool** for generating and verifying Sui token smart contracts effortlessly.

---

## Features
- **Create Command**: Interactively generates a Sui token contract with customizable parameters.
- **Verify Command**: Ensures the validity of Sui token contracts, either from a remote repository URL or a local file path.

---

## Commands
### 1. `create`
The `create` command prompts the user to enter the following parameters:

- Decimals: Number of decimal places for the token.
- Name: Name of the token.
- Symbol: Token symbol (e.g., "SUI").
- Description (optional): A brief description of the token.
- Is Frozen: Indicates whether the token is frozen at creation.
- Environment: Choose from devnet, testnet, or mainnet (devnet is default).

Internally it calls RPC create function. Upon successful execution, the function returns the following:
- **Sui token smart contract code**.
- **TOML configuration file code**.
- **Test code**.

Based on these values the contract will be generated in the current project directory.

### usage:
```bash
sui-token-gen create
```
### Output
```console
foo@bar:~$ sui-token-gen create
? Decimals:  8
? Symbol:  SUI
? Name:  Sui token
? Description:  Fake sui token
? Frozen metadata? Yes
? Select environment: devnet
Creating contract...
SUCCESS: Contract has been generated at: /foo/bar/suitoken
Token Details:
  Name: Sui token
  Symbol: SUI
  Decimals: 8
  Environment: devnet
  Description: Fake sui token
  Frozen: Yes
foo@bar:~$ 
```
The optional --rpc flag allows specifying a custom RPC URL for the token creation process. By default, the tool will use the default RPC server URL if the --rpc flag is not provided.

### 2. `verify`
The `verify` command checks whether a Sui token smart contract was generated using this CLI tool.

Flags:\
`--url`: Validate a remote repository.\
`--path`: Validate a local directory path.\
`--rpc`: Specify a custom RPC URL for verification (optional).

It supports:

Internally, the RPC is called for verification:
- **Remote Repository: By providing a GitHub/GitLab repository URL, the `verify_url` RPC function is called with the URL**.
- **Local Path: By providing the file system path to the contract directory, the contract file is read, and the `verify_content` RPC function is called with the contract content**.

If the provided contract is invalid or was not generated by this tool, an error is thrown.


### Example usage:
URL
```bash
sui-token-gen verify --url https://github.com/meumar-osec/test-sui-token
```
### Output
```console
foo@bar:~$ sui-token-gen verify --url https://github.com/meumar-osec/test-sui-token
foo@bar:~$ SUCCESS: Verified successfully from url: https://github.com/meumar-osec/test-sui-token
```
Path
```bash
sui-token-gen verify --path /Users/developer/Desktop/sui/sui-token
```
### Output
```console
foo@bar:~$ sui-token-gen verify --path /Users/developer/Desktop/sui/sui-token
foo@bar:~$ SUCCESS: Verified successfully from path: /Users/developer/Desktop/sui/sui-token
```
---

## REST APIs

### 1. Create token API:

- **URL**: `/create`
- **Method**: `POST`
- **Usage**:
  ```bash
  curl -X POST -H "Content-Type: application/json" \
  -d '{
    "decimals": 1,
    "name": "My Token",
    "symbol": "MTK",
    "description": "Test token",
    "is_frozen": false,
    "environment": "devnet"
  }' http://5.161.90.244:5001/create
  ```
- **Request Body**:
  ```json
    {
      "decimals": 8,
      "name": "MyToken",
      "symbol": "MTK",
      "description": "A custom token.",
      "is_frozen": false,
      "environment": "devnet"
    }
  ```
- **Response**:
  ```json
    {
      "success": true,                  
      "message": "Creation successful",
      "data": {
        "token": "contract...",               
        "move_toml": "move toml...",       
        "test_token": "contract test..."     
      }
    }
  ```

### 2. Verifying URL(repo) API:

- **URL**: `/verify_url`
- **Method**: `POST`
- **Usage**:
  ```bash
  curl -X POST -H "Content-Type: application/json" \
  -d '{"url": "https://github.com/meumar-osec/test-sui-token"}' http://5.161.90.244:5001/verify_url
  ```
- **Request Body**:
  ```json
    {
      "url": "https://github.com/meumar-osec/test-sui-token",
    }
  ```
- **Response**:
  ```json
    {
      "success": true,                  
      "message": "Verified successfully",
      "error": None
    }
  ```

### 3. Verifying content API:

- **URL**: `/verify_content`
- **Method**: `POST`
- **Usage**:
  ```bash
    curl -X POST -H "Content-Type: application/json" \
    -d '{
        "content": "module Mytoken::Mytoken {\n    use sui::coin::{Self, TreasuryCap};\n    public struct MYTOKEN has drop {}\n\n    /// Initialize the token with treasury and metadata\n    fun init(witness: MYTOKEN, ctx: &mut TxContext) {\n        let (treasury, metadata) = coin::create_currency(\n            witness, 8, b\"MT\", b\"My token\", b\"Tetsing\", option::none(), ctx\n        );\n        \n        transfer::public_freeze_object(metadata);\n        \n        transfer::public_transfer(treasury, ctx.sender());\n    }\n\n    public fun mint(\n\t\ttreasury_cap: &mut TreasuryCap<MYTOKEN>,\n\t\tamount: u64,\n\t\trecipient: address,\n\t\tctx: &mut TxContext,\n    ) {\n        let coin = coin::mint(treasury_cap, amount, ctx);\n        transfer::public_transfer(coin, recipient)\n    }\n}"
    }' http://5.161.90.244:5001/verify_content
  ```
- **Request Body**:
  ```json
    {
      "content": "module Mytoken::Mytoken ...",
    }
  ```
- **Response**:
  ```json
    {
      "success": true,                  
      "message": "Verified successfully",
      "error": None
    }
  ```
---