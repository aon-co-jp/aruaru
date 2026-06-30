# aruaru-core / aruaru-ai-core / aruaru-ai の境界

## 問題点

`aruaru` 関連機能をすべて一つにまとめると、AIを使う機能とAIを使わない機能の境界が曖昧になります。

その結果、以下の問題が起きます。

- 非AI機能までAI依存になる
- 中心機能の責任範囲が曖昧になる
- プラグイン分割が難しくなる
- BUG修正時に影響範囲が広がる
- 古いPCや低スペック環境で動かしにくくなる

## 原因

`aruaru-core`、`aruaru-ai-core`、`aruaru-ai` の役割を分離していないことが原因です。

## 修正方針

以下の3層に分けます。

```text
aruaru-core
  = AIを使わない中心機能

aruaru-ai-core
  = AIを使う中心機能

aruaru-ai
  = 利用者向けAIアプリ・AI操作画面
```

## 確定仕様

`aruaru-core` は、AIを使わない aruaru 関連の中心的な機能プラグインの集合である。

`aruaru-ai-core` は、aruaru 関連の中心的な機能プラグインのうち、AIを伴うものの集合であり、`aruaru-ai` の中核AI制御基盤である。

`aruaru-ai-core` は、利用者の依頼内容を解析し、ChatGPT、Grok、Claude、Gemini、DeepSeek、Qwen、ローカルLLMなどから最適なAIを自動選択する。

AIは固定せず、Rust、Poem、WunderGraph Cosmo、TypeScript、PostgreSQL、分散DB、VersionlessAPI、Git型DB履歴、UI設計、BUG修正、セキュリティ確認など、作業内容ごとに最適なAIを切り替える。

また、手動選択も可能とする。
ユーザーは「自動」「ChatGPT」「Grok」「Claude」「Gemini」「DeepSeek」「Qwen」「ローカルLLM」「複数AI比較」から選択できる。

AIの強み・弱み・メリット・デメリットは固定値ではなく、公式ドキュメント、GitHub、API情報、料金、ベンチマーク、実行ログ、コンパイル成功率、BUG修正成功率、ユーザー評価をもとに自動更新する。

`aruaru-ai-core` は、AIの出力をそのまま採用せず、品質ゲートを通過した結果のみを `aruaru-ai` へ返す。

## 確認方法

- AIを使わない機能は `aruaru-core` に入っているか
- AIを使う中心機能は `aruaru-ai-core` に入っているか
- 利用者向けAI画面・操作機能は `aruaru-ai` に入っているか
- AI選択と品質ゲートが `aruaru-ai-core` に集約されているか
