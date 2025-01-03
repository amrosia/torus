use std::f32::consts::PI; // const PI for math
use std::io::{self, Write}; // for printing the donut
use std::{thread, time}; // To control animation speed
use std::process::Command; // To clear the screen between frames
use clap::Parser;
fn main() {
    // Rotation angle base values (to change rotation, change increment angles instead of these)
    let mut a: f32 = 0.0; // X-axis rotation
    let mut b: f32 = 0.0; // Z-axis rotation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
    struct Cli {
    /// The distance from the center to the donut
    #[arg(long = "major", default_value = "2.0")]
        major_r: f32,
    /// The size of the donut
    #[arg(long = "minor", default_value = "1.0")]
        minor_r: f32,
    /// The distance from the camera to the donut
    #[arg(short = 'd', long = "dist", default_value = "5.0")]
        cam_distance: f32,
    /// The height of the terminal
    #[arg(long = "height", default_value = "24")]
        height: usize,
    /// The width of the terminal
    #[arg(long = "width", default_value = "80")]
        width: usize,
    /// The delay between each frame
    #[arg(long = "delay", default_value = "20")]
        delay: u64,
    /// The angle of the donut ring (in degrees)
    #[arg(long = "ring", default_value = "360.0")]
        donut_ring_angle: f32,
    /// The angle of the donut tube (in degrees)
    #[arg(long = "tube", default_value = "360.0")]
        donut_tube_angle: f32,
    /// X-axis increment angle
    #[arg(long = "x-inc", default_value = "0.04")]
        x_inc: f32,
    /// Z-axis increment angle
    #[arg(long = "z-inc", default_value = "0.02")]
        z_inc: f32,

    /// X-axis offset (horizontal position of the donut)
    #[arg(long = "offset-x")]
    offset_x: Option<f32>,

    /// Y-axis offset (vertical position of the donut)
    #[arg(long = "offset-y")]
    offset_y: Option<f32>,
    }

    let cli = Cli::parse();

    // Calculate default offsets for the terminal if not provided
    let offset_x = cli.offset_x.unwrap_or((cli.width as f32) / 2.0);
    let offset_y = cli.offset_y.unwrap_or((cli.height as f32) / 2.0);


    // Choose the command for clearing the terminal based on OS
    let clear_command = if cfg!(windows) { "cls" } else { "clear" };

    // Infinite loop to draw new frames
    loop {
        // z - depth buffer
        // screen - character buffer
        // 'z' will store the depth values to determine if a pixel is "in front"
        // 'screen' will store the ASCII character to display at each pixel
        let mut z = vec![0.0; cli.height * cli.width];
        let mut screen = vec![' '; cli.height * cli.width];
// i - the circle
// j - the donut ring, spins the circle that forms the donut
        let mut j: f32 = 0.0;
        // Outer loop angle j goes from 0 to 2 pi (with small steps)
        while j < (cli.donut_ring_angle / 180.0) * PI {
            j += 0.07;

            // Inner loop angle i goes from 0 to 2 pi (with smaller steps)
            let mut i: f32 = 0.0;
            while i < (cli.donut_tube_angle / 180.0) * PI {
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
                
                // Offset by 2.0 units to represent the donut's major radius
                let cos_j2 = cli.major_r + cli.minor_r * cos_j;
                // Project 3D donut onto 2D screen
                
                
                // Calculate the "inverted distance" for perpective projection
                // see: <https://www.a1k0n.net/img/perspective.pnghttps://www.a1k0n.net/img/perspective.png>
                
                // camera distance is an offset that pushed the donut in front of the camera

                // (sin_i * cos_j2 * sin_a + sin_j * cos_a) rotates the donut in 3D
                let mess = 1.0 / (cli.minor_r * sin_i * cos_j2 * sin_a + sin_j * cos_a + cli.cam_distance);

                // Value for rotations on the X-axis
                let t = sin_i * cos_j2 * cos_a - sin_j * sin_a;

                // Project the 3D coordinates to 2D screen coordinates
                // offset_x and offset_y center the donut on the screen
                // 30.0 and 15.0 scale the donut to fill the width/height
                let x = (offset_x + 30.0 * mess * (cos_i * cos_j2 * cos_b - t * sin_b)) as isize;
                let y = (offset_y + 15.0 * mess * (cos_i * cos_j2 * sin_b + t * cos_b)) as isize;

                // Convert 2D x,y into a single index for the 1D buffers
                let o = x + cli.width as isize * y;

                // n picks which character to choose from the ASCII charset based on brightness
                let n = ((8.0
                    * ((sin_j * sin_a - sin_i * cos_j * cos_a) * cos_b
                        - sin_i * cos_j * sin_a
                        - sin_j * cos_a
                        - cos_i * cos_j * sin_b))
                    as isize).clamp(0, 11); // clamp ensures that each part of the donut receives a
                                            // character even if it's brightess is out of bounds

                // Check if (x, y) is within the screen bounds
                if (0..cli.height as isize).contains(&y)
                    && (0..cli.width as isize).contains(&x)
                    && z[o as usize] < mess
                {
                    // Update the depth buffer
                    z[o as usize] = mess;
                    // Choose a character from the string based on 'n'
                    screen[o as usize] = ".,-~:;=!*#$@".chars().nth(n as usize).unwrap_or(' ');
                }
            }
        }

        // Clear the screen for the next frame
        Command::new(clear_command)
            .status()
            .expect("Failed to clear the screen");

        // Prints out the 'screen' buffer, displaying the donut
        for (index, char) in screen.iter().enumerate() {
            if index % cli.width == 0 {
                println!();
            } else {
                print!("{}", char);
            }
        }
        // Flush stdout to ensure everything is printed immediately
        io::stdout().flush().unwrap();

        // Increment angles for the next frame, causing rotation
        a += cli.x_inc; // rotate X-axis a bit more
        b += cli.z_inc; // rotate Z-axis a bit more

        // Delay between each frame, controls the animation speed  
        thread::sleep(time::Duration::from_millis(cli.delay));
    }
}
