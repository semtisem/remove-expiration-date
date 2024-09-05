use clap::Parser;

use super::config::models::{ConfigAuthCommand, ConfigCryptoCommand};

// represents password flow
#[derive(Clone)]
pub struct PasswordAuth(pub String, pub String);

#[derive(Parser)]
#[clap(rename_all = "kebab-case", about = "Syncoon (dccmd-rs)")]
pub struct Syncoon {
    #[clap(subcommand)]
    pub cmd: SyncoonCommand,

    #[clap(long, global = true)]
    pub debug: bool,

    #[clap(long, global = true)]
    pub log_file_out: bool,

    #[clap(long, global = true)]
    pub log_file_path: Option<String>,

    /// optional username
    #[clap(long, global = true)]
    pub username: Option<String>,

    /// optional password
    #[clap(long, global = true)]
    pub password: Option<String>,

    /// optional encryption password
    #[clap(long, global = true)]
    pub encryption_password: Option<String>,
}

#[derive(Parser)]
pub enum SyncoonCommand {
    Run {
        // Source DRACOON instance
        dracoon: String,

        // Target DRACOON instance
        data_room_id: u64,
    },
    /// Configure syncoon-cli
    Config {
        #[clap(subcommand)]
        cmd: ConfigCommand,
    },

    /// Print current syncoon-cli version
    Version,
}

#[derive(Parser)]
pub enum ConfigCommand {
    /// Manage Syncoon auth credentials (refresh token)
    Auth {
        #[clap(subcommand)]
        cmd: ConfigAuthCommand,
    },

    /// Manage Syncoon encryption credentials (encryption secret)
    Crypto {
        #[clap(subcommand)]
        cmd: ConfigCryptoCommand,
    },
}
