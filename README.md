# Tablenest
Tool to bulk convert .dnt (Dragon Nest table) files to .tsv (Tab-separated values).

## Benchmark
The sample used is a 18MB skillleveltable_rune.dnt file.

The result shown is taken from the fastest result.

- **Single Core Apple M3 Pro, 18GB ram, NVME SSD**
  - Result: 72.8ms

- **A9-9425, 6GB ram, SATA SSD**
  - Result: 277.2ms

- **i3 10105f, 16GB ram, NVME SSD**
  - Result: 121.8ms

- **Ryzen 7 9800X3D, 64GB ram, NVME SSD**
  - Result: 78.9ms

## Room of Improvements
You can try using rayon, AVX-512, etc. to make this shit faster.

Oh and, right now, the output file doesn't automatically rename extension to .tsv. Can't be arsed.

## Credits
- [Zenocara YVL 00pium](https://github.com/pufferffish) for ze zased vamp code
