// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur assets/pens.png assets/pens_blurred.png 2.5
//

use photo_filters::{
    blur,
    brighten,
    crop,
    rotate,
    invert,
    grayscale,
    generate_color,
    gradient,
    fractal,
    CropPayload,
    RGBColor,
    RotateDegrees
};

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let blur_amount: f32 = args.remove(0).parse().unwrap();
            // Here's how you open an existing image file
            let source_img = image::open(infile).expect("Failed to open INFILE.");
            blur(&source_img, blur_amount)
                .save(outfile)
                .expect("Failed writing OUTFILE.");
        }

        // **OPTION**
        // Brighten -- see the brighten() function below
        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let brightness_amount: i32 = args.remove(0).parse().unwrap();
            // Here's how you open an existing image file
            let source_img = image::open(infile).expect("Failed to open INFILE.");
            brighten(&source_img, brightness_amount)
                .save(outfile)
                .expect("Failed writing OUTFILE.");
        }

        // **OPTION**
        // Crop -- see the crop() function below
        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let crop_payload = CropPayload {
                x: args.remove(0).parse().unwrap(),
                y: args.remove(0).parse().unwrap(),
                width: args.remove(0).parse().unwrap(),
                height: args.remove(0).parse().unwrap(),
            };
            let mut source_img = image::open(infile).expect("Failed to open INFILE.");
            crop(&mut source_img, crop_payload)
                .save(outfile)
                .expect("Failed writing OUTFILE.");
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            let degree_input: i32 = args.remove(0).parse().unwrap();
            let degree_option = if degree_input == 180 {
                RotateDegrees::Half
            } else if degree_input == 270 {
                RotateDegrees::ThreeQuarters
            } else {
                RotateDegrees::OneFouth
            };

            let source_img = image::open(infile).expect("Failed to open INFILE.");
            rotate(&source_img, degree_option)
                .save(outfile)
                .expect("Failed writing OUTFILE.");
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // Here's how you open an existing image file
            let mut source_img = image::open(infile).expect("Failed to open INFILE.");
            invert(&mut source_img);
            source_img.save(outfile).expect("Failed writing OUTFILE.");
        }

        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }
            let infile = args.remove(0);
            let outfile = args.remove(0);
            // Here's how you open an existing image file
            let source_img = image::open(infile).expect("Failed to open INFILE.");
            grayscale(&source_img)
                .save(outfile)
                .expect("Failed writing OUTFILE.");
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate Color -- see the generate_color() function below -- this should be sort of like "fractal()"!
        "generate_color" => {
            if args.len() != 4 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            let rgb_color = RGBColor {
                red: args.remove(0).parse().unwrap(),
                blue: args.remove(0).parse().unwrap(),
                green: args.remove(0).parse().unwrap()
            };
            generate_color(rgb_color, outfile);
        }

        "gradient" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            gradient(outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("~~~PHOTO FILTERS~~~");
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}
