use std::io::{BufReader, self, BufRead};

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let mut input_handle = BufReader::new(io::stdin());
    let mut output_handle = io::stdout();

    Ok(for line in input_handle.lines() {
        let line_buf = line?;
        match line_buf.as_str() {
            _ => println!("{}", line_buf),
        }
    })
}
