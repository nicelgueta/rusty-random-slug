# Random Slug Generator
What it says on the tin - this generates random text slugs in Rust. 

Usable as a standalone binary, web applications as a WebAssembly module (WASM), or even as a Python module.

- [Rust Binary](#as-a-rust-binary)
- [Standalone Binary](#as-a-standalone-binary)
- [WASM Module](#as-a-wasm-module)
- [Python Module](#as-a-python-module)

## Why?
I needed a way to generate random slugs for a web project so thought it was a good opporunity to try out Rust's WebAssembly capabilities while also being able to use the same code as a zero-dependency python module for other projects.

### Key features
- No dependencies
- Fast
- Customisable slug length in words
- Over half a million unique combinations for 2-word slugs ranging up to over **280 trillion** unique combinations for 5-word slugs

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
from rustyrs import random_slugs
slugs: list[str] = random_slugs(3, 5)

# slugs: ['reflecting-unsealed-mamba', 'disabling-addicting-asp', 'pliable-begotten-barnacle', 'vaulting-telepathic-caracal', 'canonical-graven-beetle']
```

Other features:
- `get_slug(word_length: int) -> str`: Generate a single slug of a specific length
- `SlugGenerator(word_length: int)`: Create a generator object to generate slugs of a specific length. Can generate slugs infinitely.
    ```python
    from rustyrs import SlugGenerator
    gen = SlugGenerator(3)
    print(next(gen)) # 'unwieldy-unsuspecting-ant'
    ```
- `combinations(word_length: int) -> int`: Get the number of possible combinations for a given word length
    ```python
    from rustyrs import combinations
    print(combinations(2)) # 556,284
    ```

## Performance
- 1m x 2 word slugs: ~4.995s
- 1m x 5 word slugs: ~10.447s

## License
MIT
