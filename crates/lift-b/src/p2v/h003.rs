//! Mechanically lifted functions (scripts/lift_run.py), module h003. Each one is checked against the original
//! in the world harness (tests/world_h003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe7c96 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7c96<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7c96;
    loop {
        match pc {
        0xe7c96 => {
            // block e7c96
            #[cfg(feature = "trace")] eprintln!("f_000e7c96 pc e7c96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c96 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7c97 mov rbp,rsp
            rbp = rsp;
            // e7c9a dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // e7c9c cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // e7c9f ja 0xe7cb1
            if fl.a() { pc = 0xe7cb1; continue; }
            pc = 0xe7ca1;
        }
        0xe7ca1 => {
            // block e7ca1
            #[cfg(feature = "trace")] eprintln!("f_000e7c96 pc e7ca1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ca1 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // e7ca4 lea rcx,[rip+0x5e815] # 0x1464c0
            rcx = m.env.data_va(371853);
            // e7cab mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7caf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7cb0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7cb1 => {
            // block e7cb1
            #[cfg(feature = "trace")] eprintln!("f_000e7c96 pc e7cb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cb1 lea rax,[rip+0x51bc2] # 0x13987a
            rax = m.env.data_va(319559);
            // e7cb8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7cb9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5e82 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5e82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe5e82;
    loop {
        match pc {
        0xe5e82 => {
            // block e5e82
            #[cfg(feature = "trace")] eprintln!("f_000e5e82 pc e5e82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5e83 mov rbp,rsp
            rbp = rsp;
            // e5e86 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e5e88 jne 0xe5e8d
            if fl.ne() { pc = 0xe5e8d; continue; }
            pc = 0xe5e8a;
        }
        0xe5e8a => {
            // block e5e8a
            #[cfg(feature = "trace")] eprintln!("f_000e5e82 pc e5e8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e8a mov esi,DWORD PTR [rdi+0x8]
            rsi = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe5e8d;
        }
        0xe5e8d => {
            // block e5e8d
            #[cfg(feature = "trace")] eprintln!("f_000e5e82 pc e5e8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e8d dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e5e8f cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e5e92 ja 0xe5ea3
            if fl.a() { pc = 0xe5ea3; continue; }
            pc = 0xe5e94;
        }
        0xe5e94 => {
            // block e5e94
            #[cfg(feature = "trace")] eprintln!("f_000e5e82 pc e5e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e94 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e5e97 lea rcx,[rip+0x43ea2] # 0x129d40
            rcx = m.env.data_va(255245);
            // e5e9e mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // e5ea1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5ea2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe5ea3 => {
            // block e5ea3
            #[cfg(feature = "trace")] eprintln!("f_000e5e82 pc e5ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5ea3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5ea5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5ea6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba33 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba33<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbba33;
    loop {
        match pc {
        0xbba33 => {
            // block bba33
            #[cfg(feature = "trace")] eprintln!("f_000bba33 pc bba33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba33 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba34 mov rbp,rsp
            rbp = rsp;
            // bba37 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bba3c cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bba3f ja 0xbba5b
            if fl.a() { pc = 0xbba5b; continue; }
            pc = 0xbba41;
        }
        0xbba41 => {
            // block bba41
            #[cfg(feature = "trace")] eprintln!("f_000bba33 pc bba41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba41 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bba43 lea rcx,[rip+0x46ff6] # 0x102a40
            rcx = m.env.data_va(94733);
            // bba4a lea rdx,[rip+0x46fcf] # 0x102a20
            rdx = m.env.data_va(94701);
            // bba51 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bba54 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bba58 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbba5b;
        }
        0xbba5b => {
            // block bba5b
            #[cfg(feature = "trace")] eprintln!("f_000bba33 pc bba5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bba5c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba5d size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba5d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbba5d;
    loop {
        match pc {
        0xbba5d => {
            // block bba5d
            #[cfg(feature = "trace")] eprintln!("f_000bba5d pc bba5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba5d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba5e mov rbp,rsp
            rbp = rsp;
            // bba61 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bba66 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bba69 ja 0xbba85
            if fl.a() { pc = 0xbba85; continue; }
            pc = 0xbba6b;
        }
        0xbba6b => {
            // block bba6b
            #[cfg(feature = "trace")] eprintln!("f_000bba5d pc bba6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba6b mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bba6d lea rcx,[rip+0x4700c] # 0x102a80
            rcx = m.env.data_va(94797);
            // bba74 lea rdx,[rip+0x46fe5] # 0x102a60
            rdx = m.env.data_va(94765);
            // bba7b test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bba7e cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bba82 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbba85;
        }
        0xbba85 => {
            // block bba85
            #[cfg(feature = "trace")] eprintln!("f_000bba5d pc bba85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bba85 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bba86 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbbbc size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbbbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbbbc;
    loop {
        match pc {
        0xbbbbc => {
            // block bbbbc
            #[cfg(feature = "trace")] eprintln!("f_000bbbbc pc bbbbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbbbd mov rbp,rsp
            rbp = rsp;
            // bbbc0 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbbc5 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbbc8 ja 0xbbbe4
            if fl.a() { pc = 0xbbbe4; continue; }
            pc = 0xbbbca;
        }
        0xbbbca => {
            // block bbbca
            #[cfg(feature = "trace")] eprintln!("f_000bbbbc pc bbbca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbca mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbbcc lea rcx,[rip+0x4704d] # 0x102c20
            rcx = m.env.data_va(95213);
            // bbbd3 lea rdx,[rip+0x47026] # 0x102c00
            rdx = m.env.data_va(95181);
            // bbbda test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbbdd cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbbe1 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbbe4;
        }
        0xbbbe4 => {
            // block bbbe4
            #[cfg(feature = "trace")] eprintln!("f_000bbbbc pc bbbe4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbbe4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbbe5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbc9b size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbc9b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbc9b;
    loop {
        match pc {
        0xbbc9b => {
            // block bbc9b
            #[cfg(feature = "trace")] eprintln!("f_000bbc9b pc bbc9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbc9b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbc9c mov rbp,rsp
            rbp = rsp;
            // bbc9f mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbca4 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbca7 ja 0xbbcc3
            if fl.a() { pc = 0xbbcc3; continue; }
            pc = 0xbbca9;
        }
        0xbbca9 => {
            // block bbca9
            #[cfg(feature = "trace")] eprintln!("f_000bbc9b pc bbca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbca9 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbcab lea rcx,[rip+0x4701e] # 0x102cd0
            rcx = m.env.data_va(95389);
            // bbcb2 lea rdx,[rip+0x46ff7] # 0x102cb0
            rdx = m.env.data_va(95357);
            // bbcb9 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbcbc cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbcc0 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbcc3;
        }
        0xbbcc3 => {
            // block bbcc3
            #[cfg(feature = "trace")] eprintln!("f_000bbc9b pc bbcc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbcc3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbcc4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbcc5 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbcc5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbcc5;
    loop {
        match pc {
        0xbbcc5 => {
            // block bbcc5
            #[cfg(feature = "trace")] eprintln!("f_000bbcc5 pc bbcc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbcc5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbcc6 mov rbp,rsp
            rbp = rsp;
            // bbcc9 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // bbcce cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbcd1 ja 0xbbced
            if fl.a() { pc = 0xbbced; continue; }
            pc = 0xbbcd3;
        }
        0xbbcd3 => {
            // block bbcd3
            #[cfg(feature = "trace")] eprintln!("f_000bbcc5 pc bbcd3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbcd3 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbcd5 lea rcx,[rip+0x47034] # 0x102d10
            rcx = m.env.data_va(95453);
            // bbcdc lea rdx,[rip+0x4700d] # 0x102cf0
            rdx = m.env.data_va(95421);
            // bbce3 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbce6 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbcea mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbced;
        }
        0xbbced => {
            // block bbced
            #[cfg(feature = "trace")] eprintln!("f_000bbcc5 pc bbced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbced pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbcee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbd19 size=43
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbd19<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbd19;
    loop {
        match pc {
        0xbbd19 => {
            // block bbd19
            #[cfg(feature = "trace")] eprintln!("f_000bbd19 pc bbd19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd19 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbd1a mov rbp,rsp
            rbp = rsp;
            // bbd1d mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbd22 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // bbd25 ja 0xbbd41
            if fl.a() { pc = 0xbbd41; continue; }
            pc = 0xbbd27;
        }
        0xbbd27 => {
            // block bbd27
            #[cfg(feature = "trace")] eprintln!("f_000bbd19 pc bbd27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd27 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // bbd29 lea rcx,[rip+0x47060] # 0x102d90
            rcx = m.env.data_va(95581);
            // bbd30 lea rdx,[rip+0x47039] # 0x102d70
            rdx = m.env.data_va(95549);
            // bbd37 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // bbd3a cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbd3e mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xbbd41;
        }
        0xbbd41 => {
            // block bbd41
            #[cfg(feature = "trace")] eprintln!("f_000bbd19 pc bbd41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbd41 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbd42 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc0aa size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc0aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc0aa;
    loop {
        match pc {
        0xdc0aa => {
            // block dc0aa
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc0ab mov rbp,rsp
            rbp = rsp;
            // dc0ae mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc0b1 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // dc0b5 mov rdx,rcx
            rdx = rcx;
            // dc0b8 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // dc0bb cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // dc0be jae 0xdc0c9
            if fl.ae() { pc = 0xdc0c9; continue; }
            pc = 0xdc0c0;
        }
        0xdc0c0 => {
            // block dc0c0
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0c0 sub rsi,rdx
            { let (r, f) = sub::<64>(rsi, rdx);
              rsi = r;
              fl = f; }
            // dc0c3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0c4 jmp 0xebed6
            { let r = { let t = m.env.stub_va(0x6a3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xdc0c9 => {
            // block dc0c9
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0c9 jbe 0xdc0d7
            if fl.be() { pc = 0xdc0d7; continue; }
            pc = 0xdc0cb;
        }
        0xdc0cb => {
            // block dc0cb
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0cb add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // dc0ce cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // dc0d1 je 0xdc0d7
            if fl.e() { pc = 0xdc0d7; continue; }
            pc = 0xdc0d3;
        }
        0xdc0d3 => {
            // block dc0d3
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0d3 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            pc = 0xdc0d7;
        }
        0xdc0d7 => {
            // block dc0d7
            #[cfg(feature = "trace")] eprintln!("f_000dc0aa pc dc0d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc0d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3e7c size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3e7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3e7c;
    loop {
        match pc {
        0xc3e7c => {
            // block c3e7c
            #[cfg(feature = "trace")] eprintln!("f_000c3e7c pc c3e7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3e7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3e7d mov rbp,rsp
            rbp = rsp;
            // c3e80 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3e82 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3e83 mov rbx,rdi
            rbx = rdi;
            // c3e86 lea rdi,[rbx+0x8]
            rdi = rbx.wrapping_add(0x8);
            // c3e8a lea r14,[rbx+0x10]
            r14 = rbx.wrapping_add(0x10);
            // c3e8e mov rsi,QWORD PTR [rbx+0x10]
            rsi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // c3e92 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3e97 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // c3e9f mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // c3ea3 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // c3eab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3eac pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4270 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4270<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe4270;
    loop {
        match pc {
        0xe4270 => {
            // block e4270
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e4270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4270 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4271 mov rbp,rsp
            rbp = rsp;
            // e4274 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4276 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4277 mov rbx,rdi
            rbx = rdi;
            // e427a mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e427e jmp 0xe428d
            pc = 0xe428d; continue;
        }
        0xe4280 => {
            // block e4280
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e4280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4280 add rdi,0xffffffffffffffe8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffe8u64);
              rdi = r;
              fl = f; }
            // e4284 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // e4288 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe428d;
        }
        0xe428d => {
            // block e428d
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e428d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e428d mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e4291 cmp rdi,r14
            fl = sub::<64>(rdi, r14).1;
            // e4294 jne 0xe4280
            if fl.ne() { pc = 0xe4280; continue; }
            pc = 0xe4296;
        }
        0xe4296 => {
            // block e4296
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e4296 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4296 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e4299 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e429c je 0xe42a7
            if fl.e() { pc = 0xe42a7; continue; }
            pc = 0xe429e;
        }
        0xe429e => {
            // block e429e
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e429e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e429e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e429f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e42a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42a2 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe42a7 => {
            // block e42a7
            #[cfg(feature = "trace")] eprintln!("f_000e4270 pc e42a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e42a7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e42a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e42aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42ab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xded7a size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ded7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xded7a;
    loop {
        match pc {
        0xded7a => {
            // block ded7a
            #[cfg(feature = "trace")] eprintln!("f_000ded7a pc ded7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ded7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ded7b mov rbp,rsp
            rbp = rsp;
            // ded7e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ded80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ded81 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ded84 mov rbx,rdi
            rbx = rdi;
            // ded87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ded8a mov rsi,rdx
            rsi = rdx;
            // ded8d call QWORD PTR [rax+0x1930]
            { let r = { let t = m.r64(rax.wrapping_add(0x1930)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ded93 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ded96 mov rdi,rbx
            rdi = rbx;
            // ded99 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // ded9c call QWORD PTR [rax+0x1970]
            { let r = { let t = m.r64(rax.wrapping_add(0x1970)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deda2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deda5 mov rax,QWORD PTR [rax+0x1930]
            rax = m.r64(rax.wrapping_add(0x1930)) as u64;
            // dedac xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dedae mov rdi,rbx
            rdi = rbx;
            // dedb1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dedb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dedb4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dedb5 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc0bc size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc0bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc0bc;
    loop {
        match pc {
        0xbc0bc => {
            // block bc0bc
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc0bd mov rbp,rsp
            rbp = rsp;
            // bc0c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc0c1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bc0c2 mov rbx,rsi
            rbx = rsi;
            // bc0c5 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // bc0c8 lea rdi,[rip+0x10c559] # 0x1c8628
            rdi = m.env.data_va(904693);
            // bc0cf lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bc0d3 call 0xebdf2
            { let r = { let t = m.env.stub_va(0x5bf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0d8 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc0dd lea rdx,[rip+0x10c54c] # 0x1c8630
            rdx = m.env.data_va(904701);
            // bc0e4 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc0e7 je 0xbc0ec
            if fl.e() { pc = 0xbc0ec; continue; }
            pc = 0xbc0e9;
        }
        0xbc0e9 => {
            // block bc0e9
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0e9 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc0ec;
        }
        0xbc0ec => {
            // block bc0ec
            #[cfg(feature = "trace")] eprintln!("f_000bc0bc pc bc0ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0ec mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // bc0ee cmp ecx,0x6c
            fl = sub::<32>((rcx as u32 as u64), 0x6cu64).1;
            // bc0f1 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc0f4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bc0f8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc0f9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc0fa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc0fc size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc0fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc0fc;
    loop {
        match pc {
        0xbc0fc => {
            // block bc0fc
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc0fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc0fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc0fd mov rbp,rsp
            rbp = rsp;
            // bc100 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc101 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bc102 mov rbx,rsi
            rbx = rsi;
            // bc105 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // bc108 lea rdi,[rip+0x10c561] # 0x1c8670
            rdi = m.env.data_va(904765);
            // bc10f lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // bc113 call 0xebe40
            { let r = { let t = m.env.stub_va(0x60d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc118 mov ecx,0x6c
            rcx = (0x6cu64) as u32 as u64;
            // bc11d lea rdx,[rip+0x10c554] # 0x1c8678
            rdx = m.env.data_va(904773);
            // bc124 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // bc127 je 0xbc12c
            if fl.e() { pc = 0xbc12c; continue; }
            pc = 0xbc129;
        }
        0xbc129 => {
            // block bc129
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc129 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc129 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            pc = 0xbc12c;
        }
        0xbc12c => {
            // block bc12c
            #[cfg(feature = "trace")] eprintln!("f_000bc0fc pc bc12c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc12c mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // bc12e cmp ecx,0x6c
            fl = sub::<32>((rcx as u32 as u64), 0x6cu64).1;
            // bc131 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc134 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bc138 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc139 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc13a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5e42 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5e42<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe5e42;
    loop {
        match pc {
        0xe5e42 => {
            // block e5e42
            #[cfg(feature = "trace")] eprintln!("f_000e5e42 pc e5e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e42 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5e43 mov rbp,rsp
            rbp = rsp;
            // e5e46 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e5e48 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e4a cmp esi,0xd
            fl = sub::<32>((rsi as u32 as u64), 0xdu64).1;
            // e5e4d ja 0xe5e5c
            if fl.a() { pc = 0xe5e5c; continue; }
            pc = 0xe5e4f;
        }
        0xe5e4f => {
            // block e5e4f
            #[cfg(feature = "trace")] eprintln!("f_000e5e42 pc e5e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e4f movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e5e52 lea rcx,[rip+0x43ea7] # 0x129d00
            rcx = m.env.data_va(255181);
            // e5e59 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0xe5e5c;
        }
        0xe5e5c => {
            // block e5e5c
            #[cfg(feature = "trace")] eprintln!("f_000e5e42 pc e5e5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5e5c mov ecx,0xf0fffff
            rcx = (0xf0fffffu64) as u32 as u64;
            // e5e61 and ecx,DWORD PTR [rdi+0x84]
            { let r = (rcx as u32 as u64) & m.r32(rdi.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5e67 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // e5e69 and edx,0xd
            { let r = (rdx as u32 as u64) & 0xdu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5e6c shl edx,0x14
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e5e6f or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e5e71 and eax,0x70
            { let r = (rax as u32 as u64) & 0x70u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e74 shl eax,0x18
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5e77 or eax,edx
            { let r = (rax as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5e79 mov DWORD PTR [rdi+0x84],eax
            m.w32(rdi.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e5e7f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5e80 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0a36 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0a36;
    loop {
        match pc {
        0xe0a36 => {
            // block e0a36
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a37 mov rbp,rsp
            rbp = rsp;
            // e0a3a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0a3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a3d mov rbx,rdi
            rbx = rdi;
            // e0a40 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            // e0a43 test r14,r14
            fl = logic::<64>(r14 & r14);
            // e0a46 je 0xe0a77
            if fl.e() { pc = 0xe0a77; continue; }
            pc = 0xe0a48;
        }
        0xe0a48 => {
            // block e0a48
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a48 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a4c cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a4f je 0xe0a6b
            if fl.e() { pc = 0xe0a6b; continue; }
            pc = 0xe0a51;
        }
        0xe0a51 => {
            // block e0a51
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a51 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // e0a55 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // e0a59 mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e0a5d call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a5f mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // e0a63 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // e0a66 jne 0xe0a51
            if fl.ne() { pc = 0xe0a51; continue; }
            pc = 0xe0a68;
        }
        0xe0a68 => {
            // block e0a68
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a68 mov r14,QWORD PTR [rbx]
            r14 = m.r64(rbx) as u64;
            pc = 0xe0a6b;
        }
        0xe0a6b => {
            // block e0a6b
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a6b mov rdi,r14
            rdi = r14;
            // e0a6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a6f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a71 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a72 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe0a77 => {
            // block e0a77
            #[cfg(feature = "trace")] eprintln!("f_000e0a36 pc e0a77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0a77 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0a78 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0a7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0a7b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5bc0 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5bc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5bc0;
    loop {
        match pc {
        0xd5bc0 => {
            // block d5bc0
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5bc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5bc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5bc1 mov rbp,rsp
            rbp = rsp;
            // d5bc4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5bc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5bc7 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d5bca mov rbx,rdi
            rbx = rdi;
            // d5bcd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bd0 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5bd6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5bd8 je 0xd5bff
            if fl.e() { pc = 0xd5bff; continue; }
            pc = 0xd5bda;
        }
        0xd5bda => {
            // block d5bda
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5bda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5bda mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bdd mov rdi,rbx
            rdi = rbx;
            // d5be0 call QWORD PTR [rax+0xf8]
            { let r = { let t = m.r64(rax.wrapping_add(0xf8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5be6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5be8 je 0xd5c03
            if fl.e() { pc = 0xd5c03; continue; }
            pc = 0xd5bea;
        }
        0xd5bea => {
            // block d5bea
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5bea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5bea test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d5bed je 0xd5bff
            if fl.e() { pc = 0xd5bff; continue; }
            pc = 0xd5bef;
        }
        0xd5bef => {
            // block d5bef
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5bef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5bf2 mov rdi,rbx
            rdi = rbx;
            // d5bf5 call QWORD PTR [rax+0xf0]
            { let r = { let t = m.r64(rax.wrapping_add(0xf0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5bfb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5bfd je 0xd5c03
            if fl.e() { pc = 0xd5c03; continue; }
            pc = 0xd5bff;
        }
        0xd5bff => {
            // block d5bff
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5bff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5bff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5c01 jmp 0xd5c05
            pc = 0xd5c05; continue;
        }
        0xd5c03 => {
            // block d5c03
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c03 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5c05;
        }
        0xd5c05 => {
            // block d5c05
            #[cfg(feature = "trace")] eprintln!("f_000d5bc0 pc d5c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5c06 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5c08 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5c09 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe42ac size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e42ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe42ac;
    loop {
        match pc {
        0xe42ac => {
            // block e42ac
            #[cfg(feature = "trace")] eprintln!("f_000e42ac pc e42ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e42ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e42ad mov rbp,rsp
            rbp = rsp;
            // e42b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e42b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e42b2 mov rbx,rdi
            rbx = rdi;
            // e42b5 mov QWORD PTR [rbx],rsi
            m.w64(rbx, (rsi) as u64);
            // e42b8 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // e42c0 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // e42c8 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // e42d0 mov DWORD PTR [rbx+0x30],0x41c00
            m.w32(rbx.wrapping_add(0x30), (0x41c00u64) as u32);
            // e42d7 lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e42db mov esi,0x2008
            rsi = (0x2008u64) as u32 as u64;
            // e42e0 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e42e5 mov DWORD PTR [rbx+0x2048],0x1
            m.w32(rbx.wrapping_add(0x2048), (0x1u64) as u32);
            // e42ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e42f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e42f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e42f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe73a0 size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e73a0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe73a0;
    loop {
        match pc {
        0xe73a0 => {
            // block e73a0
            #[cfg(feature = "trace")] eprintln!("f_000e73a0 pc e73a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e73a0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e73a1 mov rbp,rsp
            rbp = rsp;
            // e73a4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e73a6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e73a8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e73aa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e73ac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e73ad push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e73ae mov r15,r8
            r15 = r8;
            // e73b1 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // e73b4 mov rbx,rsi
            rbx = rsi;
            // e73b7 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e73ba jle 0xe73db
            if fl.le() { pc = 0xe73db; continue; }
            pc = 0xe73bc;
        }
        0xe73bc => {
            // block e73bc
            #[cfg(feature = "trace")] eprintln!("f_000e73a0 pc e73bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e73bc movsxd r12,edx
            r12 = sx::<32>((rdx as u32 as u64)) as u64;
            // e73bf movsxd r13,r9d
            r13 = sx::<32>((r9 as u32 as u64)) as u64;
            pc = 0xe73c2;
        }
        0xe73c2 => {
            // block e73c2
            #[cfg(feature = "trace")] eprintln!("f_000e73a0 pc e73c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e73c2 mov rdi,r15
            rdi = r15;
            // e73c5 mov rsi,rbx
            rsi = rbx;
            // e73c8 mov rdx,r12
            rdx = r12;
            // e73cb call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e73d0 add rbx,r12
            { let (r, f) = add::<64>(rbx, r12);
              rbx = r;
              fl = f; }
            // e73d3 add r15,r13
            { let (r, f) = add::<64>(r15, r13);
              r15 = r;
              fl = f; }
            // e73d6 dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // e73d9 jne 0xe73c2
            if fl.ne() { pc = 0xe73c2; continue; }
            pc = 0xe73db;
        }
        0xe73db => {
            // block e73db
            #[cfg(feature = "trace")] eprintln!("f_000e73a0 pc e73db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e73db add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e73df pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e73e0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e73e2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e73e4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e73e6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e73e8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e73e9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba9ca size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba9ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba9ca;
    loop {
        match pc {
        0xba9ca => {
            // block ba9ca
            #[cfg(feature = "trace")] eprintln!("f_000ba9ca pc ba9ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba9cb mov rbp,rsp
            rbp = rsp;
            // ba9ce push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba9d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba9d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba9d3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba9d4 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // ba9d7 mov rbx,rdi
            rbx = rdi;
            // ba9da mov DWORD PTR [rbp-0x1c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rsi as u32 as u64)) as u32);
            // ba9dd lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // ba9e1 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // ba9e5 mov rdi,r15
            rdi = r15;
            // ba9e8 call 0xebe16
            { let r = { let t = m.env.stub_va(0x5e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba9ed add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // ba9f1 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // ba9f4 je 0xbaa0b
            if fl.e() { pc = 0xbaa0b; continue; }
            pc = 0xba9f6;
        }
        0xba9f6 => {
            // block ba9f6
            #[cfg(feature = "trace")] eprintln!("f_000ba9ca pc ba9f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9f6 cmp DWORD PTR [rax+0x20],r14d
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (r14 as u32 as u64)).1;
            // ba9fa jne 0xbaa0b
            if fl.ne() { pc = 0xbaa0b; continue; }
            pc = 0xba9fc;
        }
        0xba9fc => {
            // block ba9fc
            #[cfg(feature = "trace")] eprintln!("f_000ba9ca pc ba9fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba9fc mov rdi,r15
            rdi = r15;
            // ba9ff mov rsi,rax
            rsi = rax;
            // baa02 call 0xebe1c
            { let r = { let t = m.env.stub_va(0x5e9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // baa07 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // baa09 jmp 0xbaa0d
            pc = 0xbaa0d; continue;
        }
        0xbaa0b => {
            // block baa0b
            #[cfg(feature = "trace")] eprintln!("f_000ba9ca pc baa0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa0b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xbaa0d;
        }
        0xbaa0d => {
            // block baa0d
            #[cfg(feature = "trace")] eprintln!("f_000ba9ca pc baa0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa0d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // baa11 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // baa12 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // baa14 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // baa16 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // baa17 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9fd8 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9fd8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9fd8;
    loop {
        match pc {
        0xe9fd8 => {
            // block e9fd8
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc e9fd8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9fd8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9fd9 mov rbp,rsp
            rbp = rsp;
            // e9fdc mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // e9fe1 cmp QWORD PTR [rdi+0x78],0x0
            fl = sub::<64>(m.r64(rdi.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9fe6 je 0xea025
            if fl.e() { pc = 0xea025; continue; }
            pc = 0xe9fe8;
        }
        0xe9fe8 => {
            // block e9fe8
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc e9fe8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9fe8 mov rcx,QWORD PTR [rdi+0x18]
            rcx = m.r64(rdi.wrapping_add(0x18)) as u64;
            // e9fec cmp QWORD PTR [rdi+0x10],rcx
            fl = sub::<64>(m.r64(rdi.wrapping_add(0x10)) as u64, rcx).1;
            // e9ff0 jae 0xea025
            if fl.ae() { pc = 0xea025; continue; }
            pc = 0xe9ff2;
        }
        0xe9ff2 => {
            // block e9ff2
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc e9ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ff2 cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // e9ff5 je 0xea01c
            if fl.e() { pc = 0xea01c; continue; }
            pc = 0xe9ff7;
        }
        0xe9ff7 => {
            // block e9ff7
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc e9ff7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9ff7 test BYTE PTR [rdi+0x188],0x10
            fl = logic::<8>(m.r8(rdi.wrapping_add(0x188)) as u64 & 0x10u64);
            // e9ffe jne 0xea00e
            if fl.ne() { pc = 0xea00e; continue; }
            pc = 0xea000;
        }
        0xea000 => {
            // block ea000
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc ea000 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea000 movzx r8d,BYTE PTR [rcx-0x1]
            r8 = (m.r8(rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // ea005 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // ea009 cmp edx,r8d
            fl = sub::<32>((rdx as u32 as u64), (r8 as u32 as u64)).1;
            // ea00c jne 0xea025
            if fl.ne() { pc = 0xea025; continue; }
            pc = 0xea00e;
        }
        0xea00e => {
            // block ea00e
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc ea00e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea00e dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // ea011 mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // ea015 mov BYTE PTR [rcx],sil
            m.w8(rcx, ((rsi as u8 as u64)) as u8);
            // ea018 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // ea01a jmp 0xea025
            pc = 0xea025; continue;
        }
        0xea01c => {
            // block ea01c
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc ea01c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea01c dec rcx
            { let (r, f) = dec::<64>(rcx, fl); rcx = r; fl = f; }
            // ea01f mov QWORD PTR [rdi+0x18],rcx
            m.w64(rdi.wrapping_add(0x18), (rcx) as u64);
            // ea023 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xea025;
        }
        0xea025 => {
            // block ea025
            #[cfg(feature = "trace")] eprintln!("f_000e9fd8 pc ea025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea026 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf1a4 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df1a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdf1a4;
    loop {
        match pc {
        0xdf1a4 => {
            // block df1a4
            #[cfg(feature = "trace")] eprintln!("f_000df1a4 pc df1a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df1a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df1a5 mov rbp,rsp
            rbp = rsp;
            // df1a8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df1a9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // df1aa mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // df1ac call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.call(t, &[], &[x0 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df1b1 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // df1b3 je 0xdf1cf
            if fl.e() { pc = 0xdf1cf; continue; }
            pc = 0xdf1b5;
        }
        0xdf1b5 => {
            // block df1b5
            #[cfg(feature = "trace")] eprintln!("f_000df1a4 pc df1b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df1b5 mulsd xmm0,QWORD PTR [rip+0x23dfb] # 0x102fb8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96133)) as u128))).to_bits());
            // df1bd mulsd xmm0,QWORD PTR [rip+0x23de3] # 0x102fa8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96117)) as u128))).to_bits());
            // df1c5 addsd xmm0,QWORD PTR [rip+0x23df3] # 0x102fc0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(96141)) as u128))).to_bits());
            // df1cd jmp 0xdf1e7
            pc = 0xdf1e7; continue;
        }
        0xdf1cf => {
            // block df1cf
            #[cfg(feature = "trace")] eprintln!("f_000df1a4 pc df1cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df1cf mulsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fa0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96109)) as u128))).to_bits());
            // df1d7 mulsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fa8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(96117)) as u128))).to_bits());
            // df1df addsd xmm0,QWORD PTR [rip+0x23dc9] # 0x102fb0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(96125)) as u128))).to_bits());
            pc = 0xdf1e7;
        }
        0xdf1e7 => {
            // block df1e7
            #[cfg(feature = "trace")] eprintln!("f_000df1a4 pc df1e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df1e7 addsd xmm0,QWORD PTR [rip+0x1f6e9] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // df1ef cvttsd2si eax,xmm0
            rax = (cvtt32(f64_of(x0)) as u64) as u32 as u64;
            // df1f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // df1f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df1f8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df1f9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe126c size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e126c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe126c;
    loop {
        match pc {
        0xe126c => {
            // block e126c
            #[cfg(feature = "trace")] eprintln!("f_000e126c pc e126c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e126c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e126d mov rbp,rsp
            rbp = rsp;
            // e1270 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e1272 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1274 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1275 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1276 mov r14,rdx
            r14 = rdx;
            // e1279 mov rbx,rdi
            rbx = rdi;
            // e127c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e127f mov edx,DWORD PTR [rsi]
            rdx = (m.r32(rsi) as u64) as u32 as u64;
            // e1281 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e1284 mov esi,0x14
            rsi = (0x14u64) as u32 as u64;
            // e1289 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e128e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1291 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1294 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e1296 je 0xe12b4
            if fl.e() { pc = 0xe12b4; continue; }
            pc = 0xe1298;
        }
        0xe1298 => {
            // block e1298
            #[cfg(feature = "trace")] eprintln!("f_000e126c pc e1298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1298 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e129b mov edx,DWORD PTR [r14]
            rdx = (m.r32(r14) as u64) as u32 as u64;
            // e129e mov esi,0x10a
            rsi = (0x10au64) as u32 as u64;
            // e12a3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e12a8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e12ab mov rdi,rbx
            rdi = rbx;
            // e12ae call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e12b1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xe12b4;
        }
        0xe12b4 => {
            // block e12b4
            #[cfg(feature = "trace")] eprintln!("f_000e126c pc e12b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e12b4 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e12b7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e12bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e12bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e12be pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e12c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e12c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3680 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3680<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3680;
    loop {
        match pc {
        0xd3680 => {
            // block d3680
            #[cfg(feature = "trace")] eprintln!("f_000d3680 pc d3680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3680 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3681 mov rbp,rsp
            rbp = rsp;
            // d3684 sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d3688 xorps xmm0,xmm0
            x0 = 0;
            // d368b movaps XMMWORD PTR [rbp-0x10],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fff0), x0);
            // d368f movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // d3693 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d3697 mov DWORD PTR [rbp-0x30],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x2u64) as u32);
            // d369e mov DWORD PTR [rbp-0x2c],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x30u64) as u32);
            // d36a5 mov DWORD PTR [rbp-0x18],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rsi as u32 as u64)) as u32);
            // d36a8 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // d36ab mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d36b2 mov DWORD PTR [rbp-0xc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rcx as u32 as u64)) as u32);
            // d36b5 mov DWORD PTR [rbp-0x8],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), ((r8 as u32 as u64)) as u32);
            // d36b9 mov BYTE PTR [rbp-0x4],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x1u64) as u8);
            // d36bd mov BYTE PTR [rbp-0x3],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffd), (0x0u64) as u8);
            // d36c1 mov BYTE PTR [rbp-0x2],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fffe), (0x0u64) as u8);
            // d36c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d36c8 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d36cc call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d36d2 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // d36d6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d36d7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5388 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5388<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5388;
    loop {
        match pc {
        0xb5388 => {
            // block b5388
            #[cfg(feature = "trace")] eprintln!("f_000b5388 pc b5388 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5388 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5389 mov rbp,rsp
            rbp = rsp;
            // b538c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b538e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5391 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5392 mov r14,rsi
            r14 = rsi;
            // b5395 mov r15,rdi
            r15 = rdi;
            // b5398 mov edi,0x30
            rdi = (0x30u64) as u32 as u64;
            // b539d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b53a2 mov rbx,rax
            rbx = rax;
            // b53a5 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // b53a8 mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // b53ab mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // b53af mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // b53b3 mov rdi,r15
            rdi = r15;
            // b53b6 mov rsi,rbx
            rsi = rbx;
            // b53b9 call 0xebc84
            { let r = { let t = m.env.stub_va(0x451); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b53be mov r14,rax
            r14 = rax;
            // b53c1 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // b53c4 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // b53c8 jne 0xb53d2
            if fl.ne() { pc = 0xb53d2; continue; }
            pc = 0xb53ca;
        }
        0xb53ca => {
            // block b53ca
            #[cfg(feature = "trace")] eprintln!("f_000b5388 pc b53ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b53ca mov rdi,rbx
            rdi = rbx;
            // b53cd call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb53d2;
        }
        0xb53d2 => {
            // block b53d2
            #[cfg(feature = "trace")] eprintln!("f_000b5388 pc b53d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b53d2 mov rax,r14
            rax = r14;
            // b53d5 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b53d8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b53dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b53dd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b53df pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b53e1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b53e2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3193 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3193<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3193;
    loop {
        match pc {
        0xd3193 => {
            // block d3193
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d3193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3193 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3194 mov rbp,rsp
            rbp = rsp;
            // d3197 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3199 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d319b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d319d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d319f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d31a0 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d31a1 mov r14,rdx
            r14 = rdx;
            // d31a4 mov r15,rsi
            r15 = rsi;
            // d31a7 mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // d31aa mov ebx,DWORD PTR [r14]
            rbx = (m.r32(r14) as u64) as u32 as u64;
            // d31ad xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // d31b0 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // d31b2 jle 0xd31da
            if fl.le() { pc = 0xd31da; continue; }
            pc = 0xd31b4;
        }
        0xd31b4 => {
            // block d31b4
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31b4 movsxd rsi,r13d
            rsi = sx::<32>((r13 as u32 as u64)) as u64;
            // d31b7 add rsi,r15
            { let (r, f) = add::<64>(rsi, r15);
              rsi = r;
              fl = f; }
            // d31ba movsxd rdx,ebx
            rdx = sx::<32>((rbx as u32 as u64)) as u64;
            // d31bd xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d31bf mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // d31c2 call 0xec28a
            { let r = { let t = m.env.stub_va(0xa57); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d31c7 cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // d31ca je 0xd31da
            if fl.e() { pc = 0xd31da; continue; }
            pc = 0xd31cc;
        }
        0xd31cc => {
            // block d31cc
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31cc add r13d,eax
            { let (r, f) = add::<32>((r13 as u32 as u64), (rax as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d31cf sub ebx,eax
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // d31d1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d31d3 cmp r13d,DWORD PTR [r14]
            fl = sub::<32>((r13 as u32 as u64), m.r32(r14) as u64).1;
            // d31d6 jl 0xd31b4
            if fl.l() { pc = 0xd31b4; continue; }
            pc = 0xd31d8;
        }
        0xd31d8 => {
            // block d31d8
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31d8 jmp 0xd31df
            pc = 0xd31df; continue;
        }
        0xd31da => {
            // block d31da
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31da mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xd31df;
        }
        0xd31df => {
            // block d31df
            #[cfg(feature = "trace")] eprintln!("f_000d3193 pc d31df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d31df mov DWORD PTR [r14],r13d
            m.w32(r14, ((r13 as u32 as u64)) as u32);
            // d31e2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d31e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d31e7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d31e9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d31eb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d31ed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d31ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d31f0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba6da size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba6da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba6da;
    loop {
        match pc {
        0xba6da => {
            // block ba6da
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba6db mov rbp,rsp
            rbp = rsp;
            // ba6de mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // ba6e2 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba6e5 je 0xba71d
            if fl.e() { pc = 0xba71d; continue; }
            pc = 0xba6e7;
        }
        0xba6e7 => {
            // block ba6e7
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6e7 movzx ecx,BYTE PTR [rdx]
            rcx = (m.r8(rdx) as u64) as u32 as u64;
            // ba6ea jmp 0xba6ef
            pc = 0xba6ef; continue;
        }
        0xba6ec => {
            // block ba6ec
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6ec mov rax,rdx
            rax = rdx;
            pc = 0xba6ef;
        }
        0xba6ef => {
            // block ba6ef
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6ef movzx edx,BYTE PTR [rax+0x19]
            rdx = (m.r8(rax.wrapping_add(0x19)) as u64) as u32 as u64;
            // ba6f3 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // ba6f5 jae 0xba701
            if fl.ae() { pc = 0xba701; continue; }
            pc = 0xba6f7;
        }
        0xba6f7 => {
            // block ba6f7
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6f7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // ba6fa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // ba6fd jne 0xba6ec
            if fl.ne() { pc = 0xba6ec; continue; }
            pc = 0xba6ff;
        }
        0xba6ff => {
            // block ba6ff
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba6ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6ff jmp 0xba729
            pc = 0xba729; continue;
        }
        0xba701 => {
            // block ba701
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba701 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba701 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // ba704 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // ba706 jae 0xba731
            if fl.ae() { pc = 0xba731; continue; }
            pc = 0xba708;
        }
        0xba708 => {
            // block ba708
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba708 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // ba70c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // ba70f jne 0xba6ec
            if fl.ne() { pc = 0xba6ec; continue; }
            pc = 0xba711;
        }
        0xba711 => {
            // block ba711
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba711 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba711 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // ba714 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // ba718 mov rsi,rax
            rsi = rax;
            // ba71b jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba71d => {
            // block ba71d
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba71d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba71d add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // ba721 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // ba724 mov rsi,rdi
            rsi = rdi;
            // ba727 jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba729 => {
            // block ba729
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba729 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba729 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // ba72c mov rsi,rax
            rsi = rax;
            // ba72f jmp 0xba734
            pc = 0xba734; continue;
        }
        0xba731 => {
            // block ba731
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba731 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba731 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xba734;
        }
        0xba734 => {
            // block ba734
            #[cfg(feature = "trace")] eprintln!("f_000ba6da pc ba734 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba734 mov rax,rsi
            rax = rsi;
            // ba737 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba738 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4bd4 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4bd4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe4bd4;
    loop {
        match pc {
        0xe4bd4 => {
            // block e4bd4
            #[cfg(feature = "trace")] eprintln!("f_000e4bd4 pc e4bd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4bd4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4bd5 mov rbp,rsp
            rbp = rsp;
            // e4bd8 lea rax,[rip+0x616c9] # 0x1462a8
            rax = m.env.data_va(371317);
            // e4bdf mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // e4be2 mov QWORD PTR [rdi+0x90],0x0
            m.w64(rdi.wrapping_add(0x90), (0x0u64) as u64);
            // e4bed mov BYTE PTR [rdi+0x8c],0x0
            m.w8(rdi.wrapping_add(0x8c), (0x0u64) as u8);
            // e4bf4 mov WORD PTR [rdi+0xc],0x0
            m.w16(rdi.wrapping_add(0xc), (0x0u64) as u16);
            // e4bfa mov DWORD PTR [rdi+0x8],0x0
            m.w32(rdi.wrapping_add(0x8), (0x0u64) as u32);
            // e4c01 mov QWORD PTR [rdi+0x48],0x0
            m.w64(rdi.wrapping_add(0x48), (0x0u64) as u64);
            // e4c09 mov QWORD PTR [rdi+0x40],0x0
            m.w64(rdi.wrapping_add(0x40), (0x0u64) as u64);
            // e4c11 mov QWORD PTR [rdi+0x38],0x0
            m.w64(rdi.wrapping_add(0x38), (0x0u64) as u64);
            // e4c19 mov QWORD PTR [rdi+0x30],0x0
            m.w64(rdi.wrapping_add(0x30), (0x0u64) as u64);
            // e4c21 mov QWORD PTR [rdi+0x28],0x0
            m.w64(rdi.wrapping_add(0x28), (0x0u64) as u64);
            // e4c29 mov QWORD PTR [rdi+0x20],0x0
            m.w64(rdi.wrapping_add(0x20), (0x0u64) as u64);
            // e4c31 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4c32 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb60d4 size=102
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b60d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb60d4;
    loop {
        match pc {
        0xb60d4 => {
            // block b60d4
            #[cfg(feature = "trace")] eprintln!("f_000b60d4 pc b60d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b60d5 mov rbp,rsp
            rbp = rsp;
            // b60d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b60da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b60dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b60de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b60df mov r15,rdx
            r15 = rdx;
            // b60e2 mov r12,rsi
            r12 = rsi;
            // b60e5 mov rbx,rdi
            rbx = rdi;
            // b60e8 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // b60ec mov edi,0x38
            rdi = (0x38u64) as u32 as u64;
            // b60f1 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b60f6 mov r14,rax
            r14 = rax;
            // b60f9 mov QWORD PTR [rbx],r14
            m.w64(rbx, (r14) as u64);
            // b60fc mov QWORD PTR [rbx+0x8],r12
            m.w64(rbx.wrapping_add(0x8), (r12) as u64);
            // b6100 mov BYTE PTR [rbx+0x10],0x0
            m.w8(rbx.wrapping_add(0x10), (0x0u64) as u8);
            // b6104 lea rdi,[r14+0x20]
            rdi = r14.wrapping_add(0x20);
            // b6108 mov rsi,r15
            rsi = r15;
            // b610b call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6110 mov BYTE PTR [rbx+0x10],0x1
            m.w8(rbx.wrapping_add(0x10), (0x1u64) as u8);
            // b6114 mov rax,rbx
            rax = rbx;
            // b6117 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6118 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b611a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b611c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b611e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b611f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6120 => {
            // block b6120
            #[cfg(feature = "trace")] eprintln!("f_000b60d4 pc b6120 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6120 mov r15,rax
            r15 = rax;
            // b6123 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // b612a mov rdi,r14
            rdi = r14;
            // b612d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6132 mov rdi,r15
            rdi = r15;
            // b6135 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc533c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c533c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc533c;
    loop {
        match pc {
        0xc533c => {
            // block c533c
            #[cfg(feature = "trace")] eprintln!("f_000c533c pc c533c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c533c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c533d mov rbp,rsp
            rbp = rsp;
            // c5340 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c5342 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c5344 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c5346 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c5347 mov r12,rdx
            r12 = rdx;
            // c534a mov r14,rsi
            r14 = rsi;
            // c534d mov rbx,rdi
            rbx = rdi;
            // c5350 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c5355 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c535a mov r15,rax
            r15 = rax;
            // c535d lea rdi,[r15+0x20]
            rdi = r15.wrapping_add(0x20);
            // c5361 mov rsi,r12
            rsi = r12;
            // c5364 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c5369 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c536d mov DWORD PTR [r15+0x38],0x0
            m.w32(r15.wrapping_add(0x38), (0x0u64) as u32);
            // c5375 mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            // c5378 mov QWORD PTR [rbx+0x8],r14
            m.w64(rbx.wrapping_add(0x8), (r14) as u64);
            // c537c mov QWORD PTR [rbx+0x10],0x101
            m.w64(rbx.wrapping_add(0x10), (0x101u64) as u64);
            // c5384 mov rax,rbx
            rax = rbx;
            // c5387 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5388 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c538a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c538c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c538e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c538f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc5390 => {
            // block c5390
            #[cfg(feature = "trace")] eprintln!("f_000c533c pc c5390 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5390 mov rbx,rax
            rbx = rax;
            // c5393 mov rdi,r15
            rdi = r15;
            // c5396 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c539b mov rdi,rbx
            rdi = rbx;
            // c539e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
