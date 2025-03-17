use clap::{Args, Parser, Subcommand};
use jawir::{crack::brute_force_jwt, generate::create_jwt};

#[derive(Parser)]
#[command(name = "jawir")]
#[command(about = "crack and generate jwt with ease")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crack JWT
    Crack(CrackArgs),
    /// Generate JWT with given secret
    Generate(GenerateArgs),
}

#[derive(Args)]
struct GenerateArgs {
    #[arg(short, long)]
    json: bool,
    #[arg(short = 'H', long)]
    header: String,
    #[arg(short = 'P', long)]
    payload: String,
    #[arg(short = 'S', long)]
    secret: String,
}

#[derive(Args)]
struct CrackArgs {
    #[arg(short, long)]
    jwt: String,
    #[arg(short, long)]
    wordlist: String,
    #[arg(short, long, default_value_t = 1)]
    thread: usize,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(args) => {
            let generated_jwt = create_jwt(&args.header, &args.payload, &args.secret, !args.json);
            println!("Generated JWT: {}", generated_jwt);
        }
        Commands::Crack(args) => {
            if args.thread == 0 {
                println!("thread count must > 0");
                return;
            }

            match brute_force_jwt(&args.jwt, &args.wordlist, args.thread) {
                Some(a) => println!("{a}"),
                _ => println!("no secret found"),
            }
        }
    }
}
