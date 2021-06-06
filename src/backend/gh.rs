use std::collections::{HashMap, hash_map::Entry};
use std::io::Read;
use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use json::{self, JsonValue};
use std::fs;

pub struct GhClient {
    repo_owner: String,
    repo_name: String,
    queries_map: HashMap<String, String>,
}

impl GhClient {
    pub fn new(repo_owner: String, repo_name: String) -> Self {
        GhClient {repo_owner, repo_name, queries_map: HashMap::new()}
    }

    pub fn pr_list(&mut self) -> Result<GqlRequest> {
        let query = self.get_query("pr_list")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .set_query(query)
            .build();
        Ok(request)
    }
    
    pub fn pr_conversation(&mut self, number: u32) -> Result<GqlRequest> {
        let query = self.get_query("pr_conversation")?;
        let request = GqlQueryBuilder::new()
            .set_repo(self.repo_owner.clone(), self.repo_name.clone())
            .add_int_param("number", number)
            .set_query(query)
            .build();
        Ok(request)
    }

    fn get_query(&mut self, name: &str) -> Result<String> {
        match self.queries_map.entry(String::from(name)) {
            Entry::Occupied(q) => Ok(q.get().to_string()),
            Entry::Vacant(v) => {
                let file_name = GhClient::get_file_name(name);
                let mut file = fs::File::open(file_name)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                Ok(v.insert(content).to_string())
            }
        }
    }

    #[cfg(debug_assertions)]
    fn get_file_name(request_name: &str) -> String {
        format!("data/requests/{}.gql", request_name)
    }

    #[cfg(not(debug_assertions))]
    fn get_file_name(request_name: &str) -> String {
        //TODO use XDG_DATA_HOME
        String::new()
    }

}

pub struct GqlRequest {
    cmd: Command
}

impl GqlRequest {
    pub fn execute(&mut self) -> Result<JsonValue> {
        let output = self.cmd.output()?;
        crate::logs::log(&format!("{:?}", output));
        if !output.status.success() {
            return Err(Error::new(std::io::ErrorKind::Other, String::from_utf8(output.stderr).unwrap()));
        }

        let output = String::from_utf8(output.stdout).unwrap();
        json::parse(&output).map_err(|e| {
            std::io::Error::new(ErrorKind::Other, e.to_string())
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
