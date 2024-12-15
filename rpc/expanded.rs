#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use async_trait::async_trait;
use clap::Parser;
use futures::{future, prelude::*};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};
use service::{TokenGen, init_tracing};
use tarpc::{
    context, server::{BaseChannel, Channel},
    tokio_serde::formats::Json,
};
#[command(author, version, about, long_about = None)]
struct Flags {
    /// The port to listen on.
    #[arg(short, long, default_value_t = 5000)]
    port: u16,
}
#[automatically_derived]
#[allow(unused_qualifications, clippy::redundant_locals)]
impl clap::Parser for Flags {}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::CommandFactory for Flags {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("tarpc-example-service");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("tarpc-example-service");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::FromArgMatches for Flags {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        let v = Flags {
            port: __clap_arg_matches
                .remove_one::<u16>("port")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: port",
                ))?,
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if __clap_arg_matches.contains_id("port") {
            #[allow(non_snake_case)]
            let port = &mut self.port;
            *port = __clap_arg_matches
                .remove_one::<u16>("port")
                .ok_or_else(|| clap::Error::raw(
                    clap::error::ErrorKind::MissingRequiredArgument,
                    "The following required argument was not provided: port",
                ))?;
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(
    dead_code,
    unreachable_code,
    unused_variables,
    unused_braces,
    unused_qualifications,
)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped,
    clippy::redundant_locals,
)]
#[automatically_derived]
impl clap::Args for Flags {
    fn group_id() -> Option<clap::Id> {
        Some(clap::Id::from("Flags"))
    }
    fn augment_args<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Flags")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("port")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("port")
                        .value_name("PORT")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                u16,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("The port to listen on")
                        .long_help(None)
                        .short('p')
                        .long("port")
                        .default_value({
                            static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                            let s = DEFAULT_VALUE
                                .get_or_init(|| {
                                    let val: u16 = 5000;
                                    ::std::string::ToString::to_string(&val)
                                });
                            let s: &'static str = &*s;
                            s
                        });
                    let arg = arg;
                    arg
                });
            __clap_app
                .author("Tim Kuehn <tikue@google.com>")
                .version("0.16.0")
                .about("An example server built on tarpc.")
                .long_about(None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        {
            let __clap_app = __clap_app
                .group(
                    clap::ArgGroup::new("Flags")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("port")];
                            members
                        }),
                );
            let __clap_app = __clap_app
                .arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("port")
                        .value_name("PORT")
                        .required(false && clap::ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::impl_prelude::*;
                            let auto = ::clap_builder::builder::_infer_ValueParser_for::<
                                u16,
                            >::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(clap::ArgAction::Set);
                    let arg = arg
                        .help("The port to listen on")
                        .long_help(None)
                        .short('p')
                        .long("port")
                        .default_value({
                            static DEFAULT_VALUE: ::std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                            let s = DEFAULT_VALUE
                                .get_or_init(|| {
                                    let val: u16 = 5000;
                                    ::std::string::ToString::to_string(&val)
                                });
                            let s: &'static str = &*s;
                            s
                        });
                    let arg = arg.required(false);
                    arg
                });
            __clap_app
                .author("Tim Kuehn <tikue@google.com>")
                .version("0.16.0")
                .about("An example server built on tarpc.")
                .long_about(None)
        }
    }
}
struct TokenServer;
#[automatically_derived]
impl ::core::clone::Clone for TokenServer {
    #[inline]
    fn clone(&self) -> TokenServer {
        TokenServer
    }
}
fn get_project_root() -> Result<PathBuf, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    let project_root = if current_dir.ends_with("rpc") {
        current_dir.parent().unwrap().to_path_buf()
    } else {
        current_dir
    };
    Ok(project_root)
}
impl service::TokenGen for TokenServer {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn create<'async_trait>(
        _ctx: context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<
                    (String, String, String),
                    suitokengentest::errors::TokenGenErrors,
                >,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<(String, String, String), suitokengentest::errors::TokenGenErrors>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let _ctx = _ctx;
            let name = name;
            let symbol = symbol;
            let decimals = decimals;
            let description = description;
            let frozen = frozen;
            let environment = environment;
            let __ret: Result<
                (String, String, String),
                suitokengentest::errors::TokenGenErrors,
            > = {
                let project_root = get_project_root()?;
                let token_template = std::fs::read_to_string(
                        project_root.join("src/templates/move/token.move.template"),
                    )
                    .map_err(|e| suitokengentest::errors::TokenGenErrors::FileIoError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to read token template: {0}", e),
                            );
                            res
                        }),
                    ))?;
                let toml_template = std::fs::read_to_string(
                        project_root.join("src/templates/toml/Move.toml.template"),
                    )
                    .map_err(|e| suitokengentest::errors::TokenGenErrors::FileIoError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to read toml template: {0}", e),
                            );
                            res
                        }),
                    ))?;
                let token_content = token_template
                    .replace("{{name}}", &name)
                    .replace("{{symbol}}", &symbol)
                    .replace("{{description}}", &description)
                    .replace("{{decimals}}", &decimals.to_string())
                    .replace("{{is_frozen}}", &frozen.to_string());
                let toml_content = toml_template
                    .replace("{{name}}", &name)
                    .replace("{{symbol}}", &symbol)
                    .replace("{{environment}}", &environment);
                let temp_dir = tempfile::tempdir()
                    .map_err(|e| suitokengentest::errors::TokenGenErrors::FileIoError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to create temporary directory: {0}", e),
                            );
                            res
                        }),
                    ))?;
                Ok((
                    temp_dir.path().to_string_lossy().to_string(),
                    token_content,
                    toml_content,
                ))
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn verify_url<'async_trait>(
        _ctx: context::Context,
        url: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<(), suitokengentest::errors::TokenGenErrors>,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<(), suitokengentest::errors::TokenGenErrors>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let _ctx = _ctx;
            let url = url;
            let __ret: Result<(), suitokengentest::errors::TokenGenErrors> = {
                service::utils::verify_helper::verify_token_using_url(&url).await
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn verify_content<'async_trait>(
        _ctx: context::Context,
        content: String,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<(), suitokengentest::errors::TokenGenErrors>,
            > + ::core::marker::Send + 'async_trait,
        >,
    > {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<(), suitokengentest::errors::TokenGenErrors>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let _ctx = _ctx;
            let content = content;
            let __ret: Result<(), suitokengentest::errors::TokenGenErrors> = {
                let temp_dir = tempfile::tempdir()
                    .map_err(|e| suitokengentest::errors::TokenGenErrors::FileIoError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to create temporary directory: {0}", e),
                            );
                            res
                        }),
                    ))?;
                let temp_file = temp_dir.path().join("temp.move");
                std::fs::write(&temp_file, &content)
                    .map_err(|e| suitokengentest::errors::TokenGenErrors::FileIoError(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Failed to write temporary file: {0}", e),
                            );
                            res
                        }),
                    ))?;
                service::utils::verify_helper::verify_contract(
                        temp_dir.path(),
                        temp_dir.path(),
                    )
                    .await
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        init_tracing("token-gen-server")?;
        let flags = Flags::parse();
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
        let server = TokenServer;
        let mut listener = tarpc::serde_transport::tcp::listen(&addr, Json::default)
            .await?;
        listener.config_mut().max_frame_length(usize::MAX);
        {
            ::std::io::_print(format_args!("Server listening on {0}\n", addr));
        };
        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(BaseChannel::with_defaults)
            .for_each(|channel| {
                let server = server.clone();
                async move {
                    let _ = channel.execute(TokenGen::serve(server));
                }
            })
            .await;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
