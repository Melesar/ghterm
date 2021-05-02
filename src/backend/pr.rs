use super::gh;
use std::sync::mpsc;
use std::io::Error;
use std::thread;

#[derive(Debug)]
pub struct PrHeader {
    pub number: u32,
    pub title: String,
}

pub struct Pr {
    pub info: Option<PrInfo>,
} 

#[derive(Debug)]
pub struct PrInfo {
    pub number: u32,
    pub title: String,
    pub base_branch: String,
    pub head_branch: String,
    pub body: String,
}

pub fn list_prs () -> Result<Vec<PrHeader>, Error> {
    let output = gh::pr_list()?;
    let output = output["data"]["repository"]["pullRequests"]["edges"].members();
    let mut prs = Vec::new();
    for pr in output {
        let number = pr["node"]["number"].as_u32().unwrap();
        let title = pr["node"]["title"].as_str().unwrap().to_string();
        prs.push(PrHeader{number, title});
    }
    
    Ok(prs)
}

pub fn fetch_pr(number: u32) -> Result<Pr, Error> { 
    let output = gh::pr_view(number)?;
    let output = &output["data"]["repository"]["pullRequest"];
    let pr_info = PrInfo {
        number: output["number"].as_u32().unwrap(),
        title: output["title"].as_str().unwrap().to_string(),
        base_branch: output["baseRefName"].as_str().unwrap().to_string(),
        head_branch: output["headRefName"].as_str().unwrap().to_string(),
        body: output["body"].as_str().unwrap().to_string(),
    };
    Ok (Pr{info: Some(pr_info)})
}
