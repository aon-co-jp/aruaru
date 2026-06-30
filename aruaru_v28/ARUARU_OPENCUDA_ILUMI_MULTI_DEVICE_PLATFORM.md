# OpenCUDA iLumi Multi-Device Platform

## 目的

OpenCUDA iLumi は、Windows上で NVIDIA / AMD / Intel GPU と、PC・タブレット・スマホのCPU、さらに Copilot+ PC世代のNPUをまたいで、aruaru-ai / aruaru-web / aruaru-desktop / aruaru-llm の処理を賢く振り分ける実行基盤です。

## 対応対象

### GPU 3種類

- NVIDIA GPU
  - CUDA
  - DirectML
  - Vulkan
  - ONNX Runtime Execution Provider
- AMD GPU
  - ROCm / HIP
  - DirectML
  - Vulkan
  - ONNX Runtime Execution Provider
- Intel GPU
  - oneAPI / Level Zero
  - DirectML
  - Vulkan
  - ONNX Runtime Execution Provider

### CPU

- PC CPU
- タブレットCPU
- スマホCPU

CPUは、長文ログ、RAG、差分履歴、品質ゲート、CPU fallback、軽量推論を担当します。

### NPU

- Copilot+ PC NPU
- Qualcomm / Intel / AMD / Arm系NPU
- スマホNPU
- Apple Neural Engine / Core ML 系
- Android NNAPI / QNN 系

NPUは、低消費電力の推論、Embedding、音声/画像/小型LLM補助、ローカルAI機能を担当します。

## 基本方針

OpenCUDA iLumiは、GPUやNPUのVRAMやメモリを魔法のように合体させるものではありません。

正しい方針は、各デバイスの得意分野を判定して、処理を分割・振り分け・失敗時フォールバックすることです。

## 役割分担

| デバイス | 主な役割 |
|---|---|
| NVIDIA GPU | CUDA学習、LoRA/QLoRA候補、主推論 |
| AMD GPU | Vulkan/ROCm実験、推論、RAG、評価 |
| Intel GPU | DirectML/oneAPI推論、評価、補助処理 |
| PC CPU | 長文ログ、RAG DB、品質ゲート、CPU fallback |
| タブレットCPU | 軽量推論、要約、キャッシュRAG |
| スマホCPU | 小型モデル、音声/チャット補助、ローカル要約 |
| Copilot+ NPU | 低消費電力ONNX推論、Embedding、再ランキング |

## バックエンド

- CUDA Backend
- ROCm / HIP Backend
- oneAPI / Level Zero Backend
- DirectML Backend
- DirectCompute Backend
- Vulkan Backend
- ONNX Runtime Execution Provider Backend
- Android NNAPI / QNN Backend
- Apple Core ML Backend
- CPU Fallback Backend

## aruaru-llmとの関係

aruaru-llmは、巨大モデルを無理に1台で事前学習しません。

代わりに、以下を組み合わせます。

- OSSローカルLLM
- Folding Plugin
- SBM Optimizer Plugin
- RAG Memory
- Quality Gate
- Adapter / LoRA Candidate
- OpenCUDA iLumi Scheduler
- Human Approval Gate
- Rollback Manager

## 禁止事項

- NVIDIA + AMD + Intel のVRAMを単純合算して、1枚GPUのように扱わない
- Copilot+ NPUを本格FineTune用として扱わない
- スマホやタブレットで重い学習を自動実行しない
- DirectXそのものを改造する前提にしない
- ライセンス不明のモデルを自動更新しない
- 人間承認なしにAdapter/LoRAを本番反映しない

## v25確認条件

- NVIDIA / AMD / Intel GPU対応が明記されている
- PC / タブレット / スマホ CPU対応が明記されている
- Copilot+ NPU対応が明記されている
- DirectML / Vulkan / CUDA / ROCm / oneAPI / CPU fallback が明記されている
- 共有VRAMではなくタスク分散であることが明記されている
