#![allow(unused)]
    // Rust はエラー処理においても优秀です
    // ソフトウェアにおいてエラーは生きている証し
    // なので Rust には何かがおかしくなる場面を扱う機能がたくさんある
    // 様々な場面でコンパイラはプログラマに エラーの可能性を知ってコンパイルが通るまでに対応をできるように要求してくる
    // それによりエラーを発見してコードをリリースする前に適切に対処出来ていることを確認することでプログラムを頑健なものにできる

    // Rust ではエラーは大きく分けて2種類ある
    // 回復可能と回復不能なエラーです
    // ファイルが見つからない等の回復可能なエラーでは 問題をユーザに報告し処理を再試行することができる
    // 回復不能なエラーは配列の境界を超えた箇所へのアクセスなどでバグの兆候

    // 多くの言語ではこの2種のエラーを区別することはあまりない
    // 例外などの機構を使用して同様に扱う
    // Rust には例外が存在しない代わりに回復可能なエラーには Result<T, E>値がある
    // プログラムが回復不能なエラーに遭遇した際に実行を中止するpanic!マクロがある
    // このではまず panic! の呼び出しについて学び、それから Result<T, E>を戻り値にする方法を学ぶ
    // 加えて エラーからの回復を試みるか、実行を中止するか決定する際に 考慮すべき事についても学ぶ
pub fn panic() {
   // panic!で回復不能なエラー
    // 時としてコードで悪いことは起こる。そしてそれに対してできることは何もない
    // 贵樣ばどラずゑこともできなぃ:()
    // このような場面の時の為に panic!マクロが用意されている
    // panic!マクロが実行されると プログラムは失敗のメッセージを表示し スタックを巻き戻し掃除して 終了する
    // これがよく起こるのは何らかのバグが検出された時で、我々はどうエラーを処理すればいいかはっきりしない

    // パニックに対してスタックを巻き戻すか異常終了するかについて
        // 標準ではパニックが発生するとプログラムは巻き戻しを始める
        // つまり言語がスタックを遡って遭遇した各関数のデータを片付けるということ
        // しかしこの遡りと片付けはすべきことが多くなる
        // 対策としては即座に異常終了して片付けをせずにプログラムを終了させること
        // こうなるとプログラムが使用していたメモリは OS が片付けることになる
        // プロジェクトにおいて実行可能ファイルを極力小さくする必要があれば、
        // Cargo.toml の [profile]欄 に `panic = 'abort'` を追記することでパニック時に巻き戻しから異常終了するように切り替えることができる
        // 例えばリリースモード時に異常終了するようにしたければ以下を追記することになる
        //[profile.release]
        //panic = 'abort'

    // 取り敢えず呼ぶ
    //panic!("クラッシュして大炎上");
    // こうなると以下のエラーが出る
    //thread 'main' panicked at 'クラッシュして炎上', src\main.rs:41:5
    //note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // panic! の呼び出しがこのエラーメッセージを発生させている
    // 上の1行目でパニックメッセージとパニックが発生した箇所を表していて、src/main.rs41:5は src/main.rsの41行目の5文字目で起こったという意味
    // この場合表される行は自分のコードの一部なのでその箇所を見に行けば panic!マクロがあるってこと
    // また、panic! が自分のコードが呼び出しているコードの一部になっている可能性もある
    // その場合 報告されるファイル名と行が、panic! の呼び出しに導いた panic!マクロを呼び出している他人のコードとなる
    // panic! の発生元である関数のバックトレースを使用して問題を起こしている自分のコードの箇所を割り出すことができる
    // バックトレースがどんなものか学びませう

   // panic!バックトレースを使用する
    // 別の例でライブラリで panic!呼び出しが発生するとどうなるか診てみる
    // 添え字でベクタの要素にアクセスを試みるコードです
    let v = vec![1, 2, 3];
    //v[99]; // 範囲外へのアクセスでパニック
    // ここではベクタの100番目の要素へのアクセスを試みているがベクタには3つしか要素がない
    // このコードでは Rustはパニックする
    // []の使用は要素を返すと想定されるが 無効な添え字を渡せば Rust が返せて正しいと思われる要素は何もない

    // 他の言語(例えばC言語)ではこの場面で欲しいものではないのにまさしく要求したものを返そうとしてくる
    // メモリがベクタに属していないのにベクタ内のその要素に対応するメモリ上の箇所にあるものを何か返してくる(ガバガバで草)
    // これはバッファー外読み出しと呼ばれていて、
    // 悪人が配列の後に格納された読めるべきでないデータを読み出せるように添え字を操作できれば セキュリティの脆弱性につながる
    // この様な脆弱性からプログラムを保護するためにも、
    // 存在しない添え字の要素を読もうとしたら Rust は実行を中止して継続を諦める
    // 以下のエラーは自分のものではない vec.rs ファイルを指す
    /*
    $ cargo run
       Compiling panic v0.1.0 (file:///projects/panic)
        Finished dev [unoptimized + debuginfo] target(s) in 0.27 secs
         Running `target/debug/panic`
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
    99', /checkout/src/liballoc/vec.rs:1555:10
    note: Run with `RUST_BACKTRACE=1` for a backtrace.
    */
    // 標準ライブラリの Vec<T> の実装です
    // ベクタv に対し[]を使った時に走るコードは vec.rs にありここで実際に panic! が発生している(ソース: let v = vec![1, 2, 3];v[99];)
    // その次の注釈行(note)は RUST_BACKTRACE環境変数をセットして 何が起きてエラーが発生したかのバックトレースを得られることを教えてくれている

    // バックトレースとはここに至るまでに呼び出された全関数の一覧。/*#バックトレース*/
    // Rust のバックトレースも他言語同様に動作する。バックトレースを読むコツは 頭からスタートして自分のファイルを見つけるまで読むこと
    // そこが問題の根源になる。自分のファイルを表している場所以前は自分のコードで呼び出したコードになり、以後は自分のコードを呼び出しているコードになる
    // これらの行には Rust の核となるコードや std のコードや使用しているクレートなどが含まれる可能性がある
    // RUST_BACKTRACE環境変数を0以外の値にセットしてバックトレースを出力してみると以下のように出力される
/*
    $ RUST_BACKTRACE=1 cargo run
        Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
         Running `target/debug/panic`
    thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /checkout/src/liballoc/vec.rs:1555:10
    stack backtrace:
       0: std::sys::imp::backtrace::tracing::imp::unwind_backtrace
                 at /checkout/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
       1: std::sys_common::backtrace::_print
                 at /checkout/src/libstd/sys_common/backtrace.rs:71
       2: std::panicking::default_hook::{{closure}}
                 at /checkout/src/libstd/sys_common/backtrace.rs:60
                 at /checkout/src/libstd/panicking.rs:381
       3: std::panicking::default_hook
                 at /checkout/src/libstd/panicking.rs:397
       4: std::panicking::rust_panic_with_hook
                 at /checkout/src/libstd/panicking.rs:611
       5: std::panicking::begin_panic
                 at /checkout/src/libstd/panicking.rs:572
       6: std::panicking::begin_panic_fmt
                 at /checkout/src/libstd/panicking.rs:522
       7: rust_begin_unwind
                 at /checkout/src/libstd/panicking.rs:498
       8: core::panicking::panic_fmt
                 at /checkout/src/libcore/panicking.rs:71
       9: core::panicking::panic_bounds_check
                 at /checkout/src/libcore/panicking.rs:58
      10: <alloc::vec::Vec<T> as core::ops::index::Index<usize>>::index
                 at /checkout/src/liballoc/vec.rs:1555
      11: panic::main
                 at src/main.rs:4
      12: __rust_maybe_catch_panic
                 at /checkout/src/libpanic_unwind/lib.rs:99
      13: std::rt::lang_start
                 at /checkout/src/libstd/panicking.rs:459
                 at /checkout/src/libstd/panic.rs:361
                 at /checkout/src/libstd/rt.rs:61
      14: main
      15: __libc_start_main
      16: <unknown>
    */
    // 出力が多いです。OS や Rust のバージョンにより出力の詳細は変わる可能性がある
    // この情報とともにバックトレースを得るにはデバッグシンボルを有効にしなければいけない
    // デバッグシンボルは --releaseオプションなしで cargo build や cargo run を使用すれば標準で有効になる
    // 上の出力でバックトレースの11行目が問題発生箇所を指し示している
    // src/main.rs の4行目となる。プログラムにパニックして欲しくないなら、
    // 自分のファイルについて書いてある 最初の行で示されている箇所こそが どんなパニックを引き起こす値で、
    // どうこの箇所にたどり着いたかを割り出すため、調査を開始すべき箇所になる

    // バックトレースの使用法を示す為に故意にパニックするコードを書いた節冒頭のコードにおいて、((ソース: let v = vec![1, 2, 3];v[99];))
    // パニックを解消する方法は範囲外へのアクセスを止めること。将来コードがパニックしたら、
    // パニックを引き起こす物が どんな値で どんな動作 をしているのかと、
    // そしてコードに何をすべきなのかを考える必要がある
    
    // 次は Result を使用してエラーから回復する方法について考える
}

pub fn result() {
  // Resultで回復可能なエラー
    // プログラムを完全にストップさせるほど深刻なエラーはあまり無い
    // 時々 処理の中でエラーに対応できることがある
    // 例えばファイルを開こうとしたが 目標のファイルが存在しなくて処理に失敗したら、
    // プロセスを停止するのではなくファイルを作成したりできる
    // Result の enum が Ok と Err の2列挙子からなるよう定義されていることを思い出してください
    enum Result<T, E> { Ok(T), Err(E), }
    // T と E はジェネリックな型引数です。ジェネリクスについはもうじき議論する
    // 今知っておいて欲しいのは T が成功した時に Ok列挙子に含まれて返される値の型を表すことと、
    // Eが失敗した時に Err列挙子に含まれて返されるエラーの型を表すこと
    // Result はこのようなジェネリックな型引数を含むため std上に定義される Result型や関数などを成功時とエラー時で返す値を分けられる
    // 関数が失敗する可能性があるので Result値を返す関数を呼び出す

    // 以下ではファイルを開こうとしている
    use std::fs::File;
    let f = File::open("hello.py");

    // File::open が Result を返したものをどう取得するのでしょう
    // std の APIドキュメントでも知ることができ、 コンパイラに尋ねることもできる(vscなら変数をホバーするだけで型がわかる)
    // f に戻り値ではないと分かる型注釈を与えコードのコンパイルをしようとすれば、
    // コンパイラが「型が合わない」と教えてくれう。そしてエラーメッセージでは f の実際の型を教えてくれる
    // 試してみる
    // File::open の戻り値の型は u32 ではないと分かっているので let文を臨時で以下のように変更してみる
    //let f_ii:u32 = File::open("hello.py");
    // エラー↓

    /*
    error[E0308]: mismatched types
       --> src\main.rs:164:20
        |
    164 |     let f_ii:u32 = File::open("hello.py");
        |              ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `std::result::Result`
        |              |
        |              expected due to this
        |
        = note: expected type `u32`
                   found enum `std::result::Result<File, std::io::Error>`
    */

    // これで File::open関数の戻り値の型は Result<T, E> であることが判明した
    // ジェネリック引数の T は成功値の型 std::fs::File で埋められていて、これはファイルハンドル
    // エラー値で使用されている E の型は std::io::Error
    // この戻り値型は File::open の呼び出しが成功して 読込と書込を行えるファイルハンドルを返せるということを表す
    // また 関数呼出は失敗もする可能性がある。例えばファイルが存在しない可能性, ファイルへのアクセス権限がない可能性などによるもの
    // File::open には 成功したか失敗したかを知らせる方法 と ファイルハンドルまたはエラー情報を与える方法 が必要
    // この情報こそが Result enum が伝達するもの

    // File::open が成功した場合 変数f の値はファイルハンドルを含むOkインスタンスになる
    // 失敗した場合 エラーの種類の情報を多く持つ Errインスタンスが f の値になる
    // 初めのコードに追記をして File::open が返す値に応じ 異なる動作をする必要がある
    // 以下は基礎的な道具(match式)を使って Result を扱う方法を一つ示している
    let f_iii = File::open("hello.py");
    let f_iii = match f_iii { // f_iiiの戻り値 ( つまりFile::open()の返り値 ) に応じて条件分岐
        Ok(file) => file, // Ok値が返されたらそのまま返す
        Err(error) => panic!("ファイルを開く時に問題が発生: {:?}", error), // Err値が返されたら エラー文を返す
    };
    // Option enum と同じく Result en
}