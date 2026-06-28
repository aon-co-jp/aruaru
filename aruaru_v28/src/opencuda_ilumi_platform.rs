//! OpenCUDA iLumi multi-vendor device platform planner.
//!
//! This module models OpenCUDA iLumi as an AI execution abstraction that can route
//! work across NVIDIA, AMD, Intel GPUs, ordinary CPUs, mobile CPUs, and Copilot+ PC
//! class NPUs. It does not claim that all devices share one unified VRAM pool.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceFamily {
    NvidiaGpu,
    AmdGpu,
    IntelGpu,
    PcCpu,
    TabletCpu,
    SmartphoneCpu,
    CopilotPlusNpu,
    MobileNpu,
}

impl DeviceFamily {
    pub fn label(&self) -> &'static str {
        match self {
            Self::NvidiaGpu => "NVIDIA GPU",
            Self::AmdGpu => "AMD GPU",
            Self::IntelGpu => "Intel GPU",
            Self::PcCpu => "PC CPU",
            Self::TabletCpu => "Tablet CPU",
            Self::SmartphoneCpu => "Smartphone CPU",
            Self::CopilotPlusNpu => "Copilot+ PC NPU",
            Self::MobileNpu => "Mobile NPU",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Cuda,
    RocmHip,
    OneApiLevelZero,
    DirectMl,
    DirectCompute,
    Vulkan,
    OnnxRuntimeExecutionProvider,
    CpuFallback,
    AndroidNnapiOrQnn,
    AppleCoreMl,
}

impl BackendKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Cuda => "CUDA backend",
            Self::RocmHip => "ROCm/HIP backend",
            Self::OneApiLevelZero => "oneAPI/Level Zero backend",
            Self::DirectMl => "DirectML backend",
            Self::DirectCompute => "DirectCompute backend",
            Self::Vulkan => "Vulkan backend",
            Self::OnnxRuntimeExecutionProvider => "ONNX Runtime Execution Provider",
            Self::CpuFallback => "CPU fallback backend",
            Self::AndroidNnapiOrQnn => "Android NNAPI/QNN backend",
            Self::AppleCoreMl => "Apple Core ML backend",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadClass {
    LlmInference,
    Embedding,
    RagSearch,
    Rerank,
    QualityGate,
    FoldingCompression,
    SbmInspiredSearch,
    AdapterCandidate,
    LoraCandidate,
    FullFineTune,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportLevel {
    Primary,
    Supported,
    Experimental,
    Fallback,
    NotRecommended,
}

impl SupportLevel {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Supported => "supported",
            Self::Experimental => "experimental",
            Self::Fallback => "fallback",
            Self::NotRecommended => "not recommended",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSupportProfile {
    pub family: DeviceFamily,
    pub preferred_backends: Vec<BackendKind>,
    pub best_workloads: Vec<WorkloadClass>,
    pub support_level: SupportLevel,
    pub policy: &'static str,
}

pub fn build_device_profiles() -> Vec<DeviceSupportProfile> {
    vec![
        DeviceSupportProfile {
            family: DeviceFamily::NvidiaGpu,
            preferred_backends: vec![BackendKind::Cuda, BackendKind::DirectMl, BackendKind::Vulkan],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::AdapterCandidate,
                WorkloadClass::LoraCandidate,
                WorkloadClass::QualityGate,
            ],
            support_level: SupportLevel::Primary,
            policy: "Use NVIDIA as the first-choice training and LoRA/QLoRA backend when CUDA is available.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::AmdGpu,
            preferred_backends: vec![BackendKind::RocmHip, BackendKind::DirectMl, BackendKind::Vulkan],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::RagSearch,
                WorkloadClass::Rerank,
                WorkloadClass::SbmInspiredSearch,
            ],
            support_level: SupportLevel::Experimental,
            policy: "Use AMD for inference, RAG, ranking, Vulkan/ROCm experiments, and OpenCUDA iLumi cross-vendor research before persistent training.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::IntelGpu,
            preferred_backends: vec![BackendKind::OneApiLevelZero, BackendKind::DirectMl, BackendKind::Vulkan],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::RagSearch,
                WorkloadClass::QualityGate,
            ],
            support_level: SupportLevel::Supported,
            policy: "Use Intel GPUs as Windows/DirectML/oneAPI inference and evaluation accelerators.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::PcCpu,
            preferred_backends: vec![BackendKind::CpuFallback, BackendKind::OnnxRuntimeExecutionProvider],
            best_workloads: vec![
                WorkloadClass::RagSearch,
                WorkloadClass::QualityGate,
                WorkloadClass::FoldingCompression,
                WorkloadClass::SbmInspiredSearch,
            ],
            support_level: SupportLevel::Fallback,
            policy: "Use CPU for safe fallback, long logs, database tasks, deterministic checks, and memory-heavy orchestration.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::TabletCpu,
            preferred_backends: vec![BackendKind::CpuFallback, BackendKind::OnnxRuntimeExecutionProvider],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::FoldingCompression,
            ],
            support_level: SupportLevel::Fallback,
            policy: "Use tablet CPUs for light inference, local summaries, cached RAG lookup, and power-saving mode.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::SmartphoneCpu,
            preferred_backends: vec![
                BackendKind::AndroidNnapiOrQnn,
                BackendKind::AppleCoreMl,
                BackendKind::CpuFallback,
            ],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::FoldingCompression,
            ],
            support_level: SupportLevel::Experimental,
            policy: "Use smartphone hardware for small on-device models, summaries, voice helpers, and mobile RAG cache, not heavy training.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::CopilotPlusNpu,
            preferred_backends: vec![BackendKind::OnnxRuntimeExecutionProvider, BackendKind::DirectMl],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::Rerank,
                WorkloadClass::FoldingCompression,
            ],
            support_level: SupportLevel::Experimental,
            policy: "Use Copilot+ class NPUs for low-power ONNX inference and assistant features when drivers and execution providers are available.",
        },
        DeviceSupportProfile {
            family: DeviceFamily::MobileNpu,
            preferred_backends: vec![BackendKind::AndroidNnapiOrQnn, BackendKind::AppleCoreMl],
            best_workloads: vec![
                WorkloadClass::LlmInference,
                WorkloadClass::Embedding,
                WorkloadClass::FoldingCompression,
            ],
            support_level: SupportLevel::Experimental,
            policy: "Use mobile NPUs for privacy-preserving small inference and power-saving voice/chat features.",
        },
    ]
}

pub fn support_for(family: DeviceFamily, workload: WorkloadClass) -> SupportLevel {
    let Some(profile) = build_device_profiles().into_iter().find(|item| item.family == family) else {
        return SupportLevel::NotRecommended;
    };

    if workload == WorkloadClass::FullFineTune {
        return match family {
            DeviceFamily::NvidiaGpu => SupportLevel::Experimental,
            DeviceFamily::AmdGpu | DeviceFamily::IntelGpu => SupportLevel::NotRecommended,
            _ => SupportLevel::NotRecommended,
        };
    }

    if profile.best_workloads.contains(&workload) {
        profile.support_level
    } else {
        SupportLevel::NotRecommended
    }
}

pub fn opencuda_ilumi_platform_markdown() -> String {
    let profiles = build_device_profiles();
    let rows = profiles
        .iter()
        .map(|profile| {
            let backends = profile
                .preferred_backends
                .iter()
                .map(BackendKind::label)
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "- {}: {} / {}",
                profile.family.label(),
                profile.support_level.label(),
                backends
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "# OpenCUDA iLumi Multi-Device Platform\n\n\
OpenCUDA iLumi supports NVIDIA, AMD, and Intel GPUs, plus PC/tablet/smartphone CPUs and NPU backends.\n\n\
## Device profiles\n\n{}\n\n\
## Safety rule\n\nDo not treat mixed NVIDIA/AMD/Intel VRAM as one automatically unified training memory pool. Route tasks by backend capability and fall back safely.\n",
        rows
    )
}

pub fn quality_gate_smoke_check() -> bool {
    let profiles = build_device_profiles();
    let markdown = opencuda_ilumi_platform_markdown();
    let families = [
        DeviceFamily::NvidiaGpu,
        DeviceFamily::AmdGpu,
        DeviceFamily::IntelGpu,
        DeviceFamily::PcCpu,
        DeviceFamily::TabletCpu,
        DeviceFamily::SmartphoneCpu,
        DeviceFamily::CopilotPlusNpu,
        DeviceFamily::MobileNpu,
    ];
    let backends = [
        BackendKind::Cuda,
        BackendKind::RocmHip,
        BackendKind::OneApiLevelZero,
        BackendKind::DirectMl,
        BackendKind::DirectCompute,
        BackendKind::Vulkan,
        BackendKind::OnnxRuntimeExecutionProvider,
        BackendKind::CpuFallback,
        BackendKind::AndroidNnapiOrQnn,
        BackendKind::AppleCoreMl,
    ];

    profiles.len() == families.len()
        && families.iter().all(|family| profiles.iter().any(|profile| profile.family == *family))
        && backends.iter().all(|backend| backend.label().contains("backend") || backend.label().contains("Provider"))
        && support_for(DeviceFamily::NvidiaGpu, WorkloadClass::LoraCandidate) == SupportLevel::Primary
        && support_for(DeviceFamily::CopilotPlusNpu, WorkloadClass::LlmInference) == SupportLevel::Experimental
        && support_for(DeviceFamily::SmartphoneCpu, WorkloadClass::FullFineTune) == SupportLevel::NotRecommended
        && markdown.contains("NVIDIA")
        && markdown.contains("AMD")
        && markdown.contains("Intel")
        && markdown.contains("NPU")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_covers_three_gpu_vendors_cpu_and_npu() {
        let profiles = build_device_profiles();
        assert!(profiles.iter().any(|profile| profile.family == DeviceFamily::NvidiaGpu));
        assert!(profiles.iter().any(|profile| profile.family == DeviceFamily::AmdGpu));
        assert!(profiles.iter().any(|profile| profile.family == DeviceFamily::IntelGpu));
        assert!(profiles.iter().any(|profile| profile.family == DeviceFamily::PcCpu));
        assert!(profiles.iter().any(|profile| profile.family == DeviceFamily::CopilotPlusNpu));
    }

    #[test]
    fn nvidia_is_primary_for_lora_but_phone_is_not_for_full_finetune() {
        assert_eq!(support_for(DeviceFamily::NvidiaGpu, WorkloadClass::LoraCandidate), SupportLevel::Primary);
        assert_eq!(support_for(DeviceFamily::SmartphoneCpu, WorkloadClass::FullFineTune), SupportLevel::NotRecommended);
    }

    #[test]
    fn directml_vulkan_and_cpu_fallback_are_present() {
        let markdown = opencuda_ilumi_platform_markdown();
        assert!(markdown.contains("DirectML"));
        assert!(markdown.contains("Vulkan"));
        assert!(markdown.contains("CPU fallback"));
    }

    #[test]
    fn quality_gate_connects_all_variants() {
        assert!(quality_gate_smoke_check());
    }
}
