use supply_attack_rust_traits_fake_lib::S1;

fn main() {
    let s1 = S1 { x: 1, y: 2 };
    let s2 = S1 { x: 1, y: 2 };

    // Checking s3 and exp_output are the same.
    let s3 = s1 + s2;
    let exp_output = S1 { x: 2, y: 4 };

    assert_eq!(exp_output.x, s3.x);
    assert_eq!(exp_output.y, s3.y);
}
