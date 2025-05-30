# Random Slug Generator
What it says on the tin - this generates random text slugs in Rust.

**PyPi**

[![Downloads](https://static.pepy.tech/badge/rustyrs)](https://pepy.tech/project/rustyrs)

Usable as a standalone binary, web applications as a WebAssembly module (WASM), or even as a Python module.

- [Python Module](#as-a-python-module)
- [WASM Module](#as-a-wasm-module)
- [Rust Binary](#as-a-rust-binary)
- [Standalone Binary](#as-a-standalone-binary)

## Why?
I needed a way to generate random slugs for a web project so thought it was a good opporunity to try out Rust's WebAssembly capabilities while also being able to use the same code as a zero-dependency python module for other projects.

## Key features
- Generates unique random slugs for a input length in words
- Fast
- Zero dependencies (python and wasm)
- Pre-filtered to avoid dodgy or rude vocabulary
- Customisable slug length in words
- Over half a million unique combinations for 2-word slugs ranging up to over **280 trillion** unique combinations for 5-word slugs.

## Usage

### As a Python module

#### Install from PyPI
```bash
pip install rustyrs
```

OR

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
- `SlugGenerator(word_length: int)`: Create a generator object to generate slugs of a specific length. Will generate slugs until all unique permutations have been reached.
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
- `EternalSlugGenerator(word_length: int)`: Create iteration suffixed slugs forever. Guaranteed uniqueness.
  ```python
    from rustyrs import EternalSlugGenerator, combinations
    gen = EternalSlugGenerator(2)
    first = [next(gen) for _ in range(combinations(2))] # max no of combinations iterated
    first[-1] # 'listening-tench-0'
    next(gen) # 'existent-walrus-1'
  ```

#### Python Performance
- 0.5 million x 2 word slugs: **~210ms**
  ```bash
  time python -c "import rustyrs as r;a = set(r.random_slugs(2, 556_284));assert len(a) == 556_284"
  real    0m0.219s
  user    0m0.211s
  sys     0m0.000s
  ```
- 1 million x 5 word slugs: **~524ms**
  ```bash
  time python -c "import rustyrs as r;a = set(r.random_slugs(5, 1_000_000));assert len(a) == 1_000_000"
  real    0m0.667s
  user    0m0.524s
  sys     0m0.051s
  ```

__________________

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


## License
MIT
