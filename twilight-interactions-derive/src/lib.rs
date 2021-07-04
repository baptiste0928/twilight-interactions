use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Field, Fields,
    Lit, Meta, MetaNameValue,
};

#[proc_macro_derive(SlashCommand)]
pub fn slash_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => {
            return TokenStream::from(quote! {
                compile_error!("`#[derive(SlashCommand)] can only be applied to structs with named fields");
            })
        }
    };
    let fields_init = fields.named.iter().map(field_init);
    let fields_match_arms = fields.named.iter().map(field_match_arm);
    let fields_constructor = fields.named.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        quote!(#ident: #ident?)
    });

    let name = ident.to_string().to_lowercase();
    let descr = parse_doc(&input.attrs);
    let descr = if !descr.is_empty() {
        quote!(#descr)
    } else {
        return TokenStream::from(quote!(compile_error!("command must have a description");));
    };
    let options_len = fields.named.len();
    let options = fields.named.iter().map(create_option_for_field);

    TokenStream::from(quote! {
        impl ::twilight_slash::SlashCommand for #ident {
            fn from_interaction(
                data: ::twilight_model::application::interaction::application_command::CommandData,
            ) -> ::std::option::Option<Self> {
                #(#fields_init)*

                for opt in data.options {
                    match &*opt.name {
                        #(#fields_match_arms,)*
                        _ => return None,
                    }
                }

                Some(Self { #(#fields_constructor),* })
            }

            fn create_application_command() -> ::twilight_slash::CreateApplicationCommand {
                let mut options = ::std::vec::Vec::with_capacity(#options_len);
                #(#options)*

                ::twilight_slash::CreateApplicationCommand {
                    name: #name.to_string(),
                    description: #descr.to_string(),
                    options,
                    default_permission: None, // TODO
                }
            }
        }
    })
}

fn field_init(field: &Field) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    let ty_span = ty.span();
    let default = quote_spanned! { ty_span => <#ty as ::twilight_slash::CommandOption>::DEFAULT };
    quote!(let mut #ident = #default;)
}

fn field_match_arm(field: &Field) -> proc_macro2::TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let arm_pat = ident.to_string();
    let ty_span = field.ty.span();
    let from_option = quote_spanned!(ty_span => ::twilight_slash::CommandOption::from_option(opt.value, data.resolved.as_ref()));

    quote!(#arm_pat => #ident = Some(#from_option?))
}

fn parse_doc(attrs: &[Attribute]) -> String {
    let mut doc = String::new();
    for attr in attrs {
        match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(descr),
                ..
            })) if path.segments.len() == 1 && path.segments.first().unwrap().ident == "doc" => {
                doc.push_str(&descr.value());
                doc.push('\n');
            }
            _ => {}
        }
    }
    doc.trim().to_owned()
}

fn create_option_for_field(field: &Field) -> proc_macro2::TokenStream {
    let ty = &field.ty;
    let ty_span = ty.span();
    let name = field.ident.as_ref().unwrap().to_string().to_lowercase();
    let kind = quote_spanned! { ty_span => <#ty as ::twilight_slash::CommandOption>::OPTION_TYPE };
    let required =
        quote_spanned! { ty_span => <#ty as ::twilight_slash::CommandOption>::DEFAULT.is_none() };
    let descr = parse_doc(&field.attrs);
    let descr = if !descr.is_empty() {
        quote!(#descr)
    } else {
        let field_span = field.span();
        let msg = format!(
            "option `{}` must have a description",
            field.ident.as_ref().unwrap().to_string(),
        );
        return quote_spanned!(field_span => compile_error!(#msg););
    };
    let path = quote!(::twilight_model::application::command);
    let mut arms = Vec::with_capacity(9);
    for kind in [quote!(String), quote!(Integer)] {
        arms.push(quote! {
            #path::CommandOptionType::#kind => #path::CommandOption::#kind(
                #path::ChoiceCommandOptionData {
                    choices: ::std::vec::Vec::new(),
                    name,
                    description,
                    required,
                }
            ),
        });
    }
    for kind in [
        quote!(Boolean),
        quote!(User),
        quote!(Channel),
        quote!(Role),
        quote!(Mentionable),
    ] {
        arms.push(quote! {
            #path::CommandOptionType::#kind => #path::CommandOption::#kind(
                #path::BaseCommandOptionData {
                    name,
                    description,
                    required,
                }
            ),
        });
    }
    for kind in [quote!(SubCommand), quote!(SubCommandGroup)] {
        arms.push(quote!(#path::CommandOptionType::#kind => todo!(),))
    }

    quote! {
        let name = #name.to_string();
        let description = #descr.to_string();
        let required = #required;
        let option = match #kind {
            #(#arms)*
        };
        options.push(option);
    }
}
