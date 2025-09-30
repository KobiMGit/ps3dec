
#[derive(Debug, clap::Parser)]
#[clap(
    author, version,
    about = "PS3dec Remake is a remake of the original PS3 DISC decryption tool in rust",
    long_about = "PS3Dec is a tool to decrypt PS3 Redump ISOs..."
)]
pub struct Ps3decargs {
    #[clap(help = "Path to the PS3 ISO file to decrypt.")]
    pub iso: String,

    #[clap(short = 'k', long = "dk", help = "Decryption key (32 hex).", conflicts_with = "auto")]
    pub dk: Option<String>,

    #[clap(short = 't', long = "tc", help = "Thread count.", default_value = "32")]
    pub tc: usize,

    #[clap(short, long, help = "Autodetect key from ISO name.", action = clap::ArgAction::SetTrue)]
    pub auto: bool,

    #[clap(short = 's', long = "skip", help = "Skip exit confirmation.", action = clap::ArgAction::SetTrue)]
    pub skip: bool,

    #[clap(short = 'o', long = "output-dir", help = "Output directory.")]
    pub output_dir: Option<String>,

    #[clap(short = 'n', long = "output-name", help = "Output filename (without extension).")]
    pub output_name: Option<String>,

    #[clap(long = "chunk-size", help = "Chunk size in MiB.")]
    pub chunk_size: Option<usize>,
}



pub const DEFAULT_CHUNK: Option<usize> = Some(16 * 1024 * 1024);
