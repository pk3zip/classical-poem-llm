use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "poem-trainer")]
#[command(about = "Train an n-gram model on classical poems", long_about = None)]
pub struct Args {
    /// Path to the poems directory
    #[arg(short, long, default_value = "poems")]
    pub input_dir: String,

    /// Number of tokens to use as context
    #[arg(short, long, default_value_t = 5)]
    pub context_size: usize,

    /// Path to save the trained model
    #[arg(short, long, default_value = "model.bin")]
    pub output: String,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Enable quiet output
    #[arg(short, long)]
    pub quiet: bool,
}
