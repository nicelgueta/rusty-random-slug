# Random Slug Generator
What it says on the tin - this generates random text slugs in Rust. WASM-compatible.

## Usage

### As a Rust binary
```bash
cargo run --release [length in words] [number of slugs]
```

### As a standalone binary
```bash
./rustyrs [length in words] [number of slugs]
```

### WASM
```bash
wasm-pack build --target web --features wasm
```

In JS/TS:
```ts
import init, { random_slugs } from './pkg/rustyrs.js';
init();
const slugs: string[] = random_slugs(3, 5);
console.log(slugs);
```
>See index.html for a full example


Example Output:

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

## Word Data
> Sourced from [Corpora](https://github.com/dariusk/corpora/blob/master/data/words) by [Darius Kazemi](https://github.com/dariusk)

## License
MIT
