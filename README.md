# LolRust 🐱

![lolrust — i can haz rust?](hero-banner.jpg)

**A programming language based on Rust, but in lolcat speak.**

Write real, compiled programs using keywords like `i can haz wiggly`, `zoomies`, and `yolo`. LolRust transpiles `.meow` files into valid Rust, then compiles them with `rustc`. You get Rust's type system, borrow checker (the **No Touchie Checker**), and zero-cost abstractions — but with 100% more cat.

## Quick Start

```bash
# Install
cargo install --path .

# Hello world
echo 'iz main() { meow!("Oh hai world! :3"); }' > hello.meow
lolrust hello.meow --run
```

## Example

```rust
// fizzbuzz.meow

iz main() {
    chase n around 1..=100 {
        if ceiling cat sez n % 15 == 0 {
            meow!("FizzBuzz");
        } or if ceiling cat sez n % 3 == 0 {
            meow!("Fizz");
        } or if ceiling cat sez n % 5 == 0 {
            meow!("Buzz");
        } or basement cat sez {
            meow!("{}", n);
        }
    }
}
```

Yes, that compiles. Yes, it runs. No, we are not sorry.

## The Language

### Keywords

| LolRust | Rust | Why |
|---------|------|-----|
| `iz` | `fn` | because everything `iz` something |
| `i can haz` | `let` | the original |
| `i can haz wiggly` | `let mut` | it wiggles because it can change |
| `meow!` | `println!` | cats meow, programs print |
| `hisss!` | `eprintln!` | angry output |
| `ohno!` | `panic!` | oh no |
| `loaf` | `struct` | cats are just loaves |
| `teech` | `impl` | teach the loaf new tricks |
| `flavurz` | `enum` | different flavors of a thing |
| `skillz` | `trait` | what a kitteh can do |
| `sniff` | `match` | cats investigate by sniffing |
| `if ceiling cat sez` | `if` | ceiling cat watches over all |
| `or if ceiling cat sez` | `else if` | ceiling cat has more to say |
| `or basement cat sez` | `else` | the dark alternative |
| `zoomies` | `loop` | infinite energy |
| `chase` | `for` | cats chase things |
| `around` | `in` | chase things *around* |
| `keep goin while` | `while` | self-explanatory |
| `flop` | `break` | cat flops down, loop stops |
| `again` | `continue` | do it again! |
| `cough up` | `return` | like a hairball, but it's a value |
| `everycat` | `pub` | visible to everycat |
| `gimme` | `use` | gimme that module |
| `box` | `mod` | cats love boxes |
| `yolo` | `unsafe` | you only live once (cats have 9 tho) |
| `lazee` | `async` | will do it later |
| `waitforit` | `await` | ...wait for it... |
| `yoink` | `move` | YOINK! mine now |
| `copycat` | `.clone()` | makes a copy of ur cat |
| `wiggly` | `mut` | it wiggles |
| `forever` | `const` | forever and ever |
| `stickycat` | `static` | cat that won't move |
| `kinda` | `type` | kinda like a type alias |
| `pretend iz` | `as` | pretend it's something else |
| `but only if` | `where` | conditions apply |
| `dis` | `self` | dis right here |
| `Dis` | `Self` | Dis type right here |
| `chonk` | `super` | the big cat up a level |

### Types

| LolRust | Rust | Why |
|---------|------|-----|
| `Yarn` | `String` | cats love yarn |
| `Pile` | `Vec` | a pile of things |
| `Cardboard` | `Box` | cats WILL sit in the cardboard |
| `MaybeCheezburgr` | `Option` | maybe there's a cheeseburger, maybe not |
| `Has` | `Some` | it has the thing! |
| `EmptyBowl` | `None` | the bowl is empty :( |
| `Tryz` | `Result` | try and see what happens |
| `Purrfect` | `Ok` | purrfect, no errors |
| `Hairball` | `Err` | coughed up a hairball (error) |

### Booleans

| LolRust | Rust |
|---------|------|
| `yus` | `true` |
| `nope` | `false` |

## More Examples

### The Guessing Game

```rust
gimme std::io;

iz main() {
    i can haz secret = 42;

    meow!("=== GESSING GAEM ===");
    meow!("I iz thinking of a numbr between 1 and 100...");

    zoomies {
        meow!("Gess a numbr:");

        i can haz wiggly input = Yarn::new();
        io::stdin().read_line(&wiggly input).unwrap();

        i can haz guess: i32 = sniff input.trim().parse() {
            Purrfect(num) => num,
            Hairball(_) => {
                meow!("Dat iz not a numbr! Try again kitteh.");
                again;
            }
        };

        if ceiling cat sez guess == secret {
            meow!("U FOUND IT! :3 Ur so smart!");
            flop;
        } or if ceiling cat sez guess < secret {
            meow!("Too smol! Think bigger.");
        } or basement cat sez {
            meow!("Too chonk! Think smaller.");
        }
    }
}
```

### Structs and Impls

```rust
loaf Cat {
    name: Yarn,
    lives: i32,
    iz_chonky: bool,
}

teech Cat {
    iz new(name: Yarn) -> Dis {
        Dis {
            name,
            lives: 9,
            iz_chonky: yus,
        }
    }

    iz speak(&dis) {
        meow!("{} sez: MEOW!", dis.name);
    }
}

iz main() {
    i can haz kitty = Cat::new(Yarn::from("Sir Fluffington III"));
    kitty.speak();
}
```

### YOLO Mode

```rust
iz main() {
    i can haz wiggly num = 42;

    yolo {
        // HERE BE DRAGONS
        i can haz ptr = &wiggly num pretend iz *wiggly i32;
        *ptr = 1337;
    }

    meow!("num iz now: {}", num);
}
```

## CLI Usage

```
lolrust <file.meow> [OPTIONS]

Options:
  --emit-rs       Output transpiled .rs file instead of compiling
  --run           Compile and immediately run the result
  -o <PATH>       Output file path
  -v, --verbose   Show transpiled Rust code before compiling
  --explain       Translate rustc errors into lolcat speak
  -h, --help      Show help
  -V, --version   Show version
```

### The `--explain` Flag

When your code has errors, `--explain` translates rustc's messages into lolcat:

```
OH NOES[E0308]: WRONG FLAVOR! mismatched types
 --> lolrust_42.rs:4:21
  |
4 |     let y: String = 42;
  |            ------   ^^ i wanted `String`, but i got integer
  |            |
  |            i wanted due to this

HALP: try using a conversion method
```

The No Touchie Checker speaks:
- `error:` becomes `OH NOES:`
- `cannot borrow` becomes `NO TOUCHIE! cannot borrow`
- `cannot move out of` becomes `HEY! dat iz not urs to yoink from`
- `unused variable` becomes `u made dis but never played with it`
- `help:` becomes `HALP:`
- `warning:` becomes `CAREFUL KITTEH:`

## Kibble - The Package Manager

Because every language needs a package manager, and every package manager needs a cat pun.

```bash
# Create a new project
lolrust kibble init my_kitteh_project

# Build (outputs to litter_box/)
lolrust kibble build

# Build and run
lolrust kibble run

# Clean build artifacts
lolrust kibble clean

# Show project info
lolrust kibble info
```

Projects use `Kibble.toml` for configuration:

```toml
[kitteh]
name = "my_kitteh_project"
version = "0.1.0"
description = "a very important kitteh project"
author = "Anonymous Kitteh"

[dependencies]
# add ur dependencies here!

[treats]
# extra compiler flags
```

Build artifacts go in the `litter_box/` directory. Because that's where the output goes.

## VS Code Extension

Syntax highlighting and snippets for `.meow` files live in the `lolrust-vscode/` directory.

**Features:**
- Full syntax highlighting for all 48 keywords (including multi-word ones)
- 20 code snippets (`izmain`, `meow`, `icanhazwiggly`, `zoomies`, `sniff`, `yolo`, etc.)
- Bracket matching, auto-closing pairs, comment toggling

**Install:**
```bash
# Copy to VS Code extensions directory
cp -r lolrust-vscode ~/.vscode/extensions/lolrust

# Restart VS Code, open a .meow file, and bask in the glory
```

## How It Works

LolRust is a **transpiler** — it converts `.meow` files into valid Rust, then compiles them with `rustc`.

The transpiler uses a **single-pass character scanner** that:
1. Tracks context (strings, comments, code) so keywords inside `"strings"` and `// comments` are never replaced
2. Tries **longest matches first** so `i can haz wiggly` becomes `let mut`, not `let wiggly`
3. Checks **word boundaries** so `wizard` doesn't become `wfnard` and `dismiss` doesn't become `selfmiss`

It's ~350 lines of Rust that transforms cat speak into systems programming. The future is meow.

## Building from Source

```bash
# Clone
git clone https://github.com/YOUR_USERNAME/lolrust.git
cd lolrust

# Build
cargo build --release

# Run tests
cargo test

# Install globally
cargo install --path .
```

## FAQ

**Q: Is this a real programming language?**
A: It transpiles to Rust and compiles to native binaries. It's as real as it gets.

**Q: Should I use this in production?**
A: The real question is: should you NOT use this in production?

**Q: How do I explain this on my resume?**
A: "Designed and implemented a compiled systems programming language with a custom transpiler, package manager, and IDE support."

**Q: Why?**
A: For the shits and the giggles.

## Lore

For the devout, [*The Book of Loaf*](https://skulk.ai/lore/book-of-loaf/) is the canonical LolRust scripture — mock-theological commentary on the No Touchie Checker, the zoomies, and other feline mysteries. Seven volumes, Vol. II is in, the rest are coming.

## License

MIT - Do whatever you want. Ceiling Cat is watching though.

---

*Made with `yolo` energy and zero regrets.*
