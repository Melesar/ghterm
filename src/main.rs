mod backend; 
mod frontend;
mod app;

use app::App;
use termion::raw::IntoRawMode;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);

    let stdin = termion::async_stdin();
    let app = App::new(stdout, stdin);
    app.run()
}
