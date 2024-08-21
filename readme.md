# Stock Token Program

## Overview

This is a proof of concept transfer-hook program that exibits the power of token extensions for interesting mechanics. It ensures token transfers occur only during market hours and validates the state of token accounts before processing transfers. The program also manages additional account metadata necessary for these operations.

### Key Functions

1. **Token Transfer Execution**:
   - Validates that token accounts are in a "transferring" state.
   - Ensures transfers only occur during market hours (9:30 AM to 4:00 PM ET, Monday to Friday).
   - Executes the token transfer if conditions are met.

2. **Account Metadata Management**:
   - Initializes and updates extra metadata for token accounts.
   - Ensures metadata is associated correctly with the respective token mint and authority.

### Utilities

The program includes utility functions to:

- Determine daylight saving time.
- Convert UTC to Eastern Time.
- Find specific weekdays in a given month.
- Check if the current time falls within market hours.

### Error Handling

Handles various errors related to transfer operations and market conditions, such as:

- Attempts to transfer outside market hours.
- Invalid seeds or missing signatures.

## Instructions

### Prerequisites

- Rust and Cargo installed
- Solana CLI and Rust SDK installed

### Build and Deploy

1. Clone the repository:

    ```sh
    git clone <repository-url>
    cd <repository-directory>
    ```

2. Build the program using Cargo:

    ```sh
    cargo build-bpf
    ```

3. Deploy the program to your Solana cluster:

    ```sh
    solana program deploy /path/to/program.so
    ```

4. Test the program using Solana CLI commands or client-side scripts to ensure it behaves as expected.

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
