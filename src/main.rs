use blake3::{Hash, Hasher};
use serde::{Deserialize, Serialize};
use serde_json;
use walkdir::WalkDir;

use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read};
use std::path::PathBuf;

const USAGE: &str = "Usage: cargo r -- <static_assets_folder>";
// Size of file chunks to read to hash contents
const FILE_CHUNK_SIZE: usize = 4096;
const LOCK_FILE_PATH: &str = "assets-lock.json";

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting asset manager...");

    let mut args = env::args();
    args.next();
    let assets_dir = args.next().expect(USAGE);

    let hashes = calculate_current_hashes(&assets_dir)?;
    emit_lockfile(&hashes, LOCK_FILE_PATH)?;

    Ok(())
}

fn calculate_current_hashes(assets_dir: &str) -> Result<HashMap<PathBuf, Hash>, Box<dyn Error>> {
    let mut entries = WalkDir::new(assets_dir)
        .into_iter()
        .map(|e| {
            e.expect("failed to parse directory entry")
                .path()
                .to_path_buf()
        })
        .filter(|p| p.is_file())
        .collect::<Vec<PathBuf>>();

    entries.sort_unstable();

    let mut entries_hash = HashMap::with_capacity(entries.len());

    for entry in entries {
        println!("Path only - {}", entry.display());
        let file_hash = hash_file_contents(&entry)?;
        println!("Path: {}, Hash: {}", entry.display(), file_hash);

        entries_hash.insert(entry, file_hash);
    }

    Ok(entries_hash)
}

fn hash_file_contents(file_path: &PathBuf) -> Result<Hash, io::Error> {
    let mut buf_reader = BufReader::new(File::open(file_path)?);
    let mut buf = [0; FILE_CHUNK_SIZE];

    let mut hasher = Hasher::new();

    if buf_reader.read(&mut buf)? > 0 {
        hasher.update(&buf);
    }

    Ok(hasher.finalize())
}

fn emit_lockfile(
    file_hashes: &HashMap<PathBuf, Hash>,
    lock_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize, Deserialize)]
    struct Asset<'a> {
        path: &'a str,
        hash: String,
    }

    let mut assets: Vec<Asset> = Vec::with_capacity(file_hashes.capacity());
    for file_hash in file_hashes {
        assets.push(Asset {
            path: file_hash
                .0
                .as_path()
                .to_str()
                .expect("failed to convert file path to str"),
            hash: file_hash.1.to_string(),
        });
    }

    let assets_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(lock_file_path)?;

    let buf_writer = BufWriter::new(assets_file);
    serde_json::to_writer(buf_writer, &assets)?;

    Ok(())
}
