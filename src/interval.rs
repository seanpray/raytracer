const INTERVAL_UNIVERSE: Interval = Interval {
    min: f32::NEG_INFINITY,
    max: f32::INFINITY,
};

const INTERVAL_EMPTY: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};

#[derive(Default)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
