// TODO 自定义Error
use std::error::Error;

use structopt::StructOpt;

use crate::config::edit_configuration;
use crate::git_file::{ recipe_modified};

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

        ///server name(define in the config file first)
        #[structopt(short, long)]
        remote: String,

        ///server name(define in the config file first)
        #[structopt(short, long)]
        username: String,

        ///server name(define in the config file first)
        #[structopt(short, long)]
        password: String,
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


fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Push { source, target, remote, username, password } => {
            println!("{}", source);
            println!("{}", target);
            println!("{}", remote);
            println!("{}", username);
            println!("{}", password);

            let deal_files = recipe_modified(source.into())?;
            println!("{}", deal_files);
        }
        Opt::Show { source } => {
            let deal_files = recipe_modified(source.into())?;
            println!("{}", deal_files);
        }
        Opt::Config {} => {
            // TODO 子命令：显示配置和打开配置
            edit_configuration();
        }
    }


// deal_files()?;
    Ok(())
}

