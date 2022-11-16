    // 取り敢えず以下のコードを置く

    extern crate adder;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, adder::add_two_iv(2));
    }

    // コードの頂点に `extern crate adder` とあるが、これは各々ライブラリをインポートしている
    // 今回の場合は adderライブラリをインポートしている

    // tests/integration_test.rs のあらゆるコードは #[cfg(test)]注釈する必要がない
    // Cargo が testsディレクトリを特別に扱い、cargo testを走らせた時にのみこのディレクトリのファイルをコンパイルする:

    // $ cargo test
    //    Compiling adder v0.1.0 (...\adder)
    //     Finished test [unoptimized + debuginfo] target(s) in 0.30s
    //      Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 1 test
    // test tests_xvi::internal ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    //      Running tests\integration_test.rs (target\debug\deps\integration_test-cc68398b02743d86.exe)
    //
    // running 1 test
    // test it_adds_two ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    //      Doc-tests adder
    //
    // running 0 tests
    //
    // test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    
    // 大きく分けて3つの区域で出力がされている
    // 単体テスト, 結合テスト, ドックテストをです
    // 単体テストの部分は lib.rs にある tests_xvi::internalテストです

    // 結合テストは `Running tests\integration_test.rs` より始まる行からです
    // `Doc-tests adder` という区域が始まる直前に、結合テストのまとめのサマリー行がある

    // 単体テストの関数を追加したら 単体テスト区域のテスト結果の行が増えたのと同じく、
    // 結合テストファイルにテスト関数を追加すると、そのファイルの区域に結果の行が増えることになる
    // 結合テストファイルはそれぞれ独自の区域があるため、testsディレクトリにのテストファイルを増やせば、
    // 結合テストの区域が増えることになる
