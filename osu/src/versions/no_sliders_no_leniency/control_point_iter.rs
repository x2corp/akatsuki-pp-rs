use parse::{Beatmap, DifficultyPoint, TimingPoint};
use std::slice::Iter;

pub(crate) struct ControlPointIter<'p> {
    timing_points: Iter<'p, TimingPoint>,
    difficulty_points: Iter<'p, DifficultyPoint>,

    next_timing: Option<f32>,
    next_difficulty: Option<(f32, f32)>,
}

impl<'p> ControlPointIter<'p> {
    pub(crate) fn new(map: &'p Beatmap) -> Self {
        let mut timing_points = map.timing_points.iter();
        let mut difficulty_points = map.difficulty_points.iter();

        Self {
            next_timing: timing_points.next().map(|t| t.time),
            next_difficulty: difficulty_points
                .next()
                .map(|d| (d.time, d.speed_multiplier)),

            timing_points,
            difficulty_points,
        }
    }
}

pub(crate) enum ControlPoint {
    Timing { time: f32 },
    Difficulty { time: f32, speed_multiplier: f32 },
}

impl<'p> Iterator for ControlPointIter<'p> {
    type Item = ControlPoint;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.next_timing, self.next_difficulty) {
            (Some(time), Some((d, _))) if time < d => {
                self.next_timing = self.timing_points.next().map(|t| t.time);

                Some(ControlPoint::Timing { time })
            }
            (Some(_), Some((time, speed_multiplier))) => {
                self.next_difficulty = self
                    .difficulty_points
                    .next()
                    .map(|d| (d.time, d.speed_multiplier));

                Some(ControlPoint::Difficulty {
                    time,
                    speed_multiplier,
                })
            }
            (Some(time), _) => {
                self.next_timing = self.timing_points.next().map(|t| t.time);

                Some(ControlPoint::Timing { time })
            }
            (_, Some((time, speed_multiplier))) => {
                self.next_difficulty = self
                    .difficulty_points
                    .next()
                    .map(|d| (d.time, d.speed_multiplier));

                Some(ControlPoint::Difficulty {
                    time,
                    speed_multiplier,
                })
            }
            (None, None) => None,
        }
    }
}
