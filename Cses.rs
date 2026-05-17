use std::io::{self, Write};

fn hanoi<W: Write>(n: u32, from: u32, to: u32, aux: u32, out: &mut W) -> io::Result<()> {
    if n == 0 {
        return Ok(());
    }
    hanoi(n - 1, from, aux, to, out)?;
    writeln!(out, "{} {}", from, to)?;
    hanoi(n - 1, aux, to, from, out)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let n: u32 = input.trim().parse().unwrap_or(0);
    let k = (1u64 << n) - 1;

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{}", k)?;
    hanoi(n, 1, 3, 2, &mut handle)
}
