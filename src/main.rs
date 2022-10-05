use image::{ImageBuffer, Rgba};
use std::io::{stdin, Read};
use rand::Rng;

struct State {
    data: Vec<bool>,
    size: (usize, usize),
}

impl Clone for State {
    fn clone(&self) -> Self {
        Self{ data: self.data.to_vec(),
        size: self.size}
    }
}

impl State {
    pub fn new(data: Vec<bool>, size: (usize, usize)) -> Self {
        Self { data, size }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        let i = self.size.0 * x;
        self.data[i + y]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut bool {
        let i = self.size.0 * x;
        &mut self.data[i + y]
    }

    pub fn create_empty(n: usize, m: usize) -> Self {
        let v = vec![false; n * m];
        Self { data: v, size: (n, m) }
    }

    pub fn neigh_count(&self, ix: usize, iy: usize) -> i32 {
        let mut count = 0;
        let x = ix; let y = iy;

        if y >= 1 {
            if x >= 1 && self.get_cell(x-1, y-1) {count += 1};
            if self.get_cell(x, y-1) {count += 1};
            if x+1 < self.size.1 && self.get_cell(x+1, y-1) {count += 1};
        };

            if x >= 1 && self.get_cell(x-1, y) {count += 1};
            if x+1 < self.size.1 && self.get_cell(x+1, y) {count += 1};

        if y+1 < self.size.0 {
            if x >= 1 && self.get_cell(x-1, y+1) {count += 1};
            if self.get_cell(x, y+1) {count += 1};
            if x+1 < self.size.1 && self.get_cell(x+1, y+1) {count += 1};
        };

        count
    }

    pub fn draw(&self, name: &str) {
        let filename = format!("{}.png", name);
        let x: u32 = self.size.0.try_into().unwrap();
        let y: u32 = self.size.1.try_into().unwrap();

        let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(x, y, Rgba([0, 0, 0, 30]));

        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if self.get_cell(i, j) {
                    let pos_y: u32 = i.try_into().unwrap();
                    let pos_x: u32 = j.try_into().unwrap();

                    buffer.get_pixel_mut(pos_x, pos_y).0 = [100, 255, 100, 255];
                }
            }
        }

        let factor: f32;
        if x >= y {
            factor = 300.0 / x as f32;
        } else {
            factor = 300.0 / y as f32;
        };

        let x_resize: u32 = ( x as f32 * factor).floor() as u32;
        let y_resize: u32 = ( y as f32 * factor).floor() as u32;
        let resize = image::imageops::resize(&buffer, x_resize, y_resize, image::imageops::FilterType::Nearest);

        resize.save(filename).unwrap();
    }

    pub fn next_tick(self) -> State {
        let mut new_state = State::create_empty(self.size.0, self.size.1);
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                let neigh = self.neigh_count(i, j);
                let cell = self.get_cell(i, j);
                if cell {
                    if neigh == 2 || neigh == 3 {
                        *new_state.get_cell_mut(i, j) = true;
                    }
                } else {
                    if neigh == 3 {
                        *new_state.get_cell_mut(i, j) = true;
                    }
                }
            }
        };
        if new_state.data == self.data {
            new_state.randomize(3);
        }

        if new_state.data.iter().filter(|&n| *n == true).count() < 3 {
            new_state.randomize(5);
        }

        let mut rng = rand::thread_rng();
        if rng.gen_range(1..6) == 1 {
            new_state.randomize(1);
        }

        new_state
    }

    pub fn randomize(&mut self, num: u8) {
        let mut new_matrix = self.data.clone();
        let mut rng = rand::thread_rng();
        for _ in 0..num {
            let i = rng.gen_range(0..(self.size.0 * self.size.1));
            new_matrix[i] = !new_matrix[i];
        }
        self.data = new_matrix;
    }
}

fn generate(x:usize, y:usize, data: Vec<bool>, name: &str) -> State {
    let state = State::new(data, (x, y));
    state.draw(&name);
    state
}

fn main() {
    //let data = vec![false; 36];
    let data = vec![false, false, false, false, false, false,
                                false, false, true, true, false, false,
                                false, false, true, true, false, false,
                                false, false, false, false, false, false,
                                false, false, false, false, false, false,
                                false, false, false, false, false, false,];
    let name = "test";
    let mut state = generate(6, 6, data, name);
    state.draw(name);
    loop {
        println!("Press enter");
        stdin().read(&mut [0, 0]).unwrap();
        state = state.next_tick();
        state.draw(name);
    }
}