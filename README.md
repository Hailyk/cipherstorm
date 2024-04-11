# CipherStorm
High performance password cracker

currently only support unsalted md5 hash

## requirements
- rust, tested on 1.77.1

## compile
`cargo build --release`

compile to custom directory:

`cargo build --release --target-dir ./bin`

## usage / options
the compiled binary takes a hash list file and options.
example: `./cipherstorm password.hash -m`

flags:
- `-m` : multi-threaded mode
- `-s` : (default) single-threaded mode

## benchmark
- 5.4M/s on Ryzen 7 3700x (16 threads) @ 4.1GHz
- 340k/s on Ryzen 7 3700x (single thread) @ 4.1GHz

## notes on implementation
- The program's method of dispatching work to threads is by assigning a prefix that the thread is responsible for, the current suffix length is hardcoded to 4 characters, which allows for a reasonable TTl for each thread instead of taking the overhead of creating new thread every second.
- Previous implementation of next password generation by incrementing the current password was slow, curent implementation of prefixing and array tracked indexing yielded about a 50% improvement in performance.
- Another reason for this prefix dispatch method was due to rust memory safe bounds of only allowing one thread to touch a mutable reference to a variable at a time, and concurrency issues with multiple threading incrementing the same password. This method segments the array into chunks and only the manager is responsible for tracking the upper bounds of the array.
## license
MIT
```
MIT License

Copyright (c) 2024 Haily Kuang(@HailyK)

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
