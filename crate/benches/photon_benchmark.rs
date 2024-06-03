use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::{resize, SamplingFilter};
use std::time::Duration;
use photon_rs::PhotonImage;

fn criterion_benchmark(c: &mut Criterion) {
    let original =
        open_image("examples/input_images/underground.jpg").expect("File should open");

    c.bench_function("invert_image", |b| {
        let copy = original.clone();
        b.iter_batched(|| copy.clone(), |(mut img)| { invert_image(&mut img); }, BatchSize::SmallInput)
    });

    c.bench_function("resize_png", |b| b.iter(resize_png));

    c.bench_function("resize_jpg", |b| b.iter(resize_jpg));
}

fn invert_image(img: &mut PhotonImage) {
    // Invert the image
    photon_rs::channels::invert(img);
}

fn resize_png() {
    let img =
        open_image("examples/input_images/underground.png").expect("File should open");

    let resized_img = resize(&img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.png";

    save_image(resized_img, output_img_path).unwrap();
}

fn resize_jpg() {
    // Open the image (a PhotonImage is returned)
    let img =
        open_image("examples/input_images/underground.jpg").expect("File should open");

    let resized_img = resize(&img, 800, 600, SamplingFilter::Lanczos3);

    let output_img_path = "output.jpg";

    save_image(resized_img, output_img_path).unwrap();
}

fn alter_sample_size() -> Criterion {
    Criterion::default()
        .sample_size(10_usize)
        .measurement_time(Duration::from_secs(10_u64))
}

criterion_group! { name = benches; config = alter_sample_size(); targets = criterion_benchmark }
criterion_main!(benches);
