# Random Slug Generator
What it says on the tin - this generates random text slugs in Rust. 

Usable as a standalone binary, web applications as a WebAssembly module (WASM), or even as a Python module.

- [Rust Binary](#as-a-rust-binary)
- [Standalone Binary](#as-a-standalone-binary)
- [WASM Module](#as-a-wasm-module)
- [Python Module](#as-a-python-module)

## Why?
I needed a way to generate random slugs for a web project so thought it was a good opporunity to try out Rust's WebAssembly capabilities while also being able to use the same code as a zero-dependency python module for other projects.

## Usage

### As a Rust binary
```bash
cargo run --release [length in words] [number of slugs]
```

### As a standalone binary
```bash
cargo build --release
[build path]/rustyrs [length in words] [number of slugs]
```

#### Example Output
```
proctor-slimmer-guillemot
unsafe-warlike-avocado
garbled-pulled-stork
answerable-quick-whale
floral-apportioned-bobcat
```
____________


### As a WASM module
```bash
# If wasm pack is not already installed
cargo install wasm-pack 

# build the WASM module
wasm-pack build --target web --features wasm
```

Then from JS/TS:
```ts
import init, { random_slugs } from './pkg/rustyrs.js';
init();
const slugs: string[] = random_slugs(3, 5);
console.log(slugs);

// slugs: ['postpartum-regal-taipan', 'devastating-elven-salamander', 'immense-ambivalent-wren', 'philosophical-bandaged-gaur', 'outlaw-noncommercial-sunfish']
```
>See index.html for a full example

____________

### As a Python module

#### Install from PyPI
```bash
pip install rustyrs
```

#### Build from source
```bash
python -m venv venv
source venv/bin/activate
pip install maturin
maturin develop --features python
```

Then from Python:
```python
from rustyrs import generate_slugs
slugs: list[str] = generate_slugs(3, 5)

# slugs: ['reflecting-unsealed-mamba', 'disabling-addicting-asp', 'pliable-begotten-barnacle', 'vaulting-telepathic-caracal', 'canonical-graven-beetle']
```

## Performance
- 1m x 2 word slugs: ~4.995s
- 1m x 5 word slugs: ~10.447s

## Word Data
> Sourced from [Corpora](https://github.com/dariusk/corpora/blob/master/data/words) by [Darius Kazemi](https://github.com/dariusk)

## License
MIT
