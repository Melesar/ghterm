mod backend; 
mod frontend;
mod app;
mod logs;

extern crate args;
extern crate getopts;

use getopts::Occur;

use std::env::ArgsOs;

use app::App;
use args::{Args, ArgsError};
use backend::gh::GhClient;
use termion::raw::IntoRawMode;

#[derive(Debug)]
struct RepoParams {
    owner: String,
    repo: String,
    pr_num: Option<u32>,
}

//TODO unify errors 
fn main() -> Result<(), std::io::Error> {
    logs::start_logs()?;

    let mut description = Args::new("ghterm", "A terminal application for manipulating GitHub pull requests");
    description.flag("h", "help", "Prints help message");
    description.option("r", "repo", "Name of the repository", "REPO", Occur::Optional, Some(":repo".to_string()));
    description.option("o", "owner", "Owner of the repository", "OWNER", Occur::Optional, Some(":owner".to_string()));
    description.option("n", "number", "Number of the PR to show", "NUMBER", Occur::Optional, None);

    description.parse(std::env::args_os())
        .map_err(|e| map_args_error(e))?;

    if description.value_of("help").map_err(|e| map_args_error(e))? {
        println!("{}", description.full_usage());
        return Ok(());
    }

    let stdout = std::io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);

    let stdin = termion::async_stdin();
    let repo_params = get_repo_params(&description);
    let gh_client = GhClient::new(repo_params.owner, repo_params.repo)?;
    gh_client.validate(repo_params.pr_num).map_err(|e| std::io::Error::new (std::io::ErrorKind::Other, e.message))?;
    let app = App::new(stdout, stdin, gh_client);
    app.run(repo_params.pr_num)
}

fn get_repo_params(args: &Args) -> RepoParams {
    let owner = args.value_of("owner").unwrap();
    let repo = args.value_of("repo").unwrap();
    let pr_num = args.optional_value_of("number").unwrap();
    RepoParams {owner, repo, pr_num}
}

fn map_args_error(e: ArgsError) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
}
