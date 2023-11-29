# 2023 advent of code solutions

## Setup
```bash
for i in {01..25}; do mkdir "day${i}a" && mkdir "day${i}b"; done
for dir in day*; do cd $dir && cargo init && touch input.txt && cd ..; done
```