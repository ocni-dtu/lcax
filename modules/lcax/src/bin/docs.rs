#![cfg(feature = "docs")]
use rustdoc_md::rustdoc_json_to_markdown;
use rustdoc_types::Crate;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let target_dir = root_dir.join("target/doc");
    let api_doc_dir = root_dir.join("docs/src/pages/ReferencePages/content/api/rust");
    let files = vec![
        "lcax",
        "lcax_calculation",
        "lcax_convert",
        "lcax_core",
        "lcax_models",
        "lcax_validation",
    ];
    for file in files {
        let json_path = target_dir.join(file.to_owned() + ".json");
        let data: Crate = serde_json::from_reader(fs::File::open(json_path)?)?;

        let frontmatter = format!(
            "---\ntitle: {} API Reference\ndescription: Rust - API Reference\n---\n",
            file
        );
        let markdown = format!("{}{}", frontmatter, rustdoc_json_to_markdown(data));

        // Save the Markdown file
        let output = api_doc_dir.join(file.to_owned() + ".md");
        fs::write(output, markdown)?;
        println!("{} documentation converted successfully!", file);
    }
    println!("Documentation converted successfully!");

    Ok(())
}
