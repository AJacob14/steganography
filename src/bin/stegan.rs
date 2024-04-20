use clap::{Args, Parser, Subcommand};
use steganography::lsb;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
}

#[derive(Args, Debug)]
struct EncodeArgs {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    message: String,
}

#[derive(Args, Debug)]
struct DecodeArgs {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
}


fn main(){
    let args = Cli::parse();
    match args.subcommand {
        SubCommands::Encode(args) => {
            let result = lsb::encode(&args.input, &args.message, &args.output);
            if result.is_err() {
                println!("{}", result.unwrap_err());
            }
        }
        SubCommands::Decode(args) => {
            let result = lsb::decode(&args.input, &args.output);
            if result.is_err() {
                println!("{}", result.unwrap_err());
            }
        }
    }
}