use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use json::{self, JsonValue};

pub fn pr_list() -> Result<JsonValue> {
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

pub fn pr_view(number: u32) -> Result<JsonValue> {
    gh_qraphql(&format!("query($name: String!, $owner: String!) {{
                    repository(owner: $owner name: $name) {{
                        pullRequest(number: {}) {{
                            number
                            title
                            baseRefName
                            headRefName
                            body
                        }}
                    }}
                }}", number))
}

pub fn pr_conversation(number: u32) -> Result<JsonValue> {
    gh_qraphql(&format!("query($name: String!, $owner: String!) {{
                    repository(owner: $owner name: $name) {{
                        pullRequest(number: {}) {{
                            reviews (last: 10) {{
                                edges {{
                                  node {{
                                    author {{login}}
                                    body
                                    comments (last: 10) {{
                                      edges {{
                                        node {{
                                          author {{login}}
                                          body
                                          createdAt
                                        }}
                                      }}
                                    }}
                                  }}
                                }}
                              }}
                        }}
                    }}
                }}", number))
}

fn gh_qraphql(graphql_query: &str) -> Result<JsonValue> {
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
