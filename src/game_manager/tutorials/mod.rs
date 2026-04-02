mod message;
mod open_button;
mod ui;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    message::plugin(app);
    open_button::plugin(app);
    ui::plugin(app);
}
