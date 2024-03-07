use std::collections::HashMap;
use itertools::Itertools;
use prettytable::{Attr, Cell, color, Row, Table};

pub fn print_time_table(work_logs: HashMap<String, u64>) -> () {
    let mut table = Table::new();
    table.set_titles(
        Row::new(vec![
            Cell::new("AUTHOR").with_style(Attr::Bold),
            Cell::new("SECONDS").with_style(Attr::Bold),
            Cell::new("WORK DAYS").with_style(Attr::Bold),
        ])
    );
    work_logs.iter().sorted().for_each(|(author, seconds) | {
        table.add_row(
            Row::new(vec![
                Cell::new(author.to_string().as_str())
                    .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                Cell::new(seconds.to_string().as_str()),
                Cell::new((seconds / (60*60*8)).to_string().as_str())
                    .with_style(Attr::Bold)
            ])
        );
    }
    );
    table.printstd();
}
pub fn print_issues(issues: &Vec<(String, String, String)>, worklog_by_issue: &HashMap<String, u64>) -> () {
    let mut table = Table::new();
    table.set_titles(
        Row::new(vec![
            Cell::new("KEY").with_style(Attr::Bold),
            Cell::new("SUMMARY").with_style(Attr::Bold),
            Cell::new("SECONDS").with_style(Attr::Bold),
            Cell::new("WORK DAYS").with_style(Attr::Bold)
        ])
    );
    issues.iter().sorted().for_each(|(id, key, summary) | {
        table.add_row(
            Row::new(vec![
                Cell::new(key)
                    .with_style(Attr::ForegroundColor(color::BRIGHT_CYAN)),
                Cell::new(summary),
                Cell::new(worklog_by_issue.get(id).unwrap().to_string().as_str()),
                Cell::new((worklog_by_issue.get(id).unwrap() / (60*60*8)).to_string().as_str())
                    .with_style(Attr::Bold)
            ])
        );
    }
    );
    table.printstd();
}