use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, to_writer_pretty, from_reader};
use snap_cli::{app::App, command::Command};
use uuid::Uuid;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use chrono::{Utc, DateTime};

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: String,
    note: String,
    date: DateTime<Utc>,
}

fn main() {
    let app = App::new("Note CLI")
        .version("1.0")
        .author("Blue <Blue@blue-dev.xyz>")
        .about("A simple Note taking system using Snap CLI framework");

    let git_command = Command::new("notes")
        .about("A version control system")
        .subcommand(
            Command::new("add")
                .about("Add a note to the system database")
                .execute(|_matches| {
                    println!("Please enter a note:");
                    let mut note = String::new();
                    std::io::stdin().read_line(&mut note).unwrap();

                    // add note to json, give it a unique id and current date
                    let note = Note {
                        id: Uuid::new_v4().to_string(), // generate a unique id
                        note: note.trim().to_string(),
                        date: Utc::now(),
                    };

                    // Open the file in read mode
                    let mut file = OpenOptions::new().read(true).open("notes.json").expect("Unable to open file");

                    // Read the existing notes
                    let mut notes: Vec<Note> = from_reader(&file).unwrap_or_else(|_| Vec::new());

                    // Append the new note
                    notes.push(note);

                    // Open the file in write mode
                    let file = OpenOptions::new().write(true).truncate(true).open("notes.json").expect("Unable to open file");

                    // Write the notes to the file
                    to_writer_pretty(file, &notes).expect("Unable to write to file");
                })
        )
        .subcommand(
            Command::new("list")
                .about("List all notes in the system database")
                .execute(|_matches| {
                    // Open the file in read mode
                    let mut file = OpenOptions::new().read(true).open("notes.json").expect("Unable to open file");

                    // Read the existing notes
                    let notes: Vec<Note> = from_reader(&file).unwrap_or_else(|_| Vec::new());

                    // Print the notes
                    for note in notes {
                        println!("Note ID: {}", note.id);
                        println!("Note: {}", note.note);
                        println!("Date: {}", note.date);
                        println!();
                    }
                })
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a note from the system database")
                .execute(|_matches| {
                    println!("Please enter the ID of the note you want to delete:");
                    let mut id = String::new();
                    std::io::stdin().read_line(&mut id).unwrap();

                    // Open the file in read mode
                    let mut file = OpenOptions::new().read(true).open("notes.json").expect("Unable to open file");

                    // Read the existing notes
                    let mut notes: Vec<Note> = from_reader(&file).unwrap_or_else(|_| Vec::new());

                    // Remove the note with the given id
                    notes.retain(|note| note.id != id.trim());

                    // Open the file in write mode
                    let file = OpenOptions::new().write(true).truncate(true).open("notes.json").expect("Unable to open file");

                    // Write the notes to the file
                    to_writer_pretty(file, &notes).expect("Unable to write to file");
                })
        );
    let _matches = app.command(git_command).get_matches();
}