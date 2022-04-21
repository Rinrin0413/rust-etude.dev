#![allow(unused)]
    // 1972年のエッセイ「謙虚なプログラマ」にてエドガー・ダイクストラ氏は以下のように述べている
    // 「プログラムのテストは、バグの存在を示すには非常に効率的な手法であるが、 バグの不在を示すには望み薄く不適切である」
    // これは テストをするなというわけではない
    
    // プログラムの正当性は どこまで自分のコードが期待した動きをしているかです
    // Rust は プログラムの正当性に着目して設計されているが、 正当性は複雑であるため、単純に証明することはできない
    // Rust の型システムは この重荷の多くを肩代わりしてくれるが、その型システムは全種類の不当性を捕えてくれるわけではない
    // 故に Rust では 言語内で自動化されたソフトウェアテストを書くことを サポートしている

    // 例えば、渡された数値に2を足す add_two関数を書くとする
    // この関数のシグニチャは、引数に整数を取って 結果として整数を返す
    // この関数を実装しコンパイルすると、コンパイラは型チェックと借用チェックを行う
    // 例えば、整数以外の値や 無効な参照を この関数に渡していないかなどを確かめる
    // しかしながらコンパイラは 開発者の意図した動作をする関数になっているかはわからない
    // つまり、引数に2を加算していることを保証したい...
    // そんな贵樣に！ テストがあるのです。
    
    // 例えば add_two関数に3を渡した際、戻り値は5であると主張するテストを書くことができる
    // コードを変更した際に これらのテストを走らせて、定義した正当な振る舞いが変わっていないことを確認できる
    
    // テストは複雑なスキルです
    // より良いテストの書き方を多方面から学ぶには1章だけではできないが、
    // まずはともかく Rust のテスト機構のメカニズムについて学ぶ
    // また、テストを書くのに使えるアノテーション(注釈)とマクロについて、
    // テスト実行用に提供されているオプションと動作、
    // 更にテストをユニットテストや統合テストに体系化する方法についても学ぶ
pub fn writing_tests() {
// DOC.11-1
  // テストの記述法
    // テストとは、コードが期待された動作をしていること実証する Rust の関数
    // テスト関数の本体は 典型的に以下の3つの動作を行う
    //   1. 必要なデータや状態のセットアップ
    //   2. テスト対象のコードの実行
    //   3. 結果が想定通りであることを断定(以下、アサーションと呼称)
    // このようなテストを書くために用意されている、Rust の機能を見ていく
    // これには test属性, いくつかのマクロ. should_panic属性が含まれる

  // テスト関数の構成
    // 最も単純なテストは test属性での注釈(アサーション)です
    // 属性とは、Rustコードの部品に関するメタデータ
    // 例えば、構造体定義に使用した derive属性です。 
    // 関数をテスト関数に変えるには fn の前に #[test] を付け加える
    // そんで cargo test でテストを実行したら、
    // コンパイラは test属性で注釈された関数 を走らせる為の テスト用バイナリをビルドし、
    // 各テスト関数が通過したか失敗したかを報告する

    // 新規ライブラリプロジェクトを Cargo で作ると、テスト関数付きのモジュールが自動的に生成される
    // そのおかげで 新プロジェクトを始める每にテスト関数の文法などをいちいち検索しなくて済む
    // ここに好きなだけテスト関数やテストモジュールを追加すればいいというわけだ

    // まずは、実際にはコードをテストしない 自動生成されたテンプレのテストで実験して、テストの動作の性質をいくらか学ぶ
    // その後で、以前書いたコードをテストしてみる

    // adder という新しいライブラリプロジェクトを生成する
    // $ cargo new adder --lib
    //      Created library `adder` project

    //✔以降は ../../adder/src/lib.rs を参照
    use adder;
}

pub fn how_to_run() {
// DOC.11-2
 // テストの実行のされ方を制御する
    // cargo run がコードをコンパイルして 出来たバイナリを走らせるのと同じく、
    // cargo test はコードをテストモードでコンパイルして 出来上がったテストバイナリを実行している
    // コマンドラインオプションを指定して cargo test の挙動を変更することができる
    // 例えば、cargo test でコンパイルされた バイナリの既定動作では、テストを全て並行に実行している
    // 実行中に生成された出力をキャプチャして 出力が表示されるのを防ぎ、
    // 最終的に出力を読みやすくする

    // コマンドラインオプションには cargo test に干渉するものと、
    // 出来上がったテストバイナリに干渉するものがある
    // この2種を区別するために、cargo test にかける引数を -- という区分記号の後に列挙し、
    // それからテストバイナリにかかる引数を列挙する
    // `cargo test --help` で走らせると cargo test で使えるオプションが表示され、
    // `cargo test -- --help` を走らせると 区分記号-- の後に使えるオプションが表示される

    // $ cargo test --help
    // cargo.exe-test 
    // Execute all unit and integration tests and build examples of a local package
    //
    // USAGE:
    //     cargo.exe test [OPTIONS] [TESTNAME] [-- <args>...]
    //
    // OPTIONS:
    //     -q, --quiet                      Display one character per test instead of one line
    //         --lib                        Test only this package's library unit tests
    //         --bin <NAME>...              Test only the specified binary
    //         --bins                       Test all binaries
    //         --example <NAME>...          Test only the specified example
    //         --examples                   Test all examples
    //         --test <NAME>...             Test only the specified test target
    //         --tests                      Test all tests
    //         --bench <NAME>...            Test only the specified bench target
    //         --benches                    Test all benches
    //         --all-targets                Test all targets
    //         --doc                        Test only this library's documentation
    //         --no-run                     Compile, but don't run tests
    //         --no-fail-fast               Run all tests regardless of failure
    //     -p, --package <SPEC>...          Package to run tests for
    //         --workspace                  Test all packages in the workspace
    //         --exclude <SPEC>...          Exclude packages from the test
    //         --all                        Alias for --workspace (deprecated)
    //     -j, --jobs <N>                   Number of parallel jobs, defaults to # of CPUs
    //     -r, --release                    Build artifacts in release mode, with optimizations
    //         --profile <PROFILE-NAME>     Build artifacts with the specified profile
    //         --features <FEATURES>...     Space or comma separated list of features to activate
    //         --all-features               Activate all available features
    //         --no-default-features        Do not activate the `default` feature
    //         --target <TRIPLE>...         Build for the target triple
    //         --target-dir <DIRECTORY>     Directory for all generated artifacts
    //         --manifest-path <PATH>       Path to Cargo.toml
    //         --ignore-rust-version        Ignore `rust-version` specification in packages
    //         --message-format <FMT>...    Error format
    //         --unit-graph                 Output build graph in JSON (unstable)
    //         --future-incompat-report     Outputs a future incompatibility report at the end of the build
    //     -v, --verbose                    Use verbose output (-vv very verbose/build.rs output)
    //         --color <WHEN>               Coloring: auto, always, never
    //         --frozen                     Require Cargo.lock and cache are up to date
    //         --locked                     Require Cargo.lock is up to date
    //         --offline                    Run without accessing the network
    //         --config <KEY=VALUE>...      Override a configuration value (unstable)
    //     -Z <FLAG>...                     Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    //     -h, --help                       Prints help information
    //
    // ARGS:
    //     <TESTNAME>    If specified, only run tests containing this string in their names
    //     <args>...     Arguments for the test binary
    //
    // Run `cargo help test` for more detailed information.
    // Run `cargo test -- --help` for test binary options.
}