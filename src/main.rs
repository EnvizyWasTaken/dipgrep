mod args;
mod logic;
use crate::logic::read_path;
use args::Algorithm;
use args::Args;
use clap::Parser;
use colored::Colorize;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let term = match &args.term {
        Some(t) => t,
        None => {
            eprintln!("Error: please provide a search term with -t");
            return Err("No search term provided".into());
        }
    };
    let is_regex = matches!(args.algorithm, args::Algorithm::Regex);
    let algorithm_fn = match args.algorithm {
        Algorithm::Linear => logic::match_pattern,
        Algorithm::Insensitive => logic::match_pattern_insensitive,
        Algorithm::Exact => logic::match_pattern_exact,
        Algorithm::Regex => logic::match_pattern_regex,
        Algorithm::BoyerMoore => logic::match_pattern_boyer_moore,
    };
    let re = if is_regex {
        Regex::new(term).ok()
    } else {
        None
    };
    if std::path::Path::new(&args.path).is_dir() {
        let results = logic::search_directory(&args.path, term, args.recursive, algorithm_fn);
        for (filename, line_num, line_content) in results {
            if let Some(ref r) = re {
                print!("{} ", filename.cyan().bold());
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
                if let Some(i) = pos {
                    let start = i.saturating_sub(2);
                    let end = (i + 3).min(words.len());
                    let context: Vec<&str> = words[start..end].to_vec();
                    print!("{} ", filename.cyan().bold());
                    for word in &context {
                        if word.to_lowercase().contains(&term.to_lowercase()) {
                            print!("{} ", word.green().bold());
                        } else {
                            print!("{} ", word.dimmed());
                        }
                    }
                    println!("\t{}", format!("(line {})", line_num).white().bold());
                }
            }
        }
    } else {
        let contents = match read_path(&args.path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e.into());
            }
        };
        let results = algorithm_fn(term, &contents);
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
                if let Some(i) = pos {
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
            }
        }
    }
    Ok(())
}
