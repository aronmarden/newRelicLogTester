# New Relic Log Tester

New Relic Log Tester is a simple command-line tool written in Rust that sends user-inputted messages to the New Relic Log API. It prompts the user for their New Relic API Key on the first launch and then allows the user to send multiple log messages without having to re-enter the API key.

## Quick Use

If you are running macOS, head over to the releases tab, download and extract the new_relic_log_tester.tar.gz and simply run the binary in terminal ./new_relic_log_tester

You will most likely be told that new_relic_log_tester cannot be opened because Apple cannot check if for malicious software. You can choose to [Safely open apps on your Mac](https://support.apple.com/en-au/HT202491#:~:text=View%20the%20app%20security%20settings%20on%20your%20Mac&text=In%20System%20Preferences%2C%20click%20Security,%E2%80%9CAllow%20apps%20downloaded%20from.%E2%80%9D) if you wish. Otherwise, feel free to review the src/main.rs and cargo build or cargo run the app yourself locally with Rust.

## Prerequisites

- Rust programming language (1.39.0 or later): https://www.rust-lang.org/tools/install
- An active New Relic account: https://newrelic.com/signup
- An Ingest API Key: https://docs.newrelic.com/docs/apis/intro-apis/new-relic-api-keys/#license-key

## Usage

1. Clone this repository to your local machine:

   ```
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Build the New Relic Log Tester program:

   ```
   cargo build
   ```

3. Run the New Relic Log Tester program:

   ```
   cargo run
   ```

4. Enter your New Relic API Key when prompted:

   ```
   Please enter your New Relic API Key:
   ```

5. Enter a message to be sent to the New Relic Log API:

   ```
   Please enter a message, and it will be sent to the New Relic Log API (type 'exit' to quit):
   > Your log message here
   ```

6. The program will display "Log sent successfully!" if the log message was sent successfully. You can continue to enter messages by typing them after the `>` prompt. To exit the program, type `exit` and press Enter.

## Payload

The payload sent to the New Relic Log API includes the following keys:

- `timestamp`: The current Unix timestamp in seconds.
- `message`: The user-inputted log message.
- `logtype`: Set to "rustLogger".
- `description`: "This Log has come from the rust_logger testing tool. Please see https://github.com/aronmarden/newRelicLogTester for more details on why you are seeing this log",
- `apiKeyUsed`: A partially obfuscated version of the API key used (first 8 characters visible, followed by 8 asterisks).
