use bulletml::parse::BulletMLParser;
use bulletml::{AppRunner, BulletML, Runner, RunnerData, State};
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
    f64::atan2(to.x - from.x, to.y - from.y)
}

struct BulletMLViewerRunner {
    pos: Pos,
    direction: f64,
    speed: f64,
    vanished: bool,
    new_runners: Vec<Runner<BulletMLViewerRunner>>,
}

impl BulletMLViewerRunner {
    fn mov(&mut self) {
        self.pos.x += f64::sin(self.direction * std::f64::consts::PI / 180.) * self.speed;
        self.pos.y += f64::cos(self.direction * std::f64::consts::PI / 180.) * self.speed;
    }
}

const MAX_BULLETS: usize = 2000;

struct BulletMLViewerRunnerData<'a> {
    turn: u32,
    enemy_pos: Pos,
    ship_pos: Pos,
    bml: &'a BulletML,
    available_slots: usize,
}

impl<'a> AppRunner<BulletMLViewerRunnerData<'a>> for BulletMLViewerRunner {
    fn get_bullet_direction(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        self.direction
    }

    fn get_aim_direction(&self, data: &BulletMLViewerRunnerData) -> f64 {
        get_direction(self.pos, data.ship_pos) * 180. / std::f64::consts::PI
    }

    fn get_bullet_speed(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        self.speed
    }

    fn get_default_speed(&self) -> f64 {
        1.
    }

    fn get_rank(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        0.5
    }

    fn create_simple_bullet(
        &mut self,
        data: &mut BulletMLViewerRunnerData,
        direction: f64,
        speed: f64,
    ) {
        if data.available_slots == 0 {
            return;
        }
        let runner = Runner::new(
            BulletMLViewerRunner {
                pos: self.pos,
                direction,
                speed,
                vanished: false,
                new_runners: Vec::default(),
            },
            data.bml,
        );
        self.new_runners.push(runner);
        data.available_slots -= 1;
    }

    fn create_bullet(
        &mut self,
        data: &mut BulletMLViewerRunnerData,
        state: State,
        direction: f64,
        speed: f64,
    ) {
        if data.available_slots == 0 {
            return;
        }
        let runner = Runner::new_from_state(
            BulletMLViewerRunner {
                pos: self.pos,
                direction: direction,
                speed: speed,
                vanished: false,
                new_runners: Vec::default(),
            },
            state,
        );
        self.new_runners.push(runner);
        data.available_slots -= 1;
    }

    fn get_turn(&self, data: &BulletMLViewerRunnerData) -> u32 {
        data.turn
    }

    fn do_vanish(&mut self, _data: &mut BulletMLViewerRunnerData) {
        self.vanished = true;
    }

    fn do_change_direction(&mut self, _data: &mut BulletMLViewerRunnerData, direction: f64) {
        self.direction = direction;
    }

    fn do_change_speed(&mut self, _data: &mut BulletMLViewerRunnerData, speed: f64) {
        self.speed = speed;
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
        bml: &bml,
        available_slots: MAX_BULLETS,
    };

    let mut runners = Vec::new();

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

        let frame = (now_millis - prev_millis) / 16;
        prev_millis += frame * 16;

        if runners.is_empty() {
            runners.push(Runner::new(
                BulletMLViewerRunner {
                    pos: data.enemy_pos,
                    direction: get_direction(data.enemy_pos, data.ship_pos),
                    speed: 0.,
                    vanished: false,
                    new_runners: Vec::default(),
                },
                &bml,
            ));
            data.available_slots -= 1;
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
                    rectangle(
                        [1.0, 1.0, 0.0, 1.0],
                        [runner.pos.x - 2., runner.pos.y - 2., 4., 4.],
                        context.transform,
                        graphics,
                    );
                }
            }
        });

        for _ in 0..frame {
            let mut new_runners = Vec::new();
            for runner in &mut runners {
                runner.mov();
                runner.run(&mut RunnerData {
                    bml: &bml,
                    data: &mut data,
                });
                new_runners.extend(&mut runner.new_runners.drain(..));
            }
            runners.retain(|runner| {
                let out_of_bounds = runner.pos.x < 0.
                    || runner.pos.y < 0.
                    || runner.pos.x > WIDTH as f64
                    || runner.pos.y >= HEIGHT as f64;
                !runner.vanished && !runner.is_end() && !out_of_bounds
            });
            runners.reserve(new_runners.len());
            for runner in new_runners.drain(..) {
                runners.push(runner);
            }
            data.available_slots = MAX_BULLETS - runners.len();
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
