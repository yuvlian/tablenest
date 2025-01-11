# Tablenest
Tool to bulk convert .dnt (Dragon Nest table) files to .tsv (Tab-separated values).

## Benchmark
The sample used is a 18MB skillleveltable_rune.dnt file.

- **Single Core Apple M3 Pro, 18GB ram, NVME SSD**
  - Result: 36.48ms

- **A9-9425, 6GB ram, SATA SSD**
  - Result: 204.44ms

## Room of Improvements
You can try using rayon, AVX-512, etc. to make this shit faster.

Oh and, right now, the output file doesn't automatically rename extension to .tsv. Can't be arsed.

## Credits
- [Zenocara YVL 00pium](https://github.com/pufferffish) for ze zased vamp code
