# Degree Conversion Tool

A simple, interactive Command Line Interface (CLI) tool written in Rust to convert temperatures between Fahrenheit and Celsius. It features robust input validation, a user-friendly loop, and integrated unit testing.

---

## Features

Bidirectional Conversion: Convert Fahrenheit to Celsius and vice versa.

Input Validation: Handles non-numeric inputs and invalid menu selections gracefully without crashing.

Automated Testing: Includes a test suite using the approx crate to ensure mathematical accuracy despite floating-point nuances.

User Experience: Includes a brief pause between conversions to allow the user to read the results.

---

## Prerequisites

### To run this tool, you need to have the Rust toolchain installed

Rust / Cargo: Install Rust

---

## Installation & Setup

Clone the repository:

``` Bash
git clone https://github.com/yourusername/degree-conversion-tool.git
cd degree-conversion-tool
```

Add Dependencies:
This project uses the approx crate for testing. Ensure your Cargo.toml includes:

``` Ini
[dev-dependencies]
approx = "0.5.1"
```

---

## Usage

Run the application using Cargo:

``` Bash
cargo run
```

Menu Options:

[1]: Enter a value in Fahrenheit to see the Celsius equivalent.

[2]: Enter a value in Celsius to see the Fahrenheit equivalent.

[3]: Safely exit the program.

---

## Running Tests

To verify the conversion logic and precision, run:

```Bash
cargo test
```

The tests check several edge cases, including freezing points, boiling points, and negative temperatures.

---

## Releases & CI/CD

This project uses GitHub Actions to automatically build and deploy the application.
Downloading Binaries

If you do not want to build the project from source, you can download the latest pre-compiled executables for Linux and Windows from the Releases page.
How to Create a New Release (For Maintainers)

The CI/CD pipeline is configured to trigger a build and create a GitHub Release only when a new version tag is pushed.

### Tag the current commit

``` Bash
git tag -a v1.0.0 -m "Release version 1.0.0"
```

### Push the tag to GitHub

```Bash
git push origin v1.0.0
```

### Automation: The "Release CI" workflow will automatically

Compile optimized binaries for Ubuntu and Windows.

Create a new Release entry on GitHub.

Attach the binaries as assets (e.g., degree-converter-x86_64-pc-windows-msvc.exe).

---

## License

This project is open-source and available under the **MIT License**.
