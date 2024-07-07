/*
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use clap::Parser;
use prollytree::config::TreeConfig;
use prollytree::storage::InMemoryNodeStorage;
use prollytree::tree::{ProllyTree, Tree};
use std::fs;

/// CLI for managing Prolly Trees
#[derive(Parser)]
#[clap(
    name = "ProllyTree CLI",
    version = "1.0",
    author = "Your Name",
    about = "A CLI for managing Prolly Trees"
)]
enum Cli {
    Create {
        #[clap(short, long, default_value = "config.json")]
        config: String,
    },
    Insert {
        #[clap(short, long)]
        key: String,
        #[clap(short, long)]
        value: String,
    },
    Delete {
        #[clap(short, long)]
        key: String,
    },
    Proof {
        #[clap(short, long)]
        key: String,
    },
    Diff {
        #[clap(short = 'a', long)]
        tree1: String,
        #[clap(short = 'b', long)]
        tree2: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args {
        Cli::Create { config } => {
            let config_content = fs::read_to_string(config).expect("Unable to read config file");
            let config: TreeConfig<32> =
                serde_json::from_str(&config_content).expect("Invalid config format");
            let storage = InMemoryNodeStorage::<32>::new();
            let tree = ProllyTree::new(storage, config);
            println!("{:?}", tree.summary());
            println!("Tree created successfully.");
        }
        Cli::Insert { key, value } => {
            let storage = InMemoryNodeStorage::<32>::new();
            let config = TreeConfig::default();
            let mut tree = ProllyTree::new(storage, config);
            tree.insert(key.into_bytes(), value.into_bytes());
            println!("Key-Value pair inserted successfully.");
        }
        Cli::Delete { key } => {
            let storage = InMemoryNodeStorage::<32>::new();
            let config = TreeConfig::default();
            let mut tree = ProllyTree::new(storage, config);
            if tree.delete(key.as_bytes()) {
                println!("Key deleted successfully.");
            } else {
                println!("Key not found.");
            }
        }
        Cli::Proof { .. } | Cli::Diff { .. } => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo() {
        // 1. Create a custom tree config
        let config = TreeConfig {
            base: 131,
            modulus: 1_000_000_009,
            min_chunk_size: 4,
            max_chunk_size: 8 * 1024,
            pattern: 0b101,
            root_hash: None,
        };

        // 2. Create and Wrap the Storage Backend
        let storage = InMemoryNodeStorage::<32>::new();

        // 3. Create the Prolly Tree
        let mut tree = ProllyTree::new(storage, config);

        // 4. Insert New Key-Value Pairs
        tree.insert(b"key1".to_vec(), b"value1".to_vec());
        tree.insert(b"key2".to_vec(), b"value2".to_vec());

        // 5. Traverse the Tree with a Custom Formatter
        let traversal = tree.formatted_traverse(|node| {
            let keys_as_strings: Vec<String> =
                node.keys.iter().map(|k| format!("{:?}", k)).collect();
            format!("[L{}: {}]", node.level, keys_as_strings.join(", "))
        });
        println!("Traversal: {}", traversal);

        // 6. Update the Value for an Existing Key
        tree.update(b"key1".to_vec(), b"new_value1".to_vec());

        // 7. Find or Search for a Key
        if let Some(node) = tree.find(b"key1") {
            println!("Found key1 with value: {:?}", node);
        } else {
            println!("key1 not found");
        }

        // 8. Delete a Key-Value Pair
        if tree.delete(b"key2") {
            println!("key2 deleted");
        } else {
            println!("key2 not found");
        }

        // 9. Print tree stats
        println!("Size: {}", tree.size());
        println!("Depth: {}", tree.depth());
        println!("Summary: {}", tree.summary());
    }
}
