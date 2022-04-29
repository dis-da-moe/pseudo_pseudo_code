A fake fake esoteric programming language.

## What?

PseudoPseudoCode is a (mostly) functioning implementation of my computer science syllabus's pseudo-code guidelines, which are oddly specific for some reason. The result of implementing what's supposed to be a fake language is that the two pseudo's cancel each other out, which leaves us with generic Code.

## Code Example

Pseudo Pseudo Code is turing complete, with strings, arrays, built-in functions, loops and I/O. As a test, here is the bubble sort algorithm that I had to learn as part of my syllabus, implemented in real, runnable Code.

```
// before: 18, 98, 68, 92, 11, 3, 49, 98, 22

DECLARE upperBound : INTEGER
DECLARE lowerBound : INTEGER
DECLARE index : INTEGER
DECLARE swap : BOOLEAN
DECLARE temp : INTEGER
DECLARE top : INTEGER
upperBound ← 8
lowerBound ← 0
top ← upperBound
REPEAT
    swap ← FALSE 
    FOR index ← lowerBound TO top - 1
        IF myList[index] > myList[index + 1] 
            THEN
            temp ← myList[index] 
            myList[index] ← myList[index + 1]
            myList[index + 1] ← temp
            swap ← TRUE 
        ENDIF
    NEXT
    top ← top -1
UNTIL (NOT swap) OR (top = 0) 

// after: 3, 11, 18, 22, 49, 68, 92, 98, 98
```

The full version can be seen in `examples/bubbleSort.psps`

## Motivation

I was bored one time in class, and after experiencing a lost mark in a test because I forgot to include an `ENDPROCEDURE` at the end of my answer, I wanted to see if the Pseudo Code guidelines that my syllabus had were specific enough for an actual language to be implemented, and it turns out they definitely were. I don't know why the guidelines go so far as to specify the format and syntax of **fake** code, but they do. Some could argue that me implementing this language is going too far, however this was actually a fun project and a good excuse to learn more about Rust and programming languages.

## Running

Download the .exe and run it in the command line with the path to the desired file as the first argument. I've included examples in the `examples` folder, with `error.psps` deliberately erroring in order to show the nice error reporting.  If you want to write your own PseudoPseudoCode for some weird reason, just make a file with the `.psps` extension and run that through the command line. There's also a VS Code extension for syntax highlighting in the `extension` directory, available as a `.vsix` file.

## Building

Clone the repo, have rustup installed, then type `cargo run [FILEPATH]` into your terminal of choice. To test, type `cargo test`, which will run the examples to make sure nothing's broken.

## Contributors

This is a solo project, but I'd like to thank those who've contributed to the [Chumsky](https://github.com/zesterer/chumsky) and [Ariadne](https://github.com/zesterer/ariadne) crates for parsing and error reporting respectively. The Rust community is also very helpful, with the #lang-dev channel in the [community discord](https://discord.gg/rust-lang-community) providing valuable insight.

## Contributing 

If you find any issues or things that I could improve on, feel free to open an issue or PR. I still have some things that I'd like to implement, including:
- User declared functions and procedures
- Records and 2D arrays
- Modules, to allow multiple files to be run together
- More appendix functions and the rest of the datatypes

In order for PseudoPseudoCode to be a truly faithful implementations of a fake language.

## License

MIT, do what you'd like, if you want to credit me just link to this repo.
