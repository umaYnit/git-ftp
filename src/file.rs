use std::error::Error;
use std::path::{Path, PathBuf};

use ssh2::Sftp;

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
