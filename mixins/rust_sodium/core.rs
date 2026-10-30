#[derive(Clone, Copy)]
pub struct Budget {
    pub average_ms: f32,
    pub frames: u32,
    pub chunks: u32,
    pub kib: u32,
}
impl Default for Budget {
    fn default() -> Self {
        Self {
            average_ms: 16.67,
            frames: 0,
            chunks: 8,
            kib: 4096,
        }
    }
}
impl Budget {
    pub fn sample(&mut self, ms: f32) {
        if !ms.is_finite() || !(0.0..=250.0).contains(&ms) {
            return;
        }
        self.average_ms = self.average_ms * 0.9 + ms * 0.1;
        self.frames += 1;
        if self.frames < 30 {
            return;
        }
        self.frames = 0;
        if self.average_ms > 20.0 {
            self.chunks = (self.chunks / 2).max(2);
            self.kib = (self.kib / 2).max(1024);
        } else if self.average_ms < 14.0 {
            self.chunks = (self.chunks + 2).min(32);
            self.kib = (self.kib + 1024).min(16384);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pressure_reduces_work_and_recovery_is_bounded() {
        let mut b = Budget::default();
        for _ in 0..300 {
            b.sample(40.0);
        }
        assert_eq!((b.chunks, b.kib), (2, 1024));
        for _ in 0..1000 {
            b.sample(5.0);
        }
        assert_eq!((b.chunks, b.kib), (32, 16384));
        let old = b.average_ms;
        b.sample(f32::NAN);
        b.sample(10000.0);
        assert_eq!(old, b.average_ms);
    }
}
