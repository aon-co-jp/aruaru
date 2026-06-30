# aruaru-ai / aruaru-llm OpenCUDA iLumi 0.3.5 参照統合方針

## 目的

OpenCUDA iLumi 0.3.5 は、aruaru-llm の将来構想を空想だけにしないための、実際に動いている段階的プロトタイプとして扱います。

## 0.3.5 の現在地

- CPU バックエンドは GPU なしで検証できる基盤。
- VulkanMock は、GPUなしでも SPIR-V / OmniIR 経路の契約を検証するための安全装置。
- 実Vulkan `vector_add` は、NVIDIA / AMD / Intel へ広げる前の最小実機GPU証明。
- `vulkan_info` は、GPUスケジューラの前提診断に使える。
- `.cmd` テストスクリプトは、Windows PowerShell 実行ポリシーで止まる環境への現実的対策。

## aruaru-llm での使い方

```text
aruaru-ai / aruaru-web / aruaru-desktop
  ↓
aruaru-llm
  ↓
OSS Local LLM
  ↓
OpenCUDA iLumi 0.3.5 reference layer
  ├─ CPU fallback
  ├─ OmniIR
  ├─ VulkanMock
  ├─ real Vulkan vector_add
  ├─ vulkan_info device detection
  └─ Windows .cmd quality scripts
```

## 今は言ってはいけないこと

```text
OpenCUDA 0.3.5 はCUDA完全互換である
OpenCUDA 0.3.5 はLLM学習に対応済みである
NVIDIA + AMD のVRAMを完全共有できる
実VulkanでGEMM/Attention/量子化が完成済みである
```

## 次に進めるべき順番

```text
1. vulkan_info の表示強化
2. glslc --version 表示
3. cargo clippy --workspace --all-targets 警告削減
4. Vulkan matmul 最小実装
5. CPU matmul と Vulkan matmul の結果比較
6. GEMM / 量子化 / Attention の小型実験
7. aruaru-llm のRAG/評価/推論補助へ接続
```

## 方針

OpenCUDA iLumi 0.3.5 は、aruaru-llm の「DirectX/DirectML/Vulkan/CUDA/ROCm/CPU/NPU を束ねる将来構想」の土台です。  
ただし、0.3.5 の時点では **CPU + Mock + 最小実Vulkan** と正直に表示し、動いている範囲を過大評価しません。
