use chrono::{DateTime, Local};
use json::JsonValue;
use std::collections::HashMap;
use std::iter::Extend;

#[derive(Debug)]
pub struct PrHeader {
    pub number: u32,
    pub title: String,
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

#[derive(Debug)]
pub struct PrConversationThread {
    //TODO Parse code hunks
    // pub code_hunk: Option<CodeHunk>,
    pub comments: Vec<PrComment>,
}

#[derive(Debug)]
pub struct PrReview {
    pub review_comment: PrComment,
    pub verdict: PrReviewVerdict,
    pub threads: Vec<PrConversationThread>,
}

#[derive(Debug)]
pub enum PrReviewVerdict { Comment, Approve, ChangesRequested }

impl std::fmt::Display for PrReviewVerdict {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrReviewVerdict::Comment => write!(f, "COMMENTED"),
            PrReviewVerdict::Approve => write!(f, "APPROVED"),
            PrReviewVerdict::ChangesRequested => write!(f, "REQUESTED CHANGES")
        }
    }
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
    let threads = json["data"]["repository"]["pullRequest"]["reviewThreads"]["edges"].members();
    let reviews = json["data"]["repository"]["pullRequest"]["reviews"]["edges"].members();
    let comments = json["data"]["repository"]["pullRequest"]["comments"]["edges"].members();

    let mut threads_map = HashMap::new();
    for thread in threads {
        let thread_comments = thread["node"]["comments"]["edges"].members();
        let mut comments_list = vec![];
        let mut root_comment = String::new();
        for (index, thread_comment) in thread_comments.enumerate() {
            if index == 0 {
                root_comment = thread_comment["node"]["id"].as_str().unwrap().to_string(); 
            }
            comments_list.push(fetch_pr_comment(&thread_comment["node"]));
        }

        if root_comment.len() > 0 {
            threads_map.insert(root_comment, PrConversationThread {comments: comments_list});
        }
    }

    let mut conversation_items : Vec<ConversationItem> = vec![];
    for review in reviews {
        let verdict = match review["node"]["state"].as_str().unwrap().to_lowercase().as_str() {
            "commented" => PrReviewVerdict::Comment,
            "approved" => PrReviewVerdict::Approve,
            "changes_requested" => PrReviewVerdict::ChangesRequested,
            _ => continue
        };
        let review_comment = fetch_pr_comment(&review["node"]);
        let mut threads = vec![];
        let review_comments = review["node"]["comments"]["edges"].members();
        for comment in review_comments {
            if let Some(thread) = threads_map.remove_entry(comment["node"]["id"].as_str().unwrap()) {
                threads.push(thread.1);
            }
        }

        if !review_comment.body.is_empty() || !threads.is_empty() {
            let review = PrReview {review_comment, verdict, threads};
            conversation_items.push(ConversationItem::Review(review));
        }
    }

    conversation_items.extend(comments
                              .map(|v| fetch_pr_comment(&v["node"]))
                              .map(|c| ConversationItem::Comment(c)));

    PrConversation{items: conversation_items}
}

fn fetch_pr_comment(node: &json::JsonValue) -> PrComment {
    let id = node["id"].as_str().unwrap().to_string();
    let author_name = node["author"]["login"].as_str().unwrap().to_string();
    let body = node["body"].as_str().unwrap().to_string();
    let timestamp = node["publishedAt"].as_str().unwrap();
    let timestamp = DateTime::parse_from_rfc3339(timestamp).unwrap();
    let timestamp = timestamp.with_timezone(&Local);
    PrComment {id, author_name, body, timestamp}
}

unsafe impl Send for PrHeader { }
