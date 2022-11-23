<h1 align="center">Baseline</h1>
<p align="center">
	<a href="https://crates.io/crates/baseline">
		<img src="https://img.shields.io/crates/v/baseline" />
    </a>
	<a href="https://docs.rs/baseline">
		<img src="https://img.shields.io/badge/docs.rs-baseline-rs" />
    </a>
	<img src="https://img.shields.io/crates/l/baseline" />
</p>

## Overview
Baseline is a Rust crate providing common types and traits for dealing with financial data.
[SmolStr](https://crates.io/crates/smol_str) is used where appropriate for more efficient allocation.

The crate is in early stages and the API is **not** yet stable.

Current types include:
 - `struct security::Identifier { mic, symbol }`
 - `struct currency::Pair { base, quote }`
 - `trait bar::OHLC + bar::Volume`
 - `struct OrderBook<O: Order>`
 - `trait order_book::Order`

