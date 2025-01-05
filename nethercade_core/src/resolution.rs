pub enum Resolution {
    Full,   // 1920x1080
    High,   // 960x540
    Medium, // 640x360
    Low,    // 480x270
}

impl Resolution {
    /// Returns the width and height of the resolution.
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            Resolution::Full => (1920, 1080),
            Resolution::High => (960, 540),
            Resolution::Medium => (640, 360),
            Resolution::Low => (480, 270),
        }
    }
}
