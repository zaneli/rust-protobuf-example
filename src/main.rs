extern crate protobuf;

use protobuf::error::ProtobufError;

mod addressbook;
mod add_person;
mod list_people;

use std::{env, process};
use std::io::{self, stderr, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    get_module_name(&args)
    // io::Error -> ProtobufErrorに変換するため各所にこのような行があります。
    // あまり ProtobufErrorの実装が良くない(本来ならFrom<io::Error>を実装すべき)のでこうなってます。
    // ちゃんと管理しようと思うとこうなるよ、というデモなので面倒なら前のBox<Error>のままで構わないです。
        .map_err(ProtobufError::IoError)
        .and_then(|f|
                  get_file_path(&args)
                  .map_err(ProtobufError::IoError)
                  .and_then(f))
        .unwrap_or_else(|e| {
            // エラーハンドリングは通常Error型に対するパターンマッチで行います。
            // ただ、そのためにはBox<Error>のようにトレイトで隠蔽してしまうとハンドリング出来なくなるので
            // io::Errorなど具体的な型を使う必要がありす。
            // ただ、今回はProtobufErrorがio::Errorから変換出来るので具体的に扱えそうですね。
            // サンプルコードくらいでわざわざカッチり管理するのが面倒な時は`()`や`String`が`Error`を実装しているのでそれでどうにかするというのも手です。
            //
            // 因みにpanicからのrecoverはかなりの特殊用途向けなので基本的に使ってはいけません。
            stderr().write_fmt(format_args!("{}\n", e)).unwrap();
            process::exit(-1);
        })
}

fn get_module_name(args: &Vec<String>) -> Result<fn(&str) -> Result<(), ProtobufError>, io::Error> {
    if args.len() >= 2 {
        // AsRef::as_refはメソッドとして（第一引数がselfとして）定義されているので
        // String::as_ref(&args[1])よりもargs[1].as_ref()が好ましいというか正しい書き方です。
        // あるいはString -> &strを作るのには `&args[1][..]`のように`&s[..]`、あるいは
        // `&*args[1]`のように`&*s` も使えます。どれもゼロコピーです。
        // Stringと&strは似た型なために様々な方法で変換出来てしまいますが好きな方法を1つ覚えておけばいいと思います。
        //
        // 多くの場合、Derefによって`&s`が&strに自動で変換されますが([`Deref` による型強制](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/deref-coercions.html))、
        // 型がはっきりしない所では&Stringのままなので手で変換してあげる必要がでてきます。
        // まあ、これは型が違うよと怒られたら修正したらいいだけなのでそういうものです。
        //
        // 因みにStrnigとstrと&strの違いですが、ひとまず別の型です。相互に変換するメソッドが用意されてるだけです。
        // strは便宜的に存在しているだけで実際は&strの形で使います。
        // Strinと&strは[文字列](https://rust-lang-ja.github.io/the-rust-programming-language-ja/1.6/book/strings.html)に詳しく書いてありますが変更可能かどうかの違いですね。
        // リテラルは変更出来ないので必ず&strです。
        match args[1].as_ref() {
            "add_person" => Ok(add_person::execute),
            "list_people" => Ok(list_people::execute),
            other => {
                Err(io::Error::new(io::ErrorKind::NotFound,
                                   // formatの返り値はStringなので変換は不要です。
                                   format!("Unexpected module name: {}. (expected 'add_person' or \
                                            'list_people')", other)))
            }
        }
    } else {
        // &strをStringに変換するのにfromを使っていたようですが、to_string()でも大丈夫です。
        // io::Errorには秘技Otherが存在するのであらゆるエラーをio::Errorに変換出来ます。
        // あまり褒められたものではありませんが。
        Err(io::Error::new(io::ErrorKind::Other, "Usage: cargo run <module_name> <file_path>".to_string()))
    }
}

fn get_file_path(args: &Vec<String>) -> Result<&str, io::Error> {
    if args.len() >= 3 {
        Ok(&args[2])
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Usage: cargo run <module_name> <file_path>".to_string()))
    }
}
