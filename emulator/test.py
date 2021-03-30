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

def generate_random_inst(slot):
    global used_regs

    if slot == 0:
        picked_insns = insns['alu32']
    elif slot == 1:
        picked_insns = insns['alu32']
    elif slot == 2:
        picked_insns = insns['alu32'] + insns['xtype'] + insns['cr23']
    elif slot == 3:
        picked_insns = insns['alu32'] + insns['xtype'] + insns['cr23']
    
    # TODO: Support .new instruction
    while True:
        insn = random.choice(picked_insns)[:]
        if '.new' in insn[-1]:
            continue
        break

    # TODO: Support loop end instruction?
    if slot:
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
                val = random.randint(0, min(2 ** count - 2, 27))
                used = set([val, val + 1])
            else:
                val = random.randint(0, min(2 ** count - 1, 28))
                used = set([val])
            
            if used_regs & used:
                continue

            used_regs |= used
            break

        fill_insn(insn, c, val)
    
    print(insn)
    s = ''.join(insn[:32])
    return int(s, 2).to_bytes((len(s) + 7) // 8, byteorder='little')

def generate_random_packet():
    global used_regs
    used_regs = set()

    return b''.join([
        generate_random_inst(3),
        generate_random_inst(2),
        generate_random_inst(1),
        generate_random_inst(0)
    ])

def read_instructions_csv():
    global insns

    with open('qemu_hexagon/instructions.csv') as f:
        reader = csv.reader(f)
        rows = list(reader)

    # ALU32 instructions
    insns['alu32'] = rows[:118]

    # CR instructions
    # Slot 2, 3 
    insns['cr23'] = rows[118:122] + rows[133:147]
    # Add user control register transfer instructions
    # insns['cr23'] += rows[147:151]
    # Slot 3 only
    # insns['cr3'] = rows[122:133]

    # JR instructions
    # insns['jr'] = rows[151:164]

    # J instructions
    # insns['j'] = rows[164:258]

    # LD instructions
    # insns['ld'] = rows[258:470]

    # MEMOP instructions
    # insns['memop'] = rows[470:494]

    # NV instructions
    # insns['nv'] = rows[494:616]

    # ST instructions
    # insns['st'] = rows[616:762]

    # SYSTEM instructions
    # insns['system'] = rows[762:843]

    # XTYPE instructions
    insns['xtype'] = rows[843:]

def main():
    read_instructions_csv()

    while True:
        packet = generate_random_packet()

        with open('src/main.elf', 'rb') as f:
            data = bytearray(f.read())
        
        # better solution?
        data[0x70e4:0x70e4 + 16] = packet

        with open('bin/main.elf', 'wb') as f:
            f.write(data)

        with open('/dev/urandom', 'rb') as f:
            reg_data = f.read(4 * 29)

        print(reg_data.hex())
        
        with open('./reg_input', 'wb') as f:
            f.write(reg_data)

        proc = subprocess.Popen(['make', 'sim', 'reg_input=reg_input'], stdout=subprocess.PIPE)
        l = proc.stdout.read()
        print(l)
        if b'CRASH' in l:
            break
        proc.communicate()[0]
        if proc.returncode != 0:
            print(proc.returncode)
            exit(0)


if __name__ == "__main__":
    main()