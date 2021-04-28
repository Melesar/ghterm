use termion::raw::IntoRawMode;

mod backend; 
mod frontend;

pub use backend::pr::PR;
pub use backend::gh;
pub use frontend::repo_selection::RepoSelectionScreen;

fn main() {
    RepoSelectionScreen::draw(std::io::stdout().lock().into_raw_mode().unwrap());
}
