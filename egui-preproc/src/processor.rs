use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;
use mdbook::book::Chapter;
use regex::Regex;

use super::*;

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
    let re = Regex::new(r"\{\{sample: (.*)\}\}").expect("Regex should compile");
    chapter.content = re
        .replace_all(&chapter.content, |caps: &regex::Captures| {
            log(&format!("found group {}", &caps[1]));
            format!("This is sample: {}", &caps[1])
        })
        .to_string();
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
