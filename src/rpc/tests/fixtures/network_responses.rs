pub const MOCK_SUCCESS_RESPONSE: &str = r#"{
    "status": "success",
    "data": {
        "token_content": "module test::token {\n    use std::string;\n    use sui::coin::{Self, TreasuryCap};\n    use sui::transfer;\n    use sui::tx_context::{Self, TxContext};\n\n    struct TEST has drop {}\n\n    fun init(witness: TEST, ctx: &mut TxContext) {\n        let (treasury_cap, metadata) = coin::create_currency(\n            witness,\n            8,\n            b\"Test Token\",\n            b\"TST\",\n            b\"Test Description\",\n            option::none(),\n            ctx\n        );\n        transfer::public_freeze_object(metadata);\n        transfer::public_transfer(treasury_cap, tx_context::sender(ctx));\n    }\n}",
        "move_toml": "[package]\nname = \"Test Token\"\nversion = \"0.0.1\"\n\n[dependencies]\nSui = { git = \"https://github.com/MystenLabs/sui.git\", subdir = \"crates/sui-framework/packages/sui-framework\", rev = \"framework/devnet\" }"
    }
}"#;

pub const MOCK_ERROR_RESPONSE: &str = r#"{
    "status": "error",
    "message": "Invalid token parameters",
    "errors": {
        "decimals": "Must be between 0 and 9",
        "symbol": "Must be between 1 and 10 characters"
    }
}"#;
