
# ProllyTree-CLI

ProllyTree-CLI is a command-line interface tool for managing Prolly Trees, a type of data structure that combines the properties of Merkle trees and B-trees. This tool allows users to create, manipulate, and query Prolly Trees efficiently, providing a robust solution for data verification and synchronization.

## Features

- **Create and Initialize Trees**: Easily create and initialize new Prolly Trees with customizable configurations.
- **Insert and Delete Operations**: Perform insertions and deletions of key-value pairs in the tree.
- **Proof Generation**: Generate and verify proofs of existence for specific keys, ensuring data integrity.
- **Diff Functionality**: Compare two Prolly Trees and identify differences, facilitating efficient data synchronization.
- **Traverse and Visualize**: Traverse the tree and output its structure, aiding in debugging and visualization.
- **Configuration Management**: Load and save tree configurations, allowing for flexible and repeatable setups.

## Installation

To install ProllyTree-CLI, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/prollytree-cli.git
cd prollytree-cli
cargo build --release
```

## Usage

Here's a quick start guide on using ProllyTree-CLI:

### Create and Initialize a Tree

```bash
prollytree-cli create --config config.json
```

### Insert a Key-Value Pair

```bash
prollytree-cli insert --key 123 --value "example"
```

### Delete a Key-Value Pair

```bash
prollytree-cli delete --key 123
```

### Generate a Proof

```bash
prollytree-cli proof --key 123
```

### Compare Two Trees

```bash
prollytree-cli diff --tree1 tree1.json --tree2 tree2.json
```

## Contributing

Contributions are welcome! If you have any ideas, bug reports, or feature requests, please open an issue or submit a pull request.

## License

ProllyTree-CLI is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for more details.

## Contact

For more information, feel free to contact the project maintainer at [your-email@example.com].
