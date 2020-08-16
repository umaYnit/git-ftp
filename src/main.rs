use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use git2::Delta;
use ssh2::{Session, Sftp};
use structopt::StructOpt;

use crate::config::edit_configuration;
use crate::file::*;

mod config;

mod file;


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

#[derive(Debug, Eq, PartialEq, Hash)]
enum GitPushFile {
    CHANGED,
    DELETED,
    OTHERS,
}


fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Push => {
            let map = recipe_modified(PathBuf::from_str("C:\\Users\\Ynit\\CLionProjects\\git_push").unwrap())?;
            println!("{:?}", map);
        }
        Opt::Show => {
            let map = recipe_modified(PathBuf::from_str("C:\\Users\\Ynit\\CLionProjects\\git_push").unwrap())?;
            println!("{:?}", map);
        }
        Opt::Config => {}
    }

    edit_configuration();


// deal_files()?;
    Ok(())
}

fn recipe_modified<'a>(path: PathBuf) -> Result<HashMap<GitPushFile, Vec<PathBuf>>, Box<dyn Error>> {
    let repo = git2::Repository::discover(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commit_0 = repo.find_commit(revwalk.nth(0).unwrap()?)?.tree()?;

    let commit_1 = repo.find_commit(revwalk.nth(0).unwrap()?)?.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&commit_1), Some(&commit_0), None)?;
    let deltas = diff.deltas();
    let mut map = HashMap::new();

    deltas.for_each(|x| {
        let key = match x.status() {
            Delta::Added | Delta::Modified => GitPushFile::CHANGED,
            Delta::Deleted => GitPushFile::DELETED,
            _ => GitPushFile::OTHERS
        };
        let value = map.entry(key).or_insert(Vec::new());
        value.push(x.new_file().path().expect("path error").to_owned());
    });

    Ok(map)
}

fn deal_files(file: &Path) -> Result<(), Box<dyn Error>> {
    let tcp = TcpStream::connect("xxx.xxx.xxx.xxx:22")?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_password("xxxx", "xxxxxx")?;
    assert!(sess.authenticated());

    let sftp = sess.sftp()?;

    push_file(&sftp, file)?;

    Ok(())
}

fn push_file(sftp: &Sftp, file_name: &Path) -> Result<(), Box<dyn Error>> {
    let mut root = PathBuf::from("/root/ftp");
    root.push(file_name);
    RemoteFileEntry::create_dir_all(sftp, root.parent().unwrap())?;
    let mut file = sftp.create(&root)?;

    io::copy(&mut File::open(file_name)?, &mut file)?;
    Ok(())
}