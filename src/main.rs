mod model;
mod yolo;
use burn_ndarray::{NdArray, NdArrayDevice};

use burn::tensor::TensorData;
use burn::tensor::Tensor;
use image::GenericImageView;
use image::imageops::FilterType;
use model::best::Model;
use yolo::{decode::decode_yolov8, nms::nms};

type Backend = NdArray<f32>;

fn main() {
    let device = NdArrayDevice::Cpu;
    let model = Model::<Backend>::new(&device);
    let input = load_image("assets\\vehicle1.jpg", &device);
    let output = model.forward(input);
    println!("Output shape: {:?}", output.dims());
    let img_w = 640.0;
    let img_h = 640.0;
    let dets = decode_yolov8(output, 0.25, img_w, img_h);
    let dets = nms(dets, 0.45);

    println!("{:#?}", dets);
}
fn load_image(path: &str, device: &NdArrayDevice) -> Tensor<Backend, 4> {
    let img = image::open(path).unwrap().to_rgb8();
    let img = image::imageops::resize(&img, 640, 640, FilterType::Triangle);

    let mut data = Vec::with_capacity(1 * 3 * 640 * 640);

    for c in 0..3 {
        for y in 0..640 {
            for x in 0..640 {
                data.push(img.get_pixel(x, y)[c] as f32 / 255.0);
            }
        }
    }

    Tensor::from_data(TensorData::new(data, [1, 3, 640, 640]), device)
}
