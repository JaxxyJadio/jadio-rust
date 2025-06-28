mod cli;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jadio")]
#[command(about = "JADIO CLI Framework", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize JADIO in current directory
    Init,

    /// Install a package
    Install {
        /// Path to package to install
        path: String,
    },

    /// Uninstall a package
    Uninstall {
        /// Name of package to uninstall
        name: String,
    },

    /// List installed packages
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            cli::init::run();
        }
        Commands::Install { path } => {
            cli::install::run(&path);
        }
        Commands::Uninstall { name } => {
            cli::uninstall::run(&name);
        }
        Commands::List => {
            cli::list::run();
        }
    }
}
