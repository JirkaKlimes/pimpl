//! Probabilistic implementations for Rust.
//!
//! Mark a function with [`#[pimpl]`](macro@pimpl) and leave the body empty.
//! The spec — docstring, signature, tests — is the source of truth; an AI
//! agent generates a body, verifies it, and caches the result.

pub use pimpl_macros::pimpl;
