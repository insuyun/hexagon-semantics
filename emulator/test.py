#!/usr/bin/env python3

import csv
import random
import re
import socket
import subprocess

def set_program(packet):
    with open('src/template.c', 'r') as f:
        data = f.read()
    data = data.format(packet)
    with open('src/main.c', 'w') as f:
        f.write(data)

insns = dict()

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
        if '.new' in insn[-1] or 'deprecated' in insn[-1]:
            continue
        break

    # TODO: Support loop end instruction?
    if slot:
        fill_insn(insn, 'P', 1)
    else:
        fill_insn(insn, 'P', 3)
    fill_insn(insn, '-', 0)

    for c in ['j', 'I', 'N', 's', 't', 'u', 'v']:
        count = insn.count(c)
        if count == 0:
            continue
        if 'R' + c + c in insn[-1] and insn.count(c) == 5:
            val = random.randint(0, min(2 ** count - 2, 8))
        else:
            val = random.randint(0, min(2 ** count - 1, 9))
        fill_insn(insn, c, val)

    written = set()

    if 'decbin' in insn[-1]:
        written.add('P0')

    for c in ['d', 'x', 'y', 'e']:
        count = insn.count(c)
        if count == 0:
            continue

        if 'R' + c + c in insn[-1]:
            val = random.randint(0, min(2 ** count - 1, 8))
            written |= set(['R{}'.format(val), 'R{}'.format(val + 1)])
        else:
            rtype = insn[-1][insn[-1].find(c) - 1]
            val = random.randint(0, min(2 ** count - 1, 9))
            written |= set(['{}{}'.format(rtype, val)])
        fill_insn(insn, c, val)
    
    s = ''.join(insn[:32])
    return int(s, 2).to_bytes((len(s) + 7) // 8, byteorder='little'), insn, written

def generate_random_packet():
    # Avoid double-writing
    while True:

        res = [generate_random_inst(i) for i in range(4)]

        if (len(res[0][2] | res[1][2] | res[2][2] | res[3][2]) !=
            len(res[0][2]) + len(res[1][2]) + len(res[2][2]) + len(res[3][2])):
            continue

        return b''.join(res[i][0] for i in reversed(range(4))), [res[i][1] for i in range(4)]

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


def read_output(output):
    prog = re.compile(rb'([RC][0-9]{1,2}): ([0-9]{1,10})')

    inputs, outputs = {}, {}
    for line in output.split(b'\n'):
        res = prog.match(line)
        if not res: continue
        name, value = res.group(1).decode(), int(res.group(2))

        if name not in inputs:
            inputs[name] = value
        else:
            outputs[name] = value
    
    return inputs, outputs


def run_sim(packet, reg_data):
    with open('bin/template.elf', 'rb') as f:
        data = bytearray(f.read())
    
    # Find 4 nops to replace
    nops = data.find(
        b'\x04\xc0\x2b\x62' +
        b'\x00\x40\x00\x7f' * 3 +
        b'\x00\xc0\x00\x7f' +
        b'\x0b\xc0\x04\x6a'
    ) + 4
    data[nops:nops + 16] = packet

    with open('bin/main.elf', 'wb') as f:
        f.write(data)

    proc = subprocess.run(['make', 'sim', '-s'],
        input=reg_data,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )

    if b'CRASH' in proc.stdout:
        return None, None

    # print(proc.stdout)
    inputs, outputs = read_output(proc.stdout)
    return inputs, outputs


def run_qemu(packet, reg_data):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect(('localhost', 9000))
        s.send(packet + reg_data)
        data = s.recv(1024)
    
    if b'CRASH' in data:
        return None, None
    
    # print(data)
    inputs, outputs = read_output(data)
    return inputs, outputs


def main():
    read_instructions_csv()

    while True:
        packet, insns = generate_random_packet()
        with open('/dev/urandom', 'rb') as f:
            reg_data = f.read(4 * 11)

        sim_inputs, sim_outputs = run_sim(packet, reg_data)
        qemu_inputs, qemu_outputs = run_qemu(packet, reg_data)

        if sim_inputs != qemu_inputs or sim_outputs != qemu_outputs:
            print("insns: ")
            for i in range(4):
                print(insns[i])
            print("inputs: ")
            print(sim_inputs)
            print(qemu_inputs)
            print("outputs: ")
            print(sim_outputs)
            print(qemu_outputs)

if __name__ == "__main__":
    main()