#!/usr/bin/env python3

import copy
import csv
import itertools
import pickle
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
    insns["nop"] = rows[10]

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
    opstr_preg = {
        "j": re.compile(r"(#[su][0-9]{1,2})"),
        "I": re.compile(r"(#[SU][0-9]{1,2})"),
        "N": re.compile(r"(\[:<<N\])"),
        **{c: re.compile(r"([RPC](" + c + r"{1,2}))(\.new)?") for c in "stuvdexy"},
    }

    def __init__(self, insn_bits, insn_str):
        self.bits = insn_bits
        self.str = insn_str
        self.orig_bits = insn_bits[:]
        self.orig_str = insn_str
        self.written = set()
        self.require = set()
        self.values = dict()

    @property
    def insn_byte(self):
        return int("".join(self.bits), 2).to_bytes(4, byteorder="little")

    def print_insn(self, indent=0):
        print(" " * (indent + 49) + "".join(self.orig_bits))
        print(" " * indent + f"{self.str:48s} " + "".join(self.bits))

    def set_value(self, c, val):
        if c != "-":
            self.values[c] = val

        count = self.bits.count(c)
        for idx in range(32):
            if self.bits[idx] == c:
                self.bits[idx] = str((val >> (count - 1)) & 1)
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
            if "deprecated" in insn_str:
                continue
            insn = cls(insn_bits, insn_str)
            break

        # TODO: Support loop end instruction?
        if slot:
            insn.set_value("P", 1)
        else:
            insn.set_value("P", 3)
        insn.set_value("-", 0)

        for c in "jINstuvdexy":
            count = insn.bits.count(c)
            if count == 0:
                continue

            res = cls.opstr_preg[c].search(insn.str)
            opstr = res.group(1)
            if len(res.groups()) >= 2:
                is_pair = True if res.group(2) and len(res.group(2)) == 2 else False
                is_new_value = True if res.group(3) else False
            else:
                is_pair, is_new_value = False, False

            if c in "jIN":
                val = random.randint(0, 2 ** count - 1)
            elif is_pair:
                val = random.randint(0, min(2 ** count - 1, 8))
            else:
                val = random.randint(0, min(2 ** count - 1, 9))

            if c in "jI":
                insn.str = insn.str.replace(opstr, "#{}{}".format(val, opstr[1:]))
            elif c == "N" and val:
                assert val == 1
                insn.str = insn.str.replace(opstr, ":<<1")
            elif c == "N" and not val:
                insn.str = insn.str.replace(opstr, "")
            elif is_pair:
                insn.str = insn.str.replace(
                    opstr, "{}{}:{}".format(opstr[0], val + 1, val)
                )
            else:
                insn.str = insn.str.replace(opstr, "{}{}".format(opstr[0], val))

            if is_new_value:
                insn.require |= set(["{}{}".format(opstr[0], val)])
            if c in "dexy":
                if is_pair:
                    insn.written |= set(
                        ["{}{}".format(opstr[0], val), "{}{}".format(opstr[0], val + 1)]
                    )
                else:
                    insn.written |= set(["{}{}".format(opstr[0], val)])

            insn.set_value(c, val)

        if "decbin" in insn.str:
            insn.written.add("P0")

        return insn

    @classmethod
    def nop(cls, slot):
        nop = cls(insns["nop"][:32], insns["nop"][-1])
        if slot:
            nop.set_value("P", 1)
        else:
            nop.set_value("P", 3)
        nop.set_value("-", 0)
        return nop


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

    def get_required_insn_index(self, slots):
        all_require = set().union(*[self.insns[i].require for i in slots])
        ret = []
        for i, insn in enumerate(self.insns):
            if insn.written & all_require:
                ret.append(i)
        return ret

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


class TestCase:
    def __init__(self, packet, input):
        self.packet = packet
        self.input = input

    @staticmethod
    def generate_input():
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

        return general + predicate + user_status

    @classmethod
    def generate(cls, db=None):
        while True:
            packet = InstructionPacket.generate()
            input = cls.generate_input()
            tc = cls(packet, input)
            if db and db.check(tc):
                continue
            return tc

    def mutate(self, slots, db=None):
        required_index = self.packet.get_required_insn_index(slots)

        while True:
            packet = copy.deepcopy(self.packet)
            for i in range(4):
                if i in slots:
                    continue
                if i in required_index:
                    while True:
                        insn = Instruction.generate(i)
                        if packet.insns[i].written == insn.written:
                            packet.insns[i] = insn
                            break
                else:
                    packet.insns[i] = Instruction.nop(i)

            input = self.generate_input()
            tc = self.__class__(packet, input)

            if db and db.check(tc):
                continue
            return tc

    def print_testcase(self):
        self.packet.print_packet()
        print(self.input.hex())


class Tester:
    registers = ["R{}".format(i) for i in range(10)] + ["C4", "C8"]
    emulators = ["sdk-sim", "qemu"]

    def __init__(self):
        self.testcase = None
        self.output = None

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
        data[nops : nops + 16] = self.testcase.packet.get_bytes()

        with open("bin/main.elf", "wb") as f:
            f.write(data)

        proc = subprocess.run(
            ["make", "sim", "-s"],
            input=self.testcase.input,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )

        if b"CRASH" in proc.stdout:
            return None
        return self.parse_output(proc.stdout)

    def run_qemu(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect(("localhost", 9000))
            s.send(self.testcase.packet.get_bytes() + self.testcase.input)
            data = s.recv(1024)

        if b"CRASH" in data:
            return None
        return self.parse_output(data)

    def run(self, testcase):
        self.testcase = testcase
        self.output = {
            "qemu": self.run_qemu(),
            "sdk-sim": self.run_sim(),
        }

    def is_failed(self):
        if self.output["qemu"] is None or self.output["sdk-sim"] is None:
            return True
        if self.output["sdk-sim"]["input"] != self.output["qemu"]["input"]:
            return True
        if self.output["sdk-sim"]["output"] != self.output["qemu"]["output"]:
            return True

        written = self.testcase.packet.get_written_reg() | set(["C4", "C8"])
        not_written = set(self.registers) - written
        for _, emulator_case in self.output.items():
            for reg in not_written:
                if emulator_case["input"][reg] != emulator_case["output"][reg]:
                    return True

        return False

    def print_output(self):
        print("[Testcase]")
        self.testcase.print_testcase()

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


class TestCaseDatabase:
    def __init__(self):
        self.db = set()

    def insert(self, tc, slots):
        key = frozenset([tc.packet.insns[i].orig_str for i in slots])
        if key not in self.db:
            self.db.add(key)

    def check(self, tc):
        insn_orig_strs = [insn.orig_str for insn in tc.packet.insns]
        for n in range(1, 5):
            for key in itertools.combinations(insn_orig_strs, n):
                if frozenset(key) in self.db:
                    return True
        return False


def main():
    slots_combination = sum(
        [list(itertools.combinations([0, 1, 2, 3], n)) for n in range(1, 5)], []
    )

    tester = Tester()
    try:
        with open("tc_database.pickle", "rb") as f:
            tc_database = pickle.load(f)
    except:
        tc_database = TestCaseDatabase()

    while True:
        tc = TestCase.generate(db=tc_database)
        tester.run(tc)
        if not tester.is_failed():
            continue
        tester.print_output()

        identified = None
        for slots in slots_combination:
            flag = False
            for _ in range(100):
                mutated_tc = tc.mutate(slots, db=tc_database)
                tester.run(mutated_tc)
                if tester.is_failed():
                    flag = True
                    break
            if flag:
                identified = slots
                break

        if identified:
            print("Identified, ", identified)
            tc_database.insert(tc, identified)
        else:
            print("Failed to identify")

        with open("tc_database.pickle", "wb") as f:
            pickle.dump(tc_database, f)
        print("=" * 118)


if __name__ == "__main__":
    main()
