# Midi grep

Find consecutive notes in midi files

### Example: 

```
# Find all midi files that has c1, c2 and c5 after eachother
midigrep /some/path c1 c2 c5
```

## Usage:

```
midigrep

Usage:
    midigrep <path> <notes>...
    midigrep (-h | --help)
    midigrep (-v | --version)

Example:
    midigrep ~/midi c1 cs2 b4

Options:
    -h --help           Show this screen
    -n --notes          List of consequtive notes to search for
    -v --version        Print version
```

## Performance

Processing 192,981 midi files:

```
time ./bin/midigrep ~/Downloads/ c3 c4|wc -l
18177

real	0m3.540s
user	0m21.289s
sys	0m5.529s
```
