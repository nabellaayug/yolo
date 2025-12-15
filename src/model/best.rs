// Generated from ONNX "onnx\\best.onnx" by burn-import
use burn::prelude::*;
use burn::nn::PaddingConfig2d;
use burn::nn::conv::Conv2d;
use burn::nn::conv::Conv2dConfig;
use burn::nn::interpolate::Interpolate2d;
use burn::nn::interpolate::Interpolate2dConfig;
use burn::nn::interpolate::InterpolateMode;
use burn::nn::pool::MaxPool2d;
use burn::nn::pool::MaxPool2dConfig;
use burn::record::FullPrecisionSettings;
use burn::record::Recorder;


#[derive(Module, Debug)]
pub struct Model<B: Backend> {
    conv2d1: Conv2d<B>,
    conv2d2: Conv2d<B>,
    conv2d3: Conv2d<B>,
    conv2d4: Conv2d<B>,
    conv2d5: Conv2d<B>,
    conv2d6: Conv2d<B>,
    conv2d7: Conv2d<B>,
    conv2d8: Conv2d<B>,
    conv2d9: Conv2d<B>,
    conv2d10: Conv2d<B>,
    conv2d11: Conv2d<B>,
    conv2d12: Conv2d<B>,
    conv2d13: Conv2d<B>,
    conv2d14: Conv2d<B>,
    conv2d15: Conv2d<B>,
    conv2d16: Conv2d<B>,
    conv2d17: Conv2d<B>,
    conv2d18: Conv2d<B>,
    conv2d19: Conv2d<B>,
    conv2d20: Conv2d<B>,
    conv2d21: Conv2d<B>,
    conv2d22: Conv2d<B>,
    conv2d23: Conv2d<B>,
    conv2d24: Conv2d<B>,
    conv2d25: Conv2d<B>,
    conv2d26: Conv2d<B>,
    maxpool2d1: MaxPool2d,
    maxpool2d2: MaxPool2d,
    maxpool2d3: MaxPool2d,
    conv2d27: Conv2d<B>,
    resize1: Interpolate2d,
    conv2d28: Conv2d<B>,
    conv2d29: Conv2d<B>,
    conv2d30: Conv2d<B>,
    conv2d31: Conv2d<B>,
    resize2: Interpolate2d,
    conv2d32: Conv2d<B>,
    conv2d33: Conv2d<B>,
    conv2d34: Conv2d<B>,
    conv2d35: Conv2d<B>,
    conv2d36: Conv2d<B>,
    conv2d37: Conv2d<B>,
    conv2d38: Conv2d<B>,
    conv2d39: Conv2d<B>,
    conv2d40: Conv2d<B>,
    conv2d41: Conv2d<B>,
    conv2d42: Conv2d<B>,
    conv2d43: Conv2d<B>,
    conv2d44: Conv2d<B>,
    conv2d45: Conv2d<B>,
    conv2d46: Conv2d<B>,
    conv2d47: Conv2d<B>,
    conv2d48: Conv2d<B>,
    conv2d49: Conv2d<B>,
    conv2d50: Conv2d<B>,
    conv2d51: Conv2d<B>,
    conv2d52: Conv2d<B>,
    conv2d53: Conv2d<B>,
    conv2d54: Conv2d<B>,
    conv2d55: Conv2d<B>,
    conv2d56: Conv2d<B>,
    conv2d57: Conv2d<B>,
    conv2d58: Conv2d<B>,
    conv2d59: Conv2d<B>,
    conv2d60: Conv2d<B>,
    conv2d61: Conv2d<B>,
    conv2d62: Conv2d<B>,
    conv2d63: Conv2d<B>,
    conv2d64: Conv2d<B>,
    constant1: burn::module::Param<Tensor<B, 3>>,
    constant2: burn::module::Param<Tensor<B, 3>>,
    constant4: burn::module::Param<Tensor<B, 2>>,
    phantom: core::marker::PhantomData<B>,
    device: burn::module::Ignored<B::Device>,
}


impl<B: Backend> Default for Model<B> {
    fn default() -> Self {
        Self::from_file("--out-dir\\best", &Default::default())
    }
}

impl<B: Backend> Model<B> {
    pub fn from_file(file: &str, device: &B::Device) -> Self {
        let record = burn::record::PrettyJsonFileRecorder::<FullPrecisionSettings>::new()
            .load(file.into(), device)
            .expect("Record file to exist.");
        Self::new(device).load_record(record)
    }
}

impl<B: Backend> Model<B> {
    #[allow(unused_variables)]
    pub fn new(device: &B::Device) -> Self {
        let conv2d1 = Conv2dConfig::new([3, 16], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d2 = Conv2dConfig::new([16, 32], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d3 = Conv2dConfig::new([32, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d4 = Conv2dConfig::new([16, 16], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d5 = Conv2dConfig::new([16, 16], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d6 = Conv2dConfig::new([48, 32], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d7 = Conv2dConfig::new([32, 64], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d8 = Conv2dConfig::new([64, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d9 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d10 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d11 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d12 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d13 = Conv2dConfig::new([128, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d14 = Conv2dConfig::new([64, 128], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d15 = Conv2dConfig::new([128, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d16 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d17 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d18 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d19 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d20 = Conv2dConfig::new([256, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d21 = Conv2dConfig::new([128, 256], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d22 = Conv2dConfig::new([256, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d23 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d24 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d25 = Conv2dConfig::new([384, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d26 = Conv2dConfig::new([256, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let maxpool2d1 = MaxPool2dConfig::new([5, 5])
            .with_strides([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2))
            .with_dilation([1, 1])
            .init();
        let maxpool2d2 = MaxPool2dConfig::new([5, 5])
            .with_strides([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2))
            .with_dilation([1, 1])
            .init();
        let maxpool2d3 = MaxPool2dConfig::new([5, 5])
            .with_strides([1, 1])
            .with_padding(PaddingConfig2d::Explicit(2, 2))
            .with_dilation([1, 1])
            .init();
        let conv2d27 = Conv2dConfig::new([512, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let resize1 = Interpolate2dConfig::new()
            .with_output_size(None)
            .with_scale_factor(Some([2.0, 2.0]))
            .with_mode(InterpolateMode::Nearest)
            .init();
        let conv2d28 = Conv2dConfig::new([384, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d29 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d30 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d31 = Conv2dConfig::new([192, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let resize2 = Interpolate2dConfig::new()
            .with_output_size(None)
            .with_scale_factor(Some([2.0, 2.0]))
            .with_mode(InterpolateMode::Nearest)
            .init();
        let conv2d32 = Conv2dConfig::new([192, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d33 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d34 = Conv2dConfig::new([32, 32], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d35 = Conv2dConfig::new([96, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d36 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d37 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d38 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d39 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d40 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d41 = Conv2dConfig::new([192, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d42 = Conv2dConfig::new([64, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d43 = Conv2dConfig::new([64, 5], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d44 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d45 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d46 = Conv2dConfig::new([192, 128], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d47 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([2, 2])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d48 = Conv2dConfig::new([128, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d49 = Conv2dConfig::new([128, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d50 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d51 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d52 = Conv2dConfig::new([384, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d53 = Conv2dConfig::new([64, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d54 = Conv2dConfig::new([64, 5], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d55 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d56 = Conv2dConfig::new([128, 128], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d57 = Conv2dConfig::new([384, 256], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d58 = Conv2dConfig::new([256, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d59 = Conv2dConfig::new([256, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d60 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d61 = Conv2dConfig::new([64, 64], [3, 3])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Explicit(1, 1))
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d62 = Conv2dConfig::new([64, 64], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d63 = Conv2dConfig::new([64, 5], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(true)
            .init(device);
        let conv2d64 = Conv2dConfig::new([16, 1], [1, 1])
            .with_stride([1, 1])
            .with_padding(PaddingConfig2d::Valid)
            .with_dilation([1, 1])
            .with_groups(1)
            .with_bias(false)
            .init(device);
        let constant1: burn::module::Param<Tensor<B, 3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 3>::zeros([1, 2, 8400], device),
            device.clone(),
            false,
            [1, 2, 8400].into(),
        );
        let constant2: burn::module::Param<Tensor<B, 3>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 3>::zeros([1, 2, 8400], device),
            device.clone(),
            false,
            [1, 2, 8400].into(),
        );
        let constant4: burn::module::Param<Tensor<B, 2>> = burn::module::Param::uninitialized(
            burn::module::ParamId::new(),
            move |device, _require_grad| Tensor::<B, 2>::zeros([1, 8400], device),
            device.clone(),
            false,
            [1, 8400].into(),
        );
        Self {
            conv2d1,
            conv2d2,
            conv2d3,
            conv2d4,
            conv2d5,
            conv2d6,
            conv2d7,
            conv2d8,
            conv2d9,
            conv2d10,
            conv2d11,
            conv2d12,
            conv2d13,
            conv2d14,
            conv2d15,
            conv2d16,
            conv2d17,
            conv2d18,
            conv2d19,
            conv2d20,
            conv2d21,
            conv2d22,
            conv2d23,
            conv2d24,
            conv2d25,
            conv2d26,
            maxpool2d1,
            maxpool2d2,
            maxpool2d3,
            conv2d27,
            resize1,
            conv2d28,
            conv2d29,
            conv2d30,
            conv2d31,
            resize2,
            conv2d32,
            conv2d33,
            conv2d34,
            conv2d35,
            conv2d36,
            conv2d37,
            conv2d38,
            conv2d39,
            conv2d40,
            conv2d41,
            conv2d42,
            conv2d43,
            conv2d44,
            conv2d45,
            conv2d46,
            conv2d47,
            conv2d48,
            conv2d49,
            conv2d50,
            conv2d51,
            conv2d52,
            conv2d53,
            conv2d54,
            conv2d55,
            conv2d56,
            conv2d57,
            conv2d58,
            conv2d59,
            conv2d60,
            conv2d61,
            conv2d62,
            conv2d63,
            conv2d64,
            constant1,
            constant2,
            constant4,
            phantom: core::marker::PhantomData,
            device: burn::module::Ignored(device.clone()),
        }
    }

    #[allow(clippy::let_and_return, clippy::approx_constant)]
    pub fn forward(&self, input1: Tensor<B, 4>) -> Tensor<B, 3> {
        let conv2d1_out1 = self.conv2d1.forward(input1);
        let sigmoid1_out1 = burn::tensor::activation::sigmoid(conv2d1_out1.clone());
        let mul1_out1 = conv2d1_out1.mul(sigmoid1_out1);
        let conv2d2_out1 = self.conv2d2.forward(mul1_out1);
        let sigmoid2_out1 = burn::tensor::activation::sigmoid(conv2d2_out1.clone());
        let mul2_out1 = conv2d2_out1.mul(sigmoid2_out1);
        let conv2d3_out1 = self.conv2d3.forward(mul2_out1);
        let sigmoid3_out1 = burn::tensor::activation::sigmoid(conv2d3_out1.clone());
        let mul3_out1 = conv2d3_out1.mul(sigmoid3_out1);
        let split_tensors = mul3_out1.split_with_sizes([16, 16].to_vec(), 1);
        let [split1_out1, split1_out2] = split_tensors.try_into().unwrap();
        let conv2d4_out1 = self.conv2d4.forward(split1_out2.clone());
        let sigmoid4_out1 = burn::tensor::activation::sigmoid(conv2d4_out1.clone());
        let mul4_out1 = conv2d4_out1.mul(sigmoid4_out1);
        let conv2d5_out1 = self.conv2d5.forward(mul4_out1);
        let sigmoid5_out1 = burn::tensor::activation::sigmoid(conv2d5_out1.clone());
        let mul5_out1 = conv2d5_out1.mul(sigmoid5_out1);
        let add1_out1 = split1_out2.clone().add(mul5_out1);
        let concat1_out1 = burn::tensor::Tensor::cat(
            [split1_out1, split1_out2, add1_out1].into(),
            1,
        );
        let conv2d6_out1 = self.conv2d6.forward(concat1_out1);
        let sigmoid6_out1 = burn::tensor::activation::sigmoid(conv2d6_out1.clone());
        let mul6_out1 = conv2d6_out1.mul(sigmoid6_out1);
        let conv2d7_out1 = self.conv2d7.forward(mul6_out1);
        let sigmoid7_out1 = burn::tensor::activation::sigmoid(conv2d7_out1.clone());
        let mul7_out1 = conv2d7_out1.mul(sigmoid7_out1);
        let conv2d8_out1 = self.conv2d8.forward(mul7_out1);
        let sigmoid8_out1 = burn::tensor::activation::sigmoid(conv2d8_out1.clone());
        let mul8_out1 = conv2d8_out1.mul(sigmoid8_out1);
        let split_tensors = mul8_out1.split_with_sizes([32, 32].to_vec(), 1);
        let [split2_out1, split2_out2] = split_tensors.try_into().unwrap();
        let conv2d9_out1 = self.conv2d9.forward(split2_out2.clone());
        let sigmoid9_out1 = burn::tensor::activation::sigmoid(conv2d9_out1.clone());
        let mul9_out1 = conv2d9_out1.mul(sigmoid9_out1);
        let conv2d10_out1 = self.conv2d10.forward(mul9_out1);
        let sigmoid10_out1 = burn::tensor::activation::sigmoid(conv2d10_out1.clone());
        let mul10_out1 = conv2d10_out1.mul(sigmoid10_out1);
        let add2_out1 = split2_out2.clone().add(mul10_out1);
        let conv2d11_out1 = self.conv2d11.forward(add2_out1.clone());
        let sigmoid11_out1 = burn::tensor::activation::sigmoid(conv2d11_out1.clone());
        let mul11_out1 = conv2d11_out1.mul(sigmoid11_out1);
        let conv2d12_out1 = self.conv2d12.forward(mul11_out1);
        let sigmoid12_out1 = burn::tensor::activation::sigmoid(conv2d12_out1.clone());
        let mul12_out1 = conv2d12_out1.mul(sigmoid12_out1);
        let add3_out1 = add2_out1.clone().add(mul12_out1);
        let concat2_out1 = burn::tensor::Tensor::cat(
            [split2_out1, split2_out2, add2_out1, add3_out1].into(),
            1,
        );
        let conv2d13_out1 = self.conv2d13.forward(concat2_out1);
        let sigmoid13_out1 = burn::tensor::activation::sigmoid(conv2d13_out1.clone());
        let mul13_out1 = conv2d13_out1.mul(sigmoid13_out1);
        let conv2d14_out1 = self.conv2d14.forward(mul13_out1.clone());
        let sigmoid14_out1 = burn::tensor::activation::sigmoid(conv2d14_out1.clone());
        let mul14_out1 = conv2d14_out1.mul(sigmoid14_out1);
        let conv2d15_out1 = self.conv2d15.forward(mul14_out1);
        let sigmoid15_out1 = burn::tensor::activation::sigmoid(conv2d15_out1.clone());
        let mul15_out1 = conv2d15_out1.mul(sigmoid15_out1);
        let split_tensors = mul15_out1.split_with_sizes([64, 64].to_vec(), 1);
        let [split3_out1, split3_out2] = split_tensors.try_into().unwrap();
        let conv2d16_out1 = self.conv2d16.forward(split3_out2.clone());
        let sigmoid16_out1 = burn::tensor::activation::sigmoid(conv2d16_out1.clone());
        let mul16_out1 = conv2d16_out1.mul(sigmoid16_out1);
        let conv2d17_out1 = self.conv2d17.forward(mul16_out1);
        let sigmoid17_out1 = burn::tensor::activation::sigmoid(conv2d17_out1.clone());
        let mul17_out1 = conv2d17_out1.mul(sigmoid17_out1);
        let add4_out1 = split3_out2.clone().add(mul17_out1);
        let conv2d18_out1 = self.conv2d18.forward(add4_out1.clone());
        let sigmoid18_out1 = burn::tensor::activation::sigmoid(conv2d18_out1.clone());
        let mul18_out1 = conv2d18_out1.mul(sigmoid18_out1);
        let conv2d19_out1 = self.conv2d19.forward(mul18_out1);
        let sigmoid19_out1 = burn::tensor::activation::sigmoid(conv2d19_out1.clone());
        let mul19_out1 = conv2d19_out1.mul(sigmoid19_out1);
        let add5_out1 = add4_out1.clone().add(mul19_out1);
        let concat3_out1 = burn::tensor::Tensor::cat(
            [split3_out1, split3_out2, add4_out1, add5_out1].into(),
            1,
        );
        let conv2d20_out1 = self.conv2d20.forward(concat3_out1);
        let sigmoid20_out1 = burn::tensor::activation::sigmoid(conv2d20_out1.clone());
        let mul20_out1 = conv2d20_out1.mul(sigmoid20_out1);
        let conv2d21_out1 = self.conv2d21.forward(mul20_out1.clone());
        let sigmoid21_out1 = burn::tensor::activation::sigmoid(conv2d21_out1.clone());
        let mul21_out1 = conv2d21_out1.mul(sigmoid21_out1);
        let conv2d22_out1 = self.conv2d22.forward(mul21_out1);
        let sigmoid22_out1 = burn::tensor::activation::sigmoid(conv2d22_out1.clone());
        let mul22_out1 = conv2d22_out1.mul(sigmoid22_out1);
        let split_tensors = mul22_out1.split_with_sizes([128, 128].to_vec(), 1);
        let [split4_out1, split4_out2] = split_tensors.try_into().unwrap();
        let conv2d23_out1 = self.conv2d23.forward(split4_out2.clone());
        let sigmoid23_out1 = burn::tensor::activation::sigmoid(conv2d23_out1.clone());
        let mul23_out1 = conv2d23_out1.mul(sigmoid23_out1);
        let conv2d24_out1 = self.conv2d24.forward(mul23_out1);
        let sigmoid24_out1 = burn::tensor::activation::sigmoid(conv2d24_out1.clone());
        let mul24_out1 = conv2d24_out1.mul(sigmoid24_out1);
        let add6_out1 = split4_out2.clone().add(mul24_out1);
        let concat4_out1 = burn::tensor::Tensor::cat(
            [split4_out1, split4_out2, add6_out1].into(),
            1,
        );
        let conv2d25_out1 = self.conv2d25.forward(concat4_out1);
        let sigmoid25_out1 = burn::tensor::activation::sigmoid(conv2d25_out1.clone());
        let mul25_out1 = conv2d25_out1.mul(sigmoid25_out1);
        let conv2d26_out1 = self.conv2d26.forward(mul25_out1);
        let sigmoid26_out1 = burn::tensor::activation::sigmoid(conv2d26_out1.clone());
        let mul26_out1 = conv2d26_out1.mul(sigmoid26_out1);
        let maxpool2d1_out1 = self.maxpool2d1.forward(mul26_out1.clone());
        let maxpool2d2_out1 = self.maxpool2d2.forward(maxpool2d1_out1.clone());
        let maxpool2d3_out1 = self.maxpool2d3.forward(maxpool2d2_out1.clone());
        let concat5_out1 = burn::tensor::Tensor::cat(
            [mul26_out1, maxpool2d1_out1, maxpool2d2_out1, maxpool2d3_out1].into(),
            1,
        );
        let conv2d27_out1 = self.conv2d27.forward(concat5_out1);
        let sigmoid27_out1 = burn::tensor::activation::sigmoid(conv2d27_out1.clone());
        let mul27_out1 = conv2d27_out1.mul(sigmoid27_out1);
        let resize1_out1 = self.resize1.forward(mul27_out1.clone());
        let concat6_out1 = burn::tensor::Tensor::cat(
            [resize1_out1, mul20_out1].into(),
            1,
        );
        let conv2d28_out1 = self.conv2d28.forward(concat6_out1);
        let sigmoid28_out1 = burn::tensor::activation::sigmoid(conv2d28_out1.clone());
        let mul28_out1 = conv2d28_out1.mul(sigmoid28_out1);
        let split_tensors = mul28_out1.split_with_sizes([64, 64].to_vec(), 1);
        let [split5_out1, split5_out2] = split_tensors.try_into().unwrap();
        let conv2d29_out1 = self.conv2d29.forward(split5_out2.clone());
        let sigmoid29_out1 = burn::tensor::activation::sigmoid(conv2d29_out1.clone());
        let mul29_out1 = conv2d29_out1.mul(sigmoid29_out1);
        let conv2d30_out1 = self.conv2d30.forward(mul29_out1);
        let sigmoid30_out1 = burn::tensor::activation::sigmoid(conv2d30_out1.clone());
        let mul30_out1 = conv2d30_out1.mul(sigmoid30_out1);
        let concat7_out1 = burn::tensor::Tensor::cat(
            [split5_out1, split5_out2, mul30_out1].into(),
            1,
        );
        let conv2d31_out1 = self.conv2d31.forward(concat7_out1);
        let sigmoid31_out1 = burn::tensor::activation::sigmoid(conv2d31_out1.clone());
        let mul31_out1 = conv2d31_out1.mul(sigmoid31_out1);
        let resize2_out1 = self.resize2.forward(mul31_out1.clone());
        let concat8_out1 = burn::tensor::Tensor::cat(
            [resize2_out1, mul13_out1].into(),
            1,
        );
        let conv2d32_out1 = self.conv2d32.forward(concat8_out1);
        let sigmoid32_out1 = burn::tensor::activation::sigmoid(conv2d32_out1.clone());
        let mul32_out1 = conv2d32_out1.mul(sigmoid32_out1);
        let split_tensors = mul32_out1.split_with_sizes([32, 32].to_vec(), 1);
        let [split6_out1, split6_out2] = split_tensors.try_into().unwrap();
        let conv2d33_out1 = self.conv2d33.forward(split6_out2.clone());
        let sigmoid33_out1 = burn::tensor::activation::sigmoid(conv2d33_out1.clone());
        let mul33_out1 = conv2d33_out1.mul(sigmoid33_out1);
        let conv2d34_out1 = self.conv2d34.forward(mul33_out1);
        let sigmoid34_out1 = burn::tensor::activation::sigmoid(conv2d34_out1.clone());
        let mul34_out1 = conv2d34_out1.mul(sigmoid34_out1);
        let concat9_out1 = burn::tensor::Tensor::cat(
            [split6_out1, split6_out2, mul34_out1].into(),
            1,
        );
        let conv2d35_out1 = self.conv2d35.forward(concat9_out1);
        let sigmoid35_out1 = burn::tensor::activation::sigmoid(conv2d35_out1.clone());
        let mul35_out1 = conv2d35_out1.mul(sigmoid35_out1);
        let conv2d36_out1 = self.conv2d36.forward(mul35_out1.clone());
        let conv2d37_out1 = self.conv2d37.forward(mul35_out1.clone());
        let conv2d38_out1 = self.conv2d38.forward(mul35_out1);
        let sigmoid36_out1 = burn::tensor::activation::sigmoid(conv2d36_out1.clone());
        let sigmoid37_out1 = burn::tensor::activation::sigmoid(conv2d37_out1.clone());
        let sigmoid38_out1 = burn::tensor::activation::sigmoid(conv2d38_out1.clone());
        let mul36_out1 = conv2d36_out1.mul(sigmoid36_out1);
        let mul37_out1 = conv2d37_out1.mul(sigmoid37_out1);
        let mul38_out1 = conv2d38_out1.mul(sigmoid38_out1);
        let concat10_out1 = burn::tensor::Tensor::cat(
            [mul36_out1, mul31_out1].into(),
            1,
        );
        let conv2d39_out1 = self.conv2d39.forward(mul37_out1);
        let conv2d40_out1 = self.conv2d40.forward(mul38_out1);
        let conv2d41_out1 = self.conv2d41.forward(concat10_out1);
        let sigmoid39_out1 = burn::tensor::activation::sigmoid(conv2d39_out1.clone());
        let sigmoid40_out1 = burn::tensor::activation::sigmoid(conv2d40_out1.clone());
        let sigmoid41_out1 = burn::tensor::activation::sigmoid(conv2d41_out1.clone());
        let mul39_out1 = conv2d39_out1.mul(sigmoid39_out1);
        let mul40_out1 = conv2d40_out1.mul(sigmoid40_out1);
        let mul41_out1 = conv2d41_out1.mul(sigmoid41_out1);
        let conv2d42_out1 = self.conv2d42.forward(mul39_out1);
        let conv2d43_out1 = self.conv2d43.forward(mul40_out1);
        let split_tensors = mul41_out1.split_with_sizes([64, 64].to_vec(), 1);
        let [split7_out1, split7_out2] = split_tensors.try_into().unwrap();
        let concat11_out1 = burn::tensor::Tensor::cat(
            [conv2d42_out1, conv2d43_out1].into(),
            1,
        );
        let conv2d44_out1 = self.conv2d44.forward(split7_out2.clone());
        let reshape1_out1 = concat11_out1.reshape([1, 69, -1]);
        let sigmoid42_out1 = burn::tensor::activation::sigmoid(conv2d44_out1.clone());
        let mul42_out1 = conv2d44_out1.mul(sigmoid42_out1);
        let conv2d45_out1 = self.conv2d45.forward(mul42_out1);
        let sigmoid43_out1 = burn::tensor::activation::sigmoid(conv2d45_out1.clone());
        let mul43_out1 = conv2d45_out1.mul(sigmoid43_out1);
        let concat12_out1 = burn::tensor::Tensor::cat(
            [split7_out1, split7_out2, mul43_out1].into(),
            1,
        );
        let conv2d46_out1 = self.conv2d46.forward(concat12_out1);
        let sigmoid44_out1 = burn::tensor::activation::sigmoid(conv2d46_out1.clone());
        let mul44_out1 = conv2d46_out1.mul(sigmoid44_out1);
        let conv2d47_out1 = self.conv2d47.forward(mul44_out1.clone());
        let conv2d48_out1 = self.conv2d48.forward(mul44_out1.clone());
        let conv2d49_out1 = self.conv2d49.forward(mul44_out1);
        let sigmoid45_out1 = burn::tensor::activation::sigmoid(conv2d47_out1.clone());
        let sigmoid46_out1 = burn::tensor::activation::sigmoid(conv2d48_out1.clone());
        let sigmoid47_out1 = burn::tensor::activation::sigmoid(conv2d49_out1.clone());
        let mul45_out1 = conv2d47_out1.mul(sigmoid45_out1);
        let mul46_out1 = conv2d48_out1.mul(sigmoid46_out1);
        let mul47_out1 = conv2d49_out1.mul(sigmoid47_out1);
        let concat13_out1 = burn::tensor::Tensor::cat(
            [mul45_out1, mul27_out1].into(),
            1,
        );
        let conv2d50_out1 = self.conv2d50.forward(mul46_out1);
        let conv2d51_out1 = self.conv2d51.forward(mul47_out1);
        let conv2d52_out1 = self.conv2d52.forward(concat13_out1);
        let sigmoid48_out1 = burn::tensor::activation::sigmoid(conv2d50_out1.clone());
        let sigmoid49_out1 = burn::tensor::activation::sigmoid(conv2d51_out1.clone());
        let sigmoid50_out1 = burn::tensor::activation::sigmoid(conv2d52_out1.clone());
        let mul48_out1 = conv2d50_out1.mul(sigmoid48_out1);
        let mul49_out1 = conv2d51_out1.mul(sigmoid49_out1);
        let mul50_out1 = conv2d52_out1.mul(sigmoid50_out1);
        let conv2d53_out1 = self.conv2d53.forward(mul48_out1);
        let conv2d54_out1 = self.conv2d54.forward(mul49_out1);
        let split_tensors = mul50_out1.split_with_sizes([128, 128].to_vec(), 1);
        let [split8_out1, split8_out2] = split_tensors.try_into().unwrap();
        let concat14_out1 = burn::tensor::Tensor::cat(
            [conv2d53_out1, conv2d54_out1].into(),
            1,
        );
        let conv2d55_out1 = self.conv2d55.forward(split8_out2.clone());
        let reshape2_out1 = concat14_out1.reshape([1, 69, -1]);
        let sigmoid51_out1 = burn::tensor::activation::sigmoid(conv2d55_out1.clone());
        let mul51_out1 = conv2d55_out1.mul(sigmoid51_out1);
        let conv2d56_out1 = self.conv2d56.forward(mul51_out1);
        let sigmoid52_out1 = burn::tensor::activation::sigmoid(conv2d56_out1.clone());
        let mul52_out1 = conv2d56_out1.mul(sigmoid52_out1);
        let concat15_out1 = burn::tensor::Tensor::cat(
            [split8_out1, split8_out2, mul52_out1].into(),
            1,
        );
        let conv2d57_out1 = self.conv2d57.forward(concat15_out1);
        let sigmoid53_out1 = burn::tensor::activation::sigmoid(conv2d57_out1.clone());
        let mul53_out1 = conv2d57_out1.mul(sigmoid53_out1);
        let conv2d58_out1 = self.conv2d58.forward(mul53_out1.clone());
        let conv2d59_out1 = self.conv2d59.forward(mul53_out1);
        let sigmoid54_out1 = burn::tensor::activation::sigmoid(conv2d58_out1.clone());
        let sigmoid55_out1 = burn::tensor::activation::sigmoid(conv2d59_out1.clone());
        let mul54_out1 = conv2d58_out1.mul(sigmoid54_out1);
        let mul55_out1 = conv2d59_out1.mul(sigmoid55_out1);
        let conv2d60_out1 = self.conv2d60.forward(mul54_out1);
        let conv2d61_out1 = self.conv2d61.forward(mul55_out1);
        let sigmoid56_out1 = burn::tensor::activation::sigmoid(conv2d60_out1.clone());
        let sigmoid57_out1 = burn::tensor::activation::sigmoid(conv2d61_out1.clone());
        let mul56_out1 = conv2d60_out1.mul(sigmoid56_out1);
        let mul57_out1 = conv2d61_out1.mul(sigmoid57_out1);
        let conv2d62_out1 = self.conv2d62.forward(mul56_out1);
        let conv2d63_out1 = self.conv2d63.forward(mul57_out1);
        let concat16_out1 = burn::tensor::Tensor::cat(
            [conv2d62_out1, conv2d63_out1].into(),
            1,
        );
        let reshape3_out1 = concat16_out1.reshape([1, 69, -1]);
        let concat17_out1 = burn::tensor::Tensor::cat(
            [reshape1_out1, reshape2_out1, reshape3_out1].into(),
            2,
        );
        let split_tensors = concat17_out1.split_with_sizes([64, 5].to_vec(), 1);
        let [split9_out1, split9_out2] = split_tensors.try_into().unwrap();
        let reshape4_out1 = split9_out1.reshape([1, 4, 16, 8400]);
        let sigmoid58_out1 = burn::tensor::activation::sigmoid(split9_out2);
        let transpose1_out1 = reshape4_out1.permute([0, 2, 1, 3]);
        let softmax1_out1 = burn::tensor::activation::softmax(transpose1_out1, 1);
        let conv2d64_out1 = self.conv2d64.forward(softmax1_out1);
        let reshape5_out1 = conv2d64_out1.reshape([1, 4, 8400]);
        let slice1_out1 = reshape5_out1.clone().slice(s![.., 0..2, ..]);
        let slice2_out1 = reshape5_out1.slice(s![.., 2..4, ..]);
        let constant1_out1 = self.constant1.val();
        let sub1_out1 = constant1_out1.sub(slice1_out1);
        let constant2_out1 = self.constant2.val();
        let add7_out1 = constant2_out1.add(slice2_out1);
        let add8_out1 = sub1_out1.clone().add(add7_out1.clone());
        let sub2_out1 = add7_out1.sub(sub1_out1);
        let constant3_out1: f32 = 2f32;
        let div1_out1 = add8_out1.div_scalar(constant3_out1);
        let concat18_out1 = burn::tensor::Tensor::cat([div1_out1, sub2_out1].into(), 1);
        let constant4_out1 = self.constant4.val();
        let mul58_out1 = concat18_out1.mul(constant4_out1.unsqueeze_dims(&[0isize]));
        let concat19_out1 = burn::tensor::Tensor::cat(
            [mul58_out1, sigmoid58_out1].into(),
            1,
        );
        concat19_out1
    }
}
