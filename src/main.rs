use std::{
    io::{Read, BufReader},
    process::{Command, Stdio, Child}, sync::{Arc, Mutex},
};
use std::io::BufRead;

// use tokio::io::BufReader;
use tokio::sync::mpsc::{self, UnboundedSender};

fn listen_for_output(program: &mut Child, tx: UnboundedSender<String> ) {
    match program.stdout.as_mut() {
        Some(out) => {
            let buf_reader = BufReader::new(out);
            for line in buf_reader.lines() {
                match line {
                    Ok(l) => {
                        // output_viewer.append_string(l);
                        if let Err(err) = tx.send(l){
                            println!("{}",err);
                        };
                    }
                    Err(_) => return,
                };
            }
        }
        None => return,
    }
}

fn run_command() {

    println!("run command");
    // let mut child_command = Command::new("tree")
    let mut child_command = Command::new("top")
        // .args(&["/", "&&", "tree"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Could not spawn Child");

    let (tx,mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    let jh= tokio::task::spawn(async move {
        let mut int = 0;

        while let Some(x) = rx.recv().await{
            println!("PRINTING {}",int);
            println!("{}",x);
            int+=1;
        }
        println!("Channel Done");

    });
    
    listen_for_output(&mut child_command,tx);

    println!("Done");
}

#[tokio::main]
async fn main() {
    run_command();
}
