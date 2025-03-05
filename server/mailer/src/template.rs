use crate::error::Result;
use handlebars::Handlebars;
use include_directory::{include_directory, Dir};

/// Contents of the `partials` directory.
const PARTIALS: Dir<'_> = include_directory!("mailer/partials");
/// Contents of the `templates` directory
const TEMPLATES: Dir<'_> = include_directory!("mailer/templates/");

/// A handlebars template
pub struct HbsTemplate {
    /// The name of the template
    pub name: String,
    /// The value of the template in Handlebars
    pub content: String,
}

pub struct TemplateEngine(Handlebars<'static>);

impl TemplateEngine {
    pub fn into_inner(self) -> Handlebars<'static> {
        self.0
    }

    /// Set up the Handlebars engine.
    ///
    /// # Errors
    /// - If a partial is invalid.
    /// - If a template is invalid.
    pub fn new(extra_partials: Vec<HbsTemplate>) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.set_dev_mode(is_dev_mode());
        handlebars.set_strict_mode(true);

        // Register partials
        for template in Self::partials() {
            handlebars.register_partial(&template.name, template.content)?;
        }

        // Register the extra partials (runtime defined)
        for template in extra_partials {
            handlebars.register_partial(&template.name, template.content)?;
        }

        // Register templates
        for template in Self::templates() {
            handlebars.register_template_string(&template.name, template.content)?;
        }

        Ok(Self(handlebars))
    }

    /// Get all partials stored in the binary.
    /// Returns a tuple of (name, content).
    pub fn partials() -> Vec<HbsTemplate> {
        Self::get_embed(PARTIALS)
    }

    /// Get all templates stored in the binary.
    /// Returns a tuple of (name, content).
    pub fn templates() -> Vec<HbsTemplate> {
        Self::get_embed(TEMPLATES)
    }

    /// Get all files in an embedded direcotry.
    /// Returns a tuple of (name, content).
    fn get_embed(embed: Dir<'_>) -> Vec<HbsTemplate> {
        embed
            .files()
            .filter_map(|f| {
                let name = f
                    .path()
                    .file_name()
                    .and_then(|fname| fname.to_str())
                    // Split by .
                    .map(|fname| fname.split(".").collect::<Vec<_>>())
                    // Keep all but the last element
                    .map(|parts| all_but_last(parts.into_iter()))
                    // Re-join string
                    .map(|parts: Vec<&str>| parts.join("."))
                    .map(|fname| fname.to_string());

                let contents = f.contents_utf8().map(|cnt| cnt.to_string());

                match (name, contents) {
                    (Some(n), Some(c)) => Some(HbsTemplate {
                        name: n,
                        content: c,
                    }),
                    _ => None,
                }
            })
            .collect()
    }
}

/// Whether the program is compiled in debug mode
fn is_dev_mode() -> bool {
    cfg!(debug_assertions)
}

/// Keep all elements except the last element
fn all_but_last<T, I, B>(iter: I) -> B
where
    B: FromIterator<T>,
    I: DoubleEndedIterator<Item = T> + ExactSizeIterator<Item = T>,
{
    iter.rev().skip(1).rev().collect()
}
