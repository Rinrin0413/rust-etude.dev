#![allow(unused)]
    // std にはコレクションという非常に有益なデータ構造を含んでいる
    // コレクションは複数の値を含むことができる
    // 組み込みの配列とタプル型とは違いコレクションのデータはヒープに確保されていて
    // データ量はコンパイル時にわかる必要はなく伸縮可能である
    // 各コレクションには異なる能力とコストが存在し様々な場面においてどれを使うべきかの判断力は軽々とともに育んでいきます
    // ここでは頻繁に使用される3つのコレクションについて紹介する
    //    ベクタ型 : 可変長(伸縮可能)な値を並べて保持できる。
    //    文字列 : 文字のコレクション。以前 String型について触れたがさらに詳しく掘り下げる。
    //    ハッシュマップ : 値を特定のキーと紐付けさせることが出来るより一般的なデータ構造。マップの特定の実装である。
    // 他のコレクションについて学ぶにはドキュメント参照
    // ベクタ型, 文字列, ハッシュマップの生成と更新方法や各々の特徴について学んでいきましょう

pub fn vector() {
  // ベクタで一連の値を保持する
    // 最初に見るコレクションは Vec<T> です
    // ベクタとも言いメモリ上に値を隣り合わせに並べる単独のデータ構造に2つ以上の値を保持させてくれる
    // ベクタには同じ型の値しか保持できない、要素のリストがある場合に有用
    // 例えばテキストファイルの各行やショッピングカートのアイテムの価格など

   // 新しいベクタを生成する
    // 新しい空のベクタを作るには Vec:new関数 を使う
    let v:Vec<i32> = Vec::new();
    // 値が何もないので型がわかるように注釈します

    // ベクタはジェネリクスを使用して実装されてる
    // 独自の型でジェネリクスを使用する方法についてはいずれ学ぶ
    // 今は std により提供されている Vec<T>型はどんな型でも保持でき、ベクタが特定の型を保持するとその型は <>内に指定されるということだけが分かれば良い
    // 一旦値を挿入したらコンパイラは保持させたい値の型を推論し続けられるので、このように型注釈をすることは滅多にない
    // 初期値のある Vec<T> を生成する方が一般的ですし Rust には vec!マクロも用意されている
    // vec!マクロは与えた値を保持する新しいベクタ型を生成します
	// 以下では 1, 2, 3 を持つ新しい Vec<i32> を生成している
	let v = vec![1, 2, 3];
	// 初期値として i32値を指定したのでコンパイラは v の型が Vec<i32> であると推論でき、型注釈は必要なくなる

   // ベクタを更新する
	// pushメソッドで要素を追加できます
	// もちろん値の改変には mut キーワードでの定義が必要です
	// 中に配置する数値は全てi32型であることをコンパイラをデータから推論するので Vec<i32> という注釈は必要なくなる
	let mut rgb = Vec::new();
	rgb.push(255);
	rgb.push(172);
	rgb.push(86);

   // ベクタをドロップすれば要素もドロップする
	// ベクタもスコープを抜ければ解放される
	{
		let test_vec = vec![1, 2, 3, 4];
		// ここでは test_vec は使用可能
	} // test_vec はここでスコープを抜けて解放される

	// ベクタがドロップされると中身もドロップされる。つまり保持されていた値が片付けられるということ
	// 一見単純なこと見えるかもしれないがベクタの要素への参照を導入した場合、少し複雑になるかもしれない

   // ベクタの要素を読む
	// 生成, 更新, 破棄 ができても使えなければ意味がない
	// 中身を読む方法は2種類在ります
	// 以下の例では分かりやすくする為に関数の返り値の型を注釈します
	// 以下は両メソッドがベクタの値に対して添字記法と getメソッドによりアクセスするところです
	let g:&i32 = &rgb[1]; // rgb[1] を g が借用する
	let g_ii:Option<&i32> = rgb.get(1); // 指数を引数として getメソッドに渡し Option<&T> を得る

	// ベクタに要素が含まれないインデックスの値を使用しようとした際プログラムの振る舞いを選択できる
	//let does_not_exist = &rgb[100]; // エラー
	// getメソッドがベクタ外の添え字を渡されると、パニックすることなくNoneを返します
	let offl = rgb.get(100);
	println!("{:?}", offl);//< None
	// ベクタの範囲外にアクセスする可能性がある場合にこのメソッドを使用することになる
	// そうしたらコードには Some(&element) か None を扱うロジックが存在することになる
	// 例えば人の入力した数値を input して添え字(インデックスとして)に入れることもできる
	// もし大きすぎる値を誤って入力してもプログラムが None値を得るので、それに応じて再度質問するようにできる

	// レかレ！
	let mut v_ii = vec![1, 2, 3, 4, 5]; 
	let first = &v_ii[0];
	v_ii.push(6);
	//println!("{:?}", first); // これはエラー
	// なぜ最初の要素への参照がベクタの終端への値追加に干渉されるのでしょう
	// 新規要素をベクタの終端に追加すると新しく追加する値分の領域を用意する必要がある(場合がある)
	// メモリの新規確保をして古い要素を新しいスペースにコピーすることになる
	// 移転されてしまっているので最初の要素を指す参照は解放されたメモリを指してしまう
	// エラーとなります

   // ベクタの値を走査する
	// ベクタの要素に順番にアクセスしたいなら添え字で1回に1要素にアクセスするのではなく全要素を走査することができます
	// 例えば以下だけで可能です
	let v_iii = vec![100, 32, 57];
	for i in &v_iii { println!("{}", i); }

	// 全要素に変更を加える目的で可変なベクタの各要素への可変な参照を走査することもできる
	// 以下では全要素に 50 を加算している
	let mut v_iv = vec![100, 32, 57];
	for i in &mut v_iv { *i += 50; }
	// 可変参照が参照している値を変更するには +=演算子を使用する前に参照外し演算子(*)を使用して i の値に辿り着かないといけない

   // Enumを使って複数の型を保持する
	// ベクタは同じ型の値しか保持できない
	// しかし enum の列挙子は同じ enum の型の元に定義される故ベクタに異なる型の要素を持たせたいなら enum を使用するとよい
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}
	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];
	// rowベクタの中身は全て SpreadsheetCell型となる
	// 列挙子は違うため別の型を保持できる
}

pub fn string() {
 // 文字列でUTF-8でエンコードされたテキストを保持する
	// 生成, 更新, 読込など他のコレクションも持っている様なStringの処理について学ぶ
	// また String が他のコレクションと異なる点についても学ぶ

  // 文字列とは?
	// 文字列リテラルはプログラムのバイナリ出力に格納され、バイナリのその特定の位置を指すスライスなので文字列スライスになる
	// String型は std により提供されいるが 伸長可能, 可変, 所有権のある UTF-8エンコードされた文字列型です
	// Rust において「文字列」は どちらかではなく String と&str(文字列スライス)のことを指す
	// また std には他の文字列型も含まれている 例 OsString, OsStr, CString, CStr etc...
	// それらの名前が全て String か Str で終わっているのは、所有権ありと借用されたバージョンを指している

  // 新規文字列を生成する
	// Vec<T> で出来る処理の殆どが String でも使える
	// 空の文字列の生成は　String:new関数です
	let mut s = String::new();

	// to_stringメソッドで文字列の初期値を決める
	let data = "初期値!";
	let s_ii = data.to_string();
	
	// 文字列リテラルに直接連結も可能
	let s_iii = "初期値!".to_string();

	// String::from関数でも同じようなことが可能
	let s_iv = String::from("初期値!");

	// 文字列は色々な所で使うので多くの異なる API が用意されていて、たくさんの選択肢がある
	// 文字列は UTF-8エンコードされているので以下の文字は全て有効です
	let hello_ar = String::from("السلام عليكم");
	let hello_cs = String::from("Dobrý den");
	let hello_en = String::from("Hello");
	let hello_hb = String::from("שָׁלוֹם");
	let hello_ay = String::from("贵樣");
	let hello_hi = String::from("नमस्ते");
	let hello_ja = String::from("こんにちは");
	let hello_ko = String::from("안녕하세요");
	let hello_zh = String::from("你好");
	let hello_pt = String::from("Olá");
	let hello_ru = String::from("Здравствуйте");
	let hello_spa = String::from("Hola");

  // 文字列を更新する
	// String はサイズを伸ばすことができ Vec<T> の中身のように追加のデータをプッシュすれば中身も変化する
	// つまり String値を連結する +演算子や format!マクロを使用することができるということ

   // 1.push_str と push で文字列に追加する
	// push_strメソッドで文字列スライスを追記することで String を伸ばすことができる
	let mut yushu = String::from("优秀の");
	yushu.push_str("人材");
	println!("{}", yushu); //< 优秀の人材

    // なお、push_str関数に値を利用されても所有権は奪われません
	let mut tapioka = String::from("夕匕オ力に");
	let towshi = "投資ずゑ";
	tapioka.push_str(towshi); // towshi はまだ使える
	println!("{}", tapioka); //< 夕匕オ力に投資ずゑ
	
	// pushメソッドでは char型で追加できます
	let mut kisama = String::from("贵");
	kisama.push('樣'); // char型なのでシングルクォート
	println!("{}", kisama);

   // 2.+演算子または format!マクロで連結
	// 2つの文字列(String)を組み合わせたい場合は +演算子で連結できる
	let hello = String::from("Hello, ");
	let world = String::from("World!");
	let h_w = hello + &world; // hello はムーブされて使えなくなる
	println!("{}", h_w); //< Hello, world!
	// +演算子(addメソッドで定義されている)では足す数(右)が &String でも &str に型強制する(関数の引数の仕様)
	// その後コンパイラは参照外し型強制をして &world を &world[..] に、つまり文字列スライスにします
	// 参照外し型強制についてはいずれ学ぶ
	// +演算子が引数の所有権を奪わないので world は引き続き使える
	// しかし足される数(左)の所有権は奪います
	// メソッドの引数部分の self に & がついていないからです
	// つまり上の +演算子による計算では hello の値を奪って world　の値をコピーして連結していることになる

	// 複数の文字列を連結しようとすると +演算子のこの仕様は面倒くさくなってきます
	let sin = String::from("sine");
	let cos = String::from("cosine");
	let tan = String::from("tangent");
	let trig_fn = sin + "-" + &cos + "-" + &tan;
	println!("{}", trig_fn); //< sine-cosine-tangent
	// 非常に見ずらいです

	// このような複雑な連結には format!マクロが有用
	// println!マクロのように使えますがスクリーン出力ではなく中身を String　で返す
	// 引数の所有権を奪わない上に見やすいです
	let sin_ii = String::from("sine"); // sin変数だけ死んでるので再定義
	let trig_fn_ii = format!("{}-{}-{}", sin_ii, cos, tan);
	println!("{}", trig_fn_ii); //< sine-cosine-tangent

  // 文字列に添え字アクセスする
	// 他の多くのプログラミング言語では文字列中の文字に添え字で参照してアクセスすることが出来きます
	// しかし Rust では添え字記法で String の一部にアクセスしようとするとエラる
	let reima = String::from("Makidmo");
	//let h = reima[0]; // エラー
	// これは Rust のメモリでの文字列保持方法によるもの

   // 内部表現
	// String は Vec<u8> のラッパ。
	// まずは下を見てください
	let len = String::from("Hola").len(); //< len=4
	// 文字列"Hola"を保持するベクタの長さが 4バイト(各文字1バイト)であるため len に 3 を代入する

	// しかし下はどうでしょう
	let len = String::from("Здравствуйте").len(); //< len=24
	// この文字列の先頭はアラビア算用数字の3ではなくキリル文字です
	// この文字列の長さは 24 です。1文字が1バイトとは限らない
	// それ故に文字列のバイトの添え字は必ずしも有効な Unicodeのスカラー値とは相互に関係しない

	// 以下も見てみましょう
	let hello = "Здравствуйте";
	//let answer = &hello[0]; //エラー
	// 添え字0で参照しようとするとなぜエラーなのか
	// UTF-8エンコードされた時にЗ(基本ラテン文字)の最初のバイトは208, 2番目は151になる故
	// answer変数は本来208になって欲しいところが 208 は単独では有効な文字ではない
	// この文字列の最初の文字を求めている場合は 208 を返すことはユーザの望んでいるものではないはず
	// しかし Rust にはバイト添え字0の位置には 208 しかない
	// 文字列がラテン文字のみを含む場合でもユーザはバイト値が返ることを望まないはず
	// &"hello"[0] がバイト値を返す有効なコードだったら、hではなく、104を返す
	// 予期しない値を返し気付きずらいバグを起こさないために Rust はコンパイルせずにエラーで止めてくれる

  // バイトとスカラー値と書記素クラスタ！wtf
	// UTF-8 についての別の特徴として Rust において文字列を見るには3つの関連した方法があるということ
	// バイトとして, スカラー値として, そして書記素クラスタ(人間が文字と呼ぶものに一番近い)としてです
	// ヒンディー語の単語 नमस्ते をデーヴァナーガリー(サンスクリット語とヒンディー語を書くときに使われる書記法)で表記したものを見たら
	// 以下のような見た目のu8値のベクタとして保持される
	// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
	// 18バイトになりこのようにコンピュータは最終的にこのデータを保持している

	// Unicodeスカラー値として見たら( Rust の char型もこれこのバイトは以下のような見た目になる
	// ['न', 'म', 'स', '्', 'त', 'े']
	// ここでは 6つ char値がありますが 4番目と 6番目は文字ではありません(単独では意味をなさないダイアクリティック)

	// 最後に書記素クラスタとして見たらこのヒンディー語の単語を使う人が 4文字と呼ぶであろうものが得られる
	// ["न", "म", "स्", "ते"]

	// Rustにはデータが表す自然言語だけでなく各プログラムが必要な解釈方法を選べるよう、
	// コンピュータが保持する raw の文字列データを解釈する方法がいろいろ用意されている

	// Rust で文字を得るのに String に添え字アクセスできない最後の理由は
	// 添え字アクセスという処理が常に定数時間(O(1))になると期待されるから
	// しかし String でその処理を保証することはできない。
	// というのも 合法な文字がいくつあるか決定するのに最初から添え字まで中身を走査する必要がある故

  // 文字列をスライスする
	// 文字列への添え字アクセスはしばしば悪い考えとなる
	// 文字列添え字処理の戻り値の型が明瞭ではないから
	// なりうる値が バイト値, 文字, 書記素クラスタ, 文字列スライス など沢山あるからですね

	// 故に文字列スライスを生成する時に添え字を使う必要が出た場合、
	// コンパイラがもっと特定するよう求めてくる。添え字アクセスを特定して文字列スライスが欲しいと示唆するためには、
	// []で1つの数値により添え字アクセスするのではなく範囲とともに[]を使って特定のバイトを含む文字列スライスを作ることができる
	let hello_ii = "Здравствуйте";
	let sadon = &hello_ii[0..4]; // hello_ii の文字は各2バイトなので4バイト分スライス
	println!("{}", sadon); //< Зд
	// &hello[0..1] とやると文字をぶった切る事になるのでエラー

	// &"hello"[0] (このhelloは文字列リテラル)がバイト値を返す有効なコードだったら、hではなく、104を返すでしょう。

  // 文字列を走査するメソッド群
	// 他の方法でも文字列の要素にアクセスすることができる
	// もし個々のUnicodeスカラー値に対して処理を行いたかったら charsメソッドを使用すると良い
	// नमस्ते に対して charsメソッドを呼び出したら分解して 6つの char型の値を返します
	let dada = "नमस्ते".chars();
	println!("{:?}", dada); //< Chars(['न', 'म', 'स', '\u{94d}', 'त', '\u{947}'])
	// 走査で1つ1つ出せる
	for c in dada { println!("{}", c); } //< न
										 //...म
										 //...स
										 //...्
										 //...त
										 //...े

	// bytesメソッドは各バイトをそのまま返す
	for b in "नमस्ते".bytes() { println!("{}", b); } //< 224
												   //...164
												   //-略
												   //...165
												   //...135
	
	// 計18バイトを返す

  // 文字列はそう単純じゃない
	// 文字列は複雑た！
	// これはプログラマが UTF-8データを扱う際に頭で考えないといけないということ
}

pub fn hash_map() {
 // キーとそれに紐づいた値をハッシュマップに格納する
	// 型 HashMap<K, V> は K型のキーとV型の値の対応関係を保持する/*#ハッシュマップ*/
	// これをハッシュ関数で行う。ハッシュ関数はキーと値のメモリ配置方法を決めるもの/*#ハッシュ関数*/
	// 他の言語でもこの様なデータ構造ありますが名前が違うことが多い。hash, map, object, ハッシュテーブル, 連想配列など...

	// ハッシュマップはベクタとは違い番号ではなくどんな型にもなれるキーを使ってデータを参照したいときに有用
	// 例えばゲームで各チームのスコアをハッシュマップで追いかけることができる
	// ここで各キー(K)はチーム名値(V)が各チームのスコアになり、チーム名が与えられればスコアを扱うことができる

	// 今回はハッシュマップの基礎的な API を見ていくが、他にも沢山の機能が std により HashMap<K, V> 上に定義された関数にある
	// それについては std.doc 参照

  // 新規ハッシュマップを生成する
	// 空のハッシュマップを new で作り要素を insert で追加できる( insert(K,V) )
	// 例では名前がブルーとイエローの2チームのスコアを追いかける
	// ブルーチームは10点、イエローチームは50点から始める
	use std::collections::HashMap; // 使用頻度の低いコレクションな為自動スコープに導入はされない
	let mut scores = HashMap::new(); // 空のハッシュマップ生成(insert するのでもちろん可変に)
	scores.insert(String::from("Blue"), 10); // 青チームのスコア: 10
	scores.insert(String::from("Yellow"), 50); // 黄チームのスコア: 50

	// ベクタと同じくハッシュマップはデータをヒープに保持する
	// この HashMap はキーが String型 値は i32型。ベクタと同じく均質
	// キーは全て同じ型で無ければならない上、値も全て同じ型でなければならない

	// ハッシュマップを生成する別の方法として タプルのベクタに対して collectメソッドを使用することも可能性
	// ここで各タプルはキーと値から構成されていて collectメソッドはいろんなコレクション型にデータをまとめ上げる。 HashMap も含まれています
	// 例としてチーム名と初期スコアが別々のベクタにあった場合 zipメソッドを使ってタプルのベクタを作り上げることができそこではブルーは10とペアになるなどします
	// それから collectメソッドを使ってそのタプルのベクタをハッシュマップに変換することができる
	let teams = vec![ // チーム名を保持するベクタ
		String::from("Blue"), 
		String::from("Yellow")
	];
	let initial_scores = vec![ 10, 50 ]; // 初期スコアを保持するベクタ
	let scores:HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); // teams の中身を順番に zipメソッドで initial_scores を値とするベクタを作り
																				  // collectメソッドでそのベクタをハッシュマップに変換する
	// ここでは、HashMap<_, _>という型注釈が必要になる
	// いろんなデータ構造にまとめ上げることが出来るのでコンパイラは指定しない限りどれになるか分からないが
	// キーと値の型注釈ではアンダースコアを使用しているのでコンパイラはベクタのデータ型に基づいてハッシュマップが含む型を推論することができる

  // ハッシュマップと所有権
	// i32 のような Copyトレイトを実装する型は値がハッシュマップにコピーされる
	// String のような所有権のある値ならば値はムーブされハッシュマップが所有権を持つ
	let field_name = String::from("お気に入りの色");
	let field_value = String::from("青色");
	let mut map = HashMap::new();
	map.insert(field_name, field_value); // ここで2つの変数の値は map にムーブされる
	// field_name と field_value はもう使えない
	// もちろん参照をハッシュマップに挿入すれば値はハッシュマップにムーブされない
	// 参照している値は最低でもハッシュマップが有効な間は死んではいけない。これらの問題について詳細いずれ学ぶ

  // ハッシュマップの値にアクセスする
	// getメソッドにキーを教えればハッシュマップから値を取り出すことができる
	let mut scores_ii = HashMap::new();
	scores_ii.insert(String::from("Blue"), 10);
	scores_ii.insert(String::from("Yellow"), 50);
	let team_name = String::from("Blue"); // 青チームのキー名を格納
	let blue_score = scores_ii.get(&team_name); // getメソッドに引数を介してキーを教えて blue_score に代入
	// ここでは blue_score はブルーチームに紐づけられた値になり結果は Some(&10) となる
	// 結果はSomeに包まれる。何故なら get は Option<&V> を返すから
	// キーに対応する値がハッシュマップになかったら get は None を返す。


  // ベクタと同じく forループでハッシュマップのキーと値のペアを走査できる
	let mut scores_iii = HashMap::new();
	scores_iii.insert(String::from("Blue"), 10);
	scores_iii.insert(String::from("Yellow"), 50);
	for (key, value) in &scores_iii {
		println!("{}: {}", key, value); //< Yellow: 50
										//..Blue: 10
	}

  // ハッシュマップを更新する
	// キーと値の数は伸長可能だが各キーには1回に1つの値しか紐づけることができない
	// ハッシュマップ内のデータを変えたい時はすでにキーに値が紐づいている場合の扱い方を決めなければいけない
	// 古い値を新しい値で置き換えて古い値を完全に無視することもできるし
	// 古い値を保持して新しい値を無視しキーにまだ値がない場合に新しい値を追加するだけにすることもできる
	// 他には古い値と新しい値を組み合わせることもできまる
	// 各方法について見ていく

   // 値を上書きする
	// キーと値をハッシュマップに挿入し同じキーを異なる値で挿入するればそのキーのパートナーの値は上書きされる
	let mut scores_iv = HashMap::new();
	scores_iv.insert(String::from("Blue"), 10);
	scores_iv.insert(String::from("Blue"), 25); // 上書き
	println!("{:?}", scores_iv); //< {"Blue": 25}

   // キーに値がなかった時のみ値を挿入する
	// キーに値があるか確認することや値が存在しない時に値を挿入することはよくある
	// ハッシュマップにはこれを行うための entry と呼ばれる なAPI がある
	// これは引数としてチェックしたいキーを取る
	// entryメソッドの戻り値は Entry という enum で、これは存在するかの列挙値を持つ
	// イエローチームのキーに値が紐づけられていなかったら値に 50 を挿入, ブルーチームに対しても同様の処理をしてみる
	let mut scores_v = HashMap::new();
	scores_v.insert(String::from("Blue"), 10); // Blue に 10 を紐づける
	scores_v.entry(String::from("Yellow")).or_insert(50); // 値がないので引数にある50を挿入
	scores_v.entry(String::from("Blue")).or_insert(50); // すでに値があるのでスルー
	println!("{:?}", scores_v); //< {"Blue": 10, "Yellow": 50}
	// or_insertメソッドはキーに値があった場合にそのキーの値への可変参照を返し、
	// キーに値が無いという列挙子が見えたら or_insertメソッドの引数をそのキーの新しい値として挿入して新しい値への可変参照を返す

   // 古い値に基づいて値を更新する
	// ハッシュマップの他の使用例はとしてキーの値を探し古い値に基づいてそれを更新すること
	// 以下のコードは各単語がある変数に何回出現するかを数え上げるコードを示している
	// キーに単語を入れたハッシュマップを使ってその単語を何回見かけたかを追跡する
	// 始めて単語を見かけたら まず0という値を挿入する
	let text = "hello world wonderful world";
	let mut map_ii = HashMap::new();
	for word in text.split_whitespace() { // text を split_whitespace関数で空白で区切って1つずつ返して word に投げる
		let count = map_ii.entry(word).or_insert(0); // map_ii に走査されてきた単語についての記録が無ければ0を単語として追加して次のループ
													 // 値があればスルーして残りの処理を(可変参照を返す)
		*count += 1; // count が可変参照を保持しているので参照外しをして
	}
	println!("{:?}", map); //< {"world": 2, "hello": 1, "wonderful": 1}
	// or_insert関数は元々 キーに対する値への可変参照(&mut V)を返す
	// ここでその可変参照を count変数に保持しているのでその値に代入するにはまず * で count を参照外ししなければならない

   // ハッシュ関数
	// 標準では HashMap は DoS に対して抵抗を示す暗号学的に安全なハッシュ関数を使用する
	// これは速いハッシュアルゴリズムではないがパフォーマンスの欠落と引き換えに安全性を得るというトレードオフは価値があｒｙ
	// 自分のコードをプロファイリングして自分の目的では標準のハッシュ関数は遅すぎると判明したら異なるhasherを指定することで別の関数に切り替えることができる/*#hasher*/
	// hasher とは BuildHasherトレイトを実装する型のこと。トレイトについてとその実装方法についてはいずれ学ぶ
	// トレイトを実装する型のことです。トレイトについてとその実装方法については、第10章で語ります。 必ずしも独自のhasherを1から作り上げる必要はありません;
	// 必ずしも独自の hasher を1から作り上げる必要はない
	// crates.ioに、他のRustユーザによって共有された多くの一般的なハッシュアルゴリズムを実装した hasher を提供するライブラリがある

// *:+-_ 実践 _-+:*
  // 平均値, 中央値, 最頻値 の算出
   // 定義
	mod statistics {
		//平均値
		pub fn mean(i_l:&[i32]) -> f64 {
			use std::collections::HashMap;
			let mut material_mean:HashMap<&str,i32> = HashMap::new();
			material_mean.entry("sum").or_insert(0);
			for e in i_l.iter() {
				let total = material_mean.entry("sum").or_insert(*e);
				*total += *e;
			}
			material_mean.entry("num of elem").or_insert(i_l.len() as i32);
			let result = *material_mean.get("sum").unwrap() as f64 / *material_mean.get("num of elem").unwrap() as f64;
			return result
		}
		// 中央値
		pub fn median(i_l:&mut [i32]) -> f64 {
			i_l.sort();
			let mid = i_l.len()/2;
			if i_l.len()%2 == 0 { // eve
				return ( i_l[mid-1] as f64  + i_l[mid] as f64 )/ 2.
			} else { // odd
				return i_l[mid] as f64
			}
		}
		// 最頻値
		pub fn mode(i_l:& [i32]) -> Vec<i32> {
			use std::collections::HashMap;
			let mut num_count:HashMap<String,i32> = HashMap::new();
			for n in i_l.iter() {
				let ct = num_count.entry(n.to_string()).or_insert(0);
				*ct += 1;
			}
			let mut ct_vec:Vec<&i32> = Vec::new();
			for (k, v) in num_count.iter() { ct_vec.push(v); }
			let mut result:Vec<i32> = Vec::new();
			for (k, v) in  num_count.iter() {
				if v == ct_vec.iter().max().unwrap() as &i32 { 
					result.push( k.parse().unwrap() ); 
				}
			}
			result.sort();
			return result
		}
	}

   // 使用例
	// 平均値
	let int_i = [ 43, 57, 57, 63, 66, 82, 98 ];
	println!("平均値: {}", statistics::mean(&int_i)); //< 平均値: 66.57142857142857

	// 中央値
	let mut int_ii_eve = [ 57, 43, 63, 57, 98, 66, 82, 100 ];
	let mut int_ii_odd = [ 57, 43, 63, 57, 98, 66, 82 ];
	println!("中央値1: {}", statistics::median(&mut int_ii_eve)); // < 中央値1: 64.5
	println!("中央値2: {}", statistics::median(&mut int_ii_odd)); //< 中央値2: 63

	// 最頻値
	let int_iii_a = [ 43, 57, 57, 63, 66, 82, 98 ];
	let int_iii_b = [ 43, 57, 57, 63, 66, 66, 98 ];
	let int_iii_c = [ 0, 1, 2, 3, 4, 5 ];
	println!("最頻値1: {:?}", statistics::mode(&int_iii_a)); //< 最頻値1: [57]
	println!("最頻値2: {:?}", statistics::mode(&int_iii_b)); //< 最頻値2: [57, 66]
	println!("最頻値3: {:?}", statistics::mode(&int_iii_c)); //< 最頻値3: [0, 1, 2, 3, 4, 5]

  // 単語のピッグ・ラテンへの簡易変換
   // 定義
	mod pig_latin {
		pub fn word_to(txt:&str) -> String {
			let mut is_vowel = false;
			let first_char = txt.chars().next().unwrap();
			if "AaIiUuEeOo".contains(first_char) { is_vowel = true; }
			if is_vowel {
				return format!("{}hay", txt)
			} else {
				let origin:String = txt.chars().take(txt.len()).skip(1).collect(); // Hello -> ello
				return format!(
					"{origin}{flex}ay", 
					origin = origin, 
					flex = first_char
				)
			}
		}
	}
   // 使用例
	println!("{}", pig_latin::word_to("hello")); // ellohay
	println!("{}", pig_latin::word_to("japan")); // apanjay
	println!("{}", pig_latin::word_to("enum")); // enumhay
	println!("{}", pig_latin::word_to("open")); // openhay
	println!("{}", pig_latin::word_to("贵樣")); // 樣贵ay
}