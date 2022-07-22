use clap::Parser;
use colored::*;
use reqwest::Error;
use serde::Deserialize;
use std::fmt::Display;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let request_url = format!(
        "https://api.dictionaryapi.dev/api/{version}/entries/{language}/{word}",
        version = "v2",
        language = "en",
        word = args.word.trim()
    );

    let response = reqwest::get(&request_url).await?;

    let words: Vec<Word> = response.json().await?;

    for word in words {
        println!("{}", word);
    }

    Ok(())
}

#[derive(Deserialize)]
struct Word {
    word: String,
    phonetics: Vec<Phonetic>,
    meanings: Vec<Meaning>,
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.word.bold().underline())?;

        if !self.phonetics.is_empty() {
            write!(f, "{}", "Phonetics\n".underline())?;
        }
        for phonetic in &self.phonetics {
            write!(f, "\t- {}\n", phonetic)?;
        }

        if !self.meanings.is_empty() {
            write!(f, "{}", "Meanings\n".underline())?;
        }
        for (i, meaning) in self.meanings.iter().enumerate() {
            write!(f, "{}: {}\n", i + 1, meaning)?;

            if i < self.meanings.len() - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

impl Display for Meaning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.partOfSpeech.blue())?;

        for definition in &self.definitions {
            write!(f, "\t- {}\n", definition)?;
        }

        if !self.synonyms.is_empty() || !self.antonyms.is_empty() {
            write!(f, "\n")?;
        }

        if !self.synonyms.is_empty() {
            write!(f, "\t{}: {}", "Synonyms".red(), self.synonyms.join(", "))?;
        }

        if !self.antonyms.is_empty() {
            write!(f, "\n")?;
        }

        if !self.antonyms.is_empty() {
            write!(f, "\t{}: {}", "Antonyms".red(), self.antonyms.join(", "))?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Vec<String>,
    antonyms: Vec<String>,
}

impl Display for Definition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.definition)?;

        if let Some(example) = &self.example {
            write!(f, " ({})", example.purple())?;
        }

        // for synonym in &self.synonyms {
        //     write!(f, "{}\n", synonym)?;
        // }

        // for antonym in &self.antonyms {
        //     write!(f, "{}\n", antonym)?;
        // }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Phonetic {
    text: Option<String>,
    audio: String,
    source_url: Option<String>,
    license: Option<License>,
}

impl Display for Phonetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(text) = &self.text {
            write!(f, "{} ", text)?;
        } else {
            write!(f, "{}", "[unavailable] ".dimmed())?;
        }

        if self.audio.len() > 0 {
            write!(f, "({})", self.audio.dimmed())?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct License {
    name: String,
    url: String,
}

/// Application to look up the meaning of a word
#[derive(Parser)]
#[clap(version)]
struct Args {
    /// Word to define
    word: String,
}
