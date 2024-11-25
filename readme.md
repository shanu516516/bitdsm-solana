# bitdsm-solana

## Instructions to initialize the project in Solana for BitDSM

1. First, make sure you have Rust installed. If not, install it:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Then install Anchor CLI using cargo:

```
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest
```

3. Verify the installation:

```
anchor --version
```

4. Check your node version:

```
node --version
```

# Install Node.js using homebrew (since you're on macOS)

brew install node

# Install Yarn

npm install -g yarn

# Verify yarn installation

yarn --version

Install from Anza (recommended):

```
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```

or from Solana:

```
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
```

Verify the installation:

```
solana --version
solana-test-validator --version
```

5. Run the test validator in a new terminal:

```
solana-test-validator
```

6. Run the tests:

```
anchor test
```

Note: To create a new Anchor project :

```
anchor init bitdsm-solana
cd bitdsm-solana
```
