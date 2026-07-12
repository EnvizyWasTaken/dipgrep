use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Algorithm {
    BoyerMoore,
    Linear,
    Insensitive,
    Exact,
    Regex,
}

#[derive(Parser)]
pub struct Args {
    #[arg(short = 't', long)]
    pub term: Option<String>,

    #[arg(short = 'f', long = "find-file")]
    pub file_search: Option<String>,

    #[arg(short = 'p', long)]
    pub path: String,

    #[arg(short = 'a', long, default_value = "boyer-moore")]
    pub algorithm: Algorithm,

    #[arg(short = 'r', long, default_value_t = false)]
    pub recursive: bool,
}
