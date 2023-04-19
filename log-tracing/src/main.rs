//! file: main.rs
//! author: Jacob Xie
//! date: 2023/04/18 11:12:41 Tuesday
//! brief:

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

use anyhow::{Error, Result};
use futures::channel::mpsc::{channel, Receiver};
use futures::{SinkExt, StreamExt};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

// ================================================================================================
// Type
// ================================================================================================

// File : Position
type Recorder = HashMap<String, usize>;

// ================================================================================================
// Const
// ================================================================================================

const FILE_LIST: &[&str] = &["dev.log"];

// ================================================================================================
// Fn
// ================================================================================================

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

fn file_read<T: AsRef<Path>>(path: T, check_point: Option<usize>) -> Result<usize> {
    let mut reader = BufReader::new(File::open(path)?);

    // if check_point exists, move bufRreader to ckp
    let mut pos = if let Some(ckp) = check_point {
        reader.seek(SeekFrom::Start(ckp.try_into()?))?;
        ckp
    } else {
        0
    };

    let mut info = Vec::new();
    let mut line = String::new();

    while let Ok(num) = reader.read_line(&mut line) {
        if num == 0 {
            break;
        }
        let mut tmp = String::new();
        std::mem::swap(&mut tmp, &mut line);
        info.push(tmp);
        pos += num;
    }

    println!("{:?}", info);

    Ok(pos)
}

async fn async_watch<P: AsRef<Path>>(path: P, recorder: &mut Recorder) -> Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                // println!("changed: {:?}", event);
                let path = event
                    .paths
                    .first()
                    .ok_or(Error::msg("Empty path!"))?
                    .as_path();

                let filename = path
                    .file_name()
                    .ok_or(Error::msg("Not a filename!"))?
                    .to_str()
                    .ok_or(Error::msg("Filename to_str failed!"))?;

                // filter from watch list
                if let Some(p) = path.to_str().filter(|_| FILE_LIST.contains(&filename)) {
                    match &event.kind {
                        EventKind::Access(_) => {
                            // println!("file {} has been accessing", p);
                        }
                        EventKind::Create(_) => {
                            println!("new file {} has been created", p);
                        }
                        EventKind::Modify(_) => {
                            // get current position
                            let cur_pos = recorder.get(p).copied();
                            // read file and print
                            let new_pos = file_read(p, cur_pos)?;
                            // update position
                            recorder.insert(p.to_string(), new_pos);
                        }
                        EventKind::Remove(_) => {
                            recorder.remove(p);
                            println!("file {} has beed removed", p);
                        }
                        _ => {
                            // ignore
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

// ================================================================================================
// Main
// ================================================================================================

/// Async, futures channel based event watching
#[tokio::main]
async fn main() {
    // let path = std::env::args()
    //     .nth(1)
    //     .expect("Argument 1 needs to be a path");
    let path = "./log";
    println!("watching {}", path);

    let mut recorder = Recorder::new();

    while let Err(e) = async_watch(path, &mut recorder).await {
        println!("error: {:?}", e);
    }
}
