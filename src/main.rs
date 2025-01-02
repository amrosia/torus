use std::f32::consts::PI;
use std::io::{self, Write};
use std::{thread, time};
use std::process::Command;

fn main() {
    let mut a: f32 = 0.0;
    let mut b: f32 = 0.0;

    let height: usize = 24;
    let width: usize = 80;

    let clear_command = if cfg!(windows) { "cls" } else { "clear" };

    loop {
        let mut z = vec![0.0; height * width];
        let mut screen = vec![' '; height * width];

        let mut j: f32 = 0.0;
        while j < 2.0 * PI {
            j += 0.07;
            let mut i: f32 = 0.0;
            while i < 2.0 * PI {
                i += 0.02;

                let sin_a = a.sin();
                let cos_a = a.cos();
                let cos_b = b.cos();
                let sin_b = b.sin();

                let sin_i = i.sin();
                let cos_i = i.cos();
                let cos_j = j.cos();
                let sin_j = j.sin();

                let cos_j2 = cos_j + 2.0;
                let mess = 1.0 / (sin_i * cos_j2 * sin_a + sin_j * cos_a + 5.0);
                let t = sin_i * cos_j2 * cos_a - sin_j * sin_a;

                let x = (40.0 + 30.0 * mess * (cos_i * cos_j2 * cos_b - t * sin_b)) as isize;
                let y = (11.0 + 15.0 * mess * (cos_i * cos_j2 * sin_b + t * cos_b)) as isize;
                let o = x + width as isize * y;

                let n = ((8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b))
                    as isize).clamp(0, 28);

                if (0..height as isize).contains(&y)
                    && (0..width as isize).contains(&x)
                    && z[o as usize] < mess
                {
                    z[o as usize] = mess;
                    screen[o as usize] = ".,-~:;=!*#$@".chars().nth(n as usize).unwrap_or(' ');
                }
            }
        }

        // Clear the screen
        Command::new(clear_command)
            .status()
            .expect("Failed to clear the screen");

        for (index, char) in screen.iter().enumerate() {
            if index % width == 0 {
                println!();
            } else {
                print!("{}", char);
            }
        }

        io::stdout().flush().unwrap();

        // Increments
        a += 0.04;
        b += 0.02;

        // Sleep for a short duration to control the animation speed
        thread::sleep(time::Duration::from_millis(20));
    }
}
