use std::collections::HashMap;
use std::process::{Command, Stdio};
use json::{self, JsonValue};
use std::fs;
use crate::error::Error;
use super::diff::DiffRequest;

pub struct GhClient {
    repo_owner: String,
    repo_name: String,
    queries_map: HashMap<String, String>,
}

impl GhClient {
    pub fn new(repo_owner: String, repo_name: String) -> Result<Self, Error> {
        let queries_map = GhClient::read_queries()
            .map_err(|_| Error::Other("Failed to read queries files. Make sure you have installed ghterm correctly".to_string()))?;

        Ok(GhClient {repo_owner, repo_name, queries_map})
    }

    pub fn validate(&self, pr_num: Option<u32>) -> Result<(), Error> {
        let repo = &format!("{}/{}", self.repo_owner, self.repo_name);
        let mut cmd = Command::new("gh");
        let error : Error;
        if let Some(number) = pr_num {
            cmd.args(&["pr", "view"]);
            cmd.args(&["-R", repo]);
            cmd.arg(&number.to_string());
            error = Error::PrDoesntExist(repo.to_string(), number);
        } else {
            cmd.args(&["repo", "view", repo]);
            error = Error::NotARepo(repo.to_string());
        }
        let output = cmd.output()
            .map_err(|e| Error::Other(e.to_string()))?;

        if output.stderr.len().eq(&0) {
            Ok(())
        } else {
            Err(error)
        }
    }

    pub fn pr_list(&self) -> Result<GqlRequest, Error> {
        let query = self.get_query("pr_list")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .set_query(query)
            .build();
        Ok(request)
    }
    
    pub fn pr_conversation(&self, number: u32) -> Result<GqlRequest, Error> {
        let query = self.get_query("pr_conversation")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .add_int_param("number", number)
            .set_query(query)
            .build();
        Ok(request)
    }

    pub fn pr_diff(&self, number: u32) -> DiffRequest {
        let mut cmd = Command::new("gh");
        cmd.args(&["pr", "diff"]);
        cmd.arg(&number.to_string());
        cmd.arg(&format!("-R {}/{}", self.repo_owner, self.repo_name));

        DiffRequest::new(cmd)
    }

    fn get_query(&self, name: &str) -> Result<String, Error> {
        self.queries_map
            .get(name)
            .map(|s| s.to_string())
            .ok_or(Error::Other(format!("Query template {} wasn't found", name)))
    }

    fn read_queries() -> Result<HashMap<String, String>, std::io::Error> {
        let map : HashMap<String, String> = fs::read_dir(GhClient::get_requests_directory())?
            .filter_map(|e| e.ok())
            .filter_map(|e| match e.file_type() { 
                Ok(ft) => if ft.is_file() && e.file_name().to_str().unwrap().ends_with(".gql") {
                    Some(e.path())
                } else {
                    None
                },
                Err(_) => None
            })
            .filter_map(|p| p.file_stem()
                        .map(|os| os.to_str().map(|s| s.to_string()).unwrap())
                        .zip(fs::read_to_string(p).ok()))
            .collect();
        Ok(map)
    }


    #[cfg(debug_assertions)]
    fn get_requests_directory() -> String {
        "data/requests".to_string()
    }

    #[cfg(not(debug_assertions))]
    fn get_requests_directory() -> String {
        //TODO use XDG_DATA_HOME
        String::new()
    }

}

pub struct GqlRequest {
    cmd: Command
}

impl GqlRequest {
    pub fn execute(&mut self) -> Result<JsonValue, Error> {
        let output = self.cmd.output().map_err(|e| Error::Other(e.to_string()))?;
        crate::logs::log(&format!("{:?}", output));
        if !output.status.success() {
            return Err(Error::Other(String::from_utf8(output.stderr).unwrap()));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        json::parse(&output).map_err(|e| {
            Error::Other(format!("Got malformed json: {}", e.to_string()))
        })
    }
}

unsafe impl Send for GqlRequest {}

struct GqlQueryBuilder {
    repo_owner: String,
    repo_name: String,
    query: String,
    string_params: HashMap<String, String>,
    int_params: HashMap<String, u32>,
}

impl GqlQueryBuilder {
    fn new() -> Self {
        GqlQueryBuilder {
            repo_owner: String::from(":owner"),
            repo_name: String::from(":repo"),
            query: String::new(),
            string_params: HashMap::new(),
            int_params: HashMap::new() 
        }
    }

    fn set_repo(&mut self, owner: String, name: String) -> &mut Self {
        self.repo_owner = owner;
        self.repo_name = name;
        self
    }

    fn add_string_param(&mut self, param_name: &str, param_value: &str) -> &mut Self {
        self.string_params.insert(String::from(param_name), String::from(param_value));
        self
    }

    fn add_int_param(&mut self, param_name: &str, param_value: u32) -> &mut Self {
        self.int_params.insert(String::from(param_name), param_value);
        self
    }

    fn set_query(&mut self, query: String) -> &mut Self {
        self.query = query;
        self
    }

    fn build(&mut self) -> GqlRequest {
        let mut cmd = Command::new("gh");
        cmd.args(&["api", "graphql"]);
        cmd.args(&["-F", &format!("owner={}", self.repo_owner)]);
        cmd.args(&["-F", &format!("name={}", self.repo_name)]);

        let mut query_header = String::from("query=query($name: String!, $owner: String!");
        for (param_name, param_value) in self.string_params.iter() {
            query_header.push_str(&format!(", ${}: String!", param_name));
            cmd.args(&["-F", &format!("{}={}", param_name, param_value)]);
        }
        for (param_name, param_value) in self.int_params.iter() {
            query_header.push_str(&format!(", ${}: Int!", param_name));
            cmd.args(&["-F", &format!("{}={}", param_name, param_value)]);
        }
        query_header.push_str(") {\nrepository(owner: $owner, name: $name) {\n");
        query_header.push_str(&self.query);
        query_header.push_str("}}");
        crate::logs::log(&format!("{}", query_header));
        cmd.args(&["-f", &query_header]);
        GqlRequest {cmd}
    }
}

pub fn check_health() -> Result<bool, Error> {
   let result = check_gh_installed()? && ensure_authentication()?; 
   Ok(result)
}

fn check_gh_installed() -> Result<bool, Error> {
    Command::new("gh")
        .stdout(Stdio::null())
        .status()
        .map(|exit_code| exit_code.success())
        .map_err(|_| Error::GhNotInstalled)
}

fn ensure_authentication() -> Result<bool, Error> {
    let base_dirs = xdg::BaseDirectories::with_prefix("gh")
        .map_err(|_| Error::Other("Didn't find gh config directory".to_string()))?;

    let config_file = base_dirs.find_config_file("hosts.yml");
    match config_file {
        Some(path) => {
            let contents = fs::read_to_string(path).unwrap();
            if contents.contains("github.com") { 
                Ok(true)
            } else {
                authenticate().map_err(|e| Error::Other(e.to_string()))
            }
        },
        None => authenticate().map_err(|e| Error::Other(e.to_string()))
    }
}

fn authenticate() -> Result<bool, std::io::Error> {
    Command::new("gh")
        .args(&["auth", "login"])
        .status()
        .map(|exit_code| exit_code.success())
}
