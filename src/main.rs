mod nodes;
mod parser;
mod query;

use clap::Parser;
use std::process;

#[derive(Parser)]
#[command(name = "reerio")]
#[command(about = "Reerio, Super Fast Html Parser", long_about = None)]
struct Cli {
    /// The URL or file path to parse
    url_or_path: String,

    /// Element tag to find
    #[arg(short, long)]
    find: Option<String>,

    /// Attribute key or key=value pair
    #[arg(long)]
    attr: Option<String>,

    /// Search by inner text
    #[arg(long)]
    text: Option<String>,

    /// Only return the first match
    #[arg(long)]
    first: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.url_or_path.starts_with("http://") || cli.url_or_path.starts_with("https://") {
        if let Err(e) = handle_url(&cli).await {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = handle_file(&cli) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

async fn handle_url(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching HTML from: {}", cli.url_or_path);

    let response = reqwest::get(&cli.url_or_path).await?;
    let html_content = response.text().await?;

    if !parser::is_html(&html_content) {
        return Err("The URL did not return valid HTML content".into());
    }

    println!("HTML loaded successfully ({} bytes)", html_content.len());

    let root = parser::parse_html_to_nodes(&html_content)?;
    let results = run_query(&root, cli);

    display_results(results, cli);
    Ok(())
}

fn handle_file(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading HTML from file: {}", cli.url_or_path);

    let html_content = std::fs::read_to_string(&cli.url_or_path)?;
    if !parser::is_html(&html_content) {
        return Err("File does not contain valid HTML content".into());
    }

    println!("HTML loaded successfully ({} bytes)", html_content.len());

    let root = parser::parse_html_to_nodes(&html_content)?;
    let results = run_query(&root, cli);

    display_results(results, cli);
    Ok(())
}

/// Choose correct query function based on CLI flags
fn run_query<'a>(root: &'a nodes::HtmlNode, cli: &Cli) -> Vec<&'a nodes::HtmlNode> {
    if let Some(tag) = &cli.find {
        query::find_by_tag(root, tag)
    } else if let Some(attr) = &cli.attr {
        if let Some((k, v)) = attr.split_once('=') {
            query::find_by_attribute(root, k.trim(), v.trim())
        } else {
            query::find(root, &query::SearchCriteria::AttributeKey(attr.clone()))
        }
    } else if let Some(text) = &cli.text {
        query::find_by_text(root, text)
    } else {
        Vec::new()
    }
}

/// Pretty-print results
fn display_results(results: Vec<&nodes::HtmlNode>, cli: &Cli) {
    if results.is_empty() {
        println!("No matches found.");
        return;
    }

    if cli.first {
        if let Some(node) = results.first() {
            println!("\nFirst match: <{}> {}", node.tag, extract_text(node));
        } else {
            println!("No matches found.");
        }
        return;
    }

    println!("\nFound {} match(es):", results.len());
    for (i, node) in results.iter().enumerate().take(10) {
        println!("{}. <{}> {}", i + 1, node.tag, extract_text(node));
    }
    if results.len() > 10 {
        println!("... and {} more", results.len() - 10);
    }
}

/// Helper: grab text from node or children
fn extract_text(node: &nodes::HtmlNode) -> String {
    if let Some(t) = &node.text {
        t.clone()
    } else {
        node.children
            .iter()
            .filter_map(|c| c.text.clone())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
