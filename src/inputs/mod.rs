use self::key::Key;

pub mod key;
pub mod events;

pub enum InputEvent {
    Input(Key),
    Tick,
}