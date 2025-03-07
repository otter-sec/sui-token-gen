//! Integration Tests for Sui Token Generator
//!
//! This module contains integration tests that verify:
//! - Token creation functionality
//! - Contract verification
//! - Input validation
//! - Error handling
//!
//! # Test Setup
//! Tests require a running RPC server and clean filesystem state.
//! Use `cargo test --test-threads 1` to run tests sequentially.

pub mod common;
pub mod error_handling_tests;
pub mod integration_tests;
pub mod other_tests;
pub mod rpc_connection_tests;
pub mod success_handler_tests;
pub mod token_command_tests;
pub mod validation_tests;
