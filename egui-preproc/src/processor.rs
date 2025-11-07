use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::UNIX_EPOCH;

use anyhow::Error;
use chrono::{DateTime, Local, Utc};
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use tracing::{debug, info};

/// A no-op preprocessor.
pub struct EguiPreproc;

impl EguiPreproc {
    pub fn new() -> EguiPreproc {
        EguiPreproc
    }
}

impl Preprocessor for EguiPreproc {
    fn name(&self) -> &str {
        "egui-preproc"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(cfg) = ctx.config.get_preprocessor(self.name()) {
            if cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        book.for_each_mut(|item| match item {
            mdbook::BookItem::Chapter(chapter) => process_chapter(chapter),
            mdbook::BookItem::Separator => (),
            mdbook::BookItem::PartTitle(_) => (),
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn process_chapter(chapter: &mut Chapter) {
    if chapter.is_draft_chapter() {
        return;
    }

    debug!("Chapter '{}' processing now", chapter.name);

    let chapter_last_modified = {
        let mut total_path = PathBuf::new();
        total_path.push("src");
        total_path.extend(
            chapter
                .source_path
                .as_ref()
                .expect("Chapter should have a source file"),
        );
        last_modified_of(total_path)
    };

    let re = Regex::new(r"\{\{sample: (.*)\}\}").expect("Regex should compile");
    chapter.content = re
        .replace_all(&chapter.content, |caps: &regex::Captures| {
            let sample_name = &caps[1];
            debug!("Sample: '{sample_name}' processing now");
            if should_compile(sample_name, &chapter_last_modified) {
                info!(
                    "Compiling sample '{}' in chapter '{}'",
                    sample_name, chapter.name
                );
                Command::new("cargo")
                    .args([
                        "build",
                        "--manifest-path",
                        "./rust/Cargo.toml",
                        "--bin",
                        sample_name,
                        "--release",
                        "--target",
                        "wasm32-unknown-unknown",
                    ])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Cargo build failed to start");

                info!("Running wasm-bindgen");

                Command::new("wasm-bindgen")
                    .args([
                        &format!("./rust/target/wasm32-unknown-unknown/release/{sample_name}.wasm"),
                        "--target",
                        "web",
                        "--out-dir",
                        "./src/wasm",
                        "--no-typescript",
                    ])
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .output()
                    .expect("Cannot run wasm-bindgen");
                info!("Done running wasm-bindgen");
            }
            format!(
                "
<canvas id='{sample_name}' class='egui-sample'></canvas>
<script type='module'>
    import init from '/wasm/{sample_name}.js';
    async function run(){{
        await init();
    }}
    run();
</script>"
            )
        })
        .to_string();
}

fn should_compile(sample_name: &str, chapter_last_modified: &DateTime<Utc>) -> bool {
    let output_path = PathBuf::from("src/wasm");
    let wasm_path = output_path.join(format!("{sample_name}_bg.wasm"));
    if !fs::exists(&wasm_path).is_ok_and(|t| t) {
        info!("file '{wasm_path:?}' does not exist, need to compile");
        return false;
    }

    let js_path = output_path.join(format!("{sample_name}.js"));
    if !fs::exists(&js_path).is_ok_and(|t| t) {
        info!("file '{js_path:?}' does not exist, need to compile");
        return false;
    }

    let wasm_last_modified = last_modified_of(wasm_path);
    let js_last_modified = last_modified_of(js_path);

    debug!("wasm last modified: {}", wasm_last_modified);
    debug!("js last modified:   {}", js_last_modified);

    let last_compile = if wasm_last_modified < js_last_modified {
        wasm_last_modified
    } else {
        js_last_modified
    };

    debug!("chapter last modified: {}", chapter_last_modified);
    debug!("last compile:          {}", last_compile);

    &last_compile < chapter_last_modified
}

fn last_modified_of(p: impl AsRef<Path>) -> DateTime<Utc> {
    let system_time = fs::metadata(p)
        .expect("Cannot read metadata of file")
        .modified()
        .expect("Cannot read last modified of file");
    DateTime::<Utc>::from(UNIX_EPOCH + system_time.duration_since(UNIX_EPOCH).unwrap())
}

#[allow(unused)]
fn log(content: &str) {
    // Hard-coded file path
    let file_path = "logs/output.txt";

    // Get current local timestamp
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Open the file in append mode, create it (and parent dirs) if needed
    std::fs::create_dir_all("logs").expect("file should exsist"); // Ensure "logs" directory exists

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Should be able to open the file");

    // Write the timestamp and content
    writeln!(file, "[{}] {}", timestamp, content).expect("Should be able to write the file");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nop_preprocessor_run() {
        let input_json = r##"[
                {
                    "root": "/path/to/book",
                    "config": {
                        "book": {
                            "authors": ["AUTHOR"],
                            "language": "en",
                            "multilingual": false,
                            "src": "src",
                            "title": "TITLE"
                        },
                        "preprocessor": {
                            "nop": {}
                        }
                    },
                    "renderer": "html",
                    "mdbook_version": "0.4.21"
                },
                {
                    "sections": [
                        {
                            "Chapter": {
                                "name": "Chapter 1",
                                "content": "# Chapter 1\n",
                                "number": [1],
                                "sub_items": [],
                                "path": "chapter_1.md",
                                "source_path": "chapter_1.md",
                                "parent_names": []
                            }
                        }
                    ],
                    "__non_exhaustive": null
                }
            ]"##;
        let input_json = input_json.as_bytes();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json).unwrap();
        let expected_book = book.clone();
        let result = EguiPreproc::new().run(&ctx, book);
        assert!(result.is_ok());

        // The nop-preprocessor should not have made any changes to the book content.
        let actual_book = result.unwrap();
        assert_eq!(actual_book, expected_book);
    }
}
