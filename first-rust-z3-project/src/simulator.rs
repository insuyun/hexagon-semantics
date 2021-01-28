use crate::instruction::InstructionPacket;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::str;

pub fn run_simulator(packet: &InstructionPacket) -> std::io::Result<()> {
    // TODO: Support path for emulator
    // 1. Write the instruction packet to the file

    let mut file = File::create("emulator/src/hello.c")?;
    let content = format!(
        "#include <stdio.h>
    
int main()
{{
    __asm(
    \"__begin:\"
        \"{}\"
    \"__end:\");
}}",
        packet
    );

    file.write_all(content.as_bytes())?;

    // 2. Run simulator
    let output = Command::new("make")
        .args(&["-C", "emulator"])
        .output()
        .expect("failed to execute process");

    let hello = output.stdout;
    let hello = str::from_utf8(&hello).unwrap();

    println!("{}", hello);

    Ok(())
}
