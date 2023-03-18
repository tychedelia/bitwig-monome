#[derive(Debug)]
pub enum BitwigMessage {
    Clip(ClipMessage),
    Track(TrackMessage),
    Primary(DeviceMessage),
}

#[derive(Debug)]
pub struct DeviceMessage {
    pub(crate) param: u8,
    pub(crate) value: i32, // MAX_VALUE is u8 by default, but can be configured in the osc controller under res
}

impl DeviceMessage {
    pub fn new(param: u8, value: i32) -> Self {
        Self { param, value }
    }
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
pub struct TrackMessage {
    track: u8,
    active: bool,
    pub(crate) event: TrackEvent,
}

impl TrackMessage {
    pub fn new(track: u8, active: bool, event: TrackEvent) -> Self {
        Self {
            track,
            active,
            event,
        }
    }
}

#[derive(Debug)]
pub enum TrackEvent {
    Selected,
}

#[derive(Debug)]
pub enum ControlMessage {
    Refresh,
    Launch(u8, u8),
    Stop(u8, u8),
}
