use num_traits::Unsigned;
use pixels::Pixels;
use pixels::wgpu::Color;

use element::Element;

mod element;

pub struct World {
    height: usize,
    width: usize,
    array: Vec<Element>,

    updated_indexes: Vec<usize>,
}


impl World {
    pub fn new(width: u32, height: u32) -> Self {
        let mut array = vec![];
        let size = (height * width);
        let size_f = size as f64;

        for i in 0..size as usize {
            array.push(Element {
                colour: Color {
                    r: 0.0,
                    g: 0.0,
                    b: ((size as usize - i) as f64) / size_f,
                    a: 1.0,
                }
            })
        }
        Self {
            height: height as usize,
            width: width as usize,
            array,
            updated_indexes: vec![],
        }
    }

    ///update pixels based on content, Dummy version
    pub fn show_all(&self, pixels: &mut Pixels) -> () {
        for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
            set_pixel(pixel, self.array[i].colour)
        }
    }

    pub fn show(&mut self, pixels: &mut Pixels) -> () {
        for updated_index in &self.updated_indexes {
            let pixel_index = updated_index * 4;
            let pixel = &mut pixels.frame_mut()[pixel_index..pixel_index + 4];
            set_pixel(pixel, self.array[*updated_index].colour)
        }
        self.updated_indexes.clear()
    }

    ///update selfs
    pub fn clicked(&mut self, x: usize, y: usize) -> () {
        self.change_element_at(x, y, |e, _| {
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

    fn index_at(&self, x: usize, y: usize) -> usize {
        (y * self.width + x)
    }
}

fn set_pixel(pixel: &mut [u8], color: Color)
{
    pixel[0] = (color.r * 255f64) as u8;
    pixel[1] = (color.g * 255f64) as u8;
    pixel[2] = (color.b * 255f64) as u8;
    pixel[3] = (color.a * 255f64) as u8;
}

