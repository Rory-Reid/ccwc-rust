# ccwc in Rust

This repository [implements a wc-inspired application as part of a coding challenge](https://codingchallenges.fyi/challenges/challenge-wc) in Rust.

## Decisions

### Buffer

In an interesting exercise of self-gaslighting, I had convinced myself that the coding challenge had some late-game stretch goal suggesting that you try to make it work with a 100gb file. I'm not made of money and my RAM is too small for that, so I decided that manually buffering it into memory a chunk at a time would be better than relying on some handy "read file as string" method provided to me (and yes, I understand that can result in a slower process due to increased IO calls). On re-reading the challenge, I can't find any mention of it and cannot imagine where I pulled this requirement from.

Luckily, this is a coding challenge I'm doing exclusively in the evening to learn rust, and I'm not being paid for it, so who cares.

Also I picked a maximum 100mb buffer size based on an arbitrary "you won't notice it missing from your ram, but that's probably enough for now" without overthinking it too much.