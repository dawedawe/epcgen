#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ServiceTag {
    Bcd,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    // 001 - EWR plus Non-EWR
    V1,
    // 002 - only EWR
    V2,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CharacterSet {
    // 1
    UTF8,
    // 2
    Iso8859_1,
    // 3
    Iso8859_2,
    // 4
    Iso8859_4,
    // 5
    Iso8859_5,
    // 6
    Iso8859_7,
    // 7
    Iso8859_10,
    // 8
    Iso8859_15,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Identification {
    // SEPA Credit Transfer
    Sct,
    // Sepa Instant Credit Transfer
    // todo test if already supported by apps
    Inst,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Purpose {
    Bene,
    //
    // max len 4
    Custom(String),
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
    amount: Option<f32>,
    // Purpose of the SEPA Credit Transfer
    purpose: Option<Purpose>,
    // The Remittance Information (Structured), max len 35
    remittance_reference: Option<String>,
    // The Remittance Information (Unstructured), max len 140
    remittance: Option<String>,
    // Beneficiary to Originator Information
    information: Option<String>,
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
    amount: Option<f32>,
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
        self.iban = Some(iban);
        self
    }

    pub fn amount(mut self, amount: f32) -> Self {
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
    fn it_works() {
        let builder = Builder::new();
        assert_eq!(builder.service_tag, ServiceTag::Bcd);
        let builder = builder.version(Version::V1);
        assert_eq!(builder.version, Some(Version::V1));
        let builder = builder.character_set(CharacterSet::UTF8);
        assert_eq!(builder.character_set, Some(CharacterSet::UTF8));
        let builder = builder.identification(Identification::Sct);
        assert_eq!(builder.identification, Some(Identification::Sct));
        let builder = builder.bic("BHBLDEHHXXX".to_string());
        assert_eq!(builder.bic, Some("BHBLDEHHXXX".to_string()));
        let builder = builder.beneficiary("John Doe".to_string());
        assert_eq!(builder.beneficiary, Some("John Doe".to_string()));
        let builder = builder.iban("DE71110220330123456789".to_string());
        assert_eq!(builder.iban, Some("DE71110220330123456789".to_string()));
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
    }
}
