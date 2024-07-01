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
