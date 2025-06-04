#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

struct Frame {
    frame_type: FrameType,
    // Record roll index of frame creation so we can
    //   get the next rolls to compute this frame's score
    roll_i: usize,
}
enum FrameType {
    Open(u16, u16),
    Spare,
    Strike,
    Tenth(u16, u16, u16),
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    frames: Vec<Frame>,
    current_frame: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            frames: Vec::<Frame>::with_capacity(10),
            current_frame: Vec::with_capacity(3),
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.frames.len() == 10 {
            return Err(Error::GameComplete);
        }

        // We add with modulo because when 10 pins are down, we reset the pins
        // Ex. Last frame: 4 6 <reset> X
        let missing_pins = 10 - (self.current_frame.iter().sum::<u16>() % 10);
        if pins > missing_pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.rolls.push(pins);
        self.current_frame.push(pins);

        if let Some(frame_type) = self.is_frame_closed() {
            self.frames.push(Frame {
                roll_i: self.rolls.len() - 1,
                frame_type,
            });
            self.current_frame.clear();
        }

        Ok(())
    }

    fn is_frame_closed(&self) -> Option<FrameType> {
        if self.frames.len() < 9 {
            match *self.current_frame.as_slice() {
                [10] => Some(FrameType::Strike),
                [x, y] if x + y == 10 => Some(FrameType::Spare),
                [x, y] if x + y < 10 => Some(FrameType::Open(x, y)),
                _ => None,
            }
        } else {
            // 10th frame
            match *self.current_frame.as_slice() {
                [x, y] if x + y < 10 => Some(FrameType::Tenth(x, y, 0)),
                [x, y, z] => Some(FrameType::Tenth(x, y, z)),
                _ => None,
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames.len() < 10 {
            return None;
        }

        let score = self
            .frames
            .iter()
            .map(|f| match f.frame_type {
                FrameType::Open(x, y) => x + y,
                FrameType::Spare => 10 + self.rolls[f.roll_i + 1],
                FrameType::Strike => 10 + self.rolls[f.roll_i + 1] + self.rolls[f.roll_i + 2],
                FrameType::Tenth(x, y, z) => x + y + z,
            })
            .sum();

        Some(score)
    }
}
