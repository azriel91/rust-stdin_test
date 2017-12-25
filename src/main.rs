extern crate console;
extern crate dialoguer;

use std::env;
use std::thread;

use console::style;
use console::Term;
use dialoguer::Input;

fn child_thread(f: Box<Fn() + Send>) {
    let child = thread::spawn(move || f());
    child.join().expect("child failed to join");
}

fn read_and_echo() {
    let term = Term::stderr();
    let prompt = format!("{}", style(">>").blue().bold());
    let input = Input::new(&prompt);

    let msg = input.interact_on(&term).unwrap();
    println!("{}", msg);
}

fn main() {
    if env::args().find(|a| a == "--child").is_some() {
        child_thread(Box::new(read_and_echo));
    } else if env::args().find(|a| a == "--subchild").is_some() {
        child_thread(Box::new(|| {
            child_thread(Box::new(read_and_echo));
        }));
    } else {
        read_and_echo();
    }
}
