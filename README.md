# LingCode Rime Rust Input Method Framework

LingCode Rime is a Rust-based input method framework that aims to provide a flexible and efficient way to input Chinese characters using Pinyin (Simplified and Traditional) and Double Pinyin. This project is designed to be cross-platform, supporting Windows, macOS, Linux, iOS, and Android.

## Features

- **Pinyin Input**: Supports both Simplified and Traditional Pinyin input methods.
- **Double Pinyin**: Implements Double Pinyin input for efficient typing.
- **Cross-Platform**: Compatible with major operating systems including Windows, macOS, Linux, iOS, and Android.
- **Modular Design**: Built with a modular architecture, allowing easy extension and customization.

## Project Structure

- `crates/`: Contains the core libraries for the input method framework.
  - `core/`: Core functionalities of the input method.
  - `engine/`: Input processing engine.
  - `pinyin/`: Pinyin input handling.
  - `double-pinyin/`: Double Pinyin input handling.
  - `dict/`: Dictionary management.
  - `converters/`: Conversion utilities between Simplified and Traditional Chinese.
  - `ffi/`: Foreign Function Interface for integration with other languages.

- `frontends/`: Contains platform-specific frontends.
  - `desktop/`: Desktop applications for Windows, macOS, and Linux.
  - `mobile/`: Mobile applications for Android and iOS.

- `bindings/`: Language bindings for Node.js and Python.

- `examples/`: Example applications demonstrating usage of the framework.

- `tests/`: Contains integration tests and fixtures.

- `docs/`: Documentation including design and API references.

## Getting Started

To get started with the LingCode Rime project, clone the repository and build the project using Cargo:

```bash
git clone <repository-url>
cd lingcode-rime-rs
cargo build
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.