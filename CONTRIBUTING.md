# Contributing to Soroban Fixed-Point Math Library

Thank you for your interest in contributing to the Soroban Fixed-Point Math Library. We welcome contributions from the community and appreciate your help in making this library better.

## Getting Started

Before you begin:

1. **Create an issue** to validate demand, raise questions, or discuss your proposed changes at [issues](https://github.com/script3/soroban-fixed-point-math/issues)
2. **Fork this repository** and work on your changes in a feature branch
3. **Submit a pull request** for review by maintainers

If you're new to contributing on GitHub, refer to the [official GitHub documentation](https://docs.github.com/en/get-started/quickstart/contributing-to-projects).

## Typical Development Workflow

### 1. Clone the Repository

```bash
git clone https://github.com/script3/soroban-fixed-point-math.git
cd soroban-fixed-point-math
```

### 2. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 3. Make Your Changes

Work on your feature or bug fix, following the existing code style and patterns.

### 4. Test Your Changes

Before submitting, ensure your changes work correctly:

```bash
# Run tests
cargo test

# Build and optimize for targets

# wasm32v1-none
cargo build --target wasm32v1-none --release
stellar contract optimize --wasm target/wasm32v1-none/release/soroban_fixed_point_math.wasm

# wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/soroban_fixed_point_math.wasm
```

### 5. Commit and Push

```bash
git add .
git commit -m "Brief description of your changes"
git push origin feature/your-feature-name
```

### 6. Submit a Pull Request

Create a pull request from your feature branch to the main repository. Include:
- A clear description of what your changes do
- Reference to any related issues
- Screenshots or examples if applicable

## Pull Request Process

1. Ensure all tests pass and the code builds successfully
2. Update documentation/readme if necessary
3. Your pull request will be reviewed by maintainers
4. Once approved, your changes will be merged into the main branch

## Questions?

If you have questions about contributing, feel free to:
- Open an issue for discussion
- Ask questions in your pull request
- Refer to existing issues and pull requests for examples

Thank you for contributing to the Soroban Fixed-Point Math Library!