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

    if let Some(stdout) = &mut cmd.stdout {
        let lines = BufReader::new(stdout).lines();

        for line in lines {
            println!("READ: {:?}", line?);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn exec_and_print_with_thread(py_cmd: &str) -> std::io::Result<()> {
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let child_stdout = cmd.stdout.take().unwrap();
    let child_stderr = cmd.stderr.take().unwrap();

    let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = std::thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            println!("{:?}", line);
            stdout_tx.send(line).unwrap();
        }
    });

    let stderr_thread = std::thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            println!("{:?}", line);
            stderr_tx.send(line).unwrap();
        }
    });

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let status = cmd.wait()?;

    let stdout = stdout_rx.into_iter().collect::<Vec<_>>();
    let stderr = stderr_rx.into_iter().collect::<Vec<_>>();

    println!("status: {:?}", status);
    println!("stdout: {:?}", stdout);
    println!("stderr: {:?}", stderr);

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
    exec_and_print_with_thread(&py_cmd)?;
    // exec_and_print_inherit(&py_cmd)?;

    Ok(())
}
