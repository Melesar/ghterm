use chrono::{DateTime, Local};

use super::gh;
use std::io::Result;
use std::collections::HashMap;

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

pub fn list_prs () -> Result<Vec<PrHeader>> {
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

pub fn fetch_pr(number: u32) -> Result<Pr> { 
    let output = gh::pr_view(number)?;
    let output = &output["data"]["repository"]["pullRequest"];
    let pr_info = PrInfo {
        number: output["number"].as_u32().unwrap(),
        title: output["title"].as_str().unwrap().to_string(),
        base_branch: output["baseRefName"].as_str().unwrap().to_string(),
        head_branch: output["headRefName"].as_str().unwrap().to_string(),
        body: output["body"].as_str().unwrap().to_string(),
    };
    Ok (Pr{info: pr_info})
}

pub fn fetch_conversation(number: u32) -> Result<PrConversation> {
    let output = gh::pr_conversation(number)?;
    let output = &output["data"]["repository"]["pullRequest"];
    let reviews = &output["reviews"]["edges"];
    let mut items = vec![];
    for review in reviews.members() {
        let review_comment = fetch_pr_comment(review).0;
        let comments_json = &review["node"]["comments"]["edges"];
        let mut comments_tree : HashMap<String, Vec<PrComment>> = HashMap::new();
        for comment in comments_json.members() {
            let comment = fetch_pr_comment(comment);
            match comment.1 {
                Some(parent_id) => if let Some(replies) = comments_tree.get_mut(&parent_id) {
                    replies.push(comment.0.clone());
                },
                None => {comments_tree.insert(comment.0.id.clone(), vec![comment.0.clone()]);},
            }
        }

        let threads : Vec<PrConversationThread> = comments_tree.drain().map(|(_k, v)| PrConversationThread{comments: v}).collect();
        let review = PrReview {review_comment, threads};
        items.push(ConversationItem::Review(review));
    }
    Ok(PrConversation{items})
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
