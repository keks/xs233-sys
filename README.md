# xs233-sys

A low-level FFI crate for [c-xs233], which implements the curves in [this paper].

c-xs233 implements binary elliptic curves with ~112 bits of security. Binary elliptic curves are defined over binary fields instead of prime fields. More or less all commonly used curves are defined over prime fields. This is not because binary fields are inherently less secure. There are theoretical results that indicate that there might be a security difference, but there are no practical attacks. In the 1990s binary elliptic curves were covered by patents, but it seems like these are not relevant anymore (I am not a lawyer).

[c-xs233]: https://github.com/pornin/c-xs233
[this paper]: https://eprint.iacr.org/2022/1325.pdf
