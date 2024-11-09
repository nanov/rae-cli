use std::env;
use std::str;

use scraper::selectable::Selectable;
use scraper::ElementRef;
use scraper::{Html, Selector};
use reqwest;
use html2text;
use html2text::config;
use html2text::render::RichAnnotation;
use termion::color::*;

use std::cell::LazyCell;

use clap::{arg, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DIV_RESULTS_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse(r#"div[id="resultados"]"#).unwrap() });
const RESULT_OR_SUGGESTION_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse(r#"article, div[class="item-list"]"#).unwrap() });
const OPTIONS_SELECTOR: LazyCell<Selector> = LazyCell::new(|| { Selector::parse("a").unwrap() });

// TODO: either work with adapted colours or get rid
fn default_colour_map(
    annotations: &[RichAnnotation],
    s: &str,
) -> String {
    use RichAnnotation::*;
    // Explicit CSS colours override any other colours
    let mut start = Vec::new();
    let mut finish = Vec::new();
    for annotation in annotations.iter() {
        match annotation {
            Default => {}
            Link(_) => {
                start.push(format!("{}", termion::style::Underline));
                finish.push(format!("{}", termion::style::Reset));
            }
            Colour(c) => {
                    start.push(format!("{}", termion::color::Fg(Rgb(c.r, c.g, c.b))));
                    finish.push(format!("{}", Fg(Reset)));
            }
            BgColour(_) => {
            }
            _ => {}
        }
    }
    // Reverse the finish sequences
    finish.reverse();
    let mut result = start.join("");
    result.push_str(s);
    for s in finish {
        result.push_str(&s);
    }
    result
}

fn imprimir_palabra(definicion_html: ElementRef) {
    
    let co = config::rich(); // .use_doc_css();
    let mut redader = std::io::Cursor::new(definicion_html.inner_html());
    let d = co.coloured(&mut redader, 100, default_colour_map).unwrap();

    print!("{}", d);
}

fn print_options(options_list: ElementRef) {
    use inquire::Select;
    
    let options_list = options_list.select(&*&OPTIONS_SELECTOR).filter_map(|x| x.text().next()).collect::<Vec<&str>>();
    if options_list.len() == 1 {

        println!("La palabra hacar no está en el Diccionario. Las entradas que se muestran a continuación podrían estar relacionadas: {}", options_list[0]);
        println!();
        buschar_palabra(options_list[0]);
        return
    }


    let ans = Select::new("La palabra hacar no está en el Diccionario. Las entradas que se muestran a continuación podrían estar relacionadas:", options_list).prompt();

    match ans {
        Ok(choice) => buschar_palabra(choice),
        Err(_) => println!("There was an error, please try again"),
    }
}

fn print_definition_or_options(word: &str, page_core: ElementRef) {
    match  page_core.select(&*RESULT_OR_SUGGESTION_SELECTOR).next() {
        Some(w) => match w.value().name() {
                     "article" => imprimir_palabra(page_core),
                      "div" => print_options(w),
                        _ => println!("La palabra {} no está en el Diccionario.", word),
                },
        _ => println!("La palabra {} no está en el Diccionario.", word),
    }
}


fn buschar_palabra(palabra: &str){
    let client = reqwest::blocking::Client::new();
    let pagina = client.get(format!("https://dle.rae.es/{}", palabra)).header("User-Agent", "mitk").send().expect("no url");
    
    let raw_page = pagina.text().expect("stupid");
    let dom_fragment = Html::parse_document(&raw_page);


    match dom_fragment.select(&*DIV_RESULTS_SELECTOR).next() {
        Some(c) => {
            print_definition_or_options(palabra, c);
        },
        _ => println!("La palabra {} no está en el Diccionario.", palabra),
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

    let p = matches.get_one::<String>("palabra").expect("unreachable").clone();
    buschar_palabra(&p);
}
