#!/usr/bin/env python3

import subprocess

def set_program():
    with open('src/template.c', 'r') as f:
        data = f.read()
    data = data.format("{ r2 = add(r1, r0); r3 = add(r1, r0); r4 = add(r1, r0); r5 = add(r1, r0); }")
    with open('src/main.c', 'w') as f:
        f.write(data)

def main():
    set_program()
    proc = subprocess.Popen(['make'], stdout=subprocess.PIPE)
    #works in python 3.0+
    for line in proc.stdout:
    # for line in iter(proc.stdout.readline,''):
        print(line.rstrip())

if __name__ == "__main__":
    main()