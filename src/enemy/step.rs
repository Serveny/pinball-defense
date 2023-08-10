use crate::prelude::*;
use crate::road::points::{ROAD_DISTS, ROAD_POINTS};

pub(super) struct Step {
    pub i_road_point: usize,
    pub direction: Vec3,
    pub distance_to_walk: f32,
    pub distance_walked: f32,
}

impl Step {
    pub fn new(i_point: usize) -> Self {
        let dir = get_direction_to(i_point);
        Self {
            i_road_point: i_point,
            distance_to_walk: ROAD_DISTS[i_point - 1],
            distance_walked: 0.,
            direction: dir.normalize(),
        }
    }

    pub fn next(&self) -> Self {
        Self::new(self.i_road_point + 1)
    }
    pub fn walk(&mut self, current_pos: Vec3, distance: f32) -> Vec3 {
        self.distance_walked += distance;
        current_pos + self.direction * distance
    }

    pub fn start_pos(&self) -> Vec3 {
        ROAD_POINTS[self.i_road_point - 1]
    }

    pub fn is_reached_point(&self) -> bool {
        self.distance_walked >= self.distance_to_walk
    }

    pub fn is_reached_road_end(&self) -> bool {
        self.i_road_point == ROAD_POINTS.len() - 1 && self.is_reached_point()
    }
}

fn get_direction_to(i: usize) -> Vec3 {
    ROAD_POINTS[i] - ROAD_POINTS[i - 1]
}
