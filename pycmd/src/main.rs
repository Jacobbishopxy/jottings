//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/07 18:04:35 Friday
//! brief:
//!
//! check <https://doc.rust-lang.org/std/process/struct.Command.html> for more.

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[allow(dead_code)]
fn exec_and_print_at_once(py_cmd: &str) -> std::io::Result<()> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .output()?;

    println!("Status code: {}", output.status);
    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[allow(dead_code)]
fn exec_and_print_with_buf(py_cmd: &str) -> std::io::Result<()> {
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.as_mut().unwrap();
    let stdout_reader = BufReader::new(stdout);
    let stdout_lines = stdout_reader.lines();

    for line in stdout_lines {
        println!("READ: {:?}", line);
    }

    cmd.wait()?;

    Ok(())
}

#[allow(dead_code)]
fn exec_and_print_inherit(py_cmd: &str) -> std::io::Result<()> {
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // It's streaming here
    let status = cmd.wait()?;
    println!("Exited with status {:?}", status);

    Ok(())
}

// TODO: more flexible python calling: `conda activate [xxx]; python [py_script]`

fn main() -> std::io::Result<()> {
    let py_cmd = format!("{}/python", "/opt/homebrew/anaconda3/envs/py310/bin");

    // bash -c "cd ~/Code/jotting/pycmd/scripts/ && /opt/homebrew/anaconda3/envs/py310/bin/python dev.py"

    // exec_and_print_at_once(&py_cmd)?;
    // exec_and_print_with_buf(&py_cmd)?;
    exec_and_print_inherit(&py_cmd)?;

    Ok(())
}
