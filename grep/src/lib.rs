use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use anyhow::Error;
use structopt::{clap, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(settings=&[clap::AppSettings::NoBinaryName], about = "a simple grep without bells and whistles for demo & learning purposes")]
pub struct Flags {
    #[structopt(short = "n")]
    print_line_numbers: bool,

    #[structopt(short = "l")]
    print_only_filenames: bool,

    #[structopt(short = "i")]
    case_insensitive: bool,

    #[structopt(short = "v")]
    invert_matching: bool,

    #[structopt(short = "x")]
    match_entire_lines: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags::from_iter(flags.iter())
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut results = Vec::new();
    let pattern = if flags.case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.into()
    };
    for filename in files {
        let file = BufReader::new(File::open(PathBuf::from_str(*filename)?)?);
        for (line_number, line) in file.lines().enumerate() {
            if let Some(line_output) = match_line(
                &pattern,
                line_number + 1,
                line?,
                flags,
                if files.len() > 1 {
                    Some(filename)
                } else {
                    None
                },
            ) {
                if flags.print_only_filenames {
                    results.push(filename.to_string());
                    break;
                } else {
                    results.push(line_output);
                }
            }
        }
    }
    Ok(results)
}

fn match_line(
    pattern: &str,
    line_number: usize,
    line: String,
    flags: &Flags,
    filename: Option<&str>,
) -> Option<String> {
    let line_matched = match (flags.case_insensitive, flags.match_entire_lines) {
        (true, true) => line.to_lowercase() == pattern,
        (true, false) => line.to_lowercase().contains(pattern),
        (false, true) => line == pattern,
        (false, false) => line.contains(pattern),
    };
    match (line_matched, flags.invert_matching) {
        (true, true) => None,
        (true, false) | (false, true) => Some(match_output(line_number, line, flags, filename)),
        (false, false) => None,
    }
}

fn match_output(line_number: usize, line: String, flags: &Flags, filename: Option<&str>) -> String {
    match flags.print_only_filenames {
        true => "".into(),
        false => match (filename, flags.print_line_numbers) {
            (Some(filename), true) => {
                filename.to_string() + ":" + line_number.to_string().as_str() + ":" + line.as_str()
            }
            (Some(filename), false) => filename.to_string() + ":" + line.as_str(),
            (None, true) => line_number.to_string() + ":" + line.as_str(),
            (None, false) => line,
        },
    }
}
