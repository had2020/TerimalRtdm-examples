use ::TerimalRtdm::*;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    clear(&mut app);

    raw_mode(true);
    clear(&mut app);

    loop {
        collect_presses(&mut app);

        if Key::o()
            .no_clear()
            .case_sen(true)
            .pressed(&mut app, KeyType::Q)
        {
            break;
        }

        render(&app);
    }

    raw_mode(false);
}
