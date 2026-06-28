//! aruaru-llm original hybrid learning architecture.
//!
//! This module is intentionally practical: it does not claim that one consumer GPU
//! literally equals a 100,000 GPU supercomputer. Instead, it defines an engineering
//! path that folds context, routes work, optimizes decisions, and learns safely over
//! time on ordinary PCs.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TheoryLayer {
    FoldingCompression,
    SparseExpertRouting,
    QuantumInspiredOptimizer,
    ContinualLearningMemory,
    AdapterFineTuning,
    EvaluationGate,
    HumanApproval,
}

impl TheoryLayer {
    pub fn label(&self) -> &'static str {
        match self {
            Self::FoldingCompression => "Folding compression",
            Self::SparseExpertRouting => "Sparse expert routing",
            Self::QuantumInspiredOptimizer => "Quantum-inspired optimizer",
            Self::ContinualLearningMemory => "Continual learning memory",
            Self::AdapterFineTuning => "Adapter fine-tuning",
            Self::EvaluationGate => "Evaluation gate",
            Self::HumanApproval => "Human approval",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearningMode {
    NoLearning,
    MemoryOnly,
    RetrievalAugmented,
    AdapterCandidate,
    ApprovedFineTune,
}

impl LearningMode {
    pub fn risk_label(&self) -> &'static str {
        match self {
            Self::NoLearning => "safe baseline",
            Self::MemoryOnly => "low risk",
            Self::RetrievalAugmented => "low to medium risk",
            Self::AdapterCandidate => "medium risk",
            Self::ApprovedFineTune => "high control required",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareTier {
    Gt730CpuFirst,
    TwelveGbGpu,
    TwentyFourGbGpu,
    MultiGpuWorkstation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizerUseCase {
    PromptBudget,
    ExpertRouting,
    TestOrder,
    BugFixOrder,
    DatasetSampling,
    AdapterSelection,
    CacheEviction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AruaruLlmPlan {
    pub name: &'static str,
    pub hardware_tier: HardwareTier,
    pub core_policy: &'static str,
    pub layers: Vec<TheoryLayer>,
    pub learning_modes: Vec<LearningMode>,
    pub optimizer_use_cases: Vec<OptimizerUseCase>,
    pub must_not_claim: Vec<&'static str>,
    pub safety_gates: Vec<&'static str>,
    pub outputs: Vec<&'static str>,
}

pub fn build_aruaru_llm_plan(hardware_tier: HardwareTier) -> AruaruLlmPlan {
    let mut learning_modes = vec![
        LearningMode::NoLearning,
        LearningMode::MemoryOnly,
        LearningMode::RetrievalAugmented,
        LearningMode::AdapterCandidate,
    ];
    if hardware_tier != HardwareTier::Gt730CpuFirst {
        learning_modes.push(LearningMode::ApprovedFineTune);
    }

    AruaruLlmPlan {
        name: "Aruaru Folding-SBM Continual Learning Theory",
        hardware_tier,
        core_policy: "Small local models are only one component; aruaru-llm combines folding compression, sparse routing, quantum-inspired planning, RAG memory, adapter candidates, evaluation gates and human approval.",
        layers: vec![
            TheoryLayer::FoldingCompression,
            TheoryLayer::SparseExpertRouting,
            TheoryLayer::QuantumInspiredOptimizer,
            TheoryLayer::ContinualLearningMemory,
            TheoryLayer::AdapterFineTuning,
            TheoryLayer::EvaluationGate,
            TheoryLayer::HumanApproval,
        ],
        learning_modes,
        optimizer_use_cases: vec![
            OptimizerUseCase::PromptBudget,
            OptimizerUseCase::ExpertRouting,
            OptimizerUseCase::TestOrder,
            OptimizerUseCase::BugFixOrder,
            OptimizerUseCase::DatasetSampling,
            OptimizerUseCase::AdapterSelection,
            OptimizerUseCase::CacheEviction,
        ],
        must_not_claim: vec![
            "Do not claim one GPU literally equals a 100,000 GPU supercomputer.",
            "Do not overwrite model weights from private user data without explicit consent.",
            "Do not train on API outputs or third-party content unless the license and terms allow it.",
            "Do not accept a learning update unless it passes regression tests and rollback checks.",
        ],
        safety_gates: vec![
            "secret scan before any learning dataset is stored",
            "license and provenance check before dataset admission",
            "benchmark before and after adapter candidate creation",
            "rollback if coding, Japanese, safety or project tests regress",
            "human approval before persistent fine-tune or adapter activation",
        ],
        outputs: vec![
            "data/aruaru_llm/memory/latest.json",
            "data/aruaru_llm/adapters/candidates/",
            "data/aruaru_llm/evals/history/",
            "reports/aruaru_llm_learning.md",
            "reports/aruaru_llm_redmine.md",
        ],
    }
}

pub fn aruaru_llm_markdown() -> String {
    let plan = build_aruaru_llm_plan(HardwareTier::Gt730CpuFirst);
    let layer_text = plan
        .layers
        .iter()
        .map(TheoryLayer::label)
        .collect::<Vec<_>>()
        .join(", ");
    let learning_text = plan
        .learning_modes
        .iter()
        .map(LearningMode::risk_label)
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "# aruaru-llm Folding-SBM Continual Learning Theory\n\n\
## Core policy\n\n{}\n\n\
## Layers\n\n{}\n\n\
## Learning risk levels\n\n{}\n\n\
## Non-negotiable safety\n\n- {}\n\n\
## Practical GT730 rule\n\nGT730 class machines use CPU-first retrieval, compression, routing and evaluation. They may create adapter candidates, but persistent fine-tuning requires explicit approval and enough hardware.\n",
        plan.core_policy,
        layer_text,
        learning_text,
        plan.safety_gates.join("\n- "),
    )
}

pub fn quality_gate_smoke_check() -> bool {
    let gt730 = build_aruaru_llm_plan(HardwareTier::Gt730CpuFirst);
    let gpu24 = build_aruaru_llm_plan(HardwareTier::TwentyFourGbGpu);
    let tier12 = build_aruaru_llm_plan(HardwareTier::TwelveGbGpu);
    let multi = build_aruaru_llm_plan(HardwareTier::MultiGpuWorkstation);
    let markdown = aruaru_llm_markdown();

    let all_layers = [
        TheoryLayer::FoldingCompression,
        TheoryLayer::SparseExpertRouting,
        TheoryLayer::QuantumInspiredOptimizer,
        TheoryLayer::ContinualLearningMemory,
        TheoryLayer::AdapterFineTuning,
        TheoryLayer::EvaluationGate,
        TheoryLayer::HumanApproval,
    ];
    let all_modes = [
        LearningMode::NoLearning,
        LearningMode::MemoryOnly,
        LearningMode::RetrievalAugmented,
        LearningMode::AdapterCandidate,
        LearningMode::ApprovedFineTune,
    ];
    let all_optimizer_cases = [
        OptimizerUseCase::PromptBudget,
        OptimizerUseCase::ExpertRouting,
        OptimizerUseCase::TestOrder,
        OptimizerUseCase::BugFixOrder,
        OptimizerUseCase::DatasetSampling,
        OptimizerUseCase::AdapterSelection,
        OptimizerUseCase::CacheEviction,
    ];

    gt730.hardware_tier == HardwareTier::Gt730CpuFirst
        && !gt730.learning_modes.contains(&LearningMode::ApprovedFineTune)
        && gpu24.learning_modes.contains(&LearningMode::ApprovedFineTune)
        && tier12.layers.len() == all_layers.len()
        && multi.optimizer_use_cases.len() == all_optimizer_cases.len()
        && all_modes.iter().any(|mode| mode.risk_label().contains("approval"))
        && markdown.contains("Folding-SBM")
        && gt730.must_not_claim.iter().any(|rule| rule.contains("100,000 GPU"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gt730_does_not_enable_persistent_fine_tune_by_default() {
        let plan = build_aruaru_llm_plan(HardwareTier::Gt730CpuFirst);
        assert!(plan.learning_modes.contains(&LearningMode::RetrievalAugmented));
        assert!(!plan.learning_modes.contains(&LearningMode::ApprovedFineTune));
    }

    #[test]
    fn larger_gpu_can_create_approved_fine_tune_path() {
        let plan = build_aruaru_llm_plan(HardwareTier::TwentyFourGbGpu);
        assert!(plan.learning_modes.contains(&LearningMode::ApprovedFineTune));
    }

    #[test]
    fn theory_uses_folding_and_quantum_inspired_layers() {
        let plan = build_aruaru_llm_plan(HardwareTier::TwelveGbGpu);
        assert!(plan.layers.contains(&TheoryLayer::FoldingCompression));
        assert!(plan.layers.contains(&TheoryLayer::QuantumInspiredOptimizer));
        assert!(plan.optimizer_use_cases.contains(&OptimizerUseCase::BugFixOrder));
    }

    #[test]
    fn markdown_contains_safety_and_non_exaggeration_policy() {
        let markdown = aruaru_llm_markdown();
        assert!(markdown.contains("GT730"));
        assert!(markdown.contains("human approval"));
        assert!(quality_gate_smoke_check());
    }
}
