use palette::{Hsl, IntoColor, Srgb};
use pixels::Pixels;
use pixels::wgpu::Color;
use rand::Rng;

use element::Element;

use crate::world::element::ElementKind::{Empty, Sand};

mod element;

const SATURATION: f32 = 0.5;
const LIGHTNESS: f32 = 0.5;
const HUE_OFFSET: f32 = 1.0;

pub struct World {
    height: usize,
    width: usize,
    array: Vec<Element>,
    next_array: Vec<Element>,

    updated_indexes_internal: Vec<usize>,
    updated_indexes: Vec<usize>,
    priority: [usize; 2],
    colour: Hsl,
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
            next_array: vec![element; size],
            updated_indexes_internal: vec![],
            updated_indexes: vec![],
            priority: [0, 2],
            colour: Hsl::new(
                0., SATURATION, LIGHTNESS,
            ),
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
        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                let i = self.index_at(x, y);
                if self.array[i].kind == Sand {
                    self.sand_element_fall(x, y);
                }
            }

            self.update_buffer();
        }
    }

    fn update_buffer(&mut self) {
        for updated_index in &self.updated_indexes_internal {
            let i = *updated_index;
            self.updated_indexes.push(i);
            self.array[i] = self.next_array[i];
        }
        self.updated_indexes_internal.clear()
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
            if x >= self.width - 1 && offset == 2 {
                continue;
            }
            if y >= self.height - 1 {
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
        let color: Srgb = self.colour.into_color();
        self.colour.hue += HUE_OFFSET;
        self.change_element_at(x, y, |e, _| {
            if e.kind == Sand {
                return;
            }

            e.kind = Sand;
            e.colour = Color {
                r: color.red as f64,
                g: color.green as f64,
                b: color.blue as f64,
                a: 1.0,
            };
        });
    }

    pub fn change_element_at<F>(&mut self, x: usize, y: usize, func: F) -> () where F: Fn(&mut Element, usize) -> ()
    {
        let i = self.index_at(x, y);
        let mut e = self.array[i].clone();
        func(&mut e, i);
        self.next_array[i] = e;
        self.updated_indexes_internal.push(i);
    }

    pub fn swap_elements_at(&mut self, a: (usize, usize), b: (usize, usize)) -> ()
    {
        let i_a = self.index_at(a.0, a.1);
        let i_b = self.index_at(b.0, b.1);

        self.next_array[i_a] = self.array[i_b];
        self.next_array[i_b] = self.array[i_a];

        self.updated_indexes_internal.push(i_a);
        self.updated_indexes_internal.push(i_b);
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

