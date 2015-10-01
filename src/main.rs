use std::sync::mpsc::channel;
use std::thread;
use std::io::{self, Read, Write};

// TODO Delegate to Welcome and Seed providers
static WELCOME_MESSAGE: &'static [u8] = b"Ice9 Shell | (c) 2015 Denis Kolodin\n";
static SEED_TEXT: &'static [u8] = b"> ";

#[derive(Debug)]
enum Input {
	Line(String),
	// TODO Add AST
}

#[derive(Debug)]
enum Output {
	Welcome, // = MOTD in Bash
	Seed,    // = Prompt in Bash
	Line(String),
	// TODO Add raw
}

fn main() {	
	let (inner_tx, inner_rx) = channel::<Input>();
	let inner = thread::spawn(move || {
		let stdin = &mut io::stdin();
		let mut buffer = String::new();
		loop {
			buffer.clear();
			match stdin.read_line(&mut buffer) {
				Ok(_) => {
					let line = Input::Line(buffer.clone());
					inner_tx.send(line);
				},
				Err(_) => break,
			};
		}
	});
	
	let (outer_tx, outer_rx) = channel::<Output>();
	let outer = thread::spawn(move || {
		let stdout = &mut io::stdout();
		loop {
			match outer_rx.recv() {
				Ok(Output::Welcome) => {
					stdout.write(WELCOME_MESSAGE);
				},
				Ok(Output::Seed) => {
					stdout.write(SEED_TEXT);
				},
				Ok(Output::Line(l)) => {
					stdout.write(l.as_bytes());
				},
				Err(_) => break,
			}
			stdout.flush();
		}
	});

	outer_tx.send(Output::Welcome);
	loop {
		outer_tx.send(Output::Seed);
		match inner_rx.recv() {
			Ok(Input::Line(l)) => {
				// 1. Line Parsing
				// 2. Commands execution (with outer_tx cloning)
				// 3. Await for commands ends
				outer_tx.send(Output::Line(l));
			},
			Err(_) => break,
		}
	}
}


