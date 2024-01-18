use std::time::Duration;

use rand::Rng;
use slint::{Model, ModelRc, Timer, VecModel, Weak};

slint::include_modules!();

fn generate_random_board() -> ModelRc<i32> {
    let mut board = vec![0; 16];
    let mut rng = rand::thread_rng();
    for i in 1..=15 {
        let mut r = rng.gen_range(0..4);
        let mut c = rng.gen_range(0..4);
        while board[r * 4 + c] != 0 {
            r = rng.gen_range(0..4);
            c = rng.gen_range(0..4);
        }
        board[r * 4 + c] = i;
    }

    ModelRc::new(VecModel::from(board))
}

fn make_step(ui_weak: Weak<AppWindow>, clicked_r: i32, clicked_c: i32) {
    let ui = ui_weak.upgrade().unwrap();
    let board = ui.get_locations();
    let zero_r = board.row_data(0).unwrap() / 4;
    let zero_c = board.row_data(0).unwrap() % 4;

    for j in 1..=15 {
        let mut r = board.row_data(j).unwrap() / 4;
        let mut c = board.row_data(j).unwrap() % 4;
        if r == clicked_r && clicked_r == zero_r {
            if clicked_c <= c && c < zero_c {
                c += 1
            } else if zero_c < c && c <= clicked_c {
                c -= 1;
            }
        } else if c == clicked_c && clicked_c == zero_c {
            if clicked_r <= r && r < zero_r {
                r += 1;
            } else if zero_r < r && r <= clicked_r {
                r -= 1;
            }
        }

        board.set_row_data(j, r * 4 + c)
    }

    board.set_row_data(0, clicked_r * 4 + clicked_c);
    ui.set_moves(ui.get_moves() + 1);
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let timer = Timer::default();

    let ui_ref_for_timer = ui.as_weak();
    timer.start(
        slint::TimerMode::Repeated,
        Duration::from_secs(1),
        move || {
            let ui = ui_ref_for_timer.upgrade().unwrap();
            let seconds = ui.get_seconds();
            ui.set_seconds(seconds + 1);
        },
    );

    let ui_ref = ui.as_weak();
    ui.on_click_item(move |index| {
        let i = index as usize;
        let ui = ui_ref.upgrade().unwrap();
        let board = ui.get_locations();
        let clicked_r = board.row_data(i).unwrap() / 4;
        let clicked_c = board.row_data(i).unwrap() % 4;
        make_step(ui_ref.clone(), clicked_r, clicked_c);
    });

    let ui_ref = ui.as_weak();
    ui.on_start_new_game(move || {
        let ui = ui_ref.upgrade().unwrap();
        let new_board = generate_random_board();
        ui.set_locations(new_board);
        ui.set_moves(0);
        ui.set_seconds(0);
    });

    let ui_ref = ui.as_weak();
    ui.on_key_pressed(move |key, with_shift| {
        let ui = ui_ref.upgrade().unwrap();
        let cmd = key.chars().next().unwrap();
        let zero_loc = ui.get_locations().row_data(0).unwrap();
        let zero_r = zero_loc / 4;
        let zero_c = zero_loc % 4;
        let mut clicked_r = zero_r;
        let mut clicked_c = zero_c;

        const UP: u32 = 63232;
        const DOWN: u32 = 63233;
        const LEFT: u32 = 63234;
        const RIGHT: u32 = 63235;

        println!("{} {}", cmd as u32, with_shift);

        match (cmd as u32, with_shift) {
            (UP, false) => {
                if zero_r < 3 {
                    clicked_r = zero_r + 1;
                }
            }
            (DOWN, false) => {
                if zero_r > 0 {
                    clicked_r = zero_r - 1;
                }
            }
            (LEFT, false) => {
                if zero_c < 3 {
                    clicked_c = zero_c + 1;
                }
            }
            (RIGHT, false) => {
                if zero_c > 0 {
                    clicked_c = zero_c - 1;
                }
            }
            (UP, true) => {
                if zero_r < 3 {
                    clicked_r = 3
                }
            }
            (DOWN, true) => {
                if zero_r > 0 {
                    clicked_r = 0;
                }
            }
            (LEFT, true) => {
                if zero_c < 3 {
                    clicked_c = 3;
                }
            }
            (RIGHT, true) => {
                if zero_c > 0 {
                    clicked_c = 0;
                }
            }
            _ => {
                return;
            }
        }

        make_step(ui_ref.clone(), clicked_r, clicked_c);
    });

    let new_board = generate_random_board();
    ui.set_locations(new_board);
    ui.run()
}
