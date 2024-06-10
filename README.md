
# Days Till Counter

![Demo Image 2](demo1.png)
![Demo Image 3 - progress bar](demo2.png)
Days Till Counter is a simple application built in Rust using GTK4 to count the days until a specific date.

## Features

- Set a target date to count down to.
- Display the number of days remaining until the target date.
- Simple and intuitive user interface.

## Prerequisites

Before you begin, ensure you have the following installed on your system:

1. **Rust**: Install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2. **GTK4**: Follow the steps provided in the [GTK4 installation guide](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html).

## Installation Steps

### 1. Clone the Repository

Clone the Days Till Counter repository to your local machine:

```bash
git clone https://github.com/DagmawiSolomon/DaysTillCountdownTimer
cd daystill
```

### 2. Build the Application

Use Cargo, the Rust package manager, to build the application:

```bash
cargo build --release
```

### 3. Run the Application

After a successful build, you can run the application using Cargo:

```bash
cargo run
```

## Troubleshooting

- Ensure all dependencies are correctly installed and configured.
- Check the `Cargo.toml` file for any missing dependencies or incorrect versions.
- If you encounter any issues, consult the [GTK4-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/) and the [Rust documentation](https://doc.rust-lang.org/book/).

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request on the [GitHub repository](https://github.com/DagmawiSolomon/DaysTillCountdownTimer).

## License

This project is licensed under the [MIT License](LICENSE).

