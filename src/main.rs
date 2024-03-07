use clap::Parser;
use goji::{Credentials, Jira, SearchOptions, Worklog, Worklogs};
use log::*;

use crate::config::{Args, Config};
use chrono::Datelike;
use itertools::Itertools;
use rayon::prelude::*;

mod config;
mod innovation;

fn main() -> Result<(), anyhow::Error> {
    env_logger::init();
    info!("Starting up...");

    let args = Args::try_parse()?;
    let config = Config::read_from(&args.config)?;

    let credentials = Credentials::Basic(config.jira.email.to_string(), config.jira.api_token);
    let jira = Jira::new(config.jira.host, credentials)?;
    let work_logs = Worklogs::new(&jira);

    let current_year = chrono::Utc::now().year();

    let jql_issues_where_i_worked_on = if args.all_authors {
        format!(r#"
        worklogDate >= "{}/01/01" AND
        worklogDate <= "{}/12/31" AND
        "CategoryTypeLine[Dropdown]" = Training
        "#, current_year, current_year)
    } else {
        format!(r#"
        worklogAuthor = "{}" AND
        worklogDate >= "{}/01/01" AND
        worklogDate <= "{}/12/31" AND
        "CategoryTypeLine[Dropdown]" = Training
        "#, config.jira.email, current_year, current_year)
    };

    match jira.search().iter(jql_issues_where_i_worked_on, &Default::default()) {
        Ok(results) => {
            let training_issues = results
                .map(|i| (i.id.to_string(), i.key.to_string(), i.summary().unwrap()))
                .collect::<Vec<(String, String, String)>>();

            let worklogs = training_issues.par_iter()
                .flat_map(|(_, issue_key, _)| {
                    work_logs.list(issue_key, &SearchOptions::default()).ok()
                })
                .flat_map(|wl_results| wl_results.worklogs)
                .filter(|wl| wl.author.email_address.as_ref().is_some())
                .filter(|wl| wl.started.year() == current_year)
                .filter(|wl| args.all_authors || wl.author.email_address.as_ref().unwrap().eq_ignore_ascii_case(config.jira.email.as_str()))
                .collect::<Vec<Worklog>>();

            let worklog_map_by_user = worklogs.iter()
                .into_grouping_map_by(|wl| wl.author.email_address.clone().unwrap())
                .fold(0, |acc, _key, val| acc + val.time_spent_seconds);

            let worklog_map_by_issue = worklogs.iter()
                .into_grouping_map_by(|wl| wl.issue_id.clone())
                .fold(0, |acc, _key, val| acc + val.time_spent_seconds);

            println!("[+] got {} training issues", training_issues.iter().count());
            innovation::print_issues(training_issues.as_ref(), &worklog_map_by_issue);

            println!("[+] got {} authors working on innovation", worklog_map_by_user.iter().count());
            innovation::print_time_table(worklog_map_by_user);
        }
        Err(err) => panic!("{:#?}", err),
    }
    Ok(())
}