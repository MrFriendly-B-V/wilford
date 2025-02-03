use database::user::Locale;

/// Format the name of the template based on the locale.
/// E.g. the template name `foo` becomes `foo.nl` if the locale is [Locale::Nl].
pub fn localize_template_name(locale: &Locale, template_name: &str) -> String {
    let locale = match locale {
        Locale::Nl => "nl",
        Locale::En => "en",
    };

    format!("{template_name}.{locale}")
}
