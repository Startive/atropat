use std::io::{stdin, stdout, Read, Write};

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit the program...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
