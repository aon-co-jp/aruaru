//! OpenCUDA 0.3.5 examples/tools source review for aruaru-llm integration.
//!
//! The uploaded files include sample Cargo manifests, sample main.rs files,
//! a GLSL compute shader, and Windows/Linux helper scripts. This module records
//! how those runnable examples should influence the next aruaru/OpenCUDA steps.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleToolArea {
    CpuMatmulSample,
    VulkanVectorAddShader,
    ShaderCompilePowerShell,
    ShaderCompileShell,
    NormalCheckScript,
    RealVulkanCheckPath,
}

impl ExampleToolArea {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CpuMatmulSample => "CPU matmul sample",
            Self::VulkanVectorAddShader => "Vulkan vector_add shader",
            Self::ShaderCompilePowerShell => "PowerShell shader compiler helper",
            Self::ShaderCompileShell => "Shell shader compiler helper",
            Self::NormalCheckScript => "OpenCUDA normal check script",
            Self::RealVulkanCheckPath => "real Vulkan check path",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextIntegrationPriority {
    KeepPassing,
    ImproveDiagnostics,
    PromoteToMatmul,
    PrepareAruaruLlmBridge,
}

impl NextIntegrationPriority {
    pub fn label(&self) -> &'static str {
        match self {
            Self::KeepPassing => "keep existing checks passing",
            Self::ImproveDiagnostics => "improve diagnostics",
            Self::PromoteToMatmul => "promote correctness target to matmul",
            Self::PrepareAruaruLlmBridge => "prepare aruaru-llm bridge",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExampleToolReviewItem {
    pub area: ExampleToolArea,
    pub priority: NextIntegrationPriority,
    pub uploaded_files: &'static [&'static str],
    pub finding: &'static str,
    pub aruaru_action: &'static str,
    pub safety_note: &'static str,
}

pub fn build_examples_tools_review() -> Vec<ExampleToolReviewItem> {
    vec![
        ExampleToolReviewItem {
            area: ExampleToolArea::CpuMatmulSample,
            priority: NextIntegrationPriority::PromoteToMatmul,
            uploaded_files: &["examples/matmul/Cargo.toml", "examples/matmul/src/main.rs"],
            finding: "The CPU matmul sample verifies 64x64x64 f32 matrix multiplication against a host reference through the GpuDevice contract.",
            aruaru_action: "Use this as the correctness reference for the next Vulkan matmul milestone before GEMM or attention work.",
            safety_note: "Do not optimize Vulkan matmul before CPU/Vulkan result parity is stable.",
        },
        ExampleToolReviewItem {
            area: ExampleToolArea::VulkanVectorAddShader,
            priority: NextIntegrationPriority::KeepPassing,
            uploaded_files: &["examples/vector_add_vulkan_real/shaders/vector_add.comp"],
            finding: "The shader uses local_size_x = 256, three storage buffers, and a push constant length guard for vector_add.",
            aruaru_action: "Keep vector_add as the minimal real Vulkan health check and template for later matmul shader layout.",
            safety_note: "Do not treat a vector_add shader as proof that LLM kernels are implemented.",
        },
        ExampleToolReviewItem {
            area: ExampleToolArea::ShaderCompilePowerShell,
            priority: NextIntegrationPriority::ImproveDiagnostics,
            uploaded_files: &["tools/compile-vulkan-shaders.ps1"],
            finding: "The PowerShell helper locates examples/vector_add_vulkan_real/shaders and fails clearly when glslc is missing.",
            aruaru_action: "Add glslc --version display and keep ExecutionPolicy Bypass instructions for Windows users.",
            safety_note: "PowerShell helpers must avoid hidden failures after external commands.",
        },
        ExampleToolReviewItem {
            area: ExampleToolArea::ShaderCompileShell,
            priority: NextIntegrationPriority::ImproveDiagnostics,
            uploaded_files: &["tools/compile-vulkan-shaders.sh"],
            finding: "The shell helper uses set -euo pipefail and command -v glslc before compiling vector_add.comp to vector_add.spv.",
            aruaru_action: "Keep the Linux/macOS helper path aligned with the Windows .cmd/.ps1 path.",
            safety_note: "Do not let OS-specific scripts drift into different shader outputs.",
        },
        ExampleToolReviewItem {
            area: ExampleToolArea::NormalCheckScript,
            priority: NextIntegrationPriority::KeepPassing,
            uploaded_files: &["tools/test-v0.3.5.ps1"],
            finding: "The normal check script runs workspace check plus CPU, OmniIR, VulkanMock, matmul, and vulkan_info samples.",
            aruaru_action: "Mirror this as a separate OpenCUDA health gate instead of merging it with aruaru README generation tests.",
            safety_note: "A failed OpenCUDA gate must stop before any downstream aruaru-llm integration claim.",
        },
        ExampleToolReviewItem {
            area: ExampleToolArea::RealVulkanCheckPath,
            priority: NextIntegrationPriority::PrepareAruaruLlmBridge,
            uploaded_files: &["tools/compile-vulkan-shaders.cmd", "examples/vector_add_vulkan_real"],
            finding: "The real Vulkan path is shader compile plus vector_add_vulkan_real execution.",
            aruaru_action: "Expose this as an optional physical-GPU proof gate in aruaru-llm diagnostics.",
            safety_note: "If Vulkan SDK or GPU driver is absent, fall back to CPU/VulkanMock without failing the whole aruaru app.",
        },
    ]
}

pub fn recommended_open_cuda_036_steps() -> Vec<&'static str> {
    vec![
        "Keep CPU matmul as the reference answer for Vulkan matmul.",
        "Add glslc --version output before shader compilation on Windows and shell helpers.",
        "Add vulkan_info queue family index, device type, API version, and driver version.",
        "Add a strict OpenCUDA gate that runs cargo check, CPU samples, OmniIR, VulkanMock, matmul, and vulkan_info.",
        "Add an optional real-Vulkan gate that compiles shaders and runs vector_add_vulkan_real only when glslc and Vulkan are present.",
        "Only after those pass, start the minimum Vulkan matmul shader and compare it against the CPU sample.",
    ]
}

pub fn examples_tools_markdown() -> String {
    let rows = build_examples_tools_review()
        .iter()
        .map(|item| {
            format!(
                "- {}: {} / files: {} / action: {}",
                item.area.label(),
                item.priority.label(),
                item.uploaded_files.join(", "),
                item.aruaru_action
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let steps = recommended_open_cuda_036_steps()
        .iter()
        .map(|step| format!("- {step}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# OpenCUDA 0.3.5 Examples and Tools Review\n\n\
This review connects uploaded example programs, shaders, and helper scripts to aruaru-llm integration.\n\n\
## Example/tool review map\n\n{}\n\n\
## Recommended OpenCUDA 0.3.6 steps\n\n{}\n\n\
## Safety policy\n\nTreat CPU matmul, OmniIR vector_add, VulkanMock, and real Vulkan vector_add as separate gates. Do not claim Vulkan matmul, GEMM, attention, quantization, LLM inference, LLM learning, or mixed-vendor shared VRAM until each layer has its own checked example and comparison test.\n",
        rows, steps
    )
}

pub fn quality_gate_smoke_check() -> bool {
    let review = build_examples_tools_review();
    let markdown = examples_tools_markdown();
    let areas = [
        ExampleToolArea::CpuMatmulSample,
        ExampleToolArea::VulkanVectorAddShader,
        ExampleToolArea::ShaderCompilePowerShell,
        ExampleToolArea::ShaderCompileShell,
        ExampleToolArea::NormalCheckScript,
        ExampleToolArea::RealVulkanCheckPath,
    ];
    let priorities = [
        NextIntegrationPriority::KeepPassing,
        NextIntegrationPriority::ImproveDiagnostics,
        NextIntegrationPriority::PromoteToMatmul,
        NextIntegrationPriority::PrepareAruaruLlmBridge,
    ];

    review.len() == areas.len()
        && areas.iter().all(|area| review.iter().any(|item| item.area == *area))
        && priorities.iter().all(|priority| !priority.label().is_empty())
        && markdown.contains("CPU matmul")
        && markdown.contains("glslc --version")
        && markdown.contains("vulkan_info queue family index")
        && markdown.contains("optional real-Vulkan gate")
        && markdown.contains("Do not claim Vulkan matmul")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn review_contains_uploaded_examples_and_tools() {
        let review = build_examples_tools_review();
        assert!(review.iter().any(|item| item.area == ExampleToolArea::CpuMatmulSample));
        assert!(review.iter().any(|item| item.area == ExampleToolArea::VulkanVectorAddShader));
        assert!(review.iter().any(|item| item.area == ExampleToolArea::NormalCheckScript));
    }

    #[test]
    fn review_promotes_vulkan_matmul_only_after_correctness() {
        let steps = recommended_open_cuda_036_steps();
        assert!(steps.iter().any(|step| step.contains("CPU matmul as the reference")));
        assert!(steps.iter().any(|step| step.contains("Vulkan matmul shader")));
    }

    #[test]
    fn review_blocks_overclaiming_llm_features() {
        let markdown = examples_tools_markdown();
        assert!(markdown.contains("Do not claim Vulkan matmul"));
        assert!(markdown.contains("LLM learning"));
        assert!(markdown.contains("mixed-vendor shared VRAM"));
    }

    #[test]
    fn quality_gate_connects_examples_tools_review() {
        assert!(quality_gate_smoke_check());
    }
}
