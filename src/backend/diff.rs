use std::process::Command;
use super::gh::GhClient;
use crate::error::Error;

pub struct DiffRequest {
    cmd: Command,
}

#[derive(Debug)]
pub struct DiffHunk {
    file_name: String,
    start_line: usize,
    end_line: usize,
}

pub struct ChangeList {
    raw: String,
    files: Vec<String>,
    diffs: Vec<String>,
}

impl DiffRequest {
    pub fn new(cmd: Command) -> Self {
        DiffRequest { cmd }
    }

    pub fn execute(&mut self) -> Result<String, Error> {
        let output = self.cmd.output().map_err(|e| Error::Other(e.to_string()))?;
        if !output.status.success() {
            return Err(Error::Other(String::from_utf8(output.stderr).unwrap()));
        }

        let raw_diff = String::from_utf8(output.stdout).unwrap();
        Ok(raw_diff)
    }
}

impl ChangeList {
    pub fn new(diff: String) -> Self {
        ChangeList { raw: diff, files: vec![], diffs: vec![] }
    }

    pub fn get_hunk<'a>(&'a self, hunk: &DiffHunk) -> &'a str {
        ""
    }
}

