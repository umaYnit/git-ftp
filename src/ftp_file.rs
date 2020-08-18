use std::error::Error;
use std::fs::File;
use std::io;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use ssh2::{Session, Sftp};

use crate::config::ServerConfig;
use crate::git_file::recipe_modified;

const TRASH_DIR: &str = "/home/.trash";

#[derive(Clone, PartialEq, Eq, Ord)]
pub enum FileEntry {
    File(PathBuf, u64),
    Directory(PathBuf),
}

impl FileEntry {
    fn path(&self) -> &Path {
        match self {
            FileEntry::File(path, _) => path,
            FileEntry::Directory(path) => path,
        }
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &FileEntry) -> Option<std::cmp::Ordering> {
        self.path().partial_cmp(other.path())
    }
}


pub struct LocalFileEntry(FileEntry);

pub struct RemoteFileEntry(FileEntry);

impl RemoteFileEntry {
    pub fn exists(path: impl AsRef<Path>, sftp: &Sftp) -> Result<bool, Box<dyn Error>> {
        match sftp.stat(path.as_ref()) {
            // NOTE: `stat` will fail if this path does not exist on the remote host. We
            //        assume this is the case when `stat` returns `LIBSSH2_ERROR_SFTP_PROTOCOL`.
            Err(error) => match error.code() {
                libssh2_sys::LIBSSH2_ERROR_SFTP_PROTOCOL => Ok(false),
                _ => Err(error.into()),
            },
            Ok(_) => Ok(true),
        }
    }

    pub fn create_dir_all(sftp: &Sftp, path: &Path) -> Result<(), Box<dyn Error>> {
        if path == Path::new("") {
            return Ok(());
        };
        if RemoteFileEntry::exists(path, sftp)? {
            return Ok(());
        } else {
            match path.parent() {
                Some(p) => RemoteFileEntry::create_dir_all(sftp, p)?,
                None => {}
            }
        };
        sftp.mkdir(path, 0o644)?;

        Ok(())
    }
}

pub fn deal_git_files(server_config: &ServerConfig, source: String, target: String) -> Result<(), Box<dyn Error>> {
    let source = source.into();
    let deal_files = recipe_modified(&source)?;

    let port = server_config.port.unwrap_or(22);
    let tcp = TcpStream::connect(format!("{}:{}", server_config.ip, port))?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    sess.userauth_password(&server_config.username, &server_config.password)?;
    assert!(sess.authenticated());

    let sftp = sess.sftp()?;

    let target = &PathBuf::from(&target);

    for x in deal_files.changed() {
        push_file(&sftp, x, &source, &PathBuf::from(target))?;
    }

    RemoteFileEntry::create_dir_all(&sftp, &PathBuf::from(TRASH_DIR))?;

    for x in deal_files.deleted() {
        delete_file(&sftp, x, &PathBuf::from(target))?;
    }

    for x in deal_files.others() {
        println!("unknown file: {:?}", x);
    }

    Ok(())
}

// fn deal_file(file: &Path) -> Result<(), Box<dyn Error>> {
//     let tcp = TcpStream::connect("xxx.xxx.xxx.xxx:22")?;
//     let mut sess = Session::new()?;
//     sess.set_tcp_stream(tcp);
//     sess.handshake()?;
//
//     sess.userauth_password("xxxx", "xxxxxx")?;
//     assert!(sess.authenticated());
//
//     let sftp = sess.sftp()?;
//
//     push_file(&sftp, file)?;
//
//     Ok(())
// }


fn push_file(sftp: &Sftp, file_name: &Path, source: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut root = target.to_path_buf();
    root.push(file_name);

    let mut src = source.to_path_buf();
    src.push(file_name);

    RemoteFileEntry::create_dir_all(sftp, root.parent().unwrap())?;
    let mut file = sftp.create(&root)?;
    io::copy(&mut File::open(&src)?, &mut file)?;
    Ok(())
}

fn delete_file(sftp: &Sftp, file_name: &Path, target: &Path) -> Result<(), Box<dyn Error>> {
    let mut root = target.to_path_buf();
    root.push(file_name);

    let mut trash_path = PathBuf::from(TRASH_DIR);
    trash_path.push(file_name);

    match sftp.rename(&root, &trash_path, None) {
        // NOTE: `stat` will fail if this path does not exist on the remote host. We
        //        assume this is the case when `stat` returns `LIBSSH2_ERROR_SFTP_PROTOCOL`.
        Err(error) => match error.code() {
            libssh2_sys::LIBSSH2_ERROR_SFTP_PROTOCOL => {
                println!("file [{:?}] not exist in remote server", file_name);
                Ok(())
            }
            _ => Err(error.into()),
        },
        Ok(_) => Ok(()),
    }
}