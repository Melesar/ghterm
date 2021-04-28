use std::{io::ErrorKind, process::Command};
use std::io::Error;
use json::{self, JsonValue};

pub fn pr_list() -> Result<JsonValue, Error> {
    gh_qraphql("query($name: String!, $owner: String!) {
                  repository(owner: $owner, name: $name) {
                      pullRequests(first: 5 states: OPEN) {
                          edges {
                              node {
                                  number
                                  title
                              }
                          }
                      }
                  }
              }")
}

fn gh_qraphql(graphql_query: &str) -> Result<JsonValue, Error> {
    let output = Command::new("gh")
        .args(&["api", "graphql"])
        .args(&["-F", "owner=:owner"])
        .args(&["-F", "name=:repo"])
        .args(&["-f", &format!("query={}", graphql_query)]) 
        .output()?; 

    if !output.status.success() {
        return Err(Error::new(std::io::ErrorKind::Other, String::from_utf8(output.stderr).unwrap()));
    }

    let output = String::from_utf8(output.stdout).unwrap();
    json::parse(&output).map_err(|e| {
        std::io::Error::new(ErrorKind::Other, e.to_string())
    })

}
