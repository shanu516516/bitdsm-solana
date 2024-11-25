# bitdsm-solana

## Instructions to Initialize the Project in Solana for BitDSM

### Prerequisites

1. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Node.js** (using Homebrew for macOS):

   ```bash
   brew install node
   ```

3. **Install Yarn**:
   ```bash
   npm install -g yarn
   ```

### Install Anchor CLI

1. Install Anchor CLI using Cargo:

   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked
   avm install latest
   avm use latest
   ```

2. Verify the installation:
   ```bash
   anchor --version
   ```

### Install Solana

1. Install from Anza (recommended):

   ```bash
   sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
   ```

   or from Solana:

   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
   ```

2. Verify the installation:
   ```bash
   solana --version
   solana-test-validator --version
   ```

### Final Steps

1. **Check your Node version**:

   ```bash
   node --version
   ```

2. **Verify Yarn installation**:

   ```bash
   yarn --version
   ```

3. **Run the test validator in a new terminal**:

   ```bash
   solana-test-validator
   ```

4. **Run the tests**:

   ```bash
   anchor test
   ```

   or

   ```bash
   anchor test --skip-local-validator
   ```

### Creating a New Anchor Project

To create a new Anchor project:

```
anchor init bitdsm-solana
cd bitdsm-solana
```

### Summary of Changes:

- Grouped related instructions under clear headings (Prerequisites, Install Anchor CLI, Install Solana, Final Steps, Creating a New Anchor Project).
- Improved the flow of the document for better readability and understanding.
