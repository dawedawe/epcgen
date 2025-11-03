//! # epcgen
//!
//! A Rust library for type-safe creation of EPC (European Payments Council) QR code payloads.
//!
//! This library provides the payload string for EPC QR codes, which can be used with any
//! QR code rendering library. It supports SEPA credit transfer data encoding according to
//! the EPC Quick Response Code standard.
//!
//! ## Features
//!
//! - Type-safe builder pattern for EPC QR codes
//! - IBAN validation according to ISO 13616
//! - RF creditor reference validation according to ISO 11649
//! - Support for both structured and unstructured remittance information
//! - Minimal dependencies
//!
//! ## Example
//!
//! ```
//! use epcgen::{CharacterSet, Epc, Identification, Remittance, Version};
//!
//! let epc = Epc::builder()
//!     .version(Version::V2)
//!     .character_set(CharacterSet::UTF8)
//!     .identification(Identification::Sct)
//!     .beneficiary("John Doe")
//!     .iban("DE90 8306 5408 0004 1042 42")
//!     .amount("10.00")
//!     .remittance(Remittance::Text("Invoice 12345".to_string()))
//!     .build()
//!     .expect("Valid EPC data");
//!
//! let qr_payload = epc.to_string();
//! // Use qr_payload with your preferred QR code library
//! ```
//!
mod ibanrf;
pub use ibanrf::iban;
pub use ibanrf::rf;
mod epcgen;
pub use epcgen::*;
