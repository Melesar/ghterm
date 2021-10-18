use std::collections::HashMap;
use std::process::Command;
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
    pub header_range: Range,
    pub hunks: Vec<HunkDiffRef>,
}

#[derive(Debug)]
pub struct HunkDiffRef {
    pub changelist_range: Range,
    pub range_before: Range,
    pub range_after: Range,
}

pub struct ChangeList {
    raw: String,
    files: HashMap<String, FileDiffRef>,
}

struct DiffReadingState {
    pub line_index: usize,
    pub current_file_name: String,
    pub current_file: FileDiffRef,
    pub current_hunk: Option<HunkDiffRef>,
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

impl FileDiffRef {
    pub fn new(start_line: usize) -> Self {
        FileDiffRef { start_line, header_range: Range(0, 0), hunks: vec![] }
    }
}

impl HunkDiffRef {
    pub fn new(ranges: (Range, Range), changelist_start: usize) -> Self {
        HunkDiffRef { changelist_range: Range(changelist_start, 0), range_before: ranges.0, range_after: ranges.1 }
    }
}

impl DiffReadingState {
    pub fn new() -> Self {
        DiffReadingState { line_index: 0, current_file_name: String::new(), current_file: FileDiffRef::new(0), current_hunk: None }
    }
}

impl ChangeList {
    pub fn new(diff: String) -> Self {
        let mut files = HashMap::new();
        let mut reading_state : Option<DiffReadingState> = None;
        let file_name_regex = Regex::new(r"^diff --git a/(.+) b/(.+)$").unwrap();
        let hunk_regex = Regex::new(r"^@@ -(\d,\d) +(\d+,\d+) @@").unwrap();
        
        for (line_index, line) in diff.lines().enumerate() {

            if let Some(captures) = file_name_regex.captures(line) {
                if let Some(mut reading_state) = reading_state {
                    if let Some(mut current_hunk) = reading_state.current_hunk.take() {
                        current_hunk.changelist_range.1 = line_index - 1;
                        reading_state.current_file.hunks.push(current_hunk);
                    }

                    files.insert(reading_state.current_file_name, reading_state.current_file);
                }

                let mut new_state = DiffReadingState::new();
                new_state.current_file_name = read_file_name(captures);
                new_state.current_file.start_line = line_index;
                new_state.current_file.header_range.0 = line_index;

                reading_state = Some(new_state);
            }
            else if let Some(captures) = hunk_regex.captures(line) {
                if let Some(reading_state) = reading_state.as_mut() {
                    let hunk_ranges = read_hunk_ranges(captures);
                    let new_hunk = HunkDiffRef::new(hunk_ranges, line_index);
                    let old_hunk = reading_state.current_hunk.replace(new_hunk);

                    if let Some(mut old_hunk) = old_hunk {
                        old_hunk.changelist_range.1 = line_index - 1;
                        reading_state.current_file.hunks.push(old_hunk);
                    }
                    else {
                        reading_state.current_file.header_range.1 = line_index - 1;
                    }
                }
            }
            else if line == r"\ No newline at end of file" {
                if let Some(reading_state) = reading_state.as_mut() {
                    if let Some(mut current_hunk) = reading_state.current_hunk.take() {
                        current_hunk.changelist_range.1 = line_index - 1;
                        reading_state.current_file.hunks.push(current_hunk);
                    }
                }
            }
        }

        ChangeList { raw: diff, files }
    }

    pub fn get_hunk<'a>(&'a self, code_range: &CodeRange) -> &'a str {
        let file_diff_range = self.files.get(&code_range.file_name);
        if file_diff_range.is_none() {
            return "";
        }

        let file_diff_range = file_diff_range.unwrap();
        for hunk in file_diff_range.hunks.iter() {
            let hunk_range = match code_range.side {
                DiffSide::Left => &hunk.range_before,
                DiffSide::Right => &hunk.range_after,
            };

            if code_range.start_line < hunk_range.0 || code_range.start_line > hunk_range.1 {
                continue;
            }

            let prefix_symbol = match code_range.side {
                DiffSide::Left => "-",
                DiffSide::Right => "+",
            };

            const LINES_PADDING : usize = 4;

            let mut start_line_idx = self.raw.lines()
                .skip(hunk.changelist_range.0 + 1)
                .take(hunk.changelist_range.1 - hunk.changelist_range.0 + 1)
                .enumerate()
                .find(|(idx, line)| (line.starts_with(" ") || line.starts_with(prefix_symbol)) && idx + hunk_range.0 == code_range.start_line)
                .unwrap().0;

            let end_line_idx = start_line_idx + (code_range.end_line - code_range.start_line);

            //TODO check hunk boudaries
            if end_line_idx - start_line_idx + 1 < LINES_PADDING {
                start_line_idx -= LINES_PADDING - end_line_idx + start_line_idx - 1;
            }

            let bytes_before = self.raw.lines().take(start_line_idx).fold(0_usize, |acc, val| acc + val.len());
            let bytes_after = self.raw.lines().take(end_line_idx + 1).fold(0_usize, |acc, val| acc + val.len());
            
            return &self.raw[bytes_before..bytes_after];
        }


        ""
    }

}

fn read_file_name<'a>(captures: Captures<'a>) -> String {
    captures[1].to_string()
}

fn read_hunk_ranges<'a>(captures: Captures<'a>) -> (Range, Range) {
    let range_before = &captures[1];
    let range_after = &captures[2];
    (read_hunk_range(range_before), read_hunk_range(range_after))
}

fn read_hunk_range(range_str: &str) -> Range {
    let mut iter = range_str.split(',');
    Range(iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
}
