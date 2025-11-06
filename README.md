# homemade_yes
High-performance reimplementation of GNU yes, but made at home in Rust.

You can benchmark it by running `./homemade_yes | pv -N "Homemade yes" -r -t > /dev/null`.

On my build machine, `homemade_yes` can obtain a throughput of about ~3.5 gigabytes per second, which is comparable to GNU yes (v9.5). I'm pretty sure that my hardware is bottlenecking performance at this point, so better throughputs might be possible with some hardware tweaking.
