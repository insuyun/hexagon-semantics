from test import TestCase
import subprocess
import emilator

with open('./src/template_asm.elf', 'rb') as f:
    data = bytearray(f.read())
nops = data.find((b"\x00\x40\x00\x7f" * 3 + b"\x00\xc0\x00\x7f") * 2)

while True:
    tc = TestCase.generate()
    data[nops : nops + 16] = tc.packet.get_bytes()

    # print(tc.input)
    # exit(0)

    with open('./asm.elf', 'wb') as f:
        f.write(data)
    
    # with open('./asm.input', 'wb') as f:
    #     f.write(tc.input)

    proc = subprocess.run(
        ["python3", "test_binja_inner.py"],
        input=tc.input,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )

    if b"undefined" not in proc.stdout:
        tc.print_testcase()
        print(proc.stdout.decode())
        print(proc.stdout)
        print(proc.stderr)
        exit(0)
    
    # else:
