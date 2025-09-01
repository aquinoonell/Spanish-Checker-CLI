use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::fs;
use std::process;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(name = "spanish-checker")]
#[command(about = "Herramienta para revisar errores de español usando LanguageTool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Examina un archivo de texto
    Examine {
        /// Ruta al archivo
        file: String,
    },
}

#[derive(Debug, Deserialize)]
struct LTResponse {
    matches: Vec<Match>,
}

#[derive(Debug, Deserialize)]
struct Match {
    message: String,
    offset: usize,
    length: usize,
    replacements: Vec<Replacement>,
    context: Context,
    rule: Rule,
}

#[derive(Debug, Deserialize)]
struct Replacement {
    value: String,
}

#[derive(Debug, Deserialize)]
struct Context {
    text: String,
    offset: usize,
    length: usize,
}

#[derive(Debug, Deserialize)]
struct Rule {
    category: Category,
}

#[derive(Debug, Deserialize)]
struct Category {
    name: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Examine { file } => {
            let text = fs::read_to_string(file).unwrap_or_else(|e| {
                eprintln!("Error leyendo archivo: {}", e);
                process::exit(1);
            });

            if text.is_empty() {
                println!("El archivo está vacío.");
                return;
            }

            match check_spanish(&text).await {
                Ok(response) => print_errors(&response.matches, &text),
                Err(e) => {
                    eprintln!("Error al analizar: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}

async fn check_spanish(text: &str) -> Result<LTResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let params = [("text", text), ("language", "es"), ("enabledOnly", "false")];

    let response = client
        .post("https://api.languagetool.org/v2/check")
        .header("User-Agent", "spanish-checker/0.1")
        .form(&params)
        .send()
        .await?
        .json::<LTResponse>()
        .await?;

    Ok(response)
}

fn print_errors(matches: &[Match], text: &str) {
    if matches.is_empty() {
        println!("✓ No se encontraron errores");
        return;
    }

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    println!("\n{} errores encontrados:\n", matches.len());
    println!("{:-<60}", "");

    for (i, m) in matches.iter().enumerate() {
        let error_text = get_error_snippet(text, m.offset, m.length);

        *word_counts.entry(error_text.clone()).or_insert(0) += 1;

        let suggestions: Vec<String> = m
            .replacements
            .iter()
            .take(3)
            .map(|r| format!("'{}'", r.value))
            .collect();

        println!("{}. [{}]", i + 1, m.rule.category.name);
        println!("   Error: \"{}\"", error_text);
        println!(
            "   Sugerencias: {}",
            if suggestions.is_empty() {
                "ninguna".to_string()
            } else {
                suggestions.join(", ")
            }
        );
        println!("   Contexto: ...{}...", m.context.text);
        println!();
    }

    let total_errors = matches.len();
    let repeated_errors: usize = word_counts.values().filter(|&&v| v > 1).map(|&v| v - 1).sum();

    println!("{:-<60}", "");
    println!("Resumen:");
    println!("  Palabras mal escritas (total): {}", total_errors);
    println!("  Repeticiones de las mismas palabras: {}", repeated_errors);
}

fn get_error_snippet(text: &str, offset: usize, length: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let end = (offset + length).min(chars.len());
    chars[offset..end].iter().collect()
}

