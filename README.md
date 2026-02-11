# classical-poem-llm

A Rust workspace that trains and runs a simple n-gram language model on classical poems. The project includes CLI tools for training and text generation, plus shared crates for tokenization, modeling, and logging.

## Features

- **Train** an n-gram model from a directory of poem text files.
- **Generate** text from a trained model using a prompt.
- **Tokenization** and **serialization** built-in.
- **Verbose/quiet logging** controls.

## Workspace Layout

```
classical-poem-llm/
├── crates/
│   ├── logger/      # Simple logging helper
│   ├── model/       # N-gram model + serialization
│   ├── runner/      # CLI: generate text from a model
│   ├── tokenizer/   # Tokenization + vocab management
│   └── trainer/     # CLI: train a model
├── poems/           # Example training data (txt files)
├── Cargo.toml       # Workspace definition
└── README.md
```

## Crates

### `logger`
Minimal logger with `info`, `warn`, `error`, and `always` methods. Controls verbosity via `verbose` and `quiet`.

### `tokenizer`
Whitespace tokenizer that lowercases text, tracks vocabulary, and maps tokens to ids (and back). Serializable via `bincode`.

### `model`
N-gram model (`NGramModel`) that:
- Builds context → next-token count maps
- Predicts the most frequent next token
- Saves/loads a serializable model (including the tokenizer)

### `trainer`
CLI tool that:
- Reads `.txt` files in a directory
- Tokenizes them into a unified stream
- Trains an n-gram model
- Saves it to disk

### `runner`
CLI tool that:
- Loads a saved model
- Tokenizes a prompt
- Generates tokens by predicting next tokens

## Requirements

- Rust (edition 2024)
- Cargo

## Getting Started

### 1) Build the workspace

From the project root:

```
cargo build
```

### 2) Train a model

By default, training reads from the `poems/` directory and saves `model.bin`:

```
cargo run -p trainer -- \
  --input-dir poems \
  --context-size 5 \
  --output model.bin
```

### 3) Generate text

Use the `runner` CLI to generate text:

```
cargo run -p runner -- \
  --model-path model.bin \
  --prompt "the night was" \
  --length 20
```

## CLI Usage

### Trainer (`crates/trainer`)

```
cargo run -p trainer -- [OPTIONS]
```

Options:

- `-i, --input-dir <DIR>`: Directory of `.txt` poems (default: `poems`)
- `-c, --context-size <N>`: N-gram context size (default: `5`)
- `-o, --output <FILE>`: Output model file (default: `model.bin`)
- `-v, --verbose`: Enable verbose logging
- `-q, --quite`: Enable quiet mode

Example:

```
cargo run -p trainer -- -i poems -c 4 -o model.bin -v
```

### Runner (`crates/runner`)

```
cargo run -p runner -- [OPTIONS]
```

Options:

- `-m, --model-path <FILE>`: Model file path (default: `model.bin`)
- `-p, --prompt <TEXT>`: Prompt to start generation (default: `the night was`)
- `-l, --length <N>`: Number of tokens to generate (default: `20`)
- `-v, --verbose`: Enable verbose logging
- `-q, --quiet`: Enable quiet mode

Example:

```
cargo run -p runner -- -m model.bin -p "once upon a midnight" -l 30
```

## How It Works

1. **Training**
   - Each `.txt` file is read and tokenized into lowercase word tokens.
   - Tokens are collected into one stream.
   - For each sliding window of size `context_size + 1`, the model counts how often each token follows each context.

2. **Generation**
   - The prompt is tokenized and padded/trimmed to the model’s context size.
   - The next token is chosen as the most frequent continuation for the current context.
   - If no continuation exists, a random context is chosen from the model.

## Data

Example poems are stored in:

```
poems/
```

You can add your own `.txt` files there or point the trainer to another directory.

## Serialization

Models are saved using `bincode` and include:

- `context_size`
- Context → next-token counts
- Tokenizer (vocabulary)

## License

This project is licensed under **GPL-3.0-or-later**. See `LICENSE` for details.
