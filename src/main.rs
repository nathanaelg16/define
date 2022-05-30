use reqwest::blocking::Response;
use reqwest::blocking::Request;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::collections::HashMap;
use std::result::Result;
use text_colorizer::*;

const API_URL: &str = "http://api.wordnik.com/v4/word.json";

fn main() -> Result<(), confy::ConfyError> {
    let config : Config = confy::load("define")?;
    let args = parse_args();
    println!("{:?}\n\n", args);
    define(args.word, args.limit, args.part_of_speech, args.include_related, args.dictionaries, args.use_canonical, &config.api_key);
    Ok(())
}

fn define(word: String, limit: u8, part_of_speech: Option<String>, include_related: bool, dictionaries: Vec<String>, use_canonical: bool, api_key: &String) {
    let client = reqwest::blocking::Client::new();
    let res = match client.get(format!("{}/{}/{}", API_URL, word, "definitions"))
        .query(&[("api_key", api_key), ("includeRelated", &(include_related.to_string())), ("useCanonical", &(use_canonical.to_string()))])
        .query(&[("sourceDictionaries", dictionaries.join(","))])
        .query(&[("limit", limit)])
        .send() {
            Ok(response) => response.json::<serde_json::Value>().unwrap(),
            Err(error) => {
                eprintln!("{} {}", "Error:".red().bold(), error);
                std::process::exit(1);
            }
        };
    
    let res_arr = res.as_array().unwrap();
    
    let mut definitions : HashMap<String, Vec<Definition>> = HashMap::new();
    for res in res_arr {
        let examples = Vec::new();
        let definition = Definition {
            word: res["word"].to_string(),
            definition: res["text"].to_string(),
            part_of_speech: res["partOfSpeech"].to_string(),
            attribution_text: res["attributionText"].to_string(),
            dictionary: res["sourceDictionary"].to_string(),
            examples: examples
        };
        if definitions.contains_key(&res["word"].to_string()) {
            let vector = definitions.get_mut(&res["word"].to_string()).unwrap();
            vector.push(definition);
        } else {
            let mut vector : Vec<Definition> = Vec::new();
            vector.push(definition);
            definitions.insert(res["word"].to_string(), vector);
        }
    }

    for (word, definition_vec) in definitions {
        println!("{}\n\n", word.blue());
        for definition in definition_vec {
            println!("{} - {}\n\t* {}\n", definition.part_of_speech, definition.attribution_text.yellow().italic(), definition.definition)
        }
    }
    
    //println!("{} ({})\n\n{}\n\t* {}", first_res["word"], first_res["attributionText"], first_res["partOfSpeech"].to_string().italic(), first_res["text"]);
}

fn print_usage() {
    let usage = r"
Usage:
    define <word> [OPTIONS]

Options:
    -D --dictionary --dictionaries  [...]    Source dictionaries to return definitions from, separated by a space
    -S --partOfSpeech               [...]    The part of speech of the word whose definition is requested
    -L --limit                      [...]    Maximum number of results to return
    -A --audio                               Request an audio pronunciation of the word
    -R --includeRelated                      Request related words with definitions
    -C --useCanonical                        Tries to return the correct word root (e.g. 'cats' -> 'cat')
    -E --etymology                           Request etymology data
    -X --examples                            Request examples for the word
    -F --frequency                  [...]    Request word usage over time
    -H --hyphenation                         Request syllable information for the word
    -P --pronunciation              [...]    Request text pronunciation for the word with the specified pronunciation type
    -T --thesaurus                           Request synonym and antonym information for the word
    -U --usage --help                        Display this usage guide
";

    eprintln!(
        "{} - look up words in the dictionary of your choice",
        "define".green()
    );
    eprintln!("{}", usage)
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected at least 1, got 0.",
            "Error:".red().bold()
        );
        std::process::exit(1);
    }

    let word = args[0].clone();
    if word.contains('-') {
        print_usage();
        eprintln!(
            "{} the first argument must be a word",
            "Error:".red().bold()
        );
        std::process::exit(1);
    }

    let mut part_of_speech: Option<String> = None;
    let mut dictionaries: Vec<String> = Vec::new();
    let mut limit: u8 = 5;
    let mut audio: bool = false;
    let mut include_related: bool = false;
    let mut use_canonical: bool = false;
    let mut etymologies: bool = false;
    let mut examples: bool = false;
    let mut frequency: bool = false;
    let mut start_year: Option<u16> = None;
    let mut end_year: Option<u16> = None;
    let mut hyphenation: bool = false;
    let mut pronunciation: bool = false;
    let mut type_format: String = String::from("ahd-5");
    let mut thesaurus: bool = false;

    let mut current_op : Option<String> = None;
    for mut i in 1..args.len() {
        let arg = &args[i];
        match &current_op {
            Some(op) => {
                if arg.contains('-')  {
                    if matches!(op.as_str(), "d"|"dictionary"|"dictionaries"|"p"|"pronunciation") {
                        current_op = None;
                        i = i - 1;
                    } else {
                        print_usage();
                        eprintln!("{} unable to parse arguments", "Error:".red().bold());
                        std::process::exit(1);
                    }
                } else {
                    match op.as_str() {
                        "d"|"dictionary"|"dictionaries" => dictionaries.push(arg.clone()),
                        "l"|"limit" => {
                            limit = arg.parse::<u8>().unwrap();
                            current_op = None;
                        },
                        "f"|"frequency" => {
                            if let Some(_) = start_year {
                                end_year = Some(arg.parse::<u16>().unwrap());
                                current_op = None;
                            } else {
                                start_year = Some(arg.parse::<u16>().unwrap());
                            }
                        },
                        "s"|"partofspeech" => {
                            let supported_pos = ["noun", "adjective", "verb", "adverb", "interjection", "pronoun", "preposition", "abbreviation", "affix", "article", "auxiliary-verb", "conjunction", "definite-article", "family-name", "given-name", "idiom", "imperative", "noun-plural", "noun-posessive", "past-participle", "phrasal-prefix", "proper-noun", "proper-noun-plural", "proper-noun-posessive", "suffix", "verb-intransitive", "verb-transitive"];
                            if supported_pos.iter().any(|&s| s == arg) {
                                part_of_speech = Some(arg.clone());
                                current_op = None
                            } else {
                                eprintln!("{} Unsupported part of speech specified\n{} {:?}", "Error:".red().bold(), "Supported formats:".green().bold(), supported_pos);
                                std::process::exit(1);
                            }
                        },
                        "p"|"pronunciation" => {
                            let supported_formats = ["ahd-5", "arpabet", "gcide-diacritical", "IPA"];
                            if supported_formats.iter().any(|&s| s == arg) {
                                type_format = arg.clone();
                                current_op = None;
                            } else {
                                eprintln!("{} Unsupported pronunciation type format\n{} {:?}", "Error:".red().bold(), "Supported formats:".green().bold(), supported_formats);
                                std::process::exit(1);
                            }
                        },
                        _ => {
                            print_usage();
                            eprintln!("{} unable to parse arguments; unknown operator '{}'", "Error:".red().bold(), op.yellow());
                            std::process::exit(1);
                        }
                    }
                }
            },
            None => {
                if arg.contains("-") {
                    let arg = arg.trim_start_matches('-').to_lowercase();
                    match arg.as_str() {
                        "a" | "audio" => {
                            audio = true;
                            current_op = None;
                        }
                        "r" | "includerelated" => {
                            include_related = true;
                            current_op = None;
                        }
                        "c" | "usecanonical" => {
                            use_canonical = true;
                            current_op = None;
                        }
                        "e" | "etymology" => {
                            etymologies = true;
                            current_op = None;
                        }
                        "x" | "examples" => {
                            examples = true;
                            current_op = None;
                        }
                        "f" | "frequency" => {
                            frequency = true;
                            current_op = Some(arg.clone());
                        }
                        "h" | "hyphenation" => {
                            hyphenation = true;
                            current_op = None;
                        }
                        "p" | "pronunciation" => {
                            pronunciation = true;
                            current_op = Some(arg.clone());
                        }
                        "t" | "thesaurus" => {
                            thesaurus = true;
                            current_op = None;
                        }
                        "u" | "usage" | "help" => {
                            print_usage();
                            std::process::exit(0);
                        }
                        _ => current_op = Some(arg.clone()),
                    }
                } else {
                    print_usage();
                    eprintln!("{} unable to parse arguments", "Error:".red().bold());
                    std::process::exit(1);
                }
            }
        }
    }

    if dictionaries.len() == 0 {
        dictionaries.push(String::from("ahd-5"));
    }

    Arguments { 
        word: word,
        part_of_speech: part_of_speech,
        dictionaries: dictionaries,
        limit: limit,
        audio: audio,
        include_related: include_related,
        use_canonical: use_canonical,
        etymologies: etymologies,
        examples: examples,
        frequency: frequency,
        start_year: start_year,
        end_year: end_year,
        hyphenation: hyphenation,
        pronunciation: pronunciation,
        type_format: type_format,
        thesaurus: thesaurus
    }
}

#[derive(Debug)]
struct Arguments {
    word: String,
    part_of_speech: Option<String>,
    dictionaries: Vec<String>,
    limit: u8,
    audio: bool,
    include_related: bool,
    use_canonical: bool,
    etymologies: bool,
    examples: bool,
    frequency: bool,
    start_year: Option<u16>,
    end_year: Option<u16>,
    hyphenation: bool,
    pronunciation: bool,
    type_format: String,
    thesaurus: bool,
}

struct Definition {
    word: String,
    definition: String,
    part_of_speech: String,
    attribution_text: String,
    dictionary: String,
    examples: Vec<String>
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { api_key: "".into() } }
}

#[derive(Serialize, Deserialize)]
struct Config {
    api_key: String
}