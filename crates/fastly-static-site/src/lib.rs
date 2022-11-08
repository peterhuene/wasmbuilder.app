use anyhow::Result;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{abort_call_site, emit_call_site_warning, proc_macro_error};
use quote::quote;
use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, LitStr, Token,
};
use walkdir::WalkDir;
use wildmatch::WildMatch;

mod kw {
    syn::custom_keyword!(directory);
    syn::custom_keyword!(exclude);
    syn::custom_keyword!(root);
}

enum Field {
    Directory(Span, PathBuf),
    Exclude(Span, Vec<WildMatch>),
    Root(Span, String),
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::directory) {
            let span = input.parse::<kw::directory>()?.span;
            input.parse::<Token![:]>()?;
            Ok(Self::Directory(
                span,
                input.parse::<syn::LitStr>()?.value().into(),
            ))
        } else if l.peek(kw::exclude) {
            let span = input.parse::<kw::exclude>()?.span;
            input.parse::<Token![:]>()?;
            let content;
            syn::bracketed!(content in input);
            let wildcards = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
            Ok(Self::Exclude(
                span,
                wildcards
                    .into_iter()
                    .map(|p| WildMatch::new(&p.value()))
                    .collect(),
            ))
        } else if l.peek(kw::root) {
            let span = input.parse::<kw::root>()?.span;
            input.parse::<Token![:]>()?;
            Ok(Self::Root(span, input.parse::<syn::LitStr>()?.value()))
        } else {
            Err(l.error())
        }
    }
}

#[derive(Default)]
struct MacroInput {
    directory: Option<PathBuf>,
    exclude: Vec<WildMatch>,
    root: Option<String>,
}

impl Parse for MacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let call_site = proc_macro2::Span::call_site();
        let mut ret = Self::default();
        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            for field in Punctuated::<Field, Token![,]>::parse_terminated(&content)?.into_iter() {
                match field {
                    Field::Directory(span, path) => {
                        if !path.is_dir() {
                            return Err(syn::Error::new(
                                span,
                                format!(
                                    "`{}` does not exist or is not a directory",
                                    path.display()
                                ),
                            ));
                        }
                        if ret.directory.is_some() {
                            return Err(syn::Error::new(
                                span,
                                "cannot specify `directory` more than once",
                            ));
                        }
                        ret.directory = Some(path);
                    }
                    Field::Exclude(_, wildcards) => {
                        ret.exclude.extend(wildcards);
                    }
                    Field::Root(span, root) => {
                        if ret.root.is_some() {
                            return Err(syn::Error::new(
                                span,
                                "cannot specify `root` more than once",
                            ));
                        }
                        ret.root = Some(root);
                    }
                }
            }
        } else {
            let path: PathBuf = input.parse::<syn::LitStr>()?.value().into();
            if !path.is_dir() {
                return Err(syn::Error::new(
                    call_site,
                    format!("`{}` does not exist or is not a directory", path.display()),
                ));
            }
            ret.directory = Some(path);
        }

        if ret.directory.is_none() {
            return Err(syn::Error::new(
                call_site,
                "must specify directory to serve with the `directory` field",
            ));
        }

        Ok(ret)
    }
}

#[derive(Default)]
struct Files {
    paths: Vec<String>,
    bytes: Vec<String>,
    mimes: Vec<String>,
    caches: Vec<&'static str>,
}

impl Files {
    fn new(input: &MacroInput) -> Result<Self> {
        let directory = input.directory.as_ref().unwrap();
        let mut files = Self::default();
        let mut found_root = false;
        for entry in WalkDir::new(directory).into_iter() {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            if input
                .exclude
                .iter()
                .any(|w| w.matches(entry.file_name().to_str().unwrap()))
            {
                continue;
            }

            let path = entry.path();

            let file_path = path.strip_prefix(directory)?.to_str().unwrap().to_string();

            match &input.root {
                Some(root) if *root == file_path => {
                    found_root = true;
                    files.paths.push("".to_string());
                    files.caches.push("no-cache");
                }
                _ => {
                    files.paths.push(file_path);
                    files.caches.push("max-age=31536000, public, immutable");
                }
            }

            files
                .bytes
                .push(path.canonicalize()?.to_str().unwrap().to_string());

            let ty = match entry.path().extension() {
                Some(ext) => mime_guess::from_ext(ext.to_str().unwrap())
                    .first_or_else(|| {
                        emit_call_site_warning!(
                            "file `{}` has no MIME mapping; defaulting to `application/octet-stream`",
                            entry.path().display()
                        );
                        mime_guess::mime::APPLICATION_OCTET_STREAM
                    })
                    .to_string(),
                None => {
                    emit_call_site_warning!(
                        "file `{}` has no extension; defaulting to `application/octet-stream`",
                        entry.path().display()
                    );
                    mime_guess::mime::APPLICATION_OCTET_STREAM.to_string()
                }
            };

            files.mimes.push(ty);
        }

        if let Some(root) = &input.root {
            if !found_root {
                emit_call_site_warning!(
                    "root file `{root}` was not found in directory `{}`; not serving path `/`",
                    directory.display(),
                );
            }
        }

        Ok(files)
    }
}

#[proc_macro_error]
#[proc_macro]
pub fn serve(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as MacroInput);

    let Files {
        paths,
        bytes,
        mimes,
        caches,
    } = match Files::new(&input) {
        Ok(files) => files,
        Err(e) => {
            abort_call_site!("{}", e);
        }
    };

    if paths.is_empty() {
        abort_call_site!(format!(
            "directory `{}` is either empty or all files were excluded",
            input.directory.unwrap().display()
        ));
    }

    quote!(
        use fastly::{http::StatusCode, Error, Request, Response};

        #[fastly::main]
        fn main(req: Request) -> Result<Response, Error> {
            match req.get_url().path() {
                #(
                    concat!("/", #paths) => {
                        println!("serving file `/{}` ({})", #paths, #mimes);
                        return Ok(Response::from_status(StatusCode::OK)
                            .with_header("Content-Type", #mimes)
                            .with_header("Cache-Control", #caches)
                            .with_body(include_bytes!(#bytes) as &[u8]));
                    }
                )*
                p => {
                    println!("file `{}` was not found", p);
                    Ok(Response::from_status(StatusCode::NOT_FOUND))
                }
            }
        }
    )
    .into()
}
