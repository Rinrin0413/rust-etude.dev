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