# aruaru

`aruaru` は、アプリ・Webサイト開発、運用、DB、AI支援、品質ゲートを統合するための構想リポジトリです。

現時点では、まず設計書・役割分担・中心モジュールの境界を整理し、GitHub に push できる最小構成としてまとめています。

## 中心構成

```text
aruaru
├─ aruaru-core       # AIを使わない中心機能プラグイン群
├─ aruaru-ai-core    # AIを伴う中心機能プラグイン群 / aruaru-ai の中核AI制御基盤
├─ aruaru-ai         # 利用者向けAI機能・AI操作画面
└─ docs              # 設計書・仕様書
```

## aruaru-core

`aruaru-core` は、AIを使わない aruaru 関連の中心的な機能プラグインの集合です。

対象例:

- 設定管理
- プラグイン管理
- ログ管理
- プロジェクト管理
- 品質ゲート実行基盤
- Git連携
- DB接続管理
- バックアップ管理
- HTTPS / ドメイン管理
- CLI / GUI 共通基盤

## aruaru-ai-core

`aruaru-ai-core` は、aruaru 関連の中心的な機能プラグインのうち、AIを伴うものの集合であり、`aruaru-ai` の中核AI制御基盤です。

主な役割:

- 利用者の依頼内容解析
- AI自動選択
- AI手動選択
- 複数AI比較
- 公式Docs / GitHub / API情報の自動クロール
- AIごとの強み・弱み・メリット・デメリット更新
- BUG修正成功率 / コンパイル成功率 / ユーザー評価の蓄積
- 品質ゲートを通過した結果だけを aruaru-ai へ返す

## aruaru-ai

`aruaru-ai` は、利用者が実際に操作するAI機能・AI画面・AIアシスタント層です。

`aruaru-ai-core` がAI選択や品質判定を行い、`aruaru-ai` はその結果を利用者へわかりやすく提示します。

## 開発方針

- Rust + Poem を重要候補とする
- Tauri と REST API は基本仕様から外す
- JavaScript ではなく TypeScript を使用する
- WunderGraph Cosmo を REST API の代替として扱う
- PostgreSQL 連携を重視する
- aruaru-db 独自DB、分散DB、VersionlessAPI、移植変換、分散自動バックアップ、Git型DB履歴を構想に含める
- AIの出力をそのまま信じず、品質ゲートを必ず通す

## 現在の状態

このリポジトリは初期設計段階です。
まずは設計思想、モジュール境界、今後の実装方針を整理しています。

## 個人的背景

このプロジェクトは、まだ十分な開発資金や高性能PCがない状態から始めています。現時点では生活が厳しく、PCも GT 730 搭載環境のままですが、その制約を前提に、低コスト・省メモリ・段階的に成長できる開発基盤を目指します。

そのため、最初から重い構成にせず、古いPCでも扱える軽量な設計、ローカルLLMとクラウドAIの併用、必要に応じた段階的拡張を重視します。
