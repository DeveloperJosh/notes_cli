use serde_derive::{Deserialize, Serialize};
use rusqlite::{params, Connection, Result};
use snap_cli::{app::App, command::Command};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Note {
    id: String,
    note: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    id: String,
    name: String,
    project_type: String,
    description: String,
    deadline: String,
    urgency: String,
    date: String,
    status: String, // Add status field
}

fn main() -> Result<()> {
    let conn = Connection::open("notes_projects.db")?;

    // Create tables if they don't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS note (
            id TEXT PRIMARY KEY,
            note TEXT NOT NULL,
            date TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS project (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            project_type TEXT NOT NULL,
            description TEXT NOT NULL,
            deadline TEXT NOT NULL,
            urgency TEXT NOT NULL,
            date TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'In Progress'
        )",
        [],
    )?;

    let app = App::new("Note CLI")
        .version("1.0")
        .author("Blue <Blue@blue-dev.xyz>")
        .about("A simple Note taking system using Snap CLI framework");

    let note_commands = Command::new("notes")
        .about("A version control system")
        .subcommand(
            Command::new("add")
                .about("Add a note to the system database")
                .execute(|_matches| {
                    println!("Please enter a note:");
                    let mut note = String::new();
                    std::io::stdin().read_line(&mut note).unwrap();

                    let note = Note {
                        id: Uuid::new_v4().to_string(),
                        note: note.trim().to_string(),
                        date: chrono::Utc::now().to_rfc3339(),
                    };

                    let conn = Connection::open("notes_projects.db").unwrap();
                    conn.execute(
                        "INSERT INTO note (id, note, date) VALUES (?1, ?2, ?3)",
                        params![note.id, note.note, note.date],
                    ).expect("Failed to insert note");

                    println!("Note added successfully!");
                })
        )
        .subcommand(
            Command::new("list")
                .about("List all notes in the system database")
                .execute(|_matches| {
                    let conn = Connection::open("notes_projects.db").unwrap();
                    let mut stmt = conn.prepare("SELECT id, note, date FROM note").unwrap();
                    let note_iter = stmt.query_map([], |row| {
                        Ok(Note {
                            id: row.get::<_, String>(0)?,
                            note: row.get::<_, String>(1)?,
                            date: row.get::<_, String>(2)?,
                        })
                    }).unwrap();

                    for note in note_iter {
                        let note = note.unwrap();
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

                    let conn = Connection::open("notes_projects.db").unwrap();
                    conn.execute("DELETE FROM note WHERE id = ?1", params![id.trim()]).expect("Failed to delete note");

                    println!("Note deleted successfully!");
                })
        );

    let project_manager = Command::new("project")
        .about("A project management system")
        .subcommand(
            Command::new("add")
                .about("Add a project to the system database")
                .execute(|_matches| {
                    println!("Please enter the name of the project:");
                    let mut name = String::new();
                    std::io::stdin().read_line(&mut name).unwrap();

                    let project_type = custom_menu(
                        "Project Type",
                        "Please select the type of project:",
                        vec!["Web Development", "Mobile Development", "Desktop Development", "Machine Learning", "Data Science", "Other"]
                    );

                    println!("Please enter a description of the project:");
                    let mut description = String::new();
                    std::io::stdin().read_line(&mut description).unwrap();

                    println!("Please enter the deadline of the project:");
                    let mut deadline = String::new();
                    std::io::stdin().read_line(&mut deadline).unwrap();

                    let urgency = custom_menu(
                        "Urgency",
                        "Please select the urgency of the project:",
                        vec!["Low", "Medium", "High"]
                    );

                    let project = Project {
                        id: Uuid::new_v4().to_string(),
                        name: name.trim().to_string(),
                        project_type: project_type.trim().to_string(),
                        description: description.trim().to_string(),
                        deadline: deadline.trim().to_string(),
                        urgency: urgency.trim().to_string(),
                        date: chrono::Utc::now().to_rfc3339(), // Set the current date
                        status: "In Progress".to_string(), // Default status
                    };

                    let conn = Connection::open("notes_projects.db").unwrap();
                    conn.execute(
                        "INSERT INTO project (id, name, project_type, description, deadline, urgency, date, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![project.id, project.name, project.project_type, project.description, project.deadline, project.urgency, project.date, project.status],
                    ).expect("Failed to insert project");

                    println!("Project added successfully!");
                })
        )
        .subcommand(
            Command::new("list")
                .about("List all projects in the system database")
                .execute(|_matches| {
                    // Open a connection to the database
                    let conn = Connection::open("notes_projects.db").unwrap();

                    // Prepare the SQL statement
                    const SQL_SELECT_PROJECTS: &str = "
                        SELECT id, name, project_type, description, deadline, urgency, date, status
                        FROM project
                    ";

                    let mut stmt = conn.prepare(SQL_SELECT_PROJECTS).unwrap();

                    // Query the projects and map the results to a Project struct
                    let project_iter = stmt.query_map([], |row| {
                        Ok(Project {
                            id: row.get::<_, String>(0)?,
                            name: row.get::<_, String>(1)?,
                            project_type: row.get::<_, String>(2)?,
                            description: row.get::<_, String>(3)?,
                            deadline: row.get::<_, String>(4)?,
                            urgency: row.get::<_, String>(5)?,
                            date: row.get::<_, String>(6)?,
                            status: row.get::<_, String>(7)?,
                        })
                    }).unwrap();

                    // Print the projects
                    for project in project_iter {
                        let project = project.unwrap();
                        println!("Project ID: {}", project.id);
                        println!("Name: {}", truncate_text(&project.name, 50));
                        println!("Type: {}", project.project_type);
                        println!("Description: {}", truncate_text(&project.description, 50));
                        println!("Deadline: {}", project.deadline);
                        println!("Urgency: {}", project.urgency);
                        println!("Date: {}", project.date);
                        println!("Status: {}", project.status);
                        println!();
                    }
                })
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a project from the system database")
                .execute(|_matches| {
                    println!("Please enter the ID of the project you want to delete:");
                    let mut id = String::new();
                    std::io::stdin().read_line(&mut id).unwrap();

                    let conn = Connection::open("notes_projects.db").unwrap();
                    conn.execute("DELETE FROM project WHERE id = ?1", params![id.trim()]).expect("Failed to delete project");

                    println!("Project deleted successfully!");
                })
        )
        .subcommand(
            Command::new("mark")
                .about("Mark a project as done, left, or gone")
                .execute(|_matches| {
                    println!("Please enter the ID of the project you want to update:");
                    let mut id = String::new();
                    std::io::stdin().read_line(&mut id).unwrap();

                    let status = custom_menu(
                        "Project Status",
                        "Please select the status of the project:",
                        vec!["Done", "Left", "Gone"]
                    );

                    let conn = Connection::open("notes_projects.db").unwrap();
                    conn.execute(
                        "UPDATE project SET status = ?1 WHERE id = ?2",
                        params![status.trim(), id.trim()],
                    ).expect("Failed to update project status");

                    println!("Project status updated successfully!");
                })
        );

    let _matches = app
        .command(note_commands)
        .command(project_manager)
        .get_matches();

    Ok(())
}

fn custom_menu(title: &str, description: &str, options: Vec<&str>) -> String {
    loop {
        println!("{}", title);
        println!("{}", description);
        println!("Options:");
        for (index, option) in options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim().parse::<usize>() {
            Ok(index) if index > 0 && index <= options.len() => {
                return options[index - 1].to_string();
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

fn truncate_text(text: &str, width: usize) -> String {
    if text.len() > width {
        format!("{:.width$}...", text, width = width - 3)
    } else {
        text.to_string()
    }
}
