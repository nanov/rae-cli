use std::{env, usize};
use std::fmt::Display;
use const_format::concatcp;
use inquire::InquireError;
use reqwest::header::USER_AGENT;
use std::str;

use scraper::selectable::Selectable;
use scraper::ElementRef;
use scraper::{Html, Selector};
use reqwest::{self, StatusCode};
use html2text;
use html2text::config;

use std::cell::LazyCell;

use clap::{arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const CLI_USER_AGENT: &str = concatcp!(NAME, "/", VERSION);
const DIV_RESULTS_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse(r#"div[id="resultados"]"#).unwrap() });
const RESULT_OR_SUGGESTION_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse(r#"article, div[class="item-list"]"#).unwrap() });
const OPTIONS_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse("a").unwrap() });

#[derive(Debug)]
enum RaeError {
    RequestError(reqwest::Error),
    ResponseError(StatusCode),
    HtmlParseError(html2text::Error),
    SelectError(InquireError),
    UnexpectedSiteStructure
}

enum RaeSuccess {
    Definicion(String),
    NoEncontrado
}

type RaeResult = std::result::Result<RaeSuccess, RaeError>;

impl From<reqwest::Error> for RaeError {
    fn from(r_error: reqwest::Error) -> Self {
        Self::RequestError(r_error)
    }
}

impl From<StatusCode> for RaeError {
    fn from(s_code: StatusCode) -> Self {
        Self::ResponseError(s_code)
    }
    
}

impl From<html2text::Error> for RaeError {
    fn from(s_code: html2text::Error) -> Self {
        Self::HtmlParseError(s_code)
    }
    
}

impl From<InquireError> for RaeError {
    fn from(s_code: InquireError) -> Self {
        Self::SelectError(s_code)
    }
    
}

impl Display for RaeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RaeError::UnexpectedSiteStructure => write!(f, "[Error]: Unexpected site structre. latest version? something changed? please report :/"),
            RaeError::RequestError(r_e) => write!(f, "[Error]: {}, internet avialable? site down?", r_e),
            RaeError::ResponseError(r_e) => write!(f, "[Error]: {}, latest version?", r_e),
            RaeError::SelectError(s_e) => write!(f, "[Error]: {}", s_e),
            RaeError::HtmlParseError(h_e) => write!(f, "[Error]: {}", h_e),
        }
    }
}

impl std::error::Error for RaeError {}

fn extract_definition(definicion_html: ElementRef) -> RaeResult {
    let width = match termsize::get() {
        Some(s) => s.cols,
        _ => 80
    };

    let d = config::rich()
        .string_from_read(definicion_html.inner_html().as_bytes(), usize::from(width))?;

    Ok(RaeSuccess::Definicion(d))
}

fn handle_suggestions(options_list: ElementRef) -> RaeResult {
    use inquire::Select;
    
    let suggestion_list = options_list
        .select(&*&OPTIONS_SELECTOR)
        .filter_map(|x| x.text().next())
        .collect::<Vec<&str>>();

    match suggestion_list.len() {
        1 =>  {
            println!("La palabra hacar no está en el Diccionario. Las entradas que se muestran a continuación podrían estar relacionadas: {}", suggestion_list[0]);
            println!();
            buschar_palabra(suggestion_list[0])
        },
        0 => Ok(RaeSuccess::NoEncontrado), // hihglky unlikley
        _ => buschar_palabra(Select::new("La palabra hacar no está en el Diccionario. Las entradas que se muestran a continuación podrían estar relacionadas:", suggestion_list).prompt()?),
    }
}

fn try_get_definition(page_core: ElementRef) -> RaeResult {
    match  page_core.select(&*RESULT_OR_SUGGESTION_SELECTOR).next() {
        Some(w) => match w.value().name() {
                     "article" => extract_definition(page_core),
                      "div" => handle_suggestions(w),
                     _ => Ok(RaeSuccess::NoEncontrado),
                },
        _ => Ok(RaeSuccess::NoEncontrado),
    }
}


fn buschar_palabra(palabra: &str) -> RaeResult {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://dle.rae.es/{}", palabra);
    println!("Datos de: {}", url);
    let response = client
        .get(url)
        .header(USER_AGENT, CLI_USER_AGENT)
        .send()?;


    if !response.status().is_success() {
        Err(RaeError::ResponseError(response.status()))
    } else { // I hate it that i have to use else here
        let raw_page = response.text()?;
        let dom_fragment = Html::parse_document(&raw_page);

        match dom_fragment.select(&*DIV_RESULTS_SELECTOR).next() {
            Some(c) => try_get_definition(c),
            _ => Err(RaeError::UnexpectedSiteStructure),
        }
    }
}


fn main() {
    let matches = Command::new(NAME)
        .arg_required_else_help(true)
        .name(NAME)
        .version(VERSION)
        .about("buschar palabras en real Real Academia Española.")
        .arg(arg!([palabra] "palabra para buschar").required(true))
        .get_matches();

    let p = matches
        .get_one::<String>("palabra")
        .unwrap(); // required so unwrap is safe
    
    match buschar_palabra(&p) {
        Ok(s) => match s {
            RaeSuccess::Definicion(d) => println!("{}", d),
            RaeSuccess::NoEncontrado => println!("La palabra {} no está en el Diccionario.", p)
        },
        Err(e) => eprintln!("{}", e),
    }
}
