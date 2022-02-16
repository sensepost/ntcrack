# ltmod-ntcrack

Left To My Own Devices - NT cracker

# Invocation

`./ntcrack <input hashlist> < <wordlist>`

e.g.

`./ntcrack crackme.hashes < rockyou.txt`

It expects the hashes to be NT hashes one per line, with nothing else. So strip out hashcat or john mode information.

*NB* The wordlist needs unix line breaks, aka \n. You can convert a Windows line-broken file with the `dos2unix` utility.

# Compilation

`cargo build --release`

You'll find it in `target/release/ntcrack`

If you don't have rust and cargo, the easiest way to get it is with [rustup](https://rustup.rs).

# Prerequisites

Apart from needing hashes and a wordlist, not much.

I've only ever tested this on my Mac, but most all the rust crates are portable, so it should work elsewhere.

# Threads FYI

Right now it uses 10 threads, because that's how many my computer has. You can edit the `threadnum` variable to change that.
