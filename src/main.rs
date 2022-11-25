use std::process::Command;

fn main() {
    let commit_msg = std::env::args()
        .nth(1)
        .expect("Your commit message is missing");

    // git status for convenience
    let output = Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute git status");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

    // add .
    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("failed to execute git add");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    // commit
    let output = Command::new("git")
        .arg("commit")
        .arg("-am")
        .arg(commit_msg)
        .output()
        .expect("failed to execute git commit");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Thank you for flying Zentropy git airlines.");
}
