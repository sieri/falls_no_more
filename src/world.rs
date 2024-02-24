use pixels::Pixels;
use pixels::wgpu::Color;
use rand::Rng;

use element::Element;

use crate::world::element::ElementKind::{Empty, Sand};

mod element;

pub struct World {
    height: usize,
    width: usize,
    array: Vec<Element>,

    updated_indexes: Vec<usize>,
    priority: [usize; 2],
}


impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let element = Element {
            kind: Empty,
            colour: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        };

        let height = height as usize;
        let width = width as usize;
        let size = height * width;

        Self {
            height,
            width,
            array: vec![element; size],
            updated_indexes: vec![],
            priority: [0, 2],
        }
    }

    ///update pixels based on content, raw version, all pixels
    pub fn show_all(&self, pixels: &mut Pixels) -> () {
        for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            set_pixel(pixel, self.array[i].colour)
        }
    }

    ///update pixels with only those edited
    pub fn show(&mut self, pixels: &mut Pixels) -> () {
        for updated_index in &self.updated_indexes {
            let pixel_index = updated_index * 4;
            let pixel = &mut pixels.frame_mut()[pixel_index..pixel_index + 4];
            set_pixel(pixel, self.array[*updated_index].colour)
        }
        self.updated_indexes.clear()
    }

    ///make sand fall
    pub fn fall(&mut self)
    {
        for x in 0..self.width {
            for y in (0..self.height - 1).rev() {
                let i = self.index_at(x, y);
                if self.array[i].kind == Sand {
                    self.sand_element_fall(x, y);
                }
            }
        }
    }

    ///compute if a sand need to fall
    fn sand_element_fall(&mut self, x: usize, y: usize) {
        let i_bellow = self.index_at(x, y + 1);

        if self.array[i_bellow].kind == Empty
        {
            self.swap_elements_at((x, y), (x, y + 1));
            return;
        }

        let mut rng = rand::thread_rng();

        if rng.gen_bool(0.5) {
            let temp = self.priority[0];
            self.priority[0] = self.priority[1];
            self.priority[1] = temp;
        }

        for offset in self.priority {
            if x == 0 && offset == 0 {
                continue;
            }
            if x == self.width && offset == 2 {
                continue;
            }

            let offset_x = x + offset - 1;
            let i_next = self.index_at(offset_x, y);
            let i_next_bellow = self.index_at(offset_x, y + 1);

            if self.array[i_next].kind == Empty && self.array[i_next_bellow].kind == Empty
            {
                self.swap_elements_at((x, y), (offset_x, y + 1));
                return;
            }
        }
    }

    ///add sand to the point clicked
    pub fn clicked(&mut self, x: usize, y: usize) -> () {
        self.change_element_at(x, y, |e, _| {
            e.kind = Sand;
            e.colour = Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            };
        });
    }

    pub fn change_element_at<F>(&mut self, x: usize, y: usize, func: F) -> () where F: Fn(&mut Element, usize) -> ()
    {
        let i = self.index_at(x, y);
        let e = &mut self.array[i];
        func(e, i);
        self.updated_indexes.push(i);
    }

    pub fn swap_elements_at(&mut self, a: (usize, usize), b: (usize, usize)) -> ()
    {
        let i_a = self.index_at(a.0, a.1);
        let i_b = self.index_at(b.0, b.1);

        let temp = self.array[i_a];
        self.array[i_a] = self.array[i_b];
        self.array[i_b] = temp;

        self.updated_indexes.push(i_a);
        self.updated_indexes.push(i_b);
    }

    fn index_at(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

fn set_pixel(pixel: &mut [u8], color: Color)
{
    pixel[0] = (color.r * 255f64) as u8;
    pixel[1] = (color.g * 255f64) as u8;
    pixel[2] = (color.b * 255f64) as u8;
    pixel[3] = (color.a * 255f64) as u8;
}

