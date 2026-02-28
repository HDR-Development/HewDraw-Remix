nxo64
========

nxo64 is a python (2 & 3) library for reading NSO / NRO files and an IDAPython loader.

Installation
============

Install the modules from `requirements.txt` so IDAPython can import them.

Copy `nxo64-ida.py` and `nxo64` into IDA's `loaders` directory.

Credits
=======

I want to thank the following people for their help and/or other involvement with this or the original project:

- [@ReSwitched](https://github.com/reswitched) for creating [loaders](https://github.com/reswitched/loaders).
- [@SciresM](https://github.com/SciresM) for sharing the fixes for `kip1_blz_decompress()`.
- [@hthh](https://github.com/hthh) for creating [switch-reversing](https://github.com/hthh/switch-reversing), which contains a modified version of `nxo64.py`. Some modifications are integrated into this repo. 
