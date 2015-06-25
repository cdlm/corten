# Corten — an experiment in implementing Forth in Rust

Corten is currently not much more than the first draft of an experiment for
discovering [Rust][] by bootstrapping a minimal programming system, and as far
as minimal goes, it's hard to beat [Forth][]. Also, I've been on a
[Canon Cat][cat] research binge, recently.

In fact, I'm a little stumped with the ownership system: there is a cycle of
borrows between the interpreter, the vocabulary, and individual words. I have
way more design intuition with dynamically-typed OO than with any other
paradigm, so I'm interested in understanding how to re-architect the program the
Rust way.

[rust]: http://rustlang.org "the Rust programming language"
[forth]: http://git.annexia.org/?p=jonesforth.git;f=jonesforth.S;hb=66c56998125f3ac265a3a1df9821fd52cfeee8cc "Richard W.M. Jones's literate assembly Forth"
[cat]: http://www.canoncat.org "Jef Raskin's writing information appliance"
