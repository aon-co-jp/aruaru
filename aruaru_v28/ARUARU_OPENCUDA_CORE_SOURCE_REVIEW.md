# ARUARU OpenCUDA 0.3.5 Core Source Review

## 目的

今回アップロードされた OpenCUDA 0.3.5 の core / cpu / ir / vulkan 周辺ソースを、aruaru-ai / aruaru-llm / OpenCUDA iLumi 統合の実装参考として整理する。

## 確認した重要ソース

- `device.rs`: `GpuDevice` trait、`GpuVendor`、`DeviceInfo`、`LaunchConfig`
- `memory.rs`: `DevicePtr` と `DeviceBuffer` の二層メモリ、RAII解放、host/device copy
- `kernel.rs`: `KernelSource::{Native, SpirV, Ptx, OmniIr}`、`KernelArg`、`ResolvedArg`、`CompiledKernel`
- `registry.rs`: `DeviceRegistry`、登録済みデバイス、memory最大基準の `best_for_inference`
- `mock_device.rs` / `vulkan_mock.rs`: GPUなしでSPIR-V契約や拒否動作を確認するテスト
- `real.rs`: ashベースの最小実Vulkan Compute `vector_add` 経路
- `omniir_path.rs`: OmniIR vector_add を CPU Native / VulkanMock 契約に接続するテスト

## aruaru側で採用する設計

### 1. GpuDevice trait を aruaru-llm の下位契約にする

aruaru-llm は直接 NVIDIA / AMD / Intel を触るのではなく、OpenCUDA の `GpuDevice` 契約を通じて、CPU / Vulkan / 将来のCUDA / ROCm / oneAPIへ処理を渡す。

### 2. DevicePtr の device_id を絶対に残す

NVIDIA + AMD の2枚刺しでは、メモリを1つの共有VRAMと誤認してはいけない。`DevicePtr { addr, device_id }` は、どのGPUのメモリかを追跡する重要な安全装置である。

### 3. KernelSource をそのまま品質ゲートに使う

- `Native`: CPU fallback
- `SpirV`: Vulkan / Intel 系の実行候補
- `Ptx`: 将来のNVIDIA/CUDA系
- `OmniIr`: 将来の共通中間表現

バックエンドごとに受け付ける形式を限定し、未対応形式は明確に拒否する。

### 4. VulkanMock を軽視しない

Mockは偽物GPUではあるが、GPUがない貧乏開発環境でSPIR-V契約、エラー、引数、起動設定を崩さないために重要である。

### 5. 実Vulkan vector_add は最初の物理GPU証明

`real.rs` はまだ高性能ランタイムではないが、Vulkan loader / device / queue / buffer / shader module / pipeline / dispatch の最小経路として非常に重要。

## 次にやるべきこと

1. `vulkan_info` を aruaru のデバイス検出に接続する。
2. Vulkan `matmul` を最小実装する。
3. CPU matmul と Vulkan matmul の結果比較を必ず行う。
4. その後で GEMM / quantization / attention へ進む。
5. LLM推論補助は、matmul/GEMMが安定してから接続する。
6. LLM学習対応済みとは、まだ言わない。

## 禁止する過大表現

- CUDA完全互換が完成したとは言わない。
- NVIDIA + AMD のVRAMが自動共有されるとは言わない。
- vector_add成功だけでLLM学習対応済みとは言わない。
- Mock成功を実GPU性能とは言わない。

## aruaru品質ゲート

```powershell
cargo test opencuda_core_source_review
cargo test opencuda_035_reference
cargo test opencuda_ilumi_platform
cargo clippy --all-targets -- -D warnings
```
