/// Language of the email
pub enum Locale {
    Nl,
    En,
}

impl Locale {
    /// Format the name of the template based on the locale.
    /// E.g. the template name `foo` becomes `foo.nl` if the locale is [Locale::Nl].
    pub fn template_name_localized(&self, template_name: &str) -> String {
        let locale = match self {
            Locale::Nl => "nl",
            Locale::En => "en",
        };

        format!("{template_name}.{locale}")
    }
}
