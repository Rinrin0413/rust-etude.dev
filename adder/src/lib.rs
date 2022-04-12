#![allow(unused)]
    // 以下は 生成されたモジュール

    /* // 別の場所で被るので無効化
    // |== BEGIN ==|
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
    // |== END ==|
    */
    
    // 取り敢えずモジュールの方は無視し、関数に 着目してみる
    // #[test]注釈に注目してください。この属性は それがテスト関数であることを示すので、
    // テスト実行機はこの関数をテストとして扱うう
    // 更に testsモジュール内にテスト関数以外の関数を入れて、普通の関数として定義したり、
    // 共通の処理を行う手助けをしたりもできるので、#[test]属性でどの関数がテストかを示す必要がある

    // 関数本体では assert_eq!マクロをって 2 + 2 が 4 に等しいとアサーションしている
    // このアサーションは 典型的なテストの記述例を成している
    // 走らせてこのテストが通る(テストが成功する)ことを確かめる
    
    // $ cargo test
    //   Compiling adder v0.1.0 (..\adder)
    //    Finished test [unoptimized + debuginfo] target(s) in 0.33s
    //     Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 1 test
    // test tests::it_works ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // Cargo がテストをコンパイルして走らせました
    // Running の行の後から解読する
    // `running 1 test` : テストしたテストの数を示している。次の行、
    // `test tests::it_works ... ok` : テスト関数の名前とそのテストの実行結果(ここではok)を示している

    // で、テスト実行の総合的なまとめが次に出現する
    //  `test result: ok.` : 全テストが通ったか否かを示す
    //  `1 passed; 0 failed;` : 通過及び失敗したテストの数を示している
    //  `0 ignored;` : 無視するとして指定したテストの数を示している
    //    テストの無視についてはのちに学ぶ
    //  `0 measured;` : パフォーマンスを測定するベンチマークテスト用
    //    ベンチマークテストは今現在 nightly版Rust でのみ利用可能
    //  `0 filtered out;` : 実行するテストのフィルタがかかった数

    // その次の部分、
    //  `Doc-tests adder` : ドキュメンテーションテストの結果用
    //    まだドキュメンテーションテストはありませんが、
    //    コンパイラは APIドキュメントに現れる あらゆるコード例をコンパイルできる
    //    この機能により、ドキュメントとコードを同期することができる
    //    ドキュメンテーションテストの書き方についてはいずれ学ぶ
    //    今は Doc-tests出力は無視する

    // テスト名の変更でどうテスト出力が変わるか確かめてみる
    // it_works関数を explorationに改名してみる

    /*
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
    */
    
    // $ cargo test
    //   Compiling adder v0.1.0 (...\adder)
    //    Finished test [unoptimized + debuginfo] target(s) in 0.31s
    //     Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 1 test
    // test tests::exploration ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // テスト関数内でパニックが起こるとテストは失敗する
    // 各テストは新規スレッドで実行され、メインスレッドが テストスレッドが死んだと断定すると、
    // テストは失敗となる。panic!マクロで試してみる
    // 新規テスト関数another を見て下さい

    /* // 当然パニクるので無効化
    #[cfg(test)]
    mod tests {
        #[test]
        fn exploration() {
            assert_eq!(2 + 2, 4);
        }
        #[test]
        fn another() {
            panic!("パニック！");
        }
    }
    */
    // すると以下が吐かれる:

    // $ cargo test 
    //   Compiling adder v0.1.0 (...\adder)
    //    Finished test [unoptimized + debuginfo] target(s) in 0.65s
    //     Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 2 tests
    // test tests::another ... FAILED
    // test tests::exploration ... ok
    //
    // failures:
    //
    // ---- tests::another stdout ----
    // thread 'tests::another' panicked at 'パニック！', src\lib.rs:X:X
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    //
    // failures:
    //     tests::another
    //
    // test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    // error: test failed, to rerun pass '--lib'


    // まとめで ok の代わりに FAILED と書かれている。
    // 実行結果(test tests::...) とまとめ(test result) の間に、新たに2つのフィールドが現れました

    // 最初のフィールド
    // `---- tests::another...` : 失敗した各テストの具体的な理由を表示している
    // 今回の場合、tests::another が 'パニック！' でパニックしたと言われている

    // 次のフィールド
    // `failures: tests::another` : 失敗したテストの名前を列挙している

   // assert!マクロで結果を確認する
    // assert!マクロは標準ライブラリで提供されている
    // これでテスト内の何らかの条件が真であると確かめることが出来る
    // assert!マクロの引数には論理値(つまり式)を与える
    // true が渡されると テストは通る
    // false が渡されると assert!マクロが panic!マクロを呼び出し、テストは失敗となる
    // assert!マクロを使用することで、コードが意図した通りに機能していることを確認する助けになるわけです

    // 以前、Rectangle構造体と can_holdメソッドを使用しました
    // それを assert!マクロを用いてテストしてみる

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    // can_holdメソッドは論理値を返すので assert!マクロの出番です
    // 以下では 幅8、高さ7 の Rectangleインスタンスを生成し、
    // これが 幅5、高さ1 の別の Rectangleインスタンスに収まるかアサーションすることで、
    // can_hold をのテストを書く

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(larger.can_hold(&smaller));
        }
    //} // まだテスト関数置くので閉じさせない

    // テストは larger_can_hold_smaller と名付け、Rectangleインスタンスを2つ生成している
    // そして assert!マクロを呼び出し、larger.can_hold(&smaller) の呼び出し結果(論理値)を渡しました
    // この式は true を返し テストは通るはずです

    // $ cargo test
    //   Compiling adder v0.1.0 (...\adder)
    //    Finished test [unoptimized + debuginfo] target(s) in 0.28s
    //     Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 1 test
    // test tests::larger_can_hold_smaller ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // 通りますね＾＾
    // 別のテストを追加してみる
    // 次は 小さい長方形に大きな長方形が収まらないことをアサーションします

        #[test]
        fn smaller_cannot_hold_larger() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(!smaller.can_hold(&larger));
        }
    }

    // 今回の場合、can_hold関数が false を返すことを期待するので、
    // assert!マクロ に渡す前に論理値を反転させる必要がある
    // !smaller.can_hold(&larger) とすることで反転させている

    // $ cargo test
    //   Compiling adder v0.1.0 (...\adder)
    //    Finished test [unoptimized + debuginfo] target(s) in 0.30s
    //     Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 2 tests
    // test tests::larger_can_hold_smaller ... ok
    // test tests::smaller_cannot_hold_larger ... ok
    //
    // test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    // テストが両方通っていますね(＾___＾)
    // さて、can_hold関数をバグらせたらどうなるでしょう
    // 比較演算子を反転させてやりましょう
    // やっぱ名前 can_hold_ii関数にします
    
    impl Rectangle {
        fn can_hold_ii(&self, other: &Rectangle) -> bool {
            self.width < other.width && self.height < other.height
        }
    }

    #[cfg(test)]
    mod tests_ii {
        use super::*;

        #[test]
        fn larger_can_hold_smaller_ii() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(larger.can_hold_ii(&smaller));
        }

        #[test]
        fn smaller_cannot_hold_larger_ii() {
            let larger = Rectangle {
                width: 8,
                height: 7,
            };
            let smaller = Rectangle {
                width: 5,
                height: 1,
            };
            assert!(!smaller.can_hold_ii(&larger));
        }
    }
    
    // で、

    // $ cargo test
    //    Compiling adder v0.1.0 (...\adder)
    //     Finished test [unoptimized + debuginfo] target(s) in 0.28s
    //      Running unittests (target\debug\deps\adder-3829d87ced07c552.exe)
    //
    // running 3 tests
    // test tests_ii::larger_can_hold_smaller_ii ... FAILED
    // test tests_ii::smaller_cannot_hold_larger_ii ... FAILED
    //
    // failures:
    //
    // ---- tests_ii::larger_can_hold_smaller_ii stdout ----
    // thread 'tests_ii::larger_can_hold_smaller_ii' panicked at 'assertion failed: larger.can_hold_ii(&smaller)', src\lib.rs:278:13
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    // ---- tests_ii::smaller_cannot_hold_larger_ii stdout ----
    // thread 'tests_ii::smaller_cannot_hold_larger_ii' panicked at 'assertion failed: !smaller.can_hold_ii(&larger)', src\lib.rs:291:13
    //
    //
    // failures:
    //     tests_ii::larger_can_hold_smaller_ii
    //     tests_ii::smaller_cannot_hold_larger_ii
    //
    // test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    //
    // error: test failed, to rerun pass '--lib'