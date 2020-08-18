#[macro_use]
extern crate serde_derive;

// TODO 自定义Error
use std::error::Error;

use structopt::StructOpt;

use crate::config::{edit_configuration, load_config};
use crate::error::CustomError;
use crate::ftp_file::deal_git_files;
use crate::git_file::recipe_modified;

mod error;

mod git_file;

mod config;

mod ftp_file;


/// a tiny tool for maintain the static file server.
/// find the change in last commit, and push the change to target server.
#[derive(StructOpt, Debug)]
#[structopt(name = "git_push")]
enum Opt {
    /// push the change file in last commit to target server
    Push {
        /// git repository root dir in local
        #[structopt(short, long)]
        source: String,

        /// target root dir in server
        #[structopt(short, long)]
        target: String,
        //
        // #[structopt(flatten, conflicts_with = "remote_name")]
        // user_pass: UserPass,

        // ///use this when the server not defined in the config
        // #[structopt(short, long, conflicts_with = "remote_name")]
        // username: Option<String>,
        //
        // ///must be used with username
        // #[structopt(short, long, conflicts_with = "remote_name")]
        // password: Option<String>,

        ///server name(define in the config file first)
        #[structopt(short, long)]
        remote: String,

    },
    /// only show the change in last commit
    Show {
        /// git repository root dir in local
        #[structopt(short, long)]
        source: String,
    },
    /// edit the config
    Config {},
}

// #[derive(StructOpt, Debug)]
// struct UserPass {
//     ///use this when the server not defined in the config
//     #[structopt(short, long)]
//     username: String,
//
//     ///must be used with username
//     #[structopt(short, long)]
//     password: String,
// }


fn main() {
    match run() {
        Err(e) => { println!("{:?}", e); }
        _ => {}
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Push { source, target, remote } => {
            let conf = load_config()?;
            let server = conf.get_server(&remote).ok_or(CustomError::NotFoundServerConfig(remote))?;

            deal_git_files(server, source, target)?;
            println!("push the changed file completed!");
        }
        Opt::Show { source } => {
            let deal_files = recipe_modified(&source.into())?;
            println!("{}", deal_files);
        }
        Opt::Config {} => {
            edit_configuration()?;
        }
    }

// deal_files()?;
    Ok(())
}

