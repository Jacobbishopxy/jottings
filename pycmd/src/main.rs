//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/07 18:04:35 Friday
//! brief:

use std::process::Command;

fn main() -> std::io::Result<()> {
    let python_cmd = format!("{}/python", "/opt/homebrew/anaconda3/envs/py310/bin");

    // bash -c "cd ~/Code/jotting/pycmd/scripts/ && /opt/homebrew/anaconda3/envs/py310/bin/python dev.py"

    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            python_cmd
        ))
        .output()?;

    println!("Status code: {}", output.status);
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}
