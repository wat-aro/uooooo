# Dependency Refresh Plan

最終更新: 2026-02-22

## Goal
- Rust 本体・依存クレート・CI(Action) を最新に追従する。
- 既存ユーザー互換よりも最新追従を優先する。

## Scope
- `Cargo.toml` の direct dependencies / dev-dependencies
- `Cargo.lock` の transitive dependencies
- `.github/workflows/ci.yml` と `.github/actions/cache_cargo/action.yml` の Action バージョン
- 依存更新後に必要なコード修正・テスト修正

## Non-Goals
- 既存ユーザー向けの後方互換性維持
- 新機能開発

## Plan

### 1. Baseline 固定
- [x] `cargo fmt --all`
- [x] `cargo test`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 現状の依存グラフを記録（`cargo tree`）

完了条件:
- [x] 現在の main 相当がグリーンで再現可能

### 2. 依存更新の棚卸し
- [x] outdated 一覧を取得（`cargo update` の差分と `cargo tree` で代替）
- [x] direct dependencies の更新可否を分類（patch/minor/major）
- [x] `assert_cmd` の git 依存（`branch = "fix-stdin"`）を継続するか判定

完了条件:
- [x] 更新対象クレートと必要な追従修正が一覧化されている

### 3. クレート更新（小さく分割）
- [x] patch/minor を優先して更新
- [x] major は 1 クレートずつ更新し、都度 `cargo test` を実施
- [x] `assert_cmd` は以下のどちらかに収束
  - [x] crates.io 版へ移行
  - [ ] git 依存を commit pin に変更し、理由を明記

完了条件:
- [x] `Cargo.toml` / `Cargo.lock` が最新追従状態（major 含む）
- [x] テストが通る

### 4. CI 依存更新
- [x] `actions/checkout` を `v4` へ更新
- [x] `actions/cache` を `v4` へ更新
- [x] Rust コンポーネント追加手順が最新構成で動作するか確認

完了条件:
- [x] CI 定義が deprecated 警告なく動作

### 5. 回帰検証
- [x] `cargo fmt --all`
- [x] `cargo test`
- [x] `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 主要フィクスチャ実行確認
  - [x] `tests/examples/ABC.uooooo`
  - [x] `tests/examples/hello_world.uooooo`
  - [x] `tests/examples/getchar.bf` 相当の入力系ケース

完了条件:
- [x] 最新依存環境で主要ユースケースが成立する

### 6. 継続運用（再放置防止）
- [x] Dependabot (`github-actions`, `cargo`) を週次で有効化
- [x] 更新運用ルールを README か docs に追記
  - [x] 月次: `cargo update` + test/clippy
  - [x] 四半期: major 更新レビュー

完了条件:
- [x] 「次回も自動で更新通知が来る」状態

## Acceptance Criteria
- [x] ローカルで `fmt/test/clippy` がすべて成功
- [ ] CI が成功
- [x] 依存更新理由と既知の非互換点（もしあれば）がドキュメント化されている

## Recommended Execution Order
1. Baseline 固定
2. 棚卸し
3. クレート更新
4. CI 更新
5. 回帰検証
6. 継続運用設定
