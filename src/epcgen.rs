use crate::ibanrf::iban;
use crate::ibanrf::rf;
use std::error::Error;
use std::fmt::Display;

/// Service tag for EPC QR codes
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServiceTag {
    /// BCD - EPC Quick Response Code
    Bcd,
}

impl Display for ServiceTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceTag::Bcd => write!(f, "BCD"),
        }
    }
}

/// Version of the EPC QR code standard
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    /// 001 - EWR plus Non-EWR (BIC required)
    V1,
    /// 002 - only EWR (BIC optional)
    V2,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V1 => write!(f, "001"),
            Version::V2 => write!(f, "002"),
        }
    }
}

/// Character set encoding for the EPC QR code
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharacterSet {
    /// UTF-8 character set (value: 1)
    UTF8,
    // todo
    // 2
    // Iso8859_1,
    // 3
    // Iso8859_2,
    // 4
    // Iso8859_4,
    // 5
    // Iso8859_5,
    // 6
    // Iso8859_7,
    // 7
    // Iso8859_10,
    // 8
    // Iso8859_15,
}

impl Display for CharacterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterSet::UTF8 => write!(f, "1"),
        }
    }
}

/// Identification code for the type of SEPA credit transfer
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Identification {
    /// SEPA Credit Transfer (SCT)
    Sct,
    /// SEPA Instant Credit Transfer (INST)
    Inst,
}

impl Display for Identification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Identification::Sct => write!(f, "SCT"),
            Identification::Inst => write!(f, "INST"),
        }
    }
}

/// Purpose code for the SEPA credit transfer
#[derive(Debug, PartialEq, Clone)]
pub enum Purpose {
    /// BENE - Benefit payment
    Bene,
    // Todo add more
    /// A custom purpose code (must be exactly 4 uppercase ASCII characters)
    Custom(String),
}

impl Display for Purpose {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Purpose::Bene => write!(f, "BENE"),
            Purpose::Custom(c) => write!(f, "{}", c),
        }
    }
}

/// Remittance information for the payment
#[derive(Debug, PartialEq, Clone)]
pub enum Remittance {
    /// Structured RF creditor reference (validated against ISO 11649)
    Reference(String),
    /// Unstructured remittance information (max length: 140 characters)
    Text(String),
}

impl Display for Remittance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Remittance::Reference(r) => write!(f, "{}", r),
            Remittance::Text(r) => write!(f, "{}", r),
        }
    }
}

/// EPC QR code data structure
///
/// Use [`Epc::builder()`] to create instances.
#[derive(Debug, PartialEq)]
pub struct Epc {
    /// Service Tag
    service_tag: ServiceTag,
    /// Version
    version: Version,
    /// Character set
    character_set: CharacterSet,
    /// Identification code
    identification: Identification,
    /// The BIC code of the Beneficiary PSP
    bic: Option<String>,
    /// The name of the accout of the Beneficiary
    beneficiary: String,
    /// The IBAN of the accout of the Beneficiary
    iban: String,
    /// Amount of the SEPA Credit Transfer in Euro
    amount: Option<String>,
    /// Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    /// The Remittance Information (structured or unstructured)
    remittance: Option<Remittance>,
    /// Beneficiary to Originator Information
    information: Option<String>,
}

impl<'a> Epc {
    /// Creates a new builder for constructing an EPC QR code
    pub fn builder() -> Builder<'a> {
        Builder::default()
    }
}
impl Display for Epc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty_string = "".to_string();
        writeln!(f, "{}", self.service_tag)?;
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.character_set)?;
        writeln!(f, "{}", self.identification)?;
        let bic = self.bic.as_ref().unwrap_or(&empty_string);
        writeln!(f, "{}", bic)?;
        writeln!(f, "{}", self.beneficiary)?;
        writeln!(f, "{}", self.iban)?;
        let amount = self.amount.as_ref().unwrap_or(&empty_string);
        writeln!(f, "{}", amount)?;
        let purpose = self
            .purpose
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or("".to_string());
        writeln!(f, "{}", purpose)?;
        match &self.remittance {
            Some(Remittance::Reference(r)) => writeln!(f, "{}\n", r),
            Some(Remittance::Text(r)) => writeln!(f, "\n{}", r),
            None => writeln!(f, "\n"),
        }?;
        let information = self.information.as_ref().unwrap_or(&empty_string);
        write!(f, "{}", information)
    }
}

/// Errors that can occur when building an EPC QR code
#[derive(Debug, PartialEq)]
pub enum EpcError {
    MissingVersion,
    MissingCharacterSet,
    MissingIdentification,
    BICRequiredInConfiguredVersion,
    MissingBeneficiary,
    InvalidIBAN,
    MissingIBAN,
    InvalidAmount,
    InvalidPurpose,
    InvalidRemittanceReference,
    RemittanceTextTooLong,
}

impl Display for EpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EpcError::MissingVersion => write!(f, "Version missing"),
            EpcError::MissingCharacterSet => write!(f, "CharacterSet missing"),
            EpcError::MissingIdentification => write!(f, "Identification missing"),
            EpcError::BICRequiredInConfiguredVersion => {
                write!(f, "BIC is missing but configured Version requires it")
            }
            EpcError::MissingBeneficiary => write!(f, "Beneficiary missing"),
            EpcError::InvalidIBAN => write!(f, "Invalid IBAN"),
            EpcError::MissingIBAN => write!(f, "IBAN missing"),
            EpcError::InvalidAmount => write!(f, "Invalid amount"),
            EpcError::InvalidPurpose => write!(f, "Invalid purpose"),
            EpcError::InvalidRemittanceReference => {
                write!(f, "Invalid structured RF creditor reference")
            }
            EpcError::RemittanceTextTooLong => write!(f, "Remittance text too long (max len 140)"),
        }
    }
}

impl Error for EpcError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

/// Builder for creating EPC QR code instances
///
/// Use [`Epc::builder()`] to get a new builder instance.
pub struct Builder<'a> {
    /// Service Tag
    service_tag: ServiceTag,
    /// Version
    version: Option<Version>,
    /// Character set
    character_set: Option<CharacterSet>,
    /// Identification code
    identification: Option<Identification>,
    /// The BIC code of the Beneficiary PSP
    bic: Option<&'a str>,
    /// The name of the accout of the Beneficiary
    beneficiary: Option<&'a str>,
    /// The IBAN of the accout of the Beneficiary
    iban: Option<String>,
    /// Amount of the SEPA Credit Transfer in Euro
    amount: Option<&'a str>,
    /// Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    /// The Remittance Information (structured or unstructured)
    remittance: Option<Remittance>,
    /// Beneficiary to Originator Information
    information: Option<&'a str>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self {
            service_tag: ServiceTag::Bcd,
            version: None,
            character_set: None,
            identification: None,
            bic: None,
            beneficiary: None,
            iban: None,
            amount: None,
            purpose: None,
            remittance: None,
            information: None,
        }
    }

    /// Set the Version
    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the CharacterSet
    pub fn character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = Some(character_set);
        self
    }

    /// Set the Identification
    pub fn identification(mut self, identification: Identification) -> Self {
        self.identification = Some(identification);
        self
    }

    /// Set the BIC (Business Identifier Code) of the recipient bank
    pub fn bic(mut self, bic: &'a str) -> Self {
        self.bic = Some(bic);
        self
    }

    /// Set the name of the beneficiary
    pub fn beneficiary(mut self, beneficiary: &'a str) -> Self {
        self.beneficiary = Some(beneficiary);
        self
    }

    /// Set the IBAN of the beneficiary
    pub fn iban(mut self, iban: &'a str) -> Self {
        self.iban = Some(iban.replace(" ", ""));
        self
    }

    /// Set the amount of the transfer
    pub fn amount(mut self, amount: &'a str) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Set the purpose code
    pub fn purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = Some(purpose);
        self
    }

    /// Set the Remittance (reference)
    pub fn remittance(mut self, remittance: Remittance) -> Self {
        self.remittance = Some(remittance);
        self
    }

    /// Set the Beneficiary to Originator information
    pub fn information(mut self, information: &'a str) -> Self {
        self.information = Some(information);
        self
    }

    /// Build the resulting Epc
    pub fn build(&'_ self) -> Result<Epc, EpcError> {
        let version = if let Some(version) = self.version {
            version
        } else {
            return Result::Err(EpcError::MissingVersion);
        };

        let character_set = if let Some(character_set) = self.character_set {
            character_set
        } else {
            return Result::Err(EpcError::MissingCharacterSet);
        };

        let identification = if let Some(identification) = self.identification {
            identification
        } else {
            return Result::Err(EpcError::MissingIdentification);
        };

        if self.bic.is_none() && version != Version::V2 {
            return Result::Err(EpcError::BICRequiredInConfiguredVersion);
        }

        let beneficiary = if let Some(beneficiary) = self.beneficiary {
            beneficiary
        } else {
            return Result::Err(EpcError::MissingBeneficiary);
        };

        let iban = if let Some(iban) = self.iban.clone() {
            if iban::is_valid(iban.as_str()) {
                iban
            } else {
                return Result::Err(EpcError::InvalidIBAN);
            }
        } else {
            return Result::Err(EpcError::MissingIBAN);
        };

        let amount = if let Some(amount) = self.amount {
            let ok = amount.chars().all(|c| c.is_ascii_digit() || c == '.');
            if ok
                && let Some((i_part, d_part)) = amount.split_once(".")
                && i_part.len() <= 9
                && d_part.len() == 2
            {
                match (i_part.parse::<i128>(), d_part.parse::<i32>()) {
                    (Ok(i_part), Ok(d_part)) if i_part == 0 && (1..=99).contains(&d_part) => {
                        self.amount
                    }
                    (Ok(i_part), Ok(d_part))
                        if (1..=999999999).contains(&i_part) && (0..=99).contains(&d_part) =>
                    {
                        self.amount
                    }
                    (_, _) => return Result::Err(EpcError::InvalidAmount),
                }
            } else {
                return Result::Err(EpcError::InvalidAmount);
            }
        } else {
            None
        };

        match &self.purpose {
            Some(Purpose::Custom(p))
                if p.len() != 4 || p.chars().any(|c| !c.is_ascii_uppercase()) =>
            {
                return Result::Err(EpcError::InvalidPurpose);
            }
            _ => (),
        }

        match &self.remittance {
            Some(Remittance::Reference(s)) => {
                if !rf::is_valid(s) {
                    return Result::Err(EpcError::InvalidRemittanceReference);
                }
            }
            Some(Remittance::Text(s)) if s.len() > 140 => {
                return Result::Err(EpcError::RemittanceTextTooLong);
            }
            _ => (),
        }

        Result::Ok(Epc {
            service_tag: self.service_tag,
            version,
            character_set,
            identification,
            bic: self.bic.map(|s| s.to_string()),
            beneficiary: beneficiary.to_string(),
            iban,
            amount: amount.map(|s| s.to_string()),
            purpose: self.purpose.clone(),
            remittance: self.remittance.clone(),
            information: self.information.map(|s| s.to_string()),
        })
    }
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let builder = Builder::new();
        assert_eq!(builder.service_tag, ServiceTag::Bcd);
        let builder = builder.version(Version::V1);
        assert_eq!(builder.version, Some(Version::V1));
        let builder = builder.character_set(CharacterSet::UTF8);
        assert_eq!(builder.character_set, Some(CharacterSet::UTF8));
        let builder = builder.identification(Identification::Sct);
        assert_eq!(builder.identification, Some(Identification::Sct));
        let bic = "GENODEF1SLR";
        let builder = builder.bic(bic);
        assert_eq!(builder.bic, Some(bic));
        let beneficiary = "Codeberg e.V.";
        let builder = builder.beneficiary(beneficiary);
        assert_eq!(builder.beneficiary, Some(beneficiary));
        let builder = builder.iban("DE90 8306 5408 0004 1042 42");
        assert_eq!(builder.iban, Some("DE90830654080004104242".to_string()));
        let builder = builder.amount("999999999.99");
        assert_eq!(builder.amount, Some("999999999.99"));
        let builder = builder.purpose(Purpose::Bene);
        assert_eq!(builder.purpose, Some(Purpose::Bene));
        let builder = builder.remittance(Remittance::Text(
            "cash rules everything around me".to_string(),
        ));
        assert_eq!(
            builder.remittance,
            Some(Remittance::Text(
                "cash rules everything around me".to_string()
            ))
        );
        let builder = builder.information("thanks");
        assert_eq!(builder.information, Some("thanks"));
        let epc = builder.build();
        assert!(epc.is_ok());
        let epc = epc.unwrap();
        assert_eq!(
            "BCD\n001\n1\nSCT\nGENODEF1SLR\nCodeberg e.V.\nDE90830654080004104242\n999999999.99\nBENE\n\ncash rules everything around me\nthanks",
            epc.to_string()
        );
    }

    #[test]
    fn missing_version_should_fail() {
        let builder = Epc::builder()
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Text("1234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::MissingVersion));
    }

    #[test]
    fn missing_character_set_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .identification(Identification::Sct)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Text("1234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::MissingCharacterSet));
    }

    #[test]
    fn missing_identification_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Text("1234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::MissingIdentification));
    }

    #[test]
    fn missing_bic_in_version1_should_fail() {
        let builder = Epc::builder()
            .version(Version::V1)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("RF471234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::BICRequiredInConfiguredVersion));
    }

    #[test]
    fn missing_bic_in_version2_should_succeed() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("RF471234567890".to_string()));
        assert!(builder.build().is_ok());
    }

    #[test]
    fn missing_beneficiary_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("RF471234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::MissingBeneficiary));
    }

    #[test]
    fn missing_iban_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .remittance(Remittance::Reference("RF471234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::MissingIBAN));
    }

    #[test]
    fn invalid_iban_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .iban("DE90 8306 5408 0004 1042 43")
            .beneficiary("Codeberg e.V.")
            .remittance(Remittance::Reference("RF471234567890".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidIBAN));
    }

    #[test]
    fn invalid_amount_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .iban("DE90 8306 5408 0004 1042 42")
            .amount("-0.01")
            .beneficiary("Codeberg e.V.")
            .remittance(Remittance::Text("foo".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidAmount));

        let builder = builder.amount("0.00");
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidAmount));

        let builder = builder.amount("1.000");
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidAmount));

        let builder = builder.amount("1..00");
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidAmount));

        let builder = builder.amount("9999999990.99");
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidAmount));
    }

    #[test]
    fn invalid_purpose_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .purpose(Purpose::Custom("ABCDE".to_string()))
            .remittance(Remittance::Text("foo".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidPurpose));

        let builder = builder.purpose(Purpose::Custom("ABC".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidPurpose));

        let builder = builder.purpose(Purpose::Custom("ABC1".to_string()));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidPurpose));
    }

    #[test]
    fn invalid_remittance_reference_should_fail() {
        let builder = Epc::builder()
            .version(Version::V1)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference(
                "123456789012345678901234567890123456".to_string(),
            ));
        let r = builder.build();
        assert_eq!(r, Result::Err(EpcError::InvalidRemittanceReference));
    }
}
