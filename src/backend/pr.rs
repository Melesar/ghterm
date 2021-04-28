use super::gh;
use std::io::Error;
use std::str;

#[derive(Debug)]
pub struct PR {
    pub id: u32,
    pub title: String,
}

pub struct PR_Info {

}

impl PR {
    pub fn new(id: u32, title: String) -> PR {
        PR {id, title}
    }

    pub fn list () -> Result<Vec<PR>, Error> {
        let output = gh::pr_list()?;
        let output = output["data"]["repository"]["pullRequests"]["edges"].members();
        let mut prs = Vec::new();
        for pr in output {
            //let pr = pr["node"];
            let number = pr["node"]["number"].as_u32().unwrap();
            let title = pr["node"]["title"].as_str().unwrap().to_string();
            prs.push(PR{id: number, title});
        }
        
        Ok(prs)
    }

    pub fn view_pr(id: u32) -> PR_Info {
        PR_Info {}
    }

    pub fn view(&self) -> PR_Info {
        PR_Info {}
    }
}
