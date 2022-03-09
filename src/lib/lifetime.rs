#[allow(unused)]
pub fn lifetime() {
 // ライフタイムで参照を検証する
    // 参照や借用 において 一つ重要な詳細を説明していなかった
    // Rust において参照は 全てライフタイムを保持している
    // ライフタイムとは その参照が有効になるスコープのこと
    // 多くの場合、型が推論されるのと同じく ライフタイムも暗黙的に推論される
    // 複数の型が推論される場合は 型を注釈する必要があった
    // 同様に、参照のライフタイムが 幾つかの方法で関係する可能性がある場合は注釈しなければならない
    // コンパイラに ジェネリックライフタイム引数を使って関係を注釈して、
    // 実行時に「実際の参照が確かに有効であること」を保証しなければならない
    // ライフタイムの概念は、他のプログラミング言語の道具とは異端で 確実で Rust で1番際立つ機能となる

    // 今回でライフタイムの全体を解説することはない。ライフタイム記法が必要となる最も一般的な場合について学ぶ
    // ライフタイムの概念に馴染めるといいな( 'ω')

  // ライフタイムでダングリング参照を回避する
    // ライフタイムの主な目的は ダングリング参照を回避すること
    // ダングリング参照により プログラムが参照するつもりだったデータ以外のデータを参照してしまう
    // 以下は 外側のスコープと内側のスコープが含まれている

    // [NOTE] 値は生まれたスコープの終端に当たると死ぬ

    /*
    { //  外側のスコープ
        let r;
        { // 内側のスコープ
            let x = 5;
            r = &x;
        }
        println!("{}", r);
    }
    */

    //*note: 変数に初期値を与えずに宣言しているので 変数名は外側のスコープに存在する
    //       これは「Rust には null値が存在しない」ということと衝突してるように見えるかもしれない
    //       値を与える前に変数を使おうとすれば コンパイルエラーになる。これは Rust では null値は許可されないから

    // 外側のスコープで初期値なしの変数r を宣言して 内側のスコープで初期値が5の変数x を宣言している
    // 内側のスコープ内で r の値を xへの参照にしようとしている
    // それから内側のスコープが終わって r の値を出力しようとしている
    // r が参照している値が既にスコープを抜けるので このコードはコンパイルできない
    // つまり x の値を参照してるのに その x が既に drop(死亡) しててエラーってこと
    // こちらがエラーメッセージ

    /*
    error[E0597]: `x` does not live long enough // 訳: x の生存期間が短すぎる
      --> src/main.rs:X:X
       |
    X  |         r = &x;
       |              - borrow occurs here // 訳: 借用はここで起きている
    X  |     }
       |     ^ `x` dropped here while still borrowed // 訳: x は借用されている間にここでドロップ
    ...
    X  | }
       | - borrowed value needs to live until here // 訳: 借用された値はここまで生きる必要がある
    */

    // 変数x の「生存期間が短すぎる」の原因は、内側のスコープが終わった時点で x がスコープを抜けること
    // ですが r はまだ 外側のスコープに対して有効。スコープが大きいので「長生きする」と言う
    // Rust で このコードが動けたら r は x がスコープを抜けた時に解放されるメモリを参照していることになり、
    // r でやりたいことが正常に動作しない。ならばどの様にコンパイラは このコードが無効であると決定しているか。借用チェッカーを使用している

  // 借用精査機
	// Rustコンパイラには、スコープを比較して 全ての借用が有効であるかを決定する借用チェッカーがある
   	// 以下は 先ほどと同じコードに 変数のライフタイムを表示する注釈が付いている

    /*
    {
        let r;                // --------+- 'a   // a誕生🎊
        {                     //         |
            let x = 5;        // -+- 'b  |       // b誕生🎊
            r = &x;           //  |      |
        }                     // -+      |       // b死亡✞
        println!("{}", r);    //         |
    }                         // --------+       // a死亡✞
    */

    // ここで rのライフタイムは 'a 、xのライフタイムは 'b として注釈した
    // ご覧の通り 内側の 'b の方が 外側の 'a より遥かに短命
    // コンパイル時に コンパイラは2つのライフタイムのサイズを比較し、
    // r は 'a のライフタイムだが、'b のライフタイムのメモリを参照していると確認する
    // 'b は 'a よりも短命な故 プログラムは拒否される。参照の対象が参照の様には長生きしていない
    // 以下はコードを修正したので、ダングリング参照はなくなってエラーなくコンパイルできる

    {
        let x = 5;            // ---------+- 'b   // 'b誕生🎊
        let r = &x;           // --+- 'a  |       // 'a誕生🎊
        println!("r: {}", r); //   |      |
    }

    // ここで xのライフタイムは 'b であり、'a よりも大きい
    // つまり コンパイラは x が有効な間 r の参照も常に有効になると把握している故 rはxを参照できる
    // 今や 参照のライフタイムがどのくらいあって コンパイラが「ライフタイムを解析して参照が常に有効である」とを保証する仕組みがわかったので、
    // 関数の文脈でジェネリック引数と戻り値のライフタイムを探究する

  // 関数のジェネリックなライフタイム
    // 2つの文字列スライスのうち 長い方を返す関数を書く
    // この関数は 2つの文字列スライスを取り 1つの文字列スライスを返す
    // longest関数の実装すれば `The longest string is abcd` と出力される

    /*
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); //< The longest string is abcd
    */

    // 関数に取ってほしい引数が 文字列スライス つまり参照であることに注意(&str)
    // 何故なら longest関数に 引数の所有権を奪ってほしくない故
    // この関数に String型のスライス と 文字列リテラル を受け取らせている
    // 以下の様に longest関数を実装しようとしたら コンパイルはできない

    /*
                                    ┏ この参照の元が不明 
    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    */

    // 代わりに ライフタイムについて言及するエラーが出る
    /*
    error[E0106]: missing lifetime specifier // 訳: ライフタイム指定子が不足している
     --> src/main.rs:X:X
      |
    X | fn longest(x: &str, y: &str) -> &str {
      |                                 ^ expected lifetime parameter // 訳: ライフタイム引数が予想される
      |
      = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
            // ┗ 訳: この関数の戻り値型は借用された値を含んでいるが シグネチャでそれが x由来なのか y由来のものなのか宣言していない
    */


    // 助言(note)で 戻り値の型はジェネリックなライフタイム引数でなければならないと示している
    // というのも、返している参照が x か y のどちらを参照しているかが コンパイラには分からない
    // この関数の本体の ifブロックは x への参照を返し、
    // elseブロックは y への参照を返すので 実際どちらの参照が返されるかは私たちにもわからない

    // この関数を定義する際に 渡される具体的な値が分からないので ifケースか elseケースどちらで返却されるか分からない
    // また、渡される参照の具体的なライフタイムも分からない故、
    // 返す参照が常に有効であるかを決定したように スコープを見ることもできない
    // 借用チェッカーもこれを決定することはできない。x と y のライフタイムがどう戻り値のライフタイムと関係するか分からない故
    // このエラーを修正するには 借用チェッカーが解析を実行できるように 参照間の関係を定義するジェネリックなライフタイム引数を追加する

  // ライフタイム注釈記法
    // ライフタイム注釈が 参照の生存期間を変えることはない
    // ジェネリック型引数を宣言された関数が あらゆる型を受け取ることができるのと同じく、
    // ジェネリックなライフタイム引数を指定された関数は あらゆるライフタイムの参照を受け取ることができる
    // ライフタイム注釈は ライフタイムに影響せずに 複数の参照のライフタイムのお互いの関係を記述する

    // ライフタイム注釈は 少し不自然な記法です
    // ライフタイム引数の名前は `'` で始まらなければならず、通常全部小文字で ジェネリック型のように短ㄑ表記する
    // 多くの人は 'a という名前を使う。ライフタイム引数注釈は 参照の `&` の後に配置し 注釈と参照の型を区別するために半角空白を1つ置く
    // 以下は ライフタイム引数なしのi32への参照, 'aというライフタイム引数付きのi32への参照 そしてライフタイム'a付きのi32への可変参照

    /*
    &i32        // ただの参照
    &'a i32     // 明示的なライフタイム付きの参照
    &'a mut i32 // 明示的なライフタイム付きの可変参照
    */

    // 1つのライフタイム注釈それだけでは 大して意味はない
    // 注釈は 複数の参照のジェネリックなライフタイム引数が お互いにどう関係するかをコンパイラに指示するもの
    // 例えば ライフタイム'aの付いたi32への参照となる引数firstのある関数があるとする
    // この関数にはさらに、ライフタイム'aの付いたi32への別の参照となる引数second もある
    // ライフタイム注釈は first と second の参照がどちらも このジェネリックなライフタイムと同じだけ生きることを示唆する(同じライフタイム'aを使っているため)

  // 関数シグニチャにおけるライフタイム注釈
    // さて longest関数を例に ライフタイム注釈を詳しく見ていく
    // ジェネリック型引数と同じく 関数名と引数リスト(丸括弧の奴)の間の<>の中に ジェネリックなライフタイム引数を宣言する
    // このシグニチャで表現したい制約は 引数の全ての参照と戻り値が同じライフタイムを持つこと
    // 以下の様に、ライフタイムを'aと名付けて それを各参照に付与する

    //          ┏ ライフタイム'a の宣言
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	//                 ┗━━━━━━━━━━━┻━━━━━━━━━━━┻━ ライフタイム'a を付与
		if x.len() > y.len() {
			x
		} else {
			y
		}
	}

    // このコードはコンパイルでき、望んだ動きをするはず
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); //< The longest string is abcd

    // これで関数シグネチャは ライフタイム'aに対して関数は2つの引数を取り、
    // どちらも「少なくともライフタイム'aと同じだけ生きる文字列スライス」であるとコンパイラに示すことができる
    // また、この関数シグネチャは 関数の戻り値である文字列スライスも同じく 少なくともライフタイム'aと同じだけ生きると コンパイラに教えている
    // これらの制約は まさに私たちがコンパイラに保証して欲しかったもの


    // この関数シグニチャでライフタイム引数を指定する時 渡されたり返したりした いかなる値のライフタイムも変更していない
    // むしろ 借用チェッカーは これらの制約を守らない値全てを拒否するべきと言っている
    // longest関数は x と y の正確な生存期間を知っている必要はない
    // このシグニチャを満たすようなスコープを'aに代入できることを知っているだけ

    // 関数にライフタイムを注釈するときは 注釈は関数の本体ではなくシグニチャに付与する
    // コンパイラは本体で注釈がなくとも関数内のコードを解析できる
    // しかし 関数に 関数外からの参照や関数外への参照がある場合、
    // コンパイラが引数や戻り値のライフタイムを自力で求めるのはほぼ不可能
    // そのライフタイムは 関数が呼び出される度に変化している可能性がある
    // なので 手動でライフタイムを注釈する必要がある

    // 具体的な参照を longest に渡すと 'a に代入される具体的なライフタイムは、
    // x の居るスコープの一部であり yのスコープと重なる。言い換えると ジェネリックなライフタイム'a は、
    // xとy のライフタイムのうち、小さい方に等しい具体的なライフタイムになる:
    //'a == min(xのライフタイム, yのライフタイム)
    // 返却される参照を 同じライフタイム引数'a で注釈したので、:
    //`-> &'a str`
    // 返却される参照も xかy のライフタイムの小さい方と同じだけ有効になる:
    // 返却されるか参照のライフタイム == min(xのライフタイム, yのライフタイム)
    // ライフタイム注釈が異なる参照を渡すことで longest関数を制限する方法を考える

    let string_ii1 = String::from("long string is long");
    {
        let string_ii2 = String::from("xyz");
        let result_ii = longest(string_ii1.as_str(), string_ii2.as_str());
        println!("The longest string is {}", result_ii); //< The longest string is long string is long
    } // 内側のスコープの終端

    // この例において string_ii1 は外側のスコープの終端(このファイルではmain関数の終端)まで有効で
    // string_ii2 は内側のスコープの終わりまで有効、
    // そして result_ii 内側のスコープの終わりまで有効な何かを参照している
    // このコードを実行すると 借用チェッカーがこのコードを良しとする。要するに コンパイルでて、
    // `The longest string is long string is long` と出力される

    // 次に result_iiの参照している値のライフタイムが 2つの引数の小さい方のライフタイムになることを示す例を考える
    // result_ii変数の宣言を内側のスコープの外に移すが、result_ii変数への代入は string2 の居るスコープ内に残したままにしてみる
    // それから result_ii を呼び出す println! を内側のスコープの外に、
    // 内側のスコープが終わった後に移動する。これはコンパイルできない

    /*
    let string_iii1 = String::from("long string is long"); // main終端まで生きる
    let result_iii; // main終端まで生きる
    {
        let string_iii2 = String::from("xyz"); // 内側のスコープ終端で死ぬ
        result_iii = longest(string_iii1.as_str(), string_iii2.as_str());
    } // 内側のスコープの終端
    println!("The longest string is {}", result_iii); // もし string2 の方が長い文字列だった場合、
                                                      // そいつは 内側のスコープ終端で死亡しているため参照できない
    */

    // 以下のエラーが出る
    /*
    error[E0597]: `string_iii2` does not live long enough // 訳: string_iii2 の寿命が短すぎる
      --> src/main.rs:X:X
       |
    X  |         result_iii = longest(string_iii1.as_str(), string_iii2.as_str());
       |                                                    ----------- borrow occurs here // 訳: ここで借用されている
    X  |     }
       |     ^ `string_iii2` dropped here while still borrowed // 訳: ここで string_iii2 は借用されたまま死亡
    X  |     println!("The longest string is {}", result_iii);
    X  | }
       | - borrowed value needs to live until here // 訳: 借用されている値はここで生存していなければならない
    */

    // このエラーは println! が result_iii を呼び出すのに、
    // string_ii2 が外側のスコープの終端まで有効である必要があるとを示している
    // 関数の引数と戻り値のライフタイムを 同じライフタイム引数'aで注釈したので コンパイラはこのことを知っている

    // 人の目から見たら string_iii1 が string_iii2よりも長く、
    // それ故に result_iii が string_iii1 への参照を含んでいることは コードから読み取れる
    // まだ string_iii1 はスコープを抜けていないので string_iii1 への参照は println! にとって有効
    // だがしかし コンパイラはこの場合 参照が有効であると見なせない
    // longest関数が返す参照のライフタイムは、渡された2つの参照のうち 短命なライフタイムを 'a とした
    // 故に その渡された2つの参照は どちらも短命なライフタイムとしてコンパイラは認識した
    // したがって 借用チェッカーは上のコードを 無効な参照があるかもしれないと許可してくれない

    // 試しに 値や, longest関数に渡される参照のライフタイム, 返される参照の使用法 が異なる実験をもっとしてみてください
    // 自分の実験がコンパイル前に借用チェッカーを通るかどうか仮説を立ててください
    // そして、正しいか確かめろ(しなさい)

  // ライフタイムの観点で思考する
    // 何にライフタイム引数を指定する必要があるかは、関数が行っていることによって変わる
    // 例えば longest関数が 最長の文字列スライスではなく 常に最初の引数を返すようにしたら、
    // 第2引数に対してライフタイムを指定する必要はなくなる
    fn longest_ii<'a>(x: &'a str, y: &str) -> &'a str { x }

    // この例では 引数x と戻り値に対してライフタイム引数'aを指定したが、引数y には指定していない
    // y のライフタイムは x や戻り値のライフタイムとは何の関係もない故

    // 関数から参照を返す際 戻り値のライフタイム引数が 引数の中のいずれかのライフタイム引数と一致する必要がある
    // 返される参照が 引数のどれかを参照していなければ、この関数内で生成された値を参照していることになる
    // これは その値が関数の末端でスコープを抜けるので、ダングリング参照(復習:望まぬデータの参照)になってしまう
    // コンパイルできない longest_ii関数 実装を考えてみる

    /*
                                           ┏ ダングリング参照
    fn longest_ii<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string"); // A地点(関数の終端)で死亡
        result.as_str()
    } // A地点
    */

    // ここで 戻り値にライフタイム引数'a を指定しているが、
    // 戻り値のライフタイムは 引数のライフタイムと全く関係がないので この実装はコンパイルできない
    // 以下がエラー

    /*
        error[E0597]: `result` does not live long enough // 訳: result の寿命が短すぎる
     --> src/main.rs:X:X
      |
    X |     result.as_str()
      |     ^^^^^^ does not live long enough // 訳: 短命すぎる！
    X | }
      | - borrowed value only lives until here // 訳: 借用されたままここで死亡
      |
    note: borrowed value must be valid for the lifetime 'a as defined on the function body at X:X... 
         // ┗ 訳: 借用されてる値は ライフタイム'aに対して有効でなければならない
     --> src/main.rs:X:X
      |
    X | / fn longest_ii<'a>(x: &str, y: &str) -> &'a str {
    X | |     let result = String::from("really long string");
    X | |     result.as_str()
    X | | }
      | |_^
    */

    // 問題なのは result が longest_ii関数の末端でスコープを抜けて 片付けられてしまうこと
    // また 関数から result を返そうともしている
    // ダングリング参照を変えるであろうライフタイム引数を指定する手段はなく、
    // コンパイラは ダングリング参照を許さない。今回の場合の最善の修正案は、
    // 呼び出し元の関数が値の片付けに責任を持てるように、参照ではなく所有されたデータ型を返すことだろう
    // つまり返り値で参照してると値が片づけられちゃうから、値をムーブして返せってこと

    // ライフタイム記法は 関数のいろんな引数と戻り値のライフタイムを接続するもの
    // 一度それらが繋がれば メモリに安全な処理を許可し、
    // ダングリングポインタを生成したり メモリ安全性を侵害したり する処理を防止するのに 十分な情報をコンパイラは得たことになる

  // 構造体定義のライフタイム注釈
    // いままで 自己で所有する型を持つ構造体だけを定義してきた
    // 構造体に参照を保持させることvalleyもできるが、その場合 構造体定義の全参照にライフタイム注釈を付与する必要がある
    // 以下には 文字列スライスを保持する ImportantExcerpt(重要な一節)という構造体がある


    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("僕をイシュマエルとお呼び。何年か前・・・");
    let first_sentence = novel.split('。')
        .next()
        .expect("`。` が見つかりませんでした");
    let i = ImportantExcerpt { part: first_sentence };
    println!("先頭の文: {}", i.part); //< 先頭の文: 僕をイシュマエルとお呼び

    // この構造体には 文字列スライスを保持する1つのフィールド part があり これは参照となる
    // ジェネリック型と同じく 構造体名の後の<>の中にジェネリックなライフタイム引数の名前を宣言することで、
    // 構造体定義の本体でライフタイム引数を使える。
    // このライフタイム注釈は、ImportantExcerptのインスタンスが partフィールドが参照している値よりも長生きしないことを意味する

    // ここでは変数novel の所有する String の、最初の文への参照を保持する ImportantExcerptインスタンスを生成している
    // novel のデータ(値)は ImportantExcerptインスタンスが生成される前に存在している
    // 加えて ImportantExcerpt がスコープを抜けるまで novel もスコープを抜けないので、
    // ImportantExcerptインスタンスの参照は有効となる

  // ライフタイム省略
    // 全参照にはライフタイムがあり 参照を扱う関数や構造体には ライフタイム引数を指定する必要があることを学んだ
    // ですが 以下のような関数は ライフタイム注釈なしでコンパイルできる

    fn first_word(arg:&str) -> &str { 
        let bytes = arg.as_bytes();
        for (i, &item) in bytes.iter().enumerate() { 
            if item == b' ' { return &arg[0..i]; } 
        }
        &arg[..]
    }

    // この関数がライフタイム注釈なしでコンパイルできた理由は 歴史的なものです
    // 昔のバージョン(1.0以前)の Rust では 全参照に明示的なライフタイムが必要だったので、
    // このコードはコンパイルできなかった。その頃は関数シグニチャはこのように記述されていた
    //fn first_word<'a>(arg: &'a str) -> &'a str {

    // 多くの Rustコードを書いた Rustチームは、Rustacean が特定の場面では 何度も何度も同じライフタイム注釈を入力することに気づいた
    // これらの場面は予測可能で いくつかの決定的なパターンに従っていた
    // 開発者はこのパターンを コンパイラのコードに落とし込んだので、
    // このような場面には 借用チェッカーがライフタイムを推論するようにし 明示的な注釈を必要としなくなった

    // 他の決定的なパターンが出現し コンパイラの推論が強化されることもあり得るので この Rustの歴史は関係がある
    // 将来的には更に ライフタイム注釈を減らすことが出来るようになる可能性もある

    // コンパイラの参照解析に落とし込まれたパターンは、ライフタイム省略規則と呼ばれる /*#ライフタイム省略規則*/
    // これらはプログラマが従う規則ではない
    // コンパイラが考慮する一連の特定のケースであり、
    // 自分のコードがそのケースに当てはまれば ライフタイムを明示的に注釈する場面を減らせるだけ

    // 省略規則は完全な推論を提供しない。コンパイラは決定的に規則を適用できるが、
    // 参照が保持するライフタイムに関して それでもなお曖昧性があるならば、
    // コンパイラは 残りの参照がなるべきライフタイムを推測しない
    // その場合コンパイラは それらを推測するのではなくエラーを与える
    // これらは 参照がお互いにどう関係するかを指定するライフタイム注釈を追記することで解決できる

    // 関数やメソッドの引数のライフタイムは 入力ライフタイムと呼ばれ、
    // 戻り値のライフタイムは出力ライフタイムと称される

    // コンパイラは3つの規則を活用して 明示的な注釈がない時に 参照がどんなライフタイムになるかを計算する
    // 最初の規則は入力ライフタイムに適用され、
    // 2番目と3番目の規則は出力ライフタイムに適用される
    // コンパイラが3つの規則の最後まで到達して それでもなおライフタイムを割り出せない参照があったら コンパイラはエラーで停止する

   // 1番目の規則は、参照である各引数は 独自のライフタイム引数を得るというもの
    // 言い換えれば 引数が1つの関数は 1つのライフタイム引数を得るということ:
    //fn foo<'a>(x: &'a i32);
    // 2つ引数がある関数は 2つの個別のライフタイム引数を得る:
    //fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
    // 以下同様に...

   // 2番目の規則は、入力ライフタイム引数が1つだけあるなら そのライフタイムが 全ての出力ライフタイム引数に代入されるというもの:
    //fn foo<'a>(x: &'a i32) -> &'a i32

   // 3番目の規則は、複数の入力ライフタイム引数があっても、
    // そのうちの1つが &self や &mut self だったら(メソッド専用)、
    // self のライフタイムが 全出力ライフタイム引数に代入されるというもの
    // この3番目の規則により 必要なシンボルの数が減るので メソッドが遥かに読み書きしやすくなる
	
   // コンパイラになりきってみる
    // これらの規則を適用して以下の　first_word関数のシグニチャの参照のライフタイムが何か計算してみる
    // シグニチャは 参照に紐づけられるライフタイムがない状態から始まる
    //fn first_word(s: &str) -> &str {
    // ここでは最初の規則を適用するので 各引数が独自のライフタイムを得るようにする
    // シグニチャはまずこうなる
    //fn first_word<'a>(s: &'a str) -> &str {
    // そして 入力ライフタイムが1つだけなので 2番目の規則も適用できる
    // 入力引数のライフタイムが 出力引数に代入されるので シグニチャはこうなる
    //fn first_word<'a>(s: &'a str) -> &'a str {
    // コンパイラはシグニチャをこのような過程で計算してくれ、
    // 全ての参照にライフタイムが付いた。これでコンパイラは プログラマに関数シグニチャのライフタイムを注釈してもらわなくても 解析を続行できる

    // 別の例に目を向ける。先ほど ライフタイム引数がなかったlongest関数です:
    //fn longest(x: &str, y: &str) -> &str {
    // 1番目の規則を適用する。各引数が独自のライフタイムを得る
    // 今回は 1つではなく 2つ引数があるので ライフタイムも2種類
    //fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
    // 複数の入力ライフタイムがあるので 2番目の規則は適用されない
    // また3番目の規則も適用されない。longest はメソッドではなく関数な故 どの引数もselfではない
    // 3つの規則全部を適用してもなお まだ戻り値型のライフタイムが判明していない
    // なので この longest関数でエラーになったのです
    // コンパイラは ライフタイム省略規則全てを適用しても、シグニチャの参照全部のライフタイムを計算することができなかった

    // 3番目の規則はメソッドシグニチャでしか適用されないので、
    // 次にその文脈でライフタイムを観察して 3番目の規則がメソッドシグニチャであまり頻繁にライフタイムを注釈しなくても済むこと確認する

  // メソッド定義におけるライフタイム注釈
    // 構造体にライフタイムのあるメソッドを実装する際、
    // ジェネリック型引数と同じ記法を使用する
    // ライフタイム引数を宣言たり使用したりする場所は、構造体フィールドか, メソッド引数と戻り値に関係するかによって変わる
    // 構造体のフィールド用のライフタイム名は implキーワードの後ろに宣言する必要があり それから構造体名の後に使用される
    // そのようなライフタイムは構造体の型の一部になる故
    // implブロック内の メソッドシグニチャでは、
    // 参照は 構造体のフィールドの参照のライフタイムに紐づくか 独立している可能性がある
    // 加えて ライフタイム省略規則III により メソッドシグニチャでライフタイム注釈が必要なくなることがよくある
    // 先ほど定義した ImportantExcerpt構造体にメソッドを定義して考えてみる

    //   ┏ 構造体のフィールド用なので impl の後ろで宣言
    impl<'a> ImportantExcerpt<'a> {
    //                        ┗ 構造体のフィールドで使われるライフタイム
        fn level(&self) -> i32 { 3 }
    }

    // impl の後ろのライフタイム引数宣言と 型名の後に使用するのは必須ですが、┗
    // 省略規則Iのため self への参照のライフタイムを注釈する必要はない
}