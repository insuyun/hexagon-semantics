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
    def __init__(self, insn_bits, orig_insn_bits, insn_str, written, require):
        self.insn_bits = insn_bits
        self.orig_insn_bits = orig_insn_bits
        self.insn_str = insn_str
        self.written = written
        self.require = require

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

        while True:
            picked = random.choice(picked_insns)[:]

            insn_bits, insn_str = picked[:-1], picked[-1]
            orig_insn_bits = insn_bits[:]
            if "deprecated" in insn_str:
                continue
            break

        # TODO: Support loop end instruction?
        if slot:
            cls.fill_insn_bits(insn_bits, "P", 1)
        else:
            cls.fill_insn_bits(insn_bits, "P", 3)
        cls.fill_insn_bits(insn_bits, "-", 0)

        require = set()

        for c in ["j", "I", "N", "s", "t", "u", "v"]:
            count = insn_bits.count(c)
            if count == 0:
                continue

            is_new_value = False

            if c == "j":
                opstr = re.search(r"(#[su][0-9]{1,2})", insn_str).group(1)
            elif c == "I":
                opstr = re.search(r"(#[SU][0-9]{1,2})", insn_str).group(1)
            elif c == "N":
                opstr = "[:<<N]"
            else:
                res = re.search(r"([RPC]" + c + r"{1,2})(\.new)?", insn_str)
                opstr = res.group(1)
                is_new_value = True if res.group(2) else False

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
                if is_new_value:
                    require = set(["{}{}".format(opstr[0], val)])

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

        return cls(insn_bits, orig_insn_bits, insn_str, written, require)


class InstructionPacket:
    def __init__(self, insns):
        self.insns = insns

    def get_bytes(self):
        return b"".join(insn.insn_byte for insn in reversed(self.insns))

    def is_valid(self):
        all_require = set().union(*[insn.require for insn in self.insns])
        all_written = set().union(*[insn.written for insn in self.insns])
        return len(all_written) == sum(
            len(insn.written) for insn in self.insns
        ) and len(all_require) == len(all_require & all_written)

    def get_written_reg(self):
        return set().union(*[insn.written for insn in self.insns])

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


class Tester:
    registers = ["R{}".format(i) for i in range(10)] + ["C4", "C8"]
    emulators = ["sdk-sim", "qemu"]

    def __init__(self):
        self.packet = None
        self.input = None
        self.output = None
        self.tested = False

    def generate_case(self):
        self.packet = InstructionPacket.generate()

        with open("/dev/urandom", "rb") as f:
            general = f.read(4 * 10)

            predicate = b""
            for _ in range(4):
                t = random.randint(0, 2)
                if t == 0:
                    predicate += b"\x00"
                elif t == 1:
                    predicate += b"\xff"
                else:
                    predicate += f.read(1)

            user_status = bytearray(f.read(4))
            user_status[0] &= 0b00111110
            user_status[1] &= 0b11111100
            user_status[2] &= 0b11000001
            # What the hell is wrong with IEEE trap enable fields?
            user_status[3] &= 0b00000000

            # [16:15] HFI cannot be 11
            if user_status[1] & 0b10000000 and user_status[2] & 0b00000001:
                user_status[1] &= 0b01111111
                user_status[2] &= 0b11111110

        self.input = general + predicate + user_status
        self.tested = False

    @staticmethod
    def parse_output(output):
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

        return {"input": inputs, "output": outputs}

    def run_sim(self):
        with open("bin/template.elf", "rb") as f:
            data = bytearray(f.read())

        # Find 4 nops to replace
        nops = (
            data.find(
                b"\x08\xc0\x2c\x62"
                + b"\x00\x40\x00\x7f" * 3
                + b"\x00\xc0\x00\x7f"
                + b"\x0b\xc0\x04\x6a"
            )
            + 4
        )
        data[nops : nops + 16] = self.packet.get_bytes()

        with open("bin/main.elf", "wb") as f:
            f.write(data)

        proc = subprocess.run(
            ["make", "sim", "-s"],
            input=self.input,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        if b"CRASH" in proc.stdout:
            return None
        return self.parse_output(proc.stdout)

    def run_qemu(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect(("localhost", 9000))
            s.send(self.packet.get_bytes() + self.input)
            data = s.recv(1024)

        if b"CRASH" in data:
            return None
        return self.parse_output(data)

    def run(self):
        assert not self.tested

        self.output = {
            "qemu": self.run_qemu(),
            "sdk-sim": self.run_sim(),
        }
        self.tested = True

    def is_failed(self):
        assert self.tested

        if self.output["qemu"] is None or self.output["sdk-sim"] is None:
            return True
        if self.output["sdk-sim"]["input"] != self.output["qemu"]["input"]:
            return True
        if self.output["sdk-sim"]["output"] != self.output["qemu"]["output"]:
            return True

        written = self.packet.get_written_reg() | set(["C4", "C8"])
        not_written = set(self.registers) - written
        for _, emulator_case in self.output.items():
            for reg in not_written:
                if emulator_case["input"][reg] != emulator_case["output"][reg]:
                    return True

        return False

    def print_output(self):
        print("[Instruction]")
        self.packet.print_packet()

        print("[Inputs]")
        print(" " * 10 + "".join(f" {r:8s}" for r in self.registers))
        for e in self.emulators:
            s = f"{e:10s}"
            if self.output[e]:
                s += "".join(
                    f" {self.output[e]['input'][r]:08x}" for r in self.registers
                )
            else:
                s += "     None" * len(self.registers)
            print(s)

        print("[Outputs]")
        print(" " * 10 + "".join(f" {r:8s}" for r in self.registers))
        for e in self.emulators:
            s = f"{e:10s}"
            if self.output[e]:
                s += "".join(
                    f" {self.output[e]['output'][r]:08x}" for r in self.registers
                )
            else:
                s += "     None" * len(self.registers)
            print(s)


def main():
    tester = Tester()
    while True:
        tester.generate_case()
        tester.run()
        if tester.is_failed():
            tester.print_output()


if __name__ == "__main__":
    main()
