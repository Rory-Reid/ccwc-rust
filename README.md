# ccwc in Rust

This repository [implements a wc-inspired application as part of a coding challenge](https://codingchallenges.fyi/challenges/challenge-wc) in Rust.

## Decisions

### Buffer

In an interesting exercise of self-gaslighting, I had convinced myself that the coding challenge had some late-game stretch goal suggesting that you try to make it work with a 100gb file. I'm not made of money and my RAM is too small for that, so I decided that manually buffering it into memory a chunk at a time would be better than relying on some handy "read file as string" method provided to me (and yes, I understand that can result in a slower process due to increased IO calls). On re-reading the challenge, I can't find any mention of it and cannot imagine where I pulled this requirement from.

Luckily, this is a coding challenge I'm doing exclusively in the evening to learn rust, and I'm not being paid for it, so who cares.

Also I picked a maximum 100mb buffer size based on an arbitrary "you won't notice it missing from your ram, but that's probably enough for now" without overthinking it too much.

Note: I'm pretty sure I've introduced a string allocation which potentially quintuples the memory (rust stores characters in memory as 4 bytes per character. If you have 100mb of 1-byte ascii characters, this will buffer those fully, then allocate 400mb as a string variant). Should probably adjust this to 1/5th of the size.

### Encoding

Because I do not have the willpower or energy to do otherwise, I am only making this work with UTF-8 encoding.

### Parity with `wc`

Where possible this aims for parity with whatever version of `wc` sits on my mac. There is known deviation in the following areas:

#### Character/Byte counting

The `-w` and `-c` flags are mutually exclusive. You can specify both but `wc` will display ONLY one or the other. The `man` page for it states:

```
-c      The number of bytes in each input file is written to the standard output.  This will cancel out any prior usage of the -m option.
-m      The number of characters in each input file is written to the standard output.  If the current locale does not support multibyte characters, this is equivalent to the -c option.  This will cancel out
any prior usage of the -c option.
```

The key there is "cancels out prior usage" which means if you specify `-mc` then `-c` takes precedence, and it will output bytes, and if you specify `-cm`, the opposite happens.

Since I'm not handrolling my argument parsing, the coding challenge doesn't specify absolute parity, and frankly I cannot be bothered to do otherwise, I am making a decision to apply `-m` in all circumstances if specified, regardless of the presence of `-c`.