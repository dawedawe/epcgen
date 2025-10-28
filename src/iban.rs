fn transform(iban: &str) -> u128 {
    let first4 = iban.get(0..4).expect("expected IBAN with len >= 4");
    let after4 = iban.get(4..).expect("expected IBAN with len >= 5");
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

pub fn is_valid(iban: &str) -> bool {
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
        && transform(iban) % 97 == 1
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn invalid_ibans_should_fail() {
        assert!(!is_valid(""));
        assert!(!is_valid("DE90830654080004104243"));
    }
}
