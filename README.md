# Qreki
新暦(グレゴリオ暦)から旧暦と六曜を計算します。

## このプログラムについて
このプログラムは、高野英明氏のQREKI.AWKを勉強をかねてrustに移植したものです。
2020年1月1日から2050年12月31日までQREKI.AWKと同じ結果が得られることをテストしておりますが、正確性を保証するものではありませんのでご注意ください。

このプログラムのもとになったqreki.awkとqreki.docをsrc/qrsamp11に同梱しております。
また、同じくpythonに移植されたIto Shunsuke氏のqreki_pyを大変参考にさせていただきました。

qreki.awk
https://www.vector.co.jp/soft/dos/personal/se016093.html

qreki_py
https://github.com/fgshun/qreki_py



## 使用法
```rust
use qreki::Qreki;
fn main(){
    // システム時計の日付における旧暦
    let qreki_today= Qreki::new();

    // 任意の日付における旧暦
    let year=2021;
    let month=3;
    let day=31;
    let qreki_ymd=Qreki::from_ymd(year,month,day);

    // 得られた旧暦の年月日
    let qyear=qreki_ymd.qy;
    let qmonth=qreki_ymd.qm;
    let qday=qreki_ymd.qd;
    // 得られた旧暦が属する月が閏月か
    let is_leap=qreki_ymd.ql;
    // 六曜
    let rokuyou=qreki.rokuyou();
    println!(
        "西暦{}年{}月{}日は旧暦{}年{}月{}日{}です。",
        year,month,day,qyear,qmonth,qday,rokuyou
    );
}  
```

