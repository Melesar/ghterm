mod backend; 
mod frontend;
mod app;
mod logs;

use app::App;
use backend::gh::GhClient;
use termion::raw::IntoRawMode;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);

    let stdin = termion::async_stdin();
    let gh_client = GhClient::new(String::from("blindflugstudios"), String::from("First-Strike-Evolution")); //TODO get repo from arguments
    let app = App::new(stdout, stdin, gh_client);
    app.run()
}
