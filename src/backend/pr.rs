use chrono::{DateTime, Local};
use super::gh::*;
use json::JsonValue;
use std::io::Result;

#[derive(Debug)]
pub struct PrHeader {
    pub number: u32,
    pub title: String,
}

pub struct Pr {
    pub info: PrInfo,
} 

#[derive(Debug)]
pub struct PrInfo {
    pub number: u32,
    pub title: String,
    pub base_branch: String,
    pub head_branch: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct PrComment {
    pub id: String,
    pub author_name: String,
    pub body: String,
    pub timestamp: DateTime<Local>,
}

pub struct PrCommentReply (PrComment, Option<String>);

#[derive(Debug)]
pub struct PrConversationThread {
    // pub code_hunk: Option<CodeHunk>,
    pub comments: Vec<PrComment>,
}

#[derive(Debug)]
pub struct PrReview {
    pub review_comment: PrComment,
    pub threads: Vec<PrConversationThread>,
}

#[derive(Debug)]
pub enum ConversationItem {
    Comment(PrComment),
    Review(PrReview),
}

#[derive(Debug)]
pub struct PrConversation {
    pub items: Vec<ConversationItem>,
}

pub fn list_prs (json: JsonValue) -> Vec<PrHeader> {
    let output = json["data"]["repository"]["pullRequests"]["edges"].members();
    let mut prs = Vec::new();
    for pr in output {
        let number = pr["node"]["number"].as_u32().unwrap();
        let title = pr["node"]["title"].as_str().unwrap().to_string();
        prs.push(PrHeader{number, title});
    }
    
    prs
}


pub fn parse_conversation(json: JsonValue) -> PrConversation {
    PrConversation{items: vec![]}
}

fn fetch_pr_comment(node: &json::JsonValue) -> PrCommentReply {
    let id = node["id"].as_str().unwrap().to_string();
    let author_name = node["author"]["login"].as_str().unwrap().to_string();
    let body = node["body"].as_str().unwrap().to_string();
    let reply_to = node["replyTo"].as_str().map(|s| s.to_string());
    let timestamp = node["createdAt"].as_str().unwrap();
    let timestamp = DateTime::parse_from_rfc3339(timestamp).unwrap();
    let timestamp = timestamp.with_timezone(&Local);
    PrCommentReply(PrComment {id, author_name, body, timestamp}, reply_to)
}

unsafe impl Send for PrHeader { }
