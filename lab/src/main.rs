use std::env; // Brings `env::args` into scope so we can read command-line arguments.

#[derive(Debug, PartialEq)]
enum Command {
    Add(Vec<i32>),
    List,
    Quit,
}

fn parse_command(raw: &[String]) -> Result<Command, String> {
    if raw.is_empty() {
        return Err("no input provided".into());
    }

    let verb = raw[0].to_lowercase();
    match verb.as_str() {
        "add" => {
            if raw.len() == 1 {
                return Err("add needs at least one number".into());
            }

            let numbers = raw[1..]
                .iter()
                .map(|piece| piece.parse::<i32>().map_err(|_| format!("bad number: {piece}")))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Command::Add(numbers))
        }
        "list" => Ok(Command::List),
        "quit" => Ok(Command::Quit),
        other => Err(format!("unknown command: {other}")),
    }
}

/// Borrow the provided argument list and return either parsed integers or a human-readable error.
fn parse_args(args: &[String]) -> Result<Vec<i32>, String> {
    // `&[String]` is a shared slice reference: `&` means "borrow" without taking ownership.
    let mut numbers = Vec::new(); // `Vec::new()` allocates an empty growable array on the heap.

    for arg in args {
        // `arg.parse::<i32>()` uses the `FromStr` trait; `::<i32>` (the "turbofish") selects the target type.
        match arg.parse::<i32>() {
            Ok(value) => numbers.push(value), // `=>` separates a match pattern from the code it runs.
            Err(_) => return Err(format!("Could not parse '{arg}' as an integer")),
            // `format!` builds a String using `{}` interpolation; `return` exits the function early.
        }
    }

    Ok(numbers) // `Ok(...)` wraps the success value inside the Result type.
}

fn main() {
    // `env::args()` yields an iterator over arguments; `skip(1)` drops the binary name; `collect()` gathers into Vec<String>.
    let args: Vec<String> = env::args().skip(1).collect();

    // `&args` borrows the vector so `parse_args` can read it without taking ownership.
    let numbers = match parse_args(&args) {
        Ok(nums) => nums,
        Err(message) => {
            eprintln!("Error: {message}"); // `eprintln!` writes to stderr for error reporting.
            std::process::exit(1); // Exit with a non-zero status to signal failure to the shell.
        }
    };

    // `iter()` yields references over the numbers; `sum()` consumes the iterator and adds them up.
    let total: i32 = numbers.iter().sum();
    println!("Total: {total}"); // `println!` prints with a newline to stdout.
}

#[cfg(test)]
mod tests {
    use super::*; // Bring `parse_args` into the test module so the tests can call it.

    #[test]
    fn parses_numbers() {
        // `vec![..]` is a macro that builds a Vec; `.to_string()` turns a string slice into an owned String.
        let input = vec!["10".to_string(), "-3".to_string()];
        let numbers = parse_args(&input).expect("should parse"); // `expect` unwraps Ok or panics with this message.
        assert_eq!(numbers, vec![10, -3]); // `assert_eq!` compares two values and panics if they differ.
    }

    #[test]
    fn rejects_bad_input() {
        let input = vec!["nope".to_string()];
        let error = parse_args(&input).unwrap_err(); // `unwrap_err` returns the Err value or panics if it was Ok.
        assert!(error.contains("nope")); // `assert!` checks a boolean expression.
    }
}
