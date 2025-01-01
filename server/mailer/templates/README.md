# Email templates
These templates are the base email used for email types.
The templates render to HTML and are definined in Handlebars, a templating
language.

## Variables
The variables available depend on the email type. These are definined in the file for
that specific email. E.g. for `password_changed,`, the data available is defined on the `Data`
associated type of the `Mailable` implementation. In this case, that is the struct `PasswordChangedData`.

## Language
Multiple languages are supported. For every language there should be a template file.
For instance, if your template name is `password_changed`, you'll need two Handlebars files:
- `password_changed.nl.hbs`, for the Dutch version
- `password_changed.en.hbs`, for the English version

## Partials
All partials defined in the `partials` directory can be used in all templates.
Furthermore, the design of the API allow for runtime-defined partials. E.g. for a banner.
The partials do not support translation automatically. You could use different partials
for aach language, that is up to you.
