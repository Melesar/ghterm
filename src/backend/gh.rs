use std::collections::HashMap;
use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use json::{self, JsonValue};

pub fn pr_list() -> Result<JsonValue> {
    GqlQueryBuilder::new()
        .execute("pullRequests(first: 5 states: OPEN) {
                          edges {
                              node {
                                  number
                                  title
                              }
                          }
                      }")
}

pub fn pr_view(number: u32) -> Result<JsonValue> {
    GqlQueryBuilder::new()
        .add_int_param("number", number)
        .execute("pullRequest(number: $number) {
                            number
                            title
                            baseRefName
                            headRefName
                            body
                        }")
}

pub fn pr_conversation(number: u32) -> Result<JsonValue> {
    GqlQueryBuilder::new()
        .add_int_param("number", number)
        .execute("pullRequest(number: $number) {
                            reviews (last: 10) {
                                edges {
                                  node {
                                    id
                                    author {login}
                                    body
                                    createdAt
                                    comments (last: 10) {
                                      edges {
                                        node {
                                          id
                                          author {login}
                                          body
                                          createdAt
                                          replyTo { id }
                                    }}}}}}}")
}

struct GqlQueryBuilder {
    repo_owner: String,
    repo_name: String,
    string_params: HashMap<String, String>,
    int_params: HashMap<String, u32>,
}

impl GqlQueryBuilder {
    fn new() -> Self {
        GqlQueryBuilder {
            repo_owner: String::from(":owner"),
            repo_name: String::from(":repo"),
            string_params: HashMap::new(),
            int_params: HashMap::new() 
        }
    }

    fn set_repo(&mut self, owner: &str, name: &str) -> &mut Self {
        self.repo_owner = String::from(owner);
        self.repo_name = String::from(name);
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

    fn execute(&mut self, query: &str) -> Result<JsonValue> {
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
        query_header.push_str(query);
        query_header.push_str("}}");
        crate::logs::log(&format!("{}", query_header));
        cmd.args(&["-f", &query_header]);

        let output = cmd.output()?;
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
