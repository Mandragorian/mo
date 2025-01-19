use std::io:: Result;

fn main() -> Result<()> {
    let mut terminal = mo::tui::init()?;
    let app_result = mo::app::App::default().run(&mut terminal);
    mo::tui::restore()?;
    app_result
}
