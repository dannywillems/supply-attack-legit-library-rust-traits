use std::ops::Add;

struct S1 {
    pub x: u64,
    pub y: u64,
}

struct S2 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

impl Add for S1 {
    type Output = S2;

    fn add(self, rhs: Self) -> Self::Output {
        S2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.x + rhs.y,
        }
    }
}

// impl Add for S1 {
//     type Output = S1;

//     fn add(self, rhs: Self) -> Self::Output {
//         S1 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

fn main() {
    let s1 = S1 { x: 1, y: 2 };
    let s2 = S1 { x: 1, y: 2 };
    let s3 = s1 + s2;
    let exp_x = 2;
    let exp_y = 4;

    assert_eq!(exp_x, s3.x);
    assert_eq!(exp_y, s3.y);
}
