fn transform(s: &str) -> u128 {
    let first4 = s.get(0..4).expect("expected IBAN with len >= 4");
    let after4 = s.get(4..).expect("expected IBAN with len >= 5");
    let switched = format!("{after4}{first4}");
    let replaced: String = switched
        .chars()
        .map(|c| {
            if c.is_numeric() {
                c.to_string()
            } else {
                let v = c as u32 - 64 + 9;
                v.to_string()
            }
        })
        .collect();
    replaced
        .as_str()
        .parse()
        .expect("expected parseable string")
}

pub mod iban {
    use crate::ibanrf::transform;

    /// Check the validity of an IBAN
    pub fn is_valid(iban: &str) -> bool {
        let iban = iban.replace(" ", "");
        iban.len() > 4
            && iban.len() <= 34
            && iban
                .get(0..2)
                .unwrap()
                .chars()
                .all(|c| c.is_ascii_uppercase())
            && iban
                .get(2..)
                .unwrap()
                .chars()
                .all(|c| c.is_numeric() || c.is_ascii_uppercase())
            && transform(iban.as_str()) % 97 == 1
    }
}

pub mod rf {
    use crate::ibanrf::transform;

    /// Check the validity of a structured RF creditor reference
    pub fn is_valid(reference: &str) -> bool {
        reference.len() > 4
            && reference.len() <= 25
            && reference.starts_with("RF")
            && reference
                .get(2..)
                .unwrap()
                .chars()
                .all(|c| c.is_numeric() || c.is_ascii_uppercase())
            && transform(reference) % 97 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ibanrf::iban;
    use crate::ibanrf::rf;

    #[test]
    fn transforming_ibans_works() {
        assert_eq!(
            transform("DE68210501700012345678"),
            210501700012345678131468
        );
        assert_eq!(
            transform("GB82WEST12345698765432"),
            3214282912345698765432161182
        )
    }

    #[test]
    fn transforming_structured_references_works() {
        assert_eq!(transform("RF45G72UUR"), 1672303027271545);
        assert_eq!(transform("RF6518K5"), 18205271565);
        assert_eq!(transform("RF35C4"), 124271535);
        assert_eq!(transform("RF214377"), 4377271521);
    }

    #[test]
    fn invalid_ibans_should_fail() {
        assert!(!iban::is_valid(""));
        assert!(!iban::is_valid("DE90830654080004104243"));
    }

    #[test]
    fn invalid_structured_references_should_fail() {
        assert!(!rf::is_valid(""));
        assert!(!rf::is_valid("RF55G72UUR"));
    }
}
