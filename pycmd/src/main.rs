//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/07 18:04:35 Friday
//! brief:
//!
//! check <https://doc.rust-lang.org/std/process/struct.Command.html> for more.
//! refference: https://www.nikbrendler.com/rust-process-communication/

use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::os::fd::FromRawFd;
use std::process::{ChildStderr, ChildStdout, Command, Stdio};
use std::sync::mpsc::Sender;

use anyhow::Result;
use libc;

#[allow(dead_code)]
fn exec_and_print_at_once(py_cmd: &str) -> Result<()> {
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
fn exec_and_print_inherit(py_cmd: &str) -> Result<()> {
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev2.py",
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

#[allow(dead_code)]
fn exec_and_print_with_buf(py_cmd: &str) -> Result<()> {
    let mut cmd = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = cmd.stdout.take().unwrap();
    let stdout_reader = BufReader::new(stdout);

    for line in stdout_reader.lines() {
        println!("READ: {:?}", line);
    }

    let status = cmd.wait()?;
    println!("status: {:?}", status);

    Ok(())
}

fn stdout_lines(stdout: ChildStdout, rec: Sender<String>) -> Result<()> {
    for line in BufReader::new(stdout).lines() {
        let line = line?;
        println!("stdout > {:?}", line);
        rec.send(line)?;
    }

    Ok(())
}

fn stderr_lines(stderr: ChildStderr, rec: Sender<String>) -> Result<()> {
    for line in BufReader::new(stderr).lines() {
        let line = line?;
        println!("stderr > {:?}", line);
        rec.send(line)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn exec_and_print_with_thread(py_cmd: &str) -> Result<()> {
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

    let stdout_thread = std::thread::spawn(|| stdout_lines(child_stdout, stdout_tx));
    let stderr_thread = std::thread::spawn(|| stderr_lines(child_stderr, stderr_tx));

    stdout_thread
        .join()
        .map_err(|_| anyhow::anyhow!("thread join failed!"))??;
    stderr_thread
        .join()
        .map_err(|_| anyhow::anyhow!("thread join failed!"))??;

    let stdout_rx_thread = std::thread::spawn(move || loop {
        let msg = stdout_rx.recv();
        println!("stdout_rx: {:?}", msg);
    });
    let stderr_rx_thread = std::thread::spawn(move || loop {
        let msg = stderr_rx.recv();
        println!("stdout_rx: {:?}", msg);
    });

    stdout_rx_thread
        .join()
        .map_err(|_| anyhow::anyhow!("thread join failed"))?;
    stderr_rx_thread
        .join()
        .map_err(|_| anyhow::anyhow!("thread join failed"))?;

    let status = cmd.wait()?;
    println!("status: {:?}", status);

    Ok(())
}

#[allow(dead_code)]
fn exec_and_print_by_unsafe_libc(py_cmd: &str) -> Result<()> {
    let mut fds = [0i32; 2];
    unsafe {
        libc::pipe(&mut fds as *mut libc::c_int);
        libc::dup2(libc::STDOUT_FILENO, fds[1] as libc::c_int);
    }

    Command::new("bash")
        .arg("-c")
        .arg(format!(
            "cd ~/Code/jotting/pycmd/scripts && {} dev.py",
            py_cmd
        ))
        .spawn()
        .expect("true");

    println!("Yohoho!");
    let mut r: File;

    unsafe {
        r = FromRawFd::from_raw_fd(fds[0] as libc::c_int);
    }
    let mut buffer = [0u8; 1024];
    let count = r.read(&mut buffer).unwrap();

    while count > 0 {
        println!("{}", std::str::from_utf8(&buffer[0..count]).unwrap());
    }

    Ok(())
}

fn main() -> Result<()> {
    // let py_cmd = format!("{}/python - u", "/opt/homebrew/anaconda3/envs/py310/bin");
    let py_cmd = format!("{}/python -u ", "/home/jacob/anaconda3/envs/py310/bin");

    // bash -c "cd ~/Code/jotting/pycmd/scripts/ && /opt/homebrew/anaconda3/envs/py310/bin/python dev.py"

    // exec_and_print_at_once(&py_cmd)?;
    // exec_and_print_inherit(&py_cmd)?;
    // exec_and_print_with_buf(&py_cmd)?;
    exec_and_print_with_thread(&py_cmd)?;
    // exec_and_print_by_unsafe_libc(&py_cmd)?;

    Ok(())
}
