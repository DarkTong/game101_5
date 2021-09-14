
use rand::Rng;

pub const M_PI:f32 = 3.14159265358979323846;

pub fn solve_quadratic(a:f32, b:f32, c:f32, x0:&mut f32, x1:&mut f32) -> bool{
    let discr = b*b-4.0*a*c; // 判别式
    if discr < 0.0 {
        return false;
    }
    else if discr == 0.0 {
        *x1 = -0.5 * b / a;
        *x0 = *x1;
    }
    else {
        let q = match b > 0. {
            true    => -0.5 * (b + discr.sqrt()),
            false   => -0.5 * (b - discr.sqrt())
        };
        *x0 = q / a;
        *x1 = c / q;
    }
    if x0 > x1 {
        let _t = *x0;
        *x0 = *x1;
        *x1 = _t;
    }

    return true;
}

pub fn get_random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0f32);
}

pub fn update_progress(progress: f32){
    const bar_width :usize = 70;
    let pos = (bar_width as f32 * progress) as usize;
    print!("[");
    for i in 0..bar_width {
        if i < pos {
            print!("=");
        }
        else if i == pos {
            print!(">");
        }
        else {
            print!(" ");
        }
    }
    println!("] {}%", (progress * 100.0) as i32);
}

#[cfg(test)]
mod tests {
    use crate::global::update_progress;

    #[test]
    fn test_update_progress() {
        update_progress(0.5);
    }
}
