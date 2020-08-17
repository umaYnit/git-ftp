use std::error::Error;
use std::fs::File;
use std::io;
use std::net::TcpStream;
use std::path::{Path, PathBuf};

use ssh2::{Session, Sftp};

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