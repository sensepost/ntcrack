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

# Speed Benchmarks

Hyperfine runs comparing ntcrack to hashcat.
Run on my M1 Pro MBP.
Prepare drops the wordlist from file cache, but puts the binary and hashlist in file cache.
Run after hashcat has done its custom wordlist cache'ing.

Keeping these here so I have some version control.

```
[+] First hash in rockyou wordlist
Benchmark 1: target/release/ntcrack first  wordlists/rockyou.txt
  Time (mean ± σ):      51.2 ms ±   0.7 ms    [User: 225.8 ms, System: 20.7 ms]
  Range (min … max):    50.1 ms …  52.4 ms    10 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet first wordlists/rockyou.txt
  Time (mean ± σ):      1.302 s ±  0.017 s    [User: 0.095 s, System: 0.200 s]
  Range (min … max):    1.279 s …  1.338 s    10 runs
 
[+] Last hash in rockyou wordlist
Benchmark 1: target/release/ntcrack last  wordlists/rockyou.txt
  Time (mean ± σ):     193.2 ms ±   1.1 ms    [User: 1575.8 ms, System: 41.2 ms]
  Range (min … max):   191.7 ms … 195.7 ms    10 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet last wordlists/rockyou.txt
  Time (mean ± σ):      1.301 s ±  0.007 s    [User: 0.506 s, System: 0.219 s]
  Range (min … max):    1.285 s …  1.310 s    10 runs
 
[+] 143 hash list against rockyou wordlist
Benchmark 1: target/release/ntcrack test.hashes  wordlists/rockyou.txt
  Time (mean ± σ):     229.8 ms ±   1.7 ms    [User: 1922.2 ms, System: 43.0 ms]
  Range (min … max):   227.7 ms … 233.0 ms    10 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet test.hashes wordlists/rockyou.txt
  Time (mean ± σ):      1.295 s ±  0.007 s    [User: 0.507 s, System: 0.218 s]
  Range (min … max):    1.287 s …  1.304 s    10 runs
 
[+] All rockyou hashes against rockyou wordlist
Benchmark 1: target/release/ntcrack nthasher/rockyou.txt.utf8.hashes  wordlists/rockyou.txt
  Time (mean ± σ):      4.911 s ±  0.007 s    [User: 14.084 s, System: 0.538 s]
  Range (min … max):    4.903 s …  4.922 s    5 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet nthasher/rockyou.txt.utf8.hashes wordlists/rockyou.txt
  Time (mean ± σ):     560.816 s ±  7.839 s    [User: 245.798 s, System: 338.184 s]
  Range (min … max):   555.273 s … 566.359 s    2 runs
 
[+] 143 hash list against insidepro 1G wordlist
Benchmark 1: target/release/ntcrack test.hashes  wordlists/insidepro.dic
  Time (mean ± σ):      1.655 s ±  0.006 s    [User: 14.214 s, System: 0.310 s]
  Range (min … max):    1.647 s …  1.664 s    5 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet test.hashes wordlists/insidepro.dic
  Time (mean ± σ):      5.339 s ±  0.002 s    [User: 3.228 s, System: 0.348 s]
  Range (min … max):    5.336 s …  5.341 s    5 runs
 
[+] 143 hash list against 5G wordlist
Benchmark 1: target/release/ntcrack test.hashes  small
  Time (mean ± σ):      5.820 s ±  0.003 s    [User: 52.020 s, System: 1.623 s]
  Range (min … max):    5.816 s …  5.822 s    3 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet test.hashes small
  Time (mean ± σ):     14.427 s ±  0.005 s    [User: 10.101 s, System: 0.908 s]
  Range (min … max):   14.423 s … 14.433 s    3 runs
 
[+] 143 hash list against 11G wordlist
Benchmark 1: target/release/ntcrack test.hashes  med
  Time (mean ± σ):     11.973 s ±  0.052 s    [User: 108.656 s, System: 3.784 s]
  Range (min … max):   11.915 s … 12.015 s    3 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet test.hashes med
  Time (mean ± σ):     29.785 s ±  0.590 s    [User: 21.074 s, System: 1.741 s]
  Range (min … max):   29.432 s … 30.466 s    3 runs
 
[+] 143 hash list against 110G wordlist
Benchmark 1: target/release/ntcrack test.hashes  big
  Time (mean ± σ):     139.968 s ±  0.081 s    [User: 961.547 s, System: 117.786 s]
  Range (min … max):   139.881 s … 140.041 s    3 runs
 
Benchmark 1: hashcat -m1000 --potfile-disable -O --self-test-disable --backend-ignore-opencl --hwmon-disable --quiet test.hashes big
  Time (mean ± σ):     255.648 s ±  0.855 s    [User: 183.174 s, System: 16.709 s]
  Range (min … max):   254.690 s … 256.334 s    3 runs
``` 
