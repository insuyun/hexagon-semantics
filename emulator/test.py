#!/usr/bin/env python3

import subprocess
import random
from qemu_hexagon.hex_common import *

def set_program(packet):
    with open('src/template.c', 'r') as f:
        data = f.read()
    data = data.format(packet)
    with open('src/main.c', 'w') as f:
        f.write(data)

tagregs = {}
tagimms = {}
sem_tags = {
    # ADDED BY HAND :( :( :(
    'alu32': [
        'A2_addi', 'A2_add', 'A2_addsat',
        'A2_and', 'A4_andn', 'A2_andir',
        'A2_or', 'A4_orn', 'A2_orir', 'A2_xor',
        'A2_nop',
        'A2_sub', 'A2_subsat', 'A2_subri',
        'A2_sxtb', 'A2_sxth',
        'A2_tfrsi', 'A2_tfril', 'A2_tfrih', 'A2_tfr',
        'A2_svaddh', 'A2_svaddhs', 'A2_svadduhs',
        'A2_svavgh', 'A2_svavghs', 'A2_svnavgh',
        'A2_svsubh', 'A2_svsubhs', 'A2_svsubuhs',
        'A2_zxth',
        'A2_combinew', 'A4_combineri', 'A4_combineir', 'A2_combineii', 'A4_combineii', 
        'A2_combine_hh', 'A2_combine_hl', 'A2_combine_lh', 'A2_combine_ll',
        'C2_mux', 'C2_muxii', 'C2_muxir', 'C2_muxri', # in semantics_compare.pyinc. WHY???
        'A2_aslh', 'A2_asrh',
        'S2_packhl', # in semantics_shift.pyinc. WHY???????
        'A2_paddt', 'A2_paddf', 'A2_paddtnew', 'A2_paddfnew', 
        'A2_paddit', 'A2_paddif', 'A2_padditnew', 'A2_paddifnew',
        'A4_paslht', 'A4_paslhf', 'A4_paslhtnew', 'A4_paslhfnew',
        'A4_pasrht', 'A4_pasrhf', 'A4_pasrhtnew', 'A4_pasrhfnew',
        'C2_ccombinewnewt', 'C2_ccombinewnewf', 'C2_ccombinewt', 'C2_ccombinewf', # :(
        'A2_pandt', 'A2_pandf', 'A2_pandtnew', 'A2_pandfnew',
        'A2_port', 'A2_porf', 'A2_portnew', 'A2_porfnew',
        'A2_pxort', 'A2_pxorf', 'A2_pxortnew', 'A2_pxorfnew',
        'A2_psubt', 'A2_psubf', 'A2_psubtnew', 'A2_psubfnew',
        'A4_psxtbt', 'A4_psxtbf', 'A4_psxtbtnew', 'A4_psxtbfnew',
        'A4_psxtht', 'A4_psxthf', 'A4_psxthtnew', 'A4_psxthfnew',
        'C2_cmovenewit', 'C2_cmovenewif', 'C2_cmoveit', 'C2_cmoveif', # :(
        'A4_pzxtbt', 'A4_pzxtbf', 'A4_pzxtbtnew', 'A4_pzxtbfnew',
        'A4_pzxtht', 'A4_pzxthf', 'A4_pzxthtnew', 'A4_pzxthfnew',
        'C2_cmpeqi', 'C2_cmpeq', 'C4_cmpneqi', 'C4_cmpneq',
        'C2_cmpgt', 'C2_cmpeqi', 'C2_cmpgti', 'C4_cmplte',
        'C2_cmpgtu', 'C2_cmpgtui', 'C4_cmplteui', 'C4_cmplteu',
        'A4_rcmpeqi', 'A4_rcmpneqi', 'A4_rcmpeq', 'A4_rcmpneq'
    ]
}

used_regs = set()

sem_names = [
    "allext",
    "alu",
    "branch",
    "compare",
    "float",
    "ldst",
    "mpy",
    "shift",
    "subinsns",
    "system"
]

invalid_attribs = set([
    "A_JUMP", "A_INDIRECT", "A_CALL", "A_COF", "A_CONDEXEC", "A_DOTNEWVALUE", "A_NEWCMPJUMP",
    "A_CVI",
    "A_MEMLIKE",
    "A_GUEST",
    "A_PRIV"
])

def generate_random_reg(inp, is_operand):
    global used_regs
    regtype, regid, hl, length = inp

    if regtype not in ['P', 'R', 'C']:
        print("UH OH")
        exit(0)

    while True:   
        if regtype == 'P':
            assert len(regid) == 1
            assert hl == ''
            ret = "P{}".format(random.randint(0, 3))
            used = [ret]
        elif regtype in ['R', 'C'] and len(regid) == 2: # register pair
            # idx = random.randint(0, 15)
            idx = random.randint(0, 13)
            ret = "{}{}:{}{}".format(regtype, idx * 2 + 1, idx * 2, hl)
            used = ["{}{}".format(regtype, idx * 2), "{}{}".format(regtype, idx * 2 + 1)]
        elif regtype in ['R', 'C']:
            # idx = random.randint(0, 31)
            idx = random.randint(0, 28)
            ret = "{}{}{}".format(regtype, idx, hl)
            used = ["{}{}".format(regtype, idx)]
        else:
            print("UNREACHABLE")
            exit(0)
        
        if is_operand:
            if set(used) & used_regs:
                continue
            used_regs |= set(used)
        return ret

        
def generate_random_imm(inp):
    immtype, immlen, immoff = inp
    immtype = immtype.lower()
    immlen = int(immlen)
    immoff = ":" + immoff if immoff else ""
    if immtype == 'u':
        return str(random.randint(0, 2 ** immlen - 1)) + immoff
    elif immtype in ['s', 'm', 'r']:
        return str(random.randint(-(2 ** (immlen - 1)), 2 ** (immlen - 1) - 1)) + immoff
    else:
        print("WOW", inp)
        exit(0)

def generate_random_inst(sems):
    tags = sum([sem_tags[sem] for sem in sems], [])
    while True:
        tag = random.choice(tags)
        if attribdict[tag] & invalid_attribs:
            continue
        break

    beh = behdict[tag]
    operand = beh.split('=')[0] if len(beh.split('=')) > 1 else None

    print(tag)
    print(attribdict[tag])
    print(beh)

    for tagreg in tagregs[tag]:
        print(tagreg)
        res = generate_random_reg(tagreg, ''.join(tagreg) in operand)
        beh = beh.replace(''.join(tagreg), res)
    
    for tagimm in tagimms[tag]:
        print(tagimm)
        res = generate_random_imm(tagimm)
        beh = beh.replace("#{}{}{}".format(tagimm[0], tagimm[1], ":" + tagimm[2] if tagimm[2] else ""), '#' + res)

    return beh

def generate_random_packet():
    return "{{ {}; \\\n {}; \\\n {}; \\\n {}; \\\n }}".format(
        generate_random_inst(['alu32']),
        generate_random_inst(['alu32']),
        generate_random_inst(['alu', 'compare']),
        generate_random_inst(['alu', 'compare'])
    )

def main():
    global tagregs, tagimms, sem_tags, used_regs

    read_attribs_file("qemu_hexagon/attribs_def.h")

    for sem in sem_names:
        prev_tags = tags[:]
        read_semantics_file("qemu_hexagon/semantics_{}.pyinc".format(sem))
        sem_tags[sem] = list(set(tags) - set(prev_tags))

    calculate_attribs()
    tagregs = get_tagregs()
    tagimms = get_tagimms()

    # Remove instructions not compilable
    tags.remove('A6_vcmpbeq_notany')

    while True:
        used_regs = set()
        packet = generate_random_packet()

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