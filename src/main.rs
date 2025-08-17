mod calculate;
mod generate;
mod misc;

use clap::{Parser, Subcommand, arg, command};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Generate {
        #[arg(short, long, num_args = 3..=4, default_values_t = [0, 0, 0, 0])]
        bg_color: Vec<u8>,
        #[arg(short, long, num_args = 3..=4, default_values_t = [0, 0, 0, 255])]
        fg_color: Vec<u8>,
        #[arg(long)]
        font: String,
        #[arg(long, num_args = 1..=2, default_values_t = [20, 30])]
        font_size: Vec<usize>,
        #[arg(default_value = "This image contains %s opaque pixels.")]
        text: String,
    },
    Verify {
        #[arg(short, long, num_args = 3..=4, default_values_t = [0, 0, 0, 255])]
        fg_color: Vec<u8>,
        file: String,
    },
}

fn main() {
    match Cli::parse().command {
        Commands::Generate {
            bg_color,
            fg_color,
            font,
            font_size,
            text,
        } => {
            let result = generate::generate_image(&text, &font, &font_size, &fg_color, &bg_color);
            match result {
                Err(e) => eprintln!("{}", e),
                Ok(_) => {}
            }
        }
        Commands::Verify { fg_color, file } => {
            let pixels = calculate::pixels_in_image(&file, &fg_color);
            match pixels {
                Err(e) => eprintln!("{}", e),
                Ok(v) => println!("{}", v),
            }
        }
    }
}
