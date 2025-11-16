# API Documentation for LingCode Rime

## Overview

The LingCode Rime project provides a flexible input method framework that supports Pinyin (Simplified and Traditional) and Double Pinyin. This document outlines the API available for developers to interact with the framework.

## Modules

### Core

The core module provides the foundational functionalities required for the input method framework.

- **Functions**
  - `initialize()`: Initializes the core components of the input method.
  - `shutdown()`: Cleans up resources used by the core components.

### Engine

The engine module handles the input processing and candidate generation.

- **Functions**
  - `process_input(input: &str) -> Vec<Candidate>`: Processes the input string and returns a list of candidate words.
  - `set_input_state(state: InputState)`: Sets the current input state.

### Pinyin

The Pinyin module provides functionalities for handling Pinyin input.

- **Functions**
  - `convert_to_simplified(pinyin: &str) -> String`: Converts Traditional Pinyin to Simplified Pinyin.
  - `convert_to_traditional(pinyin: &str) -> String`: Converts Simplified Pinyin to Traditional Pinyin.

### Double Pinyin

The Double Pinyin module provides functionalities for handling Double Pinyin input.

- **Functions**
  - `get_double_pinyin_mapping() -> HashMap<char, String>`: Returns the mapping of Double Pinyin.

### Dictionary

The Dictionary module manages the word dictionary used for candidate generation.

- **Functions**
  - `load_dictionary(file_path: &str)`: Loads a dictionary from the specified file path.
  - `get_word_frequency(word: &str) -> usize`: Returns the frequency of a given word in the dictionary.

### Converters

The Converters module provides utilities for converting between different formats.

- **Functions**
  - `s2t(text: &str) -> String`: Converts Simplified Chinese text to Traditional Chinese.
  - `t2s(text: &str) -> String`: Converts Traditional Chinese text to Simplified Chinese.

### FFI

The Foreign Function Interface (FFI) module allows interaction with other programming languages.

- **Functions**
  - `ffi_initialize()`: Initializes the FFI layer.
  - `ffi_shutdown()`: Shuts down the FFI layer.

## Usage Example

```rust
use lingcode_rime::engine;

fn main() {
    engine::initialize();
    let candidates = engine::process_input("ni hao");
    for candidate in candidates {
        println!("{}", candidate);
    }
    engine::shutdown();
}
```

## Conclusion

This API documentation provides a high-level overview of the available modules and functions within the LingCode Rime project. For more detailed information, please refer to the individual module documentation.