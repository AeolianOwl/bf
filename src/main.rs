use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("Usage: bf input.bf");
        std::process::exit(1);
    }

    let file = std::fs::read(&args[0])?;

    eval(&file)?;

    Ok(())
}

#[derive(Debug)]
struct Error;
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")
    }
}

fn eval(input: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = [0u8; 65536];
    let mut dp = 0;
    let mut ip = 0;
    let mut input_buf = String::new();

    loop {
        match input.get(ip) {
            Some(b'>') => dp += 1,
            Some(b'<') => dp -= 1,
            Some(b'+') => {
                in_range(dp)?;
                data[dp] += 1;
            }
            Some(b'-') => {
                in_range(dp)?;
                data[dp] -= 1;
            }
            Some(b'.') => {
                in_range(dp)?;
                std::io::stdout().write_all(&[data[dp]])?;
                std::io::stdout().flush()?;
            }
            Some(b',') => {
                in_range(dp)?;
                std::io::stdout().write_all("\n> ".as_bytes())?;
                std::io::stdout().flush()?;
                std::io::stdin().read_line(&mut input_buf)?;
                data[dp] = input_buf.as_bytes()[0];
                input_buf.clear();
            }
            Some(b'[') => {
                in_range(dp)?;
                if data[dp] == 0 {
                    let mut depth = 1;
                    ip += 1;
                    loop {
                        match input.get(ip) {
                            Some(b'[') => depth += 1,
                            Some(b']') => {
                                depth -= 1;
                                if depth < 0 {
                                    return Err(Error.into());
                                }
                                if depth == 0 {
                                    break;
                                }
                            }
                            Some(_) => {}
                            None => break,
                        }
                        ip += 1;
                    }
                }
            }
            Some(b']') => {
                in_range(dp)?;
                if data[dp] != 0 {
                    let mut depth = 1;
                    ip -= 1;
                    loop {
                        match input.get(ip) {
                            Some(b']') => depth += 1,
                            Some(b'[') => {
                                depth -= 1;
                                if depth < 0 {
                                    return Err(Error.into());
                                }
                                if depth == 0 {
                                    break;
                                }
                            }
                            Some(_) => {}
                            None => break,
                        }
                        ip -= 1;
                    }
                }
            }
            Some(_) => {}
            None => break,
        }
        ip += 1;
    }

    Ok(())
}

fn in_range(dp: usize) -> Result<(), Box<dyn std::error::Error>> {
    if dp > 65535 {
        Err(Error.into())
    } else {
        Ok(())
    }
}
