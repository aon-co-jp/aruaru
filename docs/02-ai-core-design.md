# aruaru-ai-core 設計書

## 目的

`aruaru-ai-core` は、AIを使う中心機能を集約し、`aruaru-ai` の中核AI制御基盤として動作します。

単一AI固定ではなく、作業内容に応じて最適なAIを自動選択し、必要に応じて手動選択や複数AI比較も可能にします。

## 対応AI候補

- ChatGPT
- Grok
- Claude
- Gemini
- DeepSeek
- Qwen
- ローカルLLM
- 複数AI比較

注: `Grock` という表記ゆれは `Grok` として扱います。

## AI自動選択対象

- Rust
- Poem
- WunderGraph Cosmo
- TypeScript
- PostgreSQL
- 分散DB
- VersionlessAPI
- Git型DB履歴
- UI設計
- BUG修正
- セキュリティ確認
- テスト生成
- ドキュメント生成

## モジュール構成案

```text
aruaru-ai-core/
├─ ai-router
├─ ai-score-engine
├─ ai-crawler
├─ ai-model-registry
├─ ai-task-classifier
├─ ai-fallback-manager
├─ ai-manual-selector
├─ ai-auto-selector
└─ ai-quality-gate
```

## ai-task-classifier

利用者の依頼内容を分類します。

分類例:

- 仕様設計
- BUG修正
- Rust実装
- Poem実装
- WunderGraph Cosmo設計
- PostgreSQL設計
- 分散DB設計
- セキュリティ確認
- テストコード生成
- UI/UX改善
- ドキュメント生成

## ai-score-engine

AIごとの適性を点数化します。

```text
score =
  技術適合度
+ 最新情報対応力
+ コード品質
+ BUG修正成功率
+ コンパイル成功率
+ セキュリティ安全性
+ コスト
+ 速度
+ 長文処理力
```

## ai-router

最適なAIへ依頼を振り分けます。

例:

```text
RustのBUG修正:
  ChatGPT + Claude

Poem公式Docs確認:
  Web検索対応AI + ChatGPT

WunderGraph Cosmo設計:
  Grok / Gemini / ChatGPT 比較

重大BUG:
  複数AI比較 + 品質ゲート必須

軽微修正:
  高速AIまたはローカルLLM
```

## ai-fallback-manager

AIが失敗した場合に別AIへ切り替えます。

```text
1回目: 最適候補AI
失敗: 次点AI
さらに失敗: 複数AI比較
さらに失敗: 公式Docs再確認 + 人間確認
```

## ai-crawler

AIの強み・弱み・メリット・デメリットを固定情報にせず、以下から自動更新します。

- 公式ドキュメント
- GitHub
- API仕様
- 料金
- コンテキスト長
- Rust対応力
- Poem対応力
- WunderGraph Cosmo対応力
- TypeScript対応力
- PostgreSQL対応力
- BUG修正能力
- セキュリティ修正能力
- 出戻り率
- 生成コードのコンパイル成功率

## ai-quality-gate

AIの回答をそのまま採用しません。

品質ゲート例:

- 公式Docs確認
- コンパイル確認
- `cargo check`
- `cargo test`
- TypeScript型チェック
- DBマイグレーション検証
- セキュリティ確認
- 回帰テスト
- 仕様との一致確認

## 確認方法

- AI選択が固定ではなくタスクごとに変わるか
- 手動選択が可能か
- 複数AI比較が可能か
- 公式情報の自動更新ができる設計か
- 品質ゲートを通過しない回答を採用しないか
