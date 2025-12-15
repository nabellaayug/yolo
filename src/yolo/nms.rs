use super::Detection;

pub fn iou(a: &Detection, b: &Detection) -> f32 {
    let x1 = a.x1.max(b.x1);
    let y1 = a.y1.max(b.y1);
    let x2 = a.x2.min(b.x2);
    let y2 = a.y2.min(b.y2);

    let inter = (x2 - x1).max(0.0) * (y2 - y1).max(0.0);
    let area_a = (a.x2 - a.x1) * (a.y2 - a.y1);
    let area_b = (b.x2 - b.x1) * (b.y2 - b.y1);

    inter / (area_a + area_b - inter + 1e-6)
}

pub fn nms(
    mut dets: Vec<Detection>,
    iou_thresh: f32,
) -> Vec<Detection> {
    dets.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let mut keep = Vec::new();

    while let Some(det) = dets.pop() {
        if keep.iter().all(|k: &Detection| {
    k.class_id != det.class_id || iou(k, &det) < iou_thresh
}) {
    keep.push(det);
}
    }

    keep
}
