use chrono::{Datelike, Local, Timelike};
use cursive::{
    Cursive, CursiveExt,
    event::Key,
    view::{Nameable, Resizable},
    views::{Checkbox, Dialog, DummyView, EditView, LinearLayout, TextView},
};
use std::{
    env::temp_dir,
    process::{Command, Stdio},
};

const GET_USER_DATA_EXPECT_MSG: &'static str = "Should be able to get user data";

#[derive(Debug)]
struct State {
    fullscreen: bool,
    temp: bool,
    copy_to_clipboard: bool,
    file_name: String,
    confirmed: bool,
}

impl State {
    fn new() -> Self {
        Self {
            fullscreen: false,
            temp: false,
            copy_to_clipboard: true,
            file_name: String::new(),
            confirmed: false,
        }
    }
}

fn get_user_data(cursive: &mut Cursive) -> &mut State {
    cursive.user_data().expect(GET_USER_DATA_EXPECT_MSG)
}

fn set_fullscreen(cursive: &mut Cursive, fullscreen: bool) {
    get_user_data(cursive).fullscreen = fullscreen;
}

fn set_temporary(cursive: &mut Cursive, temp: bool) {
    get_user_data(cursive).temp = temp;
}

fn set_copy(cursive: &mut Cursive, copy: bool) {
    get_user_data(cursive).copy_to_clipboard = copy;
}

fn confirm_exit(cursive: &mut Cursive) {
    get_user_data(cursive).confirmed = true;
    cursive.quit();
}

fn main() {
    let state = State::new();
    let mut cursive = Cursive::new();
    cursive.set_user_data(state);

    cursive
        .load_toml(include_str!("assets/style.toml"))
        .expect("Load toml file should succeed");

    let subtitle = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(TextView::new("<wl-termshot>"))
        .child(DummyView.full_width());

    let guide = TextView::new("<q> - Quit\n<Esc> - Confirm and exit");

    let fullscreen_checkbox = LinearLayout::horizontal()
        .child(TextView::new("Fullscreen?"))
        .child(DummyView.full_width())
        .child(Checkbox::new().on_change(set_fullscreen));

    let temp_checkbox = LinearLayout::horizontal()
        .child(TextView::new("Temporary?"))
        .child(DummyView.full_width())
        .child(Checkbox::new().on_change(set_temporary));

    let copy_checkbox = LinearLayout::horizontal()
        .child(TextView::new("Copy to clipboard?"))
        .child(DummyView.full_width())
        .child(Checkbox::new().checked().on_change(set_copy));

    let file_name = LinearLayout::horizontal()
        .child(TextView::new("File name"))
        .child(DummyView.full_width())
        .child(EditView::new().with_name("filename").min_width(15));

    let dialog = Dialog::around(
        LinearLayout::vertical()
            .child(subtitle)
            .child(guide)
            .child(DummyView)
            .child(fullscreen_checkbox)
            .child(temp_checkbox)
            .child(copy_checkbox)
            .child(file_name),
    )
    .title("Wayland Terminal Screenshotter")
    .button("confirm", confirm_exit)
    .with_name("dialog")
    .fixed_width(60);

    cursive.add_layer(dialog);
    cursive.add_global_callback('q', Cursive::quit);
    cursive.add_global_callback(Key::Esc, confirm_exit);

    cursive.run();

    let mut state = cursive
        .take_user_data::<State>()
        .expect(GET_USER_DATA_EXPECT_MSG);
    state.file_name = cursive
        .find_name::<EditView>("filename")
        .expect("Should find filename")
        .get_content()
        .to_string();
    if state.confirmed {
        run_command(state);
    }
}

fn run_command(state: State) {
    let mut file_name = state.file_name;
    if file_name.trim().is_empty() {
        let now = Local::now();
        let date = now.date_naive();
        let time = now.time();
        file_name = format!(
            "screenshot_{}-{:0>2}-{:0>2}-{:0>2}:{:0>2}:{:0>2}",
            date.year(),
            date.month0() + 1,
            date.day0() + 1,
            time.hour(),
            time.minute(),
            time.second()
        );
    }
    let grim_dir_env = if !state.temp {
        std::env::var("GRIM_DEFAULT_DIR").unwrap_or(".".to_string())
    } else {
        temp_dir().display().to_string()
    };
    let grim_dir = grim_dir_env.trim_end_matches('/');
    let grim_args = if state.fullscreen { "" } else { "-g (slurp)" };
    let copy_command = if state.copy_to_clipboard {
        format!("wl-copy <{grim_dir}/{file_name}.png")
    } else {
        String::new()
    };
    let notify_msg = if state.copy_to_clipboard {
        "copied"
    } else {
        "done"
    };
    let command = format!(
        "sleep 1; grim {grim_args} {grim_dir}/{file_name}.png; notify-send -i \"{grim_dir}/{file_name}.png\" -a \"Screenshotter\" \"Screenshot {notify_msg}!\" \"Saved in {grim_dir}/{file_name}.png.\"; {copy_command}"
    );
    Command::new("setsid")
        .arg("-f")
        .arg("fish")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
        .expect("Failed to take screenshot")
        .wait()
        .expect("Failed to start new session");
}
