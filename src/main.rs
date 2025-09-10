use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::process;

use colored::*;
use regex::Regex;

/// Hard-coded English stopword list
fn stopwords() -> HashSet<&'static str> {
    [
        "a", "an", "and", "are", "as", "at", "be", "but", "by",
        "for", "if", "in", "into", "is", "it", "no", "not",
        "of", "on", "or", "such", "that", "the", "their", "then",
        "there", "these", "they", "this", "to", "was", "will", "with",
    ]
    .into_iter()
    .collect()
}

/// Tokenize text into words
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect()
}

/// Generate n-grams from tokens
fn generate_ngrams(tokens: &[String], n: usize) -> Vec<String> {
    if n == 1 {
        return tokens.to_vec();
    }
    let mut ngrams = Vec::new();
    for i in 0..tokens.len().saturating_sub(n - 1) {
        ngrams.push(tokens[i..i + n].join(" "));
    }
    ngrams
}

/// Count frequencies
fn count_frequencies(words: &[String]) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    for word in words {
        *freq.entry(word.clone()).or_insert(0) += 1;
    }
    freq
}

/// Display bar chart for frequencies
fn display_bar_chart(items: &[(String, usize)], max_bar: usize) {
    if items.is_empty() {
        return;
    }
    let max_count = items[0].1 as f64;
    for (word, count) in items {
        let bar_len = (( *count as f64 / max_count) * max_bar as f64).round() as usize;
        let bar = "█".repeat(bar_len);
        println!("{:<20} {:>5} {}", word.bold(), count, bar.blue());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file-path> [--top=N] [--match-regex=PATTERN] [--ngrams=N] [--filter-stopwords]", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let mut top_n = 10;
    let mut regex_filter: Option<Regex> = None;
    let mut ngrams = 1;
    let mut filter_stopwords = false;

    // Parse optional flags
    for arg in &args[2..] {
        if arg.starts_with("--top=") {
            if let Some(n) = arg.split('=').nth(1) {
                top_n = n.parse().unwrap_or(10);
            }
        } else if arg.starts_with("--match-regex") {
            if let Some(pattern) = arg.split('=').nth(1) {
                regex_filter = Regex::new(pattern).ok();
            }
        } else if arg.starts_with("--ngrams") {
            if let Some(n) = arg.split('=').nth(1) {
                ngrams = n.parse().unwrap_or(1);
            }
        } else if arg == "--filter-stopwords" {
            filter_stopwords = true;
        }
    }

    // Read file
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    // Tokenize
    let mut tokens = tokenize(&contents);

    // Apply stopword filter
    if filter_stopwords {
        let sw = stopwords();
        tokens = tokens.into_iter().filter(|w| !sw.contains(w.as_str())).collect();
    }

    // Apply regex filter
    if let Some(re) = &regex_filter {
        tokens = tokens.into_iter().filter(|w| re.is_match(w)).collect();
    }

    // Generate N-grams
    let grams = generate_ngrams(&tokens, ngrams);

    // Count frequencies
    let frequencies = count_frequencies(&grams);

    // Sort by frequency
    let mut sorted: Vec<_> = frequencies.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // Output
    println!("{}", "Text Analysis Report".bold().underline().cyan());
    println!("{} {}", "Total items:".green(), grams.len());
    println!("{} {}", "Unique items:".green(), sorted.len());

    if !sorted.is_empty() {
        println!("\n{}", format!("Top {} frequent:", top_n).yellow().bold());
        let top_items: Vec<_> = sorted.iter().take(top_n).cloned().collect();
        display_bar_chart(&top_items, 40); // 40 chars max bar
    } else {
        println!("{}", "No words matched the filter.".red());
    }
}
