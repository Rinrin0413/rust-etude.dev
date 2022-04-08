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