use std::time::{Duration, Instant};

pub struct GameTimer {
    start_time: Option<Instant>,
    end_time: Option<Instant>,
    duration: Option<Duration>,
}

impl GameTimer {
    pub fn new() -> Self{
        Self {
            start_time: None,
            end_time: None,
            duration: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    pub fn end(&mut self) {
        match self.start_time {
            Some(start) => {
                let end = Instant::now();
                self.end_time = Some(end);
                self.duration = Some(end.duration_since(start));
            },
            None => ()
        }
    }

    pub fn is_started(&self) -> bool{
        matches!(self.start_time, Some(_))
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.end_time, Some(_))
    }

    pub fn duration(&self) -> Option<Duration> {
        self.duration
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timer_test(){
        let mut timer = GameTimer::new();
        timer.start();

        std::thread::sleep(std::time::Duration::from_secs(1));

        timer.end();

        println!("{:?}", timer.duration().unwrap());
    }
}
