#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | self.b as u32
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background: Color,
    current: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background: Color::new(20, 20, 30),
            current: Color::new(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(self.background.to_u32());
        self.zbuffer.fill(f32::INFINITY);
    }

    pub fn set_color(&mut self, color: Color) {
        self.current = color;
    }

    /// Pinta un píxel con el color actual si está más cerca que lo ya dibujado.
    pub fn point(&mut self, x: usize, y: usize, z: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = y * self.width + x;
        if z < self.zbuffer[i] {
            self.zbuffer[i] = z;
            self.buffer[i] = self.current.to_u32();
        }
    }

    pub fn save_png(&self, path: &str) -> image::ImageResult<()> {
        let mut img = image::RgbImage::new(self.width as u32, self.height as u32);
        for (i, px) in self.buffer.iter().enumerate() {
            img.put_pixel(
                (i % self.width) as u32,
                (i / self.width) as u32,
                image::Rgb([(px >> 16) as u8, (px >> 8) as u8, *px as u8]),
            );
        }
        img.save(path)
    }
}
