use regex::Regex;
use reqwest::blocking::Client;
use scraper::Html;
use scraper::Selector;
use std::collections::BTreeSet;
use std::fmt;

use crate::display::JoinedByTrait;

#[derive(Debug, Clone)]
pub struct CodeforcesTest {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone)]
pub struct CodeforcesTests(pub Vec<CodeforcesTest>);

impl fmt::Display for CodeforcesTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}~\n{}", self.input, self.output)
    }
}

impl fmt::Display for CodeforcesTests {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().joined_by("\n\\\n"))
    }
}

pub fn get_problems(contest_no: u64) -> Result<Vec<String>, reqwest::Error> {
    let body = Client::new()
        .get(&format!("https://codeforces.com/contest/{}", contest_no))
        .send()?
        .text()?;
    let regex = Regex::new(&format!(
        "\"/contest/{}/problem/([A-Za-z0-9]+)\"",
        contest_no
    ))
    .unwrap(); // unwrap is acceptable, because result of tis cannot change
    Ok(regex
        .captures_iter(&body)
        .map(|x| x.get(1).unwrap().as_str()) // unwrap is acceptable because of static regex structure
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .map(|x| x.to_lowercase())
        .collect::<Vec<String>>())
}

pub fn get_tests(contest_no: u64, problem_name: &str) -> Result<CodeforcesTests, reqwest::Error> {
    let input_selector = Selector::parse("div.input pre").unwrap(); // unwrap is acceptable, because result of this cannot change
    let output_selector = Selector::parse("div.output pre").unwrap(); // unwrap is acceptable, because result of this cannot change
    let body = Client::new()
        .get(&format!(
            "https://codeforces.com/contest/{}/problem/{}",
            contest_no, problem_name
        ))
        .send()?
        .text()?;
    let document = Html::parse_document(&body);
    let input = document.select(&input_selector).map(|x| {
        html_escape::decode_html_entities(&x.inner_html().replace("<br>", "\n")).to_string()
    });
    let output = document.select(&output_selector).map(|x| {
        html_escape::decode_html_entities(&x.inner_html().replace("<br>", "\n")).to_string()
    });
    Ok(CodeforcesTests(
        input
            .zip(output)
            .map(|(input, output)| CodeforcesTest { input, output })
            .collect::<Vec<_>>(),
    ))
}
