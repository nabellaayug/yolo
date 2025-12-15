# YOLOv8 Inference in Rust (Burn) — Python Training → ONNX → Rust

> **Status:** ✅ Working end‑to‑end (Python training → ONNX export → Rust inference)

---

## 📌 Project Overview

This project demonstrates **porting a YOLOv8 object detection model trained in Python** into a **pure Rust inference pipeline** using:

* **Ultralytics YOLOv8** (training & export)
* **ONNX** (model interchange)
* **Burn (Rust ML framework)** for inference
* **NdArray backend** (CPU)
* Custom **YOLOv8 decode + NMS** implemented manually in Rust

> ⚠️ Burn does **NOT** provide YOLO post‑processing. All decode & NMS logic is written manually.

---

## 🧠 Architecture Flow

```
Python (Train YOLOv8)
        ↓
Export to ONNX (opset >= 16)
        ↓
onnx2burn (model codegen)
        ↓
Rust Inference (Burn)
        ↓
YOLOv8 Decode + NMS
```

---

## 🐍 Python Side (Training & Export)

### 1️⃣ Install Dependencies

```bash
pip install ultralytics onnx onnxsim
```

---

### 2️⃣ Train YOLOv8

```python
from ultralytics import YOLO

model = YOLO("yolov8n.pt")
model.train(
    data="data.yaml",
    epochs=50,
    imgsz=640,
    batch=16,
)
```

Resulting weights:

```
runs/detect/train/weights/best.pt
```

---

### 3️⃣ Export to ONNX (IMPORTANT)

```python
model = YOLO("runs/detect/train/weights/best.pt")
model.export(
    format="onnx",
    opset=16,
    simplify=True,
    imgsz=640,
)
```

Output:

```
best.onnx
```

⚠️ **Notes:**

* Burn requires **ONNX opset >= 16**
* YOLOv8 output format:

```
[1, 4 + num_classes, 8400]
```

---

## 🦀 Rust Side (Burn Inference)

### 1️⃣ Convert ONNX → Burn Model

```bash
onnx2burn best.onnx --out-dir src/model
```

This generates:

```
src/model/best.rs
```

---

### 2️⃣ Dependencies (`Cargo.toml`)

```toml
[dependencies]
burn = "0.19"
burn-ndarray = "0.19"
image = "0.24"
```

⚠️ **IMPORTANT:**
`image` version **must match** `imageproc` dependency tree.

---

## 📦 YOLOv8 Output Format (CRITICAL)

YOLOv8 **DOES NOT HAVE objectness**.

Tensor shape:

```
[1, C, N]
C = 4 + num_classes
N = 8400 (for 640×640)
```

Channel layout:

```
0: cx
1: cy
2: w
3: h
4..C: class probabilities
```

All values are **sigmoid‑activated**.

---

## 🔓 YOLOv8 Decode (Rust)

Implemented manually:

```rust
pub fn decode_yolov8(
    output: Tensor<Backend, 3>,
    conf_thresh: f32,
    img_w: f32,
    img_h: f32,
) -> Vec<Detection>
```

Steps:

1. Convert tensor → `Vec<f32>`
2. Loop over `num_boxes`
3. Apply `sigmoid`
4. Pick best class score
5. Convert `cx,cy,w,h → x1,y1,x2,y2`

---

## ✂️ Non‑Maximum Suppression (NMS)

* Class‑aware NMS
* IoU thresholding
* Implemented manually in Rust

---

---

## ❗ Common Pitfalls (Encountered)

| Issue                     | Cause                   | Fix                      |
| ------------------------- | ----------------------- | ------------------------ |
| `AlignmentMismatch`       | Wrong TensorData layout | Use `Vec<f32>` only      |
| All bbox = 0              | Wrong decode logic      | YOLOv8 has NO objectness |
| Too many boxes            | NMS not applied         | Apply class‑aware NMS    |
| `GenericImageView` error  | image version mismatch  | Align crate versions     |
| Panic index out of bounds | Wrong `num_boxes` loop  | Use tensor dims          |

---

## 📚 References

* Ultralytics YOLOv8
  [https://github.com/ultralytics/ultralytics](https://github.com/ultralytics/ultralytics)

* YOLOv8 Export Docs
  [https://docs.ultralytics.com/modes/export/#onnx](https://docs.ultralytics.com/modes/export/#onnx)

* Burn Framework
  [https://github.com/tracel-ai/burn](https://github.com/tracel-ai/burn)

* Burn ONNX Importer
  [https://burn.dev/books/burn/import/onnx-model.html](https://burn.dev/books/burn/import/onnx-model.html)

* YOLOv8 Decode Reference
  [https://github.com/ultralytics/ultralytics/blob/main/ultralytics/models/yolo/detect/predict.py](https://github.com/ultralytics/ultralytics/blob/main/ultralytics/models/yolo/detect/predict.py)

* Imageproc
  [https://github.com/image-rs/imageproc](https://github.com/image-rs/imageproc)


