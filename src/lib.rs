//! Type-safe creation of EPC QR codes.
//!
//! # Examples
//! ```
//! use epcgen::{CharacterSet, Epc, Identification, Version};
//! use image::Luma;
//! use qrcode::QrCode;
//!
//! let iban = "DE90 8306 5408 0004 1042 42";
//! assert!(epcgen::iban::is_valid(iban));
//! let unused_ref = "RF45G72UUR";
//! assert!(epcgen::rf::is_valid(unused_ref));
//! let builder = Epc::builder()
//!     .version(Version::V1)
//!     .character_set(CharacterSet::UTF8)
//!     .identification(Identification::Sct)
//!     .bic("GENODEF1SLR")
//!     .beneficiary("Codeberg e.V.")
//!     .iban(iban)
//!     .amount("10.00")
//!     .remittance(epcgen::Remittance::Text("for the good cause".to_string()));
//! let epc = builder.build();
//! let epc = epc.expect("example should work");
//! let qrcode_payload = epc.to_string();
//! let code = QrCode::new(qrcode_payload).unwrap();
//! // Render the data into an image.
//! let image = code.render::<Luma<u8>>().build();
//! // Save the image.
//! image.save("./examples_basic_usage_qrcode.png").unwrap();
//! ```

mod ibanrf;
pub use ibanrf::iban;
pub use ibanrf::rf;
mod epcgen;
pub use epcgen::*;
