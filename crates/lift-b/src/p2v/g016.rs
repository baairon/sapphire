//! Mechanically lifted functions (scripts/lift_run.py), module g016. Each one is checked against the original
//! in the world harness (tests/world_g016.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe0562 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0562<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x1: u128 = fx[1] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe0562;
    loop {
        match pc {
        0xe0562 => {
            // block e0562
            // e0562 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0563 mov rbp,rsp
            rbp = rsp;
            // e0566 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e0568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e056a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e056c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e056d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0571 movsd QWORD PTR [rbp-0x28],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), x1 as u64);
            // e0576 movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // e057b mov r14d,r9d
            r14 = ((r9 as u32 as u64)) as u32 as u64;
            // e057e mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // e0581 mov r15,rcx
            r15 = rcx;
            // e0584 mov rbx,rdi
            rbx = rdi;
            // e0587 lea rax,[rip+0x65c3a] # 0x1461c8
            rax = m.env.data_va(371093);
            // e058e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e0591 mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // e0595 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e0598 lea rdi,[rip+0x4b649] # 0x12bbe8
            rdi = m.env.data_va(263093);
            // e059f cmovne rdi,rdx
            if fl.ne() { rdi = rdx; }
            // e05a3 call 0xec2ba
            { let r = { let t = m.env.stub_va(0xa87); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // e05a8 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e05ac mov DWORD PTR [rbx+0x18],r12d
            m.w32(rbx.wrapping_add(0x18), ((r12 as u32 as u64)) as u32);
            // e05b0 mov DWORD PTR [rbx+0x1c],r14d
            m.w32(rbx.wrapping_add(0x1c), ((r14 as u32 as u64)) as u32);
            // e05b4 movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // e05b9 movsd QWORD PTR [rbx+0x20],xmm0
            m.w64(rbx.wrapping_add(0x20), x0 as u64);
            // e05be movsd xmm0,QWORD PTR [rbp-0x28]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u128;
            // e05c3 movsd QWORD PTR [rbx+0x28],xmm0
            m.w64(rbx.wrapping_add(0x28), x0 as u64);
            // e05c8 mov QWORD PTR [rbx+0x30],r15
            m.w64(rbx.wrapping_add(0x30), (r15) as u64);
            // e05cc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e05d0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e05d1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e05d3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e05d5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e05d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e05d8 ret
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

/// parity: P2v @0x29e48 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029e48<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29e48;
    loop {
        match pc {
        0x29e48 => {
            // block 29e48
            // 29e48 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29e49 mov rbp,rsp
            rbp = rsp;
            // 29e4c mov r8d,DWORD PTR [rdi+0x28]
            r8 = (m.r32(rdi.wrapping_add(0x28)) as u64) as u32 as u64;
            // 29e50 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // 29e53 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 29e56 imul rcx,rax,0x24924925
            { let (r, f) = imul::<64>(rax, 0x24924925u64); rcx = r; fl = f; }
            // 29e5d shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // 29e61 mov rdi,QWORD PTR [rdi+0x20]
            rdi = m.r64(rdi.wrapping_add(0x20)) as u64;
            // 29e65 mov DWORD PTR [rsi+0x18],0x0
            m.w32(rsi.wrapping_add(0x18), (0x0u64) as u32);
            // 29e6c mov QWORD PTR [rsi+0x10],0x0
            m.w64(rsi.wrapping_add(0x10), (0x0u64) as u64);
            // 29e74 mov QWORD PTR [rsi+0x8],0x0
            m.w64(rsi.wrapping_add(0x8), (0x0u64) as u64);
            // 29e7c mov QWORD PTR [rsi],0x0
            m.w64(rsi, (0x0u64) as u64);
            // 29e83 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 29e85 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 29e87 jae 0x29ec4
            if fl.ae() { pc = 0x29ec4; continue; }
            pc = 0x29e89;
        }
        0x29e89 => {
            // block 29e89
            // 29e89 add r8d,0xffffff20
            { let (r, f) = add::<32>((r8 as u32 as u64), 0xffffff20u64);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 29e90 cmp r8d,0x1b
            fl = sub::<32>((r8 as u32 as u64), 0x1bu64).1;
            // 29e94 ja 0x29ec4
            if fl.a() { pc = 0x29ec4; continue; }
            pc = 0x29e96;
        }
        0x29e96 => {
            // block 29e96
            // 29e96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 29e99 je 0x29ec4
            if fl.e() { pc = 0x29ec4; continue; }
            pc = 0x29e9b;
        }
        0x29e9b => {
            // block 29e9b
            // 29e9b movzx eax,dx
            rax = ((rdx as u16 as u64)) as u32 as u64;
            // 29e9e imul rax,rax,0x1c
            { let (r, f) = imul::<64>(rax, 0x1cu64); rax = r; fl = f; }
            // 29ea2 mov ecx,DWORD PTR [rdi+rax*1+0x18]
            rcx = (m.r32(rdi.wrapping_add(rax).wrapping_add(0x18)) as u64) as u32 as u64;
            // 29ea6 mov DWORD PTR [rsi+0x18],ecx
            m.w32(rsi.wrapping_add(0x18), ((rcx as u32 as u64)) as u32);
            // 29ea9 mov rcx,QWORD PTR [rdi+rax*1+0x10]
            rcx = m.r64(rdi.wrapping_add(rax).wrapping_add(0x10)) as u64;
            // 29eae mov QWORD PTR [rsi+0x10],rcx
            m.w64(rsi.wrapping_add(0x10), (rcx) as u64);
            // 29eb2 mov rcx,QWORD PTR [rdi+rax*1]
            rcx = m.r64(rdi.wrapping_add(rax)) as u64;
            // 29eb6 mov rax,QWORD PTR [rdi+rax*1+0x8]
            rax = m.r64(rdi.wrapping_add(rax).wrapping_add(0x8)) as u64;
            // 29ebb mov QWORD PTR [rsi+0x8],rax
            m.w64(rsi.wrapping_add(0x8), (rax) as u64);
            // 29ebf mov QWORD PTR [rsi],rcx
            m.w64(rsi, (rcx) as u64);
            // 29ec2 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x29ec4;
        }
        0x29ec4 => {
            // block 29ec4
            // 29ec4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29ec5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd65bc size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d65bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd65bc;
    loop {
        match pc {
        0xd65bc => {
            // block d65bc
            // d65bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d65bd mov rbp,rsp
            rbp = rsp;
            // d65c0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d65c2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d65c4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d65c5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d65c6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d65c9 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // d65cd lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // d65d1 mov rcx,QWORD PTR [rip+0x69de0] # 0x1403b8
            rcx = m.r64(m.env.data_va(347013)) as u64;
            // d65d8 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // d65dc mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // d65e0 lea r14,[rdi+rax*1+0x78]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x78);
            // d65e5 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // d65e9 mov QWORD PTR [rdi+rax*1+0x78],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x78), (rcx) as u64);
            // d65ee lea rbx,[rdi+rax*1+0x10]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x10);
            // d65f3 mov rcx,QWORD PTR [rip+0x69dae] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // d65fa add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // d65fe mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // d6603 lea rdi,[rdi+rax*1+0x50]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x50);
            // d6608 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d660d mov rdi,rbx
            rdi = rbx;
            // d6610 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6615 mov rsi,QWORD PTR [rip+0x69be4] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // d661c add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // d6620 mov rdi,r15
            rdi = r15;
            // d6623 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6628 mov rdi,r14
            rdi = r14;
            // d662b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d662f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6630 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6632 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6634 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6635 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
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
            // 26d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x26d2;
        }
        0x26d2 => {
            // block 26d2
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
            // 2e7db mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e7de jmp 0x2e7e3
            pc = 0x2e7e3; continue;
        }
        0x2e7e0 => {
            // block 2e7e0
            // 2e7e0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            pc = 0x2e7e3;
        }
        0x2e7e3 => {
            // block 2e7e3
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

/// parity: P2v @0x61590 size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00061590<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x61590;
    loop {
        match pc {
        0x61590 => {
            // block 61590
            // 61590 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 61591 mov rbp,rsp
            rbp = rsp;
            // 61594 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 61595 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 61596 mov rbx,rdi
            rbx = rdi;
            // 61599 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 6159c jne 0x615b9
            if fl.ne() { pc = 0x615b9; continue; }
            pc = 0x6159e;
        }
        0x6159e => {
            // block 6159e
            // 6159e lea rax,[rip+0xce01a] # 0x12f5bf
            rax = m.env.data_va(277900);
            // 615a5 lea rsi,[rip+0xce019] # 0x12f5c5
            rsi = m.env.data_va(277906);
            // 615ac test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 615ae cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 615b2 mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 615b7 jmp 0x615d6
            pc = 0x615d6; continue;
        }
        0x615b9 => {
            // block 615b9
            // 615b9 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 615bb jne 0x615e1
            if fl.ne() { pc = 0x615e1; continue; }
            pc = 0x615bd;
        }
        0x615bd => {
            // block 615bd
            // 615bd lea rax,[rip+0xcdfe2] # 0x12f5a6
            rax = m.env.data_va(277875);
            // 615c4 lea rsi,[rip+0xcdfe2] # 0x12f5ad
            rsi = m.env.data_va(277882);
            // 615cb test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 615cd cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 615d1 mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            pc = 0x615d6;
        }
        0x615d6 => {
            // block 615d6
            // 615d6 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 615db cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 615df jmp 0x61600
            pc = 0x61600; continue;
        }
        0x615e1 => {
            // block 615e1
            // 615e1 lea rax,[rip+0xd76e0] # 0x138cc8
            rax = m.env.data_va(316565);
            // 615e8 lea rsi,[rip+0xcdfe8] # 0x12f5d7
            rsi = m.env.data_va(277924);
            // 615ef xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 615f1 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 615f3 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 615f7 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 615fc cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            pc = 0x61600;
        }
        0x61600 => {
            // block 61600
            // 61600 mov rdi,rbx
            rdi = rbx;
            // 61603 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 61608 mov rax,rbx
            rax = rbx;
            // 6160b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6160f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 61610 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 61611 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0a7c size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0a7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0a7c;
    loop {
        match pc {
        0xe0a7c => {
            // block e0a7c
            // e0a7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0a7d mov rbp,rsp
            rbp = rsp;
            // e0a80 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0a81 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0a82 mov rbx,rdi
            rbx = rdi;
            // e0a85 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a88 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a8a call QWORD PTR [rax+0x1a48]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0a95 mov rdi,rbx
            rdi = rbx;
            // e0a98 call QWORD PTR [rax+0x1a50]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a50)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0a9e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0aa1 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0aa6 mov rdi,rbx
            rdi = rbx;
            // e0aa9 call QWORD PTR [rax+0x1a58]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0aaf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ab2 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // e0ab7 mov rdi,rbx
            rdi = rbx;
            // e0aba call QWORD PTR [rax+0x1a60]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a60)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ac0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ac3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e0ac8 mov rdi,rbx
            rdi = rbx;
            // e0acb call QWORD PTR [rax+0x1a28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ad1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ad4 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e0ad9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0adb mov rdi,rbx
            rdi = rbx;
            // e0ade call QWORD PTR [rax+0x1a98]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ae4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ae7 mov rax,QWORD PTR [rax+0x1aa8]
            rax = m.r64(rax.wrapping_add(0x1aa8)) as u64;
            // e0aee mov esi,0x10000
            rsi = (0x10000u64) as u32 as u64;
            // e0af3 mov rdi,rbx
            rdi = rbx;
            // e0af6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0afa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0afb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0afc jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e3a4 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e3a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e3a4;
    loop {
        match pc {
        0x2e3a4 => {
            // block 2e3a4
            // 2e3a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e3a5 mov rbp,rsp
            rbp = rsp;
            // 2e3a8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e3aa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e3ac push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e3ae push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e3af mov r14,rdi
            r14 = rdi;
            // 2e3b2 lea rdi,[r14+0x28]
            rdi = r14.wrapping_add(0x28);
            // 2e3b6 lea rsi,[rip+0x10a90b] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 2e3bd call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3c2 mov al,BYTE PTR [r14+0x8]
            rax = (rax & !0xff) | ((m.r8(r14.wrapping_add(0x8)) as u64) as u8 as u64);
            // 2e3c6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2e3c8 jne 0x2e41e
            if fl.ne() { pc = 0x2e41e; continue; }
            pc = 0x2e3ca;
        }
        0x2e3ca => {
            // block 2e3ca
            // 2e3ca lea r15,[rip+0x18f33f] # 0x1bd710
            r15 = m.env.data_va(859869);
            // 2e3d1 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 2e3d4 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2e3d7 je 0x2e417
            if fl.e() { pc = 0x2e417; continue; }
            pc = 0x2e3d9;
        }
        0x2e3d9 => {
            // block 2e3d9
            // 2e3d9 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2e3dc;
        }
        0x2e3dc => {
            // block 2e3dc
            // 2e3dc call rcx
            { let r = { let t = rcx; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3de jmp 0x2e3f5
            pc = 0x2e3f5; continue;
        }
        0x2e3e0 => {
            // block 2e3e0
            // 2e3e0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 2e3e3 mov rdi,r14
            rdi = r14;
            // 2e3e6 mov rsi,rbx
            rsi = rbx;
            // 2e3e9 call QWORD PTR [rax+0x78]
            { let r = { let t = m.r64(rax.wrapping_add(0x78)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e3ec mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e3ef mov rdi,rbx
            rdi = rbx;
            // 2e3f2 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e3f5;
        }
        0x2e3f5 => {
            // block 2e3f5
            // 2e3f5 mov rbx,rax
            rbx = rax;
            // 2e3f8 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e3fb je 0x2e40a
            if fl.e() { pc = 0x2e40a; continue; }
            pc = 0x2e3fd;
        }
        0x2e3fd => {
            // block 2e3fd
            // 2e3fd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e400 mov rdi,rbx
            rdi = rbx;
            // 2e403 call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e406 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e408 je 0x2e3e0
            if fl.e() { pc = 0x2e3e0; continue; }
            pc = 0x2e40a;
        }
        0x2e40a => {
            // block 2e40a
            // 2e40a mov rcx,QWORD PTR [r15+r12*8+0x8]
            rcx = m.r64(r15.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8)) as u64;
            // 2e40f inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // 2e412 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 2e415 jne 0x2e3dc
            if fl.ne() { pc = 0x2e3dc; continue; }
            pc = 0x2e417;
        }
        0x2e417 => {
            // block 2e417
            // 2e417 mov BYTE PTR [r14+0x8],0x1
            m.w8(r14.wrapping_add(0x8), (0x1u64) as u8);
            // 2e41c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2e41e;
        }
        0x2e41e => {
            // block 2e41e
            // 2e41e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e41f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e421 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e423 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e425 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e426 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a600 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a600<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a600;
    loop {
        match pc {
        0x5a600 => {
            // block 5a600
            // 5a600 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a601 mov rbp,rsp
            rbp = rsp;
            // 5a604 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5a606 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5a608 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5a609 movzx r9d,cl
            r9 = ((rcx as u8 as u64)) as u32 as u64;
            // 5a60d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 5a613 xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // 5a616 lea r11,[rip+0xa6733] # 0x100d50
            r11 = m.env.data_va(87325);
            // 5a61d lea rbx,[rip+0xa691c] # 0x100f40
            rbx = m.env.data_va(87821);
            // 5a624 lea r14,[rip+0xa6b05] # 0x101130
            r14 = m.env.data_va(88317);
            // 5a62b lea r15,[rip+0xa6d5e] # 0x101390
            r15 = m.env.data_va(88925);
            pc = 0x5a632;
        }
        0x5a632 => {
            // block 5a632
            // 5a632 cmp DWORD PTR [r10+r11*1],edi
            fl = sub::<32>(m.r32(r10.wrapping_add(r11)) as u64, (rdi as u32 as u64)).1;
            // 5a636 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a638;
        }
        0x5a638 => {
            // block 5a638
            // 5a638 cmp esi,DWORD PTR [r10+rbx*1+0x4]
            fl = sub::<32>((rsi as u32 as u64), m.r32(r10.wrapping_add(rbx).wrapping_add(0x4)) as u64).1;
            // 5a63d jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a63f;
        }
        0x5a63f => {
            // block 5a63f
            // 5a63f cmp edx,DWORD PTR [r10+r14*1]
            fl = sub::<32>((rdx as u32 as u64), m.r32(r10.wrapping_add(r14)) as u64).1;
            // 5a643 jne 0x5a669
            if fl.ne() { pc = 0x5a669; continue; }
            pc = 0x5a645;
        }
        0x5a645 => {
            // block 5a645
            // 5a645 mov ecx,DWORD PTR [r10+r15*1]
            rcx = (m.r32(r10.wrapping_add(r15)) as u64) as u32 as u64;
            // 5a649 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a64b cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a64e ja 0x5a65c
            if fl.a() { pc = 0x5a65c; continue; }
            pc = 0x5a650;
        }
        0x5a650 => {
            // block 5a650
            // 5a650 mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a655 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a657 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a65a jmp 0x5a65e
            pc = 0x5a65e; continue;
        }
        0x5a65c => {
            // block 5a65c
            // 5a65c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a65e;
        }
        0x5a65e => {
            // block 5a65e
            // 5a65e movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5a661 xor eax,0x1
            { let r = (rax as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a664 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // 5a667 je 0x5a67a
            if fl.e() { pc = 0x5a67a; continue; }
            pc = 0x5a669;
        }
        0x5a669 => {
            // block 5a669
            // 5a669 inc r8
            { let (r, f) = inc::<64>(r8, fl); r8 = r; fl = f; }
            // 5a66c add r10,0x4
            { let (r, f) = add::<64>(r10, 0x4u64);
              r10 = r;
              fl = f; }
            // 5a670 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a672 cmp r8,0x7a
            fl = sub::<64>(r8, 0x7au64).1;
            // 5a676 jl 0x5a632
            if fl.l() { pc = 0x5a632; continue; }
            pc = 0x5a678;
        }
        0x5a678 => {
            // block 5a678
            // 5a678 jmp 0x5a67d
            pc = 0x5a67d; continue;
        }
        0x5a67a => {
            // block 5a67a
            // 5a67a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0x5a67d;
        }
        0x5a67d => {
            // block 5a67d
            // 5a67d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5a67e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5a680 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5a682 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a683 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x13ab0 size=134
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00013ab0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x13ab0;
    loop {
        match pc {
        0x13ab0 => {
            // block 13ab0
            // 13ab0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 13ab1 mov rbp,rsp
            rbp = rsp;
            // 13ab4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 13ab6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 13ab8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 13aba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 13abb mov r15,rsi
            r15 = rsi;
            // 13abe mov r14,rdi
            r14 = rdi;
            // 13ac1 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 13ac4 je 0x13b2d
            if fl.e() { pc = 0x13b2d; continue; }
            pc = 0x13ac6;
        }
        0x13ac6 => {
            // block 13ac6
            // 13ac6 mov al,BYTE PTR [r15]
            rax = (rax & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // 13ac9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 13acb je 0x13b2d
            if fl.e() { pc = 0x13b2d; continue; }
            pc = 0x13acd;
        }
        0x13acd => {
            // block 13acd
            // 13acd add r14,0x298
            { let (r, f) = add::<64>(r14, 0x298u64);
              r14 = r;
              fl = f; }
            // 13ad4 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 13ada jmp 0x13ae3
            pc = 0x13ae3; continue;
        }
        0x13adc => {
            // block 13adc
            // 13adc mov al,BYTE PTR [r15+rbx*1]
            rax = (rax & !0xff) | ((m.r8(r15.wrapping_add(rbx)) as u64) as u8 as u64);
            // 13ae0 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            pc = 0x13ae3;
        }
        0x13ae3 => {
            // block 13ae3
            // 13ae3 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13ae5 add cl,0xbf
            { let (r, f) = add::<8>((rcx as u8 as u64), 0xbfu64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13ae8 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13aeb cmp ecx,0x1a
            fl = sub::<32>((rcx as u32 as u64), 0x1au64).1;
            // 13aee jb 0x13b12
            if fl.b() { pc = 0x13b12; continue; }
            pc = 0x13af0;
        }
        0x13af0 => {
            // block 13af0
            // 13af0 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13af2 add cl,0x9f
            { let (r, f) = add::<8>((rcx as u8 as u64), 0x9fu64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13af5 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13af8 cmp ecx,0x1a
            fl = sub::<32>((rcx as u32 as u64), 0x1au64).1;
            // 13afb jb 0x13b12
            if fl.b() { pc = 0x13b12; continue; }
            pc = 0x13afd;
        }
        0x13afd => {
            // block 13afd
            // 13afd movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // 13b00 cmp ecx,0x5f
            fl = sub::<32>((rcx as u32 as u64), 0x5fu64).1;
            // 13b03 je 0x13b12
            if fl.e() { pc = 0x13b12; continue; }
            pc = 0x13b05;
        }
        0x13b05 => {
            // block 13b05
            // 13b05 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 13b07 add cl,0xd0
            { let (r, f) = add::<8>((rcx as u8 as u64), 0xd0u64);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 13b0a movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 13b0d cmp ecx,0x9
            fl = sub::<32>((rcx as u32 as u64), 0x9u64).1;
            // 13b10 ja 0x13b2d
            if fl.a() { pc = 0x13b2d; continue; }
            pc = 0x13b12;
        }
        0x13b12 => {
            // block 13b12
            // 13b12 movsx esi,al
            rsi = (sx::<8>((rax as u8 as u64)) as u64) as u32 as u64;
            // 13b15 mov rdi,r14
            rdi = r14;
            // 13b18 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13b1d mov ebx,r12d
            rbx = ((r12 as u32 as u64)) as u32 as u64;
            // 13b20 mov rdi,r15
            rdi = r15;
            // 13b23 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13b28 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 13b2b jb 0x13adc
            if fl.b() { pc = 0x13adc; continue; }
            pc = 0x13b2d;
        }
        0x13b2d => {
            // block 13b2d
            // 13b2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 13b2e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 13b30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13b32 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13b34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13b35 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2df5c size=135
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002df5c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2df5c;
    loop {
        match pc {
        0x2df5c => {
            // block 2df5c
            // 2df5c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2df5d mov rbp,rsp
            rbp = rsp;
            // 2df60 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2df62 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2df63 mov rax,QWORD PTR [rip+0x11247e] # 0x1403e8
            rax = m.r64(m.env.data_va(347061)) as u64;
            // 2df6a mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 2df6d lea rsi,[rip+0x118c14] # 0x146b88
            rsi = m.env.data_va(373589);
            // 2df74 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2df76 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2df7b mov rdi,rbx
            rdi = rbx;
            // 2df7e call 0xebf5a
            { let r = { let t = m.env.stub_va(0x727); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df83 mov r14,rax
            r14 = rax;
            // 2df86 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2df89 je 0x2dfde
            if fl.e() { pc = 0x2dfde; continue; }
            pc = 0x2df8b;
        }
        0x2df8b => {
            // block 2df8b
            // 2df8b mov rdi,rbx
            rdi = rbx;
            // 2df8e mov rsi,r14
            rsi = r14;
            // 2df91 call 0xebf36
            { let r = { let t = m.env.stub_va(0x703); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2df96 mov QWORD PTR [rip+0x193abb],rax # 0x1c1a58
            m.w64(m.env.data_va(877093), (rax) as u64);
            // 2df9d test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dfa0 je 0x2dfd2
            if fl.e() { pc = 0x2dfd2; continue; }
            pc = 0x2dfa2;
        }
        0x2dfa2 => {
            // block 2dfa2
            // 2dfa2 lea rsi,[rip+0x118bff] # 0x146ba8
            rsi = m.env.data_va(373621);
            // 2dfa9 mov rdi,rax
            rdi = rax;
            // 2dfac call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dfb1 mov QWORD PTR [rip+0x193aa8],rax # 0x1c1a60
            m.w64(m.env.data_va(877101), (rax) as u64);
            // 2dfb8 mov rdi,QWORD PTR [rip+0x193a99] # 0x1c1a58
            rdi = m.r64(m.env.data_va(877093)) as u64;
            // 2dfbf lea rsi,[rip+0x118c02] # 0x146bc8
            rsi = m.env.data_va(373653);
            // 2dfc6 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dfcb mov QWORD PTR [rip+0x193a96],rax # 0x1c1a68
            m.w64(m.env.data_va(877109), (rax) as u64);
            pc = 0x2dfd2;
        }
        0x2dfd2 => {
            // block 2dfd2
            // 2dfd2 mov rdi,r14
            rdi = r14;
            // 2dfd5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dfd6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2dfd8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dfd9 jmp 0xebf42
            { let r = { let t = m.env.stub_va(0x70f); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2dfde => {
            // block 2dfde
            // 2dfde pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dfdf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2dfe1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dfe2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd3547 size=135
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3547<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3547;
    loop {
        match pc {
        0xd3547 => {
            // block d3547
            // d3547 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3548 mov rbp,rsp
            rbp = rsp;
            // d354b mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // d354d cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // d3550 jne 0xd3562
            if fl.ne() { pc = 0xd3562; continue; }
            pc = 0xd3552;
        }
        0xd3552 => {
            // block d3552
            // d3552 cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3555 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3557;
        }
        0xd3557 => {
            // block d3557
            // d3557 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3559 lea rcx,[rip+0xea030] # 0x1bd590
            rcx = m.env.data_va(859485);
            // d3560 jmp 0xd358b
            pc = 0xd358b; continue;
        }
        0xd3562 => {
            // block d3562
            // d3562 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3565 jne 0xd357d
            if fl.ne() { pc = 0xd357d; continue; }
            pc = 0xd3567;
        }
        0xd3567 => {
            // block d3567
            // d3567 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // d356c cmp esi,0x14
            fl = sub::<32>((rsi as u32 as u64), 0x14u64).1;
            // d356f je 0xd35a4
            if fl.e() { pc = 0xd35a4; continue; }
            pc = 0xd3571;
        }
        0xd3571 => {
            // block d3571
            // d3571 cmp esi,0x15
            fl = sub::<32>((rsi as u32 as u64), 0x15u64).1;
            // d3574 jne 0xd35a2
            if fl.ne() { pc = 0xd35a2; continue; }
            pc = 0xd3576;
        }
        0xd3576 => {
            // block d3576
            // d3576 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // d357b jmp 0xd35a4
            pc = 0xd35a4; continue;
        }
        0xd357d => {
            // block d357d
            // d357d cmp DWORD PTR [rdi+0x4],esi
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, (rsi as u32 as u64)).1;
            // d3580 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd3582;
        }
        0xd3582 => {
            // block d3582
            // d3582 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d3584 lea rcx,[rip+0xea0c5] # 0x1bd650
            rcx = m.env.data_va(859677);
            pc = 0xd358b;
        }
        0xd358b => {
            // block d358b
            // d358b mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d358f mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d3592 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d3596 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d359b test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d359d sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d35a0 jmp 0xd35cc
            pc = 0xd35cc; continue;
        }
        0xd35a2 => {
            // block d35a2
            // d35a2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            pc = 0xd35a4;
        }
        0xd35a4 => {
            // block d35a4
            // d35a4 cmp eax,DWORD PTR [rdi+0x4]
            fl = sub::<32>((rax as u32 as u64), m.r32(rdi.wrapping_add(0x4)) as u64).1;
            // d35a7 jne 0xd35ca
            if fl.ne() { pc = 0xd35ca; continue; }
            pc = 0xd35a9;
        }
        0xd35a9 => {
            // block d35a9
            // d35a9 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // d35ab lea rcx,[rip+0xe9f1e] # 0x1bd4d0
            rcx = m.env.data_va(859293);
            // d35b2 mov rsi,QWORD PTR [rcx+rax*8]
            rsi = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // d35b6 mov edx,DWORD PTR [rdi+0x8]
            rdx = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // d35b9 add rdi,0x40
            { let (r, f) = add::<64>(rdi, 0x40u64);
              rdi = r;
              fl = f; }
            // d35bd call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d35c2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d35c4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d35c6 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d35c8 je 0xd35cc
            if fl.e() { pc = 0xd35cc; continue; }
            pc = 0xd35ca;
        }
        0xd35ca => {
            // block d35ca
            // d35ca xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd35cc;
        }
        0xd35cc => {
            // block d35cc
            // d35cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d35cd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6f36 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006f36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6f36;
    loop {
        match pc {
        0x6f36 => {
            // block 6f36
            // 6f36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6f37 mov rbp,rsp
            rbp = rsp;
            // 6f3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6f3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6f3e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6f40 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6f42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6f43 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6f44 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 6f47 mov r14,rdx
            r14 = rdx;
            // 6f4a mov r15,rsi
            r15 = rsi;
            // 6f4d mov rbx,rdi
            rbx = rdi;
            // 6f50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6f53 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 6f56 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f5c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6f5e je 0x6f65
            if fl.e() { pc = 0x6f65; continue; }
            pc = 0x6f60;
        }
        0x6f60 => {
            // block 6f60
            // 6f60 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 6f63 jmp 0x6fb1
            pc = 0x6fb1; continue;
        }
        0x6f65 => {
            // block 6f65
            // 6f65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6f68 movzx r13d,r12w
            r13 = ((r12 as u16 as u64)) as u32 as u64;
            // 6f6c lea rcx,[rip+0xf847d] # 0xff3f0
            rcx = m.env.data_va(80829);
            // 6f73 mov esi,DWORD PTR [rcx+r13*4]
            rsi = (m.r32(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 6f77 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 6f7a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6f7f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6f82 mov rdi,rbx
            rdi = rbx;
            // 6f85 mov rdx,r15
            rdx = r15;
            // 6f88 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f8b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6f8d je 0x6fb1
            if fl.e() { pc = 0x6fb1; continue; }
            pc = 0x6f8f;
        }
        0x6f8f => {
            // block 6f8f
            // 6f8f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 6f92 lea rcx,[rip+0xf8487] # 0xff420
            rcx = m.env.data_va(80877);
            // 6f99 mov esi,DWORD PTR [rcx+r13*4]
            rsi = (m.r32(rcx.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 6f9d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6fa2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6fa5 mov rdi,rbx
            rdi = rbx;
            // 6fa8 mov rdx,r14
            rdx = r14;
            // 6fab call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6fae mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x6fb1;
        }
        0x6fb1 => {
            // block 6fb1
            // 6fb1 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 6fb4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6fb9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6fbb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6fbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6fbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6fc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6fc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1af22 size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001af22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1af22;
    loop {
        match pc {
        0x1af22 => {
            // block 1af22
            // 1af22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1af23 mov rbp,rsp
            rbp = rsp;
            // 1af26 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1af28 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1af2a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1af2b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1af2c mov r14,rdi
            r14 = rdi;
            // 1af2f mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1af33 jmp 0x1af7f
            pc = 0x1af7f; continue;
        }
        0x1af35 => {
            // block 1af35
            // 1af35 lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 1af3c mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 1af40 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1af44 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1af49 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1af4d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1af52 lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 1af56 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1af5b lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 1af5f call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1af64 lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1af6b call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1af70 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1af77 mov rdi,rbx
            rdi = rbx;
            // 1af7a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1af7f;
        }
        0x1af7f => {
            // block 1af7f
            // 1af7f mov rbx,QWORD PTR [r14+0x10]
            rbx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 1af83 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 1af86 jne 0x1af35
            if fl.ne() { pc = 0x1af35; continue; }
            pc = 0x1af88;
        }
        0x1af88 => {
            // block 1af88
            // 1af88 mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // 1af8b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1af8f test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1af92 je 0x1af9f
            if fl.e() { pc = 0x1af9f; continue; }
            pc = 0x1af94;
        }
        0x1af94 => {
            // block 1af94
            // 1af94 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1af95 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1af97 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1af99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1af9a jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x1af9f => {
            // block 1af9f
            // 1af9f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1afa0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1afa2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1afa4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1afa5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5825c size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005825c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5825c;
    loop {
        match pc {
        0x5825c => {
            // block 5825c
            // 5825c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5825d mov rbp,rsp
            rbp = rsp;
            // 58260 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58262 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58264 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58265 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58266 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 58269 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5826d lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // 58271 mov rcx,QWORD PTR [rip+0xe8138] # 0x1403b0
            rcx = m.r64(m.env.data_va(347005)) as u64;
            // 58278 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 5827c mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 58280 lea r14,[rdi+rax*1+0x80]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x80);
            // 58288 lea rdx,[rcx+0x68]
            rdx = rcx.wrapping_add(0x68);
            // 5828c mov QWORD PTR [rdi+rax*1+0x80],rdx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x80), (rdx) as u64);
            // 58294 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 58298 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // 5829d lea rbx,[rdi+rax*1+0x18]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x18);
            // 582a2 mov rcx,QWORD PTR [rip+0xe80ff] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 582a9 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 582ad mov QWORD PTR [rdi+rax*1+0x18],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x18), (rcx) as u64);
            // 582b2 lea rdi,[rdi+rax*1+0x58]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x58);
            // 582b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582bc mov rdi,rbx
            rdi = rbx;
            // 582bf call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582c4 mov rsi,QWORD PTR [rip+0xe7f2d] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 582cb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 582cf mov rdi,r15
            rdi = r15;
            // 582d2 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582d7 mov rdi,r14
            rdi = r14;
            // 582da add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 582de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 582df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 582e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 582e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 582e4 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a1e0 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001a1e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a1e0;
    loop {
        match pc {
        0x1a1e0 => {
            // block 1a1e0
            // 1a1e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a1e1 mov rbp,rsp
            rbp = rsp;
            // 1a1e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a1e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a1e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a1e9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1a1ea mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1a1ed mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 1a1f1 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // 1a1f5 mov rcx,QWORD PTR [rip+0x1261bc] # 0x1403b8
            rcx = m.r64(m.env.data_va(347013)) as u64;
            // 1a1fc lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 1a200 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 1a204 lea r14,[rdi+rax*1+0x78]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x78);
            // 1a209 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 1a20d mov QWORD PTR [rdi+rax*1+0x78],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x78), (rcx) as u64);
            // 1a212 lea rbx,[rdi+rax*1+0x10]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x10);
            // 1a217 mov rcx,QWORD PTR [rip+0x12618a] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 1a21e add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 1a222 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // 1a227 lea rdi,[rdi+rax*1+0x50]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x50);
            // 1a22c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a231 mov rdi,rbx
            rdi = rbx;
            // 1a234 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a239 mov rsi,QWORD PTR [rip+0x125fc0] # 0x140200
            rsi = m.r64(m.env.data_va(346573)) as u64;
            // 1a240 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 1a244 mov rdi,r15
            rdi = r15;
            // 1a247 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a24c mov rdi,r14
            rdi = r14;
            // 1a24f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a254 mov rdi,r15
            rdi = r15;
            // 1a257 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a25b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a25c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a25e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a260 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a261 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdc3d0 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dc3d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdc3d0;
    loop {
        match pc {
        0xdc3d0 => {
            // block dc3d0
            // dc3d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc3d1 mov rbp,rsp
            rbp = rsp;
            // dc3d4 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // dc3d7 jb 0xdc3ff
            if fl.b() { pc = 0xdc3ff; continue; }
            pc = 0xdc3d9;
        }
        0xdc3d9 => {
            // block dc3d9
            // dc3d9 movzx eax,si
            rax = ((rsi as u16 as u64)) as u32 as u64;
            // dc3dc add eax,0xfffffff8
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc3df cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // dc3e2 ja 0xdc41c
            if fl.a() { pc = 0xdc41c; continue; }
            pc = 0xdc3e4;
        }
        0xdc3e4 => {
            // block dc3e4
            // dc3e4 lea rcx,[rip+0x61] # 0xdc44c
            rcx = m.env.code_va(0xdc44c);
            // dc3eb movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // dc3ef add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // dc3f2 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xdc3f4 => {
            // block dc3f4
            // dc3f4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc3f7 call QWORD PTR [rax+0x1980]
            { let r = { let t = m.r64(rax.wrapping_add(0x1980)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc3fd jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc3ff => {
            // block dc3ff
            // dc3ff mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc402 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // dc405 imul rcx,rcx,0x608
            { let (r, f) = imul::<64>(rcx, 0x608u64); rcx = r; fl = f; }
            // dc40c lea rsi,[rip+0xe138d] # 0x1bd7a0
            rsi = m.env.data_va(860013);
            // dc413 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // dc416 call QWORD PTR [rax+0x19f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdc41c;
        }
        0xdc41c => {
            // block dc41c
            // dc41c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dc41e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc41f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xdc420 => {
            // block dc420
            // dc420 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc423 call QWORD PTR [rax+0x1988]
            { let r = { let t = m.r64(rax.wrapping_add(0x1988)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc429 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc42b => {
            // block dc42b
            // dc42b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc42e call QWORD PTR [rax+0x1990]
            { let r = { let t = m.r64(rax.wrapping_add(0x1990)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc434 jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc436 => {
            // block dc436
            // dc436 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc439 call QWORD PTR [rax+0x19a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19a8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc43f jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
        0xdc441 => {
            // block dc441
            // dc441 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // dc444 call QWORD PTR [rax+0x19b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19b0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dc44a jmp 0xdc41c
            pc = 0xdc41c; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb514e size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b514e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb514e;
    loop {
        match pc {
        0xb514e => {
            // block b514e
            // b514e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b514f mov rbp,rsp
            rbp = rsp;
            // b5152 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5154 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5156 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5158 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5159 mov rbx,rsi
            rbx = rsi;
            // b515c mov r14,rdi
            r14 = rdi;
            // b515f mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // b5164 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5169 mov r12,rax
            r12 = rax;
            // b516c mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // b516e mov DWORD PTR [r12+0x20],eax
            m.w32(r12.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // b5173 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // b5177 mov QWORD PTR [r12+0x38],rax
            m.w64(r12.wrapping_add(0x38), (rax) as u64);
            // b517c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5180 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b5184 mov QWORD PTR [r12+0x30],rcx
            m.w64(r12.wrapping_add(0x30), (rcx) as u64);
            // b5189 mov QWORD PTR [r12+0x28],rax
            m.w64(r12.wrapping_add(0x28), (rax) as u64);
            // b518e mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // b5196 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // b519e mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // b51a6 mov rdi,r14
            rdi = r14;
            // b51a9 mov rsi,r12
            rsi = r12;
            // b51ac call 0xebc60
            { let r = { let t = m.env.stub_va(0x42d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51b1 mov r14,rax
            r14 = rax;
            // b51b4 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // b51b7 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // b51bb jne 0xb51d1
            if fl.ne() { pc = 0xb51d1; continue; }
            pc = 0xb51bd;
        }
        0xb51bd => {
            // block b51bd
            // b51bd mov rdi,r12
            rdi = r12;
            // b51c0 add rdi,0x28
            { let (r, f) = add::<64>(rdi, 0x28u64);
              rdi = r;
              fl = f; }
            // b51c4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b51c9 mov rdi,r12
            rdi = r12;
            // b51cc call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb51d1;
        }
        0xb51d1 => {
            // block b51d1
            // b51d1 mov rax,r14
            rax = r14;
            // b51d4 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b51d7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b51d8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b51da pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b51dc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b51de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b51df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc7f94 size=147
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c7f94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc7f94;
    loop {
        match pc {
        0xc7f94 => {
            // block c7f94
            // c7f94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7f95 mov rbp,rsp
            rbp = rsp;
            // c7f98 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c7f9a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c7f9c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c7f9e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c7fa0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c7fa1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c7fa2 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // c7fa5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // c7fa7 mov r14,rsi
            r14 = rsi;
            // c7faa mov r12,rdi
            r12 = rdi;
            // c7fad cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // c7fb0 jne 0xc7fd1
            if fl.ne() { pc = 0xc7fd1; continue; }
            pc = 0xc7fb2;
        }
        0xc7fb2 => {
            // block c7fb2
            // c7fb2 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7fb6 mov rdi,r12
            rdi = r12;
            // c7fb9 mov rsi,r14
            rsi = r14;
            // c7fbc mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fbe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c7fc2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c7fc3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c7fc5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c7fc7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c7fc9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c7fcb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c7fcc jmp 0xc68b5
            { let r = { let t = m.env.code_va(0xc68b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0xc7fd1 => {
            // block c7fd1
            // c7fd1 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // c7fd4 jne 0xc8018
            if fl.ne() { pc = 0xc8018; continue; }
            pc = 0xc7fd6;
        }
        0xc7fd6 => {
            // block c7fd6
            // c7fd6 lea edi,[rbx+rbx*1]
            rdi = (rbx.wrapping_add(rbx)) as u32 as u64;
            // c7fd9 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // c7fdc call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7fe1 mov r13,rax
            r13 = rax;
            // c7fe4 mov rdi,r12
            rdi = r12;
            // c7fe7 mov rsi,r13
            rsi = r13;
            // c7fea mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7fec call 0xc5cac
            { let r = { let t = m.env.code_va(0xc5cac); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7ff1 movzx ecx,r15b
            rcx = ((r15 as u8 as u64)) as u32 as u64;
            // c7ff5 mov rdi,r13
            rdi = r13;
            // c7ff8 mov rsi,r14
            rsi = r14;
            // c7ffb mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // c7ffd call 0xc6cdf
            { let r = { let t = m.env.code_va(0xc6cdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c8002 mov rdi,r13
            rdi = r13;
            // c8005 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c8009 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c800a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c800c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c800e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8010 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8012 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8013 jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xc8018 => {
            // block c8018
            // c8018 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c801c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c801d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c801f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c8021 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c8023 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c8025 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8026 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4244 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4244<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4244;
    loop {
        match pc {
        0xc4244 => {
            // block c4244
            // c4244 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4245 mov rbp,rsp
            rbp = rsp;
            // c4248 mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c424b mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c424f cmp rdx,r8
            fl = sub::<64>(rdx, r8).1;
            // c4252 je 0xc42a2
            if fl.e() { pc = 0xc42a2; continue; }
            pc = 0xc4254;
        }
        0xc4254 => {
            // block c4254
            // c4254 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc4258;
        }
        0xc4258 => {
            // block c4258
            // c4258 mov rax,QWORD PTR [rdx-0x8]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64;
            // c425c mov QWORD PTR [rcx-0x8],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // c4260 mov r9,QWORD PTR [rdx-0x18]
            r9 = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // c4264 mov rax,QWORD PTR [rdx-0x10]
            rax = m.r64(rdx.wrapping_add(0xffff_ffff_ffff_fff0)) as u64;
            // c4268 mov QWORD PTR [rcx-0x10],rax
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_fff0), (rax) as u64);
            // c426c mov QWORD PTR [rcx-0x18],r9
            m.w64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8), (r9) as u64);
            // c4270 mov QWORD PTR [rdx-0x8],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff8), (0x0u64) as u64);
            // c4278 mov QWORD PTR [rdx-0x10],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u64);
            // c4280 mov QWORD PTR [rdx-0x18],0x0
            m.w64(rdx.wrapping_add(0xffff_ffff_ffff_ffe8), (0x0u64) as u64);
            // c4288 lea rdx,[rdx-0x18]
            rdx = rdx.wrapping_add(0xffff_ffff_ffff_ffe8);
            // c428c mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4290 add rcx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_ffe8u64);
              rcx = r;
              fl = f; }
            // c4294 mov QWORD PTR [rsi+0x8],rcx
            m.w64(rsi.wrapping_add(0x8), (rcx) as u64);
            // c4298 cmp r8,rdx
            fl = sub::<64>(r8, rdx).1;
            // c429b jne 0xc4258
            if fl.ne() { pc = 0xc4258; continue; }
            pc = 0xc429d;
        }
        0xc429d => {
            // block c429d
            // c429d mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // c42a0 jmp 0xc42a6
            pc = 0xc42a6; continue;
        }
        0xc42a2 => {
            // block c42a2
            // c42a2 mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            pc = 0xc42a6;
        }
        0xc42a6 => {
            // block c42a6
            // c42a6 mov QWORD PTR [rdi],rcx
            m.w64(rdi, (rcx) as u64);
            // c42a9 mov QWORD PTR [rsi+0x8],r8
            m.w64(rsi.wrapping_add(0x8), (r8) as u64);
            // c42ad mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c42b1 mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c42b5 mov QWORD PTR [rdi+0x8],rcx
            m.w64(rdi.wrapping_add(0x8), (rcx) as u64);
            // c42b9 mov QWORD PTR [rsi+0x10],rax
            m.w64(rsi.wrapping_add(0x10), (rax) as u64);
            // c42bd mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c42c1 mov rcx,QWORD PTR [rsi+0x18]
            rcx = m.r64(rsi.wrapping_add(0x18)) as u64;
            // c42c5 mov QWORD PTR [rdi+0x10],rcx
            m.w64(rdi.wrapping_add(0x10), (rcx) as u64);
            // c42c9 mov QWORD PTR [rsi+0x18],rax
            m.w64(rsi.wrapping_add(0x18), (rax) as u64);
            // c42cd mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c42d1 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c42d4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c42d5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5d3a2 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005d3a2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5d3a2;
    loop {
        match pc {
        0x5d3a2 => {
            // block 5d3a2
            // 5d3a2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d3a3 mov rbp,rsp
            rbp = rsp;
            // 5d3a6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d3a7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 5d3a8 mov rbx,rdi
            rbx = rdi;
            // 5d3ab cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 5d3ae je 0x5d3d5
            if fl.e() { pc = 0x5d3d5; continue; }
            pc = 0x5d3b0;
        }
        0x5d3b0 => {
            // block 5d3b0
            // 5d3b0 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 5d3b3 jne 0x5d3f5
            if fl.ne() { pc = 0x5d3f5; continue; }
            pc = 0x5d3b5;
        }
        0x5d3b5 => {
            // block 5d3b5
            // 5d3b5 lea rax,[rip+0xce960] # 0x12bd1c
            rax = m.env.data_va(263401);
            // 5d3bc lea rsi,[rip+0xce95d] # 0x12bd20
            rsi = m.env.data_va(263405);
            // 5d3c3 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d3c5 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d3c9 mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d3ce mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 5d3d3 jmp 0x5d417
            pc = 0x5d417; continue;
        }
        0x5d3d5 => {
            // block 5d3d5
            // 5d3d5 lea rax,[rip+0xce80c] # 0x12bbe8
            rax = m.env.data_va(263093);
            // 5d3dc lea rsi,[rip+0xce953] # 0x12bd36
            rsi = m.env.data_va(263427);
            // 5d3e3 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d3e5 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d3e9 mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d3ee mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 5d3f3 jmp 0x5d417
            pc = 0x5d417; continue;
        }
        0x5d3f5 => {
            // block 5d3f5
            // 5d3f5 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 5d3f7 jne 0x5d41d
            if fl.ne() { pc = 0x5d41d; continue; }
            pc = 0x5d3f9;
        }
        0x5d3f9 => {
            // block 5d3f9
            // 5d3f9 lea rax,[rip+0xce8fd] # 0x12bcfd
            rax = m.env.data_va(263370);
            // 5d400 lea rsi,[rip+0xce8fa] # 0x12bd01
            rsi = m.env.data_va(263374);
            // 5d407 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 5d409 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 5d40d mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 5d412 mov edx,0x1a
            rdx = (0x1au64) as u32 as u64;
            pc = 0x5d417;
        }
        0x5d417 => {
            // block 5d417
            // 5d417 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 5d41b jmp 0x5d426
            pc = 0x5d426; continue;
        }
        0x5d41d => {
            // block 5d41d
            // 5d41d lea rsi,[rip+0xdb8a4] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 5d424 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x5d426;
        }
        0x5d426 => {
            // block 5d426
            // 5d426 mov rdi,rbx
            rdi = rbx;
            // 5d429 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5d42e mov rax,rbx
            rax = rbx;
            // 5d431 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5d435 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5d436 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5d437 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba598 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba598;
    loop {
        match pc {
        0xba598 => {
            // block ba598
            // ba598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba599 mov rbp,rsp
            rbp = rsp;
            // ba59c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba59e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba5a0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba5a1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba5a2 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba5a5 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // ba5a9 lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // ba5ad mov rcx,QWORD PTR [rip+0x85dfc] # 0x1403b0
            rcx = m.r64(m.env.data_va(347005)) as u64;
            // ba5b4 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // ba5b8 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // ba5bc lea r14,[rdi+rax*1+0x80]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x80);
            // ba5c4 lea rdx,[rcx+0x68]
            rdx = rcx.wrapping_add(0x68);
            // ba5c8 mov QWORD PTR [rdi+rax*1+0x80],rdx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x80), (rdx) as u64);
            // ba5d0 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // ba5d4 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // ba5d9 lea rbx,[rdi+rax*1+0x18]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x18);
            // ba5de mov rcx,QWORD PTR [rip+0x85dc3] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // ba5e5 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // ba5e9 mov QWORD PTR [rdi+rax*1+0x18],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x18), (rcx) as u64);
            // ba5ee lea rdi,[rdi+rax*1+0x58]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x58);
            // ba5f3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba5f8 mov rdi,rbx
            rdi = rbx;
            // ba5fb call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba600 mov rsi,QWORD PTR [rip+0x85bf1] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // ba607 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ba60b mov rdi,r15
            rdi = r15;
            // ba60e call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba613 mov rdi,r14
            rdi = r14;
            // ba616 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba61b mov rdi,r15
            rdi = r15;
            // ba61e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba622 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba623 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba625 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba627 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba628 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd38e0 size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d38e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd38e0;
    loop {
        match pc {
        0xd38e0 => {
            // block d38e0
            // d38e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d38e1 mov rbp,rsp
            rbp = rsp;
            // d38e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d38e9 sub rsp,0x488
            { let (r, f) = sub::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d38f0 mov r15,rsi
            r15 = rsi;
            // d38f3 mov rbx,rdi
            rbx = rdi;
            // d38f6 test r15,r15
            fl = logic::<64>(r15 & r15);
            // d38f9 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd38fb;
        }
        0xd38fb => {
            // block d38fb
            // d38fb xorps xmm0,xmm0
            x0 = 0;
            // d38fe movaps XMMWORD PTR [rbp-0x4a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fb60), x0);
            // d3905 mov QWORD PTR [rbp-0x490],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (0x0u64) as u64);
            // d3910 mov DWORD PTR [rbp-0x4a0],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb60), (0x5u64) as u32);
            // d391a mov DWORD PTR [rbp-0x49c],0x480
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fb64), (0x480u64) as u32);
            // d3924 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // d392b mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3930 mov rdi,r14
            rdi = r14;
            // d3933 mov rsi,r15
            rsi = r15;
            // d3936 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d393b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d393e lea rsi,[rbp-0x4a0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb60);
            // d3945 mov rdi,rbx
            rdi = rbx;
            // d3948 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d394e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3950 je 0xd3966
            if fl.e() { pc = 0xd3966; continue; }
            pc = 0xd3952;
        }
        0xd3952 => {
            // block d3952
            // d3952 mov edx,0x468
            rdx = (0x468u64) as u32 as u64;
            // d3957 mov rdi,r15
            rdi = r15;
            // d395a mov rsi,r14
            rsi = r14;
            // d395d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3962 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3964 jmp 0xd3968
            pc = 0xd3968; continue;
        }
        0xd3966 => {
            // block d3966
            // d3966 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3968;
        }
        0xd3968 => {
            // block d3968
            // d3968 add rsp,0x488
            { let (r, f) = add::<64>(rsp, 0x488u64);
              rsp = r;
              fl = f; }
            // d396f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3970 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3972 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3974 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3975 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68a10 size=152
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068a10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68a10;
    loop {
        match pc {
        0x68a10 => {
            // block 68a10
            // 68a10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68a11 mov rbp,rsp
            rbp = rsp;
            // 68a14 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68a15 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 68a16 mov rbx,rdi
            rbx = rdi;
            // 68a19 lea rdi,[rbx+0x1f8]
            rdi = rbx.wrapping_add(0x1f8);
            // 68a20 mov rsi,QWORD PTR [rbx+0x200]
            rsi = m.r64(rbx.wrapping_add(0x200)) as u64;
            // 68a27 call 0xebc48
            { let r = { let t = m.env.stub_va(0x415); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a2c lea rdi,[rbx+0x1e0]
            rdi = rbx.wrapping_add(0x1e0);
            // 68a33 mov rsi,QWORD PTR [rbx+0x1e8]
            rsi = m.r64(rbx.wrapping_add(0x1e8)) as u64;
            // 68a3a call 0xebc1e
            { let r = { let t = m.env.stub_va(0x3eb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a3f lea rdi,[rbx+0x1c8]
            rdi = rbx.wrapping_add(0x1c8);
            // 68a46 mov rsi,QWORD PTR [rbx+0x1d0]
            rsi = m.r64(rbx.wrapping_add(0x1d0)) as u64;
            // 68a4d call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a52 lea rdi,[rbx+0x1b0]
            rdi = rbx.wrapping_add(0x1b0);
            // 68a59 mov rsi,QWORD PTR [rbx+0x1b8]
            rsi = m.r64(rbx.wrapping_add(0x1b8)) as u64;
            // 68a60 call 0xebc36
            { let r = { let t = m.env.stub_va(0x403); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a65 lea rdi,[rbx+0x198]
            rdi = rbx.wrapping_add(0x198);
            // 68a6c mov rsi,QWORD PTR [rbx+0x1a0]
            rsi = m.r64(rbx.wrapping_add(0x1a0)) as u64;
            // 68a73 call 0xebc36
            { let r = { let t = m.env.stub_va(0x403); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a78 lea rdi,[rbx+0x180]
            rdi = rbx.wrapping_add(0x180);
            // 68a7f mov rsi,QWORD PTR [rbx+0x188]
            rsi = m.r64(rbx.wrapping_add(0x188)) as u64;
            // 68a86 call 0xebc8a
            { let r = { let t = m.env.stub_va(0x457); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68a8b mov rsi,QWORD PTR [rbx+0x170]
            rsi = m.r64(rbx.wrapping_add(0x170)) as u64;
            // 68a92 add rbx,0x168
            { let (r, f) = add::<64>(rbx, 0x168u64);
              rbx = r;
              fl = f; }
            // 68a99 mov rdi,rbx
            rdi = rbx;
            // 68a9c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68aa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68aa1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68aa2 jmp 0xebc6c
            { let r = { let t = m.env.stub_va(0x439); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdedb8 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dedb8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdedb8;
    loop {
        match pc {
        0xdedb8 => {
            // block dedb8
            // dedb8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dedb9 mov rbp,rsp
            rbp = rsp;
            // dedbc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dedbe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dedc0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dedc2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dedc3 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // dedc6 mov r14,rdx
            r14 = rdx;
            // dedc9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // dedcc mov rbx,rdi
            rbx = rdi;
            // dedcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedd2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dedd4 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dedd9 call QWORD PTR [rax+0x368]
            { let r = { let t = m.r64(rax.wrapping_add(0x368)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deddf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dede2 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // dede7 mov rdi,rbx
            rdi = rbx;
            // dedea mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // deded mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // dedf0 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dedf6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dedf9 mov rdi,rbx
            rdi = rbx;
            // dedfc mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // dedff call QWORD PTR [rax+0x1908]
            { let r = { let t = m.r64(rax.wrapping_add(0x1908)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee05 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee08 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // dee0d mov rdi,rbx
            rdi = rbx;
            // dee10 call QWORD PTR [rax+0x1940]
            { let r = { let t = m.r64(rax.wrapping_add(0x1940)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee16 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee19 mov rdi,rbx
            rdi = rbx;
            // dee1c mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee1f call QWORD PTR [rax+0x18e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee25 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee28 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee2a mov rdi,rbx
            rdi = rbx;
            // dee2d mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // dee30 call QWORD PTR [rax+0x4b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x4b0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee36 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee39 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee3b mov rdi,rbx
            rdi = rbx;
            // dee3e mov rsi,r14
            rsi = r14;
            // dee41 call QWORD PTR [rax+0x19c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x19c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee47 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // dee49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dee4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dee4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dee4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dee50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dee51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x60955 size=31
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00060955<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x60955;
    loop {
        match pc {
        0x60955 => {
            // block 60955
            // 60955 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 60956 mov rbp,rsp
            rbp = rsp;
            // 60959 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6095a push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6095b mov rbx,rdi
            rbx = rdi;
            // 6095e mov DWORD PTR [rbp-0xc],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rsi as u32 as u64)) as u32);
            // 60961 lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 60965 call 0xebbb8
            { let r = { let t = m.env.stub_va(0x385); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6096a mov rax,rbx
            rax = rbx;
            // 6096d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 60971 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 60972 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 60973 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bb44 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bb44;
    loop {
        match pc {
        0x2bb44 => {
            // block 2bb44
            // 2bb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bb45 mov rbp,rsp
            rbp = rsp;
            // 2bb48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bb4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bb4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bb4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bb4f mov r12,rsi
            r12 = rsi;
            // 2bb52 mov r14,rdi
            r14 = rdi;
            // 2bb55 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 2bb5a mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2bb5f mov rdi,r12
            rdi = r12;
            // 2bb62 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb67 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 2bb70 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2bb74 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 2bb7d mov r15,QWORD PTR [r14+0x50]
            r15 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bb81 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2bb84 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb86;
        }
        0x2bb86 => {
            // block 2bb86
            // 2bb86 cmp DWORD PTR [r14+0x58],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x58)) as u64, 0x0u64).1;
            // 2bb8b je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb8d;
        }
        0x2bb8d => {
            // block 2bb8d
            // 2bb8d mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bb91 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bb93 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb95;
        }
        0x2bb95 => {
            // block 2bb95
            // 2bb95 cmp eax,DWORD PTR [r14+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x20)) as u64).1;
            // 2bb99 jbe 0x2bba6
            if fl.be() { pc = 0x2bba6; continue; }
            pc = 0x2bb9b;
        }
        0x2bb9b => {
            // block 2bb9b
            // 2bb9b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb9d;
        }
        0x2bb9d => {
            // block 2bb9d
            // 2bb9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bba0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bba2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bba6 => {
            // block 2bba6
            // 2bba6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bba8;
        }
        0x2bba8 => {
            // block 2bba8
            // 2bba8 mov esi,DWORD PTR [r15+rbx*4]
            rsi = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bbac mov rdi,r12
            rdi = r12;
            // 2bbaf call 0x60955
            { let r = { let t = m.env.code_va(0x60955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbb4 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bbb7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bbb9 cmp ebx,DWORD PTR [r14+0x58]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x58)) as u64).1;
            // 2bbbd jb 0x2bba8
            if fl.b() { pc = 0x2bba8; continue; }
            pc = 0x2bbbf;
        }
        0x2bbbf => {
            // block 2bbbf
            // 2bbbf jmp 0x2bb9d
            pc = 0x2bb9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c9d9 size=85
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c9d9<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c9d9;
    loop {
        match pc {
        0x5c9d9 => {
            // block 5c9d9
            // 5c9d9 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c9da mov rbp,rsp
            rbp = rsp;
            // 5c9dd mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 5c9df cmp edi,esi
            fl = sub::<32>((rdi as u32 as u64), (rsi as u32 as u64)).1;
            // 5c9e1 je 0x5ca2c
            if fl.e() { pc = 0x5ca2c; continue; }
            pc = 0x5c9e3;
        }
        0x5c9e3 => {
            // block 5c9e3
            // 5c9e3 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5c9e5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5c9e8 cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // 5c9eb mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 5c9f0 ja 0x5c9ff
            if fl.a() { pc = 0x5c9ff; continue; }
            pc = 0x5c9f2;
        }
        0x5c9f2 => {
            // block 5c9f2
            // 5c9f2 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5c9f5 lea rcx,[rip+0xa4e94] # 0x101890
            rcx = m.env.data_va(90205);
            // 5c9fc mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c9ff;
        }
        0x5c9ff => {
            // block 5c9ff
            // 5c9ff dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 5ca01 cmp esi,0x11
            fl = sub::<32>((rsi as u32 as u64), 0x11u64).1;
            // 5ca04 ja 0x5ca14
            if fl.a() { pc = 0x5ca14; continue; }
            pc = 0x5ca06;
        }
        0x5ca06 => {
            // block 5ca06
            // 5ca06 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 5ca09 lea rdx,[rip+0xa4e80] # 0x101890
            rdx = m.env.data_va(90205);
            // 5ca10 mov r8d,DWORD PTR [rdx+rax*4]
            r8 = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ca14;
        }
        0x5ca14 => {
            // block 5ca14
            // 5ca14 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5ca16 cmp ecx,r8d
            fl = sub::<32>((rcx as u32 as u64), (r8 as u32 as u64)).1;
            // 5ca19 jae 0x5ca22
            if fl.ae() { pc = 0x5ca22; continue; }
            pc = 0x5ca1b;
        }
        0x5ca1b => {
            // block 5ca1b
            // 5ca1b mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // 5ca1e div ecx
            { let (q, r) = div::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // 5ca20 jmp 0x5ca27
            pc = 0x5ca27; continue;
        }
        0x5ca22 => {
            // block 5ca22
            // 5ca22 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // 5ca24 div r8d
            { let (q, r) = div::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            pc = 0x5ca27;
        }
        0x5ca27 => {
            // block 5ca27
            // 5ca27 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 5ca29 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x5ca2c;
        }
        0x5ca2c => {
            // block 5ca2c
            // 5ca2c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ca2d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5ca2e size=72
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005ca2e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5ca2e;
    loop {
        match pc {
        0x5ca2e => {
            // block 5ca2e
            // 5ca2e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5ca2f mov rbp,rsp
            rbp = rsp;
            // 5ca32 mov ecx,edi
            rcx = ((rdi as u32 as u64)) as u32 as u64;
            // 5ca34 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ca36 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 5ca38 je 0x5ca74
            if fl.e() { pc = 0x5ca74; continue; }
            pc = 0x5ca3a;
        }
        0x5ca3a => {
            // block 5ca3a
            // 5ca3a test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 5ca3c je 0x5ca74
            if fl.e() { pc = 0x5ca74; continue; }
            pc = 0x5ca3e;
        }
        0x5ca3e => {
            // block 5ca3e
            // 5ca3e dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5ca40 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ca42 cmp ecx,0x78
            fl = sub::<32>((rcx as u32 as u64), 0x78u64).1;
            // 5ca45 mov edi,0x0
            rdi = (0x0u64) as u32 as u64;
            // 5ca4a ja 0x5ca59
            if fl.a() { pc = 0x5ca59; continue; }
            pc = 0x5ca4c;
        }
        0x5ca4c => {
            // block 5ca4c
            // 5ca4c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // 5ca4f lea rdx,[rip+0xa42fa] # 0x100d50
            rdx = m.env.data_va(87325);
            // 5ca56 mov edi,DWORD PTR [rdx+rcx*4]
            rdi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ca59;
        }
        0x5ca59 => {
            // block 5ca59
            // 5ca59 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 5ca5b cmp esi,0x78
            fl = sub::<32>((rsi as u32 as u64), 0x78u64).1;
            // 5ca5e ja 0x5ca6d
            if fl.a() { pc = 0x5ca6d; continue; }
            pc = 0x5ca60;
        }
        0x5ca60 => {
            // block 5ca60
            // 5ca60 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 5ca63 lea rcx,[rip+0xa42e6] # 0x100d50
            rcx = m.env.data_va(87325);
            // 5ca6a mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5ca6d;
        }
        0x5ca6d => {
            // block 5ca6d
            // 5ca6d mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 5ca6f call 0x5c9d9
            { let r = { let t = m.env.code_va(0x5c9d9); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5ca74;
        }
        0x5ca74 => {
            // block 5ca74
            // 5ca74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ca75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x198fe size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000198fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x198fe;
    loop {
        match pc {
        0x198fe => {
            // block 198fe
            // 198fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 198ff mov rbp,rsp
            rbp = rsp;
            // 19902 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19904 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19906 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19907 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19908 mov r15,rdi
            r15 = rdi;
            // 1990b mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // 1990e test r14,r14
            fl = logic::<64>(r14 & r14);
            // 19911 je 0x19984
            if fl.e() { pc = 0x19984; continue; }
            pc = 0x19913;
        }
        0x19913 => {
            // block 19913
            // 19913 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19917 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1991a je 0x19972
            if fl.e() { pc = 0x19972; continue; }
            pc = 0x1991c;
        }
        0x1991c => {
            // block 1991c
            // 1991c lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 19923 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            // 19927 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1992b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19930 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 19934 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19939 lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 1993d call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19942 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 19946 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1994b lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 19952 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19957 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1995e mov rdi,rbx
            rdi = rbx;
            // 19961 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 19966 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1996a cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1996d jne 0x1991c
            if fl.ne() { pc = 0x1991c; continue; }
            pc = 0x1996f;
        }
        0x1996f => {
            // block 1996f
            // 1996f mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            pc = 0x19972;
        }
        0x19972 => {
            // block 19972
            // 19972 mov rdi,r14
            rdi = r14;
            // 19975 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19979 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1997a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1997c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1997e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1997f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x19984 => {
            // block 19984
            // 19984 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19988 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19989 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1998b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1998d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1998e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x58ea5 size=162
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00058ea5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x58ea5;
    loop {
        match pc {
        0x58ea5 => {
            // block 58ea5
            // 58ea5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 58ea6 mov rbp,rsp
            rbp = rsp;
            // 58ea9 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58eab push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58ead push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58eaf push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58eb1 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58eb2 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 58eb6 mov r15,r8
            r15 = r8;
            // 58eb9 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 58ebb mov rax,rdi
            rax = rdi;
            // 58ebe mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 58ec2 shr esi,1
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58ec4 mov DWORD PTR [rbp-0x2c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u32 as u64)) as u32);
            // 58ec7 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 58ec9 shr edi,1
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x1u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 58ecb mov DWORD PTR [rbp-0x3c],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rdi as u32 as u64)) as u32);
            // 58ece cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // 58ed1 je 0x58ee1
            if fl.e() { pc = 0x58ee1; continue; }
            pc = 0x58ed3;
        }
        0x58ed3 => {
            // block 58ed3
            // 58ed3 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 58ed6 jne 0x58eed
            if fl.ne() { pc = 0x58eed; continue; }
            pc = 0x58ed8;
        }
        0x58ed8 => {
            // block 58ed8
            // 58ed8 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // 58edb imul r13d,ebx
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rbx as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // 58edf jmp 0x58efb
            pc = 0x58efb; continue;
        }
        0x58ee1 => {
            // block 58ee1
            // 58ee1 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // 58ee4 imul r13d,ebx
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rbx as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // 58ee8 add r13d,edi
            { let (r, f) = add::<32>((r13 as u32 as u64), (rdi as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58eeb jmp 0x58ef8
            pc = 0x58ef8; continue;
        }
        0x58eed => {
            // block 58eed
            // 58eed xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58ef0 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // 58ef3 jne 0x58efb
            if fl.ne() { pc = 0x58efb; continue; }
            pc = 0x58ef5;
        }
        0x58ef5 => {
            // block 58ef5
            // 58ef5 mov r13d,edi
            r13 = ((rdi as u32 as u64)) as u32 as u64;
            pc = 0x58ef8;
        }
        0x58ef8 => {
            // block 58ef8
            // 58ef8 sub r13d,r9d
            { let (r, f) = sub::<32>((r13 as u32 as u64), (r9 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58efb;
        }
        0x58efb => {
            // block 58efb
            // 58efb test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 58efd je 0x58f38
            if fl.e() { pc = 0x58f38; continue; }
            pc = 0x58eff;
        }
        0x58eff => {
            // block 58eff
            // 58eff mov ecx,edi
            rcx = ((rdi as u32 as u64)) as u32 as u64;
            // 58f01 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 58f05 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 58f08 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58f0b;
        }
        0x58f0b => {
            // block 58f0b
            // 58f0b mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // 58f0e add rdi,r15
            { let (r, f) = add::<64>(rdi, r15);
              rdi = r;
              fl = f; }
            // 58f11 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 58f14 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 58f17 mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 58f1b call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58f20 mov ecx,DWORD PTR [rbp-0x3c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            // 58f23 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 58f26 add r12d,ecx
            { let (r, f) = add::<32>((r12 as u32 as u64), (rcx as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // 58f29 add r13d,ebx
            { let (r, f) = add::<32>((r13 as u32 as u64), (rbx as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58f2c inc r14d
            { let (r, f) = inc::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // 58f2f cmp r14d,eax
            fl = sub::<32>((r14 as u32 as u64), (rax as u32 as u64)).1;
            // 58f32 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 58f36 jb 0x58f0b
            if fl.b() { pc = 0x58f0b; continue; }
            pc = 0x58f38;
        }
        0x58f38 => {
            // block 58f38
            // 58f38 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 58f3c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58f3d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58f3f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58f41 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58f43 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58f45 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58f46 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x159f8 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000159f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x159f8;
    loop {
        match pc {
        0x159f8 => {
            // block 159f8
            // 159f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 159f9 mov rbp,rsp
            rbp = rsp;
            // 159fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 159fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15a00 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15a02 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15a04 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15a05 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15a06 mov r13,rsi
            r13 = rsi;
            // 15a09 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 15a0d lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // 15a11 mov rcx,r14
            rcx = r14;
            // 15a14 call 0xeb924
            { let r = { let t = m.env.stub_va(0xf1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15a19 mov r15,rax
            r15 = rax;
            // 15a1c cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // 15a1f je 0x15a8b
            if fl.e() { pc = 0x15a8b; continue; }
            pc = 0x15a21;
        }
        0x15a21 => {
            // block 15a21
            // 15a21 lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // 15a25 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // 15a2a mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 15a2d and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15a2f jne 0x15a36
            if fl.ne() { pc = 0x15a36; continue; }
            pc = 0x15a31;
        }
        0x15a31 => {
            // block 15a31
            // 15a31 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // 15a34 jmp 0x15a3a
            pc = 0x15a3a; continue;
        }
        0x15a36 => {
            // block 15a36
            // 15a36 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x15a3a;
        }
        0x15a3a => {
            // block 15a3a
            // 15a3a movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // 15a3d mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 15a3f and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15a42 jne 0x15a49
            if fl.ne() { pc = 0x15a49; continue; }
            pc = 0x15a44;
        }
        0x15a44 => {
            // block 15a44
            // 15a44 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 15a47 jmp 0x15a4d
            pc = 0x15a4d; continue;
        }
        0x15a49 => {
            // block 15a49
            // 15a49 mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0x15a4d;
        }
        0x15a4d => {
            // block 15a4d
            // 15a4d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 15a4f jne 0x15a56
            if fl.ne() { pc = 0x15a56; continue; }
            pc = 0x15a51;
        }
        0x15a51 => {
            // block 15a51
            // 15a51 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 15a54 jmp 0x15a5a
            pc = 0x15a5a; continue;
        }
        0x15a56 => {
            // block 15a56
            // 15a56 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0x15a5a;
        }
        0x15a5a => {
            // block 15a5a
            // 15a5a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 15a5c jne 0x15a63
            if fl.ne() { pc = 0x15a63; continue; }
            pc = 0x15a5e;
        }
        0x15a5e => {
            // block 15a5e
            // 15a5e inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 15a61 jmp 0x15a67
            pc = 0x15a67; continue;
        }
        0x15a63 => {
            // block 15a63
            // 15a63 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0x15a67;
        }
        0x15a67 => {
            // block 15a67
            // 15a67 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 15a6a mov rdx,r12
            rdx = r12;
            // 15a6d cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 15a71 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 15a74 je 0x15a86
            if fl.e() { pc = 0x15a86; continue; }
            pc = 0x15a76;
        }
        0x15a76 => {
            // block 15a76
            // 15a76 mov rdi,r13
            rdi = r13;
            // 15a79 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15a7e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15a80 je 0x15a86
            if fl.e() { pc = 0x15a86; continue; }
            pc = 0x15a82;
        }
        0x15a82 => {
            // block 15a82
            // 15a82 jns 0x15a8e
            if fl.ns() { pc = 0x15a8e; continue; }
            pc = 0x15a84;
        }
        0x15a84 => {
            // block 15a84
            // 15a84 jmp 0x15a8b
            pc = 0x15a8b; continue;
        }
        0x15a86 => {
            // block 15a86
            // 15a86 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 15a89 jae 0x15a8e
            if fl.ae() { pc = 0x15a8e; continue; }
            pc = 0x15a8b;
        }
        0x15a8b => {
            // block 15a8b
            // 15a8b mov r15,r14
            r15 = r14;
            pc = 0x15a8e;
        }
        0x15a8e => {
            // block 15a8e
            // 15a8e mov rax,r15
            rax = r15;
            // 15a91 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15a95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15a96 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15a98 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15a9a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15a9c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15a9e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15a9f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5d66 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5d66<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5d66;
    loop {
        match pc {
        0xb5d66 => {
            // block b5d66
            // b5d66 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5d67 mov rbp,rsp
            rbp = rsp;
            // b5d6a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5d6c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5d6e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5d70 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5d72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5d73 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b5d74 mov r13,rsi
            r13 = rsi;
            // b5d77 mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5d7b lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // b5d7f mov rcx,r14
            rcx = r14;
            // b5d82 call 0xebc9c
            { let r = { let t = m.env.stub_va(0x469); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5d87 mov r15,rax
            r15 = rax;
            // b5d8a cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // b5d8d je 0xb5df9
            if fl.e() { pc = 0xb5df9; continue; }
            pc = 0xb5d8f;
        }
        0xb5d8f => {
            // block b5d8f
            // b5d8f lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // b5d93 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // b5d98 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // b5d9b and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5d9d jne 0xb5da4
            if fl.ne() { pc = 0xb5da4; continue; }
            pc = 0xb5d9f;
        }
        0xb5d9f => {
            // block b5d9f
            // b5d9f shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // b5da2 jmp 0xb5da8
            pc = 0xb5da8; continue;
        }
        0xb5da4 => {
            // block b5da4
            // b5da4 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xb5da8;
        }
        0xb5da8 => {
            // block b5da8
            // b5da8 movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // b5dab mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // b5dad and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5db0 jne 0xb5db7
            if fl.ne() { pc = 0xb5db7; continue; }
            pc = 0xb5db2;
        }
        0xb5db2 => {
            // block b5db2
            // b5db2 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // b5db5 jmp 0xb5dbb
            pc = 0xb5dbb; continue;
        }
        0xb5db7 => {
            // block b5db7
            // b5db7 mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xb5dbb;
        }
        0xb5dbb => {
            // block b5dbb
            // b5dbb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5dbd jne 0xb5dc4
            if fl.ne() { pc = 0xb5dc4; continue; }
            pc = 0xb5dbf;
        }
        0xb5dbf => {
            // block b5dbf
            // b5dbf inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // b5dc2 jmp 0xb5dc8
            pc = 0xb5dc8; continue;
        }
        0xb5dc4 => {
            // block b5dc4
            // b5dc4 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xb5dc8;
        }
        0xb5dc8 => {
            // block b5dc8
            // b5dc8 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // b5dca jne 0xb5dd1
            if fl.ne() { pc = 0xb5dd1; continue; }
            pc = 0xb5dcc;
        }
        0xb5dcc => {
            // block b5dcc
            // b5dcc inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b5dcf jmp 0xb5dd5
            pc = 0xb5dd5; continue;
        }
        0xb5dd1 => {
            // block b5dd1
            // b5dd1 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xb5dd5;
        }
        0xb5dd5 => {
            // block b5dd5
            // b5dd5 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // b5dd8 mov rdx,r12
            rdx = r12;
            // b5ddb cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // b5ddf test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5de2 je 0xb5df4
            if fl.e() { pc = 0xb5df4; continue; }
            pc = 0xb5de4;
        }
        0xb5de4 => {
            // block b5de4
            // b5de4 mov rdi,r13
            rdi = r13;
            // b5de7 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5dec test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5dee je 0xb5df4
            if fl.e() { pc = 0xb5df4; continue; }
            pc = 0xb5df0;
        }
        0xb5df0 => {
            // block b5df0
            // b5df0 jns 0xb5dfc
            if fl.ns() { pc = 0xb5dfc; continue; }
            pc = 0xb5df2;
        }
        0xb5df2 => {
            // block b5df2
            // b5df2 jmp 0xb5df9
            pc = 0xb5df9; continue;
        }
        0xb5df4 => {
            // block b5df4
            // b5df4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // b5df7 jae 0xb5dfc
            if fl.ae() { pc = 0xb5dfc; continue; }
            pc = 0xb5df9;
        }
        0xb5df9 => {
            // block b5df9
            // b5df9 mov r15,r14
            r15 = r14;
            pc = 0xb5dfc;
        }
        0xb5dfc => {
            // block b5dfc
            // b5dfc mov rax,r15
            rax = r15;
            // b5dff add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b5e03 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5e04 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5e06 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5e08 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5e0a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5e0c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5e0d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc486a size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c486a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc486a;
    loop {
        match pc {
        0xc486a => {
            // block c486a
            // c486a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c486b mov rbp,rsp
            rbp = rsp;
            // c486e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4870 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c4872 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c4874 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c4876 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4877 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4878 mov r13,rsi
            r13 = rsi;
            // c487b mov rdx,QWORD PTR [rdi+0x8]
            rdx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c487f lea r14,[rdi+0x8]
            r14 = rdi.wrapping_add(0x8);
            // c4883 mov rcx,r14
            rcx = r14;
            // c4886 call 0xebe5e
            { let r = { let t = m.env.stub_va(0x62b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c488b mov r15,rax
            r15 = rax;
            // c488e cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // c4891 je 0xc48fd
            if fl.e() { pc = 0xc48fd; continue; }
            pc = 0xc4893;
        }
        0xc4893 => {
            // block c4893
            // c4893 lea rsi,[r15+0x20]
            rsi = r15.wrapping_add(0x20);
            // c4897 movzx r12d,BYTE PTR [r13+0x0]
            r12 = (m.r8(r13) as u64) as u32 as u64;
            // c489c mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // c489f and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c48a1 jne 0xc48a8
            if fl.ne() { pc = 0xc48a8; continue; }
            pc = 0xc48a3;
        }
        0xc48a3 => {
            // block c48a3
            // c48a3 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // c48a6 jmp 0xc48ac
            pc = 0xc48ac; continue;
        }
        0xc48a8 => {
            // block c48a8
            // c48a8 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0xc48ac;
        }
        0xc48ac => {
            // block c48ac
            // c48ac movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // c48af mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // c48b1 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c48b4 jne 0xc48bb
            if fl.ne() { pc = 0xc48bb; continue; }
            pc = 0xc48b6;
        }
        0xc48b6 => {
            // block c48b6
            // c48b6 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // c48b9 jmp 0xc48bf
            pc = 0xc48bf; continue;
        }
        0xc48bb => {
            // block c48bb
            // c48bb mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0xc48bf;
        }
        0xc48bf => {
            // block c48bf
            // c48bf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c48c1 jne 0xc48c8
            if fl.ne() { pc = 0xc48c8; continue; }
            pc = 0xc48c3;
        }
        0xc48c3 => {
            // block c48c3
            // c48c3 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // c48c6 jmp 0xc48cc
            pc = 0xc48cc; continue;
        }
        0xc48c8 => {
            // block c48c8
            // c48c8 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0xc48cc;
        }
        0xc48cc => {
            // block c48cc
            // c48cc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // c48ce jne 0xc48d5
            if fl.ne() { pc = 0xc48d5; continue; }
            pc = 0xc48d0;
        }
        0xc48d0 => {
            // block c48d0
            // c48d0 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // c48d3 jmp 0xc48d9
            pc = 0xc48d9; continue;
        }
        0xc48d5 => {
            // block c48d5
            // c48d5 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0xc48d9;
        }
        0xc48d9 => {
            // block c48d9
            // c48d9 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // c48dc mov rdx,r12
            rdx = r12;
            // c48df cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c48e3 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c48e6 je 0xc48f8
            if fl.e() { pc = 0xc48f8; continue; }
            pc = 0xc48e8;
        }
        0xc48e8 => {
            // block c48e8
            // c48e8 mov rdi,r13
            rdi = r13;
            // c48eb call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c48f0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c48f2 je 0xc48f8
            if fl.e() { pc = 0xc48f8; continue; }
            pc = 0xc48f4;
        }
        0xc48f4 => {
            // block c48f4
            // c48f4 jns 0xc4900
            if fl.ns() { pc = 0xc4900; continue; }
            pc = 0xc48f6;
        }
        0xc48f6 => {
            // block c48f6
            // c48f6 jmp 0xc48fd
            pc = 0xc48fd; continue;
        }
        0xc48f8 => {
            // block c48f8
            // c48f8 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // c48fb jae 0xc4900
            if fl.ae() { pc = 0xc4900; continue; }
            pc = 0xc48fd;
        }
        0xc48fd => {
            // block c48fd
            // c48fd mov r15,r14
            r15 = r14;
            pc = 0xc4900;
        }
        0xc4900 => {
            // block c4900
            // c4900 mov rax,r15
            rax = r15;
            // c4903 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4907 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4908 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c490a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c490c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c490e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c4910 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4911 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
