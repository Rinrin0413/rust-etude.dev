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
    
    // しっかりとバグを検知している
    
   // assert_eq!とassert_ne!マクロで等値性をテストする
    // assert(a == b)などのテストは専用のマクロがいる
    // assert_eq! と assert_ne! です
    // これらはどちらも、2つの引数を比較して 等しいか否かを確かめる
    // アサーションが失敗したら その2つの値も出力するので、テストが失敗した原因がより明確になる

    // 以下に 引数に2を加算して返す add_two関数がある
    // そして assert_eq!マクロでこの関数をテストしてみる

    pub fn add_two(a: i32) -> i32 {a + 2 }
    
    #[cfg(test)]
    mod tests_iii {
        use super::*;
    
        #[test]
        fn it_adds_two() {
            assert_eq!(4, add_two(2));
        }
    }

    // 長くなるので出力は一部分のみにする:

    // running 1 test
    // test tests_iii::it_adds_two ... ok
    //
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

    // assert_eq!マクロに与えた第1引数の4と、add_two(2)は等しい
    // 故にテストは通りました

    // add_two(_ii)関数バグらせますwwwww
    // 2の代わりに3でも加算させましょう

    pub fn add_two_ii(a: i32) -> i32 {
        a + 3
    }

    #[cfg(test)]
    mod tests_iv {
        use super::*;
    
        #[test]
        fn it_adds_two_ii() {
            assert_eq!(4, add_two_ii(2));
        }
    }

    // running 1 test
    // test tests_iv::it_adds_two_ii ... FAILED
    //
    // failures:
    //
    // ---- tests_iv::it_adds_two_ii stdout ----
    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //   left: `4`,
    //   right: `5`', src/lib.rs:X:X
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
    //
    //
    // failures:
    //     tests_iv::it_adds_two_ii
    //
    // test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
    //
    //
    // error: test failed, to rerun pass '--lib'

    // `(left == right)` という文言の下で leftが4、rightが5 であったと示されている
    // ありがたいですね
    
    // assert_ne!マクロは 与えられた2つの値が等しくなければ通り、等しければ失敗する
    // 値が何になるかは分からないけど 絶対にこれにはならないという値があれば有用

   // カスタムの失敗メッセージを追加する
    // ssert!, assert_eq!, assert_ne! の追加引数として、
    // 失敗メッセージと共にカスタムのメッセージが表示されるようにもできる
    // assert! なら第2引数以降に、assert_eq!及びassert_ne! なら第3引数以降に、
    // println!するときなどのように引数を置くことができる
    // 例として、名指しで挨拶をする関数があるとして
    // 関数に渡した名前が出力に出現することをテストしたいとしする

    pub fn greeting(name: &str) -> String {
        format!("こんちゃ {}!", name)
    }
    
    #[cfg(test)]
    mod tests_v {
        use super::*;
    
        #[test]
        fn greeting_contains_name() {
            let result = greeting("もるか");
            assert!(result.contains("もるか"));
        }
    }

    // 結果:
    // test tests_v::greeting_contains_name ... ok

    // 名指ししないようにしてみる

    pub fn greeting_ii(name: &str) -> String {
        String::from("こんちゃ!")
    }
    
    #[cfg(test)]
    mod tests_vi {
        use super::*;
    
        #[test]
        fn greeting_contains_name_ii() {
            let result = greeting_ii("もるか");
            assert!(result.contains("もるか"));
        }
    }

    // 結果:
    
    // test tests_vi::greeting_contains_name_ii ... FAILED
    //
    // failures:
    //
    // --SNIP--
    //
    // ---- tests_vi::greeting_contains_name_ii stdout ----
    // thread 'tests_vi::greeting_contains_name_ii' panicked at 'assertion failed: result.contains(\"もるか\")', src\lib.rs:X:X
    //
    //
    // failures:
    //    ...
    //    tests_iv::it_adds_two_ii

    // 失敗している
    // 今回の場合、失敗メッセージで greeting関数から得た値を出力していればありがたいはず
    // やってみう

    #[cfg(test)]
    mod tests_vii {
        use super::*;
    
        #[test]
        fn greeting_contains_name_iii() {
            let result = greeting_ii("もるか");
            assert!(
                result.contains("もるか"),
                "\n> greeting_ii関数は名指しできてないよ。\n> 値: {}\n",
                result
            );
        }
    }

    // 結果(一部):

    // ---- tests_vii::greeting_contains_name_iii stdout ----
    // thread 'tests_vii::greeting_contains_name_iii' panicked at '
    // > greeting_ii関数は名指しできてないよ。
    // > 値: こんちゃ!
    // ', src\lib.rs:X:X

    // わかりやすいですね
    // このようにして 具体値が分からなくとも 起こり得る問題をデバックできる

   // should_panicでパニックを確認する
    // 期待された値が返されることを確認するだけでなく、
    // 想定通りにエラーハンドリングしていることも確認しなければならない

    // 例えば、以前に生成した Guess型で考えてみる
    // Guess を使用するコードでは、Guessインスタンスが1から100の範囲の値しか含めない契約に依存している
    // その範囲外の値で Guessインスタンスを生成しようとすると、
    // そこでパニックするようになっている
    // これを確認するテストを書くことができる

    // これは、テスト関数に should_panic という別の属性を追加することで行える
    // この属性は、関数内のコードがパニックした場合のみテストを通す
    // つまり、パニックしなかったら テストは失敗する

    // 以下は、Guess::new のエラーハンドリングが正常に動作していることを確認している
    pub struct Guess { value: i32 }
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("予想値は1から100の間でなければなりませんが、{}でした。 ", value);
            }
            Guess { value }
        }
    }
    
    #[cfg(test)]
    mod tests_iix {
        use super::*;
    
        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(256);
        }
    }

    // #[test]属性の次の行に #[should_panic]属性を配置している
    // これでテストを実行すると:

    // test tests_iix::greater_than_100 - should panic ... ok

    // 256から Guessインスタンスを生成しようとしてパニックを起こすので、
    // テストが通っている
    // 次はエラーハンドリングを消し飛ばして実行してみる

    pub struct GuessII { value: i32 }
    impl GuessII {
        pub fn new(value: i32) -> Guess {
            /* // エラーハンドリングを無効化
            if value < 1 || value > 100 {
                panic!("予想値は1から100の間でなければなりませんが、{}でした。 ", value);
            }
            */
            Guess { value }
        }
    }

    #[cfg(test)]
    mod tests_ix {
        use super::*;
    
        #[test]
        #[should_panic]
        fn greater_than_100_ii() {
            GuessII::new(256);
        }
    }

    // テスト結果:

    // test tests_ix::greater_than_100_ii - should panic ... FAILED

    // このように、
    // パニックを起こせていないのでテストが通りません

    // should_panic でのテストは不正確なこともある
    // なぜなら、パニックの有無しか確認できないから
    // エラーハンドリング用でない部分でパニックしても通ってしまう
    // should_panic属性に expected引数を追加することでこれをカバーできます
    // このテストハーネスは、失敗メッセージに 特定のテキストが含まれていることを確かめることができる
    // expected引数で Guess を補完してみる

    pub struct GuessIII { value: i32 }
    impl GuessIII {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("予想値は1以上でなければなりませんが、{}でした。", value);
            } else if value > 100 {
                panic!("予想値は100以下でなければなりませんが、{}でした。", value);
            }
            Guess { value }
        }
    }
    
    #[cfg(test)]
    mod tests_x {
        use super::*;
    
        #[test]
        #[should_panic(expected = "予想値は100以下でなければなりません")]
        fn greater_than_100_iii() {
            GuessIII::new(256);
        }

        #[test]
        #[should_panic(expected = "予想値は1以上でなければなりません")]
        fn less_than_1() {
            GuessIII::new(-32);
        }
    }

    // テスト結果:

    // test tests_x::greater_than_100_iii - should panic ... ok
    // test tests_x::less_than_1 - should panic ... ok

    // 検出しているみたいです
    // エラーハンドリングをまたぶち壊してみます
    // `value > 100` を `value > 512` に変えてやります

    pub struct GuessIV { value: i32 }
    impl GuessIV {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!("予想値は1以上でなければなりませんが、{}でした。", value);
            } else if value > 512 {
                panic!("予想値は100以下でなければなりませんが、{}でした。", value);
            }
            Guess { value }
        }
    }
    
    #[cfg(test)]
    mod tests_xi {
        use super::*;
    
        #[test]
        #[should_panic(expected = "予想値は100以下でなければなりません")]
        fn greater_than_100_iv() {
            GuessIV::new(256);
        }

        #[test]
        #[should_panic(expected = "予想値は1以上でなければなりません")]
        fn less_than_1_ii() {
            GuessIV::new(-32);
        }
    }

    // テスト結果は:

    // test tests_xi::less_than_1_ii - should panic ... ok
    // test tests_xi::greater_than_100_iv - should panic ... FAILED

    // `GuessIV::new(256);` がパニックを起こさずに 256 を取り込みやがったので、
    // エラーは通りませんでした

   // Result<T, E> をテストで使う
    // これまでは、失敗するとパニックするようなテストを書いてきた
    // Result<T, E> を使うテストを書くこともできる
    // 以下は Result<T, E> を使い、パニックする代わりに Err を返すように書き直したもの：

    #[cfg(test)]
    mod tests_xii {

        #[test]
        fn it_works_ii() -> Result<(), String> {
            if true {
                Ok(())
            } else {
                Err(String::from("Error!!!"))
            }
        }
    }

    // 戻り値の型を Result<(), String> にしました
    // 関数内で assert_eq!マクロを呼び出す代わりに、
    // テストが成功すれば Ok(())を、
    // 失敗すれば Err にStringを入れて返すようにしている