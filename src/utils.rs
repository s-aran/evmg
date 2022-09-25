pub mod utils {
    pub fn print_hex(s: &Vec<u8>) {
        print!("     | +0 +1 +2 +3 +4 +5 +6 +7  +8 +9 +A +B +C +D +E +F");
        print!(" | 0123456789ABCDEF");
        println!("");

        print!("-----+-------------------------------------------------");
        print!("-+-----------------");

        let mut chars: Vec<char> = Vec::<char>::with_capacity(16);
        for (i, e) in s.iter().enumerate() {
            if (i & 0x0F) == 0 {
                println!("");
                print!("{:04X} |", i);
            }

            if (i & 0x0F) == 0x08 {
                print!(" ");
            }

            print!(" {:02X}", e);
            chars.push(match std::char::from_u32(*e as u32) {
                Some(c) => {
                    if c.is_ascii_graphic() {
                        c
                    } else {
                        '.'
                    }
                }
                None => '.',
            });

            if i > 0 && (i & 0x0F) == 0x0F {
                print!(" | ");
                for c in &chars {
                    print!("{}", c);
                }
                chars.clear();
            }
        }

        let slen_mod_16 = s.len() & 0x0F;
        if slen_mod_16 > 0 {
            for _ in 0..(16 - slen_mod_16) {
                print!("   ");
            }
            if slen_mod_16 <= 9 {
                print!(" ");
            }
            print!(" | ");
            for i in 0..(slen_mod_16) {
                print!(
                    "{}",
                    match std::char::from_u32(*s.get((s.len() & 0xFFFF_FFF0) | i).unwrap() as u32) {
                        Some(c) => {
                            if c.is_ascii_graphic() {
                                c
                            } else {
                                '.'
                            }
                        }
                        None => '.',
                    }
                );
            }
        }

        println!("");
    }

    pub fn print_vec(v: &Vec<String>) {
        println!("----------------");
        for e in v {
            println!("  - {}", e);
        }
        println!("----------------");
    }
}