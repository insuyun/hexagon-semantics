#!/usr/bin/env python3

import z3
import re


class RegisterSet:
    
    def __init__(self):
        self.general = [ z3.BitVec("R{}".format(i), 32) for i in range(32) ]
        self.control = [ z3.BitVec("C{}".format(i), 32) for i in range(32) ]

    def get_register(self, key):
        return self.__getattr__(key)
    
    def set_register(self, key, value):
        # General register alias
        if key == "SP":
            self.general[29] = value
        elif key == "FP":
            self.general[30] = value
        elif key == "LR":
            self.general[31] = value
        
        # TODO: Control register alias
        # SA0, LC0, SA1, LC1, M0, M1, ...

        # R0-31
        result = re.match(r'^R(\d|1\d|2\d|3(0|1))$', key)
        if result:
            self.general[int(result.group(1))] = value
        
        # C0-31
        result = re.match(r'^C(\d|1\d|2\d|3(0|1))$', key)
        if result:
            self.control[int(result.group(1))] = value

        # raise AttributeError()
    
    def __getattr__(self, key):
        # General register alias
        if key == "SP":
            return self.general[29]
        elif key == "FP":
            return self.general[30]
        elif key == "LR":
            return self.general[31]
        
        # TODO: Control register alias
        # SA0, LC0, SA1, LC1, M0, M1, ...

        # R0-31
        result = re.match(r'^R(\d|1\d|2\d|3(0|1))$', key)
        if result:
            return self.general[int(result.group(1))]
        
        # C0-31
        result = re.match(r'^C(\d|1\d|2\d|3(0|1))$', key)
        if result:
            return self.control[int(result.group(1))]
        
        # P0-3
        result = re.match(r'^P([0-3])$', key)
        if result:
            idx = int(result.group(1))
            return z3.Extract(8 * idx + 7, 8 * idx, self.control[4])

        raise AttributeError()


class AddInstruction:
    def __init__(self, rs, rt, rd):
        self.rs = rs
        self.rt = rt
        self.rd = rd

    def apply(self, register_set):
        rs = register_set.get_register(self.rs)
        rt = register_set.get_register(self.rt)
        register_set.set_register(self.rd, rs + rt)

rs = RegisterSet()
inst = AddInstruction("R0", "R1", "R2")
inst.apply(rs)

print(rs.R2)