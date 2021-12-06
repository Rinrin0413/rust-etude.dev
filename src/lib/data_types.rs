use std::convert::TryInto; // uX を iX にするモジュール

// データ型

pub fn scala() {
  // スカラー型とは単独の値を持つ型のこと
    // 整数, 浮動小数点数, 論理値, 文字があります

   // 1.整数型(int)
    // その名の通り整数の型のことです
    // 整数型には符号付きと符号なしがあり数値が正負を持つかどうかを示します
    //    大きさ	符号付き(正負持)  符号なし   最大値
    //    8bit	    i8	            u8        127(i)    255(u)
    //    16bit	    i16	            u16       32767(i)   65535(u)
    //    32bit	    i32	            u32       2147483647(i)  4294967295(u)
    //    64bit	    i64	            u64       9223372036854775807(i) 18446744073709551615(u)    
    //    arch	    isize	        usize　　 コンピュータの種類依存 ex:64bitなら64bit
    // 負の数は符号付にしか無く最小値は  -(2**(bit- 1))  で求められる
    // 符号無の最小値は自然数で最も小さい数である 0
    // 符号付の最大値は  2**(bit-1)-1  で
    // 符号無の最大値は    2**bit-1    で求められる
    let i_hachchi:i8 = -128; // -129でとエラー
    let u_hachi:u8 = 255; // 256だとエラー
    println!("i8最小値:{}\nu8最大値{}", i_hachchi, u_hachi);

    // u8 を i8 に変えることもできるが、最大値の違いに注意
    let u_turnto_i:u8 = 127;
    let u_turnto_i:i8 = u_turnto_i.try_into().unwrap(); // これでもう符号付なので負の数を扱える
    println!("{}", -u_turnto_i); // 負の数にしてみる

     // i8 や u8 の注釈は直に数字にでも付けられる。: は不必要
    let i_hachi_dayo = 127i8;
    let u_hachi_dayo = 0u8;
    println!("i8の最大値:{}\nu8の最小値:{}", i_hachi_dayo, u_hachi_dayo);

    //以下は整数リテラル。数字の手前に付けて使う
    //10進数	        //何もつけないと10進数
    //16進数	        0x
    //8進数	            0o
    //2進数	            0b
    //バイト(u8-only)   b" "
    // バイトは違って b"文字" || b"\x<数字>"
    let binary = 0b1111_0000; // 240の二進数
    println!("{0:x}", binary); // 16進数にしてみる
    println!("{:?}", b"\xf0"); // バイトは :? が必要で b"\x<hex>""

   // 2.浮動小数点型(float)
    // 浮動小数点数とは整数に小数がついたもの
    // これは f32 と f64(基準型) があり、それぞれ 32bit と 64bit
	// f32 は単精度浮動小数点数、f64 は倍精度浮動小数点数 とも言う
	let f = 16.0; // f64(基準が f64 なので注釈は必要ない)
    println!("{}", f);
    let f_sanjuni:f32 = 8.0; // f32
    println!("{}", f_sanjuni);

   // 数値演算
	const X:i32 = 64;
	const Z:i32 = 86;

    // 四則演算と除法の余りを算出できる
	let sum = X + Z; // 加法
	println!("{}", sum);
	let diff = X - Z; // 減法
	println!("{}", diff);
	let prod = X*Z; // 乗法
	println!("{}", prod);
	let multipl = X/Z; // 除法
	println!("{}", multipl);
	let remain = X % Z; // 除法の余り
	println!("{}", remain);

   // 3.論理値型(bool)
	// 真偽値のこと。if文などで使う
	// 取りうる値は true と false の2つのみ
	let bool_t:bool = true; // 真
    let bool_f:bool = false; // 偽
	println!("{0} or {1}", bool_t, bool_f);
	// 注釈を入れているが別に必須ではない

   // 4.文字型(char)
	// char型は1文字を扱える型です
	// char型ではダブルクォーテーションではなくシングルクォーテーションを使います
    let str_char0 = 'A'; // U+0041
    let str_char1 = 'z'; // U+007A
    let str_char2 = '🤓'; // U+1F913(ｵﾁﾞﾀｿ🤓)
	let str_char3 = '�'; // U+FFFD
	println!(
		"{}{}{}{}",
		str_char0,
		str_char1,
		str_char2,
		str_char3
	);
}

pub fn complex() {
    // [複合型] (Complex)
      // 複数の値を1つの型にまとめたもの
  
     // 1.タプル型
      // 丸括弧の中にカンマ区切りのリストを書いて生成する
      // タプルの位置ごとに型を設定でき、 タプル内の値はそれぞれ別々の型でも良き。
      let complex_tuple:(i32, f64, bool, char) = (128, 10.24, true, 'X');
      println!("{:?}", complex_tuple); // そのまま出力するには :? が必要
      // ここでは、注釈をあえて(説明の為)追加
}