pub struct MD4 {
    block_len: u64,
    state: [u32; 4],
}

impl MD4 {
    pub fn new() -> Self {
        let state = [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476];
        Self {
            state,
            block_len: 0,
        }
    }

    //fn compress(state: &mut [u32; 4], input: &[u8]) {
    fn compress(&mut self, input: &[u8]) {
        fn f(x: u32, y: u32, z: u32) -> u32 {
            (x & y) | (!x & z)
        }

        fn g(x: u32, y: u32, z: u32) -> u32 {
            (x & y) | (x & z) | (y & z)
        }

        fn h(x: u32, y: u32, z: u32) -> u32 {
            x ^ y ^ z
        }

        fn op1(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
            a.wrapping_add(f(b, c, d)).wrapping_add(k).rotate_left(s)
        }

        fn op2(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
            a.wrapping_add(g(b, c, d))
                .wrapping_add(k)
                .wrapping_add(0x5A82_7999)
                .rotate_left(s)
        }

        fn op3(a: u32, b: u32, c: u32, d: u32, k: u32, s: u32) -> u32 {
            a.wrapping_add(h(b, c, d))
                .wrapping_add(k)
                .wrapping_add(0x6ED9_EBA1)
                .rotate_left(s)
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        // load block to data
        let mut data = [0u32; 16]; // 32/8 == 4; 4*16 == 64
        for (o, chunk) in data.iter_mut().zip(input.chunks_exact(4)) {
            *o = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        // round 1
        for &i in &[0, 4, 8, 12] {
            a = op1(a, b, c, d, data[i], 3);
            d = op1(d, a, b, c, data[i + 1], 7);
            c = op1(c, d, a, b, data[i + 2], 11);
            b = op1(b, c, d, a, data[i + 3], 19);
        }

        // round 2
        for i in 0..4 {
            a = op2(a, b, c, d, data[i], 3);
            d = op2(d, a, b, c, data[i + 4], 5);
            c = op2(c, d, a, b, data[i + 8], 9);
            b = op2(b, c, d, a, data[i + 12], 13);
        }

        // round 3
        for &i in &[0, 2, 1, 3] {
            a = op3(a, b, c, d, data[i], 3);
            d = op3(d, a, b, c, data[i + 8], 9);
            c = op3(c, d, a, b, data[i + 4], 11);
            b = op3(b, c, d, a, data[i + 12], 15);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    pub fn get_hash(&self) -> [u8; 16] {
        let mut out: [u8; 16] = [0_u8; 16];
        for (chunk, v) in out.chunks_exact_mut(4).zip(self.state.iter()) {
            chunk.copy_from_slice(&v.to_le_bytes());
        }
        out
    }

    pub fn digest(&mut self, input: &[u8]) {
        self.block_len = self.block_len.wrapping_add(input.len() as u64);
        let total = ((self.block_len / 55) + 1) * 64;

        match total {
            64 => {
                for block in self.pad_64(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            128 => {
                for block in self.pad_128(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            192 => {
                for block in self.pad_192(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            256 => {
                for block in self.pad_256(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            320 => {
                for block in self.pad_320(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            384 => {
                for block in self.pad_384(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            448 => {
                for block in self.pad_448(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            512 => {
                for block in self.pad_512(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            576 => {
                for block in self.pad_576(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            640 => {
                for block in self.pad_640(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            704 => {
                for block in self.pad_704(&input).chunks_exact(64) {
                    self.compress(block);
                }
            },
            _ => panic!("input too big to hash: {total}"),
        }
    }

    fn pad_64(&self, input: &[u8]) -> [u8; 64] {
        const TOTAL: usize = 64;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_128(&self, input: &[u8]) -> [u8; 128] {
        const TOTAL: usize = 128;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_192(&self, input: &[u8]) -> [u8; 192] {
        const TOTAL: usize = 192;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_256(&self, input: &[u8]) -> [u8; 256] {
        const TOTAL: usize = 256;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_320(&self, input: &[u8]) -> [u8; 320] {
        const TOTAL: usize = 320;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_384(&self, input: &[u8]) -> [u8; 384] {
        const TOTAL: usize = 384;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_448(&self, input: &[u8]) -> [u8; 448] {
        const TOTAL: usize = 448;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_512(&self, input: &[u8]) -> [u8; 512] {
        const TOTAL: usize = 512;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_576(&self, input: &[u8]) -> [u8; 576] {
        const TOTAL: usize = 576;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_640(&self, input: &[u8]) -> [u8; 640] {
        const TOTAL: usize = 640;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }

    fn pad_704(&self, input: &[u8]) -> [u8; 704] {
        const TOTAL: usize = 704;
        let mut buffer = [0_u8; TOTAL];
        let block_len = self.block_len as usize;
        let bit_len: [u8; 8] = self.block_len.wrapping_mul(8).to_le_bytes();

        buffer[..block_len].copy_from_slice(input);
        buffer[block_len] = 0x80;
        buffer[TOTAL - 8..].copy_from_slice(&bit_len);
        buffer
    }
}
