# Rust and Slint Proof of Concept

## Project Overview

This project is a proof of concept demonstrating the use of Rust and Slint for building a simple logging application. It showcases how to integrate Slint for the UI and Rust for the backend logic.

## Project Structure

```
.
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── logger.rs
│   └── main.rs
└── ui
    └── app.slint
```

## Features

- Logging messages to a console UI.
- Manual input for logging messages.
- Automatic scrolling of the log console.

## How to Run

1. Ensure you have Rust and Cargo installed.
2. Clone the repository.
3. Run `cargo build` to build the project.
4. Run `cargo run` to execute the application.

## Dependencies

- `slint`: For building the UI.
- `slint-build`: For compiling Slint files during the build process.

## Notes

This project is a simple demonstration and can be extended with additional features and improvements.
