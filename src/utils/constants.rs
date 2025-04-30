// The RPC server address where the client can connect to.
// The format is [IP address]:port
pub const ADDRESS: &str = "5.161.90.244:5001";

// The error message used when the user cancels an ongoing operation.
// This message is displayed to indicate that the action was aborted by the user.
pub const CANCEL_ERROR_MESSAGE: &str = "Operation was canceled by the user";

// Options for frozen metadata. Represents a choice between two states:
// "Yes" indicates the metadata is frozen, "No" indicates it is not.
pub const FROZEN_OPTIONS: [&str; 2] = ["Yes", "No"];

// The name of the folder where source files are stored.
// This folder is expected to contain the files needed for token generation or other operations.
pub const SUB_FOLDER: &str = "sources";

// The name of the folder used for test files.
// This folder is likely used for unit or integration tests.
pub const TEST_FOLDER: &str = "tests";

// Constant for default environment
pub const DEFAULT_ENVIRONMENT: &str = "devnet";
