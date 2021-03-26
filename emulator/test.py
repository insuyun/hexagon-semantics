#!/usr/bin/env python3

import subprocess
import random
import csv

def set_program(packet):
    with open('src/template.c', 'r') as f:
        data = f.read()
    data = data.format(packet)
    with open('src/main.c', 'w') as f:
        f.write(data)

insns = dict()
used_regs = set()

def fill_insn(insn, c, val):
    count = insn.count(c)
    
    for idx in range(32):
        if insn[idx] == c:
            insn[idx] = str((val >> (count - 1)) & 1)
            count -= 1

def generate_random_inst(idx):
    global used_regs

    if idx == 0:
        picked_insns = insns['alu32']
    elif idx == 1:
        picked_insns = insns['alu32']
    elif idx == 2:
        picked_insns = insns['alu32'] + insns['xtype']
    elif idx == 3:
        picked_insns = insns['alu32'] + insns['xtype']
    
    insn = random.choice(picked_insns)[:]

    # TODO: Support loop end instruction?
    if idx <= 2:
        fill_insn(insn, 'P', 1)
    else:
        fill_insn(insn, 'P', 3)

    for c in ['-', 'j', 'I', 'N', 's', 't', 'u', 'v']:
        count = insn.count(c)
        if count == 0:
            continue
        if 'R' + c + c in insn[-1] and insn.count(c) == 5:
            val = random.randint(0, 2 ** count - 2)
        else:
            val = random.randint(0, 2 ** count - 1)
        fill_insn(insn, c, val)
    
    for c in ['d', 'x', 'y', 'e']:
        count = insn.count(c)
        if count == 0:
            continue

        # Avoid double-writing
        while True:
            if 'R' + c + c in insn[-1] and insn.count(c) == 5:
                val = random.randint(0, 2 ** count - 2)
                used = set([val, val + 1])
            else:
                val = random.randint(0, 2 ** count - 1)
                used = set([val])
            
            if used_regs & used:
                continue

            used_regs |= used
            break

        fill_insn(insn, c, val)
    
    print(insn)
    s = ''.join(insn[:32])
    return int(s, 2).to_bytes((len(s) + 7) // 8, byteorder='big')

def generate_random_packet():
    global used_regs
    used_regs = set()

    return b''.join([
        generate_random_inst(0),
        generate_random_inst(1),
        generate_random_inst(2),
        generate_random_inst(3)
    ])

def read_instructions_csv():
    global insns

    with open('qemu_hexagon/instructions.csv') as f:
        reader = csv.reader(f)
        rows = list(reader)

    insns['alu32'] = rows[:118]
    insns['cr'] = rows[118:151]
    insns['jr'] = rows[151:164]
    insns['j'] = rows[164:258]
    insns['ld'] = rows[258:470]
    insns['memop'] = rows[470:494]
    insns['nv'] = rows[494:616]
    insns['st'] = rows[616:762]
    insns['system'] = rows[762:843]
    insns['xtype'] = rows[843:]

def main():
    read_instructions_csv()

    packet = generate_random_packet()

    with open('src/main.elf', 'rb') as f:
        data = bytearray(f.read())
    
    # better solution?
    data[0x70e4:0x70e4 + 16] = packet

    with open('bin/main.elf', 'wb') as f:
        f.write(data)

    print(packet)
    exit(0)

    set_program(packet)
    proc = subprocess.Popen(['make'], stdout=subprocess.PIPE)
    for line in proc.stdout:
        print(line.rstrip())
    proc.communicate()[0]
    if proc.returncode != 0:
        print(proc.returncode)
        exit(0)

if __name__ == "__main__":
    main()