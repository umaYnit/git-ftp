use std::{fmt, result};
use std::error::Error;
use std::fmt::Formatter;
use std::path::PathBuf;

use git2::Delta;

pub struct DealFile {
    changed: Vec<PathBuf>,
    deleted: Vec<PathBuf>,
    others: Vec<PathBuf>,
}

impl DealFile {
    pub fn new() -> Self {
        DealFile {
            changed: vec![],
            deleted: vec![],
            others: vec![],
        }
    }

    pub fn changed(&mut self) -> &mut Vec<PathBuf> {
        &mut self.changed
    }
    pub fn deleted(&mut self) -> &mut Vec<PathBuf> {
        &mut self.deleted
    }
    pub fn others(&mut self) -> &mut Vec<PathBuf> {
        &mut self.others
    }
}

fn fmt_write(f: &mut Formatter<'_>, list_name: &str, list: &Vec<PathBuf>) -> result::Result<(), fmt::Error> {
    f.write_str(list_name)?;
    f.write_str(" : [")?;

    if list.len() != 0 {
        f.write_str("\n")?;
        for file in list {
            f.write_str("\t")?;
            f.write_str(file.to_str().unwrap())?;
            f.write_str(",\n")?;
        }
    }
    f.write_str("]\n")
}

impl fmt::Display for DealFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt_write(f, "CHANGED", &self.changed)?;
        fmt_write(f, "DELETED", &self.deleted)?;
        fmt_write(f, "OTHERS", &self.others)
    }
}


pub fn recipe_modified<'a>(path: PathBuf) -> Result<DealFile, Box<dyn Error>> {
    println!("{:?}", path);
    let repo = git2::Repository::discover(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commit_0 = repo.find_commit(revwalk.nth(0).unwrap()?)?.tree()?;

    let commit_1 = repo.find_commit(revwalk.nth(0).unwrap()?)?.tree()?;

    let diff = repo.diff_tree_to_tree(Some(&commit_1), Some(&commit_0), None)?;
    let deltas = diff.deltas();
    let mut deal_files = DealFile::new();

    deltas.for_each(|x| {
        let vec = match x.status() {
            Delta::Added | Delta::Modified => deal_files.changed(),
            Delta::Deleted => deal_files.deleted(),
            _ => deal_files.others()
        };
        vec.push(x.new_file().path().expect("path error").to_owned())
    });

    Ok(deal_files)
}
