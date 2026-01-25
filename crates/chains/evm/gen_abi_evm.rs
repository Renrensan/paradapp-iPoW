use ethers_contract_abigen::Abigen;
use std::path::{Path, PathBuf};

fn main() {
    let abi_path = Path::new("abi/ParadappConvert.abi");
    let out_dir = PathBuf::from("src/bindings");
    let out_file = out_dir.join("paradapp_convert.rs");

    // Tell Cargo to rerun if ABI changes
    println!("cargo:rerun-if-changed={}", abi_path.display());

    // Ensure output directory exists
    std::fs::create_dir_all(&out_dir).unwrap();

    // Generate if bindings file is missing OR ABI changed (Cargo triggers rerun)
    if !out_file.exists() {
        println!("Generating Rust EVM bindings from ABI…");

        Abigen::new("ParadappConvert", abi_path.to_str().unwrap())
            .expect("failed to create Abigen")
            .generate()
            .expect("failed to generate bindings")
            .write_to_file(&out_file)
            .expect("failed to write bindings");

        let mut contents =
            std::fs::read_to_string(&out_file).expect("failed to read generated bindings");

        if !contents.contains("clippy::module_inception") {
            contents = format!(
                r#"// @generated — DO NOT EDIT
#![allow(clippy::module_inception)]

{}
"#,
                contents
            );

            std::fs::write(&out_file, contents).expect("failed to write clippy header");
        }

        println!("Bindings written to {}", out_file.display());
    } else {
        println!("Bindings already exist, skipping generation.");
    }
}
