# Baseline
[![MIT](https://img.shields.io/crates/l/baseline?style=for-the-badge)](https://choosealicense.com/licenses/lgpl-3.0/) [![Crates.io](https://img.shields.io/crates/v/baseline?style=for-the-badge)](https://crates.io/crates/baseline) [![docs.rs](https://img.shields.io/badge/docs.rs-baseline-rs?style=for-the-badge)](https://docs.rs/baseline)

## Overview
Baseline is a Rust crate providing common types and traits for dealing with financial data.
[SmolStr](https://crates.io/crates/smol_str) is used where appropriate for more efficient allocation.

The crate is in early stages and the API is **not** yet stable.

Current types include:
 - `security::Identifier { mic, symbol }`
 - `currency::Pair { base, quote }`

