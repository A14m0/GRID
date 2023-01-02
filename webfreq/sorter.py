#!/usr/bin/env python3

# this program processes the output of webfreq and sorts them

class TagEntry:
    def __init__(self, tag, val):
        self.tag = tag
        self.val = val

    def __str__(self):
        return f"{self.tag}: {self.val}"

    def __repr__(self) -> str:
        return f"{self.tag}: {self.val}"


def sort_method(val):
    return val.val

with open("tags_freq.txt", "r") as f:
    data = f.readlines()

# ignore CSV header
data = data[1:]
vals = []
for line in data:
    line = line.split(",")
    if len(line[0]) != 0:
        vals.append(TagEntry(line[0], int(line[1])))

# now we sort it
vals.sort(key=sort_method, reverse=True)

print(vals)