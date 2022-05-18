# ltmod-ntcrack

Left To My Own Devices - NT cracker

A full writeup of how it works is available at [the SensePost blog](https://sensepost.com/blog/2022/left-to-my-own-devices-fast-ntcracking-in-rust/)

# Invocation

`./ntcrack <input hashlist> <wordlist>`

e.g.

`./ntcrack crackme.hashes rockyou.txt`

It expects the hashes to be NT hashes one per line, with nothing else. So strip out hashcat or john mode information.

# Compilation

`cargo build --release`

You'll find it in `target/release/ntcrack`

If you don't have rust and cargo, the easiest way to get it is with [rustup](https://rustup.rs).

# Prerequisites

Apart from needing hashes and a wordlist, not much.

# Tuning

The code makes reasonable choices for tuning, but you could improve it for your system with some testing.

There are three primary performance stats you can use to tune efficiency:

* Crack speed (aka number of kilo hashes generated per second)
* Read speed (aka the number of megabytes read from the wordlist per second)
* Wait speed (aka the number of times a thread sat waiting for a chunk)

If the read speed is low, check the cache and block size. If the waits are high, and you've checked the cache and block size, then experiment with changing the chunk size. Crack speed should be the result of doing those successfully.

## Block Size for Disk Reads

Right now it's using a block size to read files with of 8M. You can test which is best for your system with something like the following, and seeing which is fastest:

`for x in 1M 1M 2M 4M 8M 12M; do time dd if=somefile of=/dev/null bs=$x; done`

I run 1M twice in the above to get the file cache'd to kernel for consisten results.

## Cache Size

Assuming 16G of ram, on a Mac you get about 10G of file cache, and on Linux it looks like it can grow to fill available RAM. In both cases this drops if you have memory intensive stuff loaded (Microsoft Teams is my goto test).

I've set it to 2G, which should be reasonable for most systems. But if you have significantly more or less RAM change it.

## Chunk Size

The chunk size controls the size of the chunks of the wordlist that the main program sends to the threads. I've done pretty extensive testing on a small number of systems and I think this is a good value.

If you want to do some tests, uncomment the code to switch it to an argument and do some of your own testing.

I'd recommend changing this last, and checking block and cache size first.
