use crate::triangle::Vec3;
use std::fs;

/// Carga vértices y caras (índices base 0) de un archivo .obj.
pub fn load(path: &str) -> std::io::Result<(Vec<Vec3>, Vec<Vec<usize>>)> {
    let text = fs::read_to_string(path)?;
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in text.lines() {
        let mut parts = line.split_whitespace();
        match parts.next() {
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
                    faces.push(idx);
                }
            }
            _ => {}
        }
    }
    Ok((vertices, faces))
}
