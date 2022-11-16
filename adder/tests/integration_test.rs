    // 取り敢えず以下のコードを置く

    extern crate adder;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, adder::add_two_iv(2));
    }

    // コードの頂点に `extern crate adder` とあるが、これは各々ライブラリをインポートしている
    // 今回の場合は adderライブラリをインポートしている
