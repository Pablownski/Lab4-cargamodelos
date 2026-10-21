use crate::framebuffer::Color;
use crate::triangle::Vec3;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct Face {
    pub indices: Vec<usize>,
    pub color: Color,
}

/// Lee los colores difusos (Kd) de un archivo .mtl.
fn load_mtl(path: &Path) -> HashMap<String, Color> {
    let mut mats = HashMap::new();
    let Ok(text) = fs::read_to_string(path) else { return mats };
    let mut name = String::new();
    for line in text.lines() {
        let mut p = line.split_whitespace();
        match p.next() {
            Some("newmtl") => name = p.next().unwrap_or("").to_string(),
            Some("Kd") => {
                let c: Vec<f32> = p.filter_map(|s| s.parse().ok()).collect();
                if c.len() == 3 {
                    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0) as u8;
                    mats.insert(name.clone(), Color::new(q(c[0]), q(c[1]), q(c[2])));
                }
            }
            _ => {}
        }
    }
    mats
}

/// Carga vértices y caras (índices base 0) de un archivo .obj.
pub fn load(path: &str) -> std::io::Result<(Vec<Vec3>, Vec<Face>)> {
    let text = fs::read_to_string(path)?;
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut materials = HashMap::new();
    let mut current = Color::new(255, 255, 255);

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("mtllib") => {
                if let Some(f) = parts.next() {
                    materials = load_mtl(&Path::new(path).with_file_name(f));
                }
            }
            Some("usemtl") => {
                if let Some(c) = parts.next().and_then(|n| materials.get(n)) {
                    current = *c;
                }
            }
            Some("v") => {
                let c: Vec<f32> = parts.take(3).filter_map(|s| s.parse().ok()).collect();
                if c.len() == 3 {
                    vertices.push(Vec3::new(c[0], c[1], c[2]));
                }
            }
            Some("f") => {
                // Formatos: v, v/vt, v//vn, v/vt/vn; solo importa el primer índice.
                let idx: Vec<usize> = parts
                    .filter_map(|s| s.split('/').next()?.parse::<i64>().ok())
                    .map(|i| if i < 0 { (vertices.len() as i64 + i) as usize } else { i as usize - 1 })
                    .collect();
                if idx.len() >= 3 {
                    faces.push(Face { indices: idx, color: current });
                }
            }
            _ => {}
        }
    }
    Ok((vertices, faces))
}
