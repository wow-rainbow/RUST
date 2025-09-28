# Module 2 Walkthrough · Control Flow and Pattern Matching

This module turns the summation tool into a command-driven CLI so you can practice `if`, `match`, and iterator-driven loops. Expect to edit `lab/src/main.rs`, add a small command enum, and grow tests to cover each branch.

---

## 0. Setup Checklist

1. Open a terminal in `/workspaces/RUST`.
2. Enter the crate: `cd lab`
3. Ensure the toolchain is loaded: `source "$HOME/.cargo/env"`
4. Sanity-check the project: `cargo test`

If any command fails, fix the issue first; the exercises assume a clean slate.

---

## 1. Define the Command Enum

Add this near the top of `src/main.rs` (above `parse_args`):

```rust
#[derive(Debug, PartialEq)]
enum Command {
    Add(Vec<i32>),
    List,
    Quit,
}
```

- `#[derive(...)]` auto-implements traits so you can print (`Debug`) and compare (`PartialEq`) in tests.
- Each variant demonstrates different data shapes: one with payload, two without.

---

## 2. Parse Raw Strings Into Commands

Create a helper below the enum:

```rust
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
```

Key patterns to notice:
- `raw[1..]` slices off the first element for the operands.
- `map` + `collect::<Result<...>>` accumulates parsed integers or bubbles the first error.
- The final `_` catch-all arm surfaces typos.

Run `cargo check` to ensure no syntax errors slip in.

---

## 3. Drive Control Flow From `main`

Replace the body of `main` with:

```rust
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let command = match parse_command(&args) {
        Ok(cmd) => cmd,
        Err(message) => {
            eprintln!("Error: {message}");
            std::process::exit(1);
        }
    };

    match command {
        Command::Add(numbers) => {
            let total: i32 = numbers.iter().sum();
            println!("Total: {total}");
        }
        Command::List => {
            println!("(pretend we listed saved results)");
        }
        Command::Quit => {
            println!("Goodbye!");
        }
    }
}
```

- The outer `match` chooses behavior based on the command variant.
- `Command::Add(numbers)` uses pattern destructuring to move the vector out.
- The placeholder branches prep you for storing data later; keep them simple for now.

Run `cargo run -- add 4 5 6` and confirm `Total: 15` shows up.

---

## 4. Add Tests Covering Each Match Arm

Append to the `tests` module:

```rust
    #[test]
    fn parses_add_command() {
        let raw = vec!["add".into(), "2".into(), "3".into()];
        let cmd = parse_command(&raw).expect("should parse add");
        assert_eq!(cmd, Command::Add(vec![2, 3]));
    }

    #[test]
    fn rejects_unknown_command() {
        let raw = vec!["dance".into()];
        let err = parse_command(&raw).unwrap_err();
        assert!(err.contains("unknown"));
    }

    #[test]
    fn reports_empty_input() {
        let raw: Vec<String> = Vec::new();
        let err = parse_command(&raw).unwrap_err();
        assert!(err.contains("no input"));
    }
```

Run `cargo test` until everything passes. If a test fails, read the message, revisit the implementation, and rerun.

---

## 5. Stretch Goals (Optional)

1. Make `list` print previously added totals: store results in a file or in-memory static (`Mutex<Vec<i32>>`).
2. Accept `--help` that prints usage information.
3. Support multiple commands in one invocation (e.g., `add 1 2 ; add 3 4`).

Try these only after the basic flow feels natural.

---

## 6. Reflect

Update `diary` with:
- Which `match` arms were clearest vs. trickiest.
- How slicing (`raw[1..]`) compares to Python slicing.
- Any notation (`?`, `::<i32>`, `into()`) you want to revisit later.

Once comfortable, head toward Module 3 where ownership rules become front and center!
