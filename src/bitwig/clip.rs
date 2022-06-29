#[derive(Debug, Clone)]
pub struct Clip {
    pub(crate) state: ClipState,
    pub(crate) intensity: u8,
}

impl Clip {
    pub(crate) fn new() -> Self {
        Self {
            state: ClipState::Empty,
            intensity: 0,
        }
    }

    pub(crate) fn update_intensity(&mut self) {
        match self.state {
            ClipState::Empty => self.intensity = 0,
            ClipState::Filled => self.intensity = 100,
            ClipState::Playing => self.intensity = 255,
            ClipState::Queued => self.intensity = self.intensity.wrapping_add(1),
            ClipState::Stopping => self.intensity = self.intensity.wrapping_sub(1),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClipState {
    Empty,
    Filled,
    Playing,
    Queued,
    Stopping,
}