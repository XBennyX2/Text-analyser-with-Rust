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

/// Tokenize text into words (lowercased, alphanumeric only)
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect()
}

/// Generate n-grams (functional, no imperative loops)
fn generate_ngrams(tokens: &[String], n: usize) -> Vec<String> {
    if n == 1 {
        return tokens.to_vec();
    }
    (0..tokens.len().saturating_sub(n - 1))
        .map(|i| tokens[i..i + n].join(" "))
        .collect()
}

/// Count frequencies (functional fold)
fn count_frequencies(words: &[String]) -> HashMap<String, usize> {
    words.iter().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(w.clone()).or_insert(0) += 1;
        acc
    })
}

/// Display bar chart for frequencies
fn display_bar_chart(items: &[(String, usize)], max_bar: usize) {
    if let Some(max_count) = items.first().map(|(_, c)| *c as f64) {
        items.iter().for_each(|(word, count)| {
            let bar_len = ((*count as f64 / max_count) * max_bar as f64).round() as usize;
            let bar = "█".repeat(bar_len);
            println!("{:<20} {:>5} {}", word.bold(), count, bar.blue());
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} <file-path> [--top=N] [--match-regex=PATTERN] [--ngrams=N] [--filter-stopwords] [--min-length=N] [--starts-with=C]",
            args[0]
        );
        process::exit(1);
    }

    let filename = &args[1];
    let mut top_n = 10;
    let mut regex_filter: Option<Regex> = None;
    let mut ngrams = 1;
    let mut filter_stopwords = false;
    let mut min_length: Option<usize> = None;
    let mut starts_with: Option<char> = None;

    // Parse optional flags
    args.iter().skip(2).for_each(|arg| {
        if let Some(n) = arg.strip_prefix("--top=") {
            top_n = n.parse().unwrap_or(10);
        } else if let Some(pattern) = arg.strip_prefix("--match-regex=") {
            regex_filter = Regex::new(pattern).ok();
        } else if let Some(n) = arg.strip_prefix("--ngrams=") {
            ngrams = n.parse().unwrap_or(1);
        } else if arg == "--filter-stopwords" {
            filter_stopwords = true;
        } else if let Some(n) = arg.strip_prefix("--min-length=") {
            min_length = n.parse().ok();
        } else if let Some(c) = arg.strip_prefix("--starts-with=") {
            starts_with = c.chars().next();
        }
    });

    // Read file
    let contents = fs::read_to_string(filename).expect("Could not read the file");

    // Tokenize
    let mut tokens = tokenize(&contents);

    // Apply stopword filter
    if filter_stopwords {
        let sw = stopwords();
        tokens = tokens
            .into_iter()
            .filter(|w| !sw.contains(w.as_str()))
            .collect();
    }

    // Apply regex filter
    if let Some(re) = &regex_filter {
        tokens = tokens.into_iter().filter(|w| re.is_match(w)).collect();
    }

    // Apply min-length filter
    if let Some(n) = min_length {
        tokens = tokens.into_iter().filter(|w| w.len() > n).collect();
    }

    // Apply starts-with filter
    if let Some(c) = starts_with {
        tokens = tokens.into_iter().filter(|w| w.starts_with(c)).collect();
    }

    // Generate N-grams
    let grams = generate_ngrams(&tokens, ngrams);

    // Count frequencies (functional)
    let frequencies = count_frequencies(&grams);

    // Sort by frequency
    let mut sorted: Vec<_> = frequencies.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    // Output
    println!("{}", "Text Analysis Report".bold().underline().cyan());
    println!("{} {}", "Total items:".green(), grams.len());
    println!("{} {}", "Unique items:".green(), sorted.len());

    if let Some((word, count)) = sorted.first() {
        println!(
            "{} {} ({})",
            "Most common:".yellow().bold(),
            word.bold(),
            count
        );
    }

    if !sorted.is_empty() {
        println!("\n{}", format!("Top {} frequent:", top_n).yellow().bold());
        let top_items: Vec<_> = sorted.iter().take(top_n).cloned().collect();
        display_bar_chart(&top_items, 40); // 40 chars max bar
    } else {
        println!("{}", "No words matched the filter.".red());
    }
}
