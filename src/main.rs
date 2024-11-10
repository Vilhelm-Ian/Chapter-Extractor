use anyhow::{Context, Result};
use mupdf::pdf::document::PdfDocument;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct Chapter {
    title: String,
    start_page: u32,
}

fn extract_chapters(doc: &PdfDocument) -> Result<Vec<Chapter>> {
    let outlines = doc.outlines().context("Failed to get PDF outlines")?;

    let chapters = outlines
        .iter()
        .filter_map(|outline| {
            let page = outline.page?;
            Some(Chapter {
                title: outline.title.clone(),
                start_page: page,
            })
        })
        .collect::<Vec<_>>();

    if chapters.is_empty() {
        anyhow::bail!("No valid chapters found");
    }

    Ok(chapters)
}

fn extract_chapter_text(doc: &PdfDocument, start_page: u32, end_page: u32) -> Result<String> {
    let mut pages = doc.pages().context("Failed to get PDF pages")?;

    let mut chapter_text = String::new();

    for page_num in start_page..end_page {
        if let Some(Ok(page)) = pages.nth(page_num as usize) {
            let page_text = page.to_text().context("Failed to extract text from page")?;
            chapter_text.push_str(&page_text);
        }
    }

    Ok(chapter_text)
}

fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        anyhow::bail!("Usage: {} <pdf_file> [output_directory]", args[0]);
    }

    let pdf_path = Path::new(&args[1]);
    let output_dir = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // Ensure output directory exists
    fs::create_dir_all(&output_dir).context("Failed to create output directory")?;
    let doc = PdfDocument::open(
        pdf_path
            .to_str()
            .context("PDF path contains invalid Unicode")?,
    )
    .context("Failed to open PDF document")?;

    let chapters = extract_chapters(&doc)?;
    let total_pages = doc.page_count().context("Failed to get page count")? as u32;

    for (i, chapter) in chapters.iter().enumerate() {
        let end_page = chapters
            .get(i + 1)
            .map(|next| next.start_page)
            .unwrap_or(total_pages);

        let text = extract_chapter_text(&doc, chapter.start_page, end_page)?;

        let safe_filename = sanitize_filename(&chapter.title);
        let output_path = output_dir.join(format!("{}.txt", safe_filename));

        fs::write(&output_path, text)
            .with_context(|| format!("Failed to write chapter file: {:?}", output_path))?;
    }

    Ok(())
}
