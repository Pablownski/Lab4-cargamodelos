mod framebuffer;
mod obj;
mod triangle;

use framebuffer::{Color, Framebuffer};
use minifb::{Key, Window, WindowOptions};
use triangle::Vec3;

const WIDTH: usize = 900;
const HEIGHT: usize = 700;

/// Rota (yaw alrededor de Y, luego pitch alrededor de X).
fn rotate(v: Vec3, yaw: f32, pitch: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    let x = v.x * cy + v.z * sy;
    let z = -v.x * sy + v.z * cy;
    let y = v.y * cp - z * sp;
    let z = v.y * sp + z * cp;
    Vec3::new(x, y, z)
}

/// Dibuja todos los triángulos del modelo, centrado y escalado a la pantalla.
fn render(fb: &mut Framebuffer, vertices: &[Vec3], faces: &[Vec<usize>], yaw: f32, pitch: f32) {
    let rotated: Vec<Vec3> = vertices.iter().map(|&v| rotate(v, yaw, pitch)).collect();

    // Centro y escala calculados de la caja envolvente del modelo rotado.
    let (mut min, mut max) = (rotated[0], rotated[0]);
    for v in &rotated {
        min = Vec3::new(min.x.min(v.x), min.y.min(v.y), min.z.min(v.z));
        max = Vec3::new(max.x.max(v.x), max.y.max(v.y), max.z.max(v.z));
    }
    let center = Vec3::new(
        (min.x + max.x) / 2.0,
        (min.y + max.y) / 2.0,
        (min.z + max.z) / 2.0,
    );
    let scale = 0.8 * (fb.width as f32 / (max.x - min.x)).min(fb.height as f32 / (max.y - min.y));

    let screen: Vec<Vec3> = rotated
        .iter()
        .map(|v| {
            Vec3::new(
                fb.width as f32 / 2.0 + (v.x - center.x) * scale,
                fb.height as f32 / 2.0 - (v.y - center.y) * scale,
                v.z - center.z,
            )
        })
        .collect();

    let light = Vec3::new(0.3, 0.6, -0.75).normalized();

    // Recorremos manualmente las caras y las triangulamos (abanico).
    for face in faces {
        for i in 1..face.len() - 1 {
            let (a, b, c) = (screen[face[0]], screen[face[i]], screen[face[i + 1]]);
            let (ra, rb, rc) = (rotated[face[0]], rotated[face[i]], rotated[face[i + 1]]);
            let normal = (rb - ra).cross(rc - ra).normalized();
            let intensity = (normal.dot(light).abs() * 0.8 + 0.2).clamp(0.0, 1.0);
            fb.set_color(Color::new(
                (255.0 * intensity) as u8,
                (255.0 * intensity) as u8,
                0,
            ));
            triangle::triangle(fb, a, b, c);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = args
        .get(1)
        .filter(|a| !a.starts_with("--"))
        .cloned()
        .unwrap_or_else(|| "hoverpiilder.obj".into());
    let (vertices, faces) = obj::load(&path).expect("no se pudo cargar el modelo");
    let (mut yaw, mut pitch) = (0.6_f32, -0.5_f32);

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);

    // Modo sin ventana: cargo run --release -- modelo.obj --save salida.png
    if let Some(i) = args.iter().position(|a| a == "--save") {
        fb.clear();
        render(&mut fb, &vertices, &faces, yaw, pitch);
        fb.save_png(&args[i + 1]).expect("no se pudo guardar la imagen");
        return;
    }

    let mut window = Window::new("Software Renderer", WIDTH, HEIGHT, WindowOptions::default())
        .expect("no se pudo crear la ventana");
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Left) {
            yaw -= 0.03;
        }
        if window.is_key_down(Key::Right) {
            yaw += 0.03;
        }
        if window.is_key_down(Key::Up) {
            pitch -= 0.03;
        }
        if window.is_key_down(Key::Down) {
            pitch += 0.03;
        }

        fb.clear();
        render(&mut fb, &vertices, &faces, yaw, pitch);
        window
            .update_with_buffer(&fb.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
