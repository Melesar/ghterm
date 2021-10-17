use std::collections::HashMap;
use std::process::Command;
use super::gh::GhClient;
use crate::error::Error;
use regex::{Captures, Regex};

pub struct DiffRequest {
    cmd: Command,
}

#[derive(Debug)]
pub enum DiffSide { Left, Right }

#[derive(Debug)]
pub struct CodeRange {
    pub file_name: String,
    pub side: DiffSide,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug)]
pub struct Range(usize, usize);

pub struct FileDiffRef {
    pub start_line: usize,
    pub hunks: Vec<DiffHunkRef>,
}

#[derive(Debug)]
pub struct DiffHunkRef {
    pub changelist_range: Range,
    pub range_before: Range,
    pub range_after: Range,
}

pub struct ChangeList {
    raw: String,
    files: HashMap<String, FileDiffRef>,
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
        crate::logs::log(&raw_diff);
        Ok(raw_diff)
    }
}

impl CodeRange {
    pub fn new(file_name: String, side: DiffSide, start_line: usize, end_line: usize) -> Self {
        CodeRange { file_name, side, start_line, end_line }
    }
}

struct DiffReadingState {
    pub line_index: usize,
    pub current_file: String,
}

impl DiffReadingState {
    pub fn new() -> Self {
        DiffReadingState { line_index: 0, current_file: String::new() }
    }
}

impl ChangeList {
    pub fn new(diff: String) -> Self {
        let files = HashMap::new();
        let mut reading_state = DiffReadingState::new();
        let file_name_regex = Regex::new(r"^diff --git a/(.+) b/(.+)$").unwrap();
        let hunk_regex = Regex::new(r"^@@ -(\d,\d) +(\d+,\d+) @@$").unwrap();
        
        for line in diff.lines() {
            if let Some(captures) = file_name_regex.captures(line) {
                Self::read_file_name(captures, &mut reading_state);
            }
            else if let Some(captures) = hunk_regex.captures(line) {

            }

            reading_state.line_index += 1;
        }

        ChangeList { raw: diff, files }
    }

    pub fn get_hunk<'a>(&'a self, code_range: &CodeRange) -> &'a str {
        let file_diff_range = self.files.get(&code_range.file_name);
        if file_diff_range.is_none() {
            return "";
        }

        let file_diff_range = file_diff_range.unwrap();
        let mut line_number = file_diff_range.start_line;

        for hunk in file_diff_range.hunks.iter() {
            let hunk_range = match code_range.side {
                DiffSide::Left => &hunk.range_before,
                DiffSide::Right => &hunk.range_after,
            };

        }


        ""
    }

    fn read_file_name<'a>(captures: Captures<'a>, reading_state: &mut DiffReadingState) {
        reading_state.current_file = captures[1].to_string();
    }
}
