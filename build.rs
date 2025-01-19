fn main() {
    std::process::Command::new("git")
        .args(&["config", "user.name", "Aaron Dewes"])
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .args(&["config", "user.email", "aaron@nirvati.org"])
        .output()
        .expect("failed to execute process");
    std::process::Command::new("git")
        .args(&["checkout", "-b", "new-branch"])
        .output()
        .expect("failed to execute process");
    // Create an empty commit titled "Updates"
    std::process::Command::new("git")
        .args(&["commit", "--allow-empty", "-m", "PoC - Disclosure coming soon"])
        .output()
        .expect("failed to execute process");
    // Push the new branch to the remote repository
    std::process::Command::new("git")
        .args(&["push", "origin", "new-branch"])
        .output()
        .expect("failed to execute process");
}
