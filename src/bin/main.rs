use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use rtiow::{write_color, ColorRGB};

fn main() {
    let width = 512u64;
    let height = 512u64;

    let pb = ProgressBar::new(height);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:64}] {pos}/{len}")
            .progress_chars("=> "),
    );

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev().progress_with(pb) {
        // std::thread::sleep(std::time::Duration::from_millis(10));
        for i in 0..width {
            let pixel = ColorRGB::new(
                i as f64 / (width - 1) as f64,
                j as f64 / (height - 1) as f64,
                0.25,
            );
            write_color(&pixel);
        }
    }
}
