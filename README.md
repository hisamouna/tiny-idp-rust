# What is

[フルスクラッチして理解するOpenID Connect](https://www.m3tech.blog/entry/2024/03/05/150000)をRustで実装してみた

## How to

自分のアプリケーション(RelyingParty)を起動

```sh
┌───────────────────>
│tiny-idp-rust on  main
└─> cargo run -p tiny-rp
```

IdP起動

```sh
┌───────────────────>
│tiny-idp-rust on  main
└─> cargo run -p tiny-idp
```

### (1)

[フルスクラッチして理解するOpenID Connect (1) 認可エンドポイント編](https://www.m3tech.blog/entry/2024/03/05/150000)

> http://localhost:3000/openid-connect/auth?client_id=tiny-client&redirect_uri=http://localhost:4000/oidc/callback&scope=openid&response_type=code ログイン画面よりemail / passwordを入力すると、認可コードが表示できました。

これができた。

## Reference

axum
- https://synamon.hatenablog.com/entry/rust-server-framework-comparison

askama
- https://blog.ojisan.io/use-template-for-axum/

OpenID Connect
- https://www.m3tech.blog/entry/2024/03/05/150000
- https://docs.rs/openidconnect/latest/openidconnect/

OIDC
- https://zenn.dev/ymtdzzz/articles/13e18cdf6b9ee8

About Rust module
- https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/
- https://zenn.dev/newgyu/articles/3b4677b408676$$8

Global Variable
- https://okchan08.hateblo.jp/entry/2022/02/16/190000
    mutex
- https://qiita.com/kujirahand/items/d7f6bae84a66ab4c783d
