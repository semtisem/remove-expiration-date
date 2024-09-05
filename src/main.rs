use clap::Parser;
use cmd::{
    config::handle_config_cmd,
    models::{Syncoon, SyncoonCommand},
    print_version,
    remover::handle_remove_expiration,
};
use console::Term;

use cmd::{config::logs::init_logging, handle_error, models::PasswordAuth};

mod cmd;
mod env_vars;

#[tokio::main]
async fn main() {
    let opt = Syncoon::parse();

    let term = Term::stdout();
    let err_term = Term::stderr();

    init_logging(&err_term, opt.debug, opt.log_file_path);

    let password_auth = match (opt.username, opt.password) {
        (Some(username), Some(password)) => Some(PasswordAuth(username, password)),
        _ => None,
    };

    let res = match opt.cmd {
        SyncoonCommand::Run {
            dracoon,
            data_room_id,
        } => handle_remove_expiration(term, dracoon, data_room_id, password_auth).await,
        SyncoonCommand::Config { cmd } => handle_config_cmd(cmd, term).await,
        SyncoonCommand::Version => print_version(&term),
    };

    if let Err(e) = res {
        handle_error(&err_term, &e);
    }
}
