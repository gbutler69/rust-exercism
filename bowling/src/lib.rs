use std::ops::Add;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Copy, Clone)]
enum Frame {
    Empty,
    Incomplete { pins: u8 },
    Completed { pins: [u8; 2], score: u8 },
    IncompleteSpare { pins: [u8; 2] },
    CompletedSpare { pins: [u8; 2], score: u8 },
    IncompleteStrike1,
    IncompleteStrike2 { score: u8 },
    CompletedStrike { score: u8 },
}

enum FrameStatus {
    Incomplete,
    Completed,
    Next,
    CompletedNext,
}

impl Frame {
    fn apply_roll(&mut self, pins: u8) -> Result<FrameStatus, Error> {
        match self {
            //
            // Any over 10 pins
            _ if pins > 10 => Err(Error::NotEnoughPinsLeft),
            //
            // Empty Frames (awaiting first roll)
            Frame::Empty if pins == 10 => {
                *self = Frame::IncompleteStrike1;
                Ok(FrameStatus::Next)
            }
            Frame::Empty => {
                *self = Frame::Incomplete { pins };
                Ok(FrameStatus::Incomplete)
            }
            //
            // Incomplete Frames (awaiting 2nd roll)
            Frame::Incomplete { pins: cur_pins } if *cur_pins + pins < 10 => {
                *self = Frame::Completed {
                    pins: [*cur_pins, pins],
                    score: *cur_pins + pins,
                };
                Ok(FrameStatus::CompletedNext)
            }
            Frame::Incomplete { pins: cur_pins } if *cur_pins + pins == 10 => {
                *self = Frame::IncompleteSpare {
                    pins: [*cur_pins, pins],
                };
                Ok(FrameStatus::Next)
            }
            Frame::Incomplete { .. } => Err(Error::NotEnoughPinsLeft),
            //
            // Spare Frames (awaiting following role to complete score)
            Frame::IncompleteSpare { pins: cur_pins } => {
                *self = Frame::CompletedSpare {
                    pins: *cur_pins,
                    score: cur_pins.iter().sum::<u8>() + pins,
                };
                Ok(FrameStatus::Completed)
            }
            //
            // Strike Frames (awaiting following roles to complete score)
            Frame::IncompleteStrike1 => {
                *self = Frame::IncompleteStrike2 { score: 10 + pins };
                Ok(FrameStatus::Incomplete)
            }
            Frame::IncompleteStrike2 { score: cur_score }
                if *cur_score < 20 && *cur_score - 10 + pins > 10 =>
            {
                Err(Error::NotEnoughPinsLeft)
            }
            Frame::IncompleteStrike2 { score: cur_score } => {
                *self = Frame::CompletedStrike {
                    score: *cur_score + pins,
                };
                Ok(FrameStatus::Completed)
            }
            //
            // Completed Frames (should never have pins applied)
            Frame::Completed { .. }
            | Frame::CompletedSpare { .. }
            | Frame::CompletedStrike { .. } => Err(Error::NotEnoughPinsLeft),
        }
    }

    fn score(&self) -> u8 {
        match self {
            Frame::Completed { score, .. }
            | Frame::CompletedSpare { score, .. }
            | Frame::CompletedStrike { score, .. } => *score,
            _ => panic!("Incomplete Frame cannot be scored"),
        }
    }
}

impl Default for Frame {
    fn default() -> Self {
        Frame::Empty
    }
}

pub struct BowlingGame {
    frames: [Frame; 10],
    current_frame: u8,
    first_incomplete_frame: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: [Default::default(); 10],
            current_frame: 0,
            first_incomplete_frame: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if self.current_frame >= 10 {
            Err(Error::GameComplete)
        } else {
            for f in self.first_incomplete_frame..self.current_frame {
                match self.frames[f as usize].apply_roll(pins as u8) {
                    Ok(FrameStatus::Completed) => self.first_incomplete_frame += 1,
                    Ok(FrameStatus::Incomplete) => (),
                    Err(e) => return Err(e),
                    _ => {
                        unreachable!("All these frames will either return Completed or Incomplete")
                    }
                }
            }
            match self.frames[self.current_frame as usize].apply_roll(pins as u8) {
                Ok(FrameStatus::CompletedNext) | Ok(FrameStatus::Completed) => {
                    self.current_frame += 1;
                    self.first_incomplete_frame += 1;
                    Ok(())
                }
                Ok(FrameStatus::Next) => {
                    self.current_frame = self.current_frame.add(1).min(9);
                    Ok(())
                }
                Ok(FrameStatus::Incomplete) => Ok(()),
                Err(e) => return Err(e),
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        match self.first_incomplete_frame {
            10 => Some(self.frames.iter().map(|f| f.score() as u16).sum()),
            _ => None,
        }
    }
}
