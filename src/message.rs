#[derive(Debug)]
pub struct Clip {
    pub(crate) track: u8,
    pub(crate) scene: u8,
    pub(crate) state: ClipState,
}

#[derive(Debug, Clone)]
pub enum ClipState {
    Empty,
    Filled,
    Playing,
    Queued,
    Stopping,
}

#[derive(Debug)]
pub struct ClipMessage {
    pub(crate) track: u8,
    pub(crate) scene: u8,
    pub(crate) active: bool,
    pub(crate) event: ClipEvent,
}

impl ClipMessage {
    pub fn new(track: u8, scene: u8, active: bool, event: ClipEvent) -> Self {
        Self {
            track,
            scene,
            active,
            event,
        }
    }
}

#[derive(Debug)]
pub enum ClipEvent {
    Playing,
    Queued,
    Stopping,
    Content,
    Selected,
}

#[derive(Debug)]
pub enum ControlMessage {
    Refresh,
    Launch(u8, u8),
    Stop(u8, u8),
}
