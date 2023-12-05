use std::fs::{self, File};

use fs2::FileExt;
use proc_macro::{TokenStream, TokenTree};
use reqwest::{
    blocking::Client,
    header::{self, HeaderMap, HeaderValue},
    redirect::Policy,
};
use syn::parse_macro_input;

struct Params {
    year: u32,
    day: u32,
}

impl syn::parse::Parse for Params {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{punctuated::Punctuated, LitInt, Token};

        let literals = Punctuated::<LitInt, Token![,]>::parse_terminated(input)?;
        if literals.len() < 2 {
            return Err(syn::Error::new(input.span(), "year and day required"));
        } else if literals.len() > 2 {
            return Err(syn::Error::new(input.span(), "only year and day allowed"));
        }
        let year = literals[0].base10_parse()?;
        let day = literals[1].base10_parse()?;
        Ok(Params { year, day })
    }
}

#[proc_macro]
pub fn include_aoc(input: TokenStream) -> TokenStream {
    let Params { year, day } = parse_macro_input!(input as Params);

    let dir = scratch::path("include_aoc");
    let file_lock = File::create(dir.join(".lock")).expect("failed to create lock file in cache");
    FileExt::lock_exclusive(&file_lock).expect("failed to get lock on cache");
    let file_name = format!("y{year}d{day}.txt");
    let input_path = dir.join(file_name);

    let aoc_input = if !input_path.exists() {
        let aoc_input = load_from_aoc(year, day);
        fs::write(input_path, &aoc_input).expect("unable to save puzzle input to cache");
        aoc_input
    } else {
        fs::read_to_string(input_path).expect("unable to read input from cache")
    };

    TokenTree::Literal(proc_macro::Literal::string(&aoc_input)).into()
}

#[proc_macro]
pub fn cache_dir(_input: TokenStream) -> TokenStream {
    let dir = scratch::path("include_aoc");
    let dir = dir
        .as_os_str()
        .to_str()
        .expect("can't convert path to UTF-8");
    TokenTree::Literal(proc_macro::Literal::string(dir)).into()
}

fn load_from_aoc(year: u32, day: u32) -> String {
    let _ = dotenv::dotenv();

    let session = std::env::var("AOC_SESSION").expect("failed to load session id from AOC_SESSION");

    let session = HeaderValue::from_str(&format!("session={session}"))
        .expect("failed to construct session from AOC_SESSION");
    let content_type = HeaderValue::from_static("text/plain");
    let user_agent = HeaderValue::from_static("jamincan/aoc2023");

    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, session);
    headers.insert(header::CONTENT_TYPE, content_type);
    headers.insert(header::USER_AGENT, user_agent);

    let client = Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .expect("failed to build http client");

    let response = client
        .get(&format!("https://adventofcode.com/{year}/day/{day}/input"))
        .send()
        .and_then(|res| res.error_for_status())
        .unwrap_or_else(|err| panic!("failed to get input: {err}"));

    response
        .text()
        .expect("failed to read input from adventofcode.com")
}
