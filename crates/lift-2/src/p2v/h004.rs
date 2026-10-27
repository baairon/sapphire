//! Mechanically lifted functions (scripts/lift_run.py), module h004. Each one is checked against the original
//! in the world harness (tests/world_h004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x44fb6 size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044fb6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44fb6;
    loop {
        match pc {
        0x44fb6 => {
            // block 44fb6
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 44fb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44fb6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44fb7 mov rbp,rsp
            rbp = rsp;
            // 44fba push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44fbc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44fbe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44fbf sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 44fc3 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 44fc6 mov r14,rdx
            r14 = rdx;
            // 44fc9 mov r8d,esi
            r8 = ((rsi as u32 as u64)) as u32 as u64;
            // 44fcc mov rbx,rdi
            rbx = rdi;
            // 44fcf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 44fd1 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 44fd4 je 0x45027
            if fl.e() { pc = 0x45027; continue; }
            pc = 0x44fd6;
        }
        0x44fd6 => {
            // block 44fd6
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 44fd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44fd6 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 44fd9 je 0x45027
            if fl.e() { pc = 0x45027; continue; }
            pc = 0x44fdb;
        }
        0x44fdb => {
            // block 44fdb
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 44fdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44fdb mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 44fe2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44fe5 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 44fe9 mov rdi,rbx
            rdi = rbx;
            // 44fec mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 44fee mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // 44ff1 call QWORD PTR [rax+0x720]
            { let r = { let t = m.r64(rax.wrapping_add(0x720)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44ff7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44ff9 je 0x45025
            if fl.e() { pc = 0x45025; continue; }
            pc = 0x44ffb;
        }
        0x44ffb => {
            // block 44ffb
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 44ffb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ffb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 44ffe mov r9d,DWORD PTR [rbp-0x1c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 45002 mov DWORD PTR [rsp],r15d
            m.w32(rsp, ((r15 as u32 as u64)) as u32);
            // 45006 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 4500e mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 45013 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 45018 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4501a mov rdi,rbx
            rdi = rbx;
            // 4501d mov r8,r14
            r8 = r14;
            // 45020 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 45023 jmp 0x45027
            pc = 0x45027; continue;
        }
        0x45025 => {
            // block 45025
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 45025 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45025 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x45027;
        }
        0x45027 => {
            // block 45027
            #[cfg(feature = "trace")] eprintln!("f_00044fb6 pc 45027 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 45027 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 4502b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4502c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4502e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 45030 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 45031 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58160 size=122
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058160<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x58160;
    loop {
        match pc {
        0x58160 => {
            // block 58160
            #[cfg(feature = "trace")] eprintln!("f_00058160 pc 58160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58160 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58161 mov rbp,rsp
            rbp = rsp;
            // 58164 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58166 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58168 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58169 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5816a mov rax,QWORD PTR [rip+0xe823f] # 0x1403b0
            rax = m.r64(m.env.data_va(347005)) as u64;
            // 58171 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 58175 mov QWORD PTR [rdi-0x10],rcx
            m.w64(rdi.wrapping_add(0xffff_ffff_ffff_fff0), (rcx) as u64);
            // 58179 lea rcx,[rax+0x68]
            rcx = rax.wrapping_add(0x68);
            // 5817d mov QWORD PTR [rdi+0x70],rcx
            m.w64(rdi.wrapping_add(0x70), (rcx) as u64);
            // 58181 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 58185 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 58188 mov rax,QWORD PTR [rip+0xe8219] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 5818f add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 58193 mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            // 58197 lea r15,[rdi-0x10]
            r15 = rdi.wrapping_add(0xffff_ffff_ffff_fff0);
            // 5819b lea r14,[rdi+0x70]
            r14 = rdi.wrapping_add(0x70);
            // 5819f lea rbx,[rdi+0x8]
            rbx = rdi.wrapping_add(0x8);
            // 581a3 lea rdi,[rdi+0x48]
            rdi = rdi.wrapping_add(0x48);
            // 581a7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 581ac mov rdi,rbx
            rdi = rbx;
            // 581af call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 581b4 mov rsi,QWORD PTR [rip+0xe803d] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 581bb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 581bf mov rdi,r15
            rdi = r15;
            // 581c2 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 581c7 mov rdi,r14
            rdi = r14;
            // 581ca add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 581ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 581cf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 581d1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 581d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 581d4 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x582ea size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000582ea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x582ea;
    loop {
        match pc {
        0x582ea => {
            // block 582ea
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 582ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 582ea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 582eb mov rbp,rsp
            rbp = rsp;
            // 582ee mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 582f2 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 582f6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 582f9 je 0x5832d
            if fl.e() { pc = 0x5832d; continue; }
            pc = 0x582fb;
        }
        0x582fb => {
            // block 582fb
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 582fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 582fb mov ecx,DWORD PTR [rsi]
            rcx = (m.r32(rsi) as u64) as u32 as u64;
            // 582fd mov rsi,rdi
            rsi = rdi;
            pc = 0x58300;
        }
        0x58300 => {
            // block 58300
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58300 mov rax,rdx
            rax = rdx;
            pc = 0x58303;
        }
        0x58303 => {
            // block 58303
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58303 cmp DWORD PTR [rax+0x1c],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rcx as u32 as u64)).1;
            // 58306 jge 0x58313
            if fl.ge() { pc = 0x58313; continue; }
            pc = 0x58308;
        }
        0x58308 => {
            // block 58308
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58308 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58308 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 5830c test rax,rax
            fl = logic::<64>(rax & rax);
            // 5830f jne 0x58303
            if fl.ne() { pc = 0x58303; continue; }
            pc = 0x58311;
        }
        0x58311 => {
            // block 58311
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58311 jmp 0x58320
            pc = 0x58320; continue;
        }
        0x58313 => {
            // block 58313
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58313 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58313 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 58316 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 58319 mov rsi,rax
            rsi = rax;
            // 5831c jne 0x58300
            if fl.ne() { pc = 0x58300; continue; }
            pc = 0x5831e;
        }
        0x5831e => {
            // block 5831e
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 5831e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5831e jmp 0x58323
            pc = 0x58323; continue;
        }
        0x58320 => {
            // block 58320
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58320 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58320 mov rax,rsi
            rax = rsi;
            pc = 0x58323;
        }
        0x58323 => {
            // block 58323
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58323 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 58326 je 0x5832d
            if fl.e() { pc = 0x5832d; continue; }
            pc = 0x58328;
        }
        0x58328 => {
            // block 58328
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58328 cmp ecx,DWORD PTR [rax+0x1c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x1c)) as u64).1;
            // 5832b jge 0x58330
            if fl.ge() { pc = 0x58330; continue; }
            pc = 0x5832d;
        }
        0x5832d => {
            // block 5832d
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 5832d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5832d mov rax,rdi
            rax = rdi;
            pc = 0x58330;
        }
        0x58330 => {
            // block 58330
            #[cfg(feature = "trace")] eprintln!("f_000582ea pc 58330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58330 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58331 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1582c size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1582c;
    loop {
        match pc {
        0x1582c => {
            // block 1582c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1582d mov rbp,rsp
            rbp = rsp;
            // 15830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15839 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 1583d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 15841 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15844 je 0x15980
            if fl.e() { pc = 0x15980; continue; }
            pc = 0x1584a;
        }
        0x1584a => {
            // block 1584a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1584a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1584a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1584e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 15851 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 15854 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15858 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1585b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1585f mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 15863 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 15867 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 1586b mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1586f lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 15873 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15877 jmp 0x1587c
            pc = 0x1587c; continue;
        }
        0x15879 => {
            // block 15879
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15879 mov rbx,rax
            rbx = rax;
            pc = 0x1587c;
        }
        0x1587c => {
            // block 1587c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1587c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1587c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 1587f mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15883 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15888 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 1588c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1588f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15893 jne 0x1589d
            if fl.ne() { pc = 0x1589d; continue; }
            pc = 0x15895;
        }
        0x15895 => {
            // block 15895
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15895 mov r14,rax
            r14 = rax;
            // 15898 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 1589b jmp 0x158a1
            pc = 0x158a1; continue;
        }
        0x1589d => {
            // block 1589d
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1589d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1589d mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x158a1;
        }
        0x158a1 => {
            // block 158a1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158a1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 158a5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 158a8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 158ac cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 158b1 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 158b4 jne 0x158c0
            if fl.ne() { pc = 0x158c0; continue; }
            pc = 0x158b6;
        }
        0x158b6 => {
            // block 158b6
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158b6 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 158ba mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158be jmp 0x158c8
            pc = 0x158c8; continue;
        }
        0x158c0 => {
            // block 158c0
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158c4 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x158c8;
        }
        0x158c8 => {
            // block 158c8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c8 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 158cb mov rdx,r13
            rdx = r13;
            // 158ce cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 158d2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 158d5 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158d7;
        }
        0x158d7 => {
            // block 158d7
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158d7 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 158db call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 158e0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 158e2 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 158e6 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158e8;
        }
        0x158e8 => {
            // block 158e8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158e8 jns 0x158fe
            if fl.ns() { pc = 0x158fe; continue; }
            pc = 0x158ea;
        }
        0x158ea => {
            // block 158ea
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ea jmp 0x158f1
            pc = 0x158f1; continue;
        }
        0x158ec => {
            // block 158ec
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ec cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 158ef jae 0x158fe
            if fl.ae() { pc = 0x158fe; continue; }
            pc = 0x158f1;
        }
        0x158f1 => {
            // block 158f1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 158f4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 158f7 jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x158f9;
        }
        0x158f9 => {
            // block 158f9
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f9 jmp 0x15995
            pc = 0x15995; continue;
        }
        0x158fe => {
            // block 158fe
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158fe test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15901 jne 0x1590b
            if fl.ne() { pc = 0x1590b; continue; }
            pc = 0x15903;
        }
        0x15903 => {
            // block 15903
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15903 mov r14,rbx
            r14 = rbx;
            // 15906 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 15909 jmp 0x15912
            pc = 0x15912; continue;
        }
        0x1590b => {
            // block 1590b
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1590b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1590f mov r14,rbx
            r14 = rbx;
            pc = 0x15912;
        }
        0x15912 => {
            // block 15912
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15912 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15912 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 15916 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15919 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1591d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15922 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15925 jne 0x1592c
            if fl.ne() { pc = 0x1592c; continue; }
            pc = 0x15927;
        }
        0x15927 => {
            // block 15927
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15927 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 1592a jmp 0x15930
            pc = 0x15930; continue;
        }
        0x1592c => {
            // block 1592c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1592c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x15930;
        }
        0x15930 => {
            // block 15930
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15930 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 15933 mov rdx,rax
            rdx = rax;
            // 15936 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 1593a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 1593d je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1593f;
        }
        0x1593f => {
            // block 1593f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1593f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1593f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15942 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 15946 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 1594b mov r12,rax
            r12 = rax;
            // 1594e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15955 mov rax,r12
            rax = r12;
            // 15958 je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1595a;
        }
        0x1595a => {
            // block 1595a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595a js 0x15963
            if fl.s() { pc = 0x15963; continue; }
            pc = 0x1595c;
        }
        0x1595c => {
            // block 1595c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595c jmp 0x1598c
            pc = 0x1598c; continue;
        }
        0x1595e => {
            // block 1595e
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595e cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 15961 jae 0x1598c
            if fl.ae() { pc = 0x1598c; continue; }
            pc = 0x15963;
        }
        0x15963 => {
            // block 15963
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15963 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 15967 test rax,rax
            fl = logic::<64>(rax & rax);
            // 1596a jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x15970;
        }
        0x15970 => {
            // block 15970
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15970 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15974 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15977 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 1597b mov rax,r14
            rax = r14;
            // 1597e jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15980 => {
            // block 15980
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15980 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 15984 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 15987 mov rax,rdi
            rax = rdi;
            // 1598a jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x1598c => {
            // block 1598c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1598c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1598c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15990 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15993 jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15995 => {
            // block 15995
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15995 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15999 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 1599c mov rax,rbx
            rax = rbx;
            pc = 0x1599f;
        }
        0x1599f => {
            // block 1599f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1599f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1599f add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 159a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 159a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 159a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 159aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 159ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51e2c size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051e2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x51e2c;
    loop {
        match pc {
        0x51e2c => {
            // block 51e2c
            #[cfg(feature = "trace")] eprintln!("f_00051e2c pc 51e2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51e2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51e2d mov rbp,rsp
            rbp = rsp;
            // 51e30 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51e32 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51e35 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51e39 mov esi,0x283e
            rsi = (0x283eu64) as u32 as u64;
            // 51e3e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51e43 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51e46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51e47 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x110fc size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000110fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x110fc;
    loop {
        match pc {
        0x110fc => {
            // block 110fc
            #[cfg(feature = "trace")] eprintln!("f_000110fc pc 110fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 110fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 110fd mov rbp,rsp
            rbp = rsp;
            // 11100 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11102 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11103 mov rbx,rsi
            rbx = rsi;
            // 11106 mov r14,rdi
            r14 = rdi;
            // 11109 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 1110c je 0x11131
            if fl.e() { pc = 0x11131; continue; }
            pc = 0x1110e;
        }
        0x1110e => {
            // block 1110e
            #[cfg(feature = "trace")] eprintln!("f_000110fc pc 1110e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1110e mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 11111 mov rdi,r14
            rdi = r14;
            // 11114 call 0x110fc
            { let r = { let t = m.env.code_va(0x110fc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11119 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1111d mov rdi,r14
            rdi = r14;
            // 11120 call 0x110fc
            { let r = { let t = m.env.code_va(0x110fc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11125 mov rdi,rbx
            rdi = rbx;
            // 11128 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11129 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1112b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1112c jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11131 => {
            // block 11131
            #[cfg(feature = "trace")] eprintln!("f_000110fc pc 11131 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11131 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11132 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11134 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11135 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x52212 size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00052212<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x52212;
    loop {
        match pc {
        0x52212 => {
            // block 52212
            #[cfg(feature = "trace")] eprintln!("f_00052212 pc 52212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52212 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 52213 mov rbp,rsp
            rbp = rsp;
            // 52216 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 52219 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 5221d movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // 52221 mov esi,0x2841
            rsi = (0x2841u64) as u32 as u64;
            // 52226 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 5222b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5222e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5222f jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x439ac size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000439ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x439ac;
    loop {
        match pc {
        0x439ac => {
            // block 439ac
            #[cfg(feature = "trace")] eprintln!("f_000439ac pc 439ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 439ad mov rbp,rsp
            rbp = rsp;
            // 439b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 439b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 439b2 mov rbx,rdx
            rbx = rdx;
            // 439b5 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 439b8 jle 0x439be
            if fl.le() { pc = 0x439be; continue; }
            pc = 0x439ba;
        }
        0x439ba => {
            // block 439ba
            #[cfg(feature = "trace")] eprintln!("f_000439ac pc 439ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439ba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 439bc jmp 0x439ed
            pc = 0x439ed; continue;
        }
        0x439be => {
            // block 439be
            #[cfg(feature = "trace")] eprintln!("f_000439ac pc 439be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439be mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 439c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 439c8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 439ca lea rdx,[rip+0xbc9ef] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 439d1 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 439d4 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 439d8 mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 439dd mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 439e3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 439e6 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 439ea setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x439ed;
        }
        0x439ed => {
            // block 439ed
            #[cfg(feature = "trace")] eprintln!("f_000439ac pc 439ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439ed add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 439f1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 439f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 439f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2902e size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002902e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2902e;
    loop {
        match pc {
        0x2902e => {
            // block 2902e
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 2902e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2902e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2902f mov rbp,rsp
            rbp = rsp;
            // 29032 mov rax,rsi
            rax = rsi;
            // 29035 mov r9,rdi
            r9 = rdi;
            // 29038 mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 2903b test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2903e je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29040;
        }
        0x29040 => {
            // block 29040
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29040 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29040 mov r10d,DWORD PTR [rax+0x8]
            r10 = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29044 test r10d,r10d
            fl = logic::<32>((r10 as u32 as u64) & (r10 as u32 as u64));
            // 29047 je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29049;
        }
        0x29049 => {
            // block 29049
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29049 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29049 mov rdi,QWORD PTR [r9]
            rdi = m.r64(r9) as u64;
            // 2904c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2904f je 0x29084
            if fl.e() { pc = 0x29084; continue; }
            pc = 0x29051;
        }
        0x29051 => {
            // block 29051
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29051 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29051 mov r9d,DWORD PTR [r9+0x8]
            r9 = (m.r32(r9.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29055 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 29057 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // 2905a je 0x29086
            if fl.e() { pc = 0x29086; continue; }
            pc = 0x2905c;
        }
        0x2905c => {
            // block 2905c
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 2905c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2905c lea r11d,[r8+rdx*1]
            r11 = (r8.wrapping_add(rdx)) as u32 as u64;
            // 29060 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // 29063 ja 0x29086
            if fl.a() { pc = 0x29086; continue; }
            pc = 0x29065;
        }
        0x29065 => {
            // block 29065
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29065 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29065 lea r10d,[r8+rcx*1]
            r10 = (r8.wrapping_add(rcx)) as u32 as u64;
            // 29069 cmp r10d,r9d
            fl = sub::<32>((r10 as u32 as u64), (r9 as u32 as u64)).1;
            // 2906c ja 0x29086
            if fl.a() { pc = 0x29086; continue; }
            pc = 0x2906e;
        }
        0x2906e => {
            // block 2906e
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 2906e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2906e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 29070 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 29073 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 29075 add rdi,rax
            { let (r, f) = add::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 29078 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // 2907b call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29080 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 29082 jmp 0x29086
            pc = 0x29086; continue;
        }
        0x29084 => {
            // block 29084
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29084 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x29086;
        }
        0x29086 => {
            // block 29086
            #[cfg(feature = "trace")] eprintln!("f_0002902e pc 29086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29086 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29087 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e114 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e114<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e114;
    loop {
        match pc {
        0x2e114 => {
            // block 2e114
            #[cfg(feature = "trace")] eprintln!("f_0002e114 pc 2e114 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e114 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e115 mov rbp,rsp
            rbp = rsp;
            // 2e118 mov edi,0x50
            rdi = (0x50u64) as u32 as u64;
            // 2e11d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e122 lea rcx,[rip+0x1133c7] # 0x1414f0
            rcx = m.env.data_va(351421);
            // 2e129 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2e12c mov WORD PTR [rax+0x8],0x0
            m.w16(rax.wrapping_add(0x8), (0x0u64) as u16);
            // 2e132 mov QWORD PTR [rax+0x48],0x0
            m.w64(rax.wrapping_add(0x48), (0x0u64) as u64);
            // 2e13a mov QWORD PTR [rax+0x40],0x0
            m.w64(rax.wrapping_add(0x40), (0x0u64) as u64);
            // 2e142 mov QWORD PTR [rax+0x38],0x0
            m.w64(rax.wrapping_add(0x38), (0x0u64) as u64);
            // 2e14a mov QWORD PTR [rax+0x30],0x0
            m.w64(rax.wrapping_add(0x30), (0x0u64) as u64);
            // 2e152 mov QWORD PTR [rax+0x28],0x0
            m.w64(rax.wrapping_add(0x28), (0x0u64) as u64);
            // 2e15a mov QWORD PTR [rax+0x20],0x0
            m.w64(rax.wrapping_add(0x20), (0x0u64) as u64);
            // 2e162 mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 2e16a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2e172 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e173 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x265e size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000265e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x265e;
    loop {
        match pc {
        0x265e => {
            // block 265e
            #[cfg(feature = "trace")] eprintln!("f_0000265e pc 265e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 265e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 265f mov rbp,rsp
            rbp = rsp;
            // 2662 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2664 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2666 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2667 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2668 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 266b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 266e mov rbx,rdi
            rbx = rdi;
            // 2671 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2674 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 267a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 267c jne 0x26d0
            if fl.ne() { pc = 0x26d0; continue; }
            pc = 0x267e;
        }
        0x267e => {
            // block 267e
            #[cfg(feature = "trace")] eprintln!("f_0000265e pc 267e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 267e mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2681 and edx,0x1
            { let r = (rdx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2684 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2687 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 268a lea rsi,[rip+0xfc9ef] # 0xff080
            rsi = m.env.data_va(79949);
            // 2691 mov r15d,DWORD PTR [rsi+rcx*4]
            r15 = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2695 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 269a mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 26a0 mov rdi,rbx
            rdi = rbx;
            // 26a3 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 26a6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26a9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 26ab je 0x26d0
            if fl.e() { pc = 0x26d0; continue; }
            pc = 0x26ad;
        }
        0x26ad => {
            // block 26ad
            #[cfg(feature = "trace")] eprintln!("f_0000265e pc 26ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26ad shr r14d,1
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x1u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 26b0 and r14d,0x3
            { let r = (r14 as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 26b4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 26b7 mov ecx,0x6000000
            rcx = (0x6000000u64) as u32 as u64;
            // 26bc mov r8d,0x19
            r8 = (0x19u64) as u32 as u64;
            // 26c2 mov rdi,rbx
            rdi = rbx;
            // 26c5 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 26c8 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 26cb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26ce jmp 0x26d2
            pc = 0x26d2; continue;
        }
        0x26d0 => {
            // block 26d0
            #[cfg(feature = "trace")] eprintln!("f_0000265e pc 26d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x26d2;
        }
        0x26d2 => {
            // block 26d2
            #[cfg(feature = "trace")] eprintln!("f_0000265e pc 26d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26d2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 26d6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 26d7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 26d9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 26db pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 26dc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c6 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000005c6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c6;
    loop {
        match pc {
        0x5c6 => {
            // block 5c6
            #[cfg(feature = "trace")] eprintln!("f_000005c6 pc 5c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c7 mov rbp,rsp
            rbp = rsp;
            // 5ca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5cc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5ce push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5d0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d1 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 5d5 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5d8 mov r14,rsi
            r14 = rsi;
            // 5db mov rbx,rdi
            rbx = rdi;
            // 5de mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5e1 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5e7 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 5ea test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5ec cmovne r15d,r12d
            if fl.ne() { r15 = ((r12 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 5f0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5f3 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5f7 mov rdi,rbx
            rdi = rbx;
            // 5fa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 5fd call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 603 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 606 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 60a mov rdi,rbx
            rdi = rbx;
            // 60d mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 610 call QWORD PTR [rax+0x380]
            { let r = { let t = m.r64(rax.wrapping_add(0x380)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 616 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 619 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 61d mov rdi,rbx
            rdi = rbx;
            // 620 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 623 call QWORD PTR [rax+0x428]
            { let r = { let t = m.r64(rax.wrapping_add(0x428)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 629 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 62c lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 630 mov rdi,rbx
            rdi = rbx;
            // 633 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 636 call QWORD PTR [rax+0x440]
            { let r = { let t = m.r64(rax.wrapping_add(0x440)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 63f lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // 643 mov rdi,rbx
            rdi = rbx;
            // 646 call QWORD PTR [rax+0x458]
            { let r = { let t = m.r64(rax.wrapping_add(0x458)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 64c mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 64f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 652 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 655 mov r8d,DWORD PTR [rbp-0x30]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 659 cmp DWORD PTR [rbp-0x34],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64, 0x0u64).1;
            // 65d setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 660 movzx r9d,al
            r9 = ((rax as u8 as u64)) as u32 as u64;
            // 664 mov rdi,r14
            rdi = r14;
            // 667 call 0xd2260
            { let r = { let t = m.env.code_va(0xd2260); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 66c add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 670 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 671 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 673 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 675 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 677 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 678 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 10128 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 10142 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 1014b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1014b jbe 0x1016b
            if fl.be() { pc = 0x1016b; continue; }
            pc = 0x1014d;
        }
        0x1014d => {
            // block 1014d
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 1014d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 10156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00010128 pc 1016b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1016b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1016c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2a48 size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002a48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2a48;
    loop {
        match pc {
        0x2a48 => {
            // block 2a48
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2a49 mov rbp,rsp
            rbp = rsp;
            // 2a4c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2a4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2a4f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2a53 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2a56 mov rbx,rdi
            rbx = rdi;
            // 2a59 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a5c call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a62 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a64 jne 0x2ac9
            if fl.ne() { pc = 0x2ac9; continue; }
            pc = 0x2a66;
        }
        0x2a66 => {
            // block 2a66
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2a66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a66 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a69 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 2a6d mov rdi,rbx
            rdi = rbx;
            // 2a70 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2a73 call QWORD PTR [rax+0x530]
            { let r = { let t = m.r64(rax.wrapping_add(0x530)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a7b je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2a7d;
        }
        0x2a7d => {
            // block 2a7d
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a7d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a80 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 2a84 mov rdi,rbx
            rdi = rbx;
            // 2a87 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2a8a call QWORD PTR [rax+0x548]
            { let r = { let t = m.r64(rax.wrapping_add(0x548)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2a90 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2a92 je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2a94;
        }
        0x2a94 => {
            // block 2a94
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2a94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2a97 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 2a9a mov rdi,rbx
            rdi = rbx;
            // 2a9d mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2aa0 call QWORD PTR [rax+0x540]
            { let r = { let t = m.r64(rax.wrapping_add(0x540)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2aa6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2aa8 je 0x2ac9
            if fl.e() { pc = 0x2ac9; continue; }
            pc = 0x2aaa;
        }
        0x2aaa => {
            // block 2aaa
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2aaa mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2aad mov edx,DWORD PTR [rbp-0x18]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 2ab0 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2ab5 mov rdi,rbx
            rdi = rbx;
            // 2ab8 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 2abb call QWORD PTR [rax+0x528]
            { let r = { let t = m.r64(rax.wrapping_add(0x528)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ac1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2ac3 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2ac5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 2ac7 jne 0x2acb
            if fl.ne() { pc = 0x2acb; continue; }
            pc = 0x2ac9;
        }
        0x2ac9 => {
            // block 2ac9
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2ac9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ac9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2acb;
        }
        0x2acb => {
            // block 2acb
            #[cfg(feature = "trace")] eprintln!("f_00002a48 pc 2acb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2acb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2acf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ad0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ad2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ad3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e76e size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e76e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e76e;
    loop {
        match pc {
        0x2e76e => {
            // block 2e76e
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e76e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e76e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e76f mov rbp,rsp
            rbp = rsp;
            // 2e772 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e774 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e776 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e777 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2e778 mov r14,rsi
            r14 = rsi;
            // 2e77b mov r15,rdi
            r15 = rdi;
            // 2e77e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e781 call QWORD PTR [rax+0x70]
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e784 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e786 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2e789 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e78b;
        }
        0x2e78b => {
            // block 2e78b
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e78b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e78b mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e78f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e791 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e795 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e797;
        }
        0x2e797 => {
            // block 2e797
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e797 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 2e79a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2e79d call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7a0 mov rdi,r14
            rdi = r14;
            // 2e7a3 mov rsi,rax
            rsi = rax;
            // 2e7a6 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7ab test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e7ad je 0x2e7e0
            if fl.e() { pc = 0x2e7e0; continue; }
            pc = 0x2e7af;
        }
        0x2e7af => {
            // block 2e7af
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7af add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e7b3 cmp rbx,QWORD PTR [r15+0x18]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x18)) as u64).1;
            // 2e7b7 jne 0x2e797
            if fl.ne() { pc = 0x2e797; continue; }
            pc = 0x2e7b9;
        }
        0x2e7b9 => {
            // block 2e7b9
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7b9 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 2e7bd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7bf cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2e7c2 je 0x2e7e3
            if fl.e() { pc = 0x2e7e3; continue; }
            pc = 0x2e7c4;
        }
        0x2e7c4 => {
            // block 2e7c4
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7c4 lea rsi,[rip+0xfc3b5] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e7cb mov rdi,r14
            rdi = r14;
            // 2e7ce call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e7d3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2e7d5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e7d7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2e7d9 jne 0x2e7e3
            if fl.ne() { pc = 0x2e7e3; continue; }
            pc = 0x2e7db;
        }
        0x2e7db => {
            // block 2e7db
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7db mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e7de jmp 0x2e7e3
            pc = 0x2e7e3; continue;
        }
        0x2e7e0 => {
            // block 2e7e0
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2e7e3;
        }
        0x2e7e3 => {
            // block 2e7e3
            #[cfg(feature = "trace")] eprintln!("f_0002e76e pc 2e7e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e7e7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e7e8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e7ea pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e7ec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e7ed ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x159ae size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000159ae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x159ae;
    loop {
        match pc {
        0x159ae => {
            // block 159ae
            #[cfg(feature = "trace")] eprintln!("f_000159ae pc 159ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 159ae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 159af mov rbp,rsp
            rbp = rsp;
            // 159b2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 159b3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 159b4 mov rbx,rdi
            rbx = rdi;
            // 159b7 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 159bf mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 159c6 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 159ca mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 159cd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 159d0 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 159d4 je 0x159df
            if fl.e() { pc = 0x159df; continue; }
            pc = 0x159d6;
        }
        0x159d6 => {
            // block 159d6
            #[cfg(feature = "trace")] eprintln!("f_000159ae pc 159d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 159d6 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 159d9 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 159dc mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x159df;
        }
        0x159df => {
            // block 159df
            #[cfg(feature = "trace")] eprintln!("f_000159ae pc 159df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 159df mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 159e3 mov rsi,rcx
            rsi = rcx;
            // 159e6 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 159eb inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 159ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 159f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd192 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d192<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd192;
    loop {
        match pc {
        0xd192 => {
            // block d192
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d192 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d193 mov rbp,rsp
            rbp = rsp;
            // d196 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d198 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d19a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d19b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d19c mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d19f mov r15,rdi
            r15 = rdi;
            // d1a2 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d1a5 je 0xd216
            if fl.e() { pc = 0xd216; continue; }
            pc = 0xd1a7;
        }
        0xd1a7 => {
            // block d1a7
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1a7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1aa mov rdi,r15
            rdi = r15;
            // d1ad call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1b3 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d1b8 je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1ba;
        }
        0xd1ba => {
            // block d1ba
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ba mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1bd mov rdi,r15
            rdi = r15;
            // d1c0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1c6 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d1cb je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1cd;
        }
        0xd1cd => {
            // block d1cd
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1d0 mov rdi,r15
            rdi = r15;
            // d1d3 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1d9 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d1de je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1e0;
        }
        0xd1e0 => {
            // block d1e0
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1e3 mov rdi,r15
            rdi = r15;
            // d1e6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ec cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d1f1 je 0xd206
            if fl.e() { pc = 0xd206; continue; }
            pc = 0xd1f3;
        }
        0xd1f3 => {
            // block d1f3
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1f3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1f6 mov rdi,r15
            rdi = r15;
            // d1f9 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d1ff cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d204 jne 0xd216
            if fl.ne() { pc = 0xd216; continue; }
            pc = 0xd206;
        }
        0xd206 => {
            // block d206
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d206 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d209 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d20d mov rdi,r15
            rdi = r15;
            // d210 call QWORD PTR [rax+0x328]
            { let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd216;
        }
        0xd216 => {
            // block d216
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d216 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d216 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d219 mov rdi,r15
            rdi = r15;
            // d21c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d222 cmp eax,0x10402100
            fl = sub::<32>((rax as u32 as u64), 0x10402100u64).1;
            // d227 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d22a xor al,r14b
            { let r = (rax as u8 as u64) ^ (r14 as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d22d movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // d230 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d233 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d238 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // d23d mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // d243 mov rdi,r15
            rdi = r15;
            // d246 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d248 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d24b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d24e xor ebx,0x1
            { let r = (rbx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d251 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d256 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // d25b mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // d261 mov rdi,r15
            rdi = r15;
            // d264 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // d266 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d269 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d26d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d26e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d270 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d272 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d273 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xec66 size=230
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ec66<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xec66;
    loop {
        match pc {
        0xec66 => {
            // block ec66
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ec66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ec66 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ec67 mov rbp,rsp
            rbp = rsp;
            // ec6a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ec6c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ec6e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ec6f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ec70 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // ec73 mov r14,rsi
            r14 = rsi;
            // ec76 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // ec7d mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // ec84 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ec87 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // ec8b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ec8d mov esi,0x77
            rsi = (0x77u64) as u32 as u64;
            // ec92 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ec97 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ec9a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ec9d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ec9f je 0xed2c
            if fl.e() { pc = 0xed2c; continue; }
            pc = 0xeca5;
        }
        0xeca5 => {
            // block eca5
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc eca5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eca5 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // eca8 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // ecac jbe 0xecb2
            if fl.be() { pc = 0xecb2; continue; }
            pc = 0xecae;
        }
        0xecae => {
            // block ecae
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ecae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ecae xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ecb0 jmp 0xed2c
            pc = 0xed2c; continue;
        }
        0xecb2 => {
            // block ecb2
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ecb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ecb2 mov ecx,0xffc0
            rcx = (0xffc0u64) as u32 as u64;
            // ecb7 and ecx,DWORD PTR [rbp-0x1c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // ecba shr ecx,0x6
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x6u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // ecbd cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // ecc1 mulsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8e0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(77997)) as u128))).to_bits());
            // ecc9 mulsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8e8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78005)) as u128))).to_bits());
            // ecd1 addsd xmm0,QWORD PTR [rip+0xefc17] # 0xfe8f0
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78013)) as u128))).to_bits());
            // ecd9 lea rcx,[rip+0x5c] # 0xed3c
            rcx = m.env.code_va(0xed3c);
            // ece0 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // ece4 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // ece7 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xece9 => {
            // block ece9
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ece9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ece9 mulsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe900
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78029)) as u128))).to_bits());
            // ecf1 divsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe908
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(78037)) as u128))).to_bits());
            // ecf9 addsd xmm0,QWORD PTR [rip+0xefc0f] # 0xfe910
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78045)) as u128))).to_bits());
            // ed01 jmp 0xed25
            pc = 0xed25; continue;
        }
        0xed03 => {
            // block ed03
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ed03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed03 addsd xmm0,QWORD PTR [rip+0xefbed] # 0xfe8f8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78021)) as u128))).to_bits());
            // ed0b jmp 0xed25
            pc = 0xed25; continue;
        }
        0xed0d => {
            // block ed0d
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ed0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed0d addsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe8f8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(78021)) as u128))).to_bits());
            // ed15 mulsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe900
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(78029)) as u128))).to_bits());
            // ed1d divsd xmm0,QWORD PTR [rip+0xefbe3] # 0xfe908
            x0 = set_lo64(x0, (f64_of(x0) / f64_of((m.r64(m.env.data_va(78037)) as u128))).to_bits());
            pc = 0xed25;
        }
        0xed25 => {
            // block ed25
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ed25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed25 movsd QWORD PTR [r14],xmm0
            m.w64(r14, x0 as u64);
            // ed2a mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xed2c;
        }
        0xed2c => {
            // block ed2c
            #[cfg(feature = "trace")] eprintln!("f_0000ec66 pc ed2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed2c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // ed2e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ed32 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ed33 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ed35 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ed37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ed38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18ca1 size=682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00018ca1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18ca1;
    loop {
        match pc {
        0x18ca1 => {
            // block 18ca1
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18ca1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ca1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18ca2 mov rbp,rsp
            rbp = rsp;
            // 18ca5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18ca7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18ca9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18cab push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18cac sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18cb0 mov r15,rsi
            r15 = rsi;
            // 18cb3 mov r14,rdi
            r14 = rdi;
            // 18cb6 lea rsi,[rip+0x1116e5] # 0x12a3a2
            rsi = m.env.data_va(256879);
            // 18cbd mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 18cc2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cc7 mov rbx,rax
            rbx = rax;
            // 18cca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18ccd mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18cd1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18cd4 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18cd8 mov rdi,r12
            rdi = r12;
            // 18cdb call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ce0 mov rsi,QWORD PTR [rip+0x1274a9] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18ce7 mov rdi,r12
            rdi = r12;
            // 18cea call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18cf2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18cf6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18cfb mov rdi,rax
            rdi = rax;
            // 18cfe call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d00 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d03 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18d07 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d0c movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d10 mov rdi,rbx
            rdi = rbx;
            // 18d13 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d18 mov rdi,rbx
            rdi = rbx;
            // 18d1b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d20 lea rsi,[rip+0x111690] # 0x12a3b7
            rsi = m.env.data_va(256900);
            // 18d27 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18d2c mov rdi,rbx
            rdi = rbx;
            // 18d2f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d34 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 18d37 mov rdi,rax
            rdi = rax;
            // 18d3a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d3f mov rbx,rax
            rbx = rax;
            // 18d42 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18d45 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18d49 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18d4c lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d50 mov rdi,r12
            rdi = r12;
            // 18d53 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d58 mov rsi,QWORD PTR [rip+0x127431] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18d5f mov rdi,r12
            rdi = r12;
            // 18d62 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d67 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18d6a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18d6e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18d73 mov rdi,rax
            rdi = rax;
            // 18d76 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d78 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d7b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d7f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d84 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d88 mov rdi,rbx
            rdi = rbx;
            // 18d8b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d90 mov rdi,rbx
            rdi = rbx;
            // 18d93 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d98 lea rsi,[rip+0x11162a] # 0x12a3c9
            rsi = m.env.data_va(256918);
            // 18d9f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18da4 mov rdi,rbx
            rdi = rbx;
            // 18da7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dac mov esi,DWORD PTR [r15+0x4]
            rsi = (m.r32(r15.wrapping_add(0x4)) as u64) as u32 as u64;
            // 18db0 mov rdi,rax
            rdi = rax;
            // 18db3 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18db8 mov rbx,rax
            rbx = rax;
            // 18dbb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18dbe mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18dc2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18dc5 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18dc9 mov rdi,r12
            rdi = r12;
            // 18dcc call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dd1 mov rsi,QWORD PTR [rip+0x1273b8] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18dd8 mov rdi,r12
            rdi = r12;
            // 18ddb call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18de0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18de3 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18de7 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18dec mov rdi,rax
            rdi = rax;
            // 18def call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18df1 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18df4 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18df8 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dfd movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e01 mov rdi,rbx
            rdi = rbx;
            // 18e04 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e09 mov rdi,rbx
            rdi = rbx;
            // 18e0c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e11 lea rsi,[rip+0x1115c3] # 0x12a3db
            rsi = m.env.data_va(256936);
            // 18e18 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e1d mov rdi,rbx
            rdi = rbx;
            // 18e20 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e25 mov esi,DWORD PTR [r15+0x8]
            rsi = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // 18e29 mov rdi,rax
            rdi = rax;
            // 18e2c call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e31 mov rbx,rax
            rbx = rax;
            // 18e34 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18e37 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18e3b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18e3e lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e42 mov rdi,r12
            rdi = r12;
            // 18e45 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e4a mov rsi,QWORD PTR [rip+0x12733f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18e51 mov rdi,r12
            rdi = r12;
            // 18e54 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e59 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18e5c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18e60 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18e65 mov rdi,rax
            rdi = rax;
            // 18e68 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e6a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18e6d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e71 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e76 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e7a mov rdi,rbx
            rdi = rbx;
            // 18e7d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e82 mov rdi,rbx
            rdi = rbx;
            // 18e85 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e8a lea rsi,[rip+0x11155c] # 0x12a3ed
            rsi = m.env.data_va(256954);
            // 18e91 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e96 mov rdi,rbx
            rdi = rbx;
            // 18e99 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e9e mov esi,DWORD PTR [r15+0xc]
            rsi = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 18ea2 mov rdi,rax
            rdi = rax;
            // 18ea5 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eaa mov rbx,rax
            rbx = rax;
            // 18ead mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18eb0 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18eb4 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18eb7 lea r15,[rbp-0x28]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18ebb mov rdi,r15
            rdi = r15;
            // 18ebe call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ec3 mov rsi,QWORD PTR [rip+0x1272c6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18eca mov rdi,r15
            rdi = r15;
            // 18ecd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ed2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18ed5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18ed9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18ede mov rdi,rax
            rdi = rax;
            // 18ee1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ee3 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18ee6 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18eea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eef movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 18ef3 mov rdi,rbx
            rdi = rbx;
            // 18ef6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18efb mov rdi,rbx
            rdi = rbx;
            // 18efe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f03 mov rax,r14
            rax = r14;
            // 18f06 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18f0a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18f0b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18f0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18f0f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18f11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18f12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18f13 => {
            // block 18f13
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f13 mov rbx,rax
            rbx = rax;
            // 18f16 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18f1a jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f1c => {
            // block 18f1c
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f1c mov rbx,rax
            rbx = rax;
            // 18f1f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18f23 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f25 => {
            // block 18f25
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f25 mov rbx,rax
            rbx = rax;
            // 18f28 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18f2c jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f2e => {
            // block 18f2e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f2e mov rbx,rax
            rbx = rax;
            // 18f31 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18f35 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f37 => {
            // block 18f37
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f37 mov rbx,rax
            rbx = rax;
            // 18f3a lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x18f3e;
        }
        0x18f3e => {
            // block 18f3e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f3e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f43 mov rdi,rbx
            rdi = rbx;
            // 18f46 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42c6c size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042c6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42c6c;
    loop {
        match pc {
        0x42c6c => {
            // block 42c6c
            #[cfg(feature = "trace")] eprintln!("f_00042c6c pc 42c6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42c6d mov rbp,rsp
            rbp = rsp;
            // 42c70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42c72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42c73 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42c77 mov r14,rsi
            r14 = rsi;
            // 42c7a mov rbx,rdi
            rbx = rdi;
            // 42c7d mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // 42c84 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c87 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42c8d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42c8f je 0x42cc3
            if fl.e() { pc = 0x42cc3; continue; }
            pc = 0x42c91;
        }
        0x42c91 => {
            // block 42c91
            #[cfg(feature = "trace")] eprintln!("f_00042c6c pc 42c91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42c91 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 42c98 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42c9b lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 42c9f mov esi,0x900
            rsi = (0x900u64) as u32 as u64;
            // 42ca4 mov ecx,0x70
            rcx = (0x70u64) as u32 as u64;
            // 42ca9 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 42caf mov rdi,rbx
            rdi = rbx;
            // 42cb2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42cb5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42cb7 je 0x42cc3
            if fl.e() { pc = 0x42cc3; continue; }
            pc = 0x42cb9;
        }
        0x42cb9 => {
            // block 42cb9
            #[cfg(feature = "trace")] eprintln!("f_00042c6c pc 42cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42cb9 mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 42cbc mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 42cbf mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 42cc1 jmp 0x42cc5
            pc = 0x42cc5; continue;
        }
        0x42cc3 => {
            // block 42cc3
            #[cfg(feature = "trace")] eprintln!("f_00042c6c pc 42cc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42cc3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x42cc5;
        }
        0x42cc5 => {
            // block 42cc5
            #[cfg(feature = "trace")] eprintln!("f_00042c6c pc 42cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42cc5 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42cc9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42cca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42ccc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42ccd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
