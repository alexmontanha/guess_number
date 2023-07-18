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
        .window_size((800.0, 600.0));

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

    let mut layout = Flex::column()
        .with_child(label)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(button)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label_name)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(label_message)
        .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_spacer(VERTICAL_WIDGET_SPACING);

    // create an Vec<(String, i32)> invoked from StateMachine::load_score()
    let scores = StateMachine::load_score().unwrap_or_default();
   
    //create a flex with the array of tuples
    let mut flex_scores = Flex::column();
    for player_score in scores {
        flex_scores = flex_scores.with_child(Label::new(format!("{} => {}", player_score.0, player_score.1)));
    }

    //add the flex of scores to the layout
    layout = layout.with_child(flex_scores);

    Align::centered(layout)
}
