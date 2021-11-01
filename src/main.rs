mod backend; 
mod frontend;
mod app;
mod logs;
mod error;

extern crate args;
extern crate getopts;
extern crate xdg;

use getopts::Occur;

use app::App;
use args::Args;
use backend::gh::{self, GhClient};
use termion::raw::IntoRawMode;
use error::Error;

#[derive(Debug)]
struct RepoParams {
    owner: String,
    repo: String,
    pr_num: Option<u32>,
}

fn main() {
    logs::start_logs().unwrap();

    let mut description = Args::new("ghterm", "A terminal application for manipulating GitHub pull requests");
    description.flag("h", "help", "Prints help message");
    description.option("r", "repo", "Name of the repository", "REPO", Occur::Optional, Some(":repo".to_string()));
    description.option("o", "owner", "Owner of the repository", "OWNER", Occur::Optional, Some(":owner".to_string()));
    description.option("n", "number", "Number of the PR to show", "NUMBER", Occur::Optional, None);

    description.parse(std::env::args_os()).unwrap();

    if description.value_of("help").unwrap() {
        println!("{}", description.full_usage());
        return;
    }

    match run(&description) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

fn run(description: &Args) -> Result<(), Error> {
    match gh::check_health() {
        Ok(res) => match res {
            false => return Err(error::Error::RefusedToAuthenticate),
            true => ()
        },
        Err(e) => return Err(e),
    }

    let stdout = std::io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);

    let repo_params = get_repo_params(&description);
    let gh_client = GhClient::new(repo_params.owner, repo_params.repo)?;
    gh_client.validate(repo_params.pr_num)?;
    let app = App::new(stdout, gh_client);
    app.run(repo_params.pr_num)
}

fn get_repo_params(args: &Args) -> RepoParams {
    let owner = args.value_of("owner").unwrap();
    let repo = args.value_of("repo").unwrap();
    let pr_num = args.optional_value_of("number").unwrap();
    RepoParams {owner, repo, pr_num}
}
