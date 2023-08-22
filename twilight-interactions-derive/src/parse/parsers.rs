use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Error, Lit, Path, Result};

use super::attribute::{ParseAttribute, ParseSpanned};

/// Path to a function.
#[derive(Clone, Debug)]
pub struct FunctionPath(Path);

impl ParseAttribute for FunctionPath {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let Lit::Str(lit) = input else {
            return Err(Error::new_spanned(input, "expected string literal"));
        };

        let path = lit.parse_with(Path::parse_mod_style)?;

        Ok(Self(path))
    }
}

impl ToTokens for FunctionPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

/// Slash command or command option name.
///
/// The following requirements are validated:
/// - Length between 1 and 32 characters
/// - Only alphanumeric character allowed (except '-' and '_')
/// - Must be lowercase when possible
///
/// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming
#[derive(Clone, Debug)]
pub struct CommandName(String);

impl ParseAttribute for CommandName {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;
        let value = spanned.inner.trim();

        match value.chars().count() {
            1..=32 => (),
            _ => return Err(spanned.error("name must be between 1 and 32 characters")),
        }

        for char in value.chars() {
            if !char.is_alphanumeric() && char != '-' && char != '_' {
                return Err(spanned.error(format!(
                    "name must only contain word characters, found invalid character `{char}`"
                )));
            }

            if char.to_lowercase().to_string() != char.to_string() {
                return Err(spanned.error(format!(
                    "name must be in lowercase, found invalid character `{char}`"
                )));
            }
        }

        Ok(Self(value.to_owned()))
    }
}

impl ToTokens for CommandName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl From<CommandName> for String {
    fn from(value: CommandName) -> Self {
        value.0
    }
}

/// Slash command or command option description.
///
/// This validate that the description is between 1 and 100 characters.
#[derive(Clone, Debug)]
pub struct CommandDescription(String);

impl ParseAttribute for CommandDescription {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;
        let value = spanned.inner.trim();

        match value.chars().count() {
            1..=100 => (),
            _ => return Err(spanned.error("description must be between 1 and 100 characters")),
        }

        Ok(Self(value.to_owned()))
    }
}

impl ToTokens for CommandDescription {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}

impl From<CommandDescription> for String {
    fn from(value: CommandDescription) -> Self {
        value.0
    }
}

/// Slash command choice name.
///
/// This validate that the choice is between 1 and 100 characters.
/// https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-choice-structure
#[derive(Clone, Debug)]
pub struct ChoiceName(String);

impl ParseAttribute for ChoiceName {
    fn parse_attribute(input: Lit) -> Result<Self> {
        let spanned: ParseSpanned<String> = ParseAttribute::parse_attribute(input)?;
        let value = spanned.inner.trim();

        match value.chars().count() {
            1..=100 => (),
            _ => return Err(spanned.error("name must be between 1 and 100 characters")),
        }

        Ok(Self(value.to_owned()))
    }
}

impl ToTokens for ChoiceName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }
}
