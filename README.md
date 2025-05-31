# rollers, dice parsing utilities

## Cargo Workspace

Child crates are located under the [`./crates`](./crates/) folder.

## License

See [COPYRIGHT](./COPYRIGHT).

### Referenced Material

This project wouldn't be possible without the insights provided by the following resources:

- Inanna Malick's (@inanna-malick)'s [Recursion Scheme series from her blog recursion.wtf](https://recursion.wtf//tags/recursion-schemes/) and her MIT-or-Apache-2.0 licensed `recursion` crate the inanna-malick/recursion repository
- Yann Hamdaoui's blog post [Practical recursion schemes in Rust: traversing and extending trees](https://www.tweag.io/blog/2025-04-10-rust-recursion-schemes/)
- Adrian Sampson's (@sampsyo) article: [Flattening ASTs (and Other Compiler Data Structures)](https://www.cs.cornell.edu/~asampson/blog/flattening.html), and his MIT-licensed sampsyo/flatcalc repository
- Theodore Norvell's article: [Parsing Expressions by Recursive Descent](https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm)
- Alex Kladov's (@matklad) article: [Simple but Powerful Pratt Parsing](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html) and the accompanying MIT-or-Apache-2.0 licensed matklad/minipratt repository

### Inspirations

- The pest-parser/pest crate, which uses PEG grammars for generating parsers (formerly used).
- The many PRs by user @39555 implementing pratt parsing for winnow, including her contributions in: winnow-rs/winnow#622, winnow-rs/winnow#620, winnow-rs/winnow#618, winnow-rs/winnow#614
