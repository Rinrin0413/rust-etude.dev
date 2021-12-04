use std::convert::TryInto; // uX を iX にするモジュール

pub fn scala() {

// スカラー型は値(単体の形)です。
// 整数, 浮動小数点数, 論理値, 文字があります

   // 1.整数型は整数の型
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
}