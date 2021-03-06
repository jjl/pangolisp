# pangolisp

When it's scared, it curls its little parens up into a ball.

## Status: pile of hurriedly thrown together code

Doesn't work, barely compiles, is not safe for production etc. etc.

## What will it be?

A lisp for generating rust programs.

While we are bootstrapping, this will be a fairly straightforward
dynamic language with an api for generating rust code files. Over
time, we intend to experiment and see what happens, but we'll probably
ultimately target MIR or WASM or something.

We intend to add and extend a strong, static type system with
inference. Possibly even dependent typesthrough normalisation by
evaluation. We'll see where it goes.

We want to add support for the rust language server to be able to
introspect existing rust code from pangolisp code.

## Syntax?

It looks a fair bit like that lisp that runs on the jvm whose fans
won't shut up. But it's quite different to use. And definitely doesn't
run on the JVM.

We will be experimenting with syntax a lot. We're currently considering:

```lisp
;;; reader macro lambda
\x (+ x x)
\x \y (+ x y)
```

## Copyright and License

Copyright (c) 2021 James Laver, pangolisp contributors

[Licensed](LICENSE) under Apache License, Version 2.0
(https://www.apache.org/licenses/LICENSE-2.0), with LLVM Exceptions
(https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
