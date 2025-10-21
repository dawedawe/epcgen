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
    // todo test if already supported by apps
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
    //
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
    // The Remittance Information (Structured), max len 35
    remittance_reference: Option<String>,
    // The Remittance Information (Unstructured), max len 140
    remittance: Option<String>,
    // Beneficiary to Originator Information
    information: Option<String>,
}

impl Display for Epc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.service_tag)?;
        writeln!(f, "{}", self.version)?;
        writeln!(f, "{}", self.character_set)?;
        writeln!(f, "{}", self.identification)?;
        let bic = self.bic.clone().unwrap_or("".to_string());
        writeln!(f, "{}", bic)?;
        writeln!(f, "{}", self.beneficiary)?;
        writeln!(f, "{}", self.iban)?;
        let amount = self.amount.map(|a| a.to_string()).unwrap_or("".to_string());
        writeln!(f, "{}", amount)?;
        let purpose = self
            .purpose
            .clone()
            .map(|p| p.to_string())
            .unwrap_or("".to_string());
        writeln!(f, "{}", purpose)?;
        let remittance_reference = self.remittance_reference.clone().unwrap_or("".to_string());
        writeln!(f, "{}", remittance_reference)?;
        let remittance = self.remittance.clone().unwrap_or("".to_string());
        writeln!(f, "{}", remittance)?;
        let information = self.information.clone().unwrap_or("".to_string());
        write!(f, "{}", information)
    }
}

pub struct Builder {
    // Service Tag
    service_tag: ServiceTag,
    // Version
    version: Option<Version>,
    // Character set
    character_set: Option<CharacterSet>,
    // Identification code
    identification: Option<Identification>,
    // The BIC code of the Beneficiary PSP
    bic: Option<String>,
    // The name of the accout of the Beneficiary
    beneficiary: Option<String>,
    // The IBAN of the accout of the Beneficiary
    iban: Option<String>,
    // Amount of the SEPA Credit Transfer in Euro
    amount: Option<f64>,
    // Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    // The Remittance Information (Structured), max len 35
    remittance_reference: Option<String>,
    // The Remittance Information (Unstructured), max len 140
    remittance: Option<String>,
    // Beneficiary to Originator Information
    information: Option<String>,
}

impl Builder {
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
            remittance_reference: None,
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

    pub fn bic(mut self, bic: String) -> Self {
        self.bic = Some(bic);
        self
    }

    pub fn beneficiary(mut self, beneficiary: String) -> Self {
        self.beneficiary = Some(beneficiary);
        self
    }

    pub fn iban(mut self, iban: String) -> Self {
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

    pub fn remittance_reference(mut self, remittance_reference: String) -> Self {
        self.remittance_reference = Some(remittance_reference);
        self
    }

    pub fn remittance(mut self, remittance: String) -> Self {
        self.remittance = Some(remittance);
        self
    }

    pub fn information(mut self, information: String) -> Self {
        self.information = Some(information);
        self
    }

    pub fn build(&self) -> Result<Epc, String> {
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

        let beneficiary = if let Some(beneficiary) = self.beneficiary.clone() {
            beneficiary
        } else {
            return Result::Err("Beneficiary missing".to_string());
        };

        let iban = if let Some(iban) = self.iban.clone() {
            iban
        } else {
            return Result::Err("IBAN missing".to_string());
        };

        Result::Ok(Epc {
            service_tag: self.service_tag,
            version,
            character_set,
            identification,
            bic: self.bic.clone(),
            beneficiary,
            iban,
            amount: self.amount,
            purpose: self.purpose.clone(),
            remittance_reference: self.remittance_reference.clone(),
            remittance: self.remittance.clone(),
            information: self.information.clone(),
        })
    }
}

impl Default for Builder {
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
        let builder = builder.bic("GENODEF1SLR".to_string());
        assert_eq!(builder.bic, Some("GENODEF1SLR".to_string()));
        let builder = builder.beneficiary("Codeberg e.V.".to_string());
        assert_eq!(builder.beneficiary, Some("Codeberg e.V.".to_string()));
        let builder = builder.iban("DE90 8306 5408 0004 1042 42".to_string());
        assert_eq!(builder.iban, Some("DE90830654080004104242".to_string()));
        let builder = builder.amount(999999999.99);
        assert_eq!(builder.amount, Some(999999999.99));
        let builder = builder.purpose(Purpose::Bene);
        assert_eq!(builder.purpose, Some(Purpose::Bene));
        let builder = builder.remittance_reference("RF18539007547034".to_string());
        assert_eq!(
            builder.remittance_reference,
            Some("RF18539007547034".to_string())
        );
        let builder = builder.remittance("cash rules everything around me".to_string());
        assert_eq!(
            builder.remittance,
            Some("cash rules everything around me".to_string())
        );
        let builder = builder.information("thanks".to_string());
        assert_eq!(builder.information, Some("thanks".to_string()));
        let epc = builder.build();
        assert!(epc.is_ok());
        let epc = epc.unwrap();
        assert_eq!(
            "BCD\n001\n1\nSCT\nGENODEF1SLR\nCodeberg e.V.\nDE90830654080004104242\n999999999.99\nBENE\nRF18539007547034\ncash rules everything around me\nthanks",
            epc.to_string()
        );
    }
}
