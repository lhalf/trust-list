#[cfg_attr(test, autospy::autospy)]
pub trait ProgressBar {
    fn set_total(&mut self, total: u64);
    fn set_message(&mut self, message: &str);
    fn increment(&mut self);
}

impl ProgressBar for pbr::ProgressBar<std::io::Stdout> {
    fn set_total(&mut self, total: u64) {
        self.total = total
    }

    fn set_message(&mut self, message: &str) {
        self.message(message);
    }

    fn increment(&mut self) {
        self.inc();
    }
}

pub fn build() -> pbr::ProgressBar<std::io::Stdout> {
    let mut bar = pbr::ProgressBar::new(0);
    bar.format("╢▌▌░╟");
    bar.show_speed = false;
    bar.show_percent = false;
    bar
}
