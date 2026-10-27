//! Mechanically lifted functions (scripts/lift_run.py), module g001. Each one is checked against the original
//! in the world harness (tests/world_g001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x10128 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010128<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10128;
    loop {
        match pc {
        0x10128 => {
            // block 10128
            // 10128 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10129 mov rbp,rsp
            rbp = rsp;
            // 1012c mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 1012f mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 10133 mov rdx,rax
            rdx = rax;
            // 10136 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 10139 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // 1013d cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 10140 jae 0x1014b
            if fl.ae() { pc = 0x1014b; continue; }
            pc = 0x10142;
        }
        0x10142 => {
            // block 10142
            // 10142 sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // 10145 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10146 jmp 0xeb8a0
            { let r = { let t = m.env.stub_va(0x6d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0x1014b => {
            // block 1014b
            // 1014b jbe 0x1016b
            if fl.be() { pc = 0x1016b; continue; }
            pc = 0x1014d;
        }
        0x1014d => {
            // block 1014d
            // 1014d lea rcx,[rcx+rsi*8]
            rcx = rcx.wrapping_add(rsi.wrapping_mul(8));
            // 10151 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 10154 je 0x1016b
            if fl.e() { pc = 0x1016b; continue; }
            pc = 0x10156;
        }
        0x10156 => {
            // block 10156
            // 10156 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 1015a sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 1015d not rdx
            rdx = !rdx;
            // 10160 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 10164 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 10167 mov QWORD PTR [rdi+0x8],rdx
            m.w64(rdi.wrapping_add(0x8), (rdx) as u64);
            pc = 0x1016b;
        }
        0x1016b => {
            // block 1016b
            // 1016b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1016c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11222 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011222<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11222;
    loop {
        match pc {
        0x11222 => {
            // block 11222
            // 11222 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11223 mov rbp,rsp
            rbp = rsp;
            // 11226 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11227 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11228 mov rbx,rdi
            rbx = rdi;
            // 1122b call 0xe0758
            { let r = { let t = m.env.code_va(0xe0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11230 lea rax,[rip+0x130069] # 0x1412a0
            rax = m.env.data_va(350829);
            // 11237 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1123a mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 11242 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 1124a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1124e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1124f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11250 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2253 size=13
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2253<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd2253;
    loop {
        match pc {
        0xd2253 => {
            // block d2253
            // d2253 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2254 mov rbp,rsp
            rbp = rsp;
            // d2257 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // d225a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d225b jmp 0xd2260
            { let r = { let t = m.env.code_va(0xd2260); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb790c size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b790c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb790c;
    loop {
        match pc {
        0xb790c => {
            // block b790c
            // b790c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b790d mov rbp,rsp
            rbp = rsp;
            // b7910 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7912 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7913 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // b7916 mov rbx,rdi
            rbx = rdi;
            // b7919 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b791c call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7921 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b7924 cmp eax,r14d
            fl = sub::<32>((rax as u32 as u64), (r14 as u32 as u64)).1;
            // b7927 jbe 0xb792d
            if fl.be() { pc = 0xb792d; continue; }
            pc = 0xb7929;
        }
        0xb7929 => {
            // block b7929
            // b7929 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b792b jmp 0xb7948
            pc = 0xb7948; continue;
        }
        0xb792d => {
            // block b792d
            // b792d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b7930 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // b7935 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b793a xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b793c movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // b7940 cmp ecx,0x4
            fl = sub::<32>((rcx as u32 as u64), 0x4u64).1;
            // b7943 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b7946 or al,cl
            { let r = (rax as u8 as u64) | (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            pc = 0xb7948;
        }
        0xb7948 => {
            // block b7948
            // b7948 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7949 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b794b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b794c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd1370 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d1370<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd1370;
    loop {
        match pc {
        0xd1370 => {
            // block d1370
            // d1370 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d1371 mov rbp,rsp
            rbp = rsp;
            // d1374 cmp esi,0x3
            fl = sub::<32>((rsi as u32 as u64), 0x3u64).1;
            // d1377 jbe 0xd137d
            if fl.be() { pc = 0xd137d; continue; }
            pc = 0xd1379;
        }
        0xd1379 => {
            // block d1379
            // d1379 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d137b jmp 0xd13ac
            pc = 0xd13ac; continue;
        }
        0xd137d => {
            // block d137d
            // d137d lea rax,[rip+0x2c] # 0xd13b0
            rax = m.env.code_va(0xd13b0);
            // d1384 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d1388 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d138b jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd138d => {
            // block d138d
            // d138d mov esi,0x66
            rsi = (0x66u64) as u32 as u64;
            // d1392 jmp 0xd13a7
            pc = 0xd13a7; continue;
        }
        0xd1394 => {
            // block d1394
            // d1394 mov esi,0x67
            rsi = (0x67u64) as u32 as u64;
            // d1399 jmp 0xd13a7
            pc = 0xd13a7; continue;
        }
        0xd139b => {
            // block d139b
            // d139b mov esi,0x68
            rsi = (0x68u64) as u32 as u64;
            // d13a0 jmp 0xd13a7
            pc = 0xd13a7; continue;
        }
        0xd13a2 => {
            // block d13a2
            // d13a2 mov esi,0x69
            rsi = (0x69u64) as u32 as u64;
            pc = 0xd13a7;
        }
        0xd13a7 => {
            // block d13a7
            // d13a7 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd13ac;
        }
        0xd13ac => {
            // block d13ac
            // d13ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d13ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd12f4 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d12f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd12f4;
    loop {
        match pc {
        0xd12f4 => {
            // block d12f4
            // d12f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d12f5 mov rbp,rsp
            rbp = rsp;
            // d12f8 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // d12fb jbe 0xd1301
            if fl.be() { pc = 0xd1301; continue; }
            pc = 0xd12fd;
        }
        0xd12fd => {
            // block d12fd
            // d12fd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d12ff jmp 0xd134c
            pc = 0xd134c; continue;
        }
        0xd1301 => {
            // block d1301
            // d1301 lea rax,[rip+0x48] # 0xd1350
            rax = m.env.code_va(0xd1350);
            // d1308 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d130c add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d130f jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd1311 => {
            // block d1311
            // d1311 mov esi,0x14
            rsi = (0x14u64) as u32 as u64;
            // d1316 jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd1318 => {
            // block d1318
            // d1318 mov esi,0x15
            rsi = (0x15u64) as u32 as u64;
            // d131d jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd131f => {
            // block d131f
            // d131f mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // d1324 jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd1326 => {
            // block d1326
            // d1326 mov esi,0x17
            rsi = (0x17u64) as u32 as u64;
            // d132b jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd132d => {
            // block d132d
            // d132d mov esi,0x45
            rsi = (0x45u64) as u32 as u64;
            // d1332 jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd1334 => {
            // block d1334
            // d1334 mov esi,0x46
            rsi = (0x46u64) as u32 as u64;
            // d1339 jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd133b => {
            // block d133b
            // d133b mov esi,0x47
            rsi = (0x47u64) as u32 as u64;
            // d1340 jmp 0xd1347
            pc = 0xd1347; continue;
        }
        0xd1342 => {
            // block d1342
            // d1342 mov esi,0x48
            rsi = (0x48u64) as u32 as u64;
            pc = 0xd1347;
        }
        0xd1347 => {
            // block d1347
            // d1347 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd134c;
        }
        0xd134c => {
            // block d134c
            // d134c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d134d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd13c0 size=260
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d13c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd13c0;
    loop {
        match pc {
        0xd13c0 => {
            // block d13c0
            // d13c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d13c1 mov rbp,rsp
            rbp = rsp;
            // d13c4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d13c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d13c7 mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // d13ca cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // d13cd jbe 0xd13d6
            if fl.be() { pc = 0xd13d6; continue; }
            pc = 0xd13cf;
        }
        0xd13cf => {
            // block d13cf
            // d13cf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d13d1 jmp 0xd149c
            pc = 0xd149c; continue;
        }
        0xd13d6 => {
            // block d13d6
            // d13d6 lea rax,[rip+0xc7] # 0xd14a4
            rax = m.env.code_va(0xd14a4);
            // d13dd movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d13e1 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d13e4 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd13e6 => {
            // block d13e6
            // d13e6 mov esi,0xc
            rsi = (0xcu64) as u32 as u64;
            // d13eb mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d13ee call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d13f3 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d13f5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d13f7 jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd13fd;
        }
        0xd13fd => {
            // block d13fd
            // d13fd mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // d1402 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d1405 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d140a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d140c jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd1412;
        }
        0xd1412 => {
            // block d1412
            // d1412 mov esi,0x62
            rsi = (0x62u64) as u32 as u64;
            // d1417 jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd1419 => {
            // block d1419
            // d1419 mov esi,0xd
            rsi = (0xdu64) as u32 as u64;
            // d141e mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d1421 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1426 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d1428 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d142a jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd142c;
        }
        0xd142c => {
            // block d142c
            // d142c mov esi,0xb
            rsi = (0xbu64) as u32 as u64;
            // d1431 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d1434 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1439 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d143b jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd143d;
        }
        0xd143d => {
            // block d143d
            // d143d mov esi,0x63
            rsi = (0x63u64) as u32 as u64;
            // d1442 jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd1444 => {
            // block d1444
            // d1444 mov esi,0xe
            rsi = (0xeu64) as u32 as u64;
            // d1449 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d144c call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1451 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d1453 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d1455 jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd1457;
        }
        0xd1457 => {
            // block d1457
            // d1457 mov esi,0x64
            rsi = (0x64u64) as u32 as u64;
            // d145c jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd145e => {
            // block d145e
            // d145e mov esi,0xf
            rsi = (0xfu64) as u32 as u64;
            // d1463 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d1466 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d146b mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d146d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d146f jne 0xd149c
            if fl.ne() { pc = 0xd149c; continue; }
            pc = 0xd1471;
        }
        0xd1471 => {
            // block d1471
            // d1471 mov esi,0x65
            rsi = (0x65u64) as u32 as u64;
            // d1476 jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd1478 => {
            // block d1478
            // d1478 mov esi,0x41
            rsi = (0x41u64) as u32 as u64;
            // d147d jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd147f => {
            // block d147f
            // d147f mov esi,0x42
            rsi = (0x42u64) as u32 as u64;
            // d1484 jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd1486 => {
            // block d1486
            // d1486 mov esi,0x43
            rsi = (0x43u64) as u32 as u64;
            // d148b jmp 0xd1492
            pc = 0xd1492; continue;
        }
        0xd148d => {
            // block d148d
            // d148d mov esi,0x44
            rsi = (0x44u64) as u32 as u64;
            pc = 0xd1492;
        }
        0xd1492 => {
            // block d1492
            // d1492 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // d1495 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d149a mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd149c;
        }
        0xd149c => {
            // block d149c
            // d149c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d149e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d149f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d14a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d14a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8027 size=321
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8027<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8027;
    loop {
        match pc {
        0xc8027 => {
            // block c8027
            // c8027 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8028 mov rbp,rsp
            rbp = rsp;
            // c802b cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // c802e ja 0xc8149
            if fl.a() { pc = 0xc8149; continue; }
            pc = 0xc8034;
        }
        0xc8034 => {
            // block c8034
            // c8034 lea rax,[rip+0x111] # 0xc814c
            rax = m.env.code_va(0xc814c);
            // c803b movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c803f add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // c8042 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8044 => {
            // block c8044
            // c8044 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8046 je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc804c;
        }
        0xc804c => {
            // block c804c
            // c804c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc804e;
        }
        0xc804e => {
            // block c804e
            // c804e mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8050 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c8056 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // c8059 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c805f lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // c8062 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c8068 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // c806b mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c8071 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8074 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c8076 jb 0xc804e
            if fl.b() { pc = 0xc804e; continue; }
            pc = 0xc8078;
        }
        0xc8078 => {
            // block c8078
            // c8078 jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc807d => {
            // block c807d
            // c807d add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c807f je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc8085;
        }
        0xc8085 => {
            // block c8085
            // c8085 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc8087;
        }
        0xc8087 => {
            // block c8087
            // c8087 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8089 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c808f lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // c8092 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c8098 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c809b cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c809d jb 0xc8087
            if fl.b() { pc = 0xc8087; continue; }
            pc = 0xc809f;
        }
        0xc809f => {
            // block c809f
            // c809f jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc80a4 => {
            // block c80a4
            // c80a4 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c80a6 je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc80ac;
        }
        0xc80ac => {
            // block c80ac
            // c80ac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc80ae;
        }
        0xc80ae => {
            // block c80ae
            // c80ae lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // c80b1 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c80b7 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // c80ba mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c80c0 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // c80c3 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c80c9 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c80cc cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c80ce jb 0xc80ae
            if fl.b() { pc = 0xc80ae; continue; }
            pc = 0xc80d0;
        }
        0xc80d0 => {
            // block c80d0
            // c80d0 jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc80d2 => {
            // block c80d2
            // c80d2 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c80d4 je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc80d6;
        }
        0xc80d6 => {
            // block c80d6
            // c80d6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc80d8;
        }
        0xc80d8 => {
            // block c80d8
            // c80d8 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // c80db mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c80e1 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c80e4 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c80e6 jb 0xc80d8
            if fl.b() { pc = 0xc80d8; continue; }
            pc = 0xc80e8;
        }
        0xc80e8 => {
            // block c80e8
            // c80e8 jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc80ea => {
            // block c80ea
            // c80ea add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c80ec je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc80ee;
        }
        0xc80ee => {
            // block c80ee
            // c80ee xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc80f0;
        }
        0xc80f0 => {
            // block c80f0
            // c80f0 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c80f2 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c80f8 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // c80fb mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c8101 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // c8104 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c810a add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c810d cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c810f jb 0xc80f0
            if fl.b() { pc = 0xc80f0; continue; }
            pc = 0xc8111;
        }
        0xc8111 => {
            // block c8111
            // c8111 jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc8113 => {
            // block c8113
            // c8113 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c8115 je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc8117;
        }
        0xc8117 => {
            // block c8117
            // c8117 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc8119;
        }
        0xc8119 => {
            // block c8119
            // c8119 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c811b mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // c8121 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8124 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c8126 jb 0xc8119
            if fl.b() { pc = 0xc8119; continue; }
            pc = 0xc8128;
        }
        0xc8128 => {
            // block c8128
            // c8128 jmp 0xc8149
            pc = 0xc8149; continue;
        }
        0xc812a => {
            // block c812a
            // c812a add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c812c je 0xc8149
            if fl.e() { pc = 0xc8149; continue; }
            pc = 0xc812e;
        }
        0xc812e => {
            // block c812e
            // c812e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc8130;
        }
        0xc8130 => {
            // block c8130
            // c8130 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // c8133 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c8139 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // c813c mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // c8142 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8145 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // c8147 jb 0xc8130
            if fl.b() { pc = 0xc8130; continue; }
            pc = 0xc8149;
        }
        0xc8149 => {
            // block c8149
            // c8149 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c814a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58ca2 size=322
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058ca2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x58ca2;
    loop {
        match pc {
        0x58ca2 => {
            // block 58ca2
            // 58ca2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58ca3 mov rbp,rsp
            rbp = rsp;
            // 58ca6 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 58ca9 ja 0x58dc4
            if fl.a() { pc = 0x58dc4; continue; }
            pc = 0x58caf;
        }
        0x58caf => {
            // block 58caf
            // 58caf lea rax,[rip+0x112] # 0x58dc8
            rax = m.env.code_va(0x58dc8);
            // 58cb6 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 58cba add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 58cbd jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x58cbf => {
            // block 58cbf
            // 58cbf add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58cc1 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58cc7;
        }
        0x58cc7 => {
            // block 58cc7
            // 58cc7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58cc9;
        }
        0x58cc9 => {
            // block 58cc9
            // 58cc9 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58ccb mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58cd1 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58cd4 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58cda lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58cdd mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58ce3 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58ce6 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58cec add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58cef cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58cf1 jb 0x58cc9
            if fl.b() { pc = 0x58cc9; continue; }
            pc = 0x58cf3;
        }
        0x58cf3 => {
            // block 58cf3
            // 58cf3 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58cf8 => {
            // block 58cf8
            // 58cf8 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58cfa je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d00;
        }
        0x58d00 => {
            // block 58d00
            // 58d00 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d02;
        }
        0x58d02 => {
            // block 58d02
            // 58d02 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d04 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d0a lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d0d mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d13 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d16 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d18 jb 0x58d02
            if fl.b() { pc = 0x58d02; continue; }
            pc = 0x58d1a;
        }
        0x58d1a => {
            // block 58d1a
            // 58d1a jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d1f => {
            // block 58d1f
            // 58d1f add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d21 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d27;
        }
        0x58d27 => {
            // block 58d27
            // 58d27 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d29;
        }
        0x58d29 => {
            // block 58d29
            // 58d29 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58d2c mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d32 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d35 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d3b lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58d3e mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d44 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d47 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d49 jb 0x58d29
            if fl.b() { pc = 0x58d29; continue; }
            pc = 0x58d4b;
        }
        0x58d4b => {
            // block 58d4b
            // 58d4b jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d4d => {
            // block 58d4d
            // 58d4d add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d4f je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d51;
        }
        0x58d51 => {
            // block 58d51
            // 58d51 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d53;
        }
        0x58d53 => {
            // block 58d53
            // 58d53 lea ecx,[rax+0x2]
            rcx = (rax.wrapping_add(0x2)) as u32 as u64;
            // 58d56 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d5c add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d5f cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d61 jb 0x58d53
            if fl.b() { pc = 0x58d53; continue; }
            pc = 0x58d63;
        }
        0x58d63 => {
            // block 58d63
            // 58d63 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d65 => {
            // block 58d65
            // 58d65 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d67 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d69;
        }
        0x58d69 => {
            // block 58d69
            // 58d69 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d6b;
        }
        0x58d6b => {
            // block 58d6b
            // 58d6b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d6d mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d73 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58d76 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d7c lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58d7f mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58d85 add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d88 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58d8a jb 0x58d6b
            if fl.b() { pc = 0x58d6b; continue; }
            pc = 0x58d8c;
        }
        0x58d8c => {
            // block 58d8c
            // 58d8c jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58d8e => {
            // block 58d8e
            // 58d8e add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58d90 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58d92;
        }
        0x58d92 => {
            // block 58d92
            // 58d92 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58d94;
        }
        0x58d94 => {
            // block 58d94
            // 58d94 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 58d96 mov WORD PTR [rdi+rcx*2],0x200
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x200u64) as u16);
            // 58d9c add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58d9f cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58da1 jb 0x58d94
            if fl.b() { pc = 0x58d94; continue; }
            pc = 0x58da3;
        }
        0x58da3 => {
            // block 58da3
            // 58da3 jmp 0x58dc4
            pc = 0x58dc4; continue;
        }
        0x58da5 => {
            // block 58da5
            // 58da5 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 58da7 je 0x58dc4
            if fl.e() { pc = 0x58dc4; continue; }
            pc = 0x58da9;
        }
        0x58da9 => {
            // block 58da9
            // 58da9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x58dab;
        }
        0x58dab => {
            // block 58dab
            // 58dab lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // 58dae mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58db4 lea ecx,[rax+0x3]
            rcx = (rax.wrapping_add(0x3)) as u32 as u64;
            // 58db7 mov WORD PTR [rdi+rcx*2],0x40
            m.w16(rdi.wrapping_add(rcx.wrapping_mul(2)), (0x40u64) as u16);
            // 58dbd add eax,0x4
            { let (r, f) = add::<32>((rax as u32 as u64), 0x4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58dc0 cmp eax,edx
            fl = sub::<32>((rax as u32 as u64), (rdx as u32 as u64)).1;
            // 58dc2 jb 0x58dab
            if fl.b() { pc = 0x58dab; continue; }
            pc = 0x58dc4;
        }
        0x58dc4 => {
            // block 58dc4
            // 58dc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58dc5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc75d9 size=339
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c75d9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc75d9;
    loop {
        match pc {
        0xc75d9 => {
            // block c75d9
            // c75d9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c75da mov rbp,rsp
            rbp = rsp;
            // c75dd push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c75df push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c75e1 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c75e3 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c75e5 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c75e6 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c75e8 je 0xc7707
            if fl.e() { pc = 0xc7707; continue; }
            pc = 0xc75ee;
        }
        0xc75ee => {
            // block c75ee
            // c75ee and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c75f1 lea r9,[rdi+0x2]
            r9 = rdi.wrapping_add(0x2);
            // c75f5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // c75f8 mov r10d,0xaaaaaaab
            r10 = (0xaaaaaaabu64) as u32 as u64;
            // c75fe lea r11,[rip+0x10f] # 0xc7714
            r11 = m.env.code_va(0xc7714);
            pc = 0xc7605;
        }
        0xc7605 => {
            // block c7605
            // c7605 cmp r8d,0x2
            fl = sub::<32>((r8 as u32 as u64), 0x2u64).1;
            // c7609 je 0xc7617
            if fl.e() { pc = 0xc7617; continue; }
            pc = 0xc760b;
        }
        0xc760b => {
            // block c760b
            // c760b cmp r8d,0x3
            fl = sub::<32>((r8 as u32 as u64), 0x3u64).1;
            // c760f jne 0xc7627
            if fl.ne() { pc = 0xc7627; continue; }
            pc = 0xc7611;
        }
        0xc7611 => {
            // block c7611
            // c7611 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7613 jne 0xc761b
            if fl.ne() { pc = 0xc761b; continue; }
            pc = 0xc7615;
        }
        0xc7615 => {
            // block c7615
            // c7615 jmp 0xc763c
            pc = 0xc763c; continue;
        }
        0xc7617 => {
            // block c7617
            // c7617 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7619 jne 0xc763c
            if fl.ne() { pc = 0xc763c; continue; }
            pc = 0xc761b;
        }
        0xc761b => {
            // block c761b
            // c761b lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7620 lea r15,[rdi+r12*8+0x4]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c7625 jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7627 => {
            // block c7627
            // c7627 cmp r8d,0x4
            fl = sub::<32>((r8 as u32 as u64), 0x4u64).1;
            // c762b jne 0xc7642
            if fl.ne() { pc = 0xc7642; continue; }
            pc = 0xc762d;
        }
        0xc762d => {
            // block c762d
            // c762d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c762f jne 0xc7650
            if fl.ne() { pc = 0xc7650; continue; }
            pc = 0xc7631;
        }
        0xc7631 => {
            // block c7631
            // c7631 lea r14,[rdi+r12*8+0x2]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c7636 lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c763a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc763c => {
            // block c763c
            // c763c lea r14,[rdi+r12*8]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7640 jmp 0xc7655
            pc = 0xc7655; continue;
        }
        0xc7642 => {
            // block c7642
            // c7642 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // c7644 jne 0xc765c
            if fl.ne() { pc = 0xc765c; continue; }
            pc = 0xc7646;
        }
        0xc7646 => {
            // block c7646
            // c7646 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            // c764b mov r15,r9
            r15 = r9;
            // c764e jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc7650 => {
            // block c7650
            // c7650 lea r14,[rdi+r12*8+0x4]
            r14 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4);
            pc = 0xc7655;
        }
        0xc7655 => {
            // block c7655
            // c7655 lea r15,[rdi+r12*8+0x2]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2);
            // c765a jmp 0xc7663
            pc = 0xc7663; continue;
        }
        0xc765c => {
            // block c765c
            // c765c lea r15,[rdi+r12*8]
            r15 = rdi.wrapping_add(r12.wrapping_mul(8));
            // c7660 mov r14,r9
            r14 = r9;
            pc = 0xc7663;
        }
        0xc7663 => {
            // block c7663
            // c7663 mov r13d,r12d
            r13 = ((r12 as u32 as u64)) as u32 as u64;
            // c7666 imul r13,r10
            { let (r, f) = imul::<64>(r13, r10); r13 = r; fl = f; }
            // c766a mov rax,r13
            rax = r13;
            // c766d shr rax,0x22
            { let (r, f) = shr::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            // c7671 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7673 lea ebx,[rax+rax*2]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // c7676 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // c7679 sub eax,ebx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c767b cmp eax,0x5
            fl = sub::<32>((rax as u32 as u64), 0x5u64).1;
            // c767e ja 0xc76f7
            if fl.a() { pc = 0xc76f7; continue; }
            pc = 0xc7680;
        }
        0xc7680 => {
            // block c7680
            // c7680 movzx r14d,WORD PTR [r14]
            r14 = (m.r16(r14) as u64) as u32 as u64;
            // c7684 movzx r15d,WORD PTR [r15]
            r15 = (m.r16(r15) as u64) as u32 as u64;
            // c7688 shr r13,0x21
            { let (r, f) = shr::<64>(r13, 0x21u64, fl); r13 = r; fl = f; }
            // c768c movsxd rax,DWORD PTR [r11+rax*4]
            rax = sx::<32>(m.r32(r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // c7690 add rax,r11
            { let (r, f) = add::<64>(rax, r11);
              rax = r;
              fl = f; }
            // c7693 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7695 => {
            // block c7695
            // c7695 shr r14d,0x4
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x4u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7699 and r14d,0xffc
            { let r = (r14 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76a0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76a3 mov DWORD PTR [rsi+rax*4],r14d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r14 as u32 as u64)) as u32);
            // c76a7 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76a9 => {
            // block c76a9
            // c76a9 shl r15d,0x6
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x6u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76ad and r15d,0x3ff000
            { let r = (r15 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76b4 jmp 0xc76f0
            pc = 0xc76f0; continue;
        }
        0xc76b6 => {
            // block c76b6
            // c76b6 and r14d,0xffffffc0
            { let r = (r14 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c76ba shl r14d,0x10
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76be jmp 0xc76df
            pc = 0xc76df; continue;
        }
        0xc76c0 => {
            // block c76c0
            // c76c0 shr r15d,0x4
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0x4u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c76c4 and r15d,0xffc
            { let r = (r15 as u32 as u64) & 0xffcu64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76cb mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76ce mov DWORD PTR [rsi+rax*4],r15d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r15 as u32 as u64)) as u32);
            // c76d2 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76d4 => {
            // block c76d4
            // c76d4 shl r14d,0x6
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x6u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c76d8 and r14d,0x3ff000
            { let r = (r14 as u32 as u64) & 0x3ff000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc76df;
        }
        0xc76df => {
            // block c76df
            // c76df mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76e2 or DWORD PTR [rsi+rax*4],r14d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r14 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            // c76e6 jmp 0xc76f7
            pc = 0xc76f7; continue;
        }
        0xc76e8 => {
            // block c76e8
            // c76e8 and r15d,0xffffffc0
            { let r = (r15 as u32 as u64) & 0xffffffc0u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // c76ec shl r15d,0x10
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            pc = 0xc76f0;
        }
        0xc76f0 => {
            // block c76f0
            // c76f0 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // c76f3 or DWORD PTR [rsi+rax*4],r15d
            { let r = m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64 | (r15 as u32 as u64); let f = logic::<32>(r);
              m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), (r) as u32);
              fl = f; }
            pc = 0xc76f7;
        }
        0xc76f7 => {
            // block c76f7
            // c76f7 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // c76fa add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c76fe cmp edx,r12d
            fl = sub::<32>((rdx as u32 as u64), (r12 as u32 as u64)).1;
            // c7701 jne 0xc7605
            if fl.ne() { pc = 0xc7605; continue; }
            pc = 0xc7707;
        }
        0xc7707 => {
            // block c7707
            // c7707 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7708 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c770a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c770c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c770e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7710 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7711 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5ba7b size=359
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005ba7b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5ba7b;
    loop {
        match pc {
        0x5ba7b => {
            // block 5ba7b
            // 5ba7b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5ba7c mov rbp,rsp
            rbp = rsp;
            // 5ba7f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5ba81 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5ba83 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5ba85 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5ba87 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5ba88 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 5ba8a je 0x5ba99
            if fl.e() { pc = 0x5ba99; continue; }
            pc = 0x5ba8c;
        }
        0x5ba8c => {
            // block 5ba8c
            // 5ba8c mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 5ba91 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ba93 add eax,r8d
            { let (r, f) = add::<32>((rax as u32 as u64), (r8 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ba96 mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x5ba99;
        }
        0x5ba99 => {
            // block 5ba99
            // 5ba99 mov dl,BYTE PTR [rbp+0x10]
            rdx = (rdx & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // 5ba9c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 5ba9f imul rax,rcx,0x2aaaaaab
            { let (r, f) = imul::<64>(rcx, 0x2aaaaaabu64); rax = r; fl = f; }
            // 5baa6 mov rbx,rax
            rbx = rax;
            // 5baa9 shr rbx,0x3f
            { let (r, f) = shr::<64>(rbx, 0x3fu64, fl); rbx = r; fl = f; }
            // 5baad sar rax,0x22
            { let (r, f) = sar::<64>(rax, 0x22u64, fl); rax = r; fl = f; }
            // 5bab1 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5bab3 lea ebx,[rax*8+0x0]
            rbx = (rax.wrapping_mul(8)) as u32 as u64;
            // 5baba lea ebx,[rbx+rbx*2]
            rbx = (rbx.wrapping_add(rbx.wrapping_mul(2))) as u32 as u64;
            // 5babd sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5babf mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 5bac1 sar ebx,0x1f
            { let (r, f) = sar::<32>((rbx as u32 as u64), 0x1fu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 5bac4 shr ebx,0x1e
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x1eu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 5bac7 add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 5bac9 sar ebx,0x2
            { let (r, f) = sar::<32>((rbx as u32 as u64), 0x2u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 5bacc cmp ebx,r8d
            fl = sub::<32>((rbx as u32 as u64), (r8 as u32 as u64)).1;
            // 5bacf jle 0x5bad5
            if fl.le() { pc = 0x5bad5; continue; }
            pc = 0x5bad1;
        }
        0x5bad1 => {
            // block 5bad1
            // 5bad1 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5bad3 jmp 0x5bafa
            pc = 0x5bafa; continue;
        }
        0x5bad5 => {
            // block 5bad5
            // 5bad5 cmp r8d,0x6
            fl = sub::<32>((r8 as u32 as u64), 0x6u64).1;
            // 5bad9 jl 0x5baeb
            if fl.l() { pc = 0x5baeb; continue; }
            pc = 0x5badb;
        }
        0x5badb => {
            // block 5badb
            // 5badb add ecx,0x3
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x3u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5bade cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // 5bae1 jb 0x5baeb
            if fl.b() { pc = 0x5baeb; continue; }
            pc = 0x5bae3;
        }
        0x5bae3 => {
            // block 5bae3
            // 5bae3 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5bae5 add r8d,0xfffffffa
            { let (r, f) = add::<32>((r8 as u32 as u64), 0xfffffffau64);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5bae9 jmp 0x5bafa
            pc = 0x5bafa; continue;
        }
        0x5baeb => {
            // block 5baeb
            // 5baeb lea ecx,[r8-0x6]
            rcx = (r8.wrapping_add(0xffff_ffff_ffff_fffa)) as u32 as u64;
            // 5baef cmp r8d,0x5
            fl = sub::<32>((r8 as u32 as u64), 0x5u64).1;
            // 5baf3 cmovle ecx,r8d
            if fl.le() { rcx = ((r8 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 5baf7 mov r8d,ecx
            r8 = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x5bafa;
        }
        0x5bafa => {
            // block 5bafa
            // 5bafa lea r15,[rdi+0x18]
            r15 = rdi.wrapping_add(0x18);
            // 5bafe test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5bb00 je 0x5bb6f
            if fl.e() { pc = 0x5bb6f; continue; }
            pc = 0x5bb02;
        }
        0x5bb02 => {
            // block 5bb02
            // 5bb02 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5bb04 jle 0x5bbd7
            if fl.le() { pc = 0x5bbd7; continue; }
            pc = 0x5bb0a;
        }
        0x5bb0a => {
            // block 5bb0a
            // 5bb0a lea ecx,[r8+r9*1]
            rcx = (r8.wrapping_add(r9)) as u32 as u64;
            // 5bb0e movsxd r10,r8d
            r10 = sx::<32>((r8 as u32 as u64)) as u64;
            // 5bb11 movsxd r12,ecx
            r12 = sx::<32>((rcx as u32 as u64)) as u64;
            // 5bb14 lea ecx,[r8+0x1]
            rcx = (r8.wrapping_add(0x1)) as u32 as u64;
            // 5bb18 cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // 5bb1b mov r11d,r12d
            r11 = ((r12 as u32 as u64)) as u32 as u64;
            // 5bb1e cmovl r11d,ecx
            if fl.l() { r11 = ((rcx as u32 as u64)) as u32 as u64; } else { r11 = r11 as u32 as u64; }
            // 5bb22 dec r11d
            { let (r, f) = dec::<32>((r11 as u32 as u64), fl); r11 = (r) as u32 as u64; fl = f; }
            // 5bb25 sub r11d,r8d
            { let (r, f) = sub::<32>((r11 as u32 as u64), (r8 as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // 5bb28 inc r11
            { let (r, f) = inc::<64>(r11, fl); r11 = r; fl = f; }
            // 5bb2b lea r8d,[rax-0x1]
            r8 = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bb2f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bb32;
        }
        0x5bb32 => {
            // block 5bb32
            // 5bb32 mov r13,r15
            r13 = r15;
            // 5bb35 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // 5bb38 mov rbx,r10
            rbx = r10;
            // 5bb3b mov rdx,rsi
            rdx = rsi;
            // 5bb3e jle 0x5bb57
            if fl.le() { pc = 0x5bb57; continue; }
            pc = 0x5bb40;
        }
        0x5bb40 => {
            // block 5bb40
            // 5bb40 mov ecx,DWORD PTR [r13+rbx*4+0x0]
            rcx = (m.r32(r13.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 5bb45 mov DWORD PTR [rdx],ecx
            m.w32(rdx, ((rcx as u32 as u64)) as u32);
            // 5bb47 add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // 5bb4b inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 5bb4e cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 5bb51 jl 0x5bb40
            if fl.l() { pc = 0x5bb40; continue; }
            pc = 0x5bb53;
        }
        0x5bb53 => {
            // block 5bb53
            // 5bb53 lea rsi,[rsi+r11*4]
            rsi = rsi.wrapping_add(r11.wrapping_mul(4));
            pc = 0x5bb57;
        }
        0x5bb57 => {
            // block 5bb57
            // 5bb57 add rdi,0x30
            { let (r, f) = add::<64>(rdi, 0x30u64);
              rdi = r;
              fl = f; }
            // 5bb5b cmp r14d,r8d
            fl = sub::<32>((r14 as u32 as u64), (r8 as u32 as u64)).1;
            // 5bb5e lea ecx,[r14+0x1]
            rcx = (r14.wrapping_add(0x1)) as u32 as u64;
            // 5bb62 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 5bb65 mov r15,rdi
            r15 = rdi;
            // 5bb68 mov rdi,r13
            rdi = r13;
            // 5bb6b jne 0x5bb32
            if fl.ne() { pc = 0x5bb32; continue; }
            pc = 0x5bb6d;
        }
        0x5bb6d => {
            // block 5bb6d
            // 5bb6d jmp 0x5bbd7
            pc = 0x5bbd7; continue;
        }
        0x5bb6f => {
            // block 5bb6f
            // 5bb6f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 5bb71 jle 0x5bbd7
            if fl.le() { pc = 0x5bbd7; continue; }
            pc = 0x5bb73;
        }
        0x5bb73 => {
            // block 5bb73
            // 5bb73 movsxd r10,r8d
            r10 = sx::<32>((r8 as u32 as u64)) as u64;
            // 5bb76 add r8d,r9d
            { let (r, f) = add::<32>((r8 as u32 as u64), (r9 as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5bb79 movsxd r12,r8d
            r12 = sx::<32>((r8 as u32 as u64)) as u64;
            // 5bb7c lea ecx,[r10+0x1]
            rcx = (r10.wrapping_add(0x1)) as u32 as u64;
            // 5bb80 cmp r12d,ecx
            fl = sub::<32>((r12 as u32 as u64), (rcx as u32 as u64)).1;
            // 5bb83 mov r8d,r12d
            r8 = ((r12 as u32 as u64)) as u32 as u64;
            // 5bb86 cmovl r8d,ecx
            if fl.l() { r8 = ((rcx as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 5bb8a dec r8d
            { let (r, f) = dec::<32>((r8 as u32 as u64), fl); r8 = (r) as u32 as u64; fl = f; }
            // 5bb8d sub r8d,r10d
            { let (r, f) = sub::<32>((r8 as u32 as u64), (r10 as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5bb90 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // 5bb93 lea r11d,[rax-0x1]
            r11 = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bb97 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bb9a;
        }
        0x5bb9a => {
            // block 5bb9a
            // 5bb9a mov r13,r15
            r13 = r15;
            // 5bb9d test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // 5bba0 mov rcx,r10
            rcx = r10;
            // 5bba3 mov rdx,rsi
            rdx = rsi;
            // 5bba6 jle 0x5bbc1
            if fl.le() { pc = 0x5bbc1; continue; }
            pc = 0x5bba8;
        }
        0x5bba8 => {
            // block 5bba8
            // 5bba8 mov bx,WORD PTR [r13+rcx*4+0x2]
            rbx = (rbx & !0xffff) | ((m.r16(r13.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u16 as u64);
            // 5bbae mov WORD PTR [rdx],bx
            m.w16(rdx, ((rbx as u16 as u64)) as u16);
            // 5bbb1 add rdx,0x2
            { let (r, f) = add::<64>(rdx, 0x2u64);
              rdx = r;
              fl = f; }
            // 5bbb5 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 5bbb8 cmp rcx,r12
            fl = sub::<64>(rcx, r12).1;
            // 5bbbb jl 0x5bba8
            if fl.l() { pc = 0x5bba8; continue; }
            pc = 0x5bbbd;
        }
        0x5bbbd => {
            // block 5bbbd
            // 5bbbd lea rsi,[rsi+r8*2]
            rsi = rsi.wrapping_add(r8.wrapping_mul(2));
            pc = 0x5bbc1;
        }
        0x5bbc1 => {
            // block 5bbc1
            // 5bbc1 add rdi,0x30
            { let (r, f) = add::<64>(rdi, 0x30u64);
              rdi = r;
              fl = f; }
            // 5bbc5 cmp r14d,r11d
            fl = sub::<32>((r14 as u32 as u64), (r11 as u32 as u64)).1;
            // 5bbc8 lea ecx,[r14+0x1]
            rcx = (r14.wrapping_add(0x1)) as u32 as u64;
            // 5bbcc mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 5bbcf mov r15,rdi
            r15 = rdi;
            // 5bbd2 mov rdi,r13
            rdi = r13;
            // 5bbd5 jne 0x5bb9a
            if fl.ne() { pc = 0x5bb9a; continue; }
            pc = 0x5bbd7;
        }
        0x5bbd7 => {
            // block 5bbd7
            // 5bbd7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5bbd8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5bbda pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5bbdc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5bbde pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5bbe0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5bbe1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc77be size=442
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c77be<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc77be;
    loop {
        match pc {
        0xc77be => {
            // block c77be
            // c77be push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c77bf mov rbp,rsp
            rbp = rsp;
            // c77c2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c77c4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c77c6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c77c8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c77c9 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // c77cc mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // c77cf test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // c77d2 je 0xc7952
            if fl.e() { pc = 0xc7952; continue; }
            pc = 0xc77d8;
        }
        0xc77d8 => {
            // block c77d8
            // c77d8 and r14d,0x1
            { let r = (r14 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // c77dc dec r15d
            { let (r, f) = dec::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // c77df lea r9,[rdi+0x2]
            r9 = rdi.wrapping_add(0x2);
            // c77e3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c77e5 lea r10,[rip+0x170] # 0xc795c
            r10 = m.env.code_va(0xc795c);
            // c77ec xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc77ef;
        }
        0xc77ef => {
            // block c77ef
            // c77ef cmp r8d,0x2
            fl = sub::<32>((r8 as u32 as u64), 0x2u64).1;
            // c77f3 je 0xc7802
            if fl.e() { pc = 0xc7802; continue; }
            pc = 0xc77f5;
        }
        0xc77f5 => {
            // block c77f5
            // c77f5 cmp r8d,0x3
            fl = sub::<32>((r8 as u32 as u64), 0x3u64).1;
            // c77f9 jne 0xc7813
            if fl.ne() { pc = 0xc7813; continue; }
            pc = 0xc77fb;
        }
        0xc77fb => {
            // block c77fb
            // c77fb test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c77fe jne 0xc7807
            if fl.ne() { pc = 0xc7807; continue; }
            pc = 0xc7800;
        }
        0xc7800 => {
            // block c7800
            // c7800 jmp 0xc7829
            pc = 0xc7829; continue;
        }
        0xc7802 => {
            // block c7802
            // c7802 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7805 jne 0xc7829
            if fl.ne() { pc = 0xc7829; continue; }
            pc = 0xc7807;
        }
        0xc7807 => {
            // block c7807
            // c7807 lea rcx,[rdi+rax*8+0x2]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c780c lea rdx,[rdi+rax*8+0x4]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            // c7811 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc7813 => {
            // block c7813
            // c7813 cmp r8d,0x4
            fl = sub::<32>((r8 as u32 as u64), 0x4u64).1;
            // c7817 jne 0xc782f
            if fl.ne() { pc = 0xc782f; continue; }
            pc = 0xc7819;
        }
        0xc7819 => {
            // block c7819
            // c7819 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c781c jne 0xc783e
            if fl.ne() { pc = 0xc783e; continue; }
            pc = 0xc781e;
        }
        0xc781e => {
            // block c781e
            // c781e lea rcx,[rdi+rax*8+0x2]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c7823 lea rdx,[rdi+rax*8]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c7827 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc7829 => {
            // block c7829
            // c7829 lea rcx,[rdi+rax*8]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c782d jmp 0xc7843
            pc = 0xc7843; continue;
        }
        0xc782f => {
            // block c782f
            // c782f test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7832 jne 0xc784a
            if fl.ne() { pc = 0xc784a; continue; }
            pc = 0xc7834;
        }
        0xc7834 => {
            // block c7834
            // c7834 lea rcx,[rdi+rax*8+0x4]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            // c7839 mov rdx,r9
            rdx = r9;
            // c783c jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc783e => {
            // block c783e
            // c783e lea rcx,[rdi+rax*8+0x4]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            pc = 0xc7843;
        }
        0xc7843 => {
            // block c7843
            // c7843 lea rdx,[rdi+rax*8+0x2]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c7848 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc784a => {
            // block c784a
            // c784a lea rdx,[rdi+rax*8]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c784e mov rcx,r9
            rcx = r9;
            pc = 0xc7851;
        }
        0xc7851 => {
            // block c7851
            // c7851 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c7854 cmp r11d,0x6
            fl = sub::<32>((r11 as u32 as u64), 0x6u64).1;
            // c7858 ja 0xc7940
            if fl.a() { pc = 0xc7940; continue; }
            pc = 0xc785e;
        }
        0xc785e => {
            // block c785e
            // c785e movzx edx,WORD PTR [rdx]
            rdx = (m.r16(rdx) as u64) as u32 as u64;
            // c7861 movzx r12d,WORD PTR [rcx]
            r12 = (m.r16(rcx) as u64) as u32 as u64;
            // c7865 movsxd rbx,DWORD PTR [r10+rbx*4]
            rbx = sx::<32>(m.r32(r10.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
            // c7869 add rbx,r10
            { let (r, f) = add::<64>(rbx, r10);
              rbx = r;
              fl = f; }
            // c786c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c786e jmp rbx
            { let r = { let t = rbx; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0xc7870 => {
            // block c7870
            // c7870 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c7872 jne 0xc78ca
            if fl.ne() { pc = 0xc78ca; continue; }
            pc = 0xc7874;
        }
        0xc7874 => {
            // block c7874
            // c7874 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // c7877 and ecx,0xc000
            { let r = (rcx as u32 as u64) & 0xc000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c787d shr r12d,0x6
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x6u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // c7881 mov BYTE PTR [rsi],r12b
            m.w8(rsi, ((r12 as u8 as u64)) as u8);
            // c7884 jmp 0xc78d7
            pc = 0xc78d7; continue;
        }
        0xc7886 => {
            // block c7886
            // c7886 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c7888 jne 0xc78e9
            if fl.ne() { pc = 0xc78e9; continue; }
            pc = 0xc788a;
        }
        0xc788a => {
            // block c788a
            // c788a mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // c788d and ecx,0xf000
            { let r = (rcx as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7893 shr r12d,0x4
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x4u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // c7897 and r12b,0xfc
            { let r = (r12 as u8 as u64) & 0xfcu64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c789b or BYTE PTR [rsi],r12b
            { let r = m.r8(rsi) as u64 | (r12 as u8 as u64); let f = logic::<8>(r);
              m.w8(rsi, (r) as u8);
              fl = f; }
            // c789e jmp 0xc78f9
            pc = 0xc78f9; continue;
        }
        0xc78a0 => {
            // block c78a0
            // c78a0 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c78a2 jne 0xc790b
            if fl.ne() { pc = 0xc790b; continue; }
            pc = 0xc78a4;
        }
        0xc78a4 => {
            // block c78a4
            // c78a4 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // c78a7 and ecx,0xfc00
            { let r = (rcx as u32 as u64) & 0xfc00u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c78ad shr r12d,0x2
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x2u64, fl); r12 = (r) as u32 as u64; fl = f; }
            // c78b1 and r12b,0xf0
            { let r = (r12 as u8 as u64) & 0xf0u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c78b5 or BYTE PTR [rsi],r12b
            { let r = m.r8(rsi) as u64 | (r12 as u8 as u64); let f = logic::<8>(r);
              m.w8(rsi, (r) as u8);
              fl = f; }
            // c78b8 jmp 0xc791b
            pc = 0xc791b; continue;
        }
        0xc78ba => {
            // block c78ba
            // c78ba test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c78bc jne 0xc792d
            if fl.ne() { pc = 0xc792d; continue; }
            pc = 0xc78be;
        }
        0xc78be => {
            // block c78be
            // c78be mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // c78c1 mov cl,dl
            rcx = (rcx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // c78c3 and cl,0xc0
            { let r = (rcx as u8 as u64) & 0xc0u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c78c6 or cl,BYTE PTR [rsi]
            { let r = (rcx as u8 as u64) | m.r8(rsi) as u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c78c8 jmp 0xc7934
            pc = 0xc7934; continue;
        }
        0xc78ca => {
            // block c78ca
            // c78ca mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c78cc and ecx,0xc000
            { let r = (rcx as u32 as u64) & 0xc000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c78d2 shr edx,0x6
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c78d5 mov BYTE PTR [rsi],dl
            m.w8(rsi, ((rdx as u8 as u64)) as u8);
            pc = 0xc78d7;
        }
        0xc78d7 => {
            // block c78d7
            // c78d7 shr ecx,0xe
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xeu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c78da mov BYTE PTR [rsi+0x1],cl
            m.w8(rsi.wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // c78dd lea rsi,[rsi+0x1]
            rsi = rsi.wrapping_add(0x1);
            // c78e1 mov r11d,0x2
            r11 = (0x2u64) as u32 as u64;
            // c78e7 jmp 0xc7940
            pc = 0xc7940; continue;
        }
        0xc78e9 => {
            // block c78e9
            // c78e9 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c78eb and ecx,0xf000
            { let r = (rcx as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c78f1 shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c78f4 and dl,0xfc
            { let r = (rdx as u8 as u64) & 0xfcu64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c78f7 or BYTE PTR [rsi],dl
            { let r = m.r8(rsi) as u64 | (rdx as u8 as u64); let f = logic::<8>(r);
              m.w8(rsi, (r) as u8);
              fl = f; }
            pc = 0xc78f9;
        }
        0xc78f9 => {
            // block c78f9
            // c78f9 shr ecx,0xc
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c78fc mov BYTE PTR [rsi+0x1],cl
            m.w8(rsi.wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // c78ff lea rsi,[rsi+0x1]
            rsi = rsi.wrapping_add(0x1);
            // c7903 mov r11d,0x4
            r11 = (0x4u64) as u32 as u64;
            // c7909 jmp 0xc7940
            pc = 0xc7940; continue;
        }
        0xc790b => {
            // block c790b
            // c790b mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c790d and ecx,0xfc00
            { let r = (rcx as u32 as u64) & 0xfc00u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7913 shr edx,0x2
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c7916 and dl,0xf0
            { let r = (rdx as u8 as u64) & 0xf0u64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c7919 or BYTE PTR [rsi],dl
            { let r = m.r8(rsi) as u64 | (rdx as u8 as u64); let f = logic::<8>(r);
              m.w8(rsi, (r) as u8);
              fl = f; }
            pc = 0xc791b;
        }
        0xc791b => {
            // block c791b
            // c791b shr ecx,0xa
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0xau64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c791e mov BYTE PTR [rsi+0x1],cl
            m.w8(rsi.wrapping_add(0x1), ((rcx as u8 as u64)) as u8);
            // c7921 lea rsi,[rsi+0x1]
            rsi = rsi.wrapping_add(0x1);
            // c7925 mov r11d,0x6
            r11 = (0x6u64) as u32 as u64;
            // c792b jmp 0xc7940
            pc = 0xc7940; continue;
        }
        0xc792d => {
            // block c792d
            // c792d mov cl,dl
            rcx = (rcx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // c792f and cl,0xc0
            { let r = (rcx as u8 as u64) & 0xc0u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c7932 or cl,BYTE PTR [rsi]
            { let r = (rcx as u8 as u64) | m.r8(rsi) as u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            pc = 0xc7934;
        }
        0xc7934 => {
            // block c7934
            // c7934 mov BYTE PTR [rsi],cl
            m.w8(rsi, ((rcx as u8 as u64)) as u8);
            // c7936 mov BYTE PTR [rsi+0x1],dh
            m.w8(rsi.wrapping_add(0x1), (((rdx >> 8) as u8 as u64)) as u8);
            // c7939 add rsi,0x2
            { let (r, f) = add::<64>(rsi, 0x2u64);
              rsi = r;
              fl = f; }
            // c793d xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7940;
        }
        0xc7940 => {
            // block c7940
            // c7940 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xc7942;
        }
        0xc7942 => {
            // block c7942
            // c7942 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // c7945 add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c7949 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // c794c jne 0xc77ef
            if fl.ne() { pc = 0xc77ef; continue; }
            pc = 0xc7952;
        }
        0xc7952 => {
            // block c7952
            // c7952 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7953 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c7955 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7957 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7959 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c795a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc5ddf size=572
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c5ddf<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc5ddf;
    loop {
        match pc {
        0xc5ddf => {
            // block c5ddf
            // c5ddf push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c5de0 mov rbp,rsp
            rbp = rsp;
            // c5de3 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5de5 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5de6 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // c5de8 je 0xc6016
            if fl.e() { pc = 0xc6016; continue; }
            pc = 0xc5dee;
        }
        0xc5dee => {
            // block c5dee
            // c5dee xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c5df1 mov r9d,0x5
            r9 = (0x5u64) as u32 as u64;
            pc = 0xc5df7;
        }
        0xc5df7 => {
            // block c5df7
            // c5df7 lea eax,[r9-0x5]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // c5dfb movzx r10d,WORD PTR [rdi+rax*2]
            r10 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e00 shl r10d,0x14
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x14u64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5e04 lea eax,[r9-0x4]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_fffc)) as u32 as u64;
            // c5e08 movzx r11d,WORD PTR [rdi+rax*2]
            r11 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e0d shl r11d,0xa
            { let (r, f) = shl::<32>((r11 as u32 as u64), 0xau64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5e11 add r11d,r10d
            { let (r, f) = add::<32>((r11 as u32 as u64), (r10 as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5e14 lea eax,[r9-0x3]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_fffd)) as u32 as u64;
            // c5e18 movzx r10d,WORD PTR [rdi+rax*2]
            r10 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e1d add r10d,r11d
            { let (r, f) = add::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c5e20 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c5e22 je 0xc5f70
            if fl.e() { pc = 0xc5f70; continue; }
            pc = 0xc5e28;
        }
        0xc5e28 => {
            // block c5e28
            // c5e28 mov r11d,r10d
            r11 = ((r10 as u32 as u64)) as u32 as u64;
            // c5e2b shl r11d,0x1a
            { let (r, f) = shl::<32>((r11 as u32 as u64), 0x1au64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5e2f mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // c5e32 shl eax,0xa
            { let (r, f) = shl::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5e35 and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5e3a or eax,r11d
            { let r = (rax as u32 as u64) | (r11 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5e3d mov r11d,r10d
            r11 = ((r10 as u32 as u64)) as u32 as u64;
            // c5e40 shr r11d,0x6
            { let (r, f) = shr::<32>((r11 as u32 as u64), 0x6u64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5e44 and r11d,0xff00
            { let r = (r11 as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5e4b or r11d,eax
            { let r = (r11 as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5e4e shr r10d,0x16
            { let (r, f) = shr::<32>((r10 as u32 as u64), 0x16u64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5e52 movzx r10d,r10b
            r10 = ((r10 as u8 as u64)) as u32 as u64;
            // c5e56 or r10d,r11d
            { let r = (r10 as u32 as u64) | (r11 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // c5e59 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c5e5c mov DWORD PTR [rsi+rax*4],r10d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r10 as u32 as u64)) as u32);
            // c5e60 lea eax,[r9-0x2]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // c5e64 movzx r10d,WORD PTR [rdi+rax*2]
            r10 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e69 shl r10d,0x14
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x14u64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5e6d lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c5e71 movzx r11d,WORD PTR [rdi+rax*2]
            r11 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e76 shl r11d,0xa
            { let (r, f) = shl::<32>((r11 as u32 as u64), 0xau64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5e7a add r11d,r10d
            { let (r, f) = add::<32>((r11 as u32 as u64), (r10 as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5e7d mov eax,r9d
            rax = ((r9 as u32 as u64)) as u32 as u64;
            // c5e80 movzx r14d,WORD PTR [rdi+rax*2]
            r14 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5e85 add r14d,r11d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r11 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // c5e88 mov r10d,r14d
            r10 = ((r14 as u32 as u64)) as u32 as u64;
            // c5e8b shl r10d,0x1a
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x1au64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5e8f mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // c5e92 shl eax,0xa
            { let (r, f) = shl::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5e95 and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5e9a or eax,r10d
            { let r = (rax as u32 as u64) | (r10 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5e9d mov ebx,r14d
            rbx = ((r14 as u32 as u64)) as u32 as u64;
            // c5ea0 shr ebx,0x6
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5ea3 and ebx,0xff00
            { let r = (rbx as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5ea9 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5eab shr r14d,0x16
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x16u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c5eaf movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // c5eb3 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5eb5 lea ebx,[r8+0x1]
            rbx = (r8.wrapping_add(0x1)) as u32 as u64;
            // c5eb9 mov DWORD PTR [rsi+rbx*4],eax
            m.w32(rsi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c5ebc lea eax,[r9+0x1]
            rax = (r9.wrapping_add(0x1)) as u32 as u64;
            // c5ec0 movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5ec4 shl eax,0x14
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5ec7 lea ebx,[r9+0x2]
            rbx = (r9.wrapping_add(0x2)) as u32 as u64;
            // c5ecb movzx r10d,WORD PTR [rdi+rbx*2]
            r10 = (m.r16(rdi.wrapping_add(rbx.wrapping_mul(2))) as u64) as u32 as u64;
            // c5ed0 shl r10d,0xa
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0xau64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5ed4 add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c5ed7 lea eax,[r9+0x3]
            rax = (r9.wrapping_add(0x3)) as u32 as u64;
            // c5edb movzx r11d,WORD PTR [rdi+rax*2]
            r11 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5ee0 add r11d,r10d
            { let (r, f) = add::<32>((r11 as u32 as u64), (r10 as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5ee3 mov r10d,r11d
            r10 = ((r11 as u32 as u64)) as u32 as u64;
            // c5ee6 shl r10d,0x1a
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x1au64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5eea mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // c5eed shl eax,0xa
            { let (r, f) = shl::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5ef0 and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5ef5 or eax,r10d
            { let r = (rax as u32 as u64) | (r10 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5ef8 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c5efb shr ebx,0x6
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5efe and ebx,0xff00
            { let r = (rbx as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5f04 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5f06 shr r11d,0x16
            { let (r, f) = shr::<32>((r11 as u32 as u64), 0x16u64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5f0a movzx eax,r11b
            rax = ((r11 as u8 as u64)) as u32 as u64;
            // c5f0e or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5f10 lea ebx,[r8+0x2]
            rbx = (r8.wrapping_add(0x2)) as u32 as u64;
            // c5f14 mov DWORD PTR [rsi+rbx*4],eax
            m.w32(rsi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c5f17 lea eax,[r9+0x4]
            rax = (r9.wrapping_add(0x4)) as u32 as u64;
            // c5f1b movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f1f shl eax,0x14
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5f22 lea ebx,[r9+0x5]
            rbx = (r9.wrapping_add(0x5)) as u32 as u64;
            // c5f26 movzx r10d,WORD PTR [rdi+rbx*2]
            r10 = (m.r16(rdi.wrapping_add(rbx.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f2b shl r10d,0xa
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0xau64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5f2f add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // c5f32 lea eax,[r9+0x6]
            rax = (r9.wrapping_add(0x6)) as u32 as u64;
            // c5f36 movzx r11d,WORD PTR [rdi+rax*2]
            r11 = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f3b add r11d,r10d
            { let (r, f) = add::<32>((r11 as u32 as u64), (r10 as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c5f3e mov r10d,r11d
            r10 = ((r11 as u32 as u64)) as u32 as u64;
            // c5f41 shl r10d,0x1a
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x1au64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5f45 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // c5f48 shl eax,0xa
            { let (r, f) = shl::<32>((rax as u32 as u64), 0xau64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5f4b and eax,0xff0000
            { let r = (rax as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5f50 or eax,r10d
            { let r = (rax as u32 as u64) | (r10 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5f53 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c5f56 shr ebx,0x6
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5f59 and ebx,0xff00
            { let r = (rbx as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5f5f or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5f61 shr r11d,0x16
            { let (r, f) = shr::<32>((r11 as u32 as u64), 0x16u64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c5f65 movzx eax,r11b
            rax = ((r11 as u8 as u64)) as u32 as u64;
            // c5f69 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c5f6b jmp 0xc5ff7
            pc = 0xc5ff7; continue;
        }
        0xc5f70 => {
            // block c5f70
            // c5f70 shl r10d,0x2
            { let (r, f) = shl::<32>((r10 as u32 as u64), 0x2u64, fl); r10 = (r) as u32 as u64; fl = f; }
            // c5f74 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // c5f77 mov DWORD PTR [rsi+rax*4],r10d
            m.w32(rsi.wrapping_add(rax.wrapping_mul(4)), ((r10 as u32 as u64)) as u32);
            // c5f7b lea eax,[r9-0x2]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // c5f7f movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f83 shl eax,0x14
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5f86 lea ebx,[r9-0x1]
            rbx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c5f8a movzx ebx,WORD PTR [rdi+rbx*2]
            rbx = (m.r16(rdi.wrapping_add(rbx.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f8e shl ebx,0xa
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0xau64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5f91 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5f93 mov eax,r9d
            rax = ((r9 as u32 as u64)) as u32 as u64;
            // c5f96 movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5f9a add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c5f9c shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5f9f lea ebx,[r8+0x1]
            rbx = (r8.wrapping_add(0x1)) as u32 as u64;
            // c5fa3 mov DWORD PTR [rsi+rbx*4],eax
            m.w32(rsi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c5fa6 lea eax,[r9+0x1]
            rax = (r9.wrapping_add(0x1)) as u32 as u64;
            // c5faa movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5fae shl eax,0x14
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5fb1 lea ebx,[r9+0x2]
            rbx = (r9.wrapping_add(0x2)) as u32 as u64;
            // c5fb5 movzx ebx,WORD PTR [rdi+rbx*2]
            rbx = (m.r16(rdi.wrapping_add(rbx.wrapping_mul(2))) as u64) as u32 as u64;
            // c5fb9 shl ebx,0xa
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0xau64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5fbc add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5fbe lea eax,[r9+0x3]
            rax = (r9.wrapping_add(0x3)) as u32 as u64;
            // c5fc2 movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5fc6 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c5fc8 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5fcb lea ebx,[r8+0x2]
            rbx = (r8.wrapping_add(0x2)) as u32 as u64;
            // c5fcf mov DWORD PTR [rsi+rbx*4],eax
            m.w32(rsi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c5fd2 lea eax,[r9+0x4]
            rax = (r9.wrapping_add(0x4)) as u32 as u64;
            // c5fd6 movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5fda shl eax,0x14
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c5fdd lea ebx,[r9+0x5]
            rbx = (r9.wrapping_add(0x5)) as u32 as u64;
            // c5fe1 movzx ebx,WORD PTR [rdi+rbx*2]
            rbx = (m.r16(rdi.wrapping_add(rbx.wrapping_mul(2))) as u64) as u32 as u64;
            // c5fe5 shl ebx,0xa
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0xau64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c5fe8 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c5fea lea eax,[r9+0x6]
            rax = (r9.wrapping_add(0x6)) as u32 as u64;
            // c5fee movzx eax,WORD PTR [rdi+rax*2]
            rax = (m.r16(rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u32 as u64;
            // c5ff2 add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c5ff4 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc5ff7;
        }
        0xc5ff7 => {
            // block c5ff7
            // c5ff7 lea ebx,[r8+0x3]
            rbx = (r8.wrapping_add(0x3)) as u32 as u64;
            // c5ffb mov DWORD PTR [rsi+rbx*4],eax
            m.w32(rsi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // c5ffe add r8d,0x4
            { let (r, f) = add::<32>((r8 as u32 as u64), 0x4u64);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c6002 lea eax,[r9+0xc]
            rax = (r9.wrapping_add(0xc)) as u32 as u64;
            // c6006 add r9d,0x7
            { let (r, f) = add::<32>((r9 as u32 as u64), 0x7u64);
              r9 = (r) as u32 as u64;
              fl = f; }
            // c600a cmp r9d,edx
            fl = sub::<32>((r9 as u32 as u64), (rdx as u32 as u64)).1;
            // c600d mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // c6010 jb 0xc5df7
            if fl.b() { pc = 0xc5df7; continue; }
            pc = 0xc6016;
        }
        0xc6016 => {
            // block c6016
            // c6016 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c6017 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c6019 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c601a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc7c7f size=789
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7c7f<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc7c7f;
    loop {
        match pc {
        0xc7c7f => {
            // block c7c7f
            // c7c7f push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7c80 mov rbp,rsp
            rbp = rsp;
            // c7c83 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7c85 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7c86 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c7c89 mov rbx,rsi
            rbx = rsi;
            // c7c8c dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // c7c8e cmp ecx,0x1c
            fl = sub::<32>((rcx as u32 as u64), 0x1cu64).1;
            // c7c91 ja 0xc7f19
            if fl.a() { pc = 0xc7f19; continue; }
            pc = 0xc7c97;
        }
        0xc7c97 => {
            // block c7c97
            // c7c97 lea rax,[rip+0x282] # 0xc7f20
            rax = m.env.code_va(0xc7f20);
            // c7c9e movsxd rcx,DWORD PTR [rax+rcx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // c7ca2 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // c7ca5 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7ca7 => {
            // block c7ca7
            // c7ca7 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7cae sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb0 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7cb3 mov rsi,rbx
            rsi = rbx;
            // c7cb6 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cb9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7cbc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7cbd jmp 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7cc2 => {
            // block c7cc2
            // c7cc2 mov rsi,rbx
            rsi = rbx;
            // c7cc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7cc8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7cc9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ccb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ccc jmp 0xc5d09
            { let r = { let t = m.env.code_va(0xc5d09); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7cd1 => {
            // block c7cd1
            // c7cd1 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7cd8 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7cde;
        }
        0xc7cde => {
            // block c7cde
            // c7cde add r14d,r14d
            { let (r, f) = add::<32>((r14 as u32 as u64), (r14 as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7ce1;
        }
        0xc7ce1 => {
            // block c7ce1
            // c7ce1 movzx eax,WORD PTR [rdi]
            rax = (m.r16(rdi) as u64) as u32 as u64;
            // c7ce4 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7ce7 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // c7ce9 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // c7cec add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7cf0 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7cf3 jne 0xc7ce1
            if fl.ne() { pc = 0xc7ce1; continue; }
            pc = 0xc7cf5;
        }
        0xc7cf5 => {
            // block c7cf5
            // c7cf5 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7cfa => {
            // block c7cfa
            // c7cfa cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d01 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d03 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d06 mov rsi,rbx
            rsi = rbx;
            // c7d09 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d0c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d10 jmp 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7d15 => {
            // block c7d15
            // c7d15 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d1c sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d1e and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d21 mov rsi,rbx
            rsi = rbx;
            // c7d24 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d27 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d2c mov rdi,rbx
            rdi = rbx;
            // c7d2f mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d35 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d36 jmp 0xc7a37
            { let r = { let t = m.env.code_va(0xc7a37); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7d3b => {
            // block c7d3b
            // c7d3b cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d42 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d44 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d47 mov rsi,rbx
            rsi = rbx;
            // c7d4a mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7d4d call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d52 mov rdi,rbx
            rdi = rbx;
            // c7d55 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d59 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d5c jmp 0xc601b
            { let r = { let t = m.env.code_va(0xc601b); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xc7d61 => {
            // block c7d61
            // c7d61 test r14d,0x7fffffff
            fl = logic::<32>((r14 as u32 as u64) & 0x7fffffffu64);
            // c7d68 je 0xc7d84
            if fl.e() { pc = 0xc7d84; continue; }
            pc = 0xc7d6a;
        }
        0xc7d6a => {
            // block c7d6a
            // c7d6a lea eax,[r14+r14*1]
            rax = (r14.wrapping_add(r14)) as u32 as u64;
            // c7d6e mov rcx,rbx
            rcx = rbx;
            pc = 0xc7d71;
        }
        0xc7d71 => {
            // block c7d71
            // c7d71 movzx edx,WORD PTR [rdi]
            rdx = (m.r16(rdi) as u64) as u32 as u64;
            // c7d74 shr edx,0x2
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x2u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // c7d77 mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // c7d79 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // c7d7c add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // c7d80 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // c7d82 jne 0xc7d71
            if fl.ne() { pc = 0xc7d71; continue; }
            pc = 0xc7d84;
        }
        0xc7d84 => {
            // block c7d84
            // c7d84 mov rdi,rbx
            rdi = rbx;
            // c7d87 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7d8a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7d8b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7d8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7d8e jmp 0xc7bec
            { let r = { let t = m.env.code_va(0xc7bec); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0xc7d93 => {
            // block c7d93
            // c7d93 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7d9a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7d9f mov rsi,rbx
            rsi = rbx;
            // c7da2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7da5 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7daa shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c7dae test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7db1 je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7db7;
        }
        0xc7db7 => {
            // block c7db7
            // c7db7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7dba xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7dbc;
        }
        0xc7dbc => {
            // block c7dbc
            // c7dbc mov dl,BYTE PTR [rbx+rcx*1]
            rdx = (rdx & !0xff) | ((m.r8(rbx.wrapping_add(rcx)) as u64) as u8 as u64);
            // c7dbf mov al,BYTE PTR [rbx+rcx*1+0x2]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(rcx).wrapping_add(0x2)) as u64) as u8 as u64);
            // c7dc3 mov BYTE PTR [rbx+rcx*1],al
            m.w8(rbx.wrapping_add(rcx), ((rax as u8 as u64)) as u8);
            // c7dc6 mov BYTE PTR [rbx+rcx*1+0x2],dl
            m.w8(rbx.wrapping_add(rcx).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // c7dca add rcx,0x4
            { let (r, f) = add::<64>(rcx, 0x4u64);
              rcx = r;
              fl = f; }
            // c7dce cmp rcx,rsi
            fl = sub::<64>(rcx, rsi).1;
            // c7dd1 jb 0xc7dbc
            if fl.b() { pc = 0xc7dbc; continue; }
            pc = 0xc7dd3;
        }
        0xc7dd3 => {
            // block c7dd3
            // c7dd3 jmp 0xc7f19
            pc = 0xc7f19; continue;
        }
        0xc7dd8 => {
            // block c7dd8
            // c7dd8 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7ddf sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de1 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7de4 mov rsi,rbx
            rsi = rbx;
            // c7de7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7dea call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7def mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c7df4 jmp 0xc7e75
            pc = 0xc7e75; continue;
        }
        0xc7df6 => {
            // block c7df6
            // c7df6 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // c7dfb mov rsi,rbx
            rsi = rbx;
            // c7dfe mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e01 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e05 jmp 0xc5ddf
            { let r = { let t = m.env.code_va(0xc5ddf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e0a => {
            // block c7e0a
            // c7e0a cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e11 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e13 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e16 mov rsi,rbx
            rsi = rbx;
            // c7e19 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e1c call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e21 mov rdi,rbx
            rdi = rbx;
            // c7e24 mov rsi,rbx
            rsi = rbx;
            // c7e27 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e2e jmp 0xc7a9b
            { let r = { let t = m.env.code_va(0xc7a9b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e33 => {
            // block c7e33
            // c7e33 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e3a sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3c and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e3f mov rsi,rbx
            rsi = rbx;
            // c7e42 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e45 call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e4a mov rdi,rbx
            rdi = rbx;
            // c7e4d mov rsi,rbx
            rsi = rbx;
            // c7e50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e54 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e57 jmp 0xc7acf
            { let r = { let t = m.env.code_va(0xc7acf); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e5c => {
            // block c7e5c
            // c7e5c cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e63 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e65 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e68 mov rsi,rbx
            rsi = rbx;
            // c7e6b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e6e call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e73 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7e75;
        }
        0xc7e75 => {
            // block c7e75
            // c7e75 mov rdi,rbx
            rdi = rbx;
            // c7e78 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7e7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7e7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7e7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7e7f jmp 0xc60bc
            { let r = { let t = m.env.code_va(0xc60bc); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7e84 => {
            // block c7e84
            // c7e84 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7e8b sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e8d and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7e90 mov rsi,rbx
            rsi = rbx;
            // c7e93 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7e96 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7e9b mov rdi,rbx
            rdi = rbx;
            // c7e9e mov rsi,rbx
            rsi = rbx;
            // c7ea1 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ea4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ea5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ea7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ea8 jmp 0xc7b35
            { let r = { let t = m.env.code_va(0xc7b35); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0xc7ead => {
            // block c7ead
            // c7ead cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eb4 sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb6 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7eb9 mov rsi,rbx
            rsi = rbx;
            // c7ebc mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ebf call 0xc6487
            { let r = { let t = m.env.code_va(0xc6487); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ec4 mov rdi,rbx
            rdi = rbx;
            // c7ec7 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // c7eca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7ecb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7ecd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7ece jmp 0xc604d
            { let r = { let t = m.env.code_va(0xc604d); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xc7ed3 => {
            // block c7ed3
            // c7ed3 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // c7eda sbb ecx,ecx
            { let (r, f) = sbb::<32>((rcx as u32 as u64), (rcx as u32 as u64), fl.cf);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edc and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7edf mov rsi,rbx
            rsi = rbx;
            // c7ee2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7ee5 call 0xc70f4
            { let r = { let t = m.env.code_va(0xc70f4); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7eea test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7eed je 0xc7f19
            if fl.e() { pc = 0xc7f19; continue; }
            pc = 0xc7eef;
        }
        0xc7eef => {
            // block c7eef
            // c7eef mov rax,rbx
            rax = rbx;
            // c7ef2 add rax,0x4
            { let (r, f) = add::<64>(rax, 0x4u64);
              rax = r;
              fl = f; }
            pc = 0xc7ef6;
        }
        0xc7ef6 => {
            // block c7ef6
            // c7ef6 mov cx,WORD PTR [rax-0x4]
            rcx = (rcx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u16 as u64);
            // c7efa mov dx,WORD PTR [rax-0x2]
            rdx = (rdx & !0xffff) | ((m.r16(rax.wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u16 as u64);
            // c7efe mov si,WORD PTR [rax]
            rsi = (rsi & !0xffff) | ((m.r16(rax) as u64) as u16 as u64);
            // c7f01 mov WORD PTR [rbx],si
            m.w16(rbx, ((rsi as u16 as u64)) as u16);
            // c7f04 mov WORD PTR [rbx+0x2],dx
            m.w16(rbx.wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // c7f08 mov WORD PTR [rbx+0x4],cx
            m.w16(rbx.wrapping_add(0x4), ((rcx as u16 as u64)) as u16);
            // c7f0c add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c7f10 add rbx,0x6
            { let (r, f) = add::<64>(rbx, 0x6u64);
              rbx = r;
              fl = f; }
            // c7f14 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // c7f17 jne 0xc7ef6
            if fl.ne() { pc = 0xc7ef6; continue; }
            pc = 0xc7f19;
        }
        0xc7f19 => {
            // block c7f19
            // c7f19 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7f1a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7f1c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7f1d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc6cdf size=1045
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c6cdf<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc6cdf;
    loop {
        match pc {
        0xc6cdf => {
            // block c6cdf
            // c6cdf push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c6ce0 mov rbp,rsp
            rbp = rsp;
            // c6ce3 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c6ce5 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c6ce7 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c6ce9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c6ceb push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c6cec test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c6cee je 0xc70e9
            if fl.e() { pc = 0xc70e9; continue; }
            pc = 0xc6cf4;
        }
        0xc6cf4 => {
            // block c6cf4
            // c6cf4 movzx r11d,WORD PTR [rdi+0x4]
            r11 = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // c6cf9 movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // c6cfd mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c6d00 movzx r13d,WORD PTR [rdi]
            r13 = (m.r16(rdi) as u64) as u32 as u64;
            // c6d04 lea r8,[rdi+0x6]
            r8 = rdi.wrapping_add(0x6);
            // c6d08 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6d0b;
        }
        0xc6d0b => {
            // block c6d0b
            // c6d0b mov r12d,r10d
            r12 = ((r10 as u32 as u64)) as u32 as u64;
            // c6d0e mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // c6d11 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // c6d14 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c6d16 je 0xc6d43
            if fl.e() { pc = 0xc6d43; continue; }
            pc = 0xc6d18;
        }
        0xc6d18 => {
            // block c6d18
            // c6d18 imul r15d,eax,0x4a86
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x4a86u64); r15 = (r) as u32 as u64; fl = f; }
            // c6d1f movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c6d23 imul eax,r10d,0x6626
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x6626u64); rax = (r) as u32 as u64; fl = f; }
            // c6d2a add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6d2d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6d2f add edi,0xff211280
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xff211280u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6d35 js 0xc6d6e
            if fl.s() { pc = 0xc6d6e; continue; }
            pc = 0xc6d37;
        }
        0xc6d37 => {
            // block c6d37
            // c6d37 add eax,0xff219280
            { let (r, f) = add::<32>((rax as u32 as u64), 0xff219280u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6d3c shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6d3f mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6d41 jmp 0xc6d7a
            pc = 0xc6d7a; continue;
        }
        0xc6d43 => {
            // block c6d43
            // c6d43 imul r15d,eax,0x4ab3
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x4ab3u64); r15 = (r) as u32 as u64; fl = f; }
            // c6d4a movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c6d4e imul edi,r10d,0x77dc
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x77dcu64); rdi = (r) as u32 as u64; fl = f; }
            // c6d55 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6d58 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6d5a add eax,0xfefd9b40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfefd9b40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6d5f js 0xc6d91
            if fl.s() { pc = 0xc6d91; continue; }
            pc = 0xc6d61;
        }
        0xc6d61 => {
            // block c6d61
            // c6d61 add edi,0xfefe1b40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfefe1b40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6d67 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6d6a mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6d6c jmp 0xc6d9d
            pc = 0xc6d9d; continue;
        }
        0xc6d6e => {
            // block c6d6e
            // c6d6e mov edi,0xdf6d80
            rdi = (0xdf6d80u64) as u32 as u64;
            // c6d73 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6d75 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6d78 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6d7a;
        }
        0xc6d7a => {
            // block c6d7a
            // c6d7a movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6d7d mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6d7f cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6d85 jg 0xc6db7
            if fl.g() { pc = 0xc6db7; continue; }
            pc = 0xc6d87;
        }
        0xc6d87 => {
            // block c6d87
            // c6d87 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6d8a js 0xc6db5
            if fl.s() { pc = 0xc6db5; continue; }
            pc = 0xc6d8c;
        }
        0xc6d8c => {
            // block c6d8c
            // c6d8c mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6d8f jmp 0xc6db7
            pc = 0xc6db7; continue;
        }
        0xc6d91 => {
            // block c6d91
            // c6d91 mov eax,0x102e4c0
            rax = (0x102e4c0u64) as u32 as u64;
            // c6d96 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6d98 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6d9b neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6d9d;
        }
        0xc6d9d => {
            // block c6d9d
            // c6d9d movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6da0 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6da3 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6da9 jg 0xc6e05
            if fl.g() { pc = 0xc6e05; continue; }
            pc = 0xc6dab;
        }
        0xc6dab => {
            // block c6dab
            // c6dab test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6dae js 0xc6e03
            if fl.s() { pc = 0xc6e03; continue; }
            pc = 0xc6db0;
        }
        0xc6db0 => {
            // block c6db0
            // c6db0 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6db3 jmp 0xc6e05
            pc = 0xc6e05; continue;
        }
        0xc6db5 => {
            // block c6db5
            // c6db5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6db7;
        }
        0xc6db7 => {
            // block c6db7
            // c6db7 mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c6dbc movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c6dc0 imul edi,r14d,0x811b
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x811bu64); rdi = (r) as u32 as u64; fl = f; }
            // c6dc7 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6dca add edi,0xfeeb2880
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfeeb2880u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6dd0 js 0xc6ddf
            if fl.s() { pc = 0xc6ddf; continue; }
            pc = 0xc6dd2;
        }
        0xc6dd2 => {
            // block c6dd2
            // c6dd2 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6dd8 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6ddb mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6ddd jmp 0xc6deb
            pc = 0xc6deb; continue;
        }
        0xc6ddf => {
            // block c6ddf
            // c6ddf mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6de4 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6de6 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6de9 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6deb;
        }
        0xc6deb => {
            // block c6deb
            // c6deb movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6dee mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6df1 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6df7 jg 0xc6e53
            if fl.g() { pc = 0xc6e53; continue; }
            pc = 0xc6df9;
        }
        0xc6df9 => {
            // block c6df9
            // c6df9 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6dfc js 0xc6e51
            if fl.s() { pc = 0xc6e51; continue; }
            pc = 0xc6dfe;
        }
        0xc6dfe => {
            // block c6dfe
            // c6dfe mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6e01 jmp 0xc6e53
            pc = 0xc6e53; continue;
        }
        0xc6e03 => {
            // block c6e03
            // c6e03 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6e05;
        }
        0xc6e05 => {
            // block c6e05
            // c6e05 mov BYTE PTR [rsi+r12*4+0x2],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdi as u8 as u64)) as u8);
            // c6e0a movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c6e0e imul edi,r14d,0x8aa1
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x8aa1u64); rdi = (r) as u32 as u64; fl = f; }
            // c6e15 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e18 add edi,0xfed81140
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfed81140u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e1e js 0xc6e2d
            if fl.s() { pc = 0xc6e2d; continue; }
            pc = 0xc6e20;
        }
        0xc6e20 => {
            // block c6e20
            // c6e20 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e26 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6e29 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6e2b jmp 0xc6e39
            pc = 0xc6e39; continue;
        }
        0xc6e2d => {
            // block c6e2d
            // c6e2d mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6e32 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6e34 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6e37 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6e39;
        }
        0xc6e39 => {
            // block c6e39
            // c6e39 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6e3c mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6e3f cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6e45 jg 0xc6e77
            if fl.g() { pc = 0xc6e77; continue; }
            pc = 0xc6e47;
        }
        0xc6e47 => {
            // block c6e47
            // c6e47 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6e4a js 0xc6e75
            if fl.s() { pc = 0xc6e75; continue; }
            pc = 0xc6e4c;
        }
        0xc6e4c => {
            // block c6e4c
            // c6e4c mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6e4f jmp 0xc6e77
            pc = 0xc6e77; continue;
        }
        0xc6e51 => {
            // block c6e51
            // c6e51 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6e53;
        }
        0xc6e53 => {
            // block c6e53
            // c6e53 mov BYTE PTR [rsi+r12*4],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdi as u8 as u64)) as u8);
            // c6e57 imul eax,r14d,0xffffe6ed
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffffe6edu64); rax = (r) as u32 as u64; fl = f; }
            // c6e5e add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c6e61 imul edi,r10d,0xffffcbf8
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffffcbf8u64); rdi = (r) as u32 as u64; fl = f; }
            // c6e68 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e6b add edi,0x879480
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x879480u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e71 jns 0xc6e97
            if fl.ns() { pc = 0xc6e97; continue; }
            pc = 0xc6e73;
        }
        0xc6e73 => {
            // block c6e73
            // c6e73 jmp 0xc6ea4
            pc = 0xc6ea4; continue;
        }
        0xc6e75 => {
            // block c6e75
            // c6e75 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6e77;
        }
        0xc6e77 => {
            // block c6e77
            // c6e77 mov BYTE PTR [rsi+r12*4],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdi as u8 as u64)) as u8);
            // c6e7b imul eax,r14d,0xfffff1ff
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xfffff1ffu64); rax = (r) as u32 as u64; fl = f; }
            // c6e82 add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c6e85 imul edi,r10d,0xffffdcf4
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffffdcf4u64); rdi = (r) as u32 as u64; fl = f; }
            // c6e8c add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e8f add edi,0x4f6d40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x4f6d40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e95 js 0xc6ea4
            if fl.s() { pc = 0xc6ea4; continue; }
            pc = 0xc6e97;
        }
        0xc6e97 => {
            // block c6e97
            // c6e97 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6e9d shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6ea0 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6ea2 jmp 0xc6eb0
            pc = 0xc6eb0; continue;
        }
        0xc6ea4 => {
            // block c6ea4
            // c6ea4 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6ea9 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6eab sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6eae neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6eb0;
        }
        0xc6eb0 => {
            // block c6eb0
            // c6eb0 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6eb3 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6eb6 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6ebc jg 0xc6eca
            if fl.g() { pc = 0xc6eca; continue; }
            pc = 0xc6ebe;
        }
        0xc6ebe => {
            // block c6ebe
            // c6ebe test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6ec1 js 0xc6ec8
            if fl.s() { pc = 0xc6ec8; continue; }
            pc = 0xc6ec3;
        }
        0xc6ec3 => {
            // block c6ec3
            // c6ec3 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6ec6 jmp 0xc6eca
            pc = 0xc6eca; continue;
        }
        0xc6ec8 => {
            // block c6ec8
            // c6ec8 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6eca;
        }
        0xc6eca => {
            // block c6eca
            // c6eca mov BYTE PTR [rsi+r12*4+0x1],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdi as u8 as u64)) as u8);
            // c6ecf mov BYTE PTR [rsi+r12*4+0x3],0xac
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3), (0xacu64) as u8);
            // c6ed5 movzx edi,WORD PTR [r8]
            rdi = (m.r16(r8) as u64) as u32 as u64;
            // c6ed9 lea r10d,[r12+0x2]
            r10 = (r12.wrapping_add(0x2)) as u32 as u64;
            // c6ede cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c6ee1 jae 0xc6efb
            if fl.ae() { pc = 0xc6efb; continue; }
            pc = 0xc6ee3;
        }
        0xc6ee3 => {
            // block c6ee3
            // c6ee3 movzx r14d,WORD PTR [r8+0x2]
            r14 = (m.r16(r8.wrapping_add(0x2)) as u64) as u32 as u64;
            // c6ee8 movzx eax,WORD PTR [r8+0x4]
            rax = (m.r16(r8.wrapping_add(0x4)) as u64) as u32 as u64;
            // c6eed mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c6ef0 movzx r15d,WORD PTR [r8+0x6]
            r15 = (m.r16(r8.wrapping_add(0x6)) as u64) as u32 as u64;
            // c6ef5 add r8,0x8
            { let (r, f) = add::<64>(r8, 0x8u64);
              r8 = r;
              fl = f; }
            // c6ef9 jmp 0xc6f07
            pc = 0xc6f07; continue;
        }
        0xc6efb => {
            // block c6efb
            // c6efb add r8,0x2
            { let (r, f) = add::<64>(r8, 0x2u64);
              r8 = r;
              fl = f; }
            // c6eff mov r14w,r13w
            r14 = (r14 & !0xffff) | (((r13 as u16 as u64)) as u16 as u64);
            // c6f03 mov r15w,r11w
            r15 = (r15 & !0xffff) | (((r11 as u16 as u64)) as u16 as u64);
            pc = 0xc6f07;
        }
        0xc6f07 => {
            // block c6f07
            // c6f07 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // c6f0b movzx r13d,r14w
            r13 = ((r14 as u16 as u64)) as u32 as u64;
            // c6f0f add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // c6f12 shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // c6f15 movzx eax,r11w
            rax = ((r11 as u16 as u64)) as u32 as u64;
            // c6f19 movzx r9d,r15w
            r9 = ((r15 as u16 as u64)) as u32 as u64;
            // c6f1d add r9d,eax
            { let (r, f) = add::<32>((r9 as u32 as u64), (rax as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // c6f20 shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // c6f23 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // c6f26 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c6f28 je 0xc6f54
            if fl.e() { pc = 0xc6f54; continue; }
            pc = 0xc6f2a;
        }
        0xc6f2a => {
            // block c6f2a
            // c6f2a imul r11d,edi,0x4a86
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x4a86u64); r11 = (r) as u32 as u64; fl = f; }
            // c6f31 imul eax,r9d,0x6626
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x6626u64); rax = (r) as u32 as u64; fl = f; }
            // c6f38 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6f3b mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6f3d mov rbx,r8
            rbx = r8;
            // c6f40 add edi,0xff211280
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xff211280u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6f46 js 0xc6f7b
            if fl.s() { pc = 0xc6f7b; continue; }
            pc = 0xc6f48;
        }
        0xc6f48 => {
            // block c6f48
            // c6f48 add eax,0xff219280
            { let (r, f) = add::<32>((rax as u32 as u64), 0xff219280u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6f4d shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6f50 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6f52 jmp 0xc6f87
            pc = 0xc6f87; continue;
        }
        0xc6f54 => {
            // block c6f54
            // c6f54 imul r11d,edi,0x4ab3
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x4ab3u64); r11 = (r) as u32 as u64; fl = f; }
            // c6f5b imul eax,r9d,0x77dc
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x77dcu64); rax = (r) as u32 as u64; fl = f; }
            // c6f62 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6f65 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6f67 add edi,0xfefd9b40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfefd9b40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6f6d js 0xc6fa0
            if fl.s() { pc = 0xc6fa0; continue; }
            pc = 0xc6f6f;
        }
        0xc6f6f => {
            // block c6f6f
            // c6f6f add eax,0xfefe1b40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfefe1b40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6f74 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6f77 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6f79 jmp 0xc6fac
            pc = 0xc6fac; continue;
        }
        0xc6f7b => {
            // block c6f7b
            // c6f7b mov edi,0xdf6d80
            rdi = (0xdf6d80u64) as u32 as u64;
            // c6f80 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6f82 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6f85 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6f87;
        }
        0xc6f87 => {
            // block c6f87
            // c6f87 movsx r8d,di
            r8 = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6f8b mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6f8d cmp r8d,0xff
            fl = sub::<32>((r8 as u32 as u64), 0xffu64).1;
            // c6f94 jg 0xc6fc5
            if fl.g() { pc = 0xc6fc5; continue; }
            pc = 0xc6f96;
        }
        0xc6f96 => {
            // block c6f96
            // c6f96 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6f99 js 0xc6fc3
            if fl.s() { pc = 0xc6fc3; continue; }
            pc = 0xc6f9b;
        }
        0xc6f9b => {
            // block c6f9b
            // c6f9b mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6f9e jmp 0xc6fc5
            pc = 0xc6fc5; continue;
        }
        0xc6fa0 => {
            // block c6fa0
            // c6fa0 mov edi,0x102e4c0
            rdi = (0x102e4c0u64) as u32 as u64;
            // c6fa5 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6fa7 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6faa neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6fac;
        }
        0xc6fac => {
            // block c6fac
            // c6fac movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6faf mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6fb1 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6fb7 jg 0xc700f
            if fl.g() { pc = 0xc700f; continue; }
            pc = 0xc6fb9;
        }
        0xc6fb9 => {
            // block c6fb9
            // c6fb9 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6fbc js 0xc700d
            if fl.s() { pc = 0xc700d; continue; }
            pc = 0xc6fbe;
        }
        0xc6fbe => {
            // block c6fbe
            // c6fbe mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6fc1 jmp 0xc700f
            pc = 0xc700f; continue;
        }
        0xc6fc3 => {
            // block c6fc3
            // c6fc3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6fc5;
        }
        0xc6fc5 => {
            // block c6fc5
            // c6fc5 mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c6fca imul eax,r13d,0x811b
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x811bu64); rax = (r) as u32 as u64; fl = f; }
            // c6fd1 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6fd4 add eax,0xfeeb2880
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfeeb2880u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6fd9 mov r8,rbx
            r8 = rbx;
            // c6fdc js 0xc6fea
            if fl.s() { pc = 0xc6fea; continue; }
            pc = 0xc6fde;
        }
        0xc6fde => {
            // block c6fde
            // c6fde add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6fe3 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6fe6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6fe8 jmp 0xc6ff6
            pc = 0xc6ff6; continue;
        }
        0xc6fea => {
            // block c6fea
            // c6fea mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c6fef sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6ff1 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6ff4 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6ff6;
        }
        0xc6ff6 => {
            // block c6ff6
            // c6ff6 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6ff9 mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6ffb cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c7001 jg 0xc7056
            if fl.g() { pc = 0xc7056; continue; }
            pc = 0xc7003;
        }
        0xc7003 => {
            // block c7003
            // c7003 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c7006 js 0xc7054
            if fl.s() { pc = 0xc7054; continue; }
            pc = 0xc7008;
        }
        0xc7008 => {
            // block c7008
            // c7008 mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c700b jmp 0xc7056
            pc = 0xc7056; continue;
        }
        0xc700d => {
            // block c700d
            // c700d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc700f;
        }
        0xc700f => {
            // block c700f
            // c700f mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c7014 imul eax,r13d,0x8aa1
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x8aa1u64); rax = (r) as u32 as u64; fl = f; }
            // c701b add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c701e add eax,0xfed81140
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfed81140u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c7023 js 0xc7031
            if fl.s() { pc = 0xc7031; continue; }
            pc = 0xc7025;
        }
        0xc7025 => {
            // block c7025
            // c7025 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c702a shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c702d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c702f jmp 0xc703d
            pc = 0xc703d; continue;
        }
        0xc7031 => {
            // block c7031
            // c7031 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c7036 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7038 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c703b neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc703d;
        }
        0xc703d => {
            // block c703d
            // c703d movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c7040 mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c7042 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c7048 jg 0xc707a
            if fl.g() { pc = 0xc707a; continue; }
            pc = 0xc704a;
        }
        0xc704a => {
            // block c704a
            // c704a test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c704d js 0xc7078
            if fl.s() { pc = 0xc7078; continue; }
            pc = 0xc704f;
        }
        0xc704f => {
            // block c704f
            // c704f mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c7052 jmp 0xc707a
            pc = 0xc707a; continue;
        }
        0xc7054 => {
            // block c7054
            // c7054 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7056;
        }
        0xc7056 => {
            // block c7056
            // c7056 mov BYTE PTR [rsi+r12*4],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rax as u8 as u64)) as u8);
            // c705a imul eax,r13d,0xffffe6ed
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffe6edu64); rax = (r) as u32 as u64; fl = f; }
            // c7061 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c7064 imul edi,r9d,0xffffcbf8
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffffcbf8u64); rdi = (r) as u32 as u64; fl = f; }
            // c706b add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c706e add edi,0x879480
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x879480u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7074 jns 0xc709a
            if fl.ns() { pc = 0xc709a; continue; }
            pc = 0xc7076;
        }
        0xc7076 => {
            // block c7076
            // c7076 jmp 0xc70a7
            pc = 0xc70a7; continue;
        }
        0xc7078 => {
            // block c7078
            // c7078 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc707a;
        }
        0xc707a => {
            // block c707a
            // c707a mov BYTE PTR [rsi+r12*4],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rax as u8 as u64)) as u8);
            // c707e imul eax,r13d,0xfffff1ff
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xfffff1ffu64); rax = (r) as u32 as u64; fl = f; }
            // c7085 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c7088 imul edi,r9d,0xffffdcf4
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffffdcf4u64); rdi = (r) as u32 as u64; fl = f; }
            // c708f add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7092 add edi,0x4f6d40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x4f6d40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7098 js 0xc70a7
            if fl.s() { pc = 0xc70a7; continue; }
            pc = 0xc709a;
        }
        0xc709a => {
            // block c709a
            // c709a add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c70a0 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c70a3 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c70a5 jmp 0xc70b3
            pc = 0xc70b3; continue;
        }
        0xc70a7 => {
            // block c70a7
            // c70a7 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c70ac sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c70ae sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c70b1 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc70b3;
        }
        0xc70b3 => {
            // block c70b3
            // c70b3 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c70b6 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c70b9 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c70bf jg 0xc70cd
            if fl.g() { pc = 0xc70cd; continue; }
            pc = 0xc70c1;
        }
        0xc70c1 => {
            // block c70c1
            // c70c1 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c70c4 js 0xc70cb
            if fl.s() { pc = 0xc70cb; continue; }
            pc = 0xc70c6;
        }
        0xc70c6 => {
            // block c70c6
            // c70c6 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c70c9 jmp 0xc70cd
            pc = 0xc70cd; continue;
        }
        0xc70cb => {
            // block c70cb
            // c70cb xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc70cd;
        }
        0xc70cd => {
            // block c70cd
            // c70cd mov BYTE PTR [rsi+r12*4+0x1],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdi as u8 as u64)) as u8);
            // c70d2 mov BYTE PTR [rsi+r12*4+0x3],0xac
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3), (0xacu64) as u8);
            // c70d8 cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c70db mov r11w,r15w
            r11 = (r11 & !0xffff) | (((r15 as u16 as u64)) as u16 as u64);
            // c70df mov r13w,r14w
            r13 = (r13 & !0xffff) | (((r14 as u16 as u64)) as u16 as u64);
            // c70e3 jb 0xc6d0b
            if fl.b() { pc = 0xc6d0b; continue; }
            pc = 0xc70e9;
        }
        0xc70e9 => {
            // block c70e9
            // c70e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c70ea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c70ec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c70ee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c70f0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c70f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c70f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc68b5 size=1066
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c68b5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc68b5;
    loop {
        match pc {
        0xc68b5 => {
            // block c68b5
            // c68b5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c68b6 mov rbp,rsp
            rbp = rsp;
            // c68b9 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c68bb push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c68bd push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c68bf push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c68c1 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c68c2 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c68c4 je 0xc6cd4
            if fl.e() { pc = 0xc6cd4; continue; }
            pc = 0xc68ca;
        }
        0xc68ca => {
            // block c68ca
            // c68ca movzx r11d,BYTE PTR [rdi+0x2]
            r11 = (m.r8(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // c68cf shl r11d,0x2
            { let (r, f) = shl::<32>((r11 as u32 as u64), 0x2u64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c68d3 movzx eax,BYTE PTR [rdi+0x1]
            rax = (m.r8(rdi.wrapping_add(0x1)) as u64) as u32 as u64;
            // c68d7 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c68da mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c68dd movzx r13d,BYTE PTR [rdi]
            r13 = (m.r8(rdi) as u64) as u32 as u64;
            // c68e1 lea r8,[rdi+0x3]
            r8 = rdi.wrapping_add(0x3);
            // c68e5 shl r13d,0x2
            { let (r, f) = shl::<32>((r13 as u32 as u64), 0x2u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // c68e9 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc68ec;
        }
        0xc68ec => {
            // block c68ec
            // c68ec mov r12d,r10d
            r12 = ((r10 as u32 as u64)) as u32 as u64;
            // c68ef mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // c68f2 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // c68f5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c68f7 je 0xc6924
            if fl.e() { pc = 0xc6924; continue; }
            pc = 0xc68f9;
        }
        0xc68f9 => {
            // block c68f9
            // c68f9 imul r15d,eax,0x4a86
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x4a86u64); r15 = (r) as u32 as u64; fl = f; }
            // c6900 movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c6904 imul eax,r10d,0x6626
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x6626u64); rax = (r) as u32 as u64; fl = f; }
            // c690b add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c690e mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6910 add edi,0xff211280
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xff211280u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6916 js 0xc694f
            if fl.s() { pc = 0xc694f; continue; }
            pc = 0xc6918;
        }
        0xc6918 => {
            // block c6918
            // c6918 add eax,0xff219280
            { let (r, f) = add::<32>((rax as u32 as u64), 0xff219280u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c691d shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6920 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6922 jmp 0xc695b
            pc = 0xc695b; continue;
        }
        0xc6924 => {
            // block c6924
            // c6924 imul r15d,eax,0x4ab3
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x4ab3u64); r15 = (r) as u32 as u64; fl = f; }
            // c692b movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c692f imul edi,r10d,0x77dc
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x77dcu64); rdi = (r) as u32 as u64; fl = f; }
            // c6936 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6939 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c693b add eax,0xfefd9b40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfefd9b40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6940 js 0xc6972
            if fl.s() { pc = 0xc6972; continue; }
            pc = 0xc6942;
        }
        0xc6942 => {
            // block c6942
            // c6942 add edi,0xfefe1b40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfefe1b40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6948 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c694b mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c694d jmp 0xc697e
            pc = 0xc697e; continue;
        }
        0xc694f => {
            // block c694f
            // c694f mov edi,0xdf6d80
            rdi = (0xdf6d80u64) as u32 as u64;
            // c6954 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6956 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6959 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc695b;
        }
        0xc695b => {
            // block c695b
            // c695b movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c695e mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6960 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6966 jg 0xc6998
            if fl.g() { pc = 0xc6998; continue; }
            pc = 0xc6968;
        }
        0xc6968 => {
            // block c6968
            // c6968 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c696b js 0xc6996
            if fl.s() { pc = 0xc6996; continue; }
            pc = 0xc696d;
        }
        0xc696d => {
            // block c696d
            // c696d mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6970 jmp 0xc6998
            pc = 0xc6998; continue;
        }
        0xc6972 => {
            // block c6972
            // c6972 mov eax,0x102e4c0
            rax = (0x102e4c0u64) as u32 as u64;
            // c6977 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6979 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c697c neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc697e;
        }
        0xc697e => {
            // block c697e
            // c697e movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6981 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6984 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c698a jg 0xc69e6
            if fl.g() { pc = 0xc69e6; continue; }
            pc = 0xc698c;
        }
        0xc698c => {
            // block c698c
            // c698c test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c698f js 0xc69e4
            if fl.s() { pc = 0xc69e4; continue; }
            pc = 0xc6991;
        }
        0xc6991 => {
            // block c6991
            // c6991 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6994 jmp 0xc69e6
            pc = 0xc69e6; continue;
        }
        0xc6996 => {
            // block c6996
            // c6996 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6998;
        }
        0xc6998 => {
            // block c6998
            // c6998 mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c699d movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c69a1 imul edi,r14d,0x811b
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x811bu64); rdi = (r) as u32 as u64; fl = f; }
            // c69a8 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c69ab add edi,0xfeeb2880
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfeeb2880u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c69b1 js 0xc69c0
            if fl.s() { pc = 0xc69c0; continue; }
            pc = 0xc69b3;
        }
        0xc69b3 => {
            // block c69b3
            // c69b3 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c69b9 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c69bc mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c69be jmp 0xc69cc
            pc = 0xc69cc; continue;
        }
        0xc69c0 => {
            // block c69c0
            // c69c0 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c69c5 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c69c7 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c69ca neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc69cc;
        }
        0xc69cc => {
            // block c69cc
            // c69cc movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c69cf mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c69d2 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c69d8 jg 0xc6a34
            if fl.g() { pc = 0xc6a34; continue; }
            pc = 0xc69da;
        }
        0xc69da => {
            // block c69da
            // c69da test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c69dd js 0xc6a32
            if fl.s() { pc = 0xc6a32; continue; }
            pc = 0xc69df;
        }
        0xc69df => {
            // block c69df
            // c69df mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c69e2 jmp 0xc6a34
            pc = 0xc6a34; continue;
        }
        0xc69e4 => {
            // block c69e4
            // c69e4 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc69e6;
        }
        0xc69e6 => {
            // block c69e6
            // c69e6 mov BYTE PTR [rsi+r12*4+0x2],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdi as u8 as u64)) as u8);
            // c69eb movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c69ef imul edi,r14d,0x8aa1
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x8aa1u64); rdi = (r) as u32 as u64; fl = f; }
            // c69f6 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c69f9 add edi,0xfed81140
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfed81140u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c69ff js 0xc6a0e
            if fl.s() { pc = 0xc6a0e; continue; }
            pc = 0xc6a01;
        }
        0xc6a01 => {
            // block c6a01
            // c6a01 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a07 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6a0a mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6a0c jmp 0xc6a1a
            pc = 0xc6a1a; continue;
        }
        0xc6a0e => {
            // block c6a0e
            // c6a0e mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6a13 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6a15 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6a18 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6a1a;
        }
        0xc6a1a => {
            // block c6a1a
            // c6a1a movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6a1d mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6a20 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6a26 jg 0xc6a58
            if fl.g() { pc = 0xc6a58; continue; }
            pc = 0xc6a28;
        }
        0xc6a28 => {
            // block c6a28
            // c6a28 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6a2b js 0xc6a56
            if fl.s() { pc = 0xc6a56; continue; }
            pc = 0xc6a2d;
        }
        0xc6a2d => {
            // block c6a2d
            // c6a2d mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6a30 jmp 0xc6a58
            pc = 0xc6a58; continue;
        }
        0xc6a32 => {
            // block c6a32
            // c6a32 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6a34;
        }
        0xc6a34 => {
            // block c6a34
            // c6a34 mov BYTE PTR [rsi+r12*4],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdi as u8 as u64)) as u8);
            // c6a38 imul eax,r14d,0xffffe6ed
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffffe6edu64); rax = (r) as u32 as u64; fl = f; }
            // c6a3f add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c6a42 imul edi,r10d,0xffffcbf8
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffffcbf8u64); rdi = (r) as u32 as u64; fl = f; }
            // c6a49 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a4c add edi,0x879480
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x879480u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a52 jns 0xc6a78
            if fl.ns() { pc = 0xc6a78; continue; }
            pc = 0xc6a54;
        }
        0xc6a54 => {
            // block c6a54
            // c6a54 jmp 0xc6a85
            pc = 0xc6a85; continue;
        }
        0xc6a56 => {
            // block c6a56
            // c6a56 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6a58;
        }
        0xc6a58 => {
            // block c6a58
            // c6a58 mov BYTE PTR [rsi+r12*4],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdi as u8 as u64)) as u8);
            // c6a5c imul eax,r14d,0xfffff1ff
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xfffff1ffu64); rax = (r) as u32 as u64; fl = f; }
            // c6a63 add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c6a66 imul edi,r10d,0xffffdcf4
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffffdcf4u64); rdi = (r) as u32 as u64; fl = f; }
            // c6a6d add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a70 add edi,0x4f6d40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x4f6d40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a76 js 0xc6a85
            if fl.s() { pc = 0xc6a85; continue; }
            pc = 0xc6a78;
        }
        0xc6a78 => {
            // block c6a78
            // c6a78 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6a7e shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6a81 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6a83 jmp 0xc6a91
            pc = 0xc6a91; continue;
        }
        0xc6a85 => {
            // block c6a85
            // c6a85 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6a8a sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6a8c sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6a8f neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6a91;
        }
        0xc6a91 => {
            // block c6a91
            // c6a91 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6a94 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6a97 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6a9d jg 0xc6aab
            if fl.g() { pc = 0xc6aab; continue; }
            pc = 0xc6a9f;
        }
        0xc6a9f => {
            // block c6a9f
            // c6a9f test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6aa2 js 0xc6aa9
            if fl.s() { pc = 0xc6aa9; continue; }
            pc = 0xc6aa4;
        }
        0xc6aa4 => {
            // block c6aa4
            // c6aa4 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6aa7 jmp 0xc6aab
            pc = 0xc6aab; continue;
        }
        0xc6aa9 => {
            // block c6aa9
            // c6aa9 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6aab;
        }
        0xc6aab => {
            // block c6aab
            // c6aab mov BYTE PTR [rsi+r12*4+0x1],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdi as u8 as u64)) as u8);
            // c6ab0 mov BYTE PTR [rsi+r12*4+0x3],0xac
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3), (0xacu64) as u8);
            // c6ab6 movzx edi,BYTE PTR [r8]
            rdi = (m.r8(r8) as u64) as u32 as u64;
            // c6aba lea r10d,[r12+0x2]
            r10 = (r12.wrapping_add(0x2)) as u32 as u64;
            // c6abf cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c6ac2 jae 0xc6ae7
            if fl.ae() { pc = 0xc6ae7; continue; }
            pc = 0xc6ac4;
        }
        0xc6ac4 => {
            // block c6ac4
            // c6ac4 movzx r14d,BYTE PTR [r8+0x1]
            r14 = (m.r8(r8.wrapping_add(0x1)) as u64) as u32 as u64;
            // c6ac9 shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // c6acd movzx eax,BYTE PTR [r8+0x2]
            rax = (m.r8(r8.wrapping_add(0x2)) as u64) as u32 as u64;
            // c6ad2 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6ad5 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c6ad8 movzx r15d,BYTE PTR [r8+0x3]
            r15 = (m.r8(r8.wrapping_add(0x3)) as u64) as u32 as u64;
            // c6add add r8,0x4
            { let (r, f) = add::<64>(r8, 0x4u64);
              r8 = r;
              fl = f; }
            // c6ae1 shl r15d,0x2
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x2u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c6ae5 jmp 0xc6af2
            pc = 0xc6af2; continue;
        }
        0xc6ae7 => {
            // block c6ae7
            // c6ae7 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // c6aea mov r14w,r13w
            r14 = (r14 & !0xffff) | (((r13 as u16 as u64)) as u16 as u64);
            // c6aee mov r15w,r11w
            r15 = (r15 & !0xffff) | (((r11 as u16 as u64)) as u16 as u64);
            pc = 0xc6af2;
        }
        0xc6af2 => {
            // block c6af2
            // c6af2 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // c6af6 movzx r13d,r14w
            r13 = ((r14 as u16 as u64)) as u32 as u64;
            // c6afa add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // c6afd shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // c6b00 movzx eax,r11w
            rax = ((r11 as u16 as u64)) as u32 as u64;
            // c6b04 movzx r9d,r15w
            r9 = ((r15 as u16 as u64)) as u32 as u64;
            // c6b08 add r9d,eax
            { let (r, f) = add::<32>((r9 as u32 as u64), (rax as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // c6b0b shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // c6b0e inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // c6b11 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c6b13 je 0xc6b3f
            if fl.e() { pc = 0xc6b3f; continue; }
            pc = 0xc6b15;
        }
        0xc6b15 => {
            // block c6b15
            // c6b15 imul r11d,edi,0x12a18
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12a18u64); r11 = (r) as u32 as u64; fl = f; }
            // c6b1c imul eax,r9d,0x6626
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x6626u64); rax = (r) as u32 as u64; fl = f; }
            // c6b23 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6b26 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6b28 mov rbx,r8
            rbx = r8;
            // c6b2b add edi,0xff211280
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xff211280u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6b31 js 0xc6b66
            if fl.s() { pc = 0xc6b66; continue; }
            pc = 0xc6b33;
        }
        0xc6b33 => {
            // block c6b33
            // c6b33 add eax,0xff219280
            { let (r, f) = add::<32>((rax as u32 as u64), 0xff219280u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6b38 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6b3b mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6b3d jmp 0xc6b72
            pc = 0xc6b72; continue;
        }
        0xc6b3f => {
            // block c6b3f
            // c6b3f imul r11d,edi,0x12acc
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12accu64); r11 = (r) as u32 as u64; fl = f; }
            // c6b46 imul eax,r9d,0x77dc
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x77dcu64); rax = (r) as u32 as u64; fl = f; }
            // c6b4d add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6b50 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6b52 add edi,0xfefd9b40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfefd9b40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6b58 js 0xc6b8b
            if fl.s() { pc = 0xc6b8b; continue; }
            pc = 0xc6b5a;
        }
        0xc6b5a => {
            // block c6b5a
            // c6b5a add eax,0xfefe1b40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfefe1b40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6b5f shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6b62 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6b64 jmp 0xc6b97
            pc = 0xc6b97; continue;
        }
        0xc6b66 => {
            // block c6b66
            // c6b66 mov edi,0xdf6d80
            rdi = (0xdf6d80u64) as u32 as u64;
            // c6b6b sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6b6d sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6b70 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6b72;
        }
        0xc6b72 => {
            // block c6b72
            // c6b72 movsx r8d,di
            r8 = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6b76 mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6b78 cmp r8d,0xff
            fl = sub::<32>((r8 as u32 as u64), 0xffu64).1;
            // c6b7f jg 0xc6bb0
            if fl.g() { pc = 0xc6bb0; continue; }
            pc = 0xc6b81;
        }
        0xc6b81 => {
            // block c6b81
            // c6b81 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6b84 js 0xc6bae
            if fl.s() { pc = 0xc6bae; continue; }
            pc = 0xc6b86;
        }
        0xc6b86 => {
            // block c6b86
            // c6b86 mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6b89 jmp 0xc6bb0
            pc = 0xc6bb0; continue;
        }
        0xc6b8b => {
            // block c6b8b
            // c6b8b mov edi,0x102e4c0
            rdi = (0x102e4c0u64) as u32 as u64;
            // c6b90 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6b92 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6b95 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6b97;
        }
        0xc6b97 => {
            // block c6b97
            // c6b97 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6b9a mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6b9c cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6ba2 jg 0xc6bfa
            if fl.g() { pc = 0xc6bfa; continue; }
            pc = 0xc6ba4;
        }
        0xc6ba4 => {
            // block c6ba4
            // c6ba4 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6ba7 js 0xc6bf8
            if fl.s() { pc = 0xc6bf8; continue; }
            pc = 0xc6ba9;
        }
        0xc6ba9 => {
            // block c6ba9
            // c6ba9 mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6bac jmp 0xc6bfa
            pc = 0xc6bfa; continue;
        }
        0xc6bae => {
            // block c6bae
            // c6bae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6bb0;
        }
        0xc6bb0 => {
            // block c6bb0
            // c6bb0 mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c6bb5 imul eax,r13d,0x811b
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x811bu64); rax = (r) as u32 as u64; fl = f; }
            // c6bbc add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6bbf add eax,0xfeeb2880
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfeeb2880u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6bc4 mov r8,rbx
            r8 = rbx;
            // c6bc7 js 0xc6bd5
            if fl.s() { pc = 0xc6bd5; continue; }
            pc = 0xc6bc9;
        }
        0xc6bc9 => {
            // block c6bc9
            // c6bc9 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6bce shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6bd1 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6bd3 jmp 0xc6be1
            pc = 0xc6be1; continue;
        }
        0xc6bd5 => {
            // block c6bd5
            // c6bd5 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c6bda sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6bdc sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6bdf neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6be1;
        }
        0xc6be1 => {
            // block c6be1
            // c6be1 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6be4 mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6be6 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6bec jg 0xc6c41
            if fl.g() { pc = 0xc6c41; continue; }
            pc = 0xc6bee;
        }
        0xc6bee => {
            // block c6bee
            // c6bee test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6bf1 js 0xc6c3f
            if fl.s() { pc = 0xc6c3f; continue; }
            pc = 0xc6bf3;
        }
        0xc6bf3 => {
            // block c6bf3
            // c6bf3 mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6bf6 jmp 0xc6c41
            pc = 0xc6c41; continue;
        }
        0xc6bf8 => {
            // block c6bf8
            // c6bf8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6bfa;
        }
        0xc6bfa => {
            // block c6bfa
            // c6bfa mov BYTE PTR [rsi+r12*4+0x2],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rax as u8 as u64)) as u8);
            // c6bff imul eax,r13d,0x8aa1
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x8aa1u64); rax = (r) as u32 as u64; fl = f; }
            // c6c06 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6c09 add eax,0xfed81140
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfed81140u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6c0e js 0xc6c1c
            if fl.s() { pc = 0xc6c1c; continue; }
            pc = 0xc6c10;
        }
        0xc6c10 => {
            // block c6c10
            // c6c10 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6c15 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6c18 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6c1a jmp 0xc6c28
            pc = 0xc6c28; continue;
        }
        0xc6c1c => {
            // block c6c1c
            // c6c1c mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c6c21 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c23 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6c26 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6c28;
        }
        0xc6c28 => {
            // block c6c28
            // c6c28 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6c2b mov al,0xff
            rax = (rax & !0xff) | ((0xffu64) as u8 as u64);
            // c6c2d cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6c33 jg 0xc6c65
            if fl.g() { pc = 0xc6c65; continue; }
            pc = 0xc6c35;
        }
        0xc6c35 => {
            // block c6c35
            // c6c35 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6c38 js 0xc6c63
            if fl.s() { pc = 0xc6c63; continue; }
            pc = 0xc6c3a;
        }
        0xc6c3a => {
            // block c6c3a
            // c6c3a mov al,dil
            rax = (rax & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // c6c3d jmp 0xc6c65
            pc = 0xc6c65; continue;
        }
        0xc6c3f => {
            // block c6c3f
            // c6c3f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6c41;
        }
        0xc6c41 => {
            // block c6c41
            // c6c41 mov BYTE PTR [rsi+r12*4],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rax as u8 as u64)) as u8);
            // c6c45 imul eax,r13d,0xffffe6ed
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffe6edu64); rax = (r) as u32 as u64; fl = f; }
            // c6c4c add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c6c4f imul edi,r9d,0xffffcbf8
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffffcbf8u64); rdi = (r) as u32 as u64; fl = f; }
            // c6c56 add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c59 add edi,0x879480
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x879480u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c5f jns 0xc6c85
            if fl.ns() { pc = 0xc6c85; continue; }
            pc = 0xc6c61;
        }
        0xc6c61 => {
            // block c6c61
            // c6c61 jmp 0xc6c92
            pc = 0xc6c92; continue;
        }
        0xc6c63 => {
            // block c6c63
            // c6c63 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6c65;
        }
        0xc6c65 => {
            // block c6c65
            // c6c65 mov BYTE PTR [rsi+r12*4],al
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rax as u8 as u64)) as u8);
            // c6c69 imul eax,r13d,0xfffff1ff
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xfffff1ffu64); rax = (r) as u32 as u64; fl = f; }
            // c6c70 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c6c73 imul edi,r9d,0xffffdcf4
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffffdcf4u64); rdi = (r) as u32 as u64; fl = f; }
            // c6c7a add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c7d add edi,0x4f6d40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x4f6d40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c83 js 0xc6c92
            if fl.s() { pc = 0xc6c92; continue; }
            pc = 0xc6c85;
        }
        0xc6c85 => {
            // block c6c85
            // c6c85 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6c8b shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6c8e mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6c90 jmp 0xc6c9e
            pc = 0xc6c9e; continue;
        }
        0xc6c92 => {
            // block c6c92
            // c6c92 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6c97 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6c99 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6c9c neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6c9e;
        }
        0xc6c9e => {
            // block c6c9e
            // c6c9e movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6ca1 mov dil,0xff
            rdi = (rdi & !0xff) | ((0xffu64) as u8 as u64);
            // c6ca4 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // c6caa jg 0xc6cb8
            if fl.g() { pc = 0xc6cb8; continue; }
            pc = 0xc6cac;
        }
        0xc6cac => {
            // block c6cac
            // c6cac test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6caf js 0xc6cb6
            if fl.s() { pc = 0xc6cb6; continue; }
            pc = 0xc6cb1;
        }
        0xc6cb1 => {
            // block c6cb1
            // c6cb1 mov dil,al
            rdi = (rdi & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c6cb4 jmp 0xc6cb8
            pc = 0xc6cb8; continue;
        }
        0xc6cb6 => {
            // block c6cb6
            // c6cb6 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0xc6cb8;
        }
        0xc6cb8 => {
            // block c6cb8
            // c6cb8 mov BYTE PTR [rsi+r12*4+0x1],dil
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdi as u8 as u64)) as u8);
            // c6cbd mov BYTE PTR [rsi+r12*4+0x3],0xac
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x3), (0xacu64) as u8);
            // c6cc3 cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c6cc6 mov r11w,r15w
            r11 = (r11 & !0xffff) | (((r15 as u16 as u64)) as u16 as u64);
            // c6cca mov r13w,r14w
            r13 = (r13 & !0xffff) | (((r14 as u16 as u64)) as u16 as u64);
            // c6cce jb 0xc68ec
            if fl.b() { pc = 0xc68ec; continue; }
            pc = 0xc6cd4;
        }
        0xc6cd4 => {
            // block c6cd4
            // c6cd4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c6cd5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c6cd7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c6cd9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c6cdb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c6cdd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c6cde ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc6487 size=1070
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c6487<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc6487;
    loop {
        match pc {
        0xc6487 => {
            // block c6487
            // c6487 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c6488 mov rbp,rsp
            rbp = rsp;
            // c648b push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c648d push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c648f push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c6491 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c6493 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c6494 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c6496 je 0xc68aa
            if fl.e() { pc = 0xc68aa; continue; }
            pc = 0xc649c;
        }
        0xc649c => {
            // block c649c
            // c649c movzx r11d,WORD PTR [rdi+0x4]
            r11 = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // c64a1 movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // c64a5 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c64a8 movzx r13d,WORD PTR [rdi]
            r13 = (m.r16(rdi) as u64) as u32 as u64;
            // c64ac lea r8,[rdi+0x6]
            r8 = rdi.wrapping_add(0x6);
            // c64b0 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc64b3;
        }
        0xc64b3 => {
            // block c64b3
            // c64b3 mov r12d,r10d
            r12 = ((r10 as u32 as u64)) as u32 as u64;
            // c64b6 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // c64b9 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // c64bc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c64be je 0xc64eb
            if fl.e() { pc = 0xc64eb; continue; }
            pc = 0xc64c0;
        }
        0xc64c0 => {
            // block c64c0
            // c64c0 imul r15d,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); r15 = (r) as u32 as u64; fl = f; }
            // c64c7 movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c64cb imul edi,r10d,0x19895
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x19895u64); rdi = (r) as u32 as u64; fl = f; }
            // c64d2 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c64d5 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c64d7 add eax,0xfc8450c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfc8450c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c64dc js 0xc6516
            if fl.s() { pc = 0xc6516; continue; }
            pc = 0xc64de;
        }
        0xc64de => {
            // block c64de
            // c64de add edi,0xfc84d0c0
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfc84d0c0u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c64e4 shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c64e7 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c64e9 jmp 0xc6522
            pc = 0xc6522; continue;
        }
        0xc64eb => {
            // block c64eb
            // c64eb imul r15d,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); r15 = (r) as u32 as u64; fl = f; }
            // c64f2 movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c64f6 imul eax,r10d,0x1df71
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x1df71u64); rax = (r) as u32 as u64; fl = f; }
            // c64fd add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6500 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6502 add edi,0xfbf66a40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfbf66a40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6508 js 0xc6564
            if fl.s() { pc = 0xc6564; continue; }
            pc = 0xc650a;
        }
        0xc650a => {
            // block c650a
            // c650a add eax,0xfbf6ea40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbf6ea40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c650f shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6512 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6514 jmp 0xc6570
            pc = 0xc6570; continue;
        }
        0xc6516 => {
            // block c6516
            // c6516 mov eax,0x37c2f40
            rax = (0x37c2f40u64) as u32 as u64;
            // c651b sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c651d sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6520 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6522;
        }
        0xc6522 => {
            // block c6522
            // c6522 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6525 mov di,0x3ff
            rdi = (rdi & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6529 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c652f jg 0xc653d
            if fl.g() { pc = 0xc653d; continue; }
            pc = 0xc6531;
        }
        0xc6531 => {
            // block c6531
            // c6531 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6534 mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // c6539 cmovns di,ax
            if fl.ns() { rdi = (rdi & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc653d;
        }
        0xc653d => {
            // block c653d
            // c653d mov WORD PTR [rsi+r12*8+0x4],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((rdi as u16 as u64)) as u16);
            // c6543 movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c6547 imul eax,r14d,0x20469
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x20469u64); rax = (r) as u32 as u64; fl = f; }
            // c654e add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6551 add eax,0xfbaca8c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbaca8c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6556 js 0xc65b2
            if fl.s() { pc = 0xc65b2; continue; }
            pc = 0xc6558;
        }
        0xc6558 => {
            // block c6558
            // c6558 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c655d shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6560 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6562 jmp 0xc65be
            pc = 0xc65be; continue;
        }
        0xc6564 => {
            // block c6564
            // c6564 mov edi,0x40a15c0
            rdi = (0x40a15c0u64) as u32 as u64;
            // c6569 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c656b sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c656e neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6570;
        }
        0xc6570 => {
            // block c6570
            // c6570 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c6573 mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6577 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c657d jg 0xc658b
            if fl.g() { pc = 0xc658b; continue; }
            pc = 0xc657f;
        }
        0xc657f => {
            // block c657f
            // c657f test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6582 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c6587 cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc658b;
        }
        0xc658b => {
            // block c658b
            // c658b mov WORD PTR [rsi+r12*8+0x4],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c6591 movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c6595 imul eax,r14d,0x22a86
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x22a86u64); rax = (r) as u32 as u64; fl = f; }
            // c659c add eax,r15d
            { let (r, f) = add::<32>((rax as u32 as u64), (r15 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c659f add eax,0xfb604040
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfb604040u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c65a4 js 0xc65fc
            if fl.s() { pc = 0xc65fc; continue; }
            pc = 0xc65a6;
        }
        0xc65a6 => {
            // block c65a6
            // c65a6 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c65ab shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c65ae mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c65b0 jmp 0xc6608
            pc = 0xc6608; continue;
        }
        0xc65b2 => {
            // block c65b2
            // c65b2 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c65b7 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c65b9 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c65bc neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc65be;
        }
        0xc65be => {
            // block c65be
            // c65be movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c65c1 mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c65c5 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c65cb jg 0xc65d9
            if fl.g() { pc = 0xc65d9; continue; }
            pc = 0xc65cd;
        }
        0xc65cd => {
            // block c65cd
            // c65cd test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c65d0 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c65d5 cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc65d9;
        }
        0xc65d9 => {
            // block c65d9
            // c65d9 mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c65de imul eax,r14d,0xffff9bb6
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // c65e5 add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c65e8 imul edi,r10d,0xffff2fe1
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffff2fe1u64); rdi = (r) as u32 as u64; fl = f; }
            // c65ef add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c65f2 add edi,0x21e4cc0
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x21e4cc0u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c65f8 jns 0xc6644
            if fl.ns() { pc = 0xc6644; continue; }
            pc = 0xc65fa;
        }
        0xc65fa => {
            // block c65fa
            // c65fa jmp 0xc6651
            pc = 0xc6651; continue;
        }
        0xc65fc => {
            // block c65fc
            // c65fc mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c6601 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6603 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6606 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6608;
        }
        0xc6608 => {
            // block c6608
            // c6608 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c660b mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c660f cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c6615 jg 0xc6623
            if fl.g() { pc = 0xc6623; continue; }
            pc = 0xc6617;
        }
        0xc6617 => {
            // block c6617
            // c6617 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c661a mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c661f cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc6623;
        }
        0xc6623 => {
            // block c6623
            // c6623 mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c6628 imul eax,r14d,0xffffc7fa
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // c662f add r15d,eax
            { let (r, f) = add::<32>((r15 as u32 as u64), (rax as u32 as u64));
              r15 = (r) as u32 as u64;
              fl = f; }
            // c6632 imul edi,r10d,0xffff73ce
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffff73ceu64); rdi = (r) as u32 as u64; fl = f; }
            // c6639 add edi,r15d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r15 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c663c add edi,0x13dbc40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x13dbc40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6642 js 0xc6651
            if fl.s() { pc = 0xc6651; continue; }
            pc = 0xc6644;
        }
        0xc6644 => {
            // block c6644
            // c6644 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c664a shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c664d mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c664f jmp 0xc665d
            pc = 0xc665d; continue;
        }
        0xc6651 => {
            // block c6651
            // c6651 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6656 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6658 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c665b neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc665d;
        }
        0xc665d => {
            // block c665d
            // c665d movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6660 mov di,0x3ff
            rdi = (rdi & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6664 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c666a jg 0xc6678
            if fl.g() { pc = 0xc6678; continue; }
            pc = 0xc666c;
        }
        0xc666c => {
            // block c666c
            // c666c test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c666f mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // c6674 cmovns di,ax
            if fl.ns() { rdi = (rdi & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc6678;
        }
        0xc6678 => {
            // block c6678
            // c6678 mov WORD PTR [rsi+r12*8+0x2],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2), ((rdi as u16 as u64)) as u16);
            // c667e mov WORD PTR [rsi+r12*8+0x6],0x3ac
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x6), (0x3acu64) as u16);
            // c6686 movzx edi,WORD PTR [r8]
            rdi = (m.r16(r8) as u64) as u32 as u64;
            // c668a lea r10d,[r12+0x2]
            r10 = (r12.wrapping_add(0x2)) as u32 as u64;
            // c668f cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c6692 jae 0xc66ac
            if fl.ae() { pc = 0xc66ac; continue; }
            pc = 0xc6694;
        }
        0xc6694 => {
            // block c6694
            // c6694 movzx r14d,WORD PTR [r8+0x2]
            r14 = (m.r16(r8.wrapping_add(0x2)) as u64) as u32 as u64;
            // c6699 movzx eax,WORD PTR [r8+0x4]
            rax = (m.r16(r8.wrapping_add(0x4)) as u64) as u32 as u64;
            // c669e mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // c66a1 movzx r15d,WORD PTR [r8+0x6]
            r15 = (m.r16(r8.wrapping_add(0x6)) as u64) as u32 as u64;
            // c66a6 add r8,0x8
            { let (r, f) = add::<64>(r8, 0x8u64);
              r8 = r;
              fl = f; }
            // c66aa jmp 0xc66b8
            pc = 0xc66b8; continue;
        }
        0xc66ac => {
            // block c66ac
            // c66ac add r8,0x2
            { let (r, f) = add::<64>(r8, 0x2u64);
              r8 = r;
              fl = f; }
            // c66b0 mov r14w,r13w
            r14 = (r14 & !0xffff) | (((r13 as u16 as u64)) as u16 as u64);
            // c66b4 mov r15w,r11w
            r15 = (r15 & !0xffff) | (((r11 as u16 as u64)) as u16 as u64);
            pc = 0xc66b8;
        }
        0xc66b8 => {
            // block c66b8
            // c66b8 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // c66bc movzx r13d,r14w
            r13 = ((r14 as u16 as u64)) as u32 as u64;
            // c66c0 add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // c66c3 shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // c66c6 movzx eax,r11w
            rax = ((r11 as u16 as u64)) as u32 as u64;
            // c66ca movzx r9d,r15w
            r9 = ((r15 as u16 as u64)) as u32 as u64;
            // c66ce add r9d,eax
            { let (r, f) = add::<32>((r9 as u32 as u64), (rax as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // c66d1 shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // c66d4 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // c66d7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c66d9 je 0xc6705
            if fl.e() { pc = 0xc6705; continue; }
            pc = 0xc66db;
        }
        0xc66db => {
            // block c66db
            // c66db imul r11d,edi,0x12a15
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12a15u64); r11 = (r) as u32 as u64; fl = f; }
            // c66e2 imul eax,r9d,0x19895
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19895u64); rax = (r) as u32 as u64; fl = f; }
            // c66e9 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c66ec mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c66ee mov rbx,r8
            rbx = r8;
            // c66f1 add edi,0xfc8450c0
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfc8450c0u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c66f7 js 0xc672c
            if fl.s() { pc = 0xc672c; continue; }
            pc = 0xc66f9;
        }
        0xc66f9 => {
            // block c66f9
            // c66f9 add eax,0xfc84d0c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfc84d0c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c66fe shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6701 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6703 jmp 0xc6738
            pc = 0xc6738; continue;
        }
        0xc6705 => {
            // block c6705
            // c6705 imul r11d,edi,0x12acf
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12acfu64); r11 = (r) as u32 as u64; fl = f; }
            // c670c imul eax,r9d,0x1df71
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x1df71u64); rax = (r) as u32 as u64; fl = f; }
            // c6713 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6716 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6718 add edi,0xfbf66a40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfbf66a40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c671e js 0xc677b
            if fl.s() { pc = 0xc677b; continue; }
            pc = 0xc6720;
        }
        0xc6720 => {
            // block c6720
            // c6720 add eax,0xfbf6ea40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbf6ea40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6725 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6728 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c672a jmp 0xc6787
            pc = 0xc6787; continue;
        }
        0xc672c => {
            // block c672c
            // c672c mov edi,0x37c2f40
            rdi = (0x37c2f40u64) as u32 as u64;
            // c6731 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6733 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6736 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6738;
        }
        0xc6738 => {
            // block c6738
            // c6738 movsx r8d,di
            r8 = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c673c mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6740 cmp r8d,0x3ff
            fl = sub::<32>((r8 as u32 as u64), 0x3ffu64).1;
            // c6747 jg 0xc6755
            if fl.g() { pc = 0xc6755; continue; }
            pc = 0xc6749;
        }
        0xc6749 => {
            // block c6749
            // c6749 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c674c mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c6751 cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc6755;
        }
        0xc6755 => {
            // block c6755
            // c6755 mov WORD PTR [rsi+r12*8+0x4],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c675b imul eax,r13d,0x20469
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x20469u64); rax = (r) as u32 as u64; fl = f; }
            // c6762 add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c6765 add eax,0xfbaca8c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbaca8c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c676a mov r8,rbx
            r8 = rbx;
            // c676d js 0xc67c5
            if fl.s() { pc = 0xc67c5; continue; }
            pc = 0xc676f;
        }
        0xc676f => {
            // block c676f
            // c676f add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6774 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c6777 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c6779 jmp 0xc67d1
            pc = 0xc67d1; continue;
        }
        0xc677b => {
            // block c677b
            // c677b mov edi,0x40a15c0
            rdi = (0x40a15c0u64) as u32 as u64;
            // c6780 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6782 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6785 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc6787;
        }
        0xc6787 => {
            // block c6787
            // c6787 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c678a mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c678e cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c6794 jg 0xc67a2
            if fl.g() { pc = 0xc67a2; continue; }
            pc = 0xc6796;
        }
        0xc6796 => {
            // block c6796
            // c6796 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c6799 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c679e cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc67a2;
        }
        0xc67a2 => {
            // block c67a2
            // c67a2 mov WORD PTR [rsi+r12*8+0x4],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c67a8 imul eax,r13d,0x22a86
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x22a86u64); rax = (r) as u32 as u64; fl = f; }
            // c67af add eax,r11d
            { let (r, f) = add::<32>((rax as u32 as u64), (r11 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c67b2 add eax,0xfb604040
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfb604040u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c67b7 js 0xc680f
            if fl.s() { pc = 0xc680f; continue; }
            pc = 0xc67b9;
        }
        0xc67b9 => {
            // block c67b9
            // c67b9 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c67be shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c67c1 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c67c3 jmp 0xc681b
            pc = 0xc681b; continue;
        }
        0xc67c5 => {
            // block c67c5
            // c67c5 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c67ca sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c67cc sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c67cf neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc67d1;
        }
        0xc67d1 => {
            // block c67d1
            // c67d1 movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c67d4 mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c67d8 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c67de jg 0xc67ec
            if fl.g() { pc = 0xc67ec; continue; }
            pc = 0xc67e0;
        }
        0xc67e0 => {
            // block c67e0
            // c67e0 test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c67e3 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c67e8 cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc67ec;
        }
        0xc67ec => {
            // block c67ec
            // c67ec mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c67f1 imul eax,r13d,0xffff9bb6
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // c67f8 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c67fb imul edi,r9d,0xffff2fe1
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff2fe1u64); rdi = (r) as u32 as u64; fl = f; }
            // c6802 add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6805 add edi,0x21e4cc0
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x21e4cc0u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c680b jns 0xc6857
            if fl.ns() { pc = 0xc6857; continue; }
            pc = 0xc680d;
        }
        0xc680d => {
            // block c680d
            // c680d jmp 0xc6864
            pc = 0xc6864; continue;
        }
        0xc680f => {
            // block c680f
            // c680f mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c6814 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6816 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6819 neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc681b;
        }
        0xc681b => {
            // block c681b
            // c681b movsx ebx,di
            rbx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c681e mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6822 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c6828 jg 0xc6836
            if fl.g() { pc = 0xc6836; continue; }
            pc = 0xc682a;
        }
        0xc682a => {
            // block c682a
            // c682a test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c682d mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c6832 cmovns ax,di
            if fl.ns() { rax = (rax & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc6836;
        }
        0xc6836 => {
            // block c6836
            // c6836 mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c683b imul eax,r13d,0xffffc7fa
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // c6842 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // c6845 imul edi,r9d,0xffff73ce
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff73ceu64); rdi = (r) as u32 as u64; fl = f; }
            // c684c add edi,r11d
            { let (r, f) = add::<32>((rdi as u32 as u64), (r11 as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c684f add edi,0x13dbc40
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x13dbc40u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c6855 js 0xc6864
            if fl.s() { pc = 0xc6864; continue; }
            pc = 0xc6857;
        }
        0xc6857 => {
            // block c6857
            // c6857 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c685d shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c6860 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // c6862 jmp 0xc6870
            pc = 0xc6870; continue;
        }
        0xc6864 => {
            // block c6864
            // c6864 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c6869 sub eax,edi
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c686b sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c686e neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc6870;
        }
        0xc6870 => {
            // block c6870
            // c6870 movsx ebx,ax
            rbx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c6873 mov di,0x3ff
            rdi = (rdi & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c6877 cmp ebx,0x3ff
            fl = sub::<32>((rbx as u32 as u64), 0x3ffu64).1;
            // c687d jg 0xc688b
            if fl.g() { pc = 0xc688b; continue; }
            pc = 0xc687f;
        }
        0xc687f => {
            // block c687f
            // c687f test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c6882 mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // c6887 cmovns di,ax
            if fl.ns() { rdi = (rdi & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc688b;
        }
        0xc688b => {
            // block c688b
            // c688b mov WORD PTR [rsi+r12*8+0x2],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2), ((rdi as u16 as u64)) as u16);
            // c6891 mov WORD PTR [rsi+r12*8+0x6],0x3ac
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x6), (0x3acu64) as u16);
            // c6899 cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c689c mov r11w,r15w
            r11 = (r11 & !0xffff) | (((r15 as u16 as u64)) as u16 as u64);
            // c68a0 mov r13w,r14w
            r13 = (r13 & !0xffff) | (((r14 as u16 as u64)) as u16 as u64);
            // c68a4 jb 0xc64b3
            if fl.b() { pc = 0xc64b3; continue; }
            pc = 0xc68aa;
        }
        0xc68aa => {
            // block c68aa
            // c68aa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c68ab pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c68ad pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c68af pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c68b1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c68b3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c68b4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc70f4 size=1253
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c70f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc70f4;
    loop {
        match pc {
        0xc70f4 => {
            // block c70f4
            // c70f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c70f5 mov rbp,rsp
            rbp = rsp;
            // c70f8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c70fa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c70fc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c70fe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c7100 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7101 mov r8d,ecx
            r8 = ((rcx as u32 as u64)) as u32 as u64;
            // c7104 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c7106 je 0xc75ce
            if fl.e() { pc = 0xc75ce; continue; }
            pc = 0xc710c;
        }
        0xc710c => {
            // block c710c
            // c710c movzx r11d,WORD PTR [rdi+0x4]
            r11 = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // c7111 movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // c7115 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // c7118 movzx r13d,WORD PTR [rdi]
            r13 = (m.r16(rdi) as u64) as u32 as u64;
            // c711c lea r9,[rdi+0x6]
            r9 = rdi.wrapping_add(0x6);
            // c7120 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            pc = 0xc7123;
        }
        0xc7123 => {
            // block c7123
            // c7123 mov r12d,r10d
            r12 = ((r10 as u32 as u64)) as u32 as u64;
            // c7126 mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // c7129 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // c712c test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c712f je 0xc715a
            if fl.e() { pc = 0xc715a; continue; }
            pc = 0xc7131;
        }
        0xc7131 => {
            // block c7131
            // c7131 imul ecx,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); rcx = (r) as u32 as u64; fl = f; }
            // c7137 movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c713b imul ebx,r10d,0x19895
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x19895u64); rbx = (r) as u32 as u64; fl = f; }
            // c7142 add ebx,ecx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rcx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c7144 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // c7146 add eax,0xfc8450c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfc8450c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c714b js 0xc7183
            if fl.s() { pc = 0xc7183; continue; }
            pc = 0xc714d;
        }
        0xc714d => {
            // block c714d
            // c714d add ebx,0xfc84d0c0
            { let (r, f) = add::<32>((rbx as u32 as u64), 0xfc84d0c0u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // c7153 shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c7156 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // c7158 jmp 0xc718f
            pc = 0xc718f; continue;
        }
        0xc715a => {
            // block c715a
            // c715a imul edi,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); rdi = (r) as u32 as u64; fl = f; }
            // c7160 movzx r10d,r11w
            r10 = ((r11 as u16 as u64)) as u32 as u64;
            // c7164 imul ecx,r10d,0x1df71
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0x1df71u64); rcx = (r) as u32 as u64; fl = f; }
            // c716b add ecx,edi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c716d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c716f add eax,0xfbf66a40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbf66a40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c7174 js 0xc71d3
            if fl.s() { pc = 0xc71d3; continue; }
            pc = 0xc7176;
        }
        0xc7176 => {
            // block c7176
            // c7176 add ecx,0xfbf6ea40
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfbf6ea40u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c717c shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c717f mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c7181 jmp 0xc71df
            pc = 0xc71df; continue;
        }
        0xc7183 => {
            // block c7183
            // c7183 mov eax,0x37c2f40
            rax = (0x37c2f40u64) as u32 as u64;
            // c7188 sub eax,ebx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c718a sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c718d neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc718f;
        }
        0xc718f => {
            // block c718f
            // c718f movsx edi,ax
            rdi = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c7192 mov r15w,0x3ff
            r15 = (r15 & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7197 cmp edi,0x3ff
            fl = sub::<32>((rdi as u32 as u64), 0x3ffu64).1;
            // c719d jg 0xc71ad
            if fl.g() { pc = 0xc71ad; continue; }
            pc = 0xc719f;
        }
        0xc719f => {
            // block c719f
            // c719f test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c71a2 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // c71a8 cmovns r15w,ax
            if fl.ns() { r15 = (r15 & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc71ad;
        }
        0xc71ad => {
            // block c71ad
            // c71ad mov WORD PTR [rsi+r12*8+0x4],r15w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r15 as u16 as u64)) as u16);
            // c71b3 movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c71b7 imul eax,r14d,0x20469
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x20469u64); rax = (r) as u32 as u64; fl = f; }
            // c71be add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c71c0 add eax,0xfbaca8c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbaca8c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c71c5 js 0xc7223
            if fl.s() { pc = 0xc7223; continue; }
            pc = 0xc71c7;
        }
        0xc71c7 => {
            // block c71c7
            // c71c7 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c71cc shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c71cf mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // c71d1 jmp 0xc722f
            pc = 0xc722f; continue;
        }
        0xc71d3 => {
            // block c71d3
            // c71d3 mov eax,0x40a15c0
            rax = (0x40a15c0u64) as u32 as u64;
            // c71d8 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c71da sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c71dd neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc71df;
        }
        0xc71df => {
            // block c71df
            // c71df movsx ecx,ax
            rcx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c71e2 mov r15w,0x3ff
            r15 = (r15 & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c71e7 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // c71ed jg 0xc71fd
            if fl.g() { pc = 0xc71fd; continue; }
            pc = 0xc71ef;
        }
        0xc71ef => {
            // block c71ef
            // c71ef test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c71f2 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // c71f8 cmovns r15w,ax
            if fl.ns() { r15 = (r15 & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc71fd;
        }
        0xc71fd => {
            // block c71fd
            // c71fd mov WORD PTR [rsi+r12*8+0x4],r15w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r15 as u16 as u64)) as u16);
            // c7203 movzx r14d,r13w
            r14 = ((r13 as u16 as u64)) as u32 as u64;
            // c7207 imul eax,r14d,0x22a86
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0x22a86u64); rax = (r) as u32 as u64; fl = f; }
            // c720e add eax,edi
            { let (r, f) = add::<32>((rax as u32 as u64), (rdi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7210 add eax,0xfb604040
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfb604040u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c7215 js 0xc7276
            if fl.s() { pc = 0xc7276; continue; }
            pc = 0xc7217;
        }
        0xc7217 => {
            // block c7217
            // c7217 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c721c shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c721f mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // c7221 jmp 0xc7282
            pc = 0xc7282; continue;
        }
        0xc7223 => {
            // block c7223
            // c7223 mov ebx,0x8000
            rbx = (0x8000u64) as u32 as u64;
            // c7228 sub ebx,eax
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c722a sar ebx,0x10
            { let (r, f) = sar::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c722d neg ebx
            { let (r, f) = neg::<32>((rbx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            pc = 0xc722f;
        }
        0xc722f => {
            // block c722f
            // c722f movsx edi,bx
            rdi = (sx::<16>((rbx as u16 as u64)) as u64) as u32 as u64;
            // c7232 mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7236 cmp edi,0x3ff
            fl = sub::<32>((rdi as u32 as u64), 0x3ffu64).1;
            // c723c jg 0xc724a
            if fl.g() { pc = 0xc724a; continue; }
            pc = 0xc723e;
        }
        0xc723e => {
            // block c723e
            // c723e test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // c7241 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c7246 cmovns ax,bx
            if fl.ns() { rax = (rax & !0xffff) | (((rbx as u16 as u64)) as u16 as u64); }
            pc = 0xc724a;
        }
        0xc724a => {
            // block c724a
            // c724a mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c724f imul edi,r14d,0xffff9bb6
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffff9bb6u64); rdi = (r) as u32 as u64; fl = f; }
            // c7256 add ecx,edi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7258 imul edi,r10d,0xffff2fe1
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffff2fe1u64); rdi = (r) as u32 as u64; fl = f; }
            // c725f add edi,ecx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rcx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7261 add edi,0x21e4cc0
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x21e4cc0u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7267 js 0xc72c9
            if fl.s() { pc = 0xc72c9; continue; }
            pc = 0xc7269;
        }
        0xc7269 => {
            // block c7269
            // c7269 add edi,0x8000
            { let (r, f) = add::<32>((rdi as u32 as u64), 0x8000u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c726f shr edi,0x10
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c7272 mov ecx,edi
            rcx = ((rdi as u32 as u64)) as u32 as u64;
            // c7274 jmp 0xc72d5
            pc = 0xc72d5; continue;
        }
        0xc7276 => {
            // block c7276
            // c7276 mov ebx,0x8000
            rbx = (0x8000u64) as u32 as u64;
            // c727b sub ebx,eax
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c727d sar ebx,0x10
            { let (r, f) = sar::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c7280 neg ebx
            { let (r, f) = neg::<32>((rbx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            pc = 0xc7282;
        }
        0xc7282 => {
            // block c7282
            // c7282 movsx ecx,bx
            rcx = (sx::<16>((rbx as u16 as u64)) as u64) as u32 as u64;
            // c7285 mov ax,0x3ff
            rax = (rax & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7289 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // c728f jg 0xc729d
            if fl.g() { pc = 0xc729d; continue; }
            pc = 0xc7291;
        }
        0xc7291 => {
            // block c7291
            // c7291 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // c7294 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // c7299 cmovns ax,bx
            if fl.ns() { rax = (rax & !0xffff) | (((rbx as u16 as u64)) as u16 as u64); }
            pc = 0xc729d;
        }
        0xc729d => {
            // block c729d
            // c729d mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c72a2 imul ecx,r14d,0xffffc7fa
            { let (r, f) = imul::<32>((r14 as u32 as u64), 0xffffc7fau64); rcx = (r) as u32 as u64; fl = f; }
            // c72a9 add edi,ecx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rcx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c72ab imul ecx,r10d,0xffff73ce
            { let (r, f) = imul::<32>((r10 as u32 as u64), 0xffff73ceu64); rcx = (r) as u32 as u64; fl = f; }
            // c72b2 add ecx,edi
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c72b4 add ecx,0x13dbc40
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x13dbc40u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c72ba js 0xc72f2
            if fl.s() { pc = 0xc72f2; continue; }
            pc = 0xc72bc;
        }
        0xc72bc => {
            // block c72bc
            // c72bc add ecx,0x8000
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x8000u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c72c2 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c72c5 mov edi,ecx
            rdi = ((rcx as u32 as u64)) as u32 as u64;
            // c72c7 jmp 0xc72fe
            pc = 0xc72fe; continue;
        }
        0xc72c9 => {
            // block c72c9
            // c72c9 mov ecx,0x8000
            rcx = (0x8000u64) as u32 as u64;
            // c72ce sub ecx,edi
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rdi as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c72d0 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c72d3 neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            pc = 0xc72d5;
        }
        0xc72d5 => {
            // block c72d5
            // c72d5 movsx edi,cx
            rdi = (sx::<16>((rcx as u16 as u64)) as u64) as u32 as u64;
            // c72d8 mov bx,0x3ff
            rbx = (rbx & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c72dc cmp edi,0x3ff
            fl = sub::<32>((rdi as u32 as u64), 0x3ffu64).1;
            // c72e2 jg 0xc7319
            if fl.g() { pc = 0xc7319; continue; }
            pc = 0xc72e4;
        }
        0xc72e4 => {
            // block c72e4
            // c72e4 test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // c72e7 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // c72ec cmovns bx,cx
            if fl.ns() { rbx = (rbx & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            // c72f0 jmp 0xc7319
            pc = 0xc7319; continue;
        }
        0xc72f2 => {
            // block c72f2
            // c72f2 mov edi,0x8000
            rdi = (0x8000u64) as u32 as u64;
            // c72f7 sub edi,ecx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rcx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c72f9 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c72fc neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc72fe;
        }
        0xc72fe => {
            // block c72fe
            // c72fe movsx ecx,di
            rcx = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c7301 mov bx,0x3ff
            rbx = (rbx & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7305 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // c730b jg 0xc7319
            if fl.g() { pc = 0xc7319; continue; }
            pc = 0xc730d;
        }
        0xc730d => {
            // block c730d
            // c730d test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c7310 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // c7315 cmovns bx,di
            if fl.ns() { rbx = (rbx & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc7319;
        }
        0xc7319 => {
            // block c7319
            // c7319 mov WORD PTR [rsi+r12*8+0x6],0x3ac
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x6), (0x3acu64) as u16);
            // c7321 shl r15d,0x6
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x6u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // c7325 mov WORD PTR [rsi+r12*8+0x4],r15w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r15 as u16 as u64)) as u16);
            // c732b shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c732e mov WORD PTR [rsi+r12*8+0x2],bx
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2), ((rbx as u16 as u64)) as u16);
            // c7334 shl eax,0x6
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x6u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7337 mov WORD PTR [rsi+r12*8],ax
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rax as u16 as u64)) as u16);
            // c733c movzx edi,WORD PTR [r9]
            rdi = (m.r16(r9) as u64) as u32 as u64;
            // c7340 lea r10d,[r12+0x2]
            r10 = (r12.wrapping_add(0x2)) as u32 as u64;
            // c7345 cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c7348 jae 0xc7366
            if fl.ae() { pc = 0xc7366; continue; }
            pc = 0xc734a;
        }
        0xc734a => {
            // block c734a
            // c734a movzx r14d,WORD PTR [r9+0x2]
            r14 = (m.r16(r9.wrapping_add(0x2)) as u64) as u32 as u64;
            // c734f movzx eax,WORD PTR [r9+0x4]
            rax = (m.r16(r9.wrapping_add(0x4)) as u64) as u32 as u64;
            // c7354 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // c7357 movzx r15d,WORD PTR [r9+0x6]
            r15 = (m.r16(r9.wrapping_add(0x6)) as u64) as u32 as u64;
            // c735c add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c7360 mov QWORD PTR [rbp-0x30],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r9) as u64);
            // c7364 jmp 0xc7376
            pc = 0xc7376; continue;
        }
        0xc7366 => {
            // block c7366
            // c7366 add r9,0x2
            { let (r, f) = add::<64>(r9, 0x2u64);
              r9 = r;
              fl = f; }
            // c736a mov QWORD PTR [rbp-0x30],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r9) as u64);
            // c736e mov r14w,r13w
            r14 = (r14 & !0xffff) | (((r13 as u16 as u64)) as u16 as u64);
            // c7372 mov r15w,r11w
            r15 = (r15 & !0xffff) | (((r11 as u16 as u64)) as u16 as u64);
            pc = 0xc7376;
        }
        0xc7376 => {
            // block c7376
            // c7376 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // c737a movzx r9d,r14w
            r9 = ((r14 as u16 as u64)) as u32 as u64;
            // c737e add r9d,eax
            { let (r, f) = add::<32>((r9 as u32 as u64), (rax as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // c7381 shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // c7384 movzx eax,r11w
            rax = ((r11 as u16 as u64)) as u32 as u64;
            // c7388 movzx ebx,r15w
            rbx = ((r15 as u16 as u64)) as u32 as u64;
            // c738c add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // c738e shr ebx,1
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x1u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c7390 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // c7393 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c7396 je 0xc73be
            if fl.e() { pc = 0xc73be; continue; }
            pc = 0xc7398;
        }
        0xc7398 => {
            // block c7398
            // c7398 imul r13d,edi,0x12a15
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12a15u64); r13 = (r) as u32 as u64; fl = f; }
            // c739f imul eax,ebx,0x19895
            { let (r, f) = imul::<32>((rbx as u32 as u64), 0x19895u64); rax = (r) as u32 as u64; fl = f; }
            // c73a5 add eax,r13d
            { let (r, f) = add::<32>((rax as u32 as u64), (r13 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c73a8 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c73aa add ecx,0xfc8450c0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfc8450c0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c73b0 js 0xc73e4
            if fl.s() { pc = 0xc73e4; continue; }
            pc = 0xc73b2;
        }
        0xc73b2 => {
            // block c73b2
            // c73b2 add eax,0xfc84d0c0
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfc84d0c0u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c73b7 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c73ba mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c73bc jmp 0xc73f0
            pc = 0xc73f0; continue;
        }
        0xc73be => {
            // block c73be
            // c73be imul r13d,edi,0x12acf
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x12acfu64); r13 = (r) as u32 as u64; fl = f; }
            // c73c5 imul eax,ebx,0x1df71
            { let (r, f) = imul::<32>((rbx as u32 as u64), 0x1df71u64); rax = (r) as u32 as u64; fl = f; }
            // c73cb add eax,r13d
            { let (r, f) = add::<32>((rax as u32 as u64), (r13 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c73ce mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c73d0 add ecx,0xfbf66a40
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfbf66a40u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c73d6 js 0xc7432
            if fl.s() { pc = 0xc7432; continue; }
            pc = 0xc73d8;
        }
        0xc73d8 => {
            // block c73d8
            // c73d8 add eax,0xfbf6ea40
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfbf6ea40u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c73dd shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c73e0 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // c73e2 jmp 0xc743e
            pc = 0xc743e; continue;
        }
        0xc73e4 => {
            // block c73e4
            // c73e4 mov edi,0x37c2f40
            rdi = (0x37c2f40u64) as u32 as u64;
            // c73e9 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c73eb sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c73ee neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc73f0;
        }
        0xc73f0 => {
            // block c73f0
            // c73f0 movsx eax,di
            rax = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c73f3 mov r11w,0x3ff
            r11 = (r11 & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c73f8 cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // c73fd jg 0xc740d
            if fl.g() { pc = 0xc740d; continue; }
            pc = 0xc73ff;
        }
        0xc73ff => {
            // block c73ff
            // c73ff test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c7402 mov r11d,0x0
            r11 = (0x0u64) as u32 as u64;
            // c7408 cmovns r11w,di
            if fl.ns() { r11 = (r11 & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc740d;
        }
        0xc740d => {
            // block c740d
            // c740d mov WORD PTR [rsi+r12*8+0x4],r11w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r11 as u16 as u64)) as u16);
            // c7413 imul ecx,r9d,0x20469
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x20469u64); rcx = (r) as u32 as u64; fl = f; }
            // c741a add ecx,r13d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r13 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c741d add ecx,0xfbaca8c0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfbaca8c0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7423 js 0xc748f
            if fl.s() { pc = 0xc748f; continue; }
            pc = 0xc7425;
        }
        0xc7425 => {
            // block c7425
            // c7425 add ecx,0x8000
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x8000u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c742b shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c742e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c7430 jmp 0xc749b
            pc = 0xc749b; continue;
        }
        0xc7432 => {
            // block c7432
            // c7432 mov edi,0x40a15c0
            rdi = (0x40a15c0u64) as u32 as u64;
            // c7437 sub edi,eax
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rax as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // c7439 sar edi,0x10
            { let (r, f) = sar::<32>((rdi as u32 as u64), 0x10u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c743c neg edi
            { let (r, f) = neg::<32>((rdi as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            pc = 0xc743e;
        }
        0xc743e => {
            // block c743e
            // c743e movsx eax,di
            rax = (sx::<16>((rdi as u16 as u64)) as u64) as u32 as u64;
            // c7441 mov r11w,0x3ff
            r11 = (r11 & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7446 cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // c744b jg 0xc745b
            if fl.g() { pc = 0xc745b; continue; }
            pc = 0xc744d;
        }
        0xc744d => {
            // block c744d
            // c744d test di,di
            fl = logic::<16>((rdi as u16 as u64) & (rdi as u16 as u64));
            // c7450 mov r11d,0x0
            r11 = (0x0u64) as u32 as u64;
            // c7456 cmovns r11w,di
            if fl.ns() { r11 = (r11 & !0xffff) | (((rdi as u16 as u64)) as u16 as u64); }
            pc = 0xc745b;
        }
        0xc745b => {
            // block c745b
            // c745b mov WORD PTR [rsi+r12*8+0x4],r11w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r11 as u16 as u64)) as u16);
            // c7461 imul eax,r9d,0x22a86
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x22a86u64); rax = (r) as u32 as u64; fl = f; }
            // c7468 add eax,r13d
            { let (r, f) = add::<32>((rax as u32 as u64), (r13 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c746b add eax,0xfb604040
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfb604040u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c7470 js 0xc74e3
            if fl.s() { pc = 0xc74e3; continue; }
            pc = 0xc7472;
        }
        0xc7472 => {
            // block c7472
            // c7472 mov DWORD PTR [rbp-0x38],r10d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((r10 as u32 as u64)) as u32);
            // c7476 mov r10d,r15d
            r10 = ((r15 as u32 as u64)) as u32 as u64;
            // c7479 mov r15d,r14d
            r15 = ((r14 as u32 as u64)) as u32 as u64;
            // c747c mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c747f mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c7482 add eax,0x8000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x8000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c7487 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c748a mov r8d,eax
            r8 = ((rax as u32 as u64)) as u32 as u64;
            // c748d jmp 0xc7503
            pc = 0xc7503; continue;
        }
        0xc748f => {
            // block c748f
            // c748f mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c7494 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7496 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7499 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc749b;
        }
        0xc749b => {
            // block c749b
            // c749b movsx ecx,ax
            rcx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c749e mov di,0x3ff
            rdi = (rdi & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c74a2 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // c74a8 jg 0xc74b6
            if fl.g() { pc = 0xc74b6; continue; }
            pc = 0xc74aa;
        }
        0xc74aa => {
            // block c74aa
            // c74aa test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c74ad mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // c74b2 cmovns di,ax
            if fl.ns() { rdi = (rdi & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc74b6;
        }
        0xc74b6 => {
            // block c74b6
            // c74b6 mov WORD PTR [rsi+r12*8],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rdi as u16 as u64)) as u16);
            // c74bb imul eax,r9d,0xffff9bb6
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // c74c2 add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // c74c5 imul ecx,ebx,0xffff2fe1
            { let (r, f) = imul::<32>((rbx as u32 as u64), 0xffff2fe1u64); rcx = (r) as u32 as u64; fl = f; }
            // c74cb add ecx,r13d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r13 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c74ce add ecx,0x21e4cc0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x21e4cc0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c74d4 js 0xc754d
            if fl.s() { pc = 0xc754d; continue; }
            pc = 0xc74d6;
        }
        0xc74d6 => {
            // block c74d6
            // c74d6 add ecx,0x8000
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x8000u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c74dc shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c74df mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c74e1 jmp 0xc7559
            pc = 0xc7559; continue;
        }
        0xc74e3 => {
            // block c74e3
            // c74e3 mov DWORD PTR [rbp-0x38],r10d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((r10 as u32 as u64)) as u32);
            // c74e7 mov r10d,r15d
            r10 = ((r15 as u32 as u64)) as u32 as u64;
            // c74ea mov r15d,r14d
            r15 = ((r14 as u32 as u64)) as u32 as u64;
            // c74ed mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // c74f0 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c74f3 mov r8d,0x8000
            r8 = (0x8000u64) as u32 as u64;
            // c74f9 sub r8d,eax
            { let (r, f) = sub::<32>((r8 as u32 as u64), (rax as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // c74fc sar r8d,0x10
            { let (r, f) = sar::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c7500 neg r8d
            { let (r, f) = neg::<32>((r8 as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            pc = 0xc7503;
        }
        0xc7503 => {
            // block c7503
            // c7503 movsx eax,r8w
            rax = (sx::<16>((r8 as u16 as u64)) as u64) as u32 as u64;
            // c7507 mov di,0x3ff
            rdi = (rdi & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c750b cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // c7510 jg 0xc7520
            if fl.g() { pc = 0xc7520; continue; }
            pc = 0xc7512;
        }
        0xc7512 => {
            // block c7512
            // c7512 test r8w,r8w
            fl = logic::<16>((r8 as u16 as u64) & (r8 as u16 as u64));
            // c7516 mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // c751b cmovns di,r8w
            if fl.ns() { rdi = (rdi & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xc7520;
        }
        0xc7520 => {
            // block c7520
            // c7520 mov WORD PTR [rsi+r12*8],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rdi as u16 as u64)) as u16);
            // c7525 imul eax,r9d,0xffffc7fa
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // c752c add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // c752f imul ecx,ebx,0xffff73ce
            { let (r, f) = imul::<32>((rbx as u32 as u64), 0xffff73ceu64); rcx = (r) as u32 as u64; fl = f; }
            // c7535 add ecx,r13d
            { let (r, f) = add::<32>((rcx as u32 as u64), (r13 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7538 add ecx,0x13dbc40
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x13dbc40u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c753e js 0xc755f
            if fl.s() { pc = 0xc755f; continue; }
            pc = 0xc7540;
        }
        0xc7540 => {
            // block c7540
            // c7540 add ecx,0x8000
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x8000u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c7546 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c7549 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c754b jmp 0xc756b
            pc = 0xc756b; continue;
        }
        0xc754d => {
            // block c754d
            // c754d mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c7552 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7554 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7557 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc7559;
        }
        0xc7559 => {
            // block c7559
            // c7559 mov r9,QWORD PTR [rbp-0x30]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // c755d jmp 0xc757f
            pc = 0xc757f; continue;
        }
        0xc755f => {
            // block c755f
            // c755f mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // c7564 sub eax,ecx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // c7566 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // c7569 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0xc756b;
        }
        0xc756b => {
            // block c756b
            // c756b mov r8d,edx
            r8 = ((rdx as u32 as u64)) as u32 as u64;
            // c756e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // c7571 mov r9,QWORD PTR [rbp-0x30]
            r9 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // c7575 mov r14d,r15d
            r14 = ((r15 as u32 as u64)) as u32 as u64;
            // c7578 mov r15d,r10d
            r15 = ((r10 as u32 as u64)) as u32 as u64;
            // c757b mov r10d,DWORD PTR [rbp-0x38]
            r10 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            pc = 0xc757f;
        }
        0xc757f => {
            // block c757f
            // c757f movsx ecx,ax
            rcx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // c7582 mov bx,0x3ff
            rbx = (rbx & !0xffff) | ((0x3ffu64) as u16 as u64);
            // c7586 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // c758c jg 0xc759a
            if fl.g() { pc = 0xc759a; continue; }
            pc = 0xc758e;
        }
        0xc758e => {
            // block c758e
            // c758e test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // c7591 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // c7596 cmovns bx,ax
            if fl.ns() { rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xc759a;
        }
        0xc759a => {
            // block c759a
            // c759a mov WORD PTR [rsi+r12*8+0x6],0x3ac
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x6), (0x3acu64) as u16);
            // c75a2 shl r11d,0x6
            { let (r, f) = shl::<32>((r11 as u32 as u64), 0x6u64, fl); r11 = (r) as u32 as u64; fl = f; }
            // c75a6 mov WORD PTR [rsi+r12*8+0x4],r11w
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x4), ((r11 as u16 as u64)) as u16);
            // c75ac shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // c75af mov WORD PTR [rsi+r12*8+0x2],bx
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x2), ((rbx as u16 as u64)) as u16);
            // c75b5 shl edi,0x6
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x6u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c75b8 mov WORD PTR [rsi+r12*8],di
            m.w16(rsi.wrapping_add(r12.wrapping_mul(8)), ((rdi as u16 as u64)) as u16);
            // c75bd cmp r10d,edx
            fl = sub::<32>((r10 as u32 as u64), (rdx as u32 as u64)).1;
            // c75c0 mov r11w,r15w
            r11 = (r11 & !0xffff) | (((r15 as u16 as u64)) as u16 as u64);
            // c75c4 mov r13w,r14w
            r13 = (r13 & !0xffff) | (((r14 as u16 as u64)) as u16 as u64);
            // c75c8 jb 0xc7123
            if fl.b() { pc = 0xc7123; continue; }
            pc = 0xc75ce;
        }
        0xc75ce => {
            // block c75ce
            // c75ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c75cf pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c75d1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c75d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c75d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c75d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c75d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x65c08 size=1980
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00065c08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x65c08;
    loop {
        match pc {
        0x65c08 => {
            // block 65c08
            // 65c08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 65c09 mov rbp,rsp
            rbp = rsp;
            // 65c0c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65c0e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65c10 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65c12 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65c14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65c15 mov DWORD PTR [rbp-0x2c],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r8 as u32 as u64)) as u32);
            // 65c19 mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // 65c1c test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 65c1e je 0x663b9
            if fl.e() { pc = 0x663b9; continue; }
            pc = 0x65c24;
        }
        0x65c24 => {
            // block 65c24
            // 65c24 mov r15b,BYTE PTR [rdi+0x2]
            r15 = (r15 & !0xff) | ((m.r8(rdi.wrapping_add(0x2)) as u64) as u8 as u64);
            // 65c28 mov r10b,BYTE PTR [rdi]
            r10 = (r10 & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 65c2b mov r8b,BYTE PTR [rdi+0x1]
            r8 = (r8 & !0xff) | ((m.r8(rdi.wrapping_add(0x1)) as u64) as u8 as u64);
            // 65c2f xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            pc = 0x65c32;
        }
        0x65c32 => {
            // block 65c32
            // 65c32 mov r12d,r11d
            r12 = ((r11 as u32 as u64)) as u32 as u64;
            // 65c35 movzx eax,r8b
            rax = ((r8 as u8 as u64)) as u32 as u64;
            // 65c39 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 65c3b je 0x65c76
            if fl.e() { pc = 0x65c76; continue; }
            pc = 0x65c3d;
        }
        0x65c3d => {
            // block 65c3d
            // 65c3d mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 65c40 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 65c42 je 0x65caf
            if fl.e() { pc = 0x65caf; continue; }
            pc = 0x65c44;
        }
        0x65c44 => {
            // block 65c44
            // 65c44 imul r14d,eax,0xff40
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xff40u64); r14 = (r) as u32 as u64; fl = f; }
            // 65c4b movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65c4f imul edx,r9d,0x15ddf
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x15ddfu64); rdx = (r) as u32 as u64; fl = f; }
            // 65c56 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65c59 cmp edx,0xbee37f
            fl = sub::<32>((rdx as u32 as u64), 0xbee37fu64).1;
            // 65c5f ja 0x65d13
            if fl.a() { pc = 0x65d13; continue; }
            pc = 0x65c65;
        }
        0x65c65 => {
            // block 65c65
            // 65c65 mov eax,0xbf6380
            rax = (0xbf6380u64) as u32 as u64;
            // 65c6a sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65c6c sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65c6f neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65c71 jmp 0x65d1e
            pc = 0x65d1e; continue;
        }
        0x65c76 => {
            // block 65c76
            // 65c76 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 65c79 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 65c7b je 0x65ce1
            if fl.e() { pc = 0x65ce1; continue; }
            pc = 0x65c7d;
        }
        0x65c7d => {
            // block 65c7d
            // 65c7d imul r14d,eax,0xffdf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xffdfu64); r14 = (r) as u32 as u64; fl = f; }
            // 65c84 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65c88 imul edx,r9d,0x19a8c
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19a8cu64); rdx = (r) as u32 as u64; fl = f; }
            // 65c8f add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65c92 cmp edx,0xdd43ef
            fl = sub::<32>((rdx as u32 as u64), 0xdd43efu64).1;
            // 65c98 ja 0x65d38
            if fl.a() { pc = 0x65d38; continue; }
            pc = 0x65c9e;
        }
        0x65c9e => {
            // block 65c9e
            // 65c9e mov eax,0xddc3f0
            rax = (0xddc3f0u64) as u32 as u64;
            // 65ca3 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65ca5 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65ca8 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65caa jmp 0x65d43
            pc = 0x65d43; continue;
        }
        0x65caf => {
            // block 65caf
            // 65caf imul r14d,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); r14 = (r) as u32 as u64; fl = f; }
            // 65cb6 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65cba imul edx,r9d,0x19895
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19895u64); rdx = (r) as u32 as u64; fl = f; }
            // 65cc1 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65cc4 cmp edx,0xdeebcf
            fl = sub::<32>((rdx as u32 as u64), 0xdeebcfu64).1;
            // 65cca ja 0x65d64
            if fl.a() { pc = 0x65d64; continue; }
            pc = 0x65cd0;
        }
        0x65cd0 => {
            // block 65cd0
            // 65cd0 mov eax,0xdf6bd0
            rax = (0xdf6bd0u64) as u32 as u64;
            // 65cd5 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65cd7 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65cda neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65cdc jmp 0x65d6f
            pc = 0x65d6f; continue;
        }
        0x65ce1 => {
            // block 65ce1
            // 65ce1 imul r14d,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); r14 = (r) as u32 as u64; fl = f; }
            // 65ce8 movzx r9d,r15b
            r9 = ((r15 as u8 as u64)) as u32 as u64;
            // 65cec imul edx,r9d,0x1df71
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x1df71u64); rdx = (r) as u32 as u64; fl = f; }
            // 65cf3 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65cf6 cmp edx,0x102656f
            fl = sub::<32>((rdx as u32 as u64), 0x102656fu64).1;
            // 65cfc ja 0x65d8b
            if fl.a() { pc = 0x65d8b; continue; }
            pc = 0x65d02;
        }
        0x65d02 => {
            // block 65d02
            // 65d02 mov eax,0x102e570
            rax = (0x102e570u64) as u32 as u64;
            // 65d07 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65d09 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65d0c neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 65d0e jmp 0x65d96
            pc = 0x65d96; continue;
        }
        0x65d13 => {
            // block 65d13
            // 65d13 add edx,0xff419c80
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff419c80u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d19 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d1c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d1e;
        }
        0x65d1e => {
            // block 65d1e
            // 65d1e lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65d21 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d23 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65d29 jg 0x65db4
            if fl.g() { pc = 0x65db4; continue; }
            pc = 0x65d2f;
        }
        0x65d2f => {
            // block 65d2f
            // 65d2f cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65d32 jl 0x65db2
            if fl.l() { pc = 0x65db2; continue; }
            pc = 0x65d34;
        }
        0x65d34 => {
            // block 65d34
            // 65d34 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65d36 jmp 0x65db4
            pc = 0x65db4; continue;
        }
        0x65d38 => {
            // block 65d38
            // 65d38 add edx,0xff233c10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff233c10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d3e sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d41 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d43;
        }
        0x65d43 => {
            // block 65d43
            // 65d43 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65d46 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d48 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65d4e jg 0x65e0b
            if fl.g() { pc = 0x65e0b; continue; }
            pc = 0x65d54;
        }
        0x65d54 => {
            // block 65d54
            // 65d54 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65d57 jl 0x65e09
            if fl.l() { pc = 0x65e09; continue; }
            pc = 0x65d5d;
        }
        0x65d5d => {
            // block 65d5d
            // 65d5d mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65d5f jmp 0x65e0b
            pc = 0x65e0b; continue;
        }
        0x65d64 => {
            // block 65d64
            // 65d64 add edx,0xff219430
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff219430u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d6a sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d6d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d6f;
        }
        0x65d6f => {
            // block 65d6f
            // 65d6f mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d71 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65d76 jg 0x65e62
            if fl.g() { pc = 0x65e62; continue; }
            pc = 0x65d7c;
        }
        0x65d7c => {
            // block 65d7c
            // 65d7c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65d7e js 0x65e60
            if fl.s() { pc = 0x65e60; continue; }
            pc = 0x65d84;
        }
        0x65d84 => {
            // block 65d84
            // 65d84 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65d86 jmp 0x65e62
            pc = 0x65e62; continue;
        }
        0x65d8b => {
            // block 65d8b
            // 65d8b add edx,0xfefe1a90
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfefe1a90u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65d91 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65d94 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x65d96;
        }
        0x65d96 => {
            // block 65d96
            // 65d96 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65d98 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65d9d jg 0x65eb4
            if fl.g() { pc = 0x65eb4; continue; }
            pc = 0x65da3;
        }
        0x65da3 => {
            // block 65da3
            // 65da3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65da5 js 0x65eb2
            if fl.s() { pc = 0x65eb2; continue; }
            pc = 0x65dab;
        }
        0x65dab => {
            // block 65dab
            // 65dab mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65dad jmp 0x65eb4
            pc = 0x65eb4; continue;
        }
        0x65db2 => {
            // block 65db2
            // 65db2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65db4;
        }
        0x65db4 => {
            // block 65db4
            // 65db4 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65db9 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65dbd imul edx,r13d,0x1ba34
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1ba34u64); rdx = (r) as u32 as u64; fl = f; }
            // 65dc4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dc7 add edx,0xff12f200
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff12f200u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dcd js 0x65ddc
            if fl.s() { pc = 0x65ddc; continue; }
            pc = 0x65dcf;
        }
        0x65dcf => {
            // block 65dcf
            // 65dcf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65dd5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65dd8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65dda jmp 0x65de8
            pc = 0x65de8; continue;
        }
        0x65ddc => {
            // block 65ddc
            // 65ddc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65de1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65de3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65de6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65de8;
        }
        0x65de8 => {
            // block 65de8
            // 65de8 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65deb mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65ded cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65df3 jg 0x65f06
            if fl.g() { pc = 0x65f06; continue; }
            pc = 0x65df9;
        }
        0x65df9 => {
            // block 65df9
            // 65df9 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65dfc jl 0x65f04
            if fl.l() { pc = 0x65f04; continue; }
            pc = 0x65e02;
        }
        0x65e02 => {
            // block 65e02
            // 65e02 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65e04 jmp 0x65f06
            pc = 0x65f06; continue;
        }
        0x65e09 => {
            // block 65e09
            // 65e09 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65e0b;
        }
        0x65e0b => {
            // block 65e0b
            // 65e0b mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65e10 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65e14 imul edx,r13d,0x1dad7
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1dad7u64); rdx = (r) as u32 as u64; fl = f; }
            // 65e1b add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e1e add edx,0xff029690
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff029690u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e24 js 0x65e33
            if fl.s() { pc = 0x65e33; continue; }
            pc = 0x65e26;
        }
        0x65e26 => {
            // block 65e26
            // 65e26 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e2c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65e2f mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65e31 jmp 0x65e3f
            pc = 0x65e3f; continue;
        }
        0x65e33 => {
            // block 65e33
            // 65e33 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65e38 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65e3a sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65e3d neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65e3f;
        }
        0x65e3f => {
            // block 65e3f
            // 65e3f lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65e42 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65e44 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65e4a jg 0x65f2a
            if fl.g() { pc = 0x65f2a; continue; }
            pc = 0x65e50;
        }
        0x65e50 => {
            // block 65e50
            // 65e50 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65e53 jl 0x65f28
            if fl.l() { pc = 0x65f28; continue; }
            pc = 0x65e59;
        }
        0x65e59 => {
            // block 65e59
            // 65e59 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65e5b jmp 0x65f2a
            pc = 0x65f2a; continue;
        }
        0x65e60 => {
            // block 65e60
            // 65e60 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65e62;
        }
        0x65e62 => {
            // block 65e62
            // 65e62 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65e67 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65e6b imul edx,r13d,0x20469
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x20469u64); rdx = (r) as u32 as u64; fl = f; }
            // 65e72 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e75 add edx,0xfeeb2a30
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfeeb2a30u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e7b js 0x65e8a
            if fl.s() { pc = 0x65e8a; continue; }
            pc = 0x65e7d;
        }
        0x65e7d => {
            // block 65e7d
            // 65e7d add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65e83 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65e86 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65e88 jmp 0x65e96
            pc = 0x65e96; continue;
        }
        0x65e8a => {
            // block 65e8a
            // 65e8a mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65e8f sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65e91 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65e94 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65e96;
        }
        0x65e96 => {
            // block 65e96
            // 65e96 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65e98 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65e9d jg 0x65f7b
            if fl.g() { pc = 0x65f7b; continue; }
            pc = 0x65ea3;
        }
        0x65ea3 => {
            // block 65ea3
            // 65ea3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65ea5 js 0x65f79
            if fl.s() { pc = 0x65f79; continue; }
            pc = 0x65eab;
        }
        0x65eab => {
            // block 65eab
            // 65eab mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65ead jmp 0x65f7b
            pc = 0x65f7b; continue;
        }
        0x65eb2 => {
            // block 65eb2
            // 65eb2 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65eb4;
        }
        0x65eb4 => {
            // block 65eb4
            // 65eb4 mov BYTE PTR [rsi+r12*4+0x2],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rbx as u8 as u64)) as u8);
            // 65eb9 movzx r13d,r10b
            r13 = ((r10 as u8 as u64)) as u32 as u64;
            // 65ebd imul edx,r13d,0x22a86
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x22a86u64); rdx = (r) as u32 as u64; fl = f; }
            // 65ec4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ec7 add edx,0xfed81010
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfed81010u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ecd js 0x65edc
            if fl.s() { pc = 0x65edc; continue; }
            pc = 0x65ecf;
        }
        0x65ecf => {
            // block 65ecf
            // 65ecf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65ed5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65ed8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65eda jmp 0x65ee8
            pc = 0x65ee8; continue;
        }
        0x65edc => {
            // block 65edc
            // 65edc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65ee1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65ee3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65ee6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65ee8;
        }
        0x65ee8 => {
            // block 65ee8
            // 65ee8 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65eea cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65eef jg 0x65f9f
            if fl.g() { pc = 0x65f9f; continue; }
            pc = 0x65ef5;
        }
        0x65ef5 => {
            // block 65ef5
            // 65ef5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65ef7 js 0x65f9d
            if fl.s() { pc = 0x65f9d; continue; }
            pc = 0x65efd;
        }
        0x65efd => {
            // block 65efd
            // 65efd mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65eff jmp 0x65f9f
            pc = 0x65f9f; continue;
        }
        0x65f04 => {
            // block 65f04
            // 65f04 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f06;
        }
        0x65f06 => {
            // block 65f06
            // 65f06 mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f0a imul eax,r13d,0xffffaa1f
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffaa1fu64); rax = (r) as u32 as u64; fl = f; }
            // 65f11 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f14 imul edx,r9d,0xffff4dc9
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff4dc9u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f1b add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f1e add edx,0x741800
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x741800u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f24 jns 0x65f4a
            if fl.ns() { pc = 0x65f4a; continue; }
            pc = 0x65f26;
        }
        0x65f26 => {
            // block 65f26
            // 65f26 jmp 0x65f57
            pc = 0x65f57; continue;
        }
        0x65f28 => {
            // block 65f28
            // 65f28 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f2a;
        }
        0x65f2a => {
            // block 65f2a
            // 65f2a mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f2e imul eax,r13d,0xffffd007
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffd007u64); rax = (r) as u32 as u64; fl = f; }
            // 65f35 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f38 imul edx,r9d,0xffff87f3
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff87f3u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f3f add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f42 add edx,0x440510
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x440510u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f48 js 0x65f57
            if fl.s() { pc = 0x65f57; continue; }
            pc = 0x65f4a;
        }
        0x65f4a => {
            // block 65f4a
            // 65f4a add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f50 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65f53 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65f55 jmp 0x65f63
            pc = 0x65f63; continue;
        }
        0x65f57 => {
            // block 65f57
            // 65f57 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65f5c sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65f5e sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65f61 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65f63;
        }
        0x65f63 => {
            // block 65f63
            // 65f63 lea edx,[rax+0x10]
            rdx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 65f66 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65f68 cmp edx,0xff
            fl = sub::<32>((rdx as u32 as u64), 0xffu64).1;
            // 65f6e jg 0x65feb
            if fl.g() { pc = 0x65feb; continue; }
            pc = 0x65f70;
        }
        0x65f70 => {
            // block 65f70
            // 65f70 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 65f73 jl 0x65fe9
            if fl.l() { pc = 0x65fe9; continue; }
            pc = 0x65f75;
        }
        0x65f75 => {
            // block 65f75
            // 65f75 mov bl,dl
            rbx = (rbx & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 65f77 jmp 0x65feb
            pc = 0x65feb; continue;
        }
        0x65f79 => {
            // block 65f79
            // 65f79 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f7b;
        }
        0x65f7b => {
            // block 65f7b
            // 65f7b mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65f7f imul eax,r13d,0xffff9bb6
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // 65f86 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65f89 imul edx,r9d,0xffff2fe1
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff2fe1u64); rdx = (r) as u32 as u64; fl = f; }
            // 65f90 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f93 add edx,0x879330
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x879330u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65f99 jns 0x65fbf
            if fl.ns() { pc = 0x65fbf; continue; }
            pc = 0x65f9b;
        }
        0x65f9b => {
            // block 65f9b
            // 65f9b jmp 0x65fcc
            pc = 0x65fcc; continue;
        }
        0x65f9d => {
            // block 65f9d
            // 65f9d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65f9f;
        }
        0x65f9f => {
            // block 65f9f
            // 65f9f mov BYTE PTR [rsi+r12*4],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rbx as u8 as u64)) as u8);
            // 65fa3 imul eax,r13d,0xffffc7fa
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // 65faa add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 65fad imul edx,r9d,0xffff73ce
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff73ceu64); rdx = (r) as u32 as u64; fl = f; }
            // 65fb4 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fb7 add edx,0x4f6f10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4f6f10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fbd js 0x65fcc
            if fl.s() { pc = 0x65fcc; continue; }
            pc = 0x65fbf;
        }
        0x65fbf => {
            // block 65fbf
            // 65fbf add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 65fc5 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 65fc8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 65fca jmp 0x65fd8
            pc = 0x65fd8; continue;
        }
        0x65fcc => {
            // block 65fcc
            // 65fcc mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 65fd1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 65fd3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 65fd6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x65fd8;
        }
        0x65fd8 => {
            // block 65fd8
            // 65fd8 mov bl,0xff
            rbx = (rbx & !0xff) | ((0xffu64) as u8 as u64);
            // 65fda cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 65fdf jg 0x65feb
            if fl.g() { pc = 0x65feb; continue; }
            pc = 0x65fe1;
        }
        0x65fe1 => {
            // block 65fe1
            // 65fe1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65fe3 js 0x65fe9
            if fl.s() { pc = 0x65fe9; continue; }
            pc = 0x65fe5;
        }
        0x65fe5 => {
            // block 65fe5
            // 65fe5 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 65fe7 jmp 0x65feb
            pc = 0x65feb; continue;
        }
        0x65fe9 => {
            // block 65fe9
            // 65fe9 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x65feb;
        }
        0x65feb => {
            // block 65feb
            // 65feb mov BYTE PTR [rsi+r12*4+0x1],bl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rbx as u8 as u64)) as u8);
            // 65ff0 movzx r10d,BYTE PTR [rdi+r11*2+0x4]
            r10 = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u32 as u64;
            // 65ff6 mov r8b,BYTE PTR [rdi+r11*2+0x5]
            r8 = (r8 & !0xff) | ((m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x5)) as u64) as u8 as u64);
            // 65ffb movzx r15d,BYTE PTR [rdi+r11*2+0x6]
            r15 = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x6)) as u64) as u32 as u64;
            // 66001 add r13d,r10d
            { let (r, f) = add::<32>((r13 as u32 as u64), (r10 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // 66004 shr r13d,1
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // 66007 add r9d,r15d
            { let (r, f) = add::<32>((r9 as u32 as u64), (r15 as u32 as u64));
              r9 = (r) as u32 as u64;
              fl = f; }
            // 6600a shr r9d,1
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0x1u64, fl); r9 = (r) as u32 as u64; fl = f; }
            // 6600d lea r12d,[r11+0x1]
            r12 = (r11.wrapping_add(0x1)) as u32 as u64;
            // 66011 movzx eax,BYTE PTR [rdi+r11*2+0x3]
            rax = (m.r8(rdi.wrapping_add(r11.wrapping_mul(2)).wrapping_add(0x3)) as u64) as u32 as u64;
            // 66017 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 66019 je 0x66050
            if fl.e() { pc = 0x66050; continue; }
            pc = 0x6601b;
        }
        0x6601b => {
            // block 6601b
            // 6601b mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 6601e test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 66020 je 0x66085
            if fl.e() { pc = 0x66085; continue; }
            pc = 0x66022;
        }
        0x66022 => {
            // block 66022
            // 66022 imul r14d,eax,0xff40
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xff40u64); r14 = (r) as u32 as u64; fl = f; }
            // 66029 imul edx,r9d,0x15ddf
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x15ddfu64); rdx = (r) as u32 as u64; fl = f; }
            // 66030 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66033 cmp edx,0xbee37f
            fl = sub::<32>((rdx as u32 as u64), 0xbee37fu64).1;
            // 66039 ja 0x660dd
            if fl.a() { pc = 0x660dd; continue; }
            pc = 0x6603f;
        }
        0x6603f => {
            // block 6603f
            // 6603f mov eax,0xbf6380
            rax = (0xbf6380u64) as u32 as u64;
            // 66044 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66046 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66049 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 6604b jmp 0x660e8
            pc = 0x660e8; continue;
        }
        0x66050 => {
            // block 66050
            // 66050 mov edx,DWORD PTR [rbp-0x2c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 66053 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 66055 je 0x660b1
            if fl.e() { pc = 0x660b1; continue; }
            pc = 0x66057;
        }
        0x66057 => {
            // block 66057
            // 66057 imul r14d,eax,0xffdf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0xffdfu64); r14 = (r) as u32 as u64; fl = f; }
            // 6605e imul edx,r9d,0x19a8c
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19a8cu64); rdx = (r) as u32 as u64; fl = f; }
            // 66065 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66068 cmp edx,0xdd43ef
            fl = sub::<32>((rdx as u32 as u64), 0xdd43efu64).1;
            // 6606e ja 0x66106
            if fl.a() { pc = 0x66106; continue; }
            pc = 0x66074;
        }
        0x66074 => {
            // block 66074
            // 66074 mov eax,0xddc3f0
            rax = (0xddc3f0u64) as u32 as u64;
            // 66079 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6607b sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 6607e neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 66080 jmp 0x66111
            pc = 0x66111; continue;
        }
        0x66085 => {
            // block 66085
            // 66085 imul ebx,eax,0x12a15
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12a15u64); rbx = (r) as u32 as u64; fl = f; }
            // 6608b imul edx,r9d,0x19895
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x19895u64); rdx = (r) as u32 as u64; fl = f; }
            // 66092 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66094 cmp edx,0xdeebcf
            fl = sub::<32>((rdx as u32 as u64), 0xdeebcfu64).1;
            // 6609a ja 0x66132
            if fl.a() { pc = 0x66132; continue; }
            pc = 0x660a0;
        }
        0x660a0 => {
            // block 660a0
            // 660a0 mov eax,0xdf6bd0
            rax = (0xdf6bd0u64) as u32 as u64;
            // 660a5 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 660a7 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 660aa neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 660ac jmp 0x6613d
            pc = 0x6613d; continue;
        }
        0x660b1 => {
            // block 660b1
            // 660b1 imul ebx,eax,0x12acf
            { let (r, f) = imul::<32>((rax as u32 as u64), 0x12acfu64); rbx = (r) as u32 as u64; fl = f; }
            // 660b7 imul edx,r9d,0x1df71
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0x1df71u64); rdx = (r) as u32 as u64; fl = f; }
            // 660be add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 660c0 cmp edx,0x102656f
            fl = sub::<32>((rdx as u32 as u64), 0x102656fu64).1;
            // 660c6 ja 0x66159
            if fl.a() { pc = 0x66159; continue; }
            pc = 0x660cc;
        }
        0x660cc => {
            // block 660cc
            // 660cc mov eax,0x102e570
            rax = (0x102e570u64) as u32 as u64;
            // 660d1 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 660d3 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 660d6 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 660d8 jmp 0x66164
            pc = 0x66164; continue;
        }
        0x660dd => {
            // block 660dd
            // 660dd add edx,0xff419c80
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff419c80u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 660e3 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 660e6 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x660e8;
        }
        0x660e8 => {
            // block 660e8
            // 660e8 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 660eb mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 660ed cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 660f3 jg 0x66184
            if fl.g() { pc = 0x66184; continue; }
            pc = 0x660f9;
        }
        0x660f9 => {
            // block 660f9
            // 660f9 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 660fc jl 0x66182
            if fl.l() { pc = 0x66182; continue; }
            pc = 0x66102;
        }
        0x66102 => {
            // block 66102
            // 66102 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66104 jmp 0x66184
            pc = 0x66184; continue;
        }
        0x66106 => {
            // block 66106
            // 66106 add edx,0xff233c10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff233c10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6610c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 6610f mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x66111;
        }
        0x66111 => {
            // block 66111
            // 66111 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 66114 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66116 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 6611c jg 0x661d7
            if fl.g() { pc = 0x661d7; continue; }
            pc = 0x66122;
        }
        0x66122 => {
            // block 66122
            // 66122 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 66125 jl 0x661d5
            if fl.l() { pc = 0x661d5; continue; }
            pc = 0x6612b;
        }
        0x6612b => {
            // block 6612b
            // 6612b mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 6612d jmp 0x661d7
            pc = 0x661d7; continue;
        }
        0x66132 => {
            // block 66132
            // 66132 add edx,0xff219430
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff219430u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66138 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 6613b mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x6613d;
        }
        0x6613d => {
            // block 6613d
            // 6613d mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6613f cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 66144 jg 0x6622a
            if fl.g() { pc = 0x6622a; continue; }
            pc = 0x6614a;
        }
        0x6614a => {
            // block 6614a
            // 6614a test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6614c js 0x66228
            if fl.s() { pc = 0x66228; continue; }
            pc = 0x66152;
        }
        0x66152 => {
            // block 66152
            // 66152 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 66154 jmp 0x6622a
            pc = 0x6622a; continue;
        }
        0x66159 => {
            // block 66159
            // 66159 add edx,0xfefe1a90
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfefe1a90u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6615f sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66162 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            pc = 0x66164;
        }
        0x66164 => {
            // block 66164
            // 66164 mov r14b,0xff
            r14 = (r14 & !0xff) | ((0xffu64) as u8 as u64);
            // 66167 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 6616c jg 0x66278
            if fl.g() { pc = 0x66278; continue; }
            pc = 0x66172;
        }
        0x66172 => {
            // block 66172
            // 66172 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 66174 js 0x66275
            if fl.s() { pc = 0x66275; continue; }
            pc = 0x6617a;
        }
        0x6617a => {
            // block 6617a
            // 6617a mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6617d jmp 0x66278
            pc = 0x66278; continue;
        }
        0x66182 => {
            // block 66182
            // 66182 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x66184;
        }
        0x66184 => {
            // block 66184
            // 66184 mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 66189 imul edx,r13d,0x1ba34
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1ba34u64); rdx = (r) as u32 as u64; fl = f; }
            // 66190 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66193 add edx,0xff12f200
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff12f200u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66199 js 0x661a8
            if fl.s() { pc = 0x661a8; continue; }
            pc = 0x6619b;
        }
        0x6619b => {
            // block 6619b
            // 6619b add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661a1 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 661a4 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 661a6 jmp 0x661b4
            pc = 0x661b4; continue;
        }
        0x661a8 => {
            // block 661a8
            // 661a8 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 661ad sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 661af sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 661b2 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x661b4;
        }
        0x661b4 => {
            // block 661b4
            // 661b4 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 661b7 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 661b9 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 661bf jg 0x662c5
            if fl.g() { pc = 0x662c5; continue; }
            pc = 0x661c5;
        }
        0x661c5 => {
            // block 661c5
            // 661c5 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 661c8 jl 0x662c3
            if fl.l() { pc = 0x662c3; continue; }
            pc = 0x661ce;
        }
        0x661ce => {
            // block 661ce
            // 661ce mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 661d0 jmp 0x662c5
            pc = 0x662c5; continue;
        }
        0x661d5 => {
            // block 661d5
            // 661d5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x661d7;
        }
        0x661d7 => {
            // block 661d7
            // 661d7 mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 661dc imul edx,r13d,0x1dad7
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x1dad7u64); rdx = (r) as u32 as u64; fl = f; }
            // 661e3 add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661e6 add edx,0xff029690
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xff029690u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661ec js 0x661fb
            if fl.s() { pc = 0x661fb; continue; }
            pc = 0x661ee;
        }
        0x661ee => {
            // block 661ee
            // 661ee add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 661f4 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 661f7 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 661f9 jmp 0x66207
            pc = 0x66207; continue;
        }
        0x661fb => {
            // block 661fb
            // 661fb mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 66200 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66202 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66205 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66207;
        }
        0x66207 => {
            // block 66207
            // 66207 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 6620a mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6620c cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 66212 jg 0x662e9
            if fl.g() { pc = 0x662e9; continue; }
            pc = 0x66218;
        }
        0x66218 => {
            // block 66218
            // 66218 cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 6621b jl 0x662e7
            if fl.l() { pc = 0x662e7; continue; }
            pc = 0x66221;
        }
        0x66221 => {
            // block 66221
            // 66221 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66223 jmp 0x662e9
            pc = 0x662e9; continue;
        }
        0x66228 => {
            // block 66228
            // 66228 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6622a;
        }
        0x6622a => {
            // block 6622a
            // 6622a mov BYTE PTR [rsi+r12*4+0x2],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((rdx as u8 as u64)) as u8);
            // 6622f imul edx,r13d,0x20469
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x20469u64); rdx = (r) as u32 as u64; fl = f; }
            // 66236 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66238 add edx,0xfeeb2a30
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfeeb2a30u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6623e js 0x6624d
            if fl.s() { pc = 0x6624d; continue; }
            pc = 0x66240;
        }
        0x66240 => {
            // block 66240
            // 66240 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66246 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66249 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 6624b jmp 0x66259
            pc = 0x66259; continue;
        }
        0x6624d => {
            // block 6624d
            // 6624d mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 66252 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 66254 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66257 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66259;
        }
        0x66259 => {
            // block 66259
            // 66259 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 6625b cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 66260 jg 0x6633a
            if fl.g() { pc = 0x6633a; continue; }
            pc = 0x66266;
        }
        0x66266 => {
            // block 66266
            // 66266 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 66268 js 0x66338
            if fl.s() { pc = 0x66338; continue; }
            pc = 0x6626e;
        }
        0x6626e => {
            // block 6626e
            // 6626e mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 66270 jmp 0x6633a
            pc = 0x6633a; continue;
        }
        0x66275 => {
            // block 66275
            // 66275 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x66278;
        }
        0x66278 => {
            // block 66278
            // 66278 mov BYTE PTR [rsi+r12*4+0x2],r14b
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x2), ((r14 as u8 as u64)) as u8);
            // 6627d imul edx,r13d,0x22a86
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0x22a86u64); rdx = (r) as u32 as u64; fl = f; }
            // 66284 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66286 add edx,0xfed81010
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfed81010u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6628c js 0x6629b
            if fl.s() { pc = 0x6629b; continue; }
            pc = 0x6628e;
        }
        0x6628e => {
            // block 6628e
            // 6628e add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66294 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66297 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66299 jmp 0x662a7
            pc = 0x662a7; continue;
        }
        0x6629b => {
            // block 6629b
            // 6629b mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 662a0 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 662a2 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 662a5 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x662a7;
        }
        0x662a7 => {
            // block 662a7
            // 662a7 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 662a9 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 662ae jg 0x6635c
            if fl.g() { pc = 0x6635c; continue; }
            pc = 0x662b4;
        }
        0x662b4 => {
            // block 662b4
            // 662b4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 662b6 js 0x6635a
            if fl.s() { pc = 0x6635a; continue; }
            pc = 0x662bc;
        }
        0x662bc => {
            // block 662bc
            // 662bc mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 662be jmp 0x6635c
            pc = 0x6635c; continue;
        }
        0x662c3 => {
            // block 662c3
            // 662c3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x662c5;
        }
        0x662c5 => {
            // block 662c5
            // 662c5 mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 662c9 imul eax,r13d,0xffffaa1f
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffaa1fu64); rax = (r) as u32 as u64; fl = f; }
            // 662d0 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 662d3 imul edx,r9d,0xffff4dc9
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff4dc9u64); rdx = (r) as u32 as u64; fl = f; }
            // 662da add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 662dd add edx,0x741800
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x741800u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 662e3 jns 0x66309
            if fl.ns() { pc = 0x66309; continue; }
            pc = 0x662e5;
        }
        0x662e5 => {
            // block 662e5
            // 662e5 jmp 0x66316
            pc = 0x66316; continue;
        }
        0x662e7 => {
            // block 662e7
            // 662e7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x662e9;
        }
        0x662e9 => {
            // block 662e9
            // 662e9 mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 662ed imul eax,r13d,0xffffd007
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffd007u64); rax = (r) as u32 as u64; fl = f; }
            // 662f4 add r14d,eax
            { let (r, f) = add::<32>((r14 as u32 as u64), (rax as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // 662f7 imul edx,r9d,0xffff87f3
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff87f3u64); rdx = (r) as u32 as u64; fl = f; }
            // 662fe add edx,r14d
            { let (r, f) = add::<32>((rdx as u32 as u64), (r14 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66301 add edx,0x440510
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x440510u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66307 js 0x66316
            if fl.s() { pc = 0x66316; continue; }
            pc = 0x66309;
        }
        0x66309 => {
            // block 66309
            // 66309 add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6630f sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66312 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66314 jmp 0x66322
            pc = 0x66322; continue;
        }
        0x66316 => {
            // block 66316
            // 66316 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 6631b sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6631d sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66320 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66322;
        }
        0x66322 => {
            // block 66322
            // 66322 lea ebx,[rax+0x10]
            rbx = (rax.wrapping_add(0x10)) as u32 as u64;
            // 66325 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66327 cmp ebx,0xff
            fl = sub::<32>((rbx as u32 as u64), 0xffu64).1;
            // 6632d jg 0x663a6
            if fl.g() { pc = 0x663a6; continue; }
            pc = 0x6632f;
        }
        0x6632f => {
            // block 6632f
            // 6632f cmp eax,0xfffffff0
            fl = sub::<32>((rax as u32 as u64), 0xfffffff0u64).1;
            // 66332 jl 0x663a4
            if fl.l() { pc = 0x663a4; continue; }
            pc = 0x66334;
        }
        0x66334 => {
            // block 66334
            // 66334 mov dl,bl
            rdx = (rdx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 66336 jmp 0x663a6
            pc = 0x663a6; continue;
        }
        0x66338 => {
            // block 66338
            // 66338 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6633a;
        }
        0x6633a => {
            // block 6633a
            // 6633a mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 6633e imul eax,r13d,0xffff9bb6
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffff9bb6u64); rax = (r) as u32 as u64; fl = f; }
            // 66345 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 66347 imul edx,r9d,0xffff2fe1
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff2fe1u64); rdx = (r) as u32 as u64; fl = f; }
            // 6634e add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66350 add edx,0x879330
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x879330u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66356 jns 0x6637a
            if fl.ns() { pc = 0x6637a; continue; }
            pc = 0x66358;
        }
        0x66358 => {
            // block 66358
            // 66358 jmp 0x66387
            pc = 0x66387; continue;
        }
        0x6635a => {
            // block 6635a
            // 6635a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6635c;
        }
        0x6635c => {
            // block 6635c
            // 6635c mov BYTE PTR [rsi+r12*4],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)), ((rdx as u8 as u64)) as u8);
            // 66360 imul eax,r13d,0xffffc7fa
            { let (r, f) = imul::<32>((r13 as u32 as u64), 0xffffc7fau64); rax = (r) as u32 as u64; fl = f; }
            // 66367 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 66369 imul edx,r9d,0xffff73ce
            { let (r, f) = imul::<32>((r9 as u32 as u64), 0xffff73ceu64); rdx = (r) as u32 as u64; fl = f; }
            // 66370 add edx,ebx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66372 add edx,0x4f6f10
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x4f6f10u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66378 js 0x66387
            if fl.s() { pc = 0x66387; continue; }
            pc = 0x6637a;
        }
        0x6637a => {
            // block 6637a
            // 6637a add edx,0x8000
            { let (r, f) = add::<32>((rdx as u32 as u64), 0x8000u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 66380 sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 66383 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 66385 jmp 0x66393
            pc = 0x66393; continue;
        }
        0x66387 => {
            // block 66387
            // 66387 mov eax,0x8000
            rax = (0x8000u64) as u32 as u64;
            // 6638c sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 6638e sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 66391 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            pc = 0x66393;
        }
        0x66393 => {
            // block 66393
            // 66393 mov dl,0xff
            rdx = (rdx & !0xff) | ((0xffu64) as u8 as u64);
            // 66395 cmp eax,0xff
            fl = sub::<32>((rax as u32 as u64), 0xffu64).1;
            // 6639a jg 0x663a6
            if fl.g() { pc = 0x663a6; continue; }
            pc = 0x6639c;
        }
        0x6639c => {
            // block 6639c
            // 6639c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 6639e js 0x663a4
            if fl.s() { pc = 0x663a4; continue; }
            pc = 0x663a0;
        }
        0x663a0 => {
            // block 663a0
            // 663a0 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 663a2 jmp 0x663a6
            pc = 0x663a6; continue;
        }
        0x663a4 => {
            // block 663a4
            // 663a4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x663a6;
        }
        0x663a6 => {
            // block 663a6
            // 663a6 mov BYTE PTR [rsi+r12*4+0x1],dl
            m.w8(rsi.wrapping_add(r12.wrapping_mul(4)).wrapping_add(0x1), ((rdx as u8 as u64)) as u8);
            // 663ab add r11,0x2
            { let (r, f) = add::<64>(r11, 0x2u64);
              r11 = r;
              fl = f; }
            // 663af cmp r11d,DWORD PTR [rbp-0x30]
            fl = sub::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 663b3 jb 0x65c32
            if fl.b() { pc = 0x65c32; continue; }
            pc = 0x663b9;
        }
        0x663b9 => {
            // block 663b9
            // 663b9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 663ba pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 663bc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 663be pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 663c0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 663c2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 663c3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2260 size=2188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2260<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd2260;
    loop {
        match pc {
        0xd2260 => {
            // block d2260
            // d2260 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2261 mov rbp,rsp
            rbp = rsp;
            // d2264 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d2267 je 0xd238a
            if fl.e() { pc = 0xd238a; continue; }
            pc = 0xd226d;
        }
        0xd226d => {
            // block d226d
            // d226d mov DWORD PTR [rdi],0x0
            m.w32(rdi, (0x0u64) as u32);
            // d2273 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d2275 cmp ecx,0x5
            fl = sub::<32>((rcx as u32 as u64), 0x5u64).1;
            // d2278 ja 0xd238a
            if fl.a() { pc = 0xd238a; continue; }
            pc = 0xd227e;
        }
        0xd227e => {
            // block d227e
            // d227e lea rcx,[rip+0x663] # 0xd28e8
            rcx = m.env.code_va(0xd28e8);
            // d2285 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d2289 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // d228c jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd228e => {
            // block d228e
            // d228e add esi,0xfffffffd
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffdu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d2291 cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // d2294 ja 0xd238a
            if fl.a() { pc = 0xd238a; continue; }
            pc = 0xd229a;
        }
        0xd229a => {
            // block d229a
            // d229a lea rax,[rip+0x7d7] # 0xd2a78
            rax = m.env.code_va(0xd2a78);
            // d22a1 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d22a5 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d22a8 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd22aa => {
            // block d22aa
            // d22aa test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // d22ad je 0xd2540
            if fl.e() { pc = 0xd2540; continue; }
            pc = 0xd22b3;
        }
        0xd22b3 => {
            // block d22b3
            // d22b3 cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // d22b6 jne 0xd25ad
            if fl.ne() { pc = 0xd25ad; continue; }
            pc = 0xd22bc;
        }
        0xd22bc => {
            // block d22bc
            // d22bc mov DWORD PTR [rdi],0x79
            m.w32(rdi, (0x79u64) as u32);
            // d22c2 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd22c7 => {
            // block d22c7
            // d22c7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d22c9 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // d22cb cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // d22ce ja 0xd2821
            if fl.a() { pc = 0xd2821; continue; }
            pc = 0xd22d4;
        }
        0xd22d4 => {
            // block d22d4
            // d22d4 lea rcx,[rip+0x64d] # 0xd2928
            rcx = m.env.code_va(0xd2928);
            // d22db movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d22df add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // d22e2 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd22e4 => {
            // block d22e4
            // d22e4 mov DWORD PTR [rdi],0x5
            m.w32(rdi, (0x5u64) as u32);
            // d22ea jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd22ef => {
            // block d22ef
            // d22ef cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // d22f2 je 0xd236d
            if fl.e() { pc = 0xd236d; continue; }
            pc = 0xd22f4;
        }
        0xd22f4 => {
            // block d22f4
            // d22f4 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // d22f7 jne 0xd2378
            if fl.ne() { pc = 0xd2378; continue; }
            pc = 0xd22f9;
        }
        0xd22f9 => {
            // block d22f9
            // d22f9 mov DWORD PTR [rdi],0x23
            m.w32(rdi, (0x23u64) as u32);
            // d22ff jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2304 => {
            // block d2304
            // d2304 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // d2308 lea eax,[rax*4+0x21]
            rax = (rax.wrapping_mul(4).wrapping_add(0x21)) as u32 as u64;
            pc = 0xd230f;
        }
        0xd230f => {
            // block d230f
            // d230f mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // d2311 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2316 => {
            // block d2316
            // d2316 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // d2318 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // d231b ja 0xd238a
            if fl.a() { pc = 0xd238a; continue; }
            pc = 0xd231d;
        }
        0xd231d => {
            // block d231d
            // d231d lea rax,[rip+0x624] # 0xd2948
            rax = m.env.code_va(0xd2948);
            // d2324 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d2328 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d232b jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd232d => {
            // block d232d
            // d232d cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d2330 jg 0xd25ce
            if fl.g() { pc = 0xd25ce; continue; }
            pc = 0xd2336;
        }
        0xd2336 => {
            // block d2336
            // d2336 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d2339 jne 0xd25f1
            if fl.ne() { pc = 0xd25f1; continue; }
            pc = 0xd233f;
        }
        0xd233f => {
            // block d233f
            // d233f jmp 0xd25e6
            pc = 0xd25e6; continue;
        }
        0xd2344 => {
            // block d2344
            // d2344 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d2346 add esi,0xfffffffb
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffbu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d2349 cmp esi,0x9
            fl = sub::<32>((rsi as u32 as u64), 0x9u64).1;
            // d234c ja 0xd2821
            if fl.a() { pc = 0xd2821; continue; }
            pc = 0xd2352;
        }
        0xd2352 => {
            // block d2352
            // d2352 lea rcx,[rip+0x5a7] # 0xd2900
            rcx = m.env.code_va(0xd2900);
            // d2359 movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // d235d add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // d2360 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2362 => {
            // block d2362
            // d2362 mov DWORD PTR [rdi],0x44
            m.w32(rdi, (0x44u64) as u32);
            // d2368 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd236d => {
            // block d236d
            // d236d mov DWORD PTR [rdi],0x22
            m.w32(rdi, (0x22u64) as u32);
            // d2373 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2378 => {
            // block d2378
            // d2378 cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // d237b jne 0xd238a
            if fl.ne() { pc = 0xd238a; continue; }
            pc = 0xd237d;
        }
        0xd237d => {
            // block d237d
            // d237d movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // d2381 lea eax,[rax*4+0x20]
            rax = (rax.wrapping_mul(4).wrapping_add(0x20)) as u32 as u64;
            // d2388 jmp 0xd230f
            pc = 0xd230f; continue;
        }
        0xd238a => {
            // block d238a
            // d238a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d238c jmp 0xd2821
            pc = 0xd2821; continue;
        }
        0xd2391 => {
            // block d2391
            // d2391 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // d2394 je 0xd2554
            if fl.e() { pc = 0xd2554; continue; }
            pc = 0xd239a;
        }
        0xd239a => {
            // block d239a
            // d239a cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // d239d jne 0xd25b8
            if fl.ne() { pc = 0xd25b8; continue; }
            pc = 0xd23a3;
        }
        0xd23a3 => {
            // block d23a3
            // d23a3 mov DWORD PTR [rdi],0x78
            m.w32(rdi, (0x78u64) as u32);
            // d23a9 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd23ae => {
            // block d23ae
            // d23ae test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // d23b1 je 0xd2568
            if fl.e() { pc = 0xd2568; continue; }
            pc = 0xd23b7;
        }
        0xd23b7 => {
            // block d23b7
            // d23b7 cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // d23ba jne 0xd25c3
            if fl.ne() { pc = 0xd25c3; continue; }
            pc = 0xd23c0;
        }
        0xd23c0 => {
            // block d23c0
            // d23c0 mov DWORD PTR [rdi],0x77
            m.w32(rdi, (0x77u64) as u32);
            // d23c6 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd23cb => {
            // block d23cb
            // d23cb test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // d23ce je 0xd257f
            if fl.e() { pc = 0xd257f; continue; }
            pc = 0xd23d4;
        }
        0xd23d4 => {
            // block d23d4
            // d23d4 mov DWORD PTR [rdi],0x76
            m.w32(rdi, (0x76u64) as u32);
            // d23da jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd23df => {
            // block d23df
            // d23df test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // d23e2 je 0xd2596
            if fl.e() { pc = 0xd2596; continue; }
            pc = 0xd23e8;
        }
        0xd23e8 => {
            // block d23e8
            // d23e8 mov DWORD PTR [rdi],0x75
            m.w32(rdi, (0x75u64) as u32);
            // d23ee jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd23f3 => {
            // block d23f3
            // d23f3 mov DWORD PTR [rdi],0x4
            m.w32(rdi, (0x4u64) as u32);
            // d23f9 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd23fe => {
            // block d23fe
            // d23fe mov DWORD PTR [rdi],0x16
            m.w32(rdi, (0x16u64) as u32);
            // d2404 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2409 => {
            // block d2409
            // d2409 mov DWORD PTR [rdi],0x15
            m.w32(rdi, (0x15u64) as u32);
            // d240f jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2414 => {
            // block d2414
            // d2414 mov DWORD PTR [rdi],0x11
            m.w32(rdi, (0x11u64) as u32);
            // d241a jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd241f => {
            // block d241f
            // d241f mov DWORD PTR [rdi],0x43
            m.w32(rdi, (0x43u64) as u32);
            // d2425 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd242a => {
            // block d242a
            // d242a mov DWORD PTR [rdi],0x42
            m.w32(rdi, (0x42u64) as u32);
            pc = 0xd2430;
        }
        0xd2430 => {
            // block d2430
            // d2430 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2435 => {
            // block d2435
            // d2435 mov DWORD PTR [rdi],0x41
            m.w32(rdi, (0x41u64) as u32);
            // d243b jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2440 => {
            // block d2440
            // d2440 mov DWORD PTR [rdi],0x40
            m.w32(rdi, (0x40u64) as u32);
            // d2446 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd244b => {
            // block d244b
            // d244b cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d244e jg 0xd25fc
            if fl.g() { pc = 0xd25fc; continue; }
            pc = 0xd2454;
        }
        0xd2454 => {
            // block d2454
            // d2454 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d2457 jne 0xd261f
            if fl.ne() { pc = 0xd261f; continue; }
            pc = 0xd245d;
        }
        0xd245d => {
            // block d245d
            // d245d jmp 0xd2614
            pc = 0xd2614; continue;
        }
        0xd2462 => {
            // block d2462
            // d2462 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d2465 jg 0xd262a
            if fl.g() { pc = 0xd262a; continue; }
            pc = 0xd246b;
        }
        0xd246b => {
            // block d246b
            // d246b cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d246e jne 0xd264d
            if fl.ne() { pc = 0xd264d; continue; }
            pc = 0xd2474;
        }
        0xd2474 => {
            // block d2474
            // d2474 jmp 0xd2642
            pc = 0xd2642; continue;
        }
        0xd2479 => {
            // block d2479
            // d2479 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d247c jg 0xd2658
            if fl.g() { pc = 0xd2658; continue; }
            pc = 0xd2482;
        }
        0xd2482 => {
            // block d2482
            // d2482 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d2485 jne 0xd267b
            if fl.ne() { pc = 0xd267b; continue; }
            pc = 0xd248b;
        }
        0xd248b => {
            // block d248b
            // d248b jmp 0xd2670
            pc = 0xd2670; continue;
        }
        0xd2490 => {
            // block d2490
            // d2490 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d2493 jg 0xd2686
            if fl.g() { pc = 0xd2686; continue; }
            pc = 0xd2499;
        }
        0xd2499 => {
            // block d2499
            // d2499 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d249c jne 0xd26a9
            if fl.ne() { pc = 0xd26a9; continue; }
            pc = 0xd24a2;
        }
        0xd24a2 => {
            // block d24a2
            // d24a2 jmp 0xd269e
            pc = 0xd269e; continue;
        }
        0xd24a7 => {
            // block d24a7
            // d24a7 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d24aa jg 0xd26b4
            if fl.g() { pc = 0xd26b4; continue; }
            pc = 0xd24b0;
        }
        0xd24b0 => {
            // block d24b0
            // d24b0 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d24b3 jne 0xd26d7
            if fl.ne() { pc = 0xd26d7; continue; }
            pc = 0xd24b9;
        }
        0xd24b9 => {
            // block d24b9
            // d24b9 jmp 0xd26cc
            pc = 0xd26cc; continue;
        }
        0xd24be => {
            // block d24be
            // d24be cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d24c1 jg 0xd26e2
            if fl.g() { pc = 0xd26e2; continue; }
            pc = 0xd24c7;
        }
        0xd24c7 => {
            // block d24c7
            // d24c7 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            pc = 0xd24ca;
        }
        0xd24ca => {
            // block d24ca
            // d24ca jne 0xd2705
            if fl.ne() { pc = 0xd2705; continue; }
            pc = 0xd24d0;
        }
        0xd24d0 => {
            // block d24d0
            // d24d0 jmp 0xd26fa
            pc = 0xd26fa; continue;
        }
        0xd24d5 => {
            // block d24d5
            // d24d5 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d24d8 jg 0xd2710
            if fl.g() { pc = 0xd2710; continue; }
            pc = 0xd24de;
        }
        0xd24de => {
            // block d24de
            // d24de cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d24e1 jne 0xd2733
            if fl.ne() { pc = 0xd2733; continue; }
            pc = 0xd24e7;
        }
        0xd24e7 => {
            // block d24e7
            // d24e7 jmp 0xd2728
            pc = 0xd2728; continue;
        }
        0xd24ec => {
            // block d24ec
            // d24ec cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d24ef jne 0xd273e
            if fl.ne() { pc = 0xd273e; continue; }
            pc = 0xd24f5;
        }
        0xd24f5 => {
            // block d24f5
            // d24f5 mov DWORD PTR [rdi],0x6b
            m.w32(rdi, (0x6bu64) as u32);
            // d24fb jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2500 => {
            // block d2500
            // d2500 cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d2503 jne 0xd2749
            if fl.ne() { pc = 0xd2749; continue; }
            pc = 0xd2509;
        }
        0xd2509 => {
            // block d2509
            // d2509 mov DWORD PTR [rdi],0x6a
            m.w32(rdi, (0x6au64) as u32);
            // d250f jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2514 => {
            // block d2514
            // d2514 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d2516 cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d2519 jne 0xd2821
            if fl.ne() { pc = 0xd2821; continue; }
            pc = 0xd251f;
        }
        0xd251f => {
            // block d251f
            // d251f mov DWORD PTR [rdi],0x6d
            m.w32(rdi, (0x6du64) as u32);
            // d2525 jmp 0xd2821
            pc = 0xd2821; continue;
        }
        0xd252a => {
            // block d252a
            // d252a mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d252c cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d252f jne 0xd2821
            if fl.ne() { pc = 0xd2821; continue; }
            pc = 0xd2535;
        }
        0xd2535 => {
            // block d2535
            // d2535 mov DWORD PTR [rdi],0x6c
            m.w32(rdi, (0x6cu64) as u32);
            // d253b jmp 0xd2821
            pc = 0xd2821; continue;
        }
        0xd2540 => {
            // block d2540
            // d2540 cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d2543 jne 0xd2754
            if fl.ne() { pc = 0xd2754; continue; }
            pc = 0xd2549;
        }
        0xd2549 => {
            // block d2549
            // d2549 mov DWORD PTR [rdi],0x63
            m.w32(rdi, (0x63u64) as u32);
            // d254f jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2554 => {
            // block d2554
            // d2554 cmp edx,0x11
            fl = sub::<32>((rdx as u32 as u64), 0x11u64).1;
            // d2557 jne 0xd2768
            if fl.ne() { pc = 0xd2768; continue; }
            pc = 0xd255d;
        }
        0xd255d => {
            // block d255d
            // d255d mov DWORD PTR [rdi],0x62
            m.w32(rdi, (0x62u64) as u32);
            // d2563 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2568 => {
            // block d2568
            // d2568 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d256b jg 0xd277c
            if fl.g() { pc = 0xd277c; continue; }
            pc = 0xd2571;
        }
        0xd2571 => {
            // block d2571
            // d2571 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d2574 jne 0xd279f
            if fl.ne() { pc = 0xd279f; continue; }
            pc = 0xd257a;
        }
        0xd257a => {
            // block d257a
            // d257a jmp 0xd2794
            pc = 0xd2794; continue;
        }
        0xd257f => {
            // block d257f
            // d257f cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d2582 jg 0xd27ae
            if fl.g() { pc = 0xd27ae; continue; }
            pc = 0xd2588;
        }
        0xd2588 => {
            // block d2588
            // d2588 cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d258b jne 0xd27ce
            if fl.ne() { pc = 0xd27ce; continue; }
            pc = 0xd2591;
        }
        0xd2591 => {
            // block d2591
            // d2591 jmp 0xd27c6
            pc = 0xd27c6; continue;
        }
        0xd2596 => {
            // block d2596
            // d2596 cmp edx,0x9
            fl = sub::<32>((rdx as u32 as u64), 0x9u64).1;
            // d2599 jg 0xd27d6
            if fl.g() { pc = 0xd27d6; continue; }
            pc = 0xd259f;
        }
        0xd259f => {
            // block d259f
            // d259f cmp edx,0x5
            fl = sub::<32>((rdx as u32 as u64), 0x5u64).1;
            // d25a2 jne 0xd27f6
            if fl.ne() { pc = 0xd27f6; continue; }
            pc = 0xd25a8;
        }
        0xd25a8 => {
            // block d25a8
            // d25a8 jmp 0xd27ee
            pc = 0xd27ee; continue;
        }
        0xd25ad => {
            // block d25ad
            // d25ad mov DWORD PTR [rdi],0x14
            m.w32(rdi, (0x14u64) as u32);
            // d25b3 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd25b8 => {
            // block d25b8
            // d25b8 mov DWORD PTR [rdi],0x13
            m.w32(rdi, (0x13u64) as u32);
            // d25be jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd25c3 => {
            // block d25c3
            // d25c3 mov DWORD PTR [rdi],0x12
            m.w32(rdi, (0x12u64) as u32);
            // d25c9 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd25ce => {
            // block d25ce
            // d25ce add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d25d1 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d25d4 ja 0xd25f1
            if fl.a() { pc = 0xd25f1; continue; }
            pc = 0xd25d6;
        }
        0xd25d6 => {
            // block d25d6
            // d25d6 lea rax,[rip+0x39b] # 0xd2978
            rax = m.env.code_va(0xd2978);
            // d25dd movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d25e1 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d25e4 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd25e6 => {
            // block d25e6
            // d25e6 mov DWORD PTR [rdi],0x6e
            m.w32(rdi, (0x6eu64) as u32);
            // d25ec jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd25f1 => {
            // block d25f1
            // d25f1 mov DWORD PTR [rdi],0x19
            m.w32(rdi, (0x19u64) as u32);
            // d25f7 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd25fc => {
            // block d25fc
            // d25fc add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d25ff cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            pc = 0xd2602;
        }
        0xd2602 => {
            // block d2602
            // d2602 ja 0xd261f
            if fl.a() { pc = 0xd261f; continue; }
            pc = 0xd2604;
        }
        0xd2604 => {
            // block d2604
            // d2604 lea rax,[rip+0x38d] # 0xd2998
            rax = m.env.code_va(0xd2998);
            // d260b movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d260f add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d2612 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2614 => {
            // block d2614
            // d2614 mov DWORD PTR [rdi],0x6f
            m.w32(rdi, (0x6fu64) as u32);
            // d261a jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd261f => {
            // block d261f
            // d261f mov DWORD PTR [rdi],0x18
            m.w32(rdi, (0x18u64) as u32);
            // d2625 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd262a => {
            // block d262a
            // d262a add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d262d cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d2630 ja 0xd264d
            if fl.a() { pc = 0xd264d; continue; }
            pc = 0xd2632;
        }
        0xd2632 => {
            // block d2632
            // d2632 lea rax,[rip+0x41f] # 0xd2a58
            rax = m.env.code_va(0xd2a58);
            // d2639 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d263d add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d2640 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2642 => {
            // block d2642
            // d2642 mov DWORD PTR [rdi],0x71
            m.w32(rdi, (0x71u64) as u32);
            // d2648 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd264d => {
            // block d264d
            // d264d mov DWORD PTR [rdi],0x9
            m.w32(rdi, (0x9u64) as u32);
            // d2653 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2658 => {
            // block d2658
            // d2658 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d265b cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d265e ja 0xd267b
            if fl.a() { pc = 0xd267b; continue; }
            pc = 0xd2660;
        }
        0xd2660 => {
            // block d2660
            // d2660 lea rax,[rip+0x3d1] # 0xd2a38
            rax = m.env.code_va(0xd2a38);
            // d2667 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d266b add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d266e jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2670 => {
            // block d2670
            // d2670 mov DWORD PTR [rdi],0x70
            m.w32(rdi, (0x70u64) as u32);
            // d2676 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd267b => {
            // block d267b
            // d267b mov DWORD PTR [rdi],0x8
            m.w32(rdi, (0x8u64) as u32);
            // d2681 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2686 => {
            // block d2686
            // d2686 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d2689 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d268c ja 0xd26a9
            if fl.a() { pc = 0xd26a9; continue; }
            pc = 0xd268e;
        }
        0xd268e => {
            // block d268e
            // d268e lea rax,[rip+0x383] # 0xd2a18
            rax = m.env.code_va(0xd2a18);
            // d2695 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d2699 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d269c jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd269e => {
            // block d269e
            // d269e mov DWORD PTR [rdi],0x1a
            m.w32(rdi, (0x1au64) as u32);
            // d26a4 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd26a9 => {
            // block d26a9
            // d26a9 mov DWORD PTR [rdi],0xa
            m.w32(rdi, (0xau64) as u32);
            // d26af jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd26b4 => {
            // block d26b4
            // d26b4 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d26b7 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d26ba ja 0xd26d7
            if fl.a() { pc = 0xd26d7; continue; }
            pc = 0xd26bc;
        }
        0xd26bc => {
            // block d26bc
            // d26bc lea rax,[rip+0x335] # 0xd29f8
            rax = m.env.code_va(0xd29f8);
            // d26c3 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d26c7 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d26ca jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd26cc => {
            // block d26cc
            // d26cc mov DWORD PTR [rdi],0xe
            m.w32(rdi, (0xeu64) as u32);
            // d26d2 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd26d7 => {
            // block d26d7
            // d26d7 mov DWORD PTR [rdi],0xc
            m.w32(rdi, (0xcu64) as u32);
            // d26dd jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd26e2 => {
            // block d26e2
            // d26e2 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d26e5 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d26e8 ja 0xd2705
            if fl.a() { pc = 0xd2705; continue; }
            pc = 0xd26ea;
        }
        0xd26ea => {
            // block d26ea
            // d26ea lea rax,[rip+0x2e7] # 0xd29d8
            rax = m.env.code_va(0xd29d8);
            // d26f1 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d26f5 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            pc = 0xd26f8;
        }
        0xd26f8 => {
            // block d26f8
            // d26f8 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd26fa => {
            // block d26fa
            // d26fa mov DWORD PTR [rdi],0xd
            m.w32(rdi, (0xdu64) as u32);
            // d2700 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2705 => {
            // block d2705
            // d2705 mov DWORD PTR [rdi],0xb
            m.w32(rdi, (0xbu64) as u32);
            pc = 0xd270b;
        }
        0xd270b => {
            // block d270b
            // d270b jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2710 => {
            // block d2710
            // d2710 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d2713 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d2716 ja 0xd2733
            if fl.a() { pc = 0xd2733; continue; }
            pc = 0xd2718;
        }
        0xd2718 => {
            // block d2718
            // d2718 lea rax,[rip+0x299] # 0xd29b8
            rax = m.env.code_va(0xd29b8);
            // d271f movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d2723 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d2726 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2728 => {
            // block d2728
            // d2728 mov DWORD PTR [rdi],0x72
            m.w32(rdi, (0x72u64) as u32);
            pc = 0xd272e;
        }
        0xd272e => {
            // block d272e
            // d272e jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2733 => {
            // block d2733
            // d2733 mov DWORD PTR [rdi],0x17
            m.w32(rdi, (0x17u64) as u32);
            pc = 0xd2739;
        }
        0xd2739 => {
            // block d2739
            // d2739 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd273e => {
            // block d273e
            // d273e mov DWORD PTR [rdi],0x74
            m.w32(rdi, (0x74u64) as u32);
            // d2744 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2749 => {
            // block d2749
            // d2749 mov DWORD PTR [rdi],0x73
            m.w32(rdi, (0x73u64) as u32);
            // d274f jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2754 => {
            // block d2754
            // d2754 cmp edx,0x10
            fl = sub::<32>((rdx as u32 as u64), 0x10u64).1;
            // d2757 jne 0xd27fe
            if fl.ne() { pc = 0xd27fe; continue; }
            pc = 0xd275d;
        }
        0xd275d => {
            // block d275d
            // d275d mov DWORD PTR [rdi],0x5f
            m.w32(rdi, (0x5fu64) as u32);
            pc = 0xd2763;
        }
        0xd2763 => {
            // block d2763
            // d2763 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2768 => {
            // block d2768
            // d2768 cmp edx,0x10
            fl = sub::<32>((rdx as u32 as u64), 0x10u64).1;
            pc = 0xd276b;
        }
        0xd276b => {
            // block d276b
            // d276b jne 0xd280d
            if fl.ne() { pc = 0xd280d; continue; }
            pc = 0xd2771;
        }
        0xd2771 => {
            // block d2771
            // d2771 mov DWORD PTR [rdi],0x5e
            m.w32(rdi, (0x5eu64) as u32);
            // d2777 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd277c => {
            // block d277c
            // d277c add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d277f cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d2782 ja 0xd279f
            if fl.a() { pc = 0xd279f; continue; }
            pc = 0xd2784;
        }
        0xd2784 => {
            // block d2784
            // d2784 lea rax,[rip+0x341] # 0xd2acc
            rax = m.env.code_va(0xd2acc);
            pc = 0xd278b;
        }
        0xd278b => {
            // block d278b
            // d278b movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d278f add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d2792 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd2794 => {
            // block d2794
            // d2794 mov DWORD PTR [rdi],0x1b
            m.w32(rdi, (0x1bu64) as u32);
            // d279a jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd279f => {
            // block d279f
            // d279f test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // d27a2 mov eax,0x1c
            rax = (0x1cu64) as u32 as u64;
            // d27a7 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // d27ac jmp 0xd281a
            pc = 0xd281a; continue;
        }
        0xd27ae => {
            // block d27ae
            // d27ae add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d27b1 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d27b4 ja 0xd27ce
            if fl.a() { pc = 0xd27ce; continue; }
            pc = 0xd27b6;
        }
        0xd27b6 => {
            // block d27b6
            // d27b6 lea rax,[rip+0x2ef] # 0xd2aac
            rax = m.env.code_va(0xd2aac);
            // d27bd movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d27c1 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d27c4 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd27c6 => {
            // block d27c6
            // d27c6 mov DWORD PTR [rdi],0x10
            m.w32(rdi, (0x10u64) as u32);
            // d27cc jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd27ce => {
            // block d27ce
            // d27ce mov DWORD PTR [rdi],0x7
            m.w32(rdi, (0x7u64) as u32);
            // d27d4 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd27d6 => {
            // block d27d6
            // d27d6 add edx,0xfffffff6
            { let (r, f) = add::<32>((rdx as u32 as u64), 0xfffffff6u64);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d27d9 cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // d27dc ja 0xd27f6
            if fl.a() { pc = 0xd27f6; continue; }
            pc = 0xd27de;
        }
        0xd27de => {
            // block d27de
            // d27de lea rax,[rip+0x2a7] # 0xd2a8c
            rax = m.env.code_va(0xd2a8c);
            // d27e5 movsxd rcx,DWORD PTR [rax+rdx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // d27e9 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d27ec jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd27ee => {
            // block d27ee
            // d27ee mov DWORD PTR [rdi],0xf
            m.w32(rdi, (0xfu64) as u32);
            // d27f4 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd27f6 => {
            // block d27f6
            // d27f6 mov DWORD PTR [rdi],0x6
            m.w32(rdi, (0x6u64) as u32);
            // d27fc jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd27fe => {
            // block d27fe
            // d27fe test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // d2801 mov eax,0x1e
            rax = (0x1eu64) as u32 as u64;
            // d2806 mov ecx,0x3
            rcx = (0x3u64) as u32 as u64;
            // d280b jmp 0xd281a
            pc = 0xd281a; continue;
        }
        0xd280d => {
            // block d280d
            // d280d test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // d2810 mov eax,0x1d
            rax = (0x1du64) as u32 as u64;
            // d2815 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            pc = 0xd281a;
        }
        0xd281a => {
            // block d281a
            // d281a cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // d281d mov DWORD PTR [rdi],ecx
            m.w32(rdi, ((rcx as u32 as u64)) as u32);
            pc = 0xd281f;
        }
        0xd281f => {
            // block d281f
            // d281f mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd2821;
        }
        0xd2821 => {
            // block d2821
            // d2821 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2822 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd2823 => {
            // block d2823
            // d2823 mov DWORD PTR [rdi],0x66
            m.w32(rdi, (0x66u64) as u32);
            // d2829 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd282b => {
            // block d282b
            // d282b mov DWORD PTR [rdi],0x69
            m.w32(rdi, (0x69u64) as u32);
            // d2831 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2833 => {
            // block d2833
            // d2833 mov DWORD PTR [rdi],0x65
            m.w32(rdi, (0x65u64) as u32);
            // d2839 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd283b => {
            // block d283b
            // d283b mov DWORD PTR [rdi],0x68
            m.w32(rdi, (0x68u64) as u32);
            // d2841 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2843 => {
            // block d2843
            // d2843 mov DWORD PTR [rdi],0x5d
            m.w32(rdi, (0x5du64) as u32);
            // d2849 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd284b => {
            // block d284b
            // d284b mov DWORD PTR [rdi],0x61
            m.w32(rdi, (0x61u64) as u32);
            // d2851 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2853 => {
            // block d2853
            // d2853 mov DWORD PTR [rdi],0x5c
            m.w32(rdi, (0x5cu64) as u32);
            // d2859 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd285b => {
            // block d285b
            // d285b mov DWORD PTR [rdi],0x60
            m.w32(rdi, (0x60u64) as u32);
            // d2861 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2863 => {
            // block d2863
            // d2863 mov DWORD PTR [rdi],0x55
            m.w32(rdi, (0x55u64) as u32);
            // d2869 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd286b => {
            // block d286b
            // d286b mov DWORD PTR [rdi],0x5b
            m.w32(rdi, (0x5bu64) as u32);
            // d2871 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2873 => {
            // block d2873
            // d2873 mov DWORD PTR [rdi],0x54
            m.w32(rdi, (0x54u64) as u32);
            // d2879 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd287b => {
            // block d287b
            // d287b mov DWORD PTR [rdi],0x5a
            m.w32(rdi, (0x5au64) as u32);
            // d2881 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2883 => {
            // block d2883
            // d2883 mov DWORD PTR [rdi],0x53
            m.w32(rdi, (0x53u64) as u32);
            // d2889 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd288b => {
            // block d288b
            // d288b mov DWORD PTR [rdi],0x59
            m.w32(rdi, (0x59u64) as u32);
            // d2891 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd2893 => {
            // block d2893
            // d2893 mov DWORD PTR [rdi],0x64
            m.w32(rdi, (0x64u64) as u32);
            // d2899 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd289b => {
            // block d289b
            // d289b mov DWORD PTR [rdi],0x67
            m.w32(rdi, (0x67u64) as u32);
            // d28a1 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28a6 => {
            // block d28a6
            // d28a6 mov DWORD PTR [rdi],0x52
            m.w32(rdi, (0x52u64) as u32);
            // d28ac jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28b1 => {
            // block d28b1
            // d28b1 mov DWORD PTR [rdi],0x58
            m.w32(rdi, (0x58u64) as u32);
            // d28b7 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28bc => {
            // block d28bc
            // d28bc mov DWORD PTR [rdi],0x51
            m.w32(rdi, (0x51u64) as u32);
            // d28c2 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28c7 => {
            // block d28c7
            // d28c7 mov DWORD PTR [rdi],0x57
            m.w32(rdi, (0x57u64) as u32);
            // d28cd jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28d2 => {
            // block d28d2
            // d28d2 mov DWORD PTR [rdi],0x50
            m.w32(rdi, (0x50u64) as u32);
            // d28d8 jmp 0xd281f
            pc = 0xd281f; continue;
        }
        0xd28dd => {
            // block d28dd
            // d28dd mov DWORD PTR [rdi],0x56
            m.w32(rdi, (0x56u64) as u32);
            // d28e3 jmp 0xd281f
            pc = 0xd281f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xca0d0 size=3560
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ca0d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xca0d0;
    loop {
        match pc {
        0xca0d0 => {
            // block ca0d0
            // ca0d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ca0d1 mov rbp,rsp
            rbp = rsp;
            // ca0d4 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // ca0da jg 0xca12d
            if fl.g() { pc = 0xca12d; continue; }
            pc = 0xca0dc;
        }
        0xca0dc => {
            // block ca0dc
            // ca0dc cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // ca0e2 jg 0xca17d
            if fl.g() { pc = 0xca17d; continue; }
            pc = 0xca0e8;
        }
        0xca0e8 => {
            // block ca0e8
            // ca0e8 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // ca0ee je 0xca27c
            if fl.e() { pc = 0xca27c; continue; }
            pc = 0xca0f4;
        }
        0xca0f4 => {
            // block ca0f4
            // ca0f4 movabs rax,0x405d80030ff3
            rax = 0x0_405d_8003_0ff3u64;
            // ca0fe cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // ca104 je 0xca112
            if fl.e() { pc = 0xca112; continue; }
            pc = 0xca106;
        }
        0xca106 => {
            // block ca106
            // ca106 cmp edi,0x10266401
            fl = sub::<32>((rdi as u32 as u64), 0x10266401u64).1;
            // ca10c jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca112;
        }
        0xca112 => {
            // block ca112
            // ca112 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca115 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca11b;
        }
        0xca11b => {
            // block ca11b
            // ca11b movabs rcx,0x7fffffffffff
            rcx = 0x0_7fff_ffff_ffffu64;
            // ca125 and rax,rcx
            { let r = rax & rcx; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // ca128 jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca12d => {
            // block ca12d
            // ca12d cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // ca133 jg 0xca1ad
            if fl.g() { pc = 0xca1ad; continue; }
            pc = 0xca135;
        }
        0xca135 => {
            // block ca135
            // ca135 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // ca13b jg 0xca1ea
            if fl.g() { pc = 0xca1ea; continue; }
            pc = 0xca141;
        }
        0xca141 => {
            // block ca141
            // ca141 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // ca147 jg 0xca294
            if fl.g() { pc = 0xca294; continue; }
            pc = 0xca14d;
        }
        0xca14d => {
            // block ca14d
            // ca14d cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // ca153 je 0xca49c
            if fl.e() { pc = 0xca49c; continue; }
            pc = 0xca159;
        }
        0xca159 => {
            // block ca159
            // ca159 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // ca15f jne 0xca4b1
            if fl.ne() { pc = 0xca4b1; continue; }
            pc = 0xca165;
        }
        0xca165 => {
            // block ca165
            // ca165 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca168 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca16e;
        }
        0xca16e => {
            // block ca16e
            // ca16e movabs rax,0x40d180030453
            rax = 0x0_40d1_8003_0453u64;
            // ca178 jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca17d => {
            // block ca17d
            // ca17d cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // ca183 jg 0xca21f
            if fl.g() { pc = 0xca21f; continue; }
            pc = 0xca189;
        }
        0xca189 => {
            // block ca189
            // ca189 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // ca18f jne 0xca2c4
            if fl.ne() { pc = 0xca2c4; continue; }
            pc = 0xca195;
        }
        0xca195 => {
            // block ca195
            // ca195 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca198 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca19e;
        }
        0xca19e => {
            // block ca19e
            // ca19e movabs rax,0x409d00010701
            rax = 0x0_409d_0001_0701u64;
            // ca1a8 jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca1ad => {
            // block ca1ad
            // ca1ad cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // ca1b3 jg 0xca243
            if fl.g() { pc = 0xca243; continue; }
            pc = 0xca1b9;
        }
        0xca1b9 => {
            // block ca1b9
            // ca1b9 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // ca1bf jg 0xca2e8
            if fl.g() { pc = 0xca2e8; continue; }
            pc = 0xca1c5;
        }
        0xca1c5 => {
            // block ca1c5
            // ca1c5 cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // ca1cb jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca1d1;
        }
        0xca1d1 => {
            // block ca1d1
            // ca1d1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca1d3 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // ca1d6 jg 0xca4d2
            if fl.g() { pc = 0xca4d2; continue; }
            pc = 0xca1dc;
        }
        0xca1dc => {
            // block ca1dc
            // ca1dc cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // ca1df jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca1e5;
        }
        0xca1e5 => {
            // block ca1e5
            // ca1e5 jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca1ea => {
            // block ca1ea
            // ca1ea cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // ca1f0 jg 0xca325
            if fl.g() { pc = 0xca325; continue; }
            pc = 0xca1f6;
        }
        0xca1f6 => {
            // block ca1f6
            // ca1f6 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // ca1fc jne 0xca3a4
            if fl.ne() { pc = 0xca3a4; continue; }
            pc = 0xca202;
        }
        0xca202 => {
            // block ca202
            // ca202 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca204 cmp esi,0x61
            fl = sub::<32>((rsi as u32 as u64), 0x61u64).1;
            // ca207 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca20d;
        }
        0xca20d => {
            // block ca20d
            // ca20d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca20f lea rdx,[rip+0x80a] # 0xcaa20
            rdx = m.env.code_va(0xcaa20);
            // ca216 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca21a add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca21d jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca21f => {
            // block ca21f
            // ca21f cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // ca225 jne 0xca34e
            if fl.ne() { pc = 0xca34e; continue; }
            pc = 0xca22b;
        }
        0xca22b => {
            // block ca22b
            // ca22b cmp esi,0x31
            fl = sub::<32>((rsi as u32 as u64), 0x31u64).1;
            // ca22e jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca234;
        }
        0xca234 => {
            // block ca234
            // ca234 movabs rax,0x140d9663030f3
            rax = 0x1_40d9_6630_30f3u64;
            // ca23e jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca243 => {
            // block ca243
            // ca243 cmp edi,0x1071084f
            fl = sub::<32>((rdi as u32 as u64), 0x1071084fu64).1;
            // ca249 jg 0xca372
            if fl.g() { pc = 0xca372; continue; }
            pc = 0xca24f;
        }
        0xca24f => {
            // block ca24f
            // ca24f cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // ca255 jne 0xca3de
            if fl.ne() { pc = 0xca3de; continue; }
            pc = 0xca25b;
        }
        0xca25b => {
            // block ca25b
            // ca25b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca25d cmp esi,0x31
            fl = sub::<32>((rsi as u32 as u64), 0x31u64).1;
            // ca260 jle 0xca5a3
            if fl.le() { pc = 0xca5a3; continue; }
            pc = 0xca266;
        }
        0xca266 => {
            // block ca266
            // ca266 add esi,0xffffffce
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffceu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca269 cmp esi,0xe
            fl = sub::<32>((rsi as u32 as u64), 0xeu64).1;
            // ca26c ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca272;
        }
        0xca272 => {
            // block ca272
            // ca272 mov ecx,0x41c3
            rcx = (0x41c3u64) as u32 as u64;
            // ca277 jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca27c => {
            // block ca27c
            // ca27c cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca27f jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca285;
        }
        0xca285 => {
            // block ca285
            // ca285 movabs rax,0x400000010403
            rax = 0x0_4000_0001_0403u64;
            // ca28f jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca294 => {
            // block ca294
            // ca294 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // ca29a jg 0xca410
            if fl.g() { pc = 0xca410; continue; }
            pc = 0xca2a0;
        }
        0xca2a0 => {
            // block ca2a0
            // ca2a0 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // ca2a6 jne 0xca4e8
            if fl.ne() { pc = 0xca4e8; continue; }
            pc = 0xca2ac;
        }
        0xca2ac => {
            // block ca2ac
            // ca2ac cmp esi,0x3a
            fl = sub::<32>((rsi as u32 as u64), 0x3au64).1;
            // ca2af jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca2b5;
        }
        0xca2b5 => {
            // block ca2b5
            // ca2b5 movabs rax,0x3fd404066f0f0ff
            rax = 0x3fd_4040_66f0_f0ffu64;
            // ca2bf jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca2c4 => {
            // block ca2c4
            // ca2c4 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // ca2ca jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca2d0;
        }
        0xca2d0 => {
            // block ca2d0
            // ca2d0 cmp esi,0x31
            fl = sub::<32>((rsi as u32 as u64), 0x31u64).1;
            // ca2d3 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca2d9;
        }
        0xca2d9 => {
            // block ca2d9
            // ca2d9 movabs rax,0x1404000303033
            rax = 0x1_4040_0030_3033u64;
            // ca2e3 jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca2e8 => {
            // block ca2e8
            // ca2e8 add edi,0xef9b9900
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xef9b9900u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // ca2ee cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // ca2f1 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca2f7;
        }
        0xca2f7 => {
            // block ca2f7
            // ca2f7 lea rax,[rip+0x3fa] # 0xca6f8
            rax = m.env.code_va(0xca6f8);
            // ca2fe movsxd rcx,DWORD PTR [rax+rdi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // ca302 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // ca305 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca307 => {
            // block ca307
            // ca307 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca309 cmp esi,0x25
            fl = sub::<32>((rsi as u32 as u64), 0x25u64).1;
            // ca30c jg 0xca622
            if fl.g() { pc = 0xca622; continue; }
            pc = 0xca312;
        }
        0xca312 => {
            // block ca312
            // ca312 cmp esi,0x1e
            fl = sub::<32>((rsi as u32 as u64), 0x1eu64).1;
            // ca315 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca31b;
        }
        0xca31b => {
            // block ca31b
            // ca31b mov ecx,0x66f0f0ff
            rcx = (0x66f0f0ffu64) as u32 as u64;
            // ca320 jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca325 => {
            // block ca325
            // ca325 cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // ca32b jne 0xca439
            if fl.ne() { pc = 0xca439; continue; }
            pc = 0xca331;
        }
        0xca331 => {
            // block ca331
            // ca331 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca333 cmp esi,0x61
            fl = sub::<32>((rsi as u32 as u64), 0x61u64).1;
            // ca336 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca33c;
        }
        0xca33c => {
            // block ca33c
            // ca33c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca33e lea rdx,[rip+0x863] # 0xcaba8
            rdx = m.env.code_va(0xcaba8);
            // ca345 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca349 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca34c jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca34e => {
            // block ca34e
            // ca34e cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // ca354 jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca35a;
        }
        0xca35a => {
            // block ca35a
            // ca35a cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca35d jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca363;
        }
        0xca363 => {
            // block ca363
            // ca363 movabs rax,0x404000101033
            rax = 0x0_4040_0010_1033u64;
            // ca36d jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca372 => {
            // block ca372
            // ca372 cmp edi,0x10710850
            fl = sub::<32>((rdi as u32 as u64), 0x10710850u64).1;
            // ca378 jne 0xca458
            if fl.ne() { pc = 0xca458; continue; }
            pc = 0xca37e;
        }
        0xca37e => {
            // block ca37e
            // ca37e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca380 cmp esi,0x5d
            fl = sub::<32>((rsi as u32 as u64), 0x5du64).1;
            // ca383 jg 0xca486
            if fl.g() { pc = 0xca486; continue; }
            pc = 0xca389;
        }
        0xca389 => {
            // block ca389
            // ca389 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca38b cmp esi,0x40
            fl = sub::<32>((rsi as u32 as u64), 0x40u64).1;
            // ca38e ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca394;
        }
        0xca394 => {
            // block ca394
            // ca394 lea rdx,[rip+0x47d] # 0xca818
            rdx = m.env.code_va(0xca818);
            // ca39b movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca39f add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca3a2 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca3a4 => {
            // block ca3a4
            // ca3a4 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // ca3aa jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca3b0;
        }
        0xca3b0 => {
            // block ca3b0
            // ca3b0 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca3b2 cmp esi,0x13
            fl = sub::<32>((rsi as u32 as u64), 0x13u64).1;
            // ca3b5 jle 0xca529
            if fl.le() { pc = 0xca529; continue; }
            pc = 0xca3bb;
        }
        0xca3bb => {
            // block ca3bb
            // ca3bb cmp esi,0x2d
            fl = sub::<32>((rsi as u32 as u64), 0x2du64).1;
            // ca3be jg 0xca63d
            if fl.g() { pc = 0xca63d; continue; }
            pc = 0xca3c4;
        }
        0xca3c4 => {
            // block ca3c4
            // ca3c4 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca3c6 cmp esi,0x27
            fl = sub::<32>((rsi as u32 as u64), 0x27u64).1;
            pc = 0xca3c9;
        }
        0xca3c9 => {
            // block ca3c9
            // ca3c9 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca3cf;
        }
        0xca3cf => {
            // block ca3cf
            // ca3cf movabs rdx,0xd166300000
            rdx = 0x0_00d1_6630_0000u64;
            // ca3d9 jmp 0xca5ec
            pc = 0xca5ec; continue;
        }
        0xca3de => {
            // block ca3de
            // ca3de cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // ca3e4 jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca3ea;
        }
        0xca3ea => {
            // block ca3ea
            // ca3ea mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca3ec cmp esi,0x5d
            fl = sub::<32>((rsi as u32 as u64), 0x5du64).1;
            // ca3ef jg 0xca53c
            if fl.g() { pc = 0xca53c; continue; }
            pc = 0xca3f5;
        }
        0xca3f5 => {
            // block ca3f5
            // ca3f5 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca3f7 cmp esi,0x40
            fl = sub::<32>((rsi as u32 as u64), 0x40u64).1;
            // ca3fa ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca400;
        }
        0xca400 => {
            // block ca400
            // ca400 lea rdx,[rip+0x515] # 0xca91c
            rdx = m.env.code_va(0xca91c);
            // ca407 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca40b add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca40e jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca410 => {
            // block ca410
            // ca410 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // ca416 jne 0xca512
            if fl.ne() { pc = 0xca512; continue; }
            pc = 0xca41c;
        }
        0xca41c => {
            // block ca41c
            // ca41c mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca41e cmp esi,0x61
            fl = sub::<32>((rsi as u32 as u64), 0x61u64).1;
            // ca421 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca427;
        }
        0xca427 => {
            // block ca427
            // ca427 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca429 lea rdx,[rip+0x900] # 0xcad30
            rdx = m.env.code_va(0xcad30);
            // ca430 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca434 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca437 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca439 => {
            // block ca439
            // ca439 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // ca43f jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca445;
        }
        0xca445 => {
            // block ca445
            // ca445 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca447 cmp esi,0x5d
            fl = sub::<32>((rsi as u32 as u64), 0x5du64).1;
            // ca44a jle 0xca5d7
            if fl.le() { pc = 0xca5d7; continue; }
            pc = 0xca450;
        }
        0xca450 => {
            // block ca450
            // ca450 add esi,0xffffffa2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffa2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca453 jmp 0xca4da
            pc = 0xca4da; continue;
        }
        0xca458 => {
            // block ca458
            // ca458 cmp edi,0x10710851
            fl = sub::<32>((rdi as u32 as u64), 0x10710851u64).1;
            // ca45e jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca464;
        }
        0xca464 => {
            // block ca464
            // ca464 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca466 cmp esi,0x5d
            fl = sub::<32>((rsi as u32 as u64), 0x5du64).1;
            // ca469 jg 0xca486
            if fl.g() { pc = 0xca486; continue; }
            pc = 0xca46b;
        }
        0xca46b => {
            // block ca46b
            // ca46b mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca46d cmp esi,0x40
            fl = sub::<32>((rsi as u32 as u64), 0x40u64).1;
            // ca470 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca476;
        }
        0xca476 => {
            // block ca476
            // ca476 lea rdx,[rip+0x297] # 0xca714
            rdx = m.env.code_va(0xca714);
            // ca47d movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // ca481 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ca484 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xca486 => {
            // block ca486
            // ca486 add esi,0xffffffa2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffa2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca489 cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // ca48c ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca492;
        }
        0xca492 => {
            // block ca492
            // ca492 mov ecx,0x200f
            rcx = (0x200fu64) as u32 as u64;
            // ca497 jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca49c => {
            // block ca49c
            // ca49c cmp esi,0x3a
            fl = sub::<32>((rsi as u32 as u64), 0x3au64).1;
            // ca49f jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca4a5;
        }
        0xca4a5 => {
            // block ca4a5
            // ca4a5 movabs rax,0x3fd40c966f0f0ff
            rax = 0x3fd_40c9_66f0_f0ffu64;
            // ca4af jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca4b1 => {
            // block ca4b1
            // ca4b1 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // ca4b7 jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca4bd;
        }
        0xca4bd => {
            // block ca4bd
            // ca4bd cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca4c0 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca4c6;
        }
        0xca4c6 => {
            // block ca4c6
            // ca4c6 movabs rax,0x40dd663030f3
            rax = 0x0_40dd_6630_30f3u64;
            // ca4d0 jmp 0xca507
            pc = 0xca507; continue;
        }
        0xca4d2 => {
            // block ca4d2
            // ca4d2 cmp esi,0x25
            fl = sub::<32>((rsi as u32 as u64), 0x25u64).1;
            // ca4d5 jg 0xca552
            if fl.g() { pc = 0xca552; continue; }
            pc = 0xca4d7;
        }
        0xca4d7 => {
            // block ca4d7
            // ca4d7 add esi,0xfffffff4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffff4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xca4da;
        }
        0xca4da => {
            // block ca4da
            // ca4da cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // ca4dd jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca4e3;
        }
        0xca4e3 => {
            // block ca4e3
            // ca4e3 jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca4e8 => {
            // block ca4e8
            // ca4e8 cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // ca4ee jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca4f4;
        }
        0xca4f4 => {
            // block ca4f4
            // ca4f4 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca4f7 jae 0xca6f1
            if fl.ae() { pc = 0xca6f1; continue; }
            pc = 0xca4fd;
        }
        0xca4fd => {
            // block ca4fd
            // ca4fd movabs rax,0x408800010001
            rax = 0x0_4088_0001_0001u64;
            pc = 0xca507;
        }
        0xca507 => {
            // block ca507
            // ca507 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // ca50a shr rax,cl
            { let (r, f) = shr::<64>(rax, (rcx as u8 as u64), fl); rax = r; fl = f; }
            // ca50d and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // ca510 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ca511 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xca512 => {
            // block ca512
            // ca512 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // ca518 jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca51e;
        }
        0xca51e => {
            // block ca51e
            // ca51e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca520 cmp esi,0x13
            fl = sub::<32>((rsi as u32 as u64), 0x13u64).1;
            // ca523 jg 0xca655
            if fl.g() { pc = 0xca655; continue; }
            pc = 0xca529;
        }
        0xca529 => {
            // block ca529
            // ca529 cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // ca52c ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca532;
        }
        0xca532 => {
            // block ca532
            // ca532 mov ecx,0x30f3
            rcx = (0x30f3u64) as u32 as u64;
            // ca537 jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca53c => {
            // block ca53c
            // ca53c add esi,0xffffffa2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffa2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca53f cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // ca542 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca548;
        }
        0xca548 => {
            // block ca548
            // ca548 mov ecx,0x341f
            rcx = (0x341fu64) as u32 as u64;
            // ca54d jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca552 => {
            // block ca552
            // ca552 add esi,0xffffffda
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffdau64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca555 cmp esi,0x3b
            fl = sub::<32>((rsi as u32 as u64), 0x3bu64).1;
            // ca558 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca55e;
        }
        0xca55e => {
            // block ca55e
            // ca55e movabs rcx,0xf3f0007800c0501
            rcx = 0xf3f_0007_800c_0501u64;
            // ca568 jmp 0xca6c2
            pc = 0xca6c2; continue;
        }
        0xca56d => {
            // block ca56d
            // ca56d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca56f cmp esi,0x13
            fl = sub::<32>((rsi as u32 as u64), 0x13u64).1;
            // ca572 jle 0xca606
            if fl.le() { pc = 0xca606; continue; }
            pc = 0xca578;
        }
        0xca578 => {
            // block ca578
            // ca578 cmp esi,0x2d
            fl = sub::<32>((rsi as u32 as u64), 0x2du64).1;
            // ca57b jg 0xca6ca
            if fl.g() { pc = 0xca6ca; continue; }
            pc = 0xca581;
        }
        0xca581 => {
            // block ca581
            // ca581 cmp esi,0x14
            fl = sub::<32>((rsi as u32 as u64), 0x14u64).1;
            // ca584 je 0xca6f3
            if fl.e() { pc = 0xca6f3; continue; }
            pc = 0xca58a;
        }
        0xca58a => {
            // block ca58a
            // ca58a cmp esi,0x26
            fl = sub::<32>((rsi as u32 as u64), 0x26u64).1;
            // ca58d jne 0xca6f1
            if fl.ne() { pc = 0xca6f1; continue; }
            pc = 0xca593;
        }
        0xca593 => {
            // block ca593
            // ca593 jmp 0xca6f3
            pc = 0xca6f3; continue;
        }
        0xca598 => {
            // block ca598
            // ca598 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca59a cmp esi,0x13
            fl = sub::<32>((rsi as u32 as u64), 0x13u64).1;
            // ca59d jg 0xca674
            if fl.g() { pc = 0xca674; continue; }
            pc = 0xca5a3;
        }
        0xca5a3 => {
            // block ca5a3
            // ca5a3 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // ca5a6 jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca5ac;
        }
        0xca5ac => {
            // block ca5ac
            // ca5ac jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca5b1 => {
            // block ca5b1
            // ca5b1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca5b3 cmp esi,0x25
            fl = sub::<32>((rsi as u32 as u64), 0x25u64).1;
            // ca5b6 jg 0xca68f
            if fl.g() { pc = 0xca68f; continue; }
            pc = 0xca5bc;
        }
        0xca5bc => {
            // block ca5bc
            // ca5bc cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // ca5bf ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca5c5;
        }
        0xca5c5 => {
            // block ca5c5
            // ca5c5 mov ecx,0x30ff
            rcx = (0x30ffu64) as u32 as u64;
            // ca5ca jmp 0xca614
            pc = 0xca614; continue;
        }
        0xca5cc => {
            // block ca5cc
            // ca5cc mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca5ce cmp esi,0x5c
            fl = sub::<32>((rsi as u32 as u64), 0x5cu64).1;
            pc = 0xca5d1;
        }
        0xca5d1 => {
            // block ca5d1
            // ca5d1 jg 0xca6ec
            if fl.g() { pc = 0xca6ec; continue; }
            pc = 0xca5d7;
        }
        0xca5d7 => {
            // block ca5d7
            // ca5d7 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca5d9 cmp esi,0x39
            fl = sub::<32>((rsi as u32 as u64), 0x39u64).1;
            // ca5dc ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca5e2;
        }
        0xca5e2 => {
            // block ca5e2
            // ca5e2 movabs rdx,0x3fd404066f0f0ff
            rdx = 0x3fd_4040_66f0_f0ffu64;
            pc = 0xca5ec;
        }
        0xca5ec => {
            // block ca5ec
            // ca5ec bt rdx,rcx
            fl.cf = bit::<64>(rdx, rcx);
            // ca5f0 jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca5f6;
        }
        0xca5f6 => {
            // block ca5f6
            // ca5f6 jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca5fb => {
            // block ca5fb
            // ca5fb mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ca5fd cmp esi,0x13
            fl = sub::<32>((rsi as u32 as u64), 0x13u64).1;
            // ca600 jg 0xca6a3
            if fl.g() { pc = 0xca6a3; continue; }
            pc = 0xca606;
        }
        0xca606 => {
            // block ca606
            // ca606 cmp esi,0xc
            fl = sub::<32>((rsi as u32 as u64), 0xcu64).1;
            // ca609 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca60f;
        }
        0xca60f => {
            // block ca60f
            // ca60f mov ecx,0x10ff
            rcx = (0x10ffu64) as u32 as u64;
            pc = 0xca614;
        }
        0xca614 => {
            // block ca614
            // ca614 bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // ca617 jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca61d;
        }
        0xca61d => {
            // block ca61d
            // ca61d jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca622 => {
            // block ca622
            // ca622 add esi,0xffffffda
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffdau64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca625 cmp esi,0x37
            fl = sub::<32>((rsi as u32 as u64), 0x37u64).1;
            // ca628 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca62e;
        }
        0xca62e => {
            // block ca62e
            // ca62e movabs rcx,0x800000018ff501
            rcx = 0x80_0000_018f_f501u64;
            // ca638 jmp 0xca6c2
            pc = 0xca6c2; continue;
        }
        0xca63d => {
            // block ca63d
            // ca63d add esi,0xffffffd2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffd2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca640 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca643 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca649;
        }
        0xca649 => {
            // block ca649
            // ca649 movabs rcx,0x800000020005
            rcx = 0x0_8000_0002_0005u64;
            // ca653 jmp 0xca6c2
            pc = 0xca6c2; continue;
        }
        0xca655 => {
            // block ca655
            // ca655 cmp esi,0x2d
            fl = sub::<32>((rsi as u32 as u64), 0x2du64).1;
            // ca658 jg 0xca6b0
            if fl.g() { pc = 0xca6b0; continue; }
            pc = 0xca65a;
        }
        0xca65a => {
            // block ca65a
            // ca65a mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca65c cmp esi,0x27
            fl = sub::<32>((rsi as u32 as u64), 0x27u64).1;
            // ca65f ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca665;
        }
        0xca665 => {
            // block ca665
            // ca665 movabs rdx,0xd066300000
            rdx = 0x0_00d0_6630_0000u64;
            // ca66f jmp 0xca5ec
            pc = 0xca5ec; continue;
        }
        0xca674 => {
            // block ca674
            // ca674 cmp esi,0x5c
            fl = sub::<32>((rsi as u32 as u64), 0x5cu64).1;
            // ca677 jg 0xca6ec
            if fl.g() { pc = 0xca6ec; continue; }
            pc = 0xca679;
        }
        0xca679 => {
            // block ca679
            // ca679 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca67b cmp esi,0x2e
            fl = sub::<32>((rsi as u32 as u64), 0x2eu64).1;
            // ca67e ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca680;
        }
        0xca680 => {
            // block ca680
            // ca680 movabs rdx,0x404000300000
            rdx = 0x0_4040_0030_0000u64;
            // ca68a jmp 0xca5ec
            pc = 0xca5ec; continue;
        }
        0xca68f => {
            // block ca68f
            // ca68f add esi,0xffffffda
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffdau64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca692 cmp esi,0x37
            fl = sub::<32>((rsi as u32 as u64), 0x37u64).1;
            // ca695 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca697;
        }
        0xca697 => {
            // block ca697
            // ca697 movabs rcx,0x80000000000101
            rcx = 0x80_0000_0000_0101u64;
            // ca6a1 jmp 0xca6c2
            pc = 0xca6c2; continue;
        }
        0xca6a3 => {
            // block ca6a3
            // ca6a3 cmp esi,0x1c
            fl = sub::<32>((rsi as u32 as u64), 0x1cu64).1;
            // ca6a6 jg 0xca6d1
            if fl.g() { pc = 0xca6d1; continue; }
            pc = 0xca6a8;
        }
        0xca6a8 => {
            // block ca6a8
            // ca6a8 add esi,0xffffffec
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffecu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca6ab jmp 0xca4da
            pc = 0xca4da; continue;
        }
        0xca6b0 => {
            // block ca6b0
            // ca6b0 add esi,0xffffffd2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffd2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ca6b3 cmp esi,0x2f
            fl = sub::<32>((rsi as u32 as u64), 0x2fu64).1;
            // ca6b6 ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca6b8;
        }
        0xca6b8 => {
            // block ca6b8
            // ca6b8 movabs rcx,0xc0000003c005
            rcx = 0x0_c000_0003_c005u64;
            pc = 0xca6c2;
        }
        0xca6c2 => {
            // block ca6c2
            // ca6c2 bt rcx,rsi
            fl.cf = bit::<64>(rcx, rsi);
            // ca6c6 jb 0xca6f3
            if fl.b() { pc = 0xca6f3; continue; }
            pc = 0xca6c8;
        }
        0xca6c8 => {
            // block ca6c8
            // ca6c8 jmp 0xca6f1
            pc = 0xca6f1; continue;
        }
        0xca6ca => {
            // block ca6ca
            // ca6ca cmp esi,0x2e
            fl = sub::<32>((rsi as u32 as u64), 0x2eu64).1;
            // ca6cd je 0xca6f3
            if fl.e() { pc = 0xca6f3; continue; }
            pc = 0xca6cf;
        }
        0xca6cf => {
            // block ca6cf
            // ca6cf jmp 0xca6ec
            pc = 0xca6ec; continue;
        }
        0xca6d1 => {
            // block ca6d1
            // ca6d1 cmp esi,0x5c
            fl = sub::<32>((rsi as u32 as u64), 0x5cu64).1;
            // ca6d4 jg 0xca6ec
            if fl.g() { pc = 0xca6ec; continue; }
            pc = 0xca6d6;
        }
        0xca6d6 => {
            // block ca6d6
            // ca6d6 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // ca6d8 cmp esi,0x39
            fl = sub::<32>((rsi as u32 as u64), 0x39u64).1;
            // ca6db ja 0xca6f1
            if fl.a() { pc = 0xca6f1; continue; }
            pc = 0xca6dd;
        }
        0xca6dd => {
            // block ca6dd
            // ca6dd movabs rdx,0x3cd404060000000
            rdx = 0x3cd_4040_6000_0000u64;
            // ca6e7 jmp 0xca5ec
            pc = 0xca5ec; continue;
        }
        0xca6ec => {
            // block ca6ec
            // ca6ec cmp esi,0x5d
            fl = sub::<32>((rsi as u32 as u64), 0x5du64).1;
            // ca6ef je 0xca6f3
            if fl.e() { pc = 0xca6f3; continue; }
            pc = 0xca6f1;
        }
        0xca6f1 => {
            // block ca6f1
            // ca6f1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xca6f3;
        }
        0xca6f3 => {
            // block ca6f3
            // ca6f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ca6f4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8c60 size=4660
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8c60<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc8c60;
    loop {
        match pc {
        0xc8c60 => {
            // block c8c60
            // c8c60 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8c61 mov rbp,rsp
            rbp = rsp;
            // c8c64 cmp edi,0x1032294f
            fl = sub::<32>((rdi as u32 as u64), 0x1032294fu64).1;
            // c8c6a jg 0xc8c9d
            if fl.g() { pc = 0xc8c9d; continue; }
            pc = 0xc8c6c;
        }
        0xc8c6c => {
            // block c8c6c
            // c8c6c cmp edi,0x102802ff
            fl = sub::<32>((rdi as u32 as u64), 0x102802ffu64).1;
            // c8c72 jg 0xc8ce6
            if fl.g() { pc = 0xc8ce6; continue; }
            pc = 0xc8c74;
        }
        0xc8c74 => {
            // block c8c74
            // c8c74 cmp edi,0x10244800
            fl = sub::<32>((rdi as u32 as u64), 0x10244800u64).1;
            // c8c7a je 0xc8dec
            if fl.e() { pc = 0xc8dec; continue; }
            pc = 0xc8c80;
        }
        0xc8c80 => {
            // block c8c80
            // c8c80 cmp edi,0x10266400
            fl = sub::<32>((rdi as u32 as u64), 0x10266400u64).1;
            // c8c86 je 0xc8fdf
            if fl.e() { pc = 0xc8fdf; continue; }
            pc = 0xc8c8c;
        }
        0xc8c8c => {
            // block c8c8c
            // c8c8c cmp edi,0x10266401
            fl = sub::<32>((rdi as u32 as u64), 0x10266401u64).1;
            // c8c92 je 0xc8fdf
            if fl.e() { pc = 0xc8fdf; continue; }
            pc = 0xc8c98;
        }
        0xc8c98 => {
            // block c8c98
            // c8c98 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8c9d => {
            // block c8c9d
            // c8c9d cmp edi,0x106344ff
            fl = sub::<32>((rdi as u32 as u64), 0x106344ffu64).1;
            // c8ca3 jg 0xc8d18
            if fl.g() { pc = 0xc8d18; continue; }
            pc = 0xc8ca5;
        }
        0xc8ca5 => {
            // block c8ca5
            // c8ca5 cmp edi,0x105183ff
            fl = sub::<32>((rdi as u32 as u64), 0x105183ffu64).1;
            // c8cab jg 0xc8d59
            if fl.g() { pc = 0xc8d59; continue; }
            pc = 0xc8cb1;
        }
        0xc8cb1 => {
            // block c8cb1
            // c8cb1 cmp edi,0x104020ff
            fl = sub::<32>((rdi as u32 as u64), 0x104020ffu64).1;
            // c8cb7 jg 0xc8e06
            if fl.g() { pc = 0xc8e06; continue; }
            pc = 0xc8cbd;
        }
        0xc8cbd => {
            // block c8cbd
            // c8cbd cmp edi,0x10322950
            fl = sub::<32>((rdi as u32 as u64), 0x10322950u64).1;
            // c8cc3 je 0xc8e1e
            if fl.e() { pc = 0xc8e1e; continue; }
            pc = 0xc8cc9;
        }
        0xc8cc9 => {
            // block c8cc9
            // c8cc9 cmp edi,0x10352300
            fl = sub::<32>((rdi as u32 as u64), 0x10352300u64).1;
            // c8ccf je 0xc8fc4
            if fl.e() { pc = 0xc8fc4; continue; }
            pc = 0xc8cd5;
        }
        0xc8cd5 => {
            // block c8cd5
            // c8cd5 cmp edi,0x10378800
            fl = sub::<32>((rdi as u32 as u64), 0x10378800u64).1;
            // c8cdb je 0xc8fdf
            if fl.e() { pc = 0xc8fdf; continue; }
            pc = 0xc8ce1;
        }
        0xc8ce1 => {
            // block c8ce1
            // c8ce1 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8ce6 => {
            // block c8ce6
            // c8ce6 cmp edi,0x102946ff
            fl = sub::<32>((rdi as u32 as u64), 0x102946ffu64).1;
            // c8cec jg 0xc8d8e
            if fl.g() { pc = 0xc8d8e; continue; }
            pc = 0xc8cf2;
        }
        0xc8cf2 => {
            // block c8cf2
            // c8cf2 cmp edi,0x10280300
            fl = sub::<32>((rdi as u32 as u64), 0x10280300u64).1;
            // c8cf8 jne 0xc8e43
            if fl.ne() { pc = 0xc8e43; continue; }
            pc = 0xc8cfe;
        }
        0xc8cfe => {
            // block c8cfe
            // c8cfe dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8d00 cmp esi,0x21
            fl = sub::<32>((rsi as u32 as u64), 0x21u64).1;
            // c8d03 jae 0xc8ffb
            if fl.ae() { pc = 0xc8ffb; continue; }
            pc = 0xc8d09;
        }
        0xc8d09 => {
            // block c8d09
            // c8d09 movabs rax,0x1b8010fff
            rax = 0x0_0001_b801_0fffu64;
            // c8d13 jmp 0xc8ff0
            pc = 0xc8ff0; continue;
        }
        0xc8d18 => {
            // block c8d18
            // c8d18 cmp edi,0x106681ff
            fl = sub::<32>((rdi as u32 as u64), 0x106681ffu64).1;
            // c8d1e jg 0xc8db7
            if fl.g() { pc = 0xc8db7; continue; }
            pc = 0xc8d24;
        }
        0xc8d24 => {
            // block c8d24
            // c8d24 cmp edi,0x106466ff
            fl = sub::<32>((rdi as u32 as u64), 0x106466ffu64).1;
            // c8d2a jg 0xc8e54
            if fl.g() { pc = 0xc8e54; continue; }
            pc = 0xc8d30;
        }
        0xc8d30 => {
            // block c8d30
            // c8d30 cmp edi,0x10634500
            fl = sub::<32>((rdi as u32 as u64), 0x10634500u64).1;
            // c8d36 jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8d3c;
        }
        0xc8d3c => {
            // block c8d3c
            // c8d3c dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8d3e cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8d41 ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8d47;
        }
        0xc8d47 => {
            // block c8d47
            // c8d47 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8d49 lea rcx,[rip+0x66c] # 0xc93bc
            rcx = m.env.code_va(0xc93bc);
            // c8d50 movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8d54 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8d57 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8d59 => {
            // block c8d59
            // c8d59 cmp edi,0x105381ff
            fl = sub::<32>((rdi as u32 as u64), 0x105381ffu64).1;
            // c8d5f jg 0xc8e9e
            if fl.g() { pc = 0xc8e9e; continue; }
            pc = 0xc8d65;
        }
        0xc8d65 => {
            // block c8d65
            // c8d65 cmp edi,0x10518400
            fl = sub::<32>((rdi as u32 as u64), 0x10518400u64).1;
            // c8d6b jne 0xc8f28
            if fl.ne() { pc = 0xc8f28; continue; }
            pc = 0xc8d71;
        }
        0xc8d71 => {
            // block c8d71
            // c8d71 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8d73 cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8d76 ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8d7c;
        }
        0xc8d7c => {
            // block c8d7c
            // c8d7c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8d7e lea rcx,[rip+0x9d7] # 0xc975c
            rcx = m.env.code_va(0xc975c);
            // c8d85 movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8d89 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8d8c jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8d8e => {
            // block c8d8e
            // c8d8e cmp edi,0x10294700
            fl = sub::<32>((rdi as u32 as u64), 0x10294700u64).1;
            // c8d94 jne 0xc8ec7
            if fl.ne() { pc = 0xc8ec7; continue; }
            pc = 0xc8d9a;
        }
        0xc8d9a => {
            // block c8d9a
            // c8d9a dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8d9c cmp esi,0x71
            fl = sub::<32>((rsi as u32 as u64), 0x71u64).1;
            // c8d9f ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8da5;
        }
        0xc8da5 => {
            // block c8da5
            // c8da5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8da7 lea rcx,[rip+0xf1e] # 0xc9ccc
            rcx = m.env.code_va(0xc9ccc);
            // c8dae movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8db2 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8db5 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8db7 => {
            // block c8db7
            // c8db7 cmp edi,0x1071084f
            fl = sub::<32>((rdi as u32 as u64), 0x1071084fu64).1;
            // c8dbd jg 0xc8ed8
            if fl.g() { pc = 0xc8ed8; continue; }
            pc = 0xc8dc3;
        }
        0xc8dc3 => {
            // block c8dc3
            // c8dc3 cmp edi,0x10668200
            fl = sub::<32>((rdi as u32 as u64), 0x10668200u64).1;
            // c8dc9 jne 0xc8f39
            if fl.ne() { pc = 0xc8f39; continue; }
            pc = 0xc8dcf;
        }
        0xc8dcf => {
            // block c8dcf
            // c8dcf dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8dd1 cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8dd4 ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8dda;
        }
        0xc8dda => {
            // block c8dda
            // c8dda mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8ddc lea rcx,[rip+0x409] # 0xc91ec
            rcx = m.env.code_va(0xc91ec);
            // c8de3 movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8de7 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8dea jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8dec => {
            // block c8dec
            // c8dec dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8dee cmp esi,0x21
            fl = sub::<32>((rsi as u32 as u64), 0x21u64).1;
            // c8df1 jae 0xc8ffb
            if fl.ae() { pc = 0xc8ffb; continue; }
            pc = 0xc8df7;
        }
        0xc8df7 => {
            // block c8df7
            // c8df7 movabs rax,0x1b801cfff
            rax = 0x0_0001_b801_cfffu64;
            // c8e01 jmp 0xc8ff0
            pc = 0xc8ff0; continue;
        }
        0xc8e06 => {
            // block c8e06
            // c8e06 cmp edi,0x104782ff
            fl = sub::<32>((rdi as u32 as u64), 0x104782ffu64).1;
            // c8e0c jg 0xc8f62
            if fl.g() { pc = 0xc8f62; continue; }
            pc = 0xc8e12;
        }
        0xc8e12 => {
            // block c8e12
            // c8e12 cmp edi,0x10402100
            fl = sub::<32>((rdi as u32 as u64), 0x10402100u64).1;
            // c8e18 jne 0xc8fbc
            if fl.ne() { pc = 0xc8fbc; continue; }
            pc = 0xc8e1e;
        }
        0xc8e1e => {
            // block c8e1e
            // c8e1e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8e20 cmp esi,0x52
            fl = sub::<32>((rsi as u32 as u64), 0x52u64).1;
            // c8e23 jg 0xc8fa8
            if fl.g() { pc = 0xc8fa8; continue; }
            pc = 0xc8e29;
        }
        0xc8e29 => {
            // block c8e29
            // c8e29 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // c8e2b cmp esi,0x21
            fl = sub::<32>((rsi as u32 as u64), 0x21u64).1;
            // c8e2e ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8e34;
        }
        0xc8e34 => {
            // block c8e34
            // c8e34 movabs rdx,0x37ffffffe
            rdx = 0x0_0003_7fff_fffeu64;
            // c8e3e jmp 0xc8f08
            pc = 0xc8f08; continue;
        }
        0xc8e43 => {
            // block c8e43
            // c8e43 cmp edi,0x10293000
            fl = sub::<32>((rdi as u32 as u64), 0x10293000u64).1;
            // c8e49 je 0xc8fdf
            if fl.e() { pc = 0xc8fdf; continue; }
            pc = 0xc8e4f;
        }
        0xc8e4f => {
            // block c8e4f
            // c8e4f jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8e54 => {
            // block c8e54
            // c8e54 add edi,0xef9b9900
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xef9b9900u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8e5a cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // c8e5d ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8e63;
        }
        0xc8e63 => {
            // block c8e63
            // c8e63 lea rax,[rip+0x196] # 0xc9000
            rax = m.env.code_va(0xc9000);
            // c8e6a movsxd rcx,DWORD PTR [rax+rdi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c8e6e add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // c8e71 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8e73 => {
            // block c8e73
            // c8e73 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8e75 lea ecx,[rsi-0x1]
            rcx = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // c8e78 cmp ecx,0x1e
            fl = sub::<32>((rcx as u32 as u64), 0x1eu64).1;
            // c8e7b jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8e81;
        }
        0xc8e81 => {
            // block c8e81
            // c8e81 lea ecx,[rsi-0x6e]
            rcx = (rsi.wrapping_add(0xffff_ffff_ffff_ff92)) as u32 as u64;
            // c8e84 cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // c8e87 jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8e8d;
        }
        0xc8e8d => {
            // block c8e8d
            // c8e8d add esi,0xffffffe0
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffe0u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8e90 cmp esi,0x4
            fl = sub::<32>((rsi as u32 as u64), 0x4u64).1;
            // c8e93 jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8e99;
        }
        0xc8e99 => {
            // block c8e99
            // c8e99 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8e9e => {
            // block c8e9e
            // c8e9e cmp edi,0x10538200
            fl = sub::<32>((rdi as u32 as u64), 0x10538200u64).1;
            // c8ea4 jne 0xc8f87
            if fl.ne() { pc = 0xc8f87; continue; }
            pc = 0xc8eaa;
        }
        0xc8eaa => {
            // block c8eaa
            // c8eaa dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8eac cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8eaf ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8eb5;
        }
        0xc8eb5 => {
            // block c8eb5
            // c8eb5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8eb7 lea rcx,[rip+0xa6e] # 0xc992c
            rcx = m.env.code_va(0xc992c);
            // c8ebe movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8ec2 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8ec5 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8ec7 => {
            // block c8ec7
            // c8ec7 cmp edi,0x10294900
            fl = sub::<32>((rdi as u32 as u64), 0x10294900u64).1;
            // c8ecd jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8ed3;
        }
        0xc8ed3 => {
            // block c8ed3
            // c8ed3 jmp 0xc8fdf
            pc = 0xc8fdf; continue;
        }
        0xc8ed8 => {
            // block c8ed8
            // c8ed8 cmp edi,0x10710850
            fl = sub::<32>((rdi as u32 as u64), 0x10710850u64).1;
            // c8ede je 0xc8eec
            if fl.e() { pc = 0xc8eec; continue; }
            pc = 0xc8ee0;
        }
        0xc8ee0 => {
            // block c8ee0
            // c8ee0 cmp edi,0x10710851
            fl = sub::<32>((rdi as u32 as u64), 0x10710851u64).1;
            // c8ee6 jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8eec;
        }
        0xc8eec => {
            // block c8eec
            // c8eec mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8eee cmp esi,0x6d
            fl = sub::<32>((rsi as u32 as u64), 0x6du64).1;
            // c8ef1 jg 0xc8f17
            if fl.g() { pc = 0xc8f17; continue; }
            pc = 0xc8ef3;
        }
        0xc8ef3 => {
            // block c8ef3
            // c8ef3 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // c8ef5 cmp esi,0x23
            fl = sub::<32>((rsi as u32 as u64), 0x23u64).1;
            // c8ef8 ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8efe;
        }
        0xc8efe => {
            // block c8efe
            // c8efe movabs rdx,0xb07c27f3e
            rdx = 0x0_000b_07c2_7f3eu64;
            pc = 0xc8f08;
        }
        0xc8f08 => {
            // block c8f08
            // c8f08 bt rdx,rcx
            fl.cf = bit::<64>(rdx, rcx);
            // c8f0c jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8f12;
        }
        0xc8f12 => {
            // block c8f12
            // c8f12 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8f17 => {
            // block c8f17
            // c8f17 add esi,0xffffff92
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffff92u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8f1a cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // c8f1d jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8f23;
        }
        0xc8f23 => {
            // block c8f23
            // c8f23 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8f28 => {
            // block c8f28
            // c8f28 cmp edi,0x10518450
            fl = sub::<32>((rdi as u32 as u64), 0x10518450u64).1;
            // c8f2e je 0xc8fdf
            if fl.e() { pc = 0xc8fdf; continue; }
            pc = 0xc8f34;
        }
        0xc8f34 => {
            // block c8f34
            // c8f34 jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8f39 => {
            // block c8f39
            // c8f39 cmp edi,0x10710800
            fl = sub::<32>((rdi as u32 as u64), 0x10710800u64).1;
            // c8f3f jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8f45;
        }
        0xc8f45 => {
            // block c8f45
            // c8f45 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8f47 cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8f4a ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8f50;
        }
        0xc8f50 => {
            // block c8f50
            // c8f50 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8f52 lea rcx,[rip+0xc3] # 0xc901c
            rcx = m.env.code_va(0xc901c);
            // c8f59 movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8f5d add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8f60 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8f62 => {
            // block c8f62
            // c8f62 cmp edi,0x10478300
            fl = sub::<32>((rdi as u32 as u64), 0x10478300u64).1;
            // c8f68 jne 0xc8fd7
            if fl.ne() { pc = 0xc8fd7; continue; }
            pc = 0xc8f6a;
        }
        0xc8f6a => {
            // block c8f6a
            // c8f6a dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8f6c cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8f6f ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8f75;
        }
        0xc8f75 => {
            // block c8f75
            // c8f75 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8f77 lea rcx,[rip+0xb7e] # 0xc9afc
            rcx = m.env.code_va(0xc9afc);
            // c8f7e movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8f82 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8f85 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8f87 => {
            // block c8f87
            // c8f87 cmp edi,0x10565400
            fl = sub::<32>((rdi as u32 as u64), 0x10565400u64).1;
            // c8f8d jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8f8f;
        }
        0xc8f8f => {
            // block c8f8f
            // c8f8f dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8f91 cmp esi,0x73
            fl = sub::<32>((rsi as u32 as u64), 0x73u64).1;
            // c8f94 ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8f96;
        }
        0xc8f96 => {
            // block c8f96
            // c8f96 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // c8f98 lea rcx,[rip+0x5ed] # 0xc958c
            rcx = m.env.code_va(0xc958c);
            // c8f9f movsxd rdx,DWORD PTR [rcx+rsi*4]
            rdx = sx::<32>(m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // c8fa3 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // c8fa6 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc8fa8 => {
            // block c8fa8
            // c8fa8 add esi,0xffffffad
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xffffffadu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c8fab cmp esi,0x1e
            fl = sub::<32>((rsi as u32 as u64), 0x1eu64).1;
            // c8fae ja 0xc8ffb
            if fl.a() { pc = 0xc8ffb; continue; }
            pc = 0xc8fb0;
        }
        0xc8fb0 => {
            // block c8fb0
            // c8fb0 mov ecx,0x600067c7
            rcx = (0x600067c7u64) as u32 as u64;
            // c8fb5 bt ecx,esi
            fl.cf = bit::<32>((rcx as u32 as u64), (rsi as u32 as u64));
            // c8fb8 jb 0xc8ffd
            if fl.b() { pc = 0xc8ffd; continue; }
            pc = 0xc8fba;
        }
        0xc8fba => {
            // block c8fba
            // c8fba jmp 0xc8ffb
            pc = 0xc8ffb; continue;
        }
        0xc8fbc => {
            // block c8fbc
            // c8fbc cmp edi,0x10416000
            fl = sub::<32>((rdi as u32 as u64), 0x10416000u64).1;
            // c8fc2 jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8fc4;
        }
        0xc8fc4 => {
            // block c8fc4
            // c8fc4 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // c8fc6 cmp esi,0x21
            fl = sub::<32>((rsi as u32 as u64), 0x21u64).1;
            // c8fc9 jae 0xc8ffb
            if fl.ae() { pc = 0xc8ffb; continue; }
            pc = 0xc8fcb;
        }
        0xc8fcb => {
            // block c8fcb
            // c8fcb movabs rax,0x1be01ffff
            rax = 0x0_0001_be01_ffffu64;
            // c8fd5 jmp 0xc8ff0
            pc = 0xc8ff0; continue;
        }
        0xc8fd7 => {
            // block c8fd7
            // c8fd7 cmp edi,0x10478350
            fl = sub::<32>((rdi as u32 as u64), 0x10478350u64).1;
            // c8fdd jne 0xc8ffb
            if fl.ne() { pc = 0xc8ffb; continue; }
            pc = 0xc8fdf;
        }
        0xc8fdf => {
            // block c8fdf
            // c8fdf dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            pc = 0xc8fe1;
        }
        0xc8fe1 => {
            // block c8fe1
            // c8fe1 cmp esi,0x23
            fl = sub::<32>((rsi as u32 as u64), 0x23u64).1;
            // c8fe4 jae 0xc8ffb
            if fl.ae() { pc = 0xc8ffb; continue; }
            pc = 0xc8fe6;
        }
        0xc8fe6 => {
            // block c8fe6
            // c8fe6 movabs rax,0x7bfffffff
            rax = 0x0_0007_bfff_ffffu64;
            pc = 0xc8ff0;
        }
        0xc8ff0 => {
            // block c8ff0
            // c8ff0 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // c8ff3 shr rax,cl
            { let (r, f) = shr::<64>(rax, (rcx as u8 as u64), fl); rax = r; fl = f; }
            // c8ff6 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c8ff9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8ffa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc8ffb => {
            // block c8ffb
            // c8ffb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc8ffd;
        }
        0xc8ffd => {
            // block c8ffd
            // c8ffd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8ffe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33e6d size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033e6d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x33e6d;
    loop {
        match pc {
        0x33e6d => {
            // block 33e6d
            // 33e6d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33e6e mov rbp,rsp
            rbp = rsp;
            // 33e71 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 33e73 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 33e74 mov ebx,0xbe8
            rbx = (0xbe8u64) as u32 as u64;
            // 33e79 lea r14,[rip+0x18dbf0] # 0x1c1a70
            r14 = m.env.data_va(877117);
            pc = 0x33e80;
        }
        0x33e80 => {
            // block 33e80
            // 33e80 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 33e84 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 33e89 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 33e8d cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 33e91 jne 0x33e80
            if fl.ne() { pc = 0x33e80; continue; }
            pc = 0x33e93;
        }
        0x33e93 => {
            // block 33e93
            // 33e93 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 33e94 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 33e96 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33e97 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6860e size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006860e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6860e;
    loop {
        match pc {
        0x6860e => {
            // block 6860e
            // 6860e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6860f mov rbp,rsp
            rbp = rsp;
            // 68612 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68614 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68615 mov ebx,0x48
            rbx = (0x48u64) as u32 as u64;
            // 6861a lea r14,[rip+0xdf11f] # 0x147740
            r14 = m.env.data_va(376589);
            pc = 0x68621;
        }
        0x68621 => {
            // block 68621
            // 68621 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68625 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6862a add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 6862e cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68632 jne 0x68621
            if fl.ne() { pc = 0x68621; continue; }
            pc = 0x68634;
        }
        0x68634 => {
            // block 68634
            // 68634 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68635 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68637 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68638 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68664 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068664<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68664;
    loop {
        match pc {
        0x68664 => {
            // block 68664
            // 68664 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68665 mov rbp,rsp
            rbp = rsp;
            // 68668 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6866a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6866b mov ebx,0x60
            rbx = (0x60u64) as u32 as u64;
            // 68670 lea r14,[rip+0xdf219] # 0x147890
            r14 = m.env.data_va(376925);
            pc = 0x68677;
        }
        0x68677 => {
            // block 68677
            // 68677 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 6867b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68680 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68684 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68688 jne 0x68677
            if fl.ne() { pc = 0x68677; continue; }
            pc = 0x6868a;
        }
        0x6868a => {
            // block 6868a
            // 6868a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6868b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6868d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6868e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x686ba size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000686ba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x686ba;
    loop {
        match pc {
        0x686ba => {
            // block 686ba
            // 686ba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 686bb mov rbp,rsp
            rbp = rsp;
            // 686be push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 686c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 686c1 mov ebx,0x48
            rbx = (0x48u64) as u32 as u64;
            // 686c6 lea r14,[rip+0xdf5b3] # 0x147c80
            r14 = m.env.data_va(377933);
            pc = 0x686cd;
        }
        0x686cd => {
            // block 686cd
            // 686cd lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 686d1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 686d6 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 686da cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 686de jne 0x686cd
            if fl.ne() { pc = 0x686cd; continue; }
            pc = 0x686e0;
        }
        0x686e0 => {
            // block 686e0
            // 686e0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 686e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 686e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 686e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68710 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068710<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68710;
    loop {
        match pc {
        0x68710 => {
            // block 68710
            // 68710 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68711 mov rbp,rsp
            rbp = rsp;
            // 68714 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68716 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68717 mov ebx,0xf0
            rbx = (0xf0u64) as u32 as u64;
            // 6871c lea r14,[rip+0xdf8dd] # 0x148000
            r14 = m.env.data_va(378829);
            pc = 0x68723;
        }
        0x68723 => {
            // block 68723
            // 68723 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68727 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6872c add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68730 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68734 jne 0x68723
            if fl.ne() { pc = 0x68723; continue; }
            pc = 0x68736;
        }
        0x68736 => {
            // block 68736
            // 68736 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68737 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68739 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6873a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68766 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068766<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68766;
    loop {
        match pc {
        0x68766 => {
            // block 68766
            // 68766 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68767 mov rbp,rsp
            rbp = rsp;
            // 6876a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6876c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6876d mov ebx,0x2e8
            rbx = (0x2e8u64) as u32 as u64;
            // 68772 lea r14,[rip+0xdfaf7] # 0x148270
            r14 = m.env.data_va(379453);
            pc = 0x68779;
        }
        0x68779 => {
            // block 68779
            // 68779 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 6877d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68782 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68786 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 6878a jne 0x68779
            if fl.ne() { pc = 0x68779; continue; }
            pc = 0x6878c;
        }
        0x6878c => {
            // block 6878c
            // 6878c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6878d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6878f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68790 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x687bc size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000687bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x687bc;
    loop {
        match pc {
        0x687bc => {
            // block 687bc
            // 687bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 687bd mov rbp,rsp
            rbp = rsp;
            // 687c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 687c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 687c3 mov ebx,0x48
            rbx = (0x48u64) as u32 as u64;
            // 687c8 lea r14,[rip+0xe0141] # 0x148910
            r14 = m.env.data_va(381149);
            pc = 0x687cf;
        }
        0x687cf => {
            // block 687cf
            // 687cf lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 687d3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 687d8 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 687dc cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 687e0 jne 0x687cf
            if fl.ne() { pc = 0x687cf; continue; }
            pc = 0x687e2;
        }
        0x687e2 => {
            // block 687e2
            // 687e2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 687e3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 687e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 687e6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68812 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068812<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68812;
    loop {
        match pc {
        0x68812 => {
            // block 68812
            // 68812 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68813 mov rbp,rsp
            rbp = rsp;
            // 68816 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68818 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68819 mov ebx,0xa8
            rbx = (0xa8u64) as u32 as u64;
            // 6881e lea r14,[rip+0xe05ab] # 0x148dd0
            r14 = m.env.data_va(382365);
            pc = 0x68825;
        }
        0x68825 => {
            // block 68825
            // 68825 lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 68829 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6882e add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68832 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 68836 jne 0x68825
            if fl.ne() { pc = 0x68825; continue; }
            pc = 0x68838;
        }
        0x68838 => {
            // block 68838
            // 68838 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68839 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6883b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6883c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68868 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068868<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68868;
    loop {
        match pc {
        0x68868 => {
            // block 68868
            // 68868 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68869 mov rbp,rsp
            rbp = rsp;
            // 6886c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6886e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6886f mov ebx,0x138
            rbx = (0x138u64) as u32 as u64;
            // 68874 lea r14,[rip+0xe0945] # 0x1491c0
            r14 = m.env.data_va(383373);
            pc = 0x6887b;
        }
        0x6887b => {
            // block 6887b
            // 6887b lea rdi,[r14+rbx*1]
            rdi = r14.wrapping_add(rbx);
            // 6887f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68884 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // 68888 cmp rbx,0xffffffffffffffe8
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffe8u64).1;
            // 6888c jne 0x6887b
            if fl.ne() { pc = 0x6887b; continue; }
            pc = 0x6888e;
        }
        0x6888e => {
            // block 6888e
            // 6888e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6888f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68891 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68892 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63cb3 size=252
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063cb3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x63cb3;
    loop {
        match pc {
        0x63cb3 => {
            // block 63cb3
            // 63cb3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63cb4 mov rbp,rsp
            rbp = rsp;
            // 63cb7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 63cb9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 63cbb push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 63cbd push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 63cbf push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63cc0 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63cc4 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 63cc8 mov al,BYTE PTR [rip+0x1645ea] # 0x1c82b8
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(903813)) as u64) as u8 as u64);
            // 63cce test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63cd0 jne 0x63d2c
            if fl.ne() { pc = 0x63d2c; continue; }
            pc = 0x63cd2;
        }
        0x63cd2 => {
            // block 63cd2
            // 63cd2 lea rdi,[rip+0x1645df] # 0x1c82b8
            rdi = m.env.data_va(903813);
            // 63cd9 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63cde test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63ce0 je 0x63d2c
            if fl.e() { pc = 0x63d2c; continue; }
            pc = 0x63ce2;
        }
        0x63ce2 => {
            // block 63ce2
            // 63ce2 mov QWORD PTR [rip+0x1645c3],0x0 # 0x1c82b0
            m.w64(m.env.data_va(903805), (0x0u64) as u64);
            // 63ced mov QWORD PTR [rip+0x1645b0],0x0 # 0x1c82a8
            m.w64(m.env.data_va(903797), (0x0u64) as u64);
            // 63cf8 lea rsi,[rip+0x1645a1] # 0x1c82a0
            rsi = m.env.data_va(903789);
            // 63cff lea rax,[rip+0x1645a2] # 0x1c82a8
            rax = m.env.data_va(903797);
            // 63d06 mov QWORD PTR [rip+0x164593],rax # 0x1c82a0
            m.w64(m.env.data_va(903789), (rax) as u64);
            // 63d0d lea rdi,[rip+0xfffffffffffb0fc2] # 0x14cd6
            rdi = m.env.code_va(0x14cd6);
            // 63d14 lea rdx,[rip+0xfffffffffff9b455] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 63d1b call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d20 lea rdi,[rip+0x164591] # 0x1c82b8
            rdi = m.env.data_va(903813);
            // 63d27 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63d2c;
        }
        0x63d2c => {
            // block 63d2c
            // 63d2c xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 63d2f cmp QWORD PTR [rip+0x164579],0x0 # 0x1c82b0
            fl = sub::<64>(m.r64(m.env.data_va(903805)) as u64, 0x0u64).1;
            // 63d37 jne 0x63d7a
            if fl.ne() { pc = 0x63d7a; continue; }
            pc = 0x63d39;
        }
        0x63d39 => {
            // block 63d39
            // 63d39 lea r14,[rip+0x9ce30] # 0x100b70
            r14 = m.env.data_va(86845);
            // 63d40 lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63d44 lea r15,[rip+0x164555] # 0x1c82a0
            r15 = m.env.data_va(903789);
            pc = 0x63d4b;
        }
        0x63d4b => {
            // block 63d4b
            // 63d4b mov ebx,DWORD PTR [r14]
            rbx = (m.r32(r14) as u64) as u32 as u64;
            // 63d4e mov rdi,r12
            rdi = r12;
            // 63d51 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 63d53 call 0x633ff
            { let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d58 mov rdi,r15
            rdi = r15;
            // 63d5b mov rsi,r12
            rsi = r12;
            // 63d5e call 0xebb9a
            { let r = { let t = m.env.stub_va(0x367); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d63 mov DWORD PTR [rax],ebx
            m.w32(rax, ((rbx as u32 as u64)) as u32);
            // 63d65 mov rdi,r12
            rdi = r12;
            // 63d68 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d6d inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 63d70 add r14,0x4
            { let (r, f) = add::<64>(r14, 0x4u64);
              r14 = r;
              fl = f; }
            // 63d74 cmp r13,0x10
            fl = sub::<64>(r13, 0x10u64).1;
            // 63d78 jb 0x63d4b
            if fl.b() { pc = 0x63d4b; continue; }
            pc = 0x63d7a;
        }
        0x63d7a => {
            // block 63d7a
            // 63d7a lea rdi,[rip+0x16451f] # 0x1c82a0
            rdi = m.env.data_va(903789);
            // 63d81 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 63d85 call 0xebba0
            { let r = { let t = m.env.stub_va(0x36d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63d8a mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 63d8c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 63d90 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63d91 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 63d93 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 63d95 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63d97 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63d99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63d9a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63d9b => {
            // block 63d9b
            // 63d9b mov rbx,rax
            rbx = rax;
            // 63d9e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63da2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63da7 mov rdi,rbx
            rdi = rbx;
            // 63daa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x101e0 size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000101e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x101e0;
    loop {
        match pc {
        0x101e0 => {
            // block 101e0
            // 101e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 101e1 mov rbp,rsp
            rbp = rsp;
            // 101e4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 101e5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 101e6 mov rbx,rdi
            rbx = rdi;
            // 101e9 movdqu xmm0,XMMWORD PTR [rbx+0x20]
            x0 = m.r128(rbx.wrapping_add(0x20));
            // 101ee paddq xmm0,XMMWORD PTR [rip+0xee64a] # 0xfe840
            { let a = x0; let b = m.r128(m.env.data_va(77837)); x0 = ((a as u64).wrapping_add(b as u64) as u128) | ((((a >> 64) as u64).wrapping_add((b >> 64) as u64) as u128) << 64); }
            // 101f6 movdqu XMMWORD PTR [rbx+0x20],xmm0
            m.w128(rbx.wrapping_add(0x20), x0);
            // 101fb movq rax,xmm0
            rax = x0 as u64;
            // 10200 cmp rax,0x800
            fl = sub::<64>(rax, 0x800u64).1;
            // 10206 jb 0x10221
            if fl.b() { pc = 0x10221; continue; }
            pc = 0x10208;
        }
        0x10208 => {
            // block 10208
            // 10208 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1020c mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 1020f call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10214 add QWORD PTR [rbx+0x8],0x8
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x8u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10219 add QWORD PTR [rbx+0x20],0xfffffffffffffc00
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x20)) as u64, 0xffff_ffff_ffff_fc00u64);
              m.w64(rbx.wrapping_add(0x20), (r) as u64);
              fl = f; }
            pc = 0x10221;
        }
        0x10221 => {
            // block 10221
            // 10221 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 10225 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10226 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10227 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x110c6 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000110c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x110c6;
    loop {
        match pc {
        0x110c6 => {
            // block 110c6
            // 110c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 110c7 mov rbp,rsp
            rbp = rsp;
            // 110ca mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 110ce mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 110d2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 110d5 je 0x110ec
            if fl.e() { pc = 0x110ec; continue; }
            pc = 0x110d7;
        }
        0x110d7 => {
            // block 110d7
            // 110d7 lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 110db sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 110de not rdx
            rdx = !rdx;
            // 110e1 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 110e5 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 110e8 mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x110ec;
        }
        0x110ec => {
            // block 110ec
            // 110ec mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 110ef test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 110f2 je 0x110fa
            if fl.e() { pc = 0x110fa; continue; }
            pc = 0x110f4;
        }
        0x110f4 => {
            // block 110f4
            // 110f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 110f5 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x110fa => {
            // block 110fa
            // 110fa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 110fb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a58c size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a58c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a58c;
    loop {
        match pc {
        0x1a58c => {
            // block 1a58c
            // 1a58c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a58d mov rbp,rsp
            rbp = rsp;
            // 1a590 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1a594 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 1a598 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 1a59b je 0x1a5b2
            if fl.e() { pc = 0x1a5b2; continue; }
            pc = 0x1a59d;
        }
        0x1a59d => {
            // block 1a59d
            // 1a59d lea rdx,[rax-0x4]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fffc);
            // 1a5a1 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 1a5a4 not rdx
            rdx = !rdx;
            // 1a5a7 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 1a5ab add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1a5ae mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x1a5b2;
        }
        0x1a5b2 => {
            // block 1a5b2
            // 1a5b2 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 1a5b5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1a5b8 je 0x1a5c0
            if fl.e() { pc = 0x1a5c0; continue; }
            pc = 0x1a5ba;
        }
        0x1a5ba => {
            // block 1a5ba
            // 1a5ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a5bb jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1a5c0 => {
            // block 1a5c0
            // 1a5c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a5c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
