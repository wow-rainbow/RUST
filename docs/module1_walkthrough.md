# Module 1 Walkthrough · CLI Summation Program

This guide slows down the steps for Module 1 so you can build a small but real Rust program. Read a step, try it, and write notes in `diary/` as you go.

---

## 1. Reset the Playground

- Open a terminal at the repo root.
- Enter the lab crate: `cd lab`
- Make sure your shell knows about Cargo: `source "$HOME/.cargo/env"`
- Check the project still builds: `cargo check`

If `cargo check` fails, stop and fix the error before moving on.

---

## 2. Replace `main` with a Plan

Open `src/main.rs` and replace everything with:

```rust
fn main() {
    // Step 1: collect arguments (excluding the binary name).
    // Step 2: convert each argument to an i32.
    // Step 3: sum the numbers or report an error.
    // Step 4: print the result.
}
```

These comments act as your checklist.

---

## 3. Collect the Command-Line Arguments

- Add `use std::env;` at the top of the file.
- Inside `main`, start with:

```rust
let args: Vec<String> = env::args().skip(1).collect();
```

- Add a temporary debug print to inspect them:

```rust
println!("args: {:?}", args);
```

- Run `cargo run -- 1 2 3` and confirm you see `args: ["1", "2", "3"]`.

Once it works, delete the debug `println!`.

---

## 4. Parse the Numbers Safely

Add this helper function **above** `main`:

```rust
fn parse_args(args: &[String]) -> Result<Vec<i32>, String> {
    let mut numbers = Vec::new();

    for arg in args {
        match arg.parse::<i32>() {
            Ok(value) => numbers.push(value),
            Err(_) => return Err(format!("Could not parse '{arg}' as an integer")),
        }
    }

    Ok(numbers)
}
```

Read it carefully—notice how we borrow the slice (`&[String]`) instead of taking ownership.

Inside `main`, call it:

```rust
let numbers = match parse_args(&args) {
    Ok(nums) => nums,
    Err(message) => {
        eprintln!("Error: {message}");
        std::process::exit(1);
    }
};
```

Run `cargo run -- 1 two 3` to see the error handling in action.

---

## 5. Sum and Print the Total

Add the final steps in `main`:

```rust
let total: i32 = numbers.iter().sum();
println!("Total: {total}");
```

Test again: `cargo run -- 1 2 3` should print `Total: 6`.

---

## 6. Unit Test the Parser

At the bottom of the file, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_numbers() {
        let input = vec!["10".to_string(), "-3".to_string()];
        let numbers = parse_args(&input).expect("should parse");
        assert_eq!(numbers, vec![10, -3]);
    }

    #[test]
    fn rejects_bad_input() {
        let input = vec!["nope".to_string()];
        let error = parse_args(&input).unwrap_err();
        assert!(error.contains("nope"));
    }
}
```

Run `cargo test` to make sure both tests pass.

---

## 7. Clean Up

- Remove any leftover debug prints.
- Format the code: `cargo fmt`
- Lint it: `cargo clippy`
- Record what felt easy vs. confusing in your `diary` file.

When everything is green, congratulations—you just converted a Python-style script into idiomatic Rust!

---

## Bonus Practice

1. Support floating-point numbers using `f64` instead of `i32`.
2. Add a `--mean` flag that prints the average value.
3. Return `Result<()>` from `main` and use the `?` operator to bubble errors.

Take these only if you want extra reps; otherwise move on to Module 2 once you’re comfortable.
