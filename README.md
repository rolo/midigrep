# Midi grep

Find consecutive notes in midi files

## Usage:

```
# Find all midi files that has c1, c2 and c5 after eachother
midigrep /some/path c1 c2 c5
```

## Performance (because I care)

Processing 192,981 midi files:

```
time ./bin/midigrep ~/Downloads/ c3 c4|wc -l
18177

real	0m3.540s
user	0m21.289s
sys	0m5.529s
```

