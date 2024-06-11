#!/usr/bin/env python3
import sys
import csv

text = sys.stdin.read()

paragraphs = ["BEGIN"] + text.split("\n\n") + ["END"]

csv_writer = csv.writer(sys.stdout)
csv_writer.writerow(["front", "back"])

for par, next_par in zip(paragraphs, paragraphs[1:]):
    csv_writer.writerow([par, next_par])
