use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let archive_path = out_dir.join("source.tar.gz");

    let mut tar_gz = Builder::new(GzEncoder::new(
        fs::File::create(&archive_path).expect("Failed to create archive"),
        Compression::best(),
    ));

    // Walk project src dir (exclude target, .git, etc.)
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for entry in WalkDir::new(project_root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && !path.starts_with(project_root.join("target"))
            && !path.starts_with(project_root.join(".git"))
            && !path.to_str().unwrap().ends_with("build.rs")  // Optional: exclude this script
        {
            let rel_path = path.strip_prefix(project_root).unwrap();
            tar_gz.append_path_with_name(path, rel_path).expect("Failed to add to tar");
        }
    }
    tar_gz.finish().expect("Failed to finalize archive");

    // Print path for embedding macro
    println!("cargo:rustc-env=SOURCE_ARCHIVE={}", archive_path.display());
}
