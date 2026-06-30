# aruaru-ai-core

`aruaru-ai-core` は、aruaru 関連の中心機能プラグインのうち、AIを伴うものの集合です。

`aruaru-ai` の中核AI制御基盤として動作します。

## 役割

- 依頼内容解析
- AI自動選択
- AI手動選択
- 複数AI比較
- AI評価
- 公式Docs / GitHub / API情報の自動クロール
- AIの強み・弱み・メリット・デメリット更新
- BUG修正成功率の蓄積
- コンパイル成功率の蓄積
- 品質ゲート通過判定

## 対応AI候補

- ChatGPT
- Grok
- Claude
- Gemini
- DeepSeek
- Qwen
- ローカルLLM
- 複数AI比較

## 原則

AIの出力をそのまま採用せず、品質ゲートを通過した結果のみを `aruaru-ai` へ返します。
