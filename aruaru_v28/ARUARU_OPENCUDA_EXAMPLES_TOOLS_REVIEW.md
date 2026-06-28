# ARUARU OpenCUDA 0.3.5 Examples / Tools Review

## 目的

OpenCUDA 0.3.5 の追加アップロードに含まれる examples / shader / tools を、aruaru-llm 側の統合計画へ反映する。

今回の対象は主に以下。

```text
examples/matmul/Cargo.toml
examples/matmul/src/main.rs
examples/vector_add_vulkan_real/shaders/vector_add.comp
tools/compile-vulkan-shaders.ps1
tools/compile-vulkan-shaders.sh
tools/test-v0.3.5.ps1
```

## 判断

OpenCUDA はすでに `vector_add` だけでなく、CPU `matmul` の参照実装を持っている。したがって、次の現実的な目標は **Vulkan matmul の最小実装** とし、CPU matmul と結果を比較すること。

## 重要な安全方針

```text
vector_add 成功
→ 実Vulkan最小経路の証明

CPU matmul 成功
→ 次のVulkan matmulの正解基準

Vulkan matmul 成功
→ GEMMへ進む条件

GEMM成功
→ 量子化/Attentionへ進む条件

Attention/Quantization成功
→ LLM推論補助へ進む条件
```

まだ次は言わない。

```text
CUDA完全互換完成
NVIDIA+AMD共有VRAM完成
LLM学習対応済み
GEMM/Attention完成
```

## v0.3.6 でやるべきこと

```text
1. glslc --version を compile-vulkan-shaders 実行前に表示
2. vulkan_info に queue family index / device type / API version / driver version を表示
3. cargo clippy --workspace --all-targets の警告削減
4. CPU matmul を正解基準として固定
5. Vulkan matmul の最小実装
6. CPU matmul と Vulkan matmul の結果比較
```

## aruaru-llm 統合方針

aruaru-llm では、OpenCUDAをいきなりLLM学習基盤として扱わない。

まずは以下を個別ゲートにする。

```text
CPU backend gate
OmniIR gate
VulkanMock gate
Real Vulkan vector_add gate
CPU matmul gate
Future Vulkan matmul gate
```

OpenCUDA iLumi は、これらのゲートを通った範囲だけを aruaru-llm から使う。

## 追加したRustモジュール

```text
src/opencuda_examples_tools_review.rs
```

このモジュールでは、アップロードされた examples / shader / tools を分類し、v0.3.6 で進めるべき順番を品質ゲート化した。
