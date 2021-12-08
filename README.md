# Kana-Kilo

This is a web app to help learn the Japanese Hiragana and Katakana characters.

This takes *heavy* inspiration from 
[the type-kana project](https://github.com/fleon/type-kana) and
[this fork](https://github.com/Cody-Duncan/type-kana) of that project.
At the time of this project's start these projects were no longer being maintained,
and I took this as an opportunity to learn how to build a website. 

Enjoy!


## Implementation Details

This is built on top of the [Yew](https://yew.rs/) framework. The code is compiled
using their preferred tool, [trunk](https://trunkrs.dev/) although I am sure that
an alternative tool that compiles to WASM could be swapped in.

The Kana -> Romanji mapping is recorded within the `kana_to_romanji.csv` file
located at the project root. Currently, this file supports a single alternative
romanji value. This file automatically gets generated into rust code at compile 
time.

