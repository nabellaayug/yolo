use burn::tensor::Tensor;
use burn_ndarray::NdArray;

use super::Detection;

type Backend = NdArray<f32>;

#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

pub fn decode_yolov8(
    output: Tensor<Backend, 3>,
    conf_thresh: f32,
    img_w: f32,
    img_h: f32,
) -> Vec<Detection> {
    let data: Vec<f32> = output.clone()
        .into_data()
        .to_vec::<f32>()
        .expect("tensor to vec failed");

    let dims = output.dims(); // [1, C, N]
    let channels = dims[1];
    let num_boxes = dims[2];
    let num_classes = channels - 4;

    let mut detections = Vec::new();

    for i in 0..num_boxes {
        let cx = sigmoid(data[0 * num_boxes + i]);
        let cy = sigmoid(data[1 * num_boxes + i]);
        let w  = sigmoid(data[2 * num_boxes + i]);
        let h  = sigmoid(data[3 * num_boxes + i]);

        let mut best_score = 0.0;
        let mut best_class = 0;

        for c in 0..num_classes {
            let score = sigmoid(data[(4 + c) * num_boxes + i]);
            if score > best_score {
                best_score = score;
                best_class = c;
            }
        }

        if best_score < conf_thresh {
            continue;
        }

        let x1 = (cx - w / 2.0) * img_w;
        let y1 = (cy - h / 2.0) * img_h;
        let x2 = (cx + w / 2.0) * img_w;
        let y2 = (cy + h / 2.0) * img_h;

        if x2 <= x1 || y2 <= y1 {
            continue;
        }

        detections.push(Detection {
            x1,
            y1,
            x2,
            y2,
            score: best_score,
            class_id: best_class,
        });
    }

    detections
}
