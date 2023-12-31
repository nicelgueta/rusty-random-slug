# Random Slug Generator
First rust project.

What it says on the tin - this generates random text slugs in Rust and prints to stdout.

## Usage

```bash
./rustyrs [length in words] [number of slugs]
```

Example:

```bash
./rustyrs 3 5

# Output:
characterized-synonymous-syntax
vanilla-bonnie-comedian
perceptual-accountant-china
worldwide-engraved-vocalist
visual-totalled-voltage
```

## Performance
- 1m x 2 word slugs: ~4.995s
- 1m x 5 word slugs: ~10.447s

## Building

```bash
cargo build --release
```

## Word Data
> Sourced from [Corpora](https://github.com/dariusk/corpora/blob/master/data/words) by [Darius Kazemi](https://github.com/dariusk)

## License
MIT
