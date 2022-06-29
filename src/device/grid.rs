use crate::bitwig::clip::Clip;

pub(crate) struct Grid {
    grid: Vec<Clip>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            grid: vec![Clip::new(); 128],
        }
    }

    pub(crate) fn update_intensities(&mut self) {
        self.grid.iter_mut().for_each(|x| x.update_intensity());
    }

    pub(crate) fn get_state(&mut self, track: u8, scene: u8) -> &mut Clip {
        &mut self.grid[(scene as usize - 1) * 16 + (track as usize - 1)]
    }

    pub(crate) fn get_intensities(&self) -> Vec<u8> {
        self.grid.iter().map(|x| x.intensity).collect()
    }
}
