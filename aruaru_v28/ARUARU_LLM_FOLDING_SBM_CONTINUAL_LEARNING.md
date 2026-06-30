# aruaru-llm Folding-SBM Continual Learning Theory

## 1. 問題点

aruaru-ai / aruaru-web / aruaru-desktop の裏で動く aruaru-llm は、単に小さなオープンソースLLMを選ぶだけでは弱い。

必要なのは次の統合である。

- 長文・ログ・設計書・README・cargo errorを折り畳む圧縮理論
- DeepSeek系の MoE / MLA / 蒸留 / ルーティング思想を参考にした軽量推論
- 東芝SQBM+系の疑似量子・組合せ最適化思想を参考にした探索順序最適化
- 継続学習しながら賢くなる仕組み
- ただし、1枚GPUが本当に10万枚GPUと同等になるという誇張はしない
- API出力や第三者データを無断学習しない
- 品質劣化時にロールバックする

## 2. 原因

小型LLM単体は、以下で弱くなる。

- コンテキストが短い
- ログの重要箇所を拾い切れない
- BUG修正順序を誤る
- 仕様・過去判断・ユーザー方針を忘れる
- 学習データの品質が悪いと逆に劣化する
- GT730級PCでは大規模ファインチューニングが難しい

## 3. 修正方針

aruaru-llm は、次の独自理論として実装する。

**Aruaru Folding-SBM Continual Learning Theory**

構成は7層。

1. Folding Compression Layer  
   長文ログ、README、cargo error、設計書を要点グラフに折り畳む。

2. Sparse Expert Routing Layer  
   Rust / PowerShell / Poem / GraphQL / DATABASE / UI / 音響 / 政策など、専門ルートを分ける。

3. Toshiba SBM-Inspired Optimizer Layer  
   組合せ最適化として、BUG修正順、テスト順、プロンプト投入順、モデル選択順を最適化する。

4. Continual Learning Memory Layer  
   すぐ重みを書き換えず、まずRAGメモリ、仕様履歴、失敗履歴、成功パターンとして保存する。

5. Adapter Candidate Layer  
   十分なデータと許諾がある場合だけ、LoRA/Adapter候補を作る。

6. Evaluation Gate Layer  
   cargo test、clippy、回帰テスト、秘密情報スキャン、方針チェックを通過した候補だけ採用する。

7. Human Approval Layer  
   永続学習・ファインチューニング・本番反映は、人間承認を必須にする。

## 4. DeepSeek折り畳み理論のaruaru解釈

DeepSeek-V3は、MoE、MLA、MTP、FP8などの効率化思想を持つ大規模モデルとして知られる。aruaru-llmでは、これをそのまま複製するのではなく、次のように解釈する。

- 671B級の全パラメータを1枚GPUに載せる、とは言わない
- 必要な専門能力だけを選ぶ
- コンテキストを折り畳んで重要部分だけを渡す
- 高価なAPIや大きなローカルLLMは難所だけ使う
- 普段は小型ローカルモデル + RAG + ルール + テストで支える

## 5. Toshiba SBM-Inspired疑似量子最適化のaruaru解釈

東芝SQBM+は、量子コンピューターそのものではなく、Simulated Bifurcation Machine由来の量子インスパイアド組合せ最適化として扱う。

aruaru-llmでは次に使う。

- BUG修正順序
- cargo test / clippy / fmt の実行順序
- README変換対象の優先順位
- Promptに入れるファイル順
- RAG検索結果の採用順
- モデル選択順
- adapter候補の採用/不採用
- キャッシュ削除順

## 6. 継続学習システム

aruaru-llmは、次の順で賢くなる。

1. 失敗ログを保存する
2. 原因分類する
3. 修正成功例を記録する
4. RAGメモリに反映する
5. 次回のBUG修正計画に利用する
6. 十分な成功例だけadapter候補にする
7. 評価に合格した候補のみ有効化する
8. 劣化したら即ロールバックする

## 7. GT730級PCでの現実的な動作

GT730では、重い学習はしない。

- CPU-first
- 小型LLM
- RAGメモリ
- 折り畳み要約
- ルールベース品質ゲート
- テスト自動化
- APIは難所のみ任意利用

この設計なら、1枚GPU PCでも「巨大AIをそのまま動かす」のではなく、「巨大AI的な作業分解と記憶と最適化」を現実的に実行できる。

## 8. 禁止する誇張

aruaru-llmでは以下を禁止する。

- 1枚GPUが10万枚GPUスーパーコンピューターと完全同等と断定する
- 無断でAPI出力を学習データにする
- ユーザー秘密情報を学習する
- テスト未通過の学習結果を本番採用する
- 科学的未検証理論を確定技術扱いする

## 9. 実装ファイル

- `src/aruaru_llm_learning.rs`
- `ARUARU_LLM_FOLDING_SBM_CONTINUAL_LEARNING.md`
- `README.md`
- `DOWNLOAD_CONTENTS.md`
- `COMPILE_STATUS.md`
- `ROADMAP.md`

## 10. 合格条件

- GT730ではApprovedFineTuneをデフォルトOFF
- 24GB GPU以上ではApprovedFineTune候補を許可
- FoldingCompressionとQuantumInspiredOptimizerが両方存在
- HumanApprovalが必須
- 100,000 GPU同等という誇張を禁止
- `cargo test aruaru_llm_learning` PASS
- `cargo clippy --all-targets -- -D warnings` PASS
