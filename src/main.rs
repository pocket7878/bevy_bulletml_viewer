use bulletml::parse::BulletMLParser;
use bulletml::{AppRunner, Runner, RunnerData, State};
use piston_window::*;
use rand::Rng;
use std::env;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: f64,
    y: f64,
}

fn get_direction(from: Pos, to: Pos) -> f64 {
    f64::atan2(to.x - from.x, from.y - to.y)
}

struct Bullet {
    pos: Pos,
    direction: f64,
    speed: f64,
}

impl Bullet {
    fn mov(&mut self) {
        self.pos.x += f64::sin(self.direction * std::f64::consts::PI / 180.) * self.speed;
        self.pos.y -= f64::cos(self.direction * std::f64::consts::PI / 180.) * self.speed;
    }

    fn is_out_of_bounds(&self) -> bool {
        self.pos.x < 0.
            || self.pos.y < 0.
            || self.pos.x > WIDTH as f64
            || self.pos.y >= HEIGHT as f64
    }
}

struct BulletMLViewerRunner {
    bullet: Bullet,
    vanished: bool,
    new_runners: Vec<Runner<BulletMLViewerRunner>>,
    new_bullets: Vec<Bullet>,
}

struct BulletMLViewerRunnerData {
    turn: u32,
    enemy_pos: Pos,
    ship_pos: Pos,
}

impl AppRunner<BulletMLViewerRunnerData> for BulletMLViewerRunner {
    fn get_bullet_direction(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        self.bullet.direction
    }

    fn get_aim_direction(&self, data: &BulletMLViewerRunnerData) -> f64 {
        get_direction(self.bullet.pos, data.ship_pos) * 180. / std::f64::consts::PI
    }

    fn get_bullet_speed(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        self.bullet.speed
    }

    fn get_default_speed(&self) -> f64 {
        1.
    }

    fn get_rank(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        0.5
    }

    fn create_simple_bullet(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        direction: f64,
        speed: f64,
    ) {
        self.new_bullets.push(Bullet {
            pos: self.bullet.pos,
            direction,
            speed,
        });
    }

    fn create_bullet(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        state: State,
        direction: f64,
        speed: f64,
    ) {
        let runner = Runner::new_from_state(
            BulletMLViewerRunner {
                bullet: Bullet {
                    pos: self.bullet.pos,
                    direction,
                    speed,
                },
                vanished: false,
                new_runners: Vec::default(),
                new_bullets: Vec::default(),
            },
            state,
        );
        self.new_runners.push(runner);
    }

    fn get_turn(&self, data: &BulletMLViewerRunnerData) -> u32 {
        data.turn
    }

    fn do_vanish(&mut self, _data: &mut BulletMLViewerRunnerData) {
        self.vanished = true;
    }

    fn do_change_direction(&mut self, _data: &mut BulletMLViewerRunnerData, direction: f64) {
        self.bullet.direction = direction;
    }

    fn do_change_speed(&mut self, _data: &mut BulletMLViewerRunnerData, speed: f64) {
        self.bullet.speed = speed;
    }

    fn get_rand(&self, _data: &mut BulletMLViewerRunnerData) -> f64 {
        rand::thread_rng().gen()
    }
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bml_file = &args[1];

    let bml = BulletMLParser::with_capacities(0, 128)
        .parse_file(bml_file)
        .unwrap();

    let mut data = BulletMLViewerRunnerData {
        turn: 0,
        enemy_pos: Pos {
            x: WIDTH as f64 / 2.,
            y: HEIGHT as f64 * 0.3,
        },
        ship_pos: Pos {
            x: WIDTH as f64 / 2.,
            y: HEIGHT as f64 * 0.9,
        },
    };

    let mut runners = Vec::new();
    let mut bullets = Vec::<Bullet>::new();

    let start_time = Instant::now();
    let mut prev_millis = 0;

    let mut window: PistonWindow = WindowSettings::new("BulletML Viewer", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        let now_millis = {
            let duration = Instant::now().duration_since(start_time);
            duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
        };

        let frame = (now_millis - prev_millis) / 10;
        prev_millis += frame * 10;

        if runners.is_empty() && bullets.is_empty() {
            runners.push(Runner::new(
                BulletMLViewerRunner {
                    bullet: Bullet {
                        pos: data.enemy_pos,
                        direction: get_direction(data.enemy_pos, data.ship_pos) * 180.
                            / std::f64::consts::PI,
                        speed: 0.,
                    },
                    vanished: true,
                    new_runners: Vec::default(),
                    new_bullets: Vec::default(),
                },
                &bml,
            ));
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([0.0, 0.0, 0.0, 1.0], graphics);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [data.enemy_pos.x - 4., data.enemy_pos.y - 4., 8., 8.],
                context.transform,
                graphics,
            );
            rectangle(
                [0.0, 1.0, 0.0, 1.0],
                [data.ship_pos.x - 4., data.ship_pos.y - 4., 8., 8.],
                context.transform,
                graphics,
            );
            for runner in &runners {
                if !runner.vanished && !runner.is_end() {
                    let bullet = &runner.bullet;
                    rectangle(
                        [1.0, 1.0, 0.0, 1.0],
                        [bullet.pos.x - 2., bullet.pos.y - 2., 4., 4.],
                        context.transform,
                        graphics,
                    );
                }
            }
            for bullet in &bullets {
                rectangle(
                    [1.0, 1.0, 0.0, 1.0],
                    [bullet.pos.x - 2., bullet.pos.y - 2., 4., 4.],
                    context.transform,
                    graphics,
                );
            }
        });

        for _ in 0..frame {
            let mut new_runners = Vec::new();
            let mut new_bullets = Vec::new();
            for runner in &mut runners {
                runner.bullet.mov();
                runner.run(&mut RunnerData {
                    bml: &bml,
                    data: &mut data,
                });
                new_runners.extend(&mut runner.new_runners.drain(..));
                new_bullets.extend(&mut runner.new_bullets.drain(..));
            }
            for bullet in &mut bullets {
                bullet.mov();
            }

            runners.retain(|runner| !runner.is_end() && !runner.bullet.is_out_of_bounds());
            runners.reserve(new_runners.len());
            for runner in new_runners.drain(..) {
                runners.push(runner);
            }

            bullets.retain(|bullet| !bullet.is_out_of_bounds());
            bullets.reserve(new_bullets.len());
            for runner in new_bullets.drain(..) {
                bullets.push(runner);
            }

            data.turn += 1;
        }

        event.mouse_cursor(|pos| {
            data.ship_pos = Pos {
                x: pos[0],
                y: pos[1],
            };
        });
    }
}
