use ort::{
    CUDAExecutionProvider, CoreMLExecutionProvider, DirectMLExecutionProvider, Session,
    TensorRTExecutionProvider,
};

pub struct Onnx {
    session: Session,
}

impl Onnx {
    pub fn new() -> anyhow::Result<Self> {
        let session = Session::builder()?
            .with_execution_providers([
                // Prefer TensorRT over CUDA.
                TensorRTExecutionProvider::default().build(),
                CUDAExecutionProvider::default().build(),
                // Use DirectML on Windows if NVIDIA EPs are not available
                DirectMLExecutionProvider::default().build(),
                // Or use ANE on Apple platforms
                CoreMLExecutionProvider::default().build(),
            ])?
            .commit_from_file("assets/model.onnx")?;

        Ok(Self { session })
    }
}
