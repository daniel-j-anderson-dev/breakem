use super::Game;
use macroquad::{math::Vec2, rand::gen_range};

impl Game {
    pub(super) fn handle_collision(&mut self) {
        self.keep_in_play_field();

        // ball and blocks
        for block in self.level.blocks_mut() {
            if block.boundary().overlaps(self.ball.boundary()) && block.is_alive() {
                let offset = self.ball.boundary().center().x - block.boundary().center().x;
                let normalized_offset = offset / (block.boundary().w / 2.0);

                self.ball.set_velocity(Vec2::new(
                    normalized_offset * gen_range(2.0, 5.0),
                    -self.ball.velocity().y,
                ));

                block.set_is_alive(false);
            }
        }

        // ball and paddle
        if self.paddle.boundary().overlaps(self.ball.boundary()) {
            let offset = self.ball.boundary().center().x - self.paddle.boundary().center().x;
            let normalized_offset = offset / (self.paddle.boundary().w / 2.0);

            self.ball.set_velocity(Vec2::new(
                normalized_offset * gen_range(2.0, 5.0),
                -self.ball.velocity().y,
            ));
        }
    }
    fn keep_in_play_field(&mut self) {
        // keep paddle in bounds
        if self.paddle.next_boundary().left() < self.play_field.left() {
            self.paddle.set_position(Vec2::new(
                self.play_field.left(),
                self.paddle.next_boundary().y,
            ));
        }
        if self.paddle.next_boundary().right() > self.play_field.right() {
            self.paddle.set_position(Vec2::new(
                self.play_field.right() - self.paddle.next_boundary().w,
                self.paddle.next_boundary().y,
            ));
        }

        // keep ball in bounds. simple reflection on walls
        if self.ball.next_boundary().left() <= self.play_field.left()
            || self.ball.next_boundary().right() >= self.play_field.right()
        {
            self.ball.reflect_x_velocity();
        }
        if self.ball.next_boundary().top() <= self.play_field.top() {
            self.ball.reflect_y_velocity();
        }
        if self.ball.next_boundary().bottom() >= self.play_field.bottom() {
            self.ball.reflect_y_velocity();

            self.lives -= 1;
        }
    }
}
