//! OpenCUDA iLumi 0.3.5 reference integration plan for aruaru-llm.
//!
//! This module records what is already proven in the public OpenCUDA/iLumi
//! prototype and how aruaru-ai should consume it without exaggerating the scope.
//! The goal is to connect aruaru-llm to a real, staged Rust GPU runtime plan:
//! CPU first, Vulkan vector_add next, and LLM inference kernels later.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenCuda035Component {
    CpuBackend,
    VulkanMockBackend,
    RealVulkanVectorAdd,
    OmniIrVectorAdd,
    MatmulCpuExample,
    VulkanInfoTool,
    WindowsCmdTestScripts,
}

impl OpenCuda035Component {
    pub fn label(&self) -> &'static str {
        match self {
            Self::CpuBackend => "CPU backend",
            Self::VulkanMockBackend => "Vulkan mock backend",
            Self::RealVulkanVectorAdd => "real Vulkan vector_add",
            Self::OmniIrVectorAdd => "OmniIR vector_add",
            Self::MatmulCpuExample => "CPU matmul example",
            Self::VulkanInfoTool => "vulkan_info diagnostic tool",
            Self::WindowsCmdTestScripts => "Windows .cmd test scripts",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrationStatus {
    VerifiedPrototype,
    SafeReference,
    NextMilestone,
    DoNotClaimYet,
}

impl IntegrationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::VerifiedPrototype => "verified prototype",
            Self::SafeReference => "safe reference",
            Self::NextMilestone => "next milestone",
            Self::DoNotClaimYet => "do not claim yet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCuda035ReferenceItem {
    pub component: OpenCuda035Component,
    pub status: IntegrationStatus,
    pub aruaru_use: &'static str,
    pub verification_command: &'static str,
}

pub fn build_opencuda_035_reference_plan() -> Vec<OpenCuda035ReferenceItem> {
    vec![
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::CpuBackend,
            status: IntegrationStatus::VerifiedPrototype,
            aruaru_use: "Use as the always-available fallback for aruaru-llm RAG, quality gates, folding compression, and deterministic tests.",
            verification_command: "cargo run --release -p vector_add",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::VulkanMockBackend,
            status: IntegrationStatus::SafeReference,
            aruaru_use: "Use to keep the SPIR-V/OmniIR contract testable even without a GPU or Vulkan driver.",
            verification_command: "cargo run --release -p vector_add_vulkan",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::RealVulkanVectorAdd,
            status: IntegrationStatus::VerifiedPrototype,
            aruaru_use: "Use as the first real cross-vendor GPU proof point before claiming broader NVIDIA/AMD/Intel execution.",
            verification_command: "cargo run --release -p vector_add_vulkan_real",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::OmniIrVectorAdd,
            status: IntegrationStatus::SafeReference,
            aruaru_use: "Use as the seed for the aruaru Folding/SBM execution planner because one IR can target CPU and Vulkan-style paths.",
            verification_command: "cargo run --release -p vector_add_omniir",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::MatmulCpuExample,
            status: IntegrationStatus::NextMilestone,
            aruaru_use: "Promote CPU matmul to Vulkan matmul before attempting LLM GEMM, attention, or quantization kernels.",
            verification_command: "cargo run --release -p matmul",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::VulkanInfoTool,
            status: IntegrationStatus::VerifiedPrototype,
            aruaru_use: "Use to detect physical device, queue family, driver, and compute support before aruaru schedules GPU work.",
            verification_command: "cargo run --release -p vulkan_info",
        },
        OpenCuda035ReferenceItem {
            component: OpenCuda035Component::WindowsCmdTestScripts,
            status: IntegrationStatus::SafeReference,
            aruaru_use: "Use .cmd wrappers for Windows users whose PowerShell execution policy blocks .ps1 scripts.",
            verification_command: ".\\tools\\test-v0.3.5.cmd",
        },
    ]
}

pub fn next_safe_milestones() -> Vec<&'static str> {
    vec![
        "Improve real Vulkan error messages before adding more kernels.",
        "Show queue family index, device type, API version, and driver version in vulkan_info.",
        "Print glslc --version before compiling shaders.",
        "Reduce cargo clippy --workspace --all-targets warnings.",
        "Implement minimal Vulkan matmul and compare it with CPU matmul for correctness first.",
        "Only after matmul correctness, begin LLM-oriented GEMM, quantization, and attention experiments.",
    ]
}

pub fn opencuda_035_reference_markdown() -> String {
    let rows = build_opencuda_035_reference_plan()
        .iter()
        .map(|item| {
            format!(
                "- {}: {} / `{}`",
                item.component.label(),
                item.status.label(),
                item.verification_command
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let milestones = next_safe_milestones()
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# OpenCUDA iLumi 0.3.5 Reference Integration\n\n\
OpenCUDA iLumi 0.3.5 is treated as a real staged prototype, not as a finished CUDA replacement.\n\n\
## Current usable reference points\n\n{}\n\n\
## Next safe milestones\n\n{}\n\n\
## aruaru policy\n\nUse OpenCUDA 0.3.5 as the concrete base for CPU fallback, OmniIR, Vulkan mock, real Vulkan vector_add, and Windows test tooling. Do not claim full CUDA compatibility, unified mixed-vendor VRAM, or LLM training support before the corresponding kernels and tests exist.\n",
        rows, milestones
    )
}

pub fn quality_gate_smoke_check() -> bool {
    let plan = build_opencuda_035_reference_plan();
    let markdown = opencuda_035_reference_markdown();
    let components = [
        OpenCuda035Component::CpuBackend,
        OpenCuda035Component::VulkanMockBackend,
        OpenCuda035Component::RealVulkanVectorAdd,
        OpenCuda035Component::OmniIrVectorAdd,
        OpenCuda035Component::MatmulCpuExample,
        OpenCuda035Component::VulkanInfoTool,
        OpenCuda035Component::WindowsCmdTestScripts,
    ];
    let statuses = [
        IntegrationStatus::VerifiedPrototype,
        IntegrationStatus::SafeReference,
        IntegrationStatus::NextMilestone,
        IntegrationStatus::DoNotClaimYet,
    ];

    plan.len() == components.len()
        && components
            .iter()
            .all(|component| plan.iter().any(|item| item.component == *component))
        && statuses.iter().all(|status| !status.label().is_empty())
        && markdown.contains("OpenCUDA iLumi 0.3.5")
        && markdown.contains("real Vulkan vector_add")
        && markdown.contains("Do not claim full CUDA compatibility")
        && next_safe_milestones()
            .iter()
            .any(|item| item.contains("Vulkan matmul"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_plan_contains_035_proven_paths() {
        let plan = build_opencuda_035_reference_plan();
        assert!(plan.iter().any(|item| item.component == OpenCuda035Component::CpuBackend));
        assert!(plan
            .iter()
            .any(|item| item.component == OpenCuda035Component::RealVulkanVectorAdd));
        assert!(plan.iter().any(|item| item.component == OpenCuda035Component::OmniIrVectorAdd));
    }

    #[test]
    fn reference_plan_does_not_overclaim_cuda_or_llm_training() {
        let markdown = opencuda_035_reference_markdown();
        assert!(markdown.contains("not as a finished CUDA replacement"));
        assert!(markdown.contains("Do not claim full CUDA compatibility"));
        assert!(markdown.contains("before the corresponding kernels and tests exist"));
    }

    #[test]
    fn next_step_is_vulkan_matmul_before_llm_kernels() {
        let milestones = next_safe_milestones();
        assert!(milestones.iter().any(|item| item.contains("Vulkan matmul")));
        assert!(milestones.iter().any(|item| item.contains("GEMM")));
    }

    #[test]
    fn quality_gate_connects_reference_module() {
        assert!(quality_gate_smoke_check());
    }
}
