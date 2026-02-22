# リポジトリガイドライン

## プロジェクト構成とモジュール配置
このリポジトリは Rust 2021 で構成された、小規模な CLI とインタプリタ本体のプロジェクトです。
- `src/lib.rs`: パーサ、命令セット、実行マシン、実行時エラー定義。
- `src/main.rs`: CLI エントリポイント（`uooooo [FILE]`）とファイル I/O。
- `tests/integrations/main.rs`: `assert_cmd` を使った統合テスト。
- `tests/integrations/utils.rs`: バイナリ起動用の共通テストヘルパー。
- `tests/examples/`: 実行フィクスチャ（`*.uooooo`）と対応する Brainf**k 参照（`*.bf`）。
- `.github/workflows/ci.yml`: CI チェック（`build`、`fmt`、`clippy`）。

## ビルド・テスト・開発コマンド
- `cargo build`: プロジェクトをビルドします。
- `cargo run -- tests/examples/ABC.uooooo`: サンプルプログラムでインタプリタを実行します。
- `cargo test`: 統合テストを実行します。
- `cargo fmt --all`: コードを整形します。
- `cargo clippy --all-targets --all-features -- -D warnings`: 警告をエラーとして lint を実行します。

PR 作成前に `cargo fmt --all` と `cargo clippy --all-targets --all-features -- -D warnings` を実行し、CI と同じ基準を満たしてください。

## コーディングスタイルと命名規約
- `rustfmt` を前提に、Rust の標準的なスタイル（4 スペースインデント）を守ってください。
- 関数・変数・モジュール・テスト名は `snake_case` を使用します。
- 列挙型や型名は `PascalCase` を使用します（例: `MachineError`、`Instruction` の各バリアント）。
- VM 実装では `next_ptr` や `jump_to_end` のように責務を分け、`panic` ではなく明示的なエラー返却を優先してください。

## テスト方針
- ユーザーから見える振る舞いは `tests/integrations/main.rs` に統合テストを追加します。
- テスト名は `abc_and_newline_next_ptr` のように、内容が分かる名前にします。
- 実行用フィクスチャは `tests/examples/` に配置します。
- 新しい命令や言語機能を追加する場合は、フィクスチャの追加/更新に加え、標準出力と成功/失敗の少なくとも 1 件の検証を入れてください。

## コミットとプルリクエストの指針
- 既存履歴に合わせて、短い命令形の件名を使ってください（例: `Add Read`、`Fix loop`）。
- 1 コミット 1 論理変更を基本にしてください。
- PR には次を含めてください。
  - 変更した振る舞いの要約。
  - 関連 Issue へのリンク（あれば）。
  - ローカルで実行したコマンド（`cargo test`、`cargo fmt`、`cargo clippy`）。
  - 影響を受けるフィクスチャ名（言語/ランタイム変更時）。
