use crate::geometry::Point;

/// Check if a point is inside the circumsphere of a tetrahedron
pub fn is_in_circumsphere(verts: [&Point; 4], p: &Point) -> bool {
    // Calculate the circumsphere center and radius using determinant method
    let mut mat = [[0.0; 5]; 5];
    for i in 0..4 {
        mat[i][0] = verts[i].x;
        mat[i][1] = verts[i].y;
        mat[i][2] = verts[i].z;
        mat[i][3] = verts[i].x * verts[i].x + verts[i].y * verts[i].y + verts[i].z * verts[i].z;
        mat[i][4] = 1.0;
    }
    mat[4][0] = p.x;
    mat[4][1] = p.y;
    mat[4][2] = p.z;
    mat[4][3] = p.x * p.x + p.y * p.y + p.z * p.z;
    mat[4][4] = 1.0;
    det5(&mat) > 0.0
}

/// Calculate the determinant of a 5x5 matrix
fn det5(m: &[[f64; 5]; 5]) -> f64 {
    let mut sum = 0.0;
    for i in 0..5 {
        let mut minor = [[0.0; 4]; 4];
        for j in 0..4 {
            for k in 0..4 {
                minor[j][k] = m[(j + 1) % 5][(k + if k >= i { 1 } else { 0 }) % 5];
            }
        }
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        sum += sign * m[0][i] * det4(&minor);
    }
    sum
}

/// Calculate the determinant of a 4x4 matrix
fn det4(m: &[[f64; 4]; 4]) -> f64 {
    let mut sum = 0.0;
    for i in 0..4 {
        let mut minor = [[0.0; 3]; 3];
        for j in 0..3 {
            for k in 0..3 {
                minor[j][k] = m[(j + 1) % 4][(k + if k >= i { 1 } else { 0 }) % 4];
            }
        }
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        sum += sign * m[0][i] * det3(&minor);
    }
    sum
}

/// Calculate the determinant of a 3x3 matrix
fn det3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}
