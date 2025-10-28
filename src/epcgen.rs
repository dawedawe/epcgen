use crate::iban::is_valid;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServiceTag {
    Bcd,
}

impl Display for ServiceTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceTag::Bcd => write!(f, "BCD"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    // 001 - EWR plus Non-EWR
    V1,
    // 002 - only EWR
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharacterSet {
    // 1
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Identification {
    // SEPA Credit Transfer
    Sct,
    // Sepa Instant Credit Transfer
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

#[derive(Debug, PartialEq, Clone)]
pub enum Purpose {
    Bene,
    // Todo add more
    // max len 4
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

#[derive(Debug, PartialEq, Clone)]
pub enum Remittance {
    // The structured remittance information, max len 35
    Reference(String),
    // The unstructured remittance information, max len 140
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

#[derive(Debug, PartialEq)]
pub struct Epc {
    // Service Tag
    service_tag: ServiceTag,
    // Version
    version: Version,
    // Character set
    character_set: CharacterSet,
    // Identification code
    identification: Identification,
    // The BIC code of the Beneficiary PSP
    bic: Option<String>,
    // The name of the accout of the Beneficiary
    beneficiary: String,
    // The IBAN of the accout of the Beneficiary
    iban: String,
    // Amount of the SEPA Credit Transfer in Euro
    amount: Option<f64>,
    // Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    // The Remittance Information (structured or unstructured)
    remittance: Option<Remittance>,
    // Beneficiary to Originator Information
    information: Option<String>,
}

impl<'a> Epc {
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
        let amount = self.amount.map(|a| a.to_string()).unwrap_or("".to_string());
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

pub struct Builder<'a> {
    // Service Tag
    service_tag: ServiceTag,
    // Version
    version: Option<Version>,
    // Character set
    character_set: Option<CharacterSet>,
    // Identification code
    identification: Option<Identification>,
    // The BIC code of the Beneficiary PSP
    bic: Option<&'a str>,
    // The name of the accout of the Beneficiary
    beneficiary: Option<&'a str>,
    // The IBAN of the accout of the Beneficiary
    iban: Option<String>,
    // Amount of the SEPA Credit Transfer in Euro
    amount: Option<f64>,
    // Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    // The Remittance Information (structured or unstructured)
    remittance: Option<Remittance>,
    // Beneficiary to Originator Information
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

    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn character_set(mut self, character_set: CharacterSet) -> Self {
        self.character_set = Some(character_set);
        self
    }

    pub fn identification(mut self, identification: Identification) -> Self {
        self.identification = Some(identification);
        self
    }

    pub fn bic(mut self, bic: &'a str) -> Self {
        self.bic = Some(bic);
        self
    }

    pub fn beneficiary(mut self, beneficiary: &'a str) -> Self {
        self.beneficiary = Some(beneficiary);
        self
    }

    pub fn iban(mut self, iban: &'a str) -> Self {
        self.iban = Some(iban.replace(" ", ""));
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = Some(purpose);
        self
    }

    pub fn remittance(mut self, remittance: Remittance) -> Self {
        self.remittance = Some(remittance);
        self
    }

    pub fn information(mut self, information: &'a str) -> Self {
        self.information = Some(information);
        self
    }

    pub fn build(&'_ self) -> Result<Epc, String> {
        let version = if let Some(version) = self.version {
            version
        } else {
            return Result::Err("Version missing".to_string());
        };

        let character_set = if let Some(character_set) = self.character_set {
            character_set
        } else {
            return Result::Err("CharacterSet missing".to_string());
        };

        let identification = if let Some(identification) = self.identification {
            identification
        } else {
            return Result::Err("Identification missing".to_string());
        };

        if self.bic.is_none() && version != Version::V2 {
            return Result::Err("BIC is missing but Version is not V2".to_string());
        }

        let beneficiary = if let Some(beneficiary) = self.beneficiary {
            beneficiary
        } else {
            return Result::Err("Beneficiary missing".to_string());
        };

        let iban = if let Some(iban) = self.iban.clone() {
            if is_valid(iban.as_str()) {
                iban
            } else {
                return Result::Err("Invalid IBAN".to_string());
            }
        } else {
            return Result::Err("IBAN missing".to_string());
        };

        match &self.purpose {
            Some(Purpose::Custom(p))
                if p.len() != 4 || p.chars().any(|c| !c.is_ascii_uppercase()) =>
            {
                return Result::Err("Invalid Purpose".to_string());
            }
            _ => (),
        }

        match &self.remittance {
            // Todo check structure
            Some(Remittance::Reference(s)) if s.len() > 35 => {
                return Result::Err("Remittance::Reference max len of 35 exceeded".to_string());
            }
            Some(Remittance::Text(s)) if s.len() > 140 => {
                return Result::Err("Remittance::Text max len of 140 exceeded".to_string());
            }
            _ => (),
        }

        Result::Ok(Epc {
            service_tag: self.service_tag,
            version,
            character_set,
            identification,
            bic: self.bic.map(|s| s.to_string()).clone(),
            beneficiary: beneficiary.to_string(),
            iban,
            amount: self.amount,
            purpose: self.purpose.clone(),
            remittance: self.remittance.clone(),
            information: self.information.map(|s| s.to_string()).clone(),
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
        let builder = builder.amount(999999999.99);
        assert_eq!(builder.amount, Some(999999999.99));
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
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn missing_character_set_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .identification(Identification::Sct)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn missing_identification_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .bic("GENODEF1SLR")
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn missing_bic_in_version1_should_fail() {
        let builder = Epc::builder()
            .version(Version::V1)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn missing_bic_in_version2_should_succeed() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_ok());
    }

    #[test]
    fn missing_beneficiary_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .iban("DE90 8306 5408 0004 1042 42")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn missing_iban_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .beneficiary("Codeberg e.V.")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
    }
    #[test]
    fn invalid_iban_should_fail() {
        let builder = Epc::builder()
            .version(Version::V2)
            .character_set(CharacterSet::UTF8)
            .identification(Identification::Sct)
            .iban("DE90 8306 5408 0004 1042 43")
            .beneficiary("Codeberg e.V.")
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());
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
            .remittance(Remittance::Reference("1234567890".to_string()));
        assert!(builder.build().is_err());

        let builder = builder.purpose(Purpose::Custom("ABC".to_string()));
        assert!(builder.build().is_err());

        let builder = builder.purpose(Purpose::Custom("ABC1".to_string()));
        assert!(builder.build().is_err());
    }

    #[test]
    fn too_long_remittance_reference_should_fail() {
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
        assert!(builder.build().is_err());
    }
}
