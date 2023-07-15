mod machine;

use druid::widget::{Align, Button, Flex, Label, TextBox};
use druid::{AppLauncher, Env, LocalizedString, Widget, WidgetExt, WindowDesc};
use machine::machine::StateMachine;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<StateMachine> = LocalizedString::new("Guess the number");

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = StateMachine::new();

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<StateMachine> {
    let label =
        Label::new(|data: &StateMachine, _env: &Env| format!("Player Name: {}", data.player_name));

    let textbox = TextBox::new()
        .with_placeholder("")
        .fix_width(TEXT_BOX_WIDTH)
        .lens(StateMachine::entry);

    let button = Button::<StateMachine>::new("Confirm")
        .on_click(move |_ctx, data: &mut StateMachine, _env| data.increment())
        .padding(2.0);

    let label_name = Label::new(|data: &StateMachine, _env: &Env| format!("{}", data.name));
    let label_message = Label::new(|data: &StateMachine, _env: &Env| format!("{}", data.message));

    let layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label_message);

    Align::centered(layout)
}
