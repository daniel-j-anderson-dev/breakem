use super::Game;
use macroquad::{math::Vec2, rand::gen_range};

impl Game {
    pub(super) fn handle_collision(&mut self) {
        self.keep_in_play_field();

        // ball and blocks
        for block in self.level.blocks_mut() {
            if block.hitbox().overlaps(self.ball.hitbox()) && block.is_alive() {
                let offset = self.ball.hitbox().center().x - block.hitbox().center().x;
                let normalized_offset = offset / (block.hitbox().w / 2.0);

                self.ball.set_velocity(Vec2::new(
                    normalized_offset * gen_range(2.0, 5.0),
                    -self.ball.velocity().y,
                ));

                block.set_is_alive(false);
            }
        }

        // ball and paddle
        if self.paddle.hitbox().overlaps(self.ball.hitbox()) {
            let offset = self.ball.hitbox().center().x - self.paddle.hitbox().center().x;
            let normalized_offset = offset / (self.paddle.hitbox().w / 2.0);

            self.ball.set_velocity(Vec2::new(
                normalized_offset * gen_range(2.0, 5.0),
                -self.ball.velocity().y,
            ));
        }
    }
    fn keep_in_play_field(&mut self) {
        // keep paddle in bounds
        if self.paddle.next_hitbox().left() < self.play_field_hitbox.left() {
            self.paddle.set_position(Vec2::new(
                self.play_field_hitbox.left(),
                self.paddle.next_hitbox().y,
            ));
        }
        if self.paddle.next_hitbox().right() > self.play_field_hitbox.right() {
            self.paddle.set_position(Vec2::new(
                self.play_field_hitbox.right() - self.paddle.next_hitbox().w,
                self.paddle.next_hitbox().y,
            ));
        }

        // keep ball in bounds. simple reflection on walls
        if self.ball.next_hitbox().left() <= self.play_field_hitbox.left()
            || self.ball.next_hitbox().right() >= self.play_field_hitbox.right()
        {
            self.ball.reflect_x_velocity();
        }
        if self.ball.next_hitbox().top() <= self.play_field_hitbox.top() {
            self.ball.reflect_y_velocity();
        }
        if self.ball.next_hitbox().bottom() >= self.play_field_hitbox.bottom() {
            self.ball.reflect_y_velocity();

            self.lives -= 1;
        }
    }
}
