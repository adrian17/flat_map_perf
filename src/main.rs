#![feature(test)]

extern crate test;

#[derive(Clone, Copy)]
pub struct Color(i32);
impl Color {
    #[inline(always)] pub fn blue(&self) -> u8 { (self.0 & 0xFF) as u8}
    #[inline(always)] pub fn green(&self) -> u8 { ((self.0 >> 8) & 0xFF) as u8}
    #[inline(always)] pub fn red(&self) -> u8 { ((self.0 >> 16) & 0xFF) as u8}
    #[inline(always)] pub fn alpha(&self) -> u8 { ((self.0 >> 24) & 0xFF) as u8}
}

pub struct BitmapData { pub pixels: Vec<Color> }

impl BitmapData {
    pub fn collect_loop(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for p in &self.pixels {
            output.extend_from_slice(&[p.red(), p.green(), p.blue(), p.alpha()])
        }
        output
    }
    pub fn collect_loop_with_prealloc(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.pixels.len()*4);
        for p in &self.pixels {
            output.extend_from_slice(&[p.red(), p.green(), p.blue(), p.alpha()])
        }
        output
    }
    pub fn collect_with_flat_map(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|p| [p.red(), p.green(), p.blue(), p.alpha()])
            .collect()
    }
    pub fn iteration_nested_loop(&self) -> u32 {
        let mut a: u32 = 0;
        for p in &self.pixels {
            for j in [p.red(), p.green(), p.blue(), p.alpha()] {
                a += j as u32;
            }
        }
        a
    }
    pub fn loop_flat_map(&self) -> u32 {
        let mut a: u32 = 0;
        for i in self.pixels.iter().flat_map(|p| [p.red(), p.green(), p.blue(), p.alpha()]) {
            a += i as u32;
        }
        a
    }
}

pub struct NestedData { pub pixels: Vec<Vec<u8>> }
impl NestedData {
    pub fn collect_loop(&self) -> Vec<u8> {
        let mut output = Vec::new();
        for p in &self.pixels {
            output.extend_from_slice(&p);
        }
        output
    }
    pub fn collect_loop_with_prealloc(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.pixels.len()*self.pixels[0].len());
        for p in &self.pixels {
            output.extend_from_slice(&p);
        }
        output
    }
    pub fn collect_with_flatten(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flatten()
            .copied()
            .collect()
    }
    pub fn iteration_nested_loop(&self) -> u32 {
        let mut a: u32 = 0;
        for p in &self.pixels {
            for j in p {
                a += *j as u32;
            }
        }
        a
    }
    pub fn iteration_flatten(&self) -> u32 {
        let mut a: u32 = 0;
        for i in self.pixels.iter().flatten() {
            a += *i as u32;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use crate::*;

    #[bench]
    fn bench_array_4x500000_collect_loop(b: &mut Bencher) {
        let data = BitmapData {pixels: vec![Color(777); 500000]};
        b.iter(|| { black_box(data.collect_loop()); });
    }
    #[bench]
    fn bench_array_4x500000_collect_loop_with_prealloc(b: &mut Bencher) {
        let data = BitmapData {pixels: vec![Color(777); 500000]};
        b.iter(|| { black_box(data.collect_loop_with_prealloc()); });
    }
    #[bench]
    fn bench_array_4x500000_collect_with_flat_map(b: &mut Bencher) {
        let data = BitmapData {pixels: vec![Color(777); 500000]};
        b.iter(|| { black_box(data.collect_with_flat_map()); });
    }

    #[bench]
    fn bench_array_4x500000_iteration_nested_loop(b: &mut Bencher) {
        let data = BitmapData {pixels: vec![Color(777); 500000]};
        b.iter(|| { black_box(data.iteration_nested_loop()); });
    }
    #[bench]
    fn bench_array_4x500000_iteration_flat_map(b: &mut Bencher) {
        let data = BitmapData {pixels: vec![Color(777); 500000]};
        b.iter(|| { black_box(data.loop_flat_map()); });
    }

    #[bench]
    fn bench_iter_4x500000_collect_loop(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4]; 500000]};
        b.iter(|| { black_box(data.collect_loop()); });
    }
    #[bench]
    fn bench_iter_4000x500_collect_loop(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4*1000]; 500]};
        b.iter(|| { black_box(data.collect_loop()); });
    }
    #[bench]
    fn bench_iter_4x500000_collect_loop_with_prealloc(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4]; 500000]};
        b.iter(|| { black_box(data.collect_loop_with_prealloc()); });
    }
    #[bench]
    fn bench_iter_4000x500_collect_loop_with_prealloc(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4*1000]; 500]};
        b.iter(|| { black_box(data.collect_loop_with_prealloc()); });
    }
    #[bench]
    fn bench_iter_4x500000_collect_with_flatten(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4]; 500000]};
        b.iter(|| { black_box(data.collect_with_flatten()); });
    }
    #[bench]
    fn bench_iter_4000x500_collect_with_flatten(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4*1000]; 500]};
        b.iter(|| { black_box(data.collect_with_flatten()); });
    }



    #[bench]
    fn bench_iter_4x500000_iteration_nested_loop(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4]; 500000]};
        b.iter(|| { black_box(data.iteration_nested_loop()); });
    }
    #[bench]
    fn bench_iter_4000x500_iteration_nested_loop(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4*1000]; 500]};
        b.iter(|| { black_box(data.iteration_nested_loop()); });
    }
    #[bench]
    fn bench_iter_4x500000_iteration_flatten(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4]; 500000]};
        b.iter(|| { black_box(data.iteration_flatten()); });
    }
    #[bench]
    fn bench_iter_4000x500_iteration_flatten(b: &mut Bencher) {
        let data = NestedData {pixels: vec![vec![77; 4*1000]; 500]};
        b.iter(|| { black_box(data.iteration_flatten()); });
    }
}

fn main() {}
