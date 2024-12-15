#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use opentelemetry::trace::TracerProvider as _;
use suitokengentest::errors::TokenGenErrors;
use tracing_subscriber::{fmt::format::FmtSpan, prelude::*};
pub mod utils {
    pub mod errors {
        pub use suitokengentest::errors::TokenGenErrors;
    }
    pub mod generation {
        use chrono::{Datelike, Utc};
        use serde::Serialize;
        use std::{collections::HashMap, env};
        use tera::{Context, Tera};
        use crate::utils::{
            helpers::sanitize_name, variables::{SUI_PROJECT, SUI_PROJECT_SUB_DIR},
        };
        struct Package {
            name: String,
            edition: String,
            version: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Package {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Package",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "edition",
                        &self.edition,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "version",
                        &self.version,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        struct Dependency {
            #[serde(rename = "Sui")]
            sui: SuiDependency,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Dependency {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Dependency",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "Sui",
                        &self.sui,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        struct SuiDependency {
            git: String,
            subdir: String,
            rev: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for SuiDependency {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SuiDependency",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "git",
                        &self.git,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "subdir",
                        &self.subdir,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "rev",
                        &self.rev,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        struct MoveToml {
            package: Package,
            dependencies: Dependency,
            addresses: HashMap<String, String>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for MoveToml {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "MoveToml",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "package",
                        &self.package,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "dependencies",
                        &self.dependencies,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "addresses",
                        &self.addresses,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub fn generate_token(
            decimals: u8,
            symbol: String,
            name: &str,
            description: String,
            is_frozen: bool,
            is_test: bool,
        ) -> String {
            let slug = sanitize_name(&name.to_string());
            let module_name = slug.clone();
            let token_type = slug.to_uppercase();
            let current_dir = env::current_dir().unwrap();
            let templates_path = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!("{0}/src/templates/**/*", current_dir.display()),
                );
                res
            });
            let tera = Tera::new(&templates_path).unwrap();
            let mut context = Context::new();
            context.insert("module_name", &module_name);
            context.insert("token_type", &token_type);
            context.insert("name", &name);
            context.insert("symbol", &symbol);
            context.insert("decimals", &decimals);
            context.insert("description", &description);
            context.insert("is_frozen", &is_frozen);
            let template_file = if is_test {
                "test_token_template.move"
            } else {
                "token_template.move"
            };
            let token_template: String = tera.render(template_file, &context).unwrap();
            token_template
        }
        pub fn generate_move_toml(package_name: &str, environment: String) -> String {
            let current_year: u32 = Utc::now().year_ce().1;
            let move_toml = MoveToml {
                package: Package {
                    name: package_name.to_string(),
                    edition: ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.beta", current_year),
                        );
                        res
                    }),
                    version: "0.0.1".to_string(),
                },
                dependencies: Dependency {
                    sui: SuiDependency {
                        git: SUI_PROJECT.to_string(),
                        subdir: SUI_PROJECT_SUB_DIR.to_string(),
                        rev: ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("framework/{0}", environment),
                            );
                            res
                        }),
                    },
                },
                addresses: {
                    let mut addresses = HashMap::new();
                    addresses.insert(package_name.to_string(), "0x0".to_string());
                    addresses
                },
            };
            let toml_content: String = toml::to_string(&move_toml).unwrap();
            toml_content
        }
    }
    pub mod helpers {
        use regex::Regex;
        pub fn is_valid_github_url(url: &str) -> bool {
            let github_url_pattern = r"^https?://(www\.)?github\.com/[\w\-]+/[\w\-]+(/)?$";
            let re = Regex::new(github_url_pattern).expect("Invalid pattern");
            re.is_match(url)
        }
        pub fn sanitize_name(name: &String) -> String {
            name.chars().filter(|c| c.is_alphanumeric()).collect::<String>()
        }
        pub fn filter_token_content(content: &str) -> String {
            let re = Regex::new(r"///.*|//.*").unwrap();
            let cleaned_content: std::borrow::Cow<'_, str> = re.replace_all(content, "");
            let non_empty_lines: Vec<&str> = cleaned_content
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim())
                .collect();
            non_empty_lines.join("")
        }
        pub fn get_token_info(content: &str) -> (u8, String, String, String, bool) {
            let mut decimals = 0;
            let mut symbol = String::new();
            let mut name = String::new();
            let mut description = String::new();
            let mut is_frozen = false;
            let mut tokens = content.split_whitespace().peekable();
            while let Some(token) = tokens.next() {
                if token.contains("witness") {
                    let mut args = Vec::new();
                    let mut char = String::new();
                    for arg in tokens.by_ref() {
                        if arg.ends_with(");") || arg.ends_with(")")
                            || arg.ends_with("option::none(),")
                        {
                            let trimmed = char
                                .trim_end_matches(&[')', ';'][..])
                                .to_string();
                            args.push(trimmed);
                            break;
                        }
                        if arg.starts_with("b\"") {
                            let trimmed = char
                                .trim_end_matches(',')
                                .trim_start_matches(" b\"")
                                .to_string();
                            args.push(trimmed);
                            char.clear();
                        }
                        if char.is_empty() {
                            char = "".to_string() + arg.trim_end_matches("\",");
                        } else {
                            char += " ";
                            char += arg.trim_end_matches("\",");
                        }
                    }
                    if args.len() >= 4 {
                        decimals = args[0].trim().parse().unwrap_or(0);
                        symbol = args[1]
                            .trim_start_matches("b\"")
                            .trim_end_matches("\"")
                            .to_string();
                        name = args[2]
                            .trim_start_matches("b\"")
                            .trim_end_matches("\"")
                            .to_string();
                        description = args[3]
                            .trim_start_matches("b\"")
                            .trim_end_matches("\"")
                            .to_string();
                    }
                } else if token.contains("transfer::public_freeze_object") {
                    is_frozen = true;
                }
            }
            (decimals, symbol, name, description, is_frozen)
        }
        pub fn is_running_test() -> bool {
            std::env::var("RUNNING_TEST").map_or(false, |val| val == "true")
        }
    }
    pub mod variables {
        pub const SUB_FOLDER: &str = "sources";
        pub const SUI_PROJECT: &str = "https://github.com/MystenLabs/sui.git";
        pub const SUI_PROJECT_SUB_DIR: &str = "crates/sui-framework/packages/sui-framework";
    }
    pub mod verify_helper {
        use anyhow::Result;
        use git2::Repository;
        use std::{
            env, fs::{self, ReadDir},
            io, path::Path,
        };
        use suitokengentest::errors::TokenGenErrors;
        use url::Url;
        use crate::utils::{
            generation::generate_token,
            helpers::{
                filter_token_content, get_token_info, is_running_test,
                is_valid_github_url,
            },
            variables::SUB_FOLDER,
        };
        pub async fn verify_token_using_url(url: &str) -> Result<(), TokenGenErrors> {
            Url::parse(url)
                .map_err(|_| {
                    TokenGenErrors::InvalidUrl(
                        "The provided URL is not a valid URL.".to_string(),
                    )
                })?;
            if !is_valid_github_url(url) {
                return Err(
                    TokenGenErrors::InvalidUrl(
                        "The provided URL is not a valid GitHub URL.".to_string(),
                    ),
                );
            }
            let repo_name = url
                .trim_end_matches(".git")
                .rsplit('/')
                .next()
                .ok_or_else(|| {
                    TokenGenErrors::InvalidUrl(
                        "Failed to extract repository name.".to_string(),
                    )
                })?;
            let clone_path = Path::new(repo_name);
            check_cloned_contract(clone_path)?;
            Repository::clone(url, clone_path)
                .map_err(|e| TokenGenErrors::GitError(e.to_string()))?;
            let current_dir = env::current_dir()
                .map_err(|e| TokenGenErrors::FileIoError(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Failed to read current dir: {0}", e),
                        );
                        res
                    }),
                ))?;
            let templates_path: String = ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(
                    format_args!(
                        "{0}/{1}/{2}",
                        current_dir.display(),
                        repo_name,
                        SUB_FOLDER,
                    ),
                );
                res
            });
            let templates_path_ref: &Path = Path::new(&templates_path);
            if templates_path_ref.exists() && templates_path_ref.is_dir() {
                verify_contract(templates_path_ref, clone_path).await?;
                check_cloned_contract(clone_path)?;
            } else {
                return Err(
                    TokenGenErrors::InvalidPath("Cloned repo not found".to_string()),
                );
            }
            Ok(())
        }
        fn check_cloned_contract(path: &Path) -> Result<(), TokenGenErrors> {
            if path.exists() && path.is_dir() && !is_running_test() {
                fs::remove_dir_all(path)
                    .map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
            }
            Ok(())
        }
        pub fn read_file(file_path: &Path) -> Result<String, TokenGenErrors> {
            if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
                return Err(
                    TokenGenErrors::FileIoError("File is not a .move file".to_string()),
                );
            }
            let content = fs::read_to_string(file_path)
                .map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
            Ok(content)
        }
        pub fn read_dir(dir: &Path) -> io::Result<ReadDir> {
            let content = fs::read_dir(dir)?;
            Ok(content)
        }
        pub async fn verify_contract(
            dir: &Path,
            clone_path: &Path,
        ) -> Result<(), TokenGenErrors> {
            if !dir.is_dir() {
                return Err(
                    TokenGenErrors::InvalidPath("Path is not a directory".to_string()),
                );
            }
            let entries = read_dir(dir)
                .map_err(|e| TokenGenErrors::FileIoError(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Failed to read directory: {0}", e),
                        );
                        res
                    }),
                ))?;
            for entry in entries {
                let entry = entry
                    .map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|e| e == "move") {
                    let current_content = read_file(&path)?;
                    compare_contract_content(current_content, Some(clone_path))?;
                }
            }
            Ok(())
        }
        pub fn compare_contract_content(
            current_content: String,
            clone_path: Option<&Path>,
        ) -> Result<(), TokenGenErrors> {
            let cleaned_current_content: String = filter_token_content(&current_content);
            let details: (u8, String, String, String, bool) = get_token_info(
                &current_content,
            );
            let expected_content: String = generate_token(
                details.0,
                details.1,
                &details.2,
                details.3.to_owned(),
                details.4,
                false,
            );
            let cleaned_expected_content: String = filter_token_content(
                &expected_content,
            );
            if cleaned_current_content != cleaned_expected_content {
                if let Some(path) = clone_path {
                    check_cloned_contract(path)?;
                }
                return Err(
                    TokenGenErrors::VerificationError(
                        "Contract verification failed: content mismatch detected"
                            .to_string(),
                    ),
                );
            }
            Ok(())
        }
    }
}
pub trait TokenGen: ::core::marker::Sized {
    async fn create(
        self,
        context: ::tarpc::context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), TokenGenErrors>;
    async fn verify_url(
        self,
        context: ::tarpc::context::Context,
        url: String,
    ) -> Result<(), TokenGenErrors>;
    async fn verify_content(
        self,
        context: ::tarpc::context::Context,
        content: String,
    ) -> Result<(), TokenGenErrors>;
    /// Returns a serving function to use with
    /// [InFlightRequest::execute](::tarpc::server::InFlightRequest::execute).
    fn serve(self) -> ServeTokenGen<Self> {
        ServeTokenGen { service: self }
    }
}
///The stub trait for service [`TokenGen`].
pub trait TokenGenStub: ::tarpc::client::stub::Stub<
        Req = TokenGenRequest,
        Resp = TokenGenResponse,
    > {}
impl<S> TokenGenStub for S
where
    S: ::tarpc::client::stub::Stub<Req = TokenGenRequest, Resp = TokenGenResponse>,
{}
/// A serving function to use with [::tarpc::server::InFlightRequest::execute].
pub struct ServeTokenGen<S> {
    service: S,
}
#[automatically_derived]
impl<S: ::core::clone::Clone> ::core::clone::Clone for ServeTokenGen<S> {
    #[inline]
    fn clone(&self) -> ServeTokenGen<S> {
        ServeTokenGen {
            service: ::core::clone::Clone::clone(&self.service),
        }
    }
}
impl<S> ::tarpc::server::Serve for ServeTokenGen<S>
where
    S: TokenGen,
{
    type Req = TokenGenRequest;
    type Resp = TokenGenResponse;
    async fn serve(
        self,
        ctx: ::tarpc::context::Context,
        req: TokenGenRequest,
    ) -> ::core::result::Result<TokenGenResponse, ::tarpc::ServerError> {
        match req {
            TokenGenRequest::Create {
                name,
                symbol,
                decimals,
                description,
                frozen,
                environment,
            } => {
                ::core::result::Result::Ok(
                    TokenGenResponse::Create(
                        TokenGen::create(
                                self.service,
                                ctx,
                                name,
                                symbol,
                                decimals,
                                description,
                                frozen,
                                environment,
                            )
                            .await,
                    ),
                )
            }
            TokenGenRequest::VerifyUrl { url } => {
                ::core::result::Result::Ok(
                    TokenGenResponse::VerifyUrl(
                        TokenGen::verify_url(self.service, ctx, url).await,
                    ),
                )
            }
            TokenGenRequest::VerifyContent { content } => {
                ::core::result::Result::Ok(
                    TokenGenResponse::VerifyContent(
                        TokenGen::verify_content(self.service, ctx, content).await,
                    ),
                )
            }
        }
    }
}
/// The request sent over the wire from the client to the server.
#[allow(missing_docs)]
#[serde(crate = "::tarpc::serde")]
pub enum TokenGenRequest {
    Create {
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    },
    VerifyUrl { url: String },
    VerifyContent { content: String },
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::tarpc::serde as _serde;
    #[automatically_derived]
    impl ::tarpc::serde::Serialize for TokenGenRequest {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::tarpc::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::tarpc::serde::Serializer,
        {
            match *self {
                TokenGenRequest::Create {
                    ref name,
                    ref symbol,
                    ref decimals,
                    ref description,
                    ref frozen,
                    ref environment,
                } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "TokenGenRequest",
                        0u32,
                        "Create",
                        0 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "name",
                        name,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "symbol",
                        symbol,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "decimals",
                        decimals,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "description",
                        description,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "frozen",
                        frozen,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "environment",
                        environment,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                TokenGenRequest::VerifyUrl { ref url } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "TokenGenRequest",
                        1u32,
                        "VerifyUrl",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "url",
                        url,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
                TokenGenRequest::VerifyContent { ref content } => {
                    let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                        __serializer,
                        "TokenGenRequest",
                        2u32,
                        "VerifyContent",
                        0 + 1,
                    )?;
                    _serde::ser::SerializeStructVariant::serialize_field(
                        &mut __serde_state,
                        "content",
                        content,
                    )?;
                    _serde::ser::SerializeStructVariant::end(__serde_state)
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::tarpc::serde as _serde;
    #[automatically_derived]
    impl<'de> ::tarpc::serde::Deserialize<'de> for TokenGenRequest {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::tarpc::serde::__private::Result<Self, __D::Error>
        where
            __D: ::tarpc::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Create" => _serde::__private::Ok(__Field::__field0),
                        "VerifyUrl" => _serde::__private::Ok(__Field::__field1),
                        "VerifyContent" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Create" => _serde::__private::Ok(__Field::__field0),
                        b"VerifyUrl" => _serde::__private::Ok(__Field::__field1),
                        b"VerifyContent" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TokenGenRequest>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TokenGenRequest;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TokenGenRequest",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                                __field4,
                                __field5,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        1u64 => _serde::__private::Ok(__Field::__field1),
                                        2u64 => _serde::__private::Ok(__Field::__field2),
                                        3u64 => _serde::__private::Ok(__Field::__field3),
                                        4u64 => _serde::__private::Ok(__Field::__field4),
                                        5u64 => _serde::__private::Ok(__Field::__field5),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "name" => _serde::__private::Ok(__Field::__field0),
                                        "symbol" => _serde::__private::Ok(__Field::__field1),
                                        "decimals" => _serde::__private::Ok(__Field::__field2),
                                        "description" => _serde::__private::Ok(__Field::__field3),
                                        "frozen" => _serde::__private::Ok(__Field::__field4),
                                        "environment" => _serde::__private::Ok(__Field::__field5),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"name" => _serde::__private::Ok(__Field::__field0),
                                        b"symbol" => _serde::__private::Ok(__Field::__field1),
                                        b"decimals" => _serde::__private::Ok(__Field::__field2),
                                        b"description" => _serde::__private::Ok(__Field::__field3),
                                        b"frozen" => _serde::__private::Ok(__Field::__field4),
                                        b"environment" => _serde::__private::Ok(__Field::__field5),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<TokenGenRequest>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = TokenGenRequest;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant TokenGenRequest::Create",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field1 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    1usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field2 = match _serde::de::SeqAccess::next_element::<
                                        u8,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    2usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field3 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    3usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field4 = match _serde::de::SeqAccess::next_element::<
                                        bool,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    4usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    let __field5 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    5usize,
                                                    &"struct variant TokenGenRequest::Create with 6 elements",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::Create {
                                        name: __field0,
                                        symbol: __field1,
                                        decimals: __field2,
                                        description: __field3,
                                        frozen: __field4,
                                        environment: __field5,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                    let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                    let mut __field2: _serde::__private::Option<u8> = _serde::__private::None;
                                    let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                                    let mut __field4: _serde::__private::Option<bool> = _serde::__private::None;
                                    let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            __Field::__field1 => {
                                                if _serde::__private::Option::is_some(&__field1) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("symbol"),
                                                    );
                                                }
                                                __field1 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            __Field::__field2 => {
                                                if _serde::__private::Option::is_some(&__field2) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "decimals",
                                                        ),
                                                    );
                                                }
                                                __field2 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                                );
                                            }
                                            __Field::__field3 => {
                                                if _serde::__private::Option::is_some(&__field3) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "description",
                                                        ),
                                                    );
                                                }
                                                __field3 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            __Field::__field4 => {
                                                if _serde::__private::Option::is_some(&__field4) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("frozen"),
                                                    );
                                                }
                                                __field4 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                );
                                            }
                                            __Field::__field5 => {
                                                if _serde::__private::Option::is_some(&__field5) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "environment",
                                                        ),
                                                    );
                                                }
                                                __field5 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("name")?
                                        }
                                    };
                                    let __field1 = match __field1 {
                                        _serde::__private::Some(__field1) => __field1,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("symbol")?
                                        }
                                    };
                                    let __field2 = match __field2 {
                                        _serde::__private::Some(__field2) => __field2,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("decimals")?
                                        }
                                    };
                                    let __field3 = match __field3 {
                                        _serde::__private::Some(__field3) => __field3,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("description")?
                                        }
                                    };
                                    let __field4 = match __field4 {
                                        _serde::__private::Some(__field4) => __field4,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("frozen")?
                                        }
                                    };
                                    let __field5 = match __field5 {
                                        _serde::__private::Some(__field5) => __field5,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("environment")?
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::Create {
                                        name: __field0,
                                        symbol: __field1,
                                        decimals: __field2,
                                        description: __field3,
                                        frozen: __field4,
                                        environment: __field5,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &[
                                "name",
                                "symbol",
                                "decimals",
                                "description",
                                "frozen",
                                "environment",
                            ];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<TokenGenRequest>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field1, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "url" => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"url" => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<TokenGenRequest>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = TokenGenRequest;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant TokenGenRequest::VerifyUrl",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant TokenGenRequest::VerifyUrl with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::VerifyUrl {
                                        url: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("url")?
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::VerifyUrl {
                                        url: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["url"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<TokenGenRequest>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                        (__Field::__field2, __variant) => {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "content" => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"content" => _serde::__private::Ok(__Field::__field0),
                                        _ => _serde::__private::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private::PhantomData<TokenGenRequest>,
                                lifetime: _serde::__private::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = TokenGenRequest;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private::Formatter,
                                ) -> _serde::__private::fmt::Result {
                                    _serde::__private::Formatter::write_str(
                                        __formatter,
                                        "struct variant TokenGenRequest::VerifyContent",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        String,
                                    >(&mut __seq)? {
                                        _serde::__private::Some(__value) => __value,
                                        _serde::__private::None => {
                                            return _serde::__private::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct variant TokenGenRequest::VerifyContent with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::VerifyContent {
                                        content: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private::Option::is_some(&__field0) {
                                                    return _serde::__private::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                                            "content",
                                                        ),
                                                    );
                                                }
                                                __field0 = _serde::__private::Some(
                                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private::Some(__field0) => __field0,
                                        _serde::__private::None => {
                                            _serde::__private::de::missing_field("content")?
                                        }
                                    };
                                    _serde::__private::Ok(TokenGenRequest::VerifyContent {
                                        content: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["content"];
                            _serde::de::VariantAccess::struct_variant(
                                __variant,
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private::PhantomData::<TokenGenRequest>,
                                    lifetime: _serde::__private::PhantomData,
                                },
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "Create",
                "VerifyUrl",
                "VerifyContent",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TokenGenRequest",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TokenGenRequest>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(missing_docs)]
impl ::core::fmt::Debug for TokenGenRequest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TokenGenRequest::Create {
                name: __self_0,
                symbol: __self_1,
                decimals: __self_2,
                description: __self_3,
                frozen: __self_4,
                environment: __self_5,
            } => {
                let names: &'static _ = &[
                    "name",
                    "symbol",
                    "decimals",
                    "description",
                    "frozen",
                    "environment",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    __self_0,
                    __self_1,
                    __self_2,
                    __self_3,
                    __self_4,
                    &__self_5,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Create",
                    names,
                    values,
                )
            }
            TokenGenRequest::VerifyUrl { url: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "VerifyUrl",
                    "url",
                    &__self_0,
                )
            }
            TokenGenRequest::VerifyContent { content: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "VerifyContent",
                    "content",
                    &__self_0,
                )
            }
        }
    }
}
impl ::tarpc::RequestName for TokenGenRequest {
    fn name(&self) -> &str {
        match self {
            TokenGenRequest::Create { .. } => "TokenGen.create",
            TokenGenRequest::VerifyUrl { .. } => "TokenGen.verify_url",
            TokenGenRequest::VerifyContent { .. } => "TokenGen.verify_content",
        }
    }
}
/// The response sent over the wire from the server to the client.
#[allow(missing_docs)]
#[serde(crate = "::tarpc::serde")]
pub enum TokenGenResponse {
    Create(Result<(String, String, String), TokenGenErrors>),
    VerifyUrl(Result<(), TokenGenErrors>),
    VerifyContent(Result<(), TokenGenErrors>),
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::tarpc::serde as _serde;
    #[automatically_derived]
    impl ::tarpc::serde::Serialize for TokenGenResponse {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> ::tarpc::serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: ::tarpc::serde::Serializer,
        {
            match *self {
                TokenGenResponse::Create(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "TokenGenResponse",
                        0u32,
                        "Create",
                        __field0,
                    )
                }
                TokenGenResponse::VerifyUrl(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "TokenGenResponse",
                        1u32,
                        "VerifyUrl",
                        __field0,
                    )
                }
                TokenGenResponse::VerifyContent(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "TokenGenResponse",
                        2u32,
                        "VerifyContent",
                        __field0,
                    )
                }
            }
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::tarpc::serde as _serde;
    #[automatically_derived]
    impl<'de> ::tarpc::serde::Deserialize<'de> for TokenGenResponse {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::tarpc::serde::__private::Result<Self, __D::Error>
        where
            __D: ::tarpc::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "variant identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 3",
                                ),
                            )
                        }
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "Create" => _serde::__private::Ok(__Field::__field0),
                        "VerifyUrl" => _serde::__private::Ok(__Field::__field1),
                        "VerifyContent" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"Create" => _serde::__private::Ok(__Field::__field0),
                        b"VerifyUrl" => _serde::__private::Ok(__Field::__field1),
                        b"VerifyContent" => _serde::__private::Ok(__Field::__field2),
                        _ => {
                            let __value = &_serde::__private::from_utf8_lossy(__value);
                            _serde::__private::Err(
                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                            )
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<TokenGenResponse>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TokenGenResponse;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "enum TokenGenResponse",
                    )
                }
                fn visit_enum<__A>(
                    self,
                    __data: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::EnumAccess<'de>,
                {
                    match _serde::de::EnumAccess::variant(__data)? {
                        (__Field::__field0, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<(String, String, String), TokenGenErrors>,
                                >(__variant),
                                TokenGenResponse::Create,
                            )
                        }
                        (__Field::__field1, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<(), TokenGenErrors>,
                                >(__variant),
                                TokenGenResponse::VerifyUrl,
                            )
                        }
                        (__Field::__field2, __variant) => {
                            _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<
                                    Result<(), TokenGenErrors>,
                                >(__variant),
                                TokenGenResponse::VerifyContent,
                            )
                        }
                    }
                }
            }
            #[doc(hidden)]
            const VARIANTS: &'static [&'static str] = &[
                "Create",
                "VerifyUrl",
                "VerifyContent",
            ];
            _serde::Deserializer::deserialize_enum(
                __deserializer,
                "TokenGenResponse",
                VARIANTS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TokenGenResponse>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[automatically_derived]
#[allow(missing_docs)]
impl ::core::fmt::Debug for TokenGenResponse {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            TokenGenResponse::Create(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Create", &__self_0)
            }
            TokenGenResponse::VerifyUrl(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "VerifyUrl",
                    &__self_0,
                )
            }
            TokenGenResponse::VerifyContent(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "VerifyContent",
                    &__self_0,
                )
            }
        }
    }
}
#[allow(unused)]
/// The client stub that makes RPC calls to the server. All request methods return
/// [Futures](::core::future::Future).
pub struct TokenGenClient<
    Stub = ::tarpc::client::Channel<TokenGenRequest, TokenGenResponse>,
>(
    Stub,
);
#[automatically_derived]
#[allow(unused)]
impl<Stub: ::core::clone::Clone> ::core::clone::Clone for TokenGenClient<Stub> {
    #[inline]
    fn clone(&self) -> TokenGenClient<Stub> {
        TokenGenClient(::core::clone::Clone::clone(&self.0))
    }
}
#[automatically_derived]
#[allow(unused)]
impl<Stub: ::core::fmt::Debug> ::core::fmt::Debug for TokenGenClient<Stub> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "TokenGenClient", &&self.0)
    }
}
impl TokenGenClient {
    /// Returns a new client stub that sends requests over the given transport.
    pub fn new<T>(
        config: ::tarpc::client::Config,
        transport: T,
    ) -> ::tarpc::client::NewClient<
        Self,
        ::tarpc::client::RequestDispatch<TokenGenRequest, TokenGenResponse, T>,
    >
    where
        T: ::tarpc::Transport<
            ::tarpc::ClientMessage<TokenGenRequest>,
            ::tarpc::Response<TokenGenResponse>,
        >,
    {
        let new_client = ::tarpc::client::new(config, transport);
        ::tarpc::client::NewClient {
            client: TokenGenClient(new_client.client),
            dispatch: new_client.dispatch,
        }
    }
}
impl<Stub> ::core::convert::From<Stub> for TokenGenClient<Stub>
where
    Stub: ::tarpc::client::stub::Stub<Req = TokenGenRequest, Resp = TokenGenResponse>,
{
    /// Returns a new client stub that sends requests over the given transport.
    fn from(stub: Stub) -> Self {
        TokenGenClient(stub)
    }
}
impl<Stub> TokenGenClient<Stub>
where
    Stub: ::tarpc::client::stub::Stub<Req = TokenGenRequest, Resp = TokenGenResponse>,
{
    #[allow(unused)]
    pub fn create(
        &self,
        ctx: ::tarpc::context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    ) -> impl ::core::future::Future<
        Output = ::core::result::Result<
            Result<(String, String, String), TokenGenErrors>,
            ::tarpc::client::RpcError,
        >,
    > + '_ {
        let request = TokenGenRequest::Create {
            name,
            symbol,
            decimals,
            description,
            frozen,
            environment,
        };
        let resp = self.0.call(ctx, request);
        async move {
            match resp.await? {
                TokenGenResponse::Create(msg) => ::core::result::Result::Ok(msg),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    #[allow(unused)]
    pub fn verify_url(
        &self,
        ctx: ::tarpc::context::Context,
        url: String,
    ) -> impl ::core::future::Future<
        Output = ::core::result::Result<
            Result<(), TokenGenErrors>,
            ::tarpc::client::RpcError,
        >,
    > + '_ {
        let request = TokenGenRequest::VerifyUrl { url };
        let resp = self.0.call(ctx, request);
        async move {
            match resp.await? {
                TokenGenResponse::VerifyUrl(msg) => ::core::result::Result::Ok(msg),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
    #[allow(unused)]
    pub fn verify_content(
        &self,
        ctx: ::tarpc::context::Context,
        content: String,
    ) -> impl ::core::future::Future<
        Output = ::core::result::Result<
            Result<(), TokenGenErrors>,
            ::tarpc::client::RpcError,
        >,
    > + '_ {
        let request = TokenGenRequest::VerifyContent {
            content,
        };
        let resp = self.0.call(ctx, request);
        async move {
            match resp.await? {
                TokenGenResponse::VerifyContent(msg) => ::core::result::Result::Ok(msg),
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
    }
}
/// Initializes an OpenTelemetry tracing subscriber with a OTLP backend.
pub fn init_tracing(service_name: &'static str) -> anyhow::Result<()> {
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(
            opentelemetry_sdk::trace::Config::default()
                .with_resource(
                    opentelemetry_sdk::Resource::new([
                        opentelemetry::KeyValue::new(
                            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                            service_name,
                        ),
                    ]),
                ),
        )
        .with_batch_config(opentelemetry_sdk::trace::BatchConfig::default())
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());
    let tracer = tracer_provider.tracer(service_name);
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE),
        )
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .try_init()?;
    Ok(())
}
