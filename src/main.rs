use clap::Parser;
use kwic_rust::{process_kwic, read_lines, load_stop_words, KwicError};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input_file: PathBuf,
    
    #[arg(short, long)]
    stop_words: Option<PathBuf>,
    
    #[arg(short, long)]
    case_sensitive: bool,
    
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), KwicError> {
    let args = Args::parse();
    
    let lines = read_lines(&args.input_file)?;
    
    let stop_words = load_stop_words(args.stop_words.as_ref())?;
    
    let results = process_kwic(&lines, &stop_words, args.case_sensitive);
    
    if let Some(output_path) = args.output {
        std::fs::write(
            output_path,
            results.iter()
                .map(|(kw, ctx)| format!("{}: {}\n", kw, ctx))
                .collect::<String>(),
        )?;
    } else {
        for (keyword, context) in results {
            println!("{}: {}", keyword, context);
        }
    }
    
    Ok(())
}