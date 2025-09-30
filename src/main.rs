
use clap::Parser;
use log::{ error, info};
use std::io::{self};
use std::path::Path;
use std::{env,};
use std::process::exit;
pub mod args;
pub mod autodetect;
pub mod utils;


pub use utils::{
    read_exact_at, write_all_at, extract_regions, generate_iv, is_encrypted,
    decrypt_sector, setup_logging,
};
use crate::args::{Ps3decargs, DEFAULT_CHUNK};
use crate::autodetect::detect_key;
use crate::utils::key_validation;

// Either drag and drop which will auto-detect key, OR launch through CLI.
fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "--help" {
        let _ = Ps3decargs::parse_from(&["", "--help"]);
        return Ok(());
    }

    utils::setup_logging().expect("Failed to setup logging");

    let mut skip_wait = false;

    if args.len() == 2 {
        // drag & drop / single-arg path
        let raw = &args[1];
        let dragdrop = raw.trim_matches(|c| c == '"' || c == '\''); 
        let path = Path::new(dragdrop);

        let is_iso = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("iso"))
            .unwrap_or(false);

        if is_iso {
            let filename = path.file_stem().and_then(|f| f.to_str()).unwrap_or("");

            info!("ISO path: {}", path.display());
            if let Ok(c) = path.canonicalize() {
                info!("Canonical path: {}", c.display());
            }
            info!("Filename used for key lookup: {}", filename);

            match detect_key(filename.to_string()) {
                Ok(Some(key)) => {
                    info!("Detected key for {}: {}", filename, key);
                    ps3dec::decrypt(dragdrop.to_string(), &key, num_cpus::get(), None, None,DEFAULT_CHUNK)?;
                }
                _ => error!("No key found for {}", filename),
            }
        } else {
            error!("The file must be an ISO file with .iso extension");
        }
    } else if args.len() > 1 {
        let ps3_args = Ps3decargs::parse();
        let p = Path::new(&ps3_args.iso);
        let is_iso = p
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("iso"))
            .unwrap_or(false);
        if !is_iso {
            error!("The file must be an ISO file with .iso extension");
            return Ok(());
        }

        let filename = p.file_stem().and_then(|f| f.to_str()).unwrap_or("");
        info!("ISO path: {}", p.display());
        if let Ok(c) = p.canonicalize() {
            info!("Canonical path: {}", c.display());
        }
        info!("Filename used for key lookup: {}", filename);

        if ps3_args.auto {
            if let Ok(Some(key)) = detect_key(filename.to_string()) {
                info!("Auto-detected key for {}: {}", filename, key);
                ps3dec::decrypt(
                    ps3_args.iso,
                    &key,
                    ps3_args.tc,
                    ps3_args.output_dir,
                    ps3_args.output_name,
                    ps3_args.chunk_size
                )?;
            } else {
                error!("No key could be auto-detected for {}", filename);
            }
        } else if let Some(dk) = ps3_args.dk {
            if key_validation(&dk) {
                info!("Using provided decryption key: {}", dk);
                ps3dec::decrypt(
                    ps3_args.iso,
                    &dk,
                    ps3_args.tc,
                    ps3_args.output_dir,
                    ps3_args.output_name,
                    ps3_args.chunk_size,
                )?;
            } else {
                error!("Error: Invalid PS3 decryption key format.");
            }
        } else {
            error!("Error: Decryption key is required unless '--auto' is specified.");
        }

        skip_wait = ps3_args.skip;
    } else {
        error!("Please provide an ISO file path. Use --help for more information.");
        exit(0)
    }

    if !skip_wait {
        info!("Job done, press any button to exit...");
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");
        info!("Ciao!");
    }

    Ok(())
}