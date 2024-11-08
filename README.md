# Avatar Downloader

A simple Rust command-line tool that downloads random avatars from tapback.co and saves them as WebP files.

## Features

- Downloads multiple avatars in parallel
- Saves avatars with unique filenames
- Creates output directory automatically
- Handles errors gracefully
- Command-line interface

## Prerequisites

- Rust and Cargo installed on your system
- Internet connection to download avatars

## Installation

Clone this repository and build the project:

```bash
git clone https://github.com/yourusername/avatar-downloader
cd avatar-downloader
cargo build --release
```

## Usage

Run the program with two arguments:

1. Number of avatars to download
2. Output folder path

```bash
cargo run -- <number_of_avatars> <output_folder>
```

Example:

```bash
cargo run -- 5 ./avatars
```

This will download 5 avatars and save them in the `./avatars` directory.

## Dependencies

- `reqwest`: For making HTTP requests
- Standard Rust libraries (`std::fs`, `std::io`, etc.)

## Error Handling

The program handles various error cases:

- Invalid command-line arguments
- Network connection issues
- File system errors
- Invalid number format

Error messages will be displayed in the console if any issues occur during execution.

## License

[MIT License](LICENSE)

## Contributing

Feel free to open issues or submit pull requests with improvements.

Would you like me to add any additional sections or modify anything in the README?
