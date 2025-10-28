use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Run sh -c "touch /tmp/it_works"
    let output = Command::new("sh")
        .arg("-c")
        .arg("touch /tmp/it_works")
        .output()
        .expect("Failed to execute command");
    
    if output.status.success() {
        println!("Successfully created /tmp/it_works");
    } else {
        eprintln!("Failed to create /tmp/it_works");
        std::process::exit(1);
    }
}
