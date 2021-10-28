use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DataStruct, DeriveInput, Error, Fields, Result};

use crate::{
    attributes::{find_attr, parse_doc, ChannelType, TypeAttribute},
    fields::StructField,
};

/// Implementation of CreateCommand derive macro
pub fn impl_create_command(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let span = ident.span();

    // Parse type fields
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => StructField::from_fields(fields)?,
        _ => {
            return Err(Error::new(
                span,
                "`#[derive(CommandModel)] can only be applied to structs with named fields",
            ));
        }
    };

    check_fields_order(&fields)?;

    let attributes = match find_attr(&input.attrs, "command") {
        Some(attr) => TypeAttribute::parse(attr)?,
        None => {
            return Err(Error::new(
                span,
                "Missing required #[command(...)] attribute",
            ))
        }
    };

    let field_options = fields
        .iter()
        .map(field_option)
        .collect::<Result<Vec<_>>>()?;

    let name = &attributes.name;
    let description = match &attributes.desc {
        Some(desc) => desc.clone(),
        None => parse_doc(&input.attrs, span)?,
    };
    let default_permission = attributes.default_permission;

    Ok(quote! {
        impl ::twilight_interactions::command::CreateCommand for #ident {
            fn create_command() -> ::twilight_interactions::command::ApplicationCommandData {
                let mut command_options = ::std::vec::Vec::new();

                #(#field_options)*

                ::twilight_interactions::command::ApplicationCommandData {
                    name: ::std::convert::From::from(#name),
                    description: ::std::convert::From::from(#description),
                    options: command_options,
                    default_permission: #default_permission,
                }
            }
        }
    })
}

/// Generate field option code
fn field_option(field: &StructField) -> Result<TokenStream> {
    let ty = &field.ty;
    let span = field.span;

    let name = field.attributes.name_default(field.ident.to_string());
    let description = match &field.attributes.desc {
        Some(desc) => desc.clone(),
        None => parse_doc(&field.raw_attrs, field.span)?,
    };
    let required = field.kind.required();
    let channel_types = field.attributes.channel_types.iter().map(channel_type);

    Ok(quote_spanned! {span=>
        command_options.push(<#ty as ::twilight_interactions::command::CreateOption>::create_option(
            ::twilight_interactions::command::CommandOptionData {
                name: ::std::convert::From::from(#name),
                description: ::std::convert::From::from(#description),
                required: #required,
                channel_types: ::std::vec![#(#channel_types),*]
            }
        ));
    })
}

/// Ensure optional options are after required ones
fn check_fields_order(fields: &[StructField]) -> Result<()> {
    let mut optional_option_added = false;

    for field in fields {
        if !optional_option_added && !field.kind.required() {
            optional_option_added = true;
        }

        if optional_option_added && field.kind.required() {
            return Err(Error::new(
                field.span,
                "Required options should be added before optional",
            ));
        }
    }

    Ok(())
}

/// Convert a [`ChannelType`] into a [`TokenStream`]
fn channel_type(kind: &ChannelType) -> TokenStream {
    match kind {
        ChannelType::GuildText => quote!(::twilight_model::channel::ChannelType::GuildText),
        ChannelType::Private => quote!(::twilight_model::channel::ChannelType::Private),
        ChannelType::GuildVoice => quote!(::twilight_model::channel::ChannelType::GuildVoice),
        ChannelType::Group => quote!(::twilight_model::channel::ChannelType::Group),
        ChannelType::GuildCategory => quote!(::twilight_model::channel::ChannelType::GuildCategory),
        ChannelType::GuildNews => quote!(::twilight_model::channel::ChannelType::GuildNews),
        ChannelType::GuildStore => quote!(::twilight_model::channel::ChannelType::GuildStore),
        ChannelType::GuildNewsThread => {
            quote!(::twilight_model::channel::ChannelType::GuildNewsThread)
        }
        ChannelType::GuildPublicThread => {
            quote!(::twilight_model::channel::ChannelType::GuildPublicThread)
        }
        ChannelType::GuildPrivateThread => {
            quote!(::twilight_model::channel::ChannelType::GuildPrivateThread)
        }
        ChannelType::GuildStageVoice => {
            quote!(::twilight_model::channel::ChannelType::GuildStageVoice)
        }
    }
}
