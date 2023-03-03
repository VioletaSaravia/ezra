use ezra::Ezra;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ezra = Ezra::new("test")?;
    println!("{}", ezra.initial_message);

    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut input)?;

        let reply = ezra.reply(input.clone())?;
        println!("{reply}");
    }
}
