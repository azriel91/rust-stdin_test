use std::env;
use std::io;
use std::thread;

fn child_thread(f: Box<Fn() + Send>) {
    let child = thread::spawn(move || f());
    child.join().expect("child failed to join");
}

fn read_and_echo() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input.truncate(n);
            println!("{}", input);
        }
        Err(error) => panic!("error: {}", error),
    }
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
