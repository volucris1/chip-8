#[cfg(test)]
mod opcode {
    use crate::chip8::opcode::Opcode;

    #[test]
    fn set_from_u8() {
        let mut opcode = Opcode::new();
        opcode.set_from_u8(0x12, 0x34);

        assert_eq!(opcode.code(), 0x1234);
    }

    #[test]
    fn nnn() {
        let mut opcode = Opcode::new();
        opcode.set_from_u16(0x1234);

        assert_eq!(opcode.nnn(), 0x0234);
    }

    #[test]
    fn nn() {
        let mut opcode = Opcode::new();
        opcode.set_from_u16(0x1234);

        assert_eq!(opcode.nn(), 0x0034);
    }

    #[test]
    fn n() {
        let mut opcode = Opcode::new();
        opcode.set_from_u16(0x1234);

        assert_eq!(opcode.n(), 0x0004);
    }

    #[test]
    fn x() {
        let mut opcode = Opcode::new();
        opcode.set_from_u16(0x1234);

        assert_eq!(opcode.x(), 0x2);
    }

    #[test]
    fn y() {
        let mut opcode = Opcode::new();
        opcode.set_from_u16(0x1234);

        assert_eq!(opcode.y(), 0x3);
    }
}

#[cfg(test)]
mod instructions {
    use super::super::Chip8;
    #[test]
    fn cls_00e0() {
        let mut chip8 = Chip8::new();

        let vram_prev = [[0; 64]; 32];
        let vram = [[1; 64]; 32];
        chip8.screen.set_vram(vram);

        chip8.cls_00e0();
        assert_eq!(
            vram_prev,
            chip8.screen.vram(),
            "Screen should be fill by 0 values"
        );
    }
    #[test]
    fn ret_00ee() {
        let mut chip8 = Chip8::new();

        chip8.stack.call(123);

        chip8.ret_00ee();
        assert_eq!(
            chip8.stack.stack(),
            vec![],
            "It suppouse to remove last element from stack and set pc to it"
        );
    }
    #[test]
    fn jp_1nnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0x1123);

        chip8.jp_1nnn();
        assert_eq!(chip8.pc, 0x0123);
    }
    #[test]
    fn call_2nnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0x2123);

        chip8.call_2nnn();
        assert_eq!(chip8.pc, 0x0123, "Program counter should change to 0x0123");
        assert_eq!(
            chip8.stack.stack(),
            vec![0x0200],
            "Stack should containt previous Program counter value, which is 0x0200 on start"
        );
    }
    #[test]
    fn se_3xnn() {
        let mut chip8 = Chip8::new();

        chip8.v[0] = 0x12;
        chip8.opcode.set_from_u16(0x3012);

        let pc = chip8.pc;
        chip8.se_3xnn();
        assert_eq!(chip8.pc, pc + 2);

        let pc = chip8.pc;
        chip8.v[0] = 0x00;

        chip8.se_3xnn();
        assert_eq!(chip8.pc, pc);
    }
    #[test]
    fn sne_4xnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0x3012);

        let pc = chip8.pc;
        chip8.v[0] = 0x12;
        chip8.sne_4xnn();
        assert_eq!(chip8.pc, pc);

        let pc = chip8.pc;
        chip8.v[0] = 0x00;

        chip8.sne_4xnn();
        assert_eq!(chip8.pc, pc + 2);
    }
    #[test]
    fn se_5xy0() {
        let mut chip8 = Chip8::new();

        let pc = chip8.pc;
        chip8.v[0] = 0x12;
        chip8.v[1] = 0x12;
        chip8.opcode.set_from_u16(0x3010);

        chip8.se_5xy0();
        assert_eq!(chip8.pc, pc + 2);

        let pc = chip8.pc;
        chip8.v[0] = 0x00;

        chip8.se_5xy0();
        assert_eq!(chip8.pc, pc);
    }
    #[test]
    fn ld_6xnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0x3010);

        chip8.ld_6xnn();
        assert_eq!(chip8.v[0], 0x10);
    }
    #[test]
    fn add_7xnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0x3010);

        chip8.add_7xnn();
        assert_eq!(chip8.v[0], 0x10);

        chip8.add_7xnn();
        assert_eq!(chip8.v[0], 0x20);
    }
    #[test]
    fn ld_8xy0() {
        let mut chip8 = Chip8::new();

        chip8.v[0x1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);

        chip8.ld_8xy0();
        assert_eq!(chip8.v[0], chip8.v[1]);
    }
    #[test]
    fn or_8xy1() {
        let mut chip8 = Chip8::new();

        chip8.v[0x1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);

        chip8.or_8xy1();
        assert_eq!(chip8.v[0], chip8.v[0] | chip8.v[1]);
    }
    #[test]
    fn and_8xy2() {
        let mut chip8 = Chip8::new();

        chip8.v[0x1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);

        chip8.and_8xy2();
        assert_eq!(chip8.v[0], chip8.v[0] & chip8.v[1]);
    }
    #[test]
    fn xor_8xy3() {
        let mut chip8 = Chip8::new();

        chip8.v[0x1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);

        chip8.xor_8xy3();
        assert_eq!(chip8.v[0], chip8.v[0] ^ chip8.v[1]);
    }
    #[test]
    fn add_8xy4() {
        let mut chip8 = Chip8::new();

        chip8.v[0x0] = 0xFF;
        chip8.v[0x1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);

        chip8.add_8xy4();
        assert_eq!(chip8.v[0xF], 1);

        chip8.add_8xy4();
        assert_eq!(chip8.v[0xF], 0);
    }
    #[test]
    fn sub_8xy5() {
        let mut chip8 = Chip8::new();

        chip8.v[0x0] = 0xFF;
        chip8.v[0x1] = 0xAA;
        chip8.opcode.set_from_u16(0x8010);

        chip8.sub_8xy5();
        assert_eq!(chip8.v[0xF], 1);
    }
    #[test]
    fn shr_8xy6() {
        let mut chip8 = Chip8::new();

        chip8.v[0x0] = 0xFF;
        chip8.v[0x1] = 0x1;
        chip8.opcode.set_from_u16(0x8010);

        chip8.shr_8xy6();
        assert_eq!(chip8.v[0xF], 1);
    }
    #[test]
    fn subn_8xy7() {
        let mut chip8 = Chip8::new();

        chip8.v[0x0] = 0xAC;
        chip8.v[0x1] = 0xAA;
        chip8.opcode.set_from_u16(0x8010);

        chip8.subn_8xy7();
        assert_eq!(chip8.v[0xF], 0);
    }
    #[test]
    fn shl_8xye() {
        let mut chip8 = Chip8::new();

        chip8.v[0x0] = 0xFF;
        chip8.v[0x1] = 0x1;
        chip8.opcode.set_from_u16(0x8010);

        chip8.shl_8xye();
        assert_eq!(chip8.v[0xF], 1);
    }
    #[test]
    fn sne_9xy0() {
        let mut chip8 = Chip8::new();

        chip8.v[0] = 0x10;
        chip8.v[1] = 0x12;
        chip8.opcode.set_from_u16(0x8010);
        let pc = chip8.pc;
        chip8.sne_9xy0();
        assert_eq!(chip8.pc, pc + 2);

        chip8.v[0] = 0x10;
        chip8.v[1] = 0x10;
        let pc = chip8.pc;
        chip8.sne_9xy0();
        assert_eq!(chip8.pc, pc);
    }
    #[test]
    fn ld_annn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0xA123);

        chip8.ld_annn();
        assert_eq!(chip8.i, 0x0123);
    }
    #[test]
    fn jp_bnnn() {
        let mut chip8 = Chip8::new();

        chip8.opcode.set_from_u16(0xA123);

        chip8.v[0] = 0x10;

        let v0 = chip8.v[0] as usize;
        chip8.jp_bnnn();

        assert_eq!(chip8.pc, v0 + 0x0123);
    }
}
