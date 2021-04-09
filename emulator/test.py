#!/usr/bin/env python3

import csv
import random
import re
import socket
import subprocess


insns = {}


def read_instructions_csv():
    global insns

    with open("qemu_hexagon/instructions.csv") as f:
        reader = csv.reader(f)
        rows = list(reader)

    # ALU32 instructions
    insns["alu32"] = rows[:118]

    # CR instructions
    # Slot 2, 3
    insns["cr23"] = rows[118:122] + rows[133:147]
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
    insns["xtype"] = rows[843:]


read_instructions_csv()


class Instruction:
    def __init__(self, insn_bits, orig_insn_bits, insn_str, written):
        self.insn_bits = insn_bits
        self.orig_insn_bits = orig_insn_bits
        self.insn_str = insn_str
        self.written = written

        s = "".join(insn_bits[:32])
        self.insn_byte = int(s, 2).to_bytes((len(s) + 7) // 8, byteorder="little")

    def print_insn(self, indent=0):
        print(" " * (indent + 49) + "".join(self.orig_insn_bits))
        print(" " * indent + f"{self.insn_str:48s} " + "".join(self.insn_bits))

    @staticmethod
    def fill_insn_bits(insn_bits, c, val):
        count = insn_bits.count(c)

        for idx in range(32):
            if insn_bits[idx] == c:
                insn_bits[idx] = str((val >> (count - 1)) & 1)
                count -= 1

    @classmethod
    def generate(cls, slot):
        global insns

        if slot == 0:
            picked_insns = insns["alu32"]
        elif slot == 1:
            picked_insns = insns["alu32"]
        elif slot == 2:
            picked_insns = insns["alu32"] + insns["xtype"] + insns["cr23"]
        elif slot == 3:
            picked_insns = insns["alu32"] + insns["xtype"] + insns["cr23"]

        # TODO: Support .new instruction
        while True:
            picked = random.choice(picked_insns)[:]

            insn_bits, insn_str = picked[:-1], picked[-1]
            orig_insn_bits = insn_bits[:]
            if ".new" in insn_str or "deprecated" in insn_str:
                continue
            break

        # TODO: Support loop end instruction?
        if slot:
            cls.fill_insn_bits(insn_bits, "P", 1)
        else:
            cls.fill_insn_bits(insn_bits, "P", 3)
        cls.fill_insn_bits(insn_bits, "-", 0)

        for c in ["j", "I", "N", "s", "t", "u", "v"]:
            count = insn_bits.count(c)
            if count == 0:
                continue

            if c == "j":
                opstr = re.search(r"(#[su][0-9]{1,2})", insn_str).group(1)
            elif c == "I":
                opstr = re.search(r"(#[SU][0-9]{1,2})", insn_str).group(1)
            elif c == "N":
                opstr = "[:<<N]"
            else:
                opstr = re.search(r"([RPC]" + c + r"{1,2})", insn_str).group(1)

            if "R" + c + c == opstr:
                val = random.randint(0, min(2 ** count - 1, 8))
            else:
                val = random.randint(0, min(2 ** count - 1, 9))

            if c in ["j", "I"]:
                insn_str = insn_str.replace(opstr, "#{}{}".format(val, opstr[1:]))
            elif c == "N" and val:
                assert val == 1
                insn_str = insn_str.replace(opstr, ":<<1")
            elif c == "N" and not val:
                insn_str = insn_str.replace(opstr, "")
            elif "R" + c + c == opstr:
                insn_str = insn_str.replace(
                    opstr, "{}{}:{}".format(opstr[0], val + 1, val)
                )
            else:
                insn_str = insn_str.replace(opstr, "{}{}".format(opstr[0], val))
            cls.fill_insn_bits(insn_bits, c, val)

        written = set()

        if "decbin" in insn_str:
            written.add("P0")

        for c in ["d", "x", "y", "e"]:
            count = insn_bits.count(c)
            if count == 0:
                continue
            opstr = re.search(r"([RPC]" + c + r"{1,2})", insn_str).group(1)

            if "R" + c + c == opstr:
                val = random.randint(0, min(2 ** count - 1, 8))
                written |= set(["R{}".format(val), "R{}".format(val + 1)])
            else:
                val = random.randint(0, min(2 ** count - 1, 9))
                written |= set(["{}{}".format(opstr[0], val)])

            if "R" + c + c == opstr:
                insn_str = insn_str.replace(
                    opstr, "{}{}:{}".format(opstr[0], val + 1, val)
                )
            else:
                insn_str = insn_str.replace(opstr, "{}{}".format(opstr[0], val))
            cls.fill_insn_bits(insn_bits, c, val)

        return cls(insn_bits, orig_insn_bits, insn_str, written)


class InstructionPacket:
    def __init__(self, insns):
        self.insns = insns

    def get_bytes(self):
        return b"".join(insn.insn_byte for insn in reversed(self.insns))

    def is_valid(self):
        all_written = set().union(*[insn.written for insn in self.insns])
        return len(all_written) == sum(len(insn.written) for insn in self.insns)

    def print_packet(self):
        print("{")
        for insn in self.insns:
            insn.print_insn(indent=4)
        print("}")

    @classmethod
    def generate(cls):
        # Avoid double-writing
        while True:
            insns = [Instruction.generate(slot) for slot in range(4)]
            packet = cls(insns)
            if not packet.is_valid():
                continue
            return packet


def read_output(output):
    prog = re.compile(rb"([RC][0-9]{1,2}): ([0-9]{1,10})")

    inputs, outputs = {}, {}
    for line in output.split(b"\n"):
        res = prog.match(line)
        if not res:
            continue
        name, value = res.group(1).decode(), int(res.group(2))

        if name not in inputs:
            inputs[name] = value
        else:
            outputs[name] = value

    return inputs, outputs


def run_sim(packet, reg_data):
    with open("bin/template.elf", "rb") as f:
        data = bytearray(f.read())

    # Find 4 nops to replace
    nops = (
        data.find(
            b"\x04\xc0\x2b\x62"
            + b"\x00\x40\x00\x7f" * 3
            + b"\x00\xc0\x00\x7f"
            + b"\x0b\xc0\x04\x6a"
        )
        + 4
    )
    data[nops : nops + 16] = packet.get_bytes()

    with open("bin/main.elf", "wb") as f:
        f.write(data)

    proc = subprocess.run(
        ["make", "sim", "-s"],
        input=reg_data,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    if b"CRASH" in proc.stdout:
        return None, None

    # print(proc.stdout)
    inputs, outputs = read_output(proc.stdout)
    return inputs, outputs


def run_qemu(packet, reg_data):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect(("localhost", 9000))
        s.send(packet.get_bytes() + reg_data)
        data = s.recv(1024)

    if b"CRASH" in data:
        return None, None

    # print(data)
    inputs, outputs = read_output(data)
    return inputs, outputs


def main():
    while True:
        packet = InstructionPacket.generate()
        with open("/dev/urandom", "rb") as f:
            reg_data = f.read(4 * 11)

        sim_inputs, sim_outputs = run_sim(packet, reg_data)
        qemu_inputs, qemu_outputs = run_qemu(packet, reg_data)

        if sim_inputs != qemu_inputs or sim_outputs != qemu_outputs:
            print("insns: ")
            packet.print_packet()
            print("inputs: ")
            print(sim_inputs)
            print(qemu_inputs)
            print("outputs: ")
            print(sim_outputs)
            print(qemu_outputs)


if __name__ == "__main__":
    main()
