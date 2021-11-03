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
use error::Error;

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let repo_params = get_repo_params(&description);
    let gh_client = GhClient::new(repo_params.owner, repo_params.repo)?;
    gh_client.validate(repo_params.pr_num)?;
    let app = App::new(&mut terminal, gh_client);
    let res = app.run(repo_params.pr_num);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;

    res
}

fn get_repo_params(args: &Args) -> RepoParams {
    let owner = args.value_of("owner").unwrap();
    let repo = args.value_of("repo").unwrap();
    let pr_num = args.optional_value_of("number").unwrap();
    RepoParams {owner, repo, pr_num}
}
