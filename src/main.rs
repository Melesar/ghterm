mod backend; 
mod frontend;
mod app;
mod logs;

use app::App;
use termion::raw::IntoRawMode;

fn main() -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let stdout = stdout.lock().into_raw_mode().unwrap();
    let stdout = termion::screen::AlternateScreen::from(stdout);

    let stdin = termion::async_stdin();
    let app = App::new(stdout, stdin);
    let result = app.run();
    logs::clear_logs();
    result
}
