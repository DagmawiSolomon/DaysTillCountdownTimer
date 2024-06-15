Sure, here are the updated instructions including help messages for Windows and macOS users to find the path to the compiled binary file and set up automatic running:

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

## Finding the Compiled Binary

After building the application, the compiled binary file will be located in the `target/release` directory within your project folder. You can find the path to the compiled binary file by navigating to this directory:

```bash
cd target/release
```

The binary file will be named `daystill` (or `daystill.exe` on Windows). You can use the full path to this binary file in your system's task scheduler or crontab to automate running the program.

### Windows

To find the path to the compiled binary:

1. Open File Explorer and navigate to your project directory.
2. Go to the `target\release` folder.
3. Note the full path to `daystill.exe`.

To run your program every day at a specific time or every time the computer boots:

1. Open Task Scheduler.
2. Click on "Create Basic Task".
3. Follow the prompts to set the task name, description, and trigger (daily or at startup).
4. In the "Action" section, select "Start a program" and browse to the location of `daystill.exe`.
5. Complete the setup and save the task.

### macOS

To find the path to the compiled binary:

1. Open Finder and navigate to your project directory.
2. Go to the `target/release` folder.
3. Note the full path to `daystill`.

To run your program every day at a specific time or every time the computer boots:

1. Open Terminal.
2. Type `crontab -e` to edit your user's crontab file.
3. Add the following lines to run your program at reboot or daily at a specific time:

```bash
# To run the program every time the computer boots
@reboot /path/to/your/program
# To run the program at a specific time every day
30 14 * * * /path/to/your/program
```

Replace `/path/to/your/program` with the path to your compiled binary file. For example, if your binary file is located in `target/release/daystill`, you would use `/full/path/to/target/release/daystill`.

### Linux

To run your program every day at a specific time or every time the computer boots:

1. Open a terminal.
2. Type `crontab -e` to edit your user's crontab file.
3. Add the following lines to run your program at reboot or daily at a specific time:

```bash
# To run the program every time the computer boots
@reboot /path/to/your/program
# To run the program at a specific time every day
30 14 * * * /path/to/your/program
```

Replace `/path/to/your/program` with the path to your compiled binary file. For example, if your binary file is located in `target/release/daystill`, you would use `/full/path/to/target/release/daystill`.

## Troubleshooting

- Ensure all dependencies are correctly installed and configured.
- Check the `Cargo.toml` file for any missing dependencies or incorrect versions.
- If you encounter any issues, consult the [GTK4-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/) and the [Rust documentation](https://doc.rust-lang.org/book/).

## Configuration

The application can be configured using a `config.json` file. For detailed information on how to configure the application, see the [Configuration Guide](CONFIGURATION.md).

## Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue or submit a pull request on the [GitHub repository](https://github.com/DagmawiSolomon/DaysTillCountdownTimer).

## License

This project is licensed under the [MIT License](LICENSE).