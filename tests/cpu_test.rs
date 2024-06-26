extern crate nesilk_lib;

use nesilk_lib::cpu::Flags;
use serde_json;
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use nesilk_lib::bus::{BusLike, MockBus};
use nesilk_lib::cpu::NES6502;

#[test]
fn adc() {
  run_opcode_tests("69");
  run_opcode_tests("65");
  run_opcode_tests("75");
  run_opcode_tests("6d");
  run_opcode_tests("7d");
  run_opcode_tests("79");
  run_opcode_tests("61");
  run_opcode_tests("71");
}

#[test]
fn and() {
  run_opcode_tests("29");
  run_opcode_tests("25");
  run_opcode_tests("35");
  run_opcode_tests("2d");
  run_opcode_tests("3d");
  run_opcode_tests("39");
  run_opcode_tests("21");
  run_opcode_tests("31");
}

#[test]
fn asl() {
  run_opcode_tests("0a");
  run_opcode_tests("06");
  run_opcode_tests("16");
  run_opcode_tests("0e");
  run_opcode_tests("1e");
}

#[test]
fn bcc() {
  run_opcode_tests("90");
}

#[test]
fn bcs() {
  run_opcode_tests("b0");
}

#[test]
fn beq() {
  run_opcode_tests("f0");
}

#[test]
fn bit() {
  run_opcode_tests("24");
  run_opcode_tests("2c");
}

#[test]
fn bmi() {
  run_opcode_tests("30");
}

#[test]
fn bne() {
  run_opcode_tests("d0");
}

#[test]
fn bpl() {
  run_opcode_tests("10");
}

#[test]
fn brk() {
  run_opcode_tests("00");
}

#[test]
fn bvc() {
  run_opcode_tests("50");
}

#[test]
fn bvs() {
  run_opcode_tests("70");
}

#[test]
fn clc() {
  run_opcode_tests("18");
}

#[test]
fn cld() {
  run_opcode_tests("d8");
}

#[test]
fn cli() {
  run_opcode_tests("58");
}

#[test]
fn clv() {
  run_opcode_tests("b8");
}

#[test]
fn cmp() {
  run_opcode_tests("c9");
  run_opcode_tests("c5");
  run_opcode_tests("d5");
  run_opcode_tests("cd");
  run_opcode_tests("dd");
  run_opcode_tests("d9");
  run_opcode_tests("c1");
  run_opcode_tests("d1");
}

#[test]
fn cpx() {
  run_opcode_tests("e0");
  run_opcode_tests("e4");
  run_opcode_tests("ec");
}

#[test]
fn cpy() {
  run_opcode_tests("c0");
  run_opcode_tests("c4");
  run_opcode_tests("cc");
}

#[test]
fn dec() {
  run_opcode_tests("c6");
  run_opcode_tests("d6");
  run_opcode_tests("ce");
  run_opcode_tests("de");
}

#[test]
fn dex() {
  run_opcode_tests("ca");
}

#[test]
fn dey() {
  run_opcode_tests("88");
}

#[test]
fn eor() {
  run_opcode_tests("49");
  run_opcode_tests("45");
  run_opcode_tests("55");
  run_opcode_tests("4d");
  run_opcode_tests("5d");
  run_opcode_tests("59");
  run_opcode_tests("41");
  run_opcode_tests("51");
}

#[test]
fn inc() {
  run_opcode_tests("e6");
  run_opcode_tests("f6");
  run_opcode_tests("ee");
  run_opcode_tests("fe");
}

#[test]
fn inx() {
  run_opcode_tests("e8");
}

#[test]
fn iny() {
  run_opcode_tests("c8");
}

#[test]
fn jmp() {
  run_opcode_tests("4c");
  run_opcode_tests("6c");
}

#[test]
fn jsr() {
  run_opcode_tests("20");
}

#[test]
fn lda() {
  run_opcode_tests("a9");
  run_opcode_tests("a5");
  run_opcode_tests("b5");
  run_opcode_tests("ad");
  run_opcode_tests("bd");
  run_opcode_tests("b9");
  run_opcode_tests("a1");
  run_opcode_tests("b1");
}

#[test]
fn ldx() {
  run_opcode_tests("a2");
  run_opcode_tests("a6");
  run_opcode_tests("b6");
  run_opcode_tests("ae");
  run_opcode_tests("be");
}

#[test]
fn ldy() {
  run_opcode_tests("a0");
  run_opcode_tests("a4");
  run_opcode_tests("b4");
  run_opcode_tests("ac");
  run_opcode_tests("bc");
}

#[test]
fn lsr() {
  run_opcode_tests("4a");
  run_opcode_tests("46");
  run_opcode_tests("56");
  run_opcode_tests("4e");
  run_opcode_tests("5e");
}

#[test]
fn nop() {
  run_opcode_tests("ea");
}

#[test]
fn ora() {
  run_opcode_tests("09");
  run_opcode_tests("05");
  run_opcode_tests("15");
  run_opcode_tests("0d");
  run_opcode_tests("1d");
  run_opcode_tests("19");
  run_opcode_tests("01");
  run_opcode_tests("11");
}

#[test]
fn pha() {
  run_opcode_tests("48");
}

#[test]
fn php() {
  run_opcode_tests("08");
}

#[test]
fn pla() {
  run_opcode_tests("68");
}

#[test]
fn plp() {
  run_opcode_tests("28");
}

#[test]
fn rol() {
  run_opcode_tests("2a");
  run_opcode_tests("26");
  run_opcode_tests("36");
  run_opcode_tests("2e");
  run_opcode_tests("3e");
}

#[test]
fn ror() {
  run_opcode_tests("6a");
  run_opcode_tests("66");
  run_opcode_tests("76");
  run_opcode_tests("6e");
  run_opcode_tests("7e");
}

#[test]
fn rti() {
  run_opcode_tests("40");
}

#[test]
fn rts() {
  run_opcode_tests("60");
}

#[test]
fn sbc() {
  run_opcode_tests("e9");
  run_opcode_tests("e5");
  run_opcode_tests("f5");
  run_opcode_tests("ed");
  run_opcode_tests("fd");
  run_opcode_tests("f9");
  run_opcode_tests("e1");
  run_opcode_tests("f1");
}

#[test]
fn sec() {
  run_opcode_tests("38");
}

#[test]
fn sed() {
  run_opcode_tests("f8");
}

#[test]
fn sei() {
  run_opcode_tests("78");
}

#[test]
fn sta() {
  run_opcode_tests("85");
  run_opcode_tests("95");
  run_opcode_tests("8d");
  run_opcode_tests("9d");
  run_opcode_tests("99");
  run_opcode_tests("81");
  run_opcode_tests("91");
}

#[test]
fn stx() {
  run_opcode_tests("86");
  run_opcode_tests("96");
  run_opcode_tests("8e");
}

#[test]
fn sty() {
  run_opcode_tests("84");
  run_opcode_tests("94");
  run_opcode_tests("8c");
}

#[test]
fn tax() {
  run_opcode_tests("aa");
}

#[test]
fn tay() {
  run_opcode_tests("a8");
}

#[test]
fn tsx() {
  run_opcode_tests("ba");
}

#[test]
fn txa() {
  run_opcode_tests("8a");
}

#[test]
fn txs() {
  run_opcode_tests("9a");
}

#[test]
fn tya() {
  run_opcode_tests("98");
}

fn run_opcode_tests(filename: &str) {
  let file = std::fs::read(Path::new(&format!("D:/ProcessorTests-main/nes6502/v1/{}.json", filename))).unwrap();
  let json: serde_json::Value = serde_json::from_slice(file.as_slice()).unwrap();

  // Create bus
  let bus = Rc::new(RefCell::new(Box::new(MockBus::new()) as Box<dyn BusLike>));

  // Create CPU
  let cpu = Rc::new(RefCell::new(NES6502::new()));

  // Connect bus to CPU
  {
    let mut bus_ref = bus.borrow_mut();
    let cpu_ref = Rc::clone(&cpu);
    bus_ref.connect_cpu(Rc::clone(&cpu_ref));
  }

  // Connect CPU to bus
  {
    let mut cpu_ref = cpu.borrow_mut();
    let bus_ref = Rc::clone(&bus);
    cpu_ref.connect_to_bus(Rc::clone(&bus_ref));
  }

  for i in 0..json.as_array().unwrap().len() {
    println!("Running test {} of opcode {}", i, filename);
    // Extract the values we need from the JSON
    let entry = &*json.get(i).unwrap();
    let initial = entry.get("initial").unwrap();
    let final_state = entry.get("final").unwrap();
  
    // Write our starting RAM state to CPU RAM
    let initial_ram = initial.get("ram").unwrap().as_array().unwrap();
    for (i, entry) in initial_ram.iter().enumerate() {
      let address = entry.get(0).unwrap().as_u64().unwrap();
      let data = entry.get(1).unwrap().as_u64().unwrap();
      bus.borrow_mut().cpu_write(address as u16, data as u8);
    }
  
    // Set our starting register values
    let initial_pc = initial.get("pc").unwrap().as_u64().unwrap() as u16;
    let initial_sp = initial.get("s").unwrap().as_u64().unwrap() as u8;
    let initial_a = initial.get("a").unwrap().as_u64().unwrap() as u8;
    let initial_x = initial.get("x").unwrap().as_u64().unwrap() as u8;
    let initial_y = initial.get("y").unwrap().as_u64().unwrap() as u8;
    let initial_flags = initial.get("p").unwrap().as_u64().unwrap() as u8;
  
    cpu.borrow_mut().pc = initial_pc;
    cpu.borrow_mut().sp = initial_sp;
    cpu.borrow_mut().a = initial_a;
    cpu.borrow_mut().x = initial_x;
    cpu.borrow_mut().y = initial_y;
    cpu.borrow_mut().flags = Flags::from_u8(initial_flags);
  
    // Read the opcode and let it execute the instruction fully
    cpu.borrow_mut().step();
    while cpu.borrow().cycles > 0 {
      cpu.borrow_mut().step();
    }

    let final_pc = final_state.get("pc").unwrap().as_u64().unwrap() as u16;
    let final_sp = final_state.get("s").unwrap().as_u64().unwrap() as u8;
    let final_a = final_state.get("a").unwrap().as_u64().unwrap() as u8;
    let final_x = final_state.get("x").unwrap().as_u64().unwrap() as u8;
    let final_y = final_state.get("y").unwrap().as_u64().unwrap() as u8;
    let final_flags = final_state.get("p").unwrap().as_u64().unwrap() as u8;
  
    assert_eq!(cpu.borrow().pc, final_pc);
    assert_eq!(cpu.borrow().sp, final_sp);
    assert_eq!(cpu.borrow().a, final_a);
    assert_eq!(cpu.borrow().x, final_x);
    assert_eq!(cpu.borrow().y, final_y);
    assert_eq!(cpu.borrow().flags.to_u8(), final_flags);
  
    let final_ram = final_state.get("ram").unwrap().as_array().unwrap();
    for (_, entry) in final_ram.iter().enumerate() {
      let address = entry.get(0).unwrap().as_u64().unwrap() as u16;
      let data = entry.get(1).unwrap().as_u64().unwrap() as u8;
      assert_eq!(bus.borrow_mut().cpu_read(address), data);
    }
  }
}