pub mod critters {
    use ggez::mint::Point2;
    use ggez::graphics::Color;
    use rand::{thread_rng, Rng};
    use crate::math::math::{distance, anglebetween};

    pub const MAX_HUNGER: f32 = 500.0;

    pub struct Prey {

        // Non-genetic traits, mostly for drawing and graphics
        pub position: Point2<f32>,
        pub color: Color,
        pub direction: Point2<f32>,

        // Genetic traits
        pub size: f32,      //
        speed: f32,         // faster creatures consume energy more quickly
        pub eyesight: f32,  // creatures with better eyesight can sense food further away

        // "Lifetime" traits
        pub hunger: f32,        //decreases with movement, creature dies if it starves to death
        pub is_dead: bool,      //dead flag, all dead creatures are removed each update() tick
        pub wants_mate: bool,   //horny flag, signals whether or not the critter wants to mate
    }

    impl Prey {
        pub fn new() -> Self {
            let mut rng = thread_rng();
            let angle: f32 = rng.gen_range(0.0, 6.28318530718);
            let size: f32 = rng.gen_range(5.0, 10.0);

            Prey {
                position: Point2 {x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0)},
                direction: Point2 {x: angle.cos(), y: angle.sin()},
                color: Color::from_rgb(200, 50, 90),

                speed: rng.gen_range(0.1, 0.5),
                size: size,
                eyesight: rng.gen_range(size + 10.0, size + 50.0),

                hunger: MAX_HUNGER,
                is_dead: false,
                wants_mate: false,
            }
        }

        pub fn update(&mut self) {
            if self.hunger > 0.0 {
                if self.position.x + self.size > 800.0 || self.position.x - self.size < 0.0 {
                    self.direction.x *= -1.0;
                }
                if self.position.y + self.size > 600.0 || self.position.y - self.size < 0.0 {
                    self.direction.y *= -1.0;
                }
                self.position.x += self.direction.x * self.speed;
                self.position.y += self.direction.y * self.speed;

                self.hunger -= self.speed;
                if self.hunger > MAX_HUNGER * 0.75 {
                    self.wants_mate = true;
                }
            } else {
                self.is_dead = true;
            }
        }

        pub fn seek_food(&mut self, food_vec: &mut Vec<Food>) {
            if food_vec.is_empty() {
                return;
            }

            let mut nearest_food = None;
            let mut nearest_distance: f32 = std::f32::INFINITY;
            for f in food_vec {
                let current_distance = distance(&self.position, &f.position) - f.size;
                if current_distance < nearest_distance {
                    nearest_food = Some(f);
                    nearest_distance = current_distance;
                }
            }

            let target_food = nearest_food.unwrap();

            if nearest_distance < self.eyesight {
                let angle = anglebetween(&self.position, &target_food.position);
                self.direction.x = angle.cos();
                self.direction.y = angle.sin();
            }

            if nearest_distance < self.size {
                self.hunger +=  10.0 * target_food.sustenance;
                if self.hunger > MAX_HUNGER {
                    self.hunger = MAX_HUNGER;
                }
                target_food.consumed = true;
            }
        }
    }

    pub fn mate_prey(population: &mut Vec<Prey>) {
        population.push(Prey::new());
    }

    pub struct Food {
        pub position: Point2<f32>,
        pub size: f32,
        pub color: Color,

        pub sustenance: f32,

        pub consumed: bool,
    }

    impl Food {
        pub(crate) fn new() -> Self {
            let mut rng = thread_rng();
            let size = rng.gen_range(5.0, 15.0);
            Self {
                position: Point2 {x: rng.gen_range(0.0, 800.0), y: rng.gen_range(0.0, 600.0)},
                size: size,
                color: Color::from_rgb(40, 90, 30),
                sustenance: size * 2.0,
                consumed: false,
            }
        }
    }
}