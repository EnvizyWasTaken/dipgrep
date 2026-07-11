mod args;
mod logic;
use crate::logic::read_path;
use args::Args;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

fn main() {
    let args = Args::parse();
    let term = match &args.term {
        Some(t) => t,
        None => {
            eprintln!("Error: please provide a search term with -t");
            return;
        }
    };
    let is_regex = args.algorithm == "regex";
    let algorithm = match args.algorithm.as_str() {
        "insensitive" => logic::match_pattern_insensitive,
        "exact" => logic::match_pattern_exact,
        "regex" => logic::match_pattern_regex,
        _ => logic::match_pattern,
    };
    let re = if is_regex {
        Regex::new(term).ok()
    } else {
        None
    };
    if std::path::Path::new(&args.path).is_dir() {
        let results = logic::search_directory(&args.path, term, args.recursive, algorithm);
        for (filename, line_num, line_content) in results {
            print!("{} ", filename.cyan().bold());
            if let Some(ref r) = re {
                let words: Vec<&str> = line_content.split_whitespace().collect();
                for word in &words {
                    if r.is_match(word) {
                        print!("{} ", word.green().bold());
                    } else {
                        print!("{} ", word.dimmed());
                    }
                }
                println!("\t{}", format!("(line {})", line_num).white().bold());
            } else {
                let words: Vec<&str> = line_content.split_whitespace().collect();
                let pos = words
                    .iter()
                    .position(|w| w.to_lowercase().contains(&term.to_lowercase()));
                match pos {
                    Some(i) => {
                        let start = i.saturating_sub(2);
                        let end = (i + 3).min(words.len());
                        let context: Vec<&str> = words[start..end].to_vec();
                        for word in &context {
                            if word.to_lowercase().contains(&term.to_lowercase()) {
                                print!("{} ", word.green().bold());
                            } else {
                                print!("{} ", word.dimmed());
                            }
                        }
                        println!("\t{}", format!("(line {})", line_num).white().bold());
                    }
                    None => {}
                }
            }
        }
    } else {
        let contents = match read_path(&args.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };
        let results = algorithm(term, &contents);
        for line in results {
            if let Some(ref r) = re {
                let words: Vec<&str> = line.1.split_whitespace().collect();
                for word in &words {
                    if r.is_match(word) {
                        print!("{} ", word.green().bold());
                    } else {
                        print!("{} ", word.dimmed());
                    }
                }
                println!(
                    "\t{}",
                    format!("(was found on line {})", line.0).white().bold()
                );
            } else {
                let words: Vec<&str> = line.1.split_whitespace().collect();
                let pos = words
                    .iter()
                    .position(|w| w.to_lowercase().contains(&term.to_lowercase()));
                match pos {
                    Some(i) => {
                        let start = i.saturating_sub(2);
                        let end = (i + 3).min(words.len());
                        let context: Vec<&str> = words[start..end].to_vec();
                        for word in &context {
                            if word.to_lowercase().contains(&term.to_lowercase()) {
                                print!("{} ", word.green().bold());
                            } else {
                                print!("{} ", word.dimmed());
                            }
                        }
                        println!(
                            "\t{}",
                            format!("(was found on line {})", line.0).white().bold()
                        );
                    }
                    None => {}
                }
            }
        }
    }
}
