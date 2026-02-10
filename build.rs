use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::Builder;
use ignore::WalkBuilder;

// Code written mainly by perplexity.ai

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let archive_path = out_dir.join("source.tar.gz");

    let mut tar_gz = Builder::new(GzEncoder::new(
        File::create(&archive_path).expect("Failed to create archive"),
        Compression::best(),
    ));

    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!("{}", project_root.display());
    
    // Use ignore crate to respect .gitignore rules (root + nested)
    let walker = WalkBuilder::new(project_root)
        .standard_filters(true)  // Respect .gitignore, global excludes
        .hidden(false)           // Do skip hidden files
        .build();

    for result in walker {
        let entry = result.expect("Failed to read entry");
        let path = entry.path();
        
        if entry.file_type().map_or(false, |ft| ft.is_file()) 
        {
            let rel_path = path.strip_prefix(project_root).unwrap();
            if 
                rel_path.starts_with(project_root.join("target"))
                ||
                rel_path.starts_with(".git/")
            {
                // TODO compose.yml null gets included somehow??
                // TODO enclosing dir
                continue;
            }
            eprintln!("Including in source archive: {}", path.display());
            tar_gz.append_path_with_name(rel_path, rel_path).expect("Failed to add to tar");
        }
    }
    
    tar_gz.finish().expect("Failed to finalize archive");
    println!("cargo:rustc-env=SOURCE_ARCHIVE={}", archive_path.display());
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=.gitignore");  // Rebuild on gitignore changes
}
