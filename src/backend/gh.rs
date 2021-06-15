use std::collections::HashMap;
use std::process::Command;
use json::{self, JsonValue};
use std::fs;

#[derive(Debug)]
pub struct GhError {
    pub message: String
}

impl GhError {
    fn new(message: &str) -> Self {
        GhError {message: message.to_string() }
    }
}

pub struct GhClient {
    repo_owner: String,
    repo_name: String,
    queries_map: HashMap<String, String>,
}

impl GhClient {
    pub fn new(repo_owner: String, repo_name: String) -> Result<Self, std::io::Error> {
        let queries_map = GhClient::read_queries()?;
        Ok(GhClient {repo_owner, repo_name, queries_map})
    }

    pub fn validate(&self, pr_num: Option<u32>) -> Result<(), GhError> {
        let repo = &format!("{}/{}", self.repo_owner, self.repo_name);
        let mut cmd = Command::new("gh");
        let error_msg : &str;
        if let Some(number) = pr_num {
            cmd.args(&["pr", "view"]);
            cmd.args(&["-R", repo]);
            cmd.arg(&number.to_string());
            error_msg = "Repository not found or pull request number is invalid";
        } else {
            cmd.args(&["repo", "view", repo]);
            error_msg = "Repository not found";
        }
        let output = cmd.output()
            .map_err(|e| GhError {message: e.to_string()})?;

        if output.stderr.len().eq(&0) {
            Ok(())
        } else {
            Err(GhError {message: String::from(error_msg)})
        }
    }

    pub fn pr_list(&self) -> Result<GqlRequest, GhError> {
        let query = self.get_query("pr_list")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .set_query(query)
            .build();
        Ok(request)
    }
    
    pub fn pr_conversation(&self, number: u32) -> Result<GqlRequest, GhError> {
        let query = self.get_query("pr_conversation")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .add_int_param("number", number)
            .set_query(query)
            .build();
        Ok(request)
    }

    fn get_query(&self, name: &str) -> Result<String, GhError> {
        self.queries_map
            .get(name)
            .map(|s| s.to_string())
            .ok_or(GhError::new(&format!("Query template {} wasn't found", name)))
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
    pub fn execute(&mut self) -> Result<JsonValue, GhError> {
        let output = self.cmd.output().map_err(|e| GhError::new(&e.to_string()))?;
        crate::logs::log(&format!("{:?}", output));
        if !output.status.success() {
            return Err(GhError::new(&String::from_utf8(output.stderr).unwrap()));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        json::parse(&output).map_err(|e| {
            GhError::new(&format!("Got malformed json: {}", e.to_string()))
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
