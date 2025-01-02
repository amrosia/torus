use std::f32::consts::PI; // const PI for math
use std::io::{self, Write}; // for printing the donut
use std::{thread, time}; // To control animation speed
use std::process::Command; // To clear the screen between frames

fn main() {
    // Rotation angles
    let mut a: f32 = 0.0; // X axis rotation
    let mut b: f32 = 0.0; // Z axis rotation

    // Terminal size
    let height: usize = 24;
    let width: usize = 80;

    // Choose the command for clearing the terminal based on OS
    let clear_command = if cfg!(windows) { "cls" } else { "clear" };

    // Infinite loop to draw new frames
    loop {
        // z - depth buffer
        // screen - character buffer
        // 'z' will store the depth values to determine if a pixel is "in front"
        // 'screen' will store the ASCII character to display at each pixel
        let mut z = vec![0.0; height * width];
        let mut screen = vec![' '; height * width];
// i - the circle
// j - the donut ring, spins the circle that forms the donut
        let mut j: f32 = 0.0;
        // Outer loop angle j goes from 0 to 2 pi (with small steps)
        while j < 2.0 * PI {
            j += 0.07;

            // Inner loop angle i goes from 0 to 2 pi (with smaller steps)
            let mut i: f32 = 0.0;
            while i < 2.0 * PI {
                i += 0.02;


                // Precompute sines and cosines for angles a, b 
                let sin_a = a.sin();
                let cos_a = a.cos();
                let cos_b = b.cos();
                let sin_b = b.sin();
                
                // Precompute sines and cosines for angles i, j 
                let sin_i = i.sin();
                let cos_i = i.cos();
                let cos_j = j.cos();
                let sin_j = j.sin();
                
                // Shift the circle by +2.0, adding radius from the center. 
                let cos_j2 = cos_j + 2.0; 

                // Project 3D donut onto 2D screen
                
                
                // Calculate the "inverted distance" for perpective projection
                // see: <https://www.a1k0n.net/img/perspective.pnghttps://www.a1k0n.net/img/perspective.png>
                
                // 5.0 is an offset tha pushed the donut in front of the camera
                // (sin_i * cos_j2 * sin_a + sin_j * cos_a)
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

        // Clear the screen to make space for next frame
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

        // Delay between each frame, controls the animation speed  
        thread::sleep(time::Duration::from_millis(20));
    }
}
