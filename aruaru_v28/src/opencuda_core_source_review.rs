//! OpenCUDA 0.3.5 uploaded source review for aruaru-llm integration.
//!
//! This module maps the actual uploaded OpenCUDA core/Vulkan source shape to the
//! aruaru-ai integration plan. It intentionally treats OpenCUDA 0.3.5 as a real
//! prototype with a small verified surface, not as a finished accelerator stack.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadedSourceArea {
    CoreDeviceTrait,
    TwoLayerMemory,
    KernelSourceEnum,
    DeviceRegistry,
    VulkanMockContract,
    RealVulkanVectorAddPath,
    OmniIrVectorAddPath,
}

impl UploadedSourceArea {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CoreDeviceTrait => "GpuDevice trait",
            Self::TwoLayerMemory => "DevicePtr + DeviceBuffer memory model",
            Self::KernelSourceEnum => "KernelSource enum",
            Self::DeviceRegistry => "DeviceRegistry scheduler seed",
            Self::VulkanMockContract => "VulkanMock contract tests",
            Self::RealVulkanVectorAddPath => "real Vulkan vector_add path",
            Self::OmniIrVectorAddPath => "OmniIR vector_add path",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AruaruIntegrationRole {
    StableContract,
    SafetyGate,
    SchedulingSeed,
    VerifiedGpuProof,
    NextCompilerSeed,
}

impl AruaruIntegrationRole {
    pub fn label(&self) -> &'static str {
        match self {
            Self::StableContract => "stable contract",
            Self::SafetyGate => "safety gate",
            Self::SchedulingSeed => "scheduling seed",
            Self::VerifiedGpuProof => "verified GPU proof",
            Self::NextCompilerSeed => "next compiler seed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCudaSourceReviewItem {
    pub area: UploadedSourceArea,
    pub role: AruaruIntegrationRole,
    pub source_files: &'static [&'static str],
    pub aruaru_action: &'static str,
    pub do_not_overclaim: &'static str,
}

pub fn build_source_review_plan() -> Vec<OpenCudaSourceReviewItem> {
    vec![
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::CoreDeviceTrait,
            role: AruaruIntegrationRole::StableContract,
            source_files: &["device.rs", "lib.rs"],
            aruaru_action: "Use GpuDevice as the bridge contract between aruaru-llm tasks and OpenCUDA/iLumi backends.",
            do_not_overclaim: "Do not claim all NVIDIA, AMD, Intel, CPU, and NPU paths are already implemented just because the trait exists.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::TwoLayerMemory,
            role: AruaruIntegrationRole::SafetyGate,
            source_files: &["memory.rs"],
            aruaru_action: "Preserve DevicePtr device_id ownership and DeviceBuffer RAII release when aruaru adds multi-device scheduling.",
            do_not_overclaim: "Do not describe separate device memory as one automatically shared VRAM pool.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::KernelSourceEnum,
            role: AruaruIntegrationRole::StableContract,
            source_files: &["kernel.rs"],
            aruaru_action: "Route Native, SpirV, Ptx, and OmniIr as explicit source kinds so each backend can accept only what it supports.",
            do_not_overclaim: "Do not claim PTX or full OmniIR JIT support is complete before compiler paths exist.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::DeviceRegistry,
            role: AruaruIntegrationRole::SchedulingSeed,
            source_files: &["registry.rs"],
            aruaru_action: "Start aruaru device selection with registered devices and memory-aware inference selection, then later add topology and thermal scoring.",
            do_not_overclaim: "Do not treat memory-max selection as a complete heterogeneous scheduler.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::VulkanMockContract,
            role: AruaruIntegrationRole::SafetyGate,
            source_files: &["mock_device.rs", "vulkan_mock.rs"],
            aruaru_action: "Keep mock tests so SPIR-V and backend-contract errors are testable without owning a GPU.",
            do_not_overclaim: "Do not treat mock success as real GPU performance.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::RealVulkanVectorAddPath,
            role: AruaruIntegrationRole::VerifiedGpuProof,
            source_files: &["real.rs"],
            aruaru_action: "Use the real Vulkan vector_add path as the first physical GPU proof before moving to Vulkan matmul.",
            do_not_overclaim: "Do not claim GEMM, attention, quantization, or LLM training are implemented from vector_add alone.",
        },
        OpenCudaSourceReviewItem {
            area: UploadedSourceArea::OmniIrVectorAddPath,
            role: AruaruIntegrationRole::NextCompilerSeed,
            source_files: &["omniir_path.rs", "kernel.rs"],
            aruaru_action: "Use OmniIR vector_add as the seed for future Folding/SBM planned execution paths and CPU/Vulkan comparison tests.",
            do_not_overclaim: "Do not claim full CUDA C, HIP, or SYCL parsing is complete.",
        },
    ]
}

pub fn recommended_next_steps() -> Vec<&'static str> {
    vec![
        "Keep CPU, VulkanMock, OmniIR vector_add, and real Vulkan vector_add as separate quality gates.",
        "Promote vulkan_info into the aruaru device detector before scheduling GPU work.",
        "Implement Vulkan matmul as the next correctness milestone before GEMM or attention.",
        "Add source-level documentation that mixed NVIDIA and AMD memory is staged, not automatically unified.",
        "Add an aruaru-llm adapter that can ask OpenCUDA for device inventory, supported kernel source kinds, and safe fallback mode.",
        "Only after Vulkan matmul correctness, begin small GEMM, quantization, and attention kernels for local LLM inference.",
    ]
}

pub fn source_review_markdown() -> String {
    let rows = build_source_review_plan()
        .iter()
        .map(|item| {
            format!(
                "- {}: {} / files: {} / action: {}",
                item.area.label(),
                item.role.label(),
                item.source_files.join(", "),
                item.aruaru_action
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let steps = recommended_next_steps()
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# OpenCUDA 0.3.5 Source Review Integration\n\n\
This review connects uploaded OpenCUDA core/Vulkan source files to aruaru-llm.\n\n\
## Source review map\n\n{}\n\n\
## Recommended next steps\n\n{}\n\n\
## Safety policy\n\nUse the uploaded source as a concrete implementation reference for DevicePtr ownership, GpuDevice contracts, KernelSource routing, VulkanMock quality gates, OmniIR vector_add, and real Vulkan vector_add. Do not claim complete CUDA compatibility, automatic mixed-vendor shared VRAM, or LLM training support before the corresponding kernels and tests are implemented.\n",
        rows, steps
    )
}

pub fn quality_gate_smoke_check() -> bool {
    let plan = build_source_review_plan();
    let markdown = source_review_markdown();
    let areas = [
        UploadedSourceArea::CoreDeviceTrait,
        UploadedSourceArea::TwoLayerMemory,
        UploadedSourceArea::KernelSourceEnum,
        UploadedSourceArea::DeviceRegistry,
        UploadedSourceArea::VulkanMockContract,
        UploadedSourceArea::RealVulkanVectorAddPath,
        UploadedSourceArea::OmniIrVectorAddPath,
    ];
    let roles = [
        AruaruIntegrationRole::StableContract,
        AruaruIntegrationRole::SafetyGate,
        AruaruIntegrationRole::SchedulingSeed,
        AruaruIntegrationRole::VerifiedGpuProof,
        AruaruIntegrationRole::NextCompilerSeed,
    ];

    plan.len() == areas.len()
        && areas.iter().all(|area| plan.iter().any(|item| item.area == *area))
        && roles.iter().all(|role| !role.label().is_empty())
        && markdown.contains("DevicePtr ownership")
        && markdown.contains("GpuDevice contracts")
        && markdown.contains("Vulkan matmul")
        && markdown.contains("Do not claim complete CUDA compatibility")
        && recommended_next_steps().iter().any(|step| step.contains("device inventory"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_contains_uploaded_core_contracts() {
        let plan = build_source_review_plan();
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::CoreDeviceTrait));
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::TwoLayerMemory));
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::KernelSourceEnum));
    }

    #[test]
    fn review_contains_vulkan_and_omniir_paths() {
        let plan = build_source_review_plan();
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::RealVulkanVectorAddPath));
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::OmniIrVectorAddPath));
        assert!(plan.iter().any(|item| item.area == UploadedSourceArea::VulkanMockContract));
    }

    #[test]
    fn review_blocks_overclaiming() {
        let markdown = source_review_markdown();
        assert!(markdown.contains("Do not claim complete CUDA compatibility"));
        assert!(markdown.contains("automatic mixed-vendor shared VRAM"));
        assert!(markdown.contains("before the corresponding kernels and tests are implemented"));
    }

    #[test]
    fn quality_gate_connects_source_review() {
        assert!(quality_gate_smoke_check());
    }
}
