/// Returns the name of the application.
pub fn application_name() -> &'static str {
    "bomproof"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_name_is_bomproof() {
        assert_eq!(application_name(), "bomproof");
    }
}
