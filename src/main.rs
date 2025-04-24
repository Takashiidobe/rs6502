use std::env;
use std::fs;
use std::process;

mod assembler;
mod cpu;
mod memory;

use cpu::CPU;
use memory::Memory;

const PROGRAM_START_ADDRESS: u16 = 0x0600; // Common starting address for programs

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <assembly_file>", args[0]);
        process::exit(1);
    }

    let assembly_file = &args[1];

    let assembly_code = fs::read_to_string(assembly_file).expect("Failed to read assembly file");

    let machine_code = assembler::assemble(&assembly_code);
    println!("Machine code: {:02X?}", machine_code);

    let mut memory = Memory::new();
    memory.load_program(machine_code, PROGRAM_START_ADDRESS);

    memory.write_u16(0xFFFC, PROGRAM_START_ADDRESS);

    let mut cpu = CPU::new(memory);
    cpu.reset();

    println!("Starting execution...");
    loop {
        println!(
            "PC: {:04X}, A: {:02X}, X: {:02X}, Y: {:02X}, SP: {:02X}, Status: {:02X}",
            cpu.pc, cpu.a, cpu.x, cpu.y, cpu.sp, cpu.status
        );

        cpu.execute_instruction();

        if cpu.halted {
            println!("Execution halted. Final accumulator value: {}", cpu.a);
            process::exit(cpu.a as i32); // Use accumulator value as exit code
        }
    }
}
