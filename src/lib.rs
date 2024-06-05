use image::DynamicImage;
use rand::Rng;

pub struct CropPayload {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct RGBColor {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

pub enum RotateDegrees {
    OneFouth = 90,
    Half = 180,
    ThreeQuarters = 270,
}

#[derive(PartialEq)]
enum GreenIntensity {
    Up = 1,
    Down = 0,
}

/*      F I L T E R S    */

pub fn blur(source: &DynamicImage, blur_amount: f32) -> DynamicImage {
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    return source.blur(blur_amount);
}

pub fn brighten(source: &DynamicImage, brightness_amount: i32) -> DynamicImage {
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers Downen it.  It returns a new image.
    return source.brighten(brightness_amount);
}

pub fn crop(source: &mut DynamicImage, payload: CropPayload) -> DynamicImage {
    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.
    return source.crop(payload.x, payload.y, payload.width, payload.height);
}

pub fn rotate(source: &DynamicImage, degree_option: RotateDegrees) -> DynamicImage {
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.
    let rotated_img: DynamicImage = match degree_option {
        RotateDegrees::OneFouth => source.rotate90(),
        RotateDegrees::Half => source.rotate180(),
        RotateDegrees::ThreeQuarters => source.rotate270(),
    };
    return rotated_img;
}

pub fn invert(source: &mut DynamicImage) {
    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.
    source.invert();
}

pub fn grayscale(source: &DynamicImage) -> DynamicImage {
    // .grayscale() takes no arguments. It returns a new image.
    return source.grayscale();
}

pub fn generate_color(rgb_color: RGBColor, outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 1000;
    let height = 1000;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        // Set the image to some solid color. -- see fractal() for an example
        // Challenge: parse some color data from the command-line, pass it through
        // to this function to use for the solid color.
        let red = rgb_color.red;
        let blue = rgb_color.blue;
        let green = rgb_color.green;

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

pub fn gradient(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example
    let width = 1000;
    let height = 1000;

    const MAX_RGB_INTENSITY: u8 = 255;
    const MIN_RGB_INTENSITY: u8 = 0;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    let mut temp_y = 0;
    let mut green_intensity = GreenIntensity::Up;
    let mut rng = rand::thread_rng();
    let red = rng.gen_range(0..255) as u8;
    let blue = rng.gen_range(0..255) as u8;
    let mut green = 0;

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    for (_, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Challenge 2: Generate something more interesting!

        if temp_y != y {
            temp_y = y;
            green = 0;
            green_intensity = GreenIntensity::Up;
        }

        if green == MAX_RGB_INTENSITY {
            green_intensity = GreenIntensity::Down;
        } 
        if green == MIN_RGB_INTENSITY {
            green_intensity = GreenIntensity::Up;
        }
        if green_intensity == GreenIntensity::Up && green < MAX_RGB_INTENSITY {
            green += 1;
        } 
        if green_intensity == GreenIntensity::Down && green > MIN_RGB_INTENSITY {
            green -= 1;
        }  

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
