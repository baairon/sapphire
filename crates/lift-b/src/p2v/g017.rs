//! Mechanically lifted functions (scripts/lift_run.py), module g017. Each one is checked against the original
//! in the world harness (tests/world_g017.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xbc13c size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc13c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc13c;
    loop {
        match pc {
        0xbc13c => {
            // block bc13c
            // bc13c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc13d mov rbp,rsp
            rbp = rsp;
            // bc140 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc142 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc144 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc146 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc147 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc14b mov r15,rsi
            r15 = rsi;
            // bc14e mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc151 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc155 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc159 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc15d mov rdi,r15
            rdi = r15;
            // bc160 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc165 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc16d mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc170 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc178 lea rdi,[rip+0x10c521] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // bc17f lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc183 call 0xebdc8
            { let r = { let t = m.env.stub_va(0x595); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc188 mov rbx,rax
            rbx = rax;
            // bc18b lea r12,[rip+0x10c516] # 0x1c86a8
            r12 = m.env.data_va(904821);
            // bc192 jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc194 => {
            // block bc194
            // bc194 cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc198 jne 0xbc1d0
            if fl.ne() { pc = 0xbc1d0; continue; }
            pc = 0xbc19a;
        }
        0xbc19a => {
            // block bc19a
            // bc19a lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc19e mov rdi,r15
            rdi = r15;
            // bc1a1 call 0xebd98
            { let r = { let t = m.env.stub_va(0x565); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc1a6 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc1aa test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1ad je 0xbc1bc
            if fl.e() { pc = 0xbc1bc; continue; }
            pc = 0xbc1af;
        }
        0xbc1af => {
            // block bc1af
            // bc1af mov rbx,rax
            rbx = rax;
            // bc1b2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc1b5 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc1b8 jne 0xbc1af
            if fl.ne() { pc = 0xbc1af; continue; }
            pc = 0xbc1ba;
        }
        0xbc1ba => {
            // block bc1ba
            // bc1ba jmp 0xbc1cb
            pc = 0xbc1cb; continue;
        }
        0xbc1bc => {
            // block bc1bc
            // bc1bc mov rax,rbx
            rax = rbx;
            pc = 0xbc1bf;
        }
        0xbc1bf => {
            // block bc1bf
            // bc1bf mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc1c3 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc1c6 mov rax,rbx
            rax = rbx;
            // bc1c9 jne 0xbc1bf
            if fl.ne() { pc = 0xbc1bf; continue; }
            pc = 0xbc1cb;
        }
        0xbc1cb => {
            // block bc1cb
            // bc1cb cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc1ce jne 0xbc194
            if fl.ne() { pc = 0xbc194; continue; }
            pc = 0xbc1d0;
        }
        0xbc1d0 => {
            // block bc1d0
            // bc1d0 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc1d5 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc1d8 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc1dc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc1dd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc1df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc1e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc1e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc1e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc4d4 size=170
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc4d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc4d4;
    loop {
        match pc {
        0xbc4d4 => {
            // block bc4d4
            // bc4d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc4d5 mov rbp,rsp
            rbp = rsp;
            // bc4d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc4da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc4dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc4de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc4df sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc4e3 mov r15,rsi
            r15 = rsi;
            // bc4e6 mov r14d,edi
            r14 = ((rdi as u32 as u64)) as u32 as u64;
            // bc4e9 mov DWORD PTR [rbp-0x24],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((r14 as u32 as u64)) as u32);
            // bc4ed lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // bc4f1 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // bc4f5 mov rdi,r15
            rdi = r15;
            // bc4f8 call 0xebdb6
            { let r = { let t = m.env.stub_va(0x583); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc4fd mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // bc505 mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // bc508 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // bc510 lea rdi,[rip+0x10c171] # 0x1c8688
            rdi = m.env.data_va(904789);
            // bc517 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bc51b call 0xebde0
            { let r = { let t = m.env.stub_va(0x5ad); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc520 mov rbx,rax
            rbx = rax;
            // bc523 lea r12,[rip+0x10c166] # 0x1c8690
            r12 = m.env.data_va(904797);
            // bc52a jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc52c => {
            // block bc52c
            // bc52c cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc530 jne 0xbc568
            if fl.ne() { pc = 0xbc568; continue; }
            pc = 0xbc532;
        }
        0xbc532 => {
            // block bc532
            // bc532 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc536 mov rdi,r15
            rdi = r15;
            // bc539 call 0xebdaa
            { let r = { let t = m.env.stub_va(0x577); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc53e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc542 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc545 je 0xbc554
            if fl.e() { pc = 0xbc554; continue; }
            pc = 0xbc547;
        }
        0xbc547 => {
            // block bc547
            // bc547 mov rbx,rax
            rbx = rax;
            // bc54a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc54d test rax,rax
            fl = logic::<64>(rax & rax);
            // bc550 jne 0xbc547
            if fl.ne() { pc = 0xbc547; continue; }
            pc = 0xbc552;
        }
        0xbc552 => {
            // block bc552
            // bc552 jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc554 => {
            // block bc554
            // bc554 mov rax,rbx
            rax = rbx;
            pc = 0xbc557;
        }
        0xbc557 => {
            // block bc557
            // bc557 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc55b cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // bc55e mov rax,rbx
            rax = rbx;
            // bc561 jne 0xbc557
            if fl.ne() { pc = 0xbc557; continue; }
            pc = 0xbc563;
        }
        0xbc563 => {
            // block bc563
            // bc563 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc566 jne 0xbc52c
            if fl.ne() { pc = 0xbc52c; continue; }
            pc = 0xbc568;
        }
        0xbc568 => {
            // block bc568
            // bc568 cmp QWORD PTR [r15+0x10],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc56d setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc570 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc574 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc575 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc577 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc579 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc57b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc57c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x61612 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00061612<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x61612;
    loop {
        match pc {
        0x61612 => {
            // block 61612
            // 61612 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 61613 mov rbp,rsp
            rbp = rsp;
            // 61616 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 61617 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 61618 mov rbx,rdi
            rbx = rdi;
            // 6161b cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 6161e je 0x61645
            if fl.e() { pc = 0x61645; continue; }
            pc = 0x61620;
        }
        0x61620 => {
            // block 61620
            // 61620 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 61623 jne 0x61665
            if fl.ne() { pc = 0x61665; continue; }
            pc = 0x61625;
        }
        0x61625 => {
            // block 61625
            // 61625 lea rax,[rip+0xcdfd3] # 0x12f5ff
            rax = m.env.data_va(277964);
            // 6162c lea rsi,[rip+0xcdfd1] # 0x12f604
            rsi = m.env.data_va(277969);
            // 61633 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 61635 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 61639 mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // 6163e mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 61643 jmp 0x61687
            pc = 0x61687; continue;
        }
        0x61645 => {
            // block 61645
            // 61645 lea rax,[rip+0xcdfcb] # 0x12f617
            rax = m.env.data_va(277988);
            // 6164c lea rsi,[rip+0xcdfcb] # 0x12f61e
            rsi = m.env.data_va(277995);
            // 61653 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 61655 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 61659 mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 6165e mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 61663 jmp 0x61687
            pc = 0x61687; continue;
        }
        0x61665 => {
            // block 61665
            // 61665 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 61667 jne 0x6168d
            if fl.ne() { pc = 0x6168d; continue; }
            pc = 0x61669;
        }
        0x61669 => {
            // block 61669
            // 61669 lea rax,[rip+0xcdf79] # 0x12f5e9
            rax = m.env.data_va(277942);
            // 61670 lea rsi,[rip+0xcdf76] # 0x12f5ed
            rsi = m.env.data_va(277946);
            // 61677 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 61679 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 6167d mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 61682 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            pc = 0x61687;
        }
        0x61687 => {
            // block 61687
            // 61687 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 6168b jmp 0x616ac
            pc = 0x616ac; continue;
        }
        0x6168d => {
            // block 6168d
            // 6168d lea rax,[rip+0xd7634] # 0x138cc8
            rax = m.env.data_va(316565);
            // 61694 lea rsi,[rip+0xcdf98] # 0x12f633
            rsi = m.env.data_va(278016);
            // 6169b xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 6169d test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 6169f cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 616a3 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // 616a8 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            pc = 0x616ac;
        }
        0x616ac => {
            // block 616ac
            // 616ac mov rdi,rbx
            rdi = rbx;
            // 616af call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 616b4 mov rax,rbx
            rax = rbx;
            // 616b7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 616bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 616bc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 616bd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd35ce size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d35ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd35ce;
    loop {
        match pc {
        0xd35ce => {
            // block d35ce
            // d35ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d35cf mov rbp,rsp
            rbp = rsp;
            // d35d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d35d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d35d6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d35d8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d35d9 sub rsp,0x1a0
            { let (r, f) = sub::<64>(rsp, 0x1a0u64);
              rsp = r;
              fl = f; }
            // d35e0 mov rbx,rsi
            rbx = rsi;
            // d35e3 mov r15,rdi
            r15 = rdi;
            // d35e6 mov r12,QWORD PTR [rip+0x6cdeb] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // d35ed mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // d35f1 mov QWORD PTR [rbp-0x28],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (r12) as u64);
            // d35f5 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d35f8 je 0xd3662
            if fl.e() { pc = 0xd3662; continue; }
            pc = 0xd35fa;
        }
        0xd35fa => {
            // block d35fa
            // d35fa xorps xmm0,xmm0
            x0 = 0;
            // d35fd movaps XMMWORD PTR [rbp-0x1c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe40), x0);
            // d3604 mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // d360f movabs rax,0x19400000001
            rax = 0x0_0194_0000_0001u64;
            // d3619 mov QWORD PTR [rbp-0x1c0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rax) as u64);
            // d3620 lea r14,[rbp-0x1a8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe58);
            // d3627 mov edx,0x17c
            rdx = (0x17cu64) as u32 as u64;
            // d362c mov rdi,r14
            rdi = r14;
            // d362f mov rsi,rbx
            rsi = rbx;
            // d3632 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3637 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d363a lea rsi,[rbp-0x1c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe40);
            // d3641 mov rdi,r15
            rdi = r15;
            // d3644 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d364a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d364c je 0xd3662
            if fl.e() { pc = 0xd3662; continue; }
            pc = 0xd364e;
        }
        0xd364e => {
            // block d364e
            // d364e mov edx,0x17c
            rdx = (0x17cu64) as u32 as u64;
            // d3653 mov rdi,rbx
            rdi = rbx;
            // d3656 mov rsi,r14
            rsi = r14;
            // d3659 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d365e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3660 jmp 0xd3664
            pc = 0xd3664; continue;
        }
        0xd3662 => {
            // block d3662
            // d3662 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3664;
        }
        0xd3664 => {
            // block d3664
            // d3664 cmp r12,QWORD PTR [rbp-0x28]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64).1;
            // d3668 jne 0xd367a
            if fl.ne() { pc = 0xd367a; continue; }
            pc = 0xd366a;
        }
        0xd366a => {
            // block d366a
            // d366a add rsp,0x1a0
            { let (r, f) = add::<64>(rsp, 0x1a0u64);
              rsp = r;
              fl = f; }
            // d3671 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3672 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3674 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3676 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3678 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3679 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd367a => {
            // block d367a
            // d367a call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe08f8 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e08f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe08f8;
    loop {
        match pc {
        0xe08f8 => {
            // block e08f8
            // e08f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e08f9 mov rbp,rsp
            rbp = rsp;
            // e08fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e08fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0900 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0902 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0904 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0905 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0906 mov r14,rsi
            r14 = rsi;
            // e0909 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e090b test r14,r14
            fl = logic::<64>(r14 & r14);
            // e090e je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe0914;
        }
        0xe0914 => {
            // block e0914
            // e0914 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0917 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e091a mov r15,QWORD PTR [rax]
            r15 = m.r64(rax) as u64;
            // e091d mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // e0921 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0923 mov rax,rcx
            rax = rcx;
            // e0926 sub rax,r15
            { let (r, f) = sub::<64>(rax, r15);
              rax = r;
              fl = f; }
            // e0929 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe092b;
        }
        0xe092b => {
            // block e092b
            // e092b mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // e092f sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // e0933 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // e093d imul r13,rax
            { let (r, f) = imul::<64>(r13, rax); r13 = r; fl = f; }
            // e0941 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e0944 mov rbx,r15
            rbx = r15;
            pc = 0xe0947;
        }
        0xe0947 => {
            // block e0947
            // e0947 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e094b mov rsi,r14
            rsi = r14;
            // e094e call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0955 je 0xe096d
            if fl.e() { pc = 0xe096d; continue; }
            pc = 0xe0957;
        }
        0xe0957 => {
            // block e0957
            // e0957 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e095a add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e095e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // e0961 jb 0xe0947
            if fl.b() { pc = 0xe0947; continue; }
            pc = 0xe0963;
        }
        0xe0963 => {
            // block e0963
            // e0963 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0965 cmp QWORD PTR [rbp-0x30],r15
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r15).1;
            // e0969 jne 0xe097b
            if fl.ne() { pc = 0xe097b; continue; }
            pc = 0xe096b;
        }
        0xe096b => {
            // block e096b
            // e096b jmp 0xe0997
            pc = 0xe0997; continue;
        }
        0xe096d => {
            // block e096d
            // e096d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e0970 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e0974 jne 0xe0997
            if fl.ne() { pc = 0xe0997; continue; }
            pc = 0xe0976;
        }
        0xe0976 => {
            // block e0976
            // e0976 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e0979 je 0xe0997
            if fl.e() { pc = 0xe0997; continue; }
            pc = 0xe097b;
        }
        0xe097b => {
            // block e097b
            // e097b lea rsi,[rip+0x4a1fe] # 0x12ab80
            rsi = m.env.data_va(258893);
            // e0982 mov rdi,r14
            rdi = r14;
            // e0985 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e098a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e098c test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e098e cmove rcx,r15
            if fl.e() { rcx = r15; }
            // e0992 mov rax,rcx
            rax = rcx;
            // e0995 jmp 0xe099a
            pc = 0xe099a; continue;
        }
        0xe0997 => {
            // block e0997
            // e0997 mov rax,rbx
            rax = rbx;
            pc = 0xe099a;
        }
        0xe099a => {
            // block e099a
            // e099a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e099e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e099f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e09a1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e09a3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e09a5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e09a7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e09a8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb905b size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b905b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb905b;
    loop {
        match pc {
        0xb905b => {
            // block b905b
            // b905b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b905c mov rbp,rsp
            rbp = rsp;
            // b905f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9061 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9063 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9065 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9067 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9068 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b906c mov r15,rsi
            r15 = rsi;
            // b906f mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // b9073 mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // b9076 cmp r14,QWORD PTR [r15+0x8]
            fl = sub::<64>(r14, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // b907a je 0xb90ec
            if fl.e() { pc = 0xb90ec; continue; }
            pc = 0xb907c;
        }
        0xb907c => {
            // block b907c
            // b907c lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xb9080;
        }
        0xb9080 => {
            // block b9080
            // b9080 movsd xmm0,QWORD PTR [r14]
            x0 = m.r64(r14) as u128;
            // b9085 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b9089 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b908e mov rbx,rax
            rbx = rax;
            // b9091 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b9094 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b9098 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b909b mov rdi,r13
            rdi = r13;
            // b909e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90a3 mov rdi,r13
            rdi = r13;
            // b90a6 mov rsi,QWORD PTR [rip+0x870e3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b90ad call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90b2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b90b5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b90b9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b90be mov rdi,rax
            rdi = rax;
            // b90c1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90c3 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b90c6 mov rdi,r13
            rdi = r13;
            // b90c9 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90ce movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b90d2 mov rdi,rbx
            rdi = rbx;
            // b90d5 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90da mov rdi,rbx
            rdi = rbx;
            // b90dd call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b90e2 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b90e6 cmp r14,QWORD PTR [r15+0x8]
            fl = sub::<64>(r14, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // b90ea jne 0xb9080
            if fl.ne() { pc = 0xb9080; continue; }
            pc = 0xb90ec;
        }
        0xb90ec => {
            // block b90ec
            // b90ec mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b90f0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b90f4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b90f5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b90f7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b90f9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b90fb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b90fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b90fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb90ff => {
            // block b90ff
            // b90ff mov rbx,rax
            rbx = rax;
            // b9102 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b9106 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b910b mov rdi,rbx
            rdi = rbx;
            // b910e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd31f1 size=190
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d31f1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd31f1;
    loop {
        match pc {
        0xd31f1 => {
            // block d31f1
            // d31f1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d31f2 mov rbp,rsp
            rbp = rsp;
            // d31f5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d31f7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d31f9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d31fa sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d3201 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d3203 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d3206 mov r14,rsi
            r14 = rsi;
            // d3209 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // d320b xorps xmm0,xmm0
            x0 = 0;
            // d320e movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d3212 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d3216 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d321a movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d321e movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3222 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3226 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d322d movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // d3234 movsxd rdi,ebx
            rdi = sx::<32>((rbx as u32 as u64)) as u64;
            // d3237 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d323c mov cl,dil
            rcx = (rcx & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // d323f shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // d3241 mov rcx,rdi
            rcx = rdi;
            // d3244 shr rcx,0x5
            { let (r, f) = shr::<64>(rcx, 0x5u64, fl); rcx = r; fl = f; }
            // d3248 or DWORD PTR [rbp+rcx*4-0xa0],edx
            { let r = m.r32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60)) as u64 | (rdx as u32 as u64); let f = logic::<32>(r);
              m.w32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60), (r) as u32);
              fl = f; }
            // d324f cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // d3251 mov QWORD PTR [rbp-0xb0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (rax) as u64);
            // d3258 mov DWORD PTR [rbp-0xa8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u32);
            // d3262 inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // d3264 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d326b lea r8,[rbp-0xb0]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3272 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3274 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3276 call 0xec284
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d327b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d327d mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3282 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3284 je 0xd32a1
            if fl.e() { pc = 0xd32a1; continue; }
            pc = 0xd3286;
        }
        0xd3286 => {
            // block d3286
            // d3286 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3289 jne 0xd3292
            if fl.ne() { pc = 0xd3292; continue; }
            pc = 0xd328b;
        }
        0xd328b => {
            // block d328b
            // d328b mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d3290 jmp 0xd32a1
            pc = 0xd32a1; continue;
        }
        0xd3292 => {
            // block d3292
            // d3292 movsxd rdx,r15d
            rdx = sx::<32>((r15 as u32 as u64)) as u64;
            // d3295 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3297 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // d3299 mov rsi,r14
            rsi = r14;
            // d329c call 0xec278
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd32a1;
        }
        0xd32a1 => {
            // block d32a1
            // d32a1 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // d32a8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d32a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d32ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d32ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d32ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd32af size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d32af<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd32af;
    loop {
        match pc {
        0xd32af => {
            // block d32af
            // d32af push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d32b0 mov rbp,rsp
            rbp = rsp;
            // d32b3 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d32b5 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d32b7 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d32b9 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d32ba sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // d32c1 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // d32c3 mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // d32c6 xorps xmm0,xmm0
            x0 = 0;
            // d32c9 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d32cd movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d32d1 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d32d5 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d32d9 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d32dd movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d32e1 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d32e8 movaps XMMWORD PTR [rbp-0xa0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff60), x0);
            // d32ef movsxd rdi,r12d
            rdi = sx::<32>((r12 as u32 as u64)) as u64;
            // d32f2 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // d32f7 mov cl,dil
            rcx = (rcx & !0xff) | (((rdi as u8 as u64)) as u8 as u64);
            // d32fa shl ebx,cl
            { let (r, f) = shl::<32>((rbx as u32 as u64), (rcx as u8 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // d32fc mov rcx,rdi
            rcx = rdi;
            // d32ff shr rcx,0x5
            { let (r, f) = shr::<64>(rcx, 0x5u64, fl); rcx = r; fl = f; }
            // d3303 or DWORD PTR [rbp+rcx*4-0xa0],ebx
            { let r = m.r32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60)) as u64 | (rbx as u32 as u64); let f = logic::<32>(r);
              m.w32(rbp.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff60), (r) as u32);
              fl = f; }
            // d330a mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // d330d mov r14,rsi
            r14 = rsi;
            // d3310 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // d331b mov DWORD PTR [rbp-0xa8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff58), ((rax as u32 as u64)) as u32);
            // d3321 inc edi
            { let (r, f) = inc::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // d3323 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d332a lea r8,[rbp-0xb0]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3331 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3333 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3335 call 0xec284
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d333a mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d333c mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3341 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3343 je 0xd3361
            if fl.e() { pc = 0xd3361; continue; }
            pc = 0xd3345;
        }
        0xd3345 => {
            // block d3345
            // d3345 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3348 jne 0xd3351
            if fl.ne() { pc = 0xd3351; continue; }
            pc = 0xd334a;
        }
        0xd334a => {
            // block d334a
            // d334a mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d334f jmp 0xd3361
            pc = 0xd3361; continue;
        }
        0xd3351 => {
            // block d3351
            // d3351 movsxd rdx,r15d
            rdx = sx::<32>((r15 as u32 as u64)) as u64;
            // d3354 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3356 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // d3359 mov rsi,r14
            rsi = r14;
            // d335c call 0xec278
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd3361;
        }
        0xd3361 => {
            // block d3361
            // d3361 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // d3368 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3369 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d336b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d336d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d336f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3370 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x60274 size=195
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00060274<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x60274;
    loop {
        match pc {
        0x60274 => {
            // block 60274
            // 60274 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 60275 mov rbp,rsp
            rbp = rsp;
            // 60278 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 60279 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6027a mov rbx,rdi
            rbx = rdi;
            // 6027d cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 60280 jg 0x602a6
            if fl.g() { pc = 0x602a6; continue; }
            pc = 0x60282;
        }
        0x60282 => {
            // block 60282
            // 60282 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 60284 jne 0x602cb
            if fl.ne() { pc = 0x602cb; continue; }
            pc = 0x60286;
        }
        0x60286 => {
            // block 60286
            // 60286 lea rax,[rip+0xca5c6] # 0x12a853
            rax = m.env.data_va(258080);
            // 6028d lea rsi,[rip+0xced95] # 0x12f029
            rsi = m.env.data_va(276470);
            // 60294 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 60296 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 6029a mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 6029f mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 602a4 jmp 0x60316
            pc = 0x60316; continue;
        }
        0x602a6 => {
            // block 602a6
            // 602a6 cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 602a9 jne 0x602f0
            if fl.ne() { pc = 0x602f0; continue; }
            pc = 0x602ab;
        }
        0x602ab => {
            // block 602ab
            // 602ab lea rax,[rip+0xceda7] # 0x12f059
            rax = m.env.data_va(276518);
            // 602b2 lea rsi,[rip+0xceda4] # 0x12f05d
            rsi = m.env.data_va(276522);
            // 602b9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 602bb cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 602bf mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 602c4 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 602c9 jmp 0x60316
            pc = 0x60316; continue;
        }
        0x602cb => {
            // block 602cb
            // 602cb cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 602ce jne 0x6031c
            if fl.ne() { pc = 0x6031c; continue; }
            pc = 0x602d0;
        }
        0x602d0 => {
            // block 602d0
            // 602d0 lea rax,[rip+0xced65] # 0x12f03c
            rax = m.env.data_va(276489);
            // 602d7 lea rsi,[rip+0xced67] # 0x12f045
            rsi = m.env.data_va(276498);
            // 602de test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 602e0 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 602e4 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 602e9 mov edx,0x13
            rdx = (0x13u64) as u32 as u64;
            // 602ee jmp 0x60316
            pc = 0x60316; continue;
        }
        0x602f0 => {
            // block 602f0
            // 602f0 cmp esi,0xff
            fl = sub::<32>((rsi as u32 as u64), 0xffu64).1;
            // 602f6 jne 0x6031c
            if fl.ne() { pc = 0x6031c; continue; }
            pc = 0x602f8;
        }
        0x602f8 => {
            // block 602f8
            // 602f8 lea rax,[rip+0xced6d] # 0x12f06c
            rax = m.env.data_va(276537);
            // 602ff lea rsi,[rip+0xced69] # 0x12f06f
            rsi = m.env.data_va(276540);
            // 60306 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 60308 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 6030c mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 60311 mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            pc = 0x60316;
        }
        0x60316 => {
            // block 60316
            // 60316 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 6031a jmp 0x60325
            pc = 0x60325; continue;
        }
        0x6031c => {
            // block 6031c
            // 6031c lea rsi,[rip+0xd89a5] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 60323 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x60325;
        }
        0x60325 => {
            // block 60325
            // 60325 mov rdi,rbx
            rdi = rbx;
            // 60328 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6032d mov rax,rbx
            rax = rbx;
            // 60330 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 60334 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 60335 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 60336 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x29700 size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00029700<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x29700;
    loop {
        match pc {
        0x29700 => {
            // block 29700
            // 29700 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 29701 mov rbp,rsp
            rbp = rsp;
            // 29704 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 29706 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 29708 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2970a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2970b mov r14,rsi
            r14 = rsi;
            // 2970e mov esi,DWORD PTR [rdi+0x58]
            rsi = (m.r32(rdi.wrapping_add(0x58)) as u64) as u32 as u64;
            // 29711 mov r12,QWORD PTR [rdi+0x50]
            r12 = m.r64(rdi.wrapping_add(0x50)) as u64;
            // 29715 mov rcx,QWORD PTR [r14]
            rcx = m.r64(r14) as u64;
            // 29718 mov rdi,QWORD PTR [r14+0x8]
            rdi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 2971c cmp rdi,rcx
            fl = sub::<64>(rdi, rcx).1;
            // 2971f je 0x29748
            if fl.e() { pc = 0x29748; continue; }
            pc = 0x29721;
        }
        0x29721 => {
            // block 29721
            // 29721 lea rax,[rdi-0xc]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fff4);
            // 29725 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 29728 movabs rcx,0xaaaaaaaaaaaaaaab
            rcx = 0xaaaa_aaaa_aaaa_aaabu64;
            // 29732 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // 29735 shr rdx,0x3
            { let (r, f) = shr::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // 29739 not rdx
            rdx = !rdx;
            // 2973c lea rax,[rdx+rdx*2]
            rax = rdx.wrapping_add(rdx.wrapping_mul(2));
            // 29740 lea rcx,[rdi+rax*4]
            rcx = rdi.wrapping_add(rax.wrapping_mul(4));
            // 29744 mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            pc = 0x29748;
        }
        0x29748 => {
            // block 29748
            // 29748 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2974b je 0x297a8
            if fl.e() { pc = 0x297a8; continue; }
            pc = 0x2974d;
        }
        0x2974d => {
            // block 2974d
            // 2974d mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 2974f mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 29754 imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // 29758 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // 2975c cmp esi,0x14f
            fl = sub::<32>((rsi as u32 as u64), 0x14fu64).1;
            // 29762 mov ebx,0x1b
            rbx = (0x1bu64) as u32 as u64;
            // 29767 cmovbe ebx,edx
            if fl.be() { rbx = ((rdx as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // 2976a mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 2976d test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2976f jne 0x2977b
            if fl.ne() { pc = 0x2977b; continue; }
            pc = 0x29771;
        }
        0x29771 => {
            // block 29771
            // 29771 jmp 0x297ab
            pc = 0x297ab; continue;
        }
        0x29773 => {
            // block 29773
            // 29773 add r12,0xc
            { let (r, f) = add::<64>(r12, 0xcu64);
              r12 = r;
              fl = f; }
            // 29777 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            pc = 0x2977b;
        }
        0x2977b => {
            // block 2977b
            // 2977b cmp rcx,QWORD PTR [r14+0x10]
            fl = sub::<64>(rcx, m.r64(r14.wrapping_add(0x10)) as u64).1;
            // 2977f je 0x29797
            if fl.e() { pc = 0x29797; continue; }
            pc = 0x29781;
        }
        0x29781 => {
            // block 29781
            // 29781 mov eax,DWORD PTR [r12+0x8]
            rax = (m.r32(r12.wrapping_add(0x8)) as u64) as u32 as u64;
            // 29786 mov DWORD PTR [rcx+0x8],eax
            m.w32(rcx.wrapping_add(0x8), ((rax as u32 as u64)) as u32);
            // 29789 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2978d mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 29790 add QWORD PTR [r14+0x8],0xc
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x8)) as u64, 0xcu64);
              m.w64(r14.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 29795 jmp 0x297a2
            pc = 0x297a2; continue;
        }
        0x29797 => {
            // block 29797
            // 29797 mov rdi,r14
            rdi = r14;
            // 2979a mov rsi,r12
            rsi = r12;
            // 2979d call 0xebaf2
            { let r = { let t = m.env.stub_va(0x2bf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x297a2;
        }
        0x297a2 => {
            // block 297a2
            // 297a2 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 297a4 jne 0x29773
            if fl.ne() { pc = 0x29773; continue; }
            pc = 0x297a6;
        }
        0x297a6 => {
            // block 297a6
            // 297a6 jmp 0x297ab
            pc = 0x297ab; continue;
        }
        0x297a8 => {
            // block 297a8
            // 297a8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x297ab;
        }
        0x297ab => {
            // block 297ab
            // 297ab mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 297ae pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 297af pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 297b1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 297b3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 297b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 297b6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b76a size=18
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b76a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2b76a;
    loop {
        match pc {
        0x2b76a => {
            // block 2b76a
            // 2b76a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b76b mov rbp,rsp
            rbp = rsp;
            // 2b76e add rdi,0x11c
            { let (r, f) = add::<64>(rdi, 0x11cu64);
              rdi = r;
              fl = f; }
            // 2b775 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b776 jmp 0x29700
            { let r = { let t = m.env.code_va(0x29700); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5c92 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5c92<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5c92;
    loop {
        match pc {
        0xb5c92 => {
            // block b5c92
            // b5c92 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5c93 mov rbp,rsp
            rbp = rsp;
            // b5c96 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5c98 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5c9a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5c9c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5c9e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5c9f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5ca3 mov rbx,rcx
            rbx = rcx;
            // b5ca6 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5ca9 je 0xb5d53
            if fl.e() { pc = 0xb5d53; continue; }
            pc = 0xb5caf;
        }
        0xb5caf => {
            // block b5caf
            // b5caf movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // b5cb3 mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b5cb6 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5cba shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b5cbd mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // b5cc1 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b5cc5 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b5cc9 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5ccd lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // b5cd1 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xb5cd5;
        }
        0xb5cd5 => {
            // block b5cd5
            // b5cd5 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // b5cd9 mov rbx,rdx
            rbx = rdx;
            pc = 0xb5cdc;
        }
        0xb5cdc => {
            // block b5cdc
            // b5cdc movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5ce1 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b5ce4 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5ce6 jne 0xb5ced
            if fl.ne() { pc = 0xb5ced; continue; }
            pc = 0xb5ce8;
        }
        0xb5ce8 => {
            // block b5ce8
            // b5ce8 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // b5ceb jmp 0xb5cf1
            pc = 0xb5cf1; continue;
        }
        0xb5ced => {
            // block b5ced
            // b5ced mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb5cf1;
        }
        0xb5cf1 => {
            // block b5cf1
            // b5cf1 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5cf4 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b5cf8 cmove r12,r14
            if fl.e() { r12 = r14; }
            // b5cfc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b5cfe jne 0xb5d09
            if fl.ne() { pc = 0xb5d09; continue; }
            pc = 0xb5d00;
        }
        0xb5d00 => {
            // block b5d00
            // b5d00 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // b5d04 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b5d07 jmp 0xb5d0d
            pc = 0xb5d0d; continue;
        }
        0xb5d09 => {
            // block b5d09
            // b5d09 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb5d0d;
        }
        0xb5d0d => {
            // block b5d0d
            // b5d0d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // b5d10 mov rdx,r13
            rdx = r13;
            // b5d13 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // b5d17 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5d1a je 0xb5d35
            if fl.e() { pc = 0xb5d35; continue; }
            pc = 0xb5d1c;
        }
        0xb5d1c => {
            // block b5d1c
            // b5d1c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5d1f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b5d23 cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b5d28 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5d2d test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b5d2f je 0xb5d35
            if fl.e() { pc = 0xb5d35; continue; }
            pc = 0xb5d31;
        }
        0xb5d31 => {
            // block b5d31
            // b5d31 js 0xb5d3a
            if fl.s() { pc = 0xb5d3a; continue; }
            pc = 0xb5d33;
        }
        0xb5d33 => {
            // block b5d33
            // b5d33 jmp 0xb5d45
            pc = 0xb5d45; continue;
        }
        0xb5d35 => {
            // block b5d35
            // b5d35 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // b5d38 jae 0xb5d45
            if fl.ae() { pc = 0xb5d45; continue; }
            pc = 0xb5d3a;
        }
        0xb5d3a => {
            // block b5d3a
            // b5d3a mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b5d3e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5d41 jne 0xb5cdc
            if fl.ne() { pc = 0xb5cdc; continue; }
            pc = 0xb5d43;
        }
        0xb5d43 => {
            // block b5d43
            // b5d43 jmp 0xb5d4f
            pc = 0xb5d4f; continue;
        }
        0xb5d45 => {
            // block b5d45
            // b5d45 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // b5d48 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5d4b jne 0xb5cd5
            if fl.ne() { pc = 0xb5cd5; continue; }
            pc = 0xb5d4d;
        }
        0xb5d4d => {
            // block b5d4d
            // b5d4d jmp 0xb5d53
            pc = 0xb5d53; continue;
        }
        0xb5d4f => {
            // block b5d4f
            // b5d4f mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb5d53;
        }
        0xb5d53 => {
            // block b5d53
            // b5d53 mov rax,rbx
            rax = rbx;
            // b5d56 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5d5a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5d5b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5d5d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5d5f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5d61 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5d63 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5d64 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4912 size=212
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4912<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4912;
    loop {
        match pc {
        0xc4912 => {
            // block c4912
            // c4912 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4913 mov rbp,rsp
            rbp = rsp;
            // c4916 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c4918 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c491a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c491c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c491e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c491f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c4923 mov rbx,rcx
            rbx = rcx;
            // c4926 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c4929 je 0xc49d3
            if fl.e() { pc = 0xc49d3; continue; }
            pc = 0xc492f;
        }
        0xc492f => {
            // block c492f
            // c492f movzx r14d,BYTE PTR [rsi]
            r14 = (m.r8(rsi) as u64) as u32 as u64;
            // c4933 mov r15b,r14b
            r15 = (r15 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // c4936 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c493a shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c493d mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4941 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c4945 mov rax,QWORD PTR [rsi+0x10]
            rax = m.r64(rsi.wrapping_add(0x10)) as u64;
            // c4949 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c494d lea rax,[rsi+0x1]
            rax = rsi.wrapping_add(0x1);
            // c4951 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            pc = 0xc4955;
        }
        0xc4955 => {
            // block c4955
            // c4955 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // c4959 mov rbx,rdx
            rbx = rdx;
            pc = 0xc495c;
        }
        0xc495c => {
            // block c495c
            // c495c movzx r13d,BYTE PTR [rbx+0x20]
            r13 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c4961 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // c4964 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c4966 jne 0xc496d
            if fl.ne() { pc = 0xc496d; continue; }
            pc = 0xc4968;
        }
        0xc4968 => {
            // block c4968
            // c4968 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // c496b jmp 0xc4971
            pc = 0xc4971; continue;
        }
        0xc496d => {
            // block c496d
            // c496d mov r13,QWORD PTR [rbx+0x28]
            r13 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc4971;
        }
        0xc4971 => {
            // block c4971
            // c4971 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c4974 mov r12,QWORD PTR [rbp-0x30]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // c4978 cmove r12,r14
            if fl.e() { r12 = r14; }
            // c497c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c497e jne 0xc4989
            if fl.ne() { pc = 0xc4989; continue; }
            pc = 0xc4980;
        }
        0xc4980 => {
            // block c4980
            // c4980 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // c4984 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c4987 jmp 0xc498d
            pc = 0xc498d; continue;
        }
        0xc4989 => {
            // block c4989
            // c4989 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc498d;
        }
        0xc498d => {
            // block c498d
            // c498d cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // c4990 mov rdx,r13
            rdx = r13;
            // c4993 cmovb rdx,r12
            if fl.b() { rdx = r12; }
            // c4997 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c499a je 0xc49b5
            if fl.e() { pc = 0xc49b5; continue; }
            pc = 0xc499c;
        }
        0xc499c => {
            // block c499c
            // c499c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c499f mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c49a3 cmove rsi,QWORD PTR [rbp-0x40]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // c49a8 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c49ad test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c49af je 0xc49b5
            if fl.e() { pc = 0xc49b5; continue; }
            pc = 0xc49b1;
        }
        0xc49b1 => {
            // block c49b1
            // c49b1 js 0xc49ba
            if fl.s() { pc = 0xc49ba; continue; }
            pc = 0xc49b3;
        }
        0xc49b3 => {
            // block c49b3
            // c49b3 jmp 0xc49c5
            pc = 0xc49c5; continue;
        }
        0xc49b5 => {
            // block c49b5
            // c49b5 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // c49b8 jae 0xc49c5
            if fl.ae() { pc = 0xc49c5; continue; }
            pc = 0xc49ba;
        }
        0xc49ba => {
            // block c49ba
            // c49ba mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c49be test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c49c1 jne 0xc495c
            if fl.ne() { pc = 0xc495c; continue; }
            pc = 0xc49c3;
        }
        0xc49c3 => {
            // block c49c3
            // c49c3 jmp 0xc49cf
            pc = 0xc49cf; continue;
        }
        0xc49c5 => {
            // block c49c5
            // c49c5 mov rdx,QWORD PTR [rbx]
            rdx = m.r64(rbx) as u64;
            // c49c8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c49cb jne 0xc4955
            if fl.ne() { pc = 0xc4955; continue; }
            pc = 0xc49cd;
        }
        0xc49cd => {
            // block c49cd
            // c49cd jmp 0xc49d3
            pc = 0xc49d3; continue;
        }
        0xc49cf => {
            // block c49cf
            // c49cf mov rbx,QWORD PTR [rbp-0x48]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xc49d3;
        }
        0xc49d3 => {
            // block c49d3
            // c49d3 mov rax,rbx
            rax = rbx;
            // c49d6 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c49da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c49db pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c49dd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c49df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c49e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c49e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c49e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8250 size=216
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd8250;
    loop {
        match pc {
        0xd8250 => {
            // block d8250
            // d8250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8251 mov rbp,rsp
            rbp = rsp;
            // d8254 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d8255 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d8256 mov rbx,rdi
            rbx = rdi;
            // d8259 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d825c mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8261 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d8263 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8268 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d826b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d826e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8271 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8276 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d827b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8280 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8283 mov rdi,rbx
            rdi = rbx;
            // d8286 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8289 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d828c mov rdi,rbx
            rdi = rbx;
            // d828f call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8295 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8298 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d829d xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d829f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82a4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82a7 mov rdi,rbx
            rdi = rbx;
            // d82aa call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ad mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82b0 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82b5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d82ba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82bf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82c2 mov rdi,rbx
            rdi = rbx;
            // d82c5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82c8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82cb mov rdi,rbx
            rdi = rbx;
            // d82ce call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82d4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82d7 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d82dc mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d82e1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d82e6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d82e9 mov rdi,rbx
            rdi = rbx;
            // d82ec call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82f2 mov rdi,rbx
            rdi = rbx;
            // d82f5 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d82fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d82fe mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d8303 mov edx,0xc7
            rdx = (0xc7u64) as u32 as u64;
            // d8308 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d830d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8310 mov rdi,rbx
            rdi = rbx;
            // d8313 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8316 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8319 mov rdi,rbx
            rdi = rbx;
            // d831c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d8320 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d8321 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8322 jmp QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc4eb0 size=220
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c4eb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc4eb0;
    loop {
        match pc {
        0xc4eb0 => {
            // block c4eb0
            // c4eb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c4eb1 mov rbp,rsp
            rbp = rsp;
            // c4eb4 lea r9,[rdi+0x8]
            r9 = rdi.wrapping_add(0x8);
            // c4eb8 cmp r9,rsi
            fl = sub::<64>(r9, rsi).1;
            // c4ebb je 0xc4ec8
            if fl.e() { pc = 0xc4ec8; continue; }
            pc = 0xc4ebd;
        }
        0xc4ebd => {
            // block c4ebd
            // c4ebd mov r8d,DWORD PTR [rcx]
            r8 = (m.r32(rcx) as u64) as u32 as u64;
            // c4ec0 mov eax,DWORD PTR [rsi+0x1c]
            rax = (m.r32(rsi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4ec3 cmp r8d,eax
            fl = sub::<32>((r8 as u32 as u64), (rax as u32 as u64)).1;
            // c4ec6 jae 0xc4ee9
            if fl.ae() { pc = 0xc4ee9; continue; }
            pc = 0xc4ec8;
        }
        0xc4ec8 => {
            // block c4ec8
            // c4ec8 cmp QWORD PTR [rdi],rsi
            fl = sub::<64>(m.r64(rdi) as u64, rsi).1;
            // c4ecb mov r8,QWORD PTR [rsi]
            r8 = m.r64(rsi) as u64;
            // c4ece mov r9,rsi
            r9 = rsi;
            // c4ed1 je 0xc4f1b
            if fl.e() { pc = 0xc4f1b; continue; }
            pc = 0xc4ed3;
        }
        0xc4ed3 => {
            // block c4ed3
            // c4ed3 test r8,r8
            fl = logic::<64>(r8 & r8);
            // c4ed6 je 0xc4f04
            if fl.e() { pc = 0xc4f04; continue; }
            pc = 0xc4ed8;
        }
        0xc4ed8 => {
            // block c4ed8
            // c4ed8 mov rax,r8
            rax = r8;
            pc = 0xc4edb;
        }
        0xc4edb => {
            // block c4edb
            // c4edb mov r9,rax
            r9 = rax;
            // c4ede mov rax,QWORD PTR [r9+0x8]
            rax = m.r64(r9.wrapping_add(0x8)) as u64;
            // c4ee2 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4ee5 jne 0xc4edb
            if fl.ne() { pc = 0xc4edb; continue; }
            pc = 0xc4ee7;
        }
        0xc4ee7 => {
            // block c4ee7
            // c4ee7 jmp 0xc4f13
            pc = 0xc4f13; continue;
        }
        0xc4ee9 => {
            // block c4ee9
            // c4ee9 cmp eax,r8d
            fl = sub::<32>((rax as u32 as u64), (r8 as u32 as u64)).1;
            // c4eec jae 0xc4f34
            if fl.ae() { pc = 0xc4f34; continue; }
            pc = 0xc4eee;
        }
        0xc4eee => {
            // block c4eee
            // c4eee mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c4ef2 test rax,rax
            fl = logic::<64>(rax & rax);
            // c4ef5 je 0xc4f39
            if fl.e() { pc = 0xc4f39; continue; }
            pc = 0xc4ef7;
        }
        0xc4ef7 => {
            // block c4ef7
            // c4ef7 mov r10,rax
            r10 = rax;
            // c4efa mov rax,QWORD PTR [r10]
            rax = m.r64(r10) as u64;
            // c4efd test rax,rax
            fl = logic::<64>(rax & rax);
            // c4f00 jne 0xc4ef7
            if fl.ne() { pc = 0xc4ef7; continue; }
            pc = 0xc4f02;
        }
        0xc4f02 => {
            // block c4f02
            // c4f02 jmp 0xc4f48
            pc = 0xc4f48; continue;
        }
        0xc4f04 => {
            // block c4f04
            // c4f04 mov rax,rsi
            rax = rsi;
            pc = 0xc4f07;
        }
        0xc4f07 => {
            // block c4f07
            // c4f07 mov r9,QWORD PTR [rax+0x10]
            r9 = m.r64(rax.wrapping_add(0x10)) as u64;
            // c4f0b cmp QWORD PTR [r9],rax
            fl = sub::<64>(m.r64(r9) as u64, rax).1;
            // c4f0e mov rax,r9
            rax = r9;
            // c4f11 je 0xc4f07
            if fl.e() { pc = 0xc4f07; continue; }
            pc = 0xc4f13;
        }
        0xc4f13 => {
            // block c4f13
            // c4f13 mov eax,DWORD PTR [r9+0x1c]
            rax = (m.r32(r9.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c4f17 cmp eax,DWORD PTR [rcx]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx) as u64).1;
            // c4f19 jae 0xc4f62
            if fl.ae() { pc = 0xc4f62; continue; }
            pc = 0xc4f1b;
        }
        0xc4f1b => {
            // block c4f1b
            // c4f1b test r8,r8
            fl = logic::<64>(r8 & r8);
            // c4f1e je 0xc4f2c
            if fl.e() { pc = 0xc4f2c; continue; }
            pc = 0xc4f20;
        }
        0xc4f20 => {
            // block c4f20
            // c4f20 mov QWORD PTR [rdx],r9
            m.w64(rdx, (r9) as u64);
            // c4f23 add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c4f27 mov rdx,r9
            rdx = r9;
            // c4f2a jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f2c => {
            // block c4f2c
            // c4f2c mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f2f mov rdx,rsi
            rdx = rsi;
            // c4f32 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f34 => {
            // block c4f34
            // c4f34 mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f37 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f39 => {
            // block c4f39
            // c4f39 mov rax,rsi
            rax = rsi;
            pc = 0xc4f3c;
        }
        0xc4f3c => {
            // block c4f3c
            // c4f3c mov r10,QWORD PTR [rax+0x10]
            r10 = m.r64(rax.wrapping_add(0x10)) as u64;
            // c4f40 cmp QWORD PTR [r10],rax
            fl = sub::<64>(m.r64(r10) as u64, rax).1;
            // c4f43 mov rax,r10
            rax = r10;
            // c4f46 jne 0xc4f3c
            if fl.ne() { pc = 0xc4f3c; continue; }
            pc = 0xc4f48;
        }
        0xc4f48 => {
            // block c4f48
            // c4f48 cmp r10,r9
            fl = sub::<64>(r10, r9).1;
            // c4f4b je 0xc4f53
            if fl.e() { pc = 0xc4f53; continue; }
            pc = 0xc4f4d;
        }
        0xc4f4d => {
            // block c4f4d
            // c4f4d cmp r8d,DWORD PTR [r10+0x1c]
            fl = sub::<32>((r8 as u32 as u64), m.r32(r10.wrapping_add(0x1c)) as u64).1;
            // c4f51 jae 0xc4f62
            if fl.ae() { pc = 0xc4f62; continue; }
            pc = 0xc4f53;
        }
        0xc4f53 => {
            // block c4f53
            // c4f53 cmp QWORD PTR [rsi+0x8],0x0
            fl = sub::<64>(m.r64(rsi.wrapping_add(0x8)) as u64, 0x0u64).1;
            // c4f58 je 0xc4f6e
            if fl.e() { pc = 0xc4f6e; continue; }
            pc = 0xc4f5a;
        }
        0xc4f5a => {
            // block c4f5a
            // c4f5a mov QWORD PTR [rdx],r10
            m.w64(rdx, (r10) as u64);
            // c4f5d mov rdx,r10
            rdx = r10;
            // c4f60 jmp 0xc4f78
            pc = 0xc4f78; continue;
        }
        0xc4f62 => {
            // block c4f62
            // c4f62 mov rsi,rdx
            rsi = rdx;
            // c4f65 mov rdx,rcx
            rdx = rcx;
            // c4f68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4f69 jmp 0xebd8c
            { let r = { let t = m.env.stub_va(0x559); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
        0xc4f6e => {
            // block c4f6e
            // c4f6e mov QWORD PTR [rdx],rsi
            m.w64(rdx, (rsi) as u64);
            // c4f71 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // c4f75 mov rdx,rsi
            rdx = rsi;
            pc = 0xc4f78;
        }
        0xc4f78 => {
            // block c4f78
            // c4f78 mov rax,rdx
            rax = rdx;
            // c4f7b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4f7c ret
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

/// parity: P2v @0xe64b6 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e64b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe64b6;
    loop {
        match pc {
        0xe64b6 => {
            // block e64b6
            // e64b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e64b7 mov rbp,rsp
            rbp = rsp;
            // e64ba mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64bc shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64bf mov DWORD PTR [rdi+0x7c],eax
            m.w32(rdi.wrapping_add(0x7c), ((rax as u32 as u64)) as u32);
            // e64c2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64c4 shr eax,0x18
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64c7 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64ca mov DWORD PTR [rdi+0x78],eax
            m.w32(rdi.wrapping_add(0x78), ((rax as u32 as u64)) as u32);
            // e64cd mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64cf shr eax,0x14
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64d2 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64d5 mov DWORD PTR [rdi+0x74],eax
            m.w32(rdi.wrapping_add(0x74), ((rax as u32 as u64)) as u32);
            // e64d8 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e64da shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e64dd and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e64e0 mov DWORD PTR [rdi+0x70],eax
            m.w32(rdi.wrapping_add(0x70), ((rax as u32 as u64)) as u32);
            // e64e3 mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // e64e6 shr r10d,0xc
            { let (r, f) = shr::<32>((r10 as u32 as u64), 0xcu64, fl); r10 = (r) as u32 as u64; fl = f; }
            // e64ea mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // e64ec shr edx,0x8
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x8u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e64ef mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // e64f1 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e64f4 movd xmm0,r10d
            x0 = ((r10 as u32 as u64)) as u32 as u128;
            // e64f9 movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // e64fd punpckldq xmm1,xmm0
            { let a = x1; let b = x0; x1 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e6501 movd xmm0,esi
            x0 = ((rsi as u32 as u64)) as u32 as u128;
            // e6505 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // e6509 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e650d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e6511 pand xmm0,XMMWORD PTR [rip+0x41e07] # 0x128320
            x0 &= m.r128(m.env.data_va(248557));
            // e6519 movdqu XMMWORD PTR [rdi+0x60],xmm0
            m.w128(rdi.wrapping_add(0x60), x0);
            // e651e mov r8d,0xf0f0f0f
            r8 = (0xf0f0f0fu64) as u32 as u64;
            // e6524 mov r9d,DWORD PTR [rdi+0x88]
            r9 = (m.r32(rdi.wrapping_add(0x88)) as u64) as u32 as u64;
            // e652b and r9d,r8d
            { let r = (r9 as u32 as u64) & (r8 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // e652e and r8d,DWORD PTR [rdi+0x84]
            { let r = (r8 as u32 as u64) & m.r32(rdi.wrapping_add(0x84)) as u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e6535 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e6537 and eax,0xf0000000
            { let r = (rax as u32 as u64) & 0xf0000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e653c and ecx,0xf00000
            { let r = (rcx as u32 as u64) & 0xf00000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6542 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6544 and edx,0xf000
            { let r = (rdx as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e654a or edx,ecx
            { let r = (rdx as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e654c and r10d,0xf0
            { let r = (r10 as u32 as u64) & 0xf0u64; let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6553 or r10d,edx
            { let r = (r10 as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6556 or r10d,r9d
            { let r = (r10 as u32 as u64) | (r9 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // e6559 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // e655b shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e655e and eax,0xf0000000
            { let r = (rax as u32 as u64) & 0xf0000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6563 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // e6565 shl ecx,0xc
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0xcu64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e6568 and ecx,0xf00000
            { let r = (rcx as u32 as u64) & 0xf00000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e656e or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6570 shl esi,0x8
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x8u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e6573 and esi,0xf000
            { let r = (rsi as u32 as u64) & 0xf000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e6579 or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e657b movd eax,xmm0
            rax = (x0 as u32 as u64) as u32 as u64;
            // e657f shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e6582 or eax,esi
            { let r = (rax as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6584 or eax,r8d
            { let r = (rax as u32 as u64) | (r8 as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e6587 mov DWORD PTR [rdi+0x88],r10d
            m.w32(rdi.wrapping_add(0x88), ((r10 as u32 as u64)) as u32);
            // e658e mov DWORD PTR [rdi+0x84],eax
            m.w32(rdi.wrapping_add(0x84), ((rax as u32 as u64)) as u32);
            // e6594 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e6596 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6597 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0404 size=230
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0404<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0404;
    loop {
        match pc {
        0xe0404 => {
            // block e0404
            // e0404 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0405 mov rbp,rsp
            rbp = rsp;
            // e0408 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e040a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e040c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e040e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e040f imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e0413 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // e0415 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0416 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // e0419 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // e041c imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e0420 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e0422 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0423 idiv r8d
            { let (q, r) = idiv::<32>(rdx, rax, (r8 as u32 as u64)); rax = q; rdx = r; }
            // e0426 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // e0429 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // e042c jge 0xe04e0
            if fl.ge() { pc = 0xe04e0; continue; }
            pc = 0xe0432;
        }
        0xe0432 => {
            // block e0432
            // e0432 movzx ecx,WORD PTR [rbp+0x10]
            rcx = (m.r16(rbp.wrapping_add(0x10)) as u64) as u32 as u64;
            // e0436 shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // e043a mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // e043d cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e043e idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // e0441 sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // e0444 movsxd rdx,r11d
            rdx = sx::<32>((r11 as u32 as u64)) as u64;
            // e0447 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            // e044b movsx r9,cx
            r9 = sx::<16>((rcx as u16 as u64)) as u64;
            // e044f imul rdx,r9
            { let (r, f) = imul::<64>(rdx, r9); rdx = r; fl = f; }
            // e0453 lea r15,[rsi+rdx*2]
            r15 = rsi.wrapping_add(rdx.wrapping_mul(2));
            // e0457 add r9,r9
            { let (r, f) = add::<64>(r9, r9);
              r9 = r;
              fl = f; }
            // e045a lea r14,[rip+0x47bbf] # 0x128020
            r14 = m.env.data_va(247789);
            // e0461 mov r8w,0x7fff
            r8 = (r8 & !0xffff) | ((0x7fffu64) as u16 as u64);
            pc = 0xe0466;
        }
        0xe0466 => {
            // block e0466
            // e0466 mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // e0469 sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e046c mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e046f shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0472 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0475 movsxd r12,ecx
            r12 = sx::<32>((rcx as u32 as u64)) as u64;
            // e0478 movsx esi,WORD PTR [rdi+r12*2-0x2]
            rsi = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e047e mov edx,0x20
            rdx = (0x20u64) as u32 as u64;
            // e0483 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0485 imul esi,DWORD PTR [r14+rdx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r14.wrapping_add(rdx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e048a movsx edx,WORD PTR [rdi+r12*2]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e048f mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // e0494 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0496 imul edx,DWORD PTR [r14+rcx*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rcx.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e049b add edx,esi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rsi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e049d movsx ecx,WORD PTR [rdi+r12*2+0x2]
            rcx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e04a3 mov esi,0x60
            rsi = (0x60u64) as u32 as u64;
            // e04a8 sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e04aa imul ecx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e04af add ecx,edx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e04b1 movsx edx,WORD PTR [rdi+r12*2+0x4]
            rdx = (sx::<16>(m.r16(rdi.wrapping_add(r12.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e04b7 mov esi,0x80
            rsi = (0x80u64) as u32 as u64;
            // e04bc sub esi,ebx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e04be imul edx,DWORD PTR [r14+rsi*4]
            { let (r, f) = imul::<32>((rdx as u32 as u64), m.r32(r14.wrapping_add(rsi.wrapping_mul(4))) as u64); rdx = (r) as u32 as u64; fl = f; }
            // e04c3 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e04c5 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // e04c7 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e04ca test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e04cc cmovs cx,r8w
            if fl.s() { rcx = (rcx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            // e04d1 mov WORD PTR [r15],cx
            m.w16(r15, ((rcx as u16 as u64)) as u16);
            // e04d5 add r11d,eax
            { let (r, f) = add::<32>((r11 as u32 as u64), (rax as u32 as u64));
              r11 = (r) as u32 as u64;
              fl = f; }
            // e04d8 add r15,r9
            { let (r, f) = add::<64>(r15, r9);
              r15 = r;
              fl = f; }
            // e04db dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e04de jne 0xe0466
            if fl.ne() { pc = 0xe0466; continue; }
            pc = 0xe04e0;
        }
        0xe04e0 => {
            // block e04e0
            // e04e0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e04e1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e04e3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e04e5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e04e7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e04e8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5556 size=240
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5556<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5556;
    loop {
        match pc {
        0xb5556 => {
            // block b5556
            // b5556 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5557 mov rbp,rsp
            rbp = rsp;
            // b555a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b555c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b555e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5560 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5562 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5563 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b5567 mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b556b test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b556e je 0xb5621
            if fl.e() { pc = 0xb5621; continue; }
            pc = 0xb5574;
        }
        0xb5574 => {
            // block b5574
            // b5574 mov QWORD PTR [rbp-0x48],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rsi) as u64);
            // b5578 movzx r15d,BYTE PTR [rdx]
            r15 = (m.r8(rdx) as u64) as u32 as u64;
            // b557c mov r12b,r15b
            r12 = (r12 & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b557f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5583 shr r15,1
            { let (r, f) = shr::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // b5586 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // b558a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b558e mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // b5592 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5596 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // b559a mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // b559e jmp 0xb55a3
            pc = 0xb55a3; continue;
        }
        0xb55a0 => {
            // block b55a0
            // b55a0 mov rbx,rax
            rbx = rax;
            pc = 0xb55a3;
        }
        0xb55a3 => {
            // block b55a3
            // b55a3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55a6 mov r13,QWORD PTR [rbp-0x30]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b55aa cmove r13,r15
            if fl.e() { r13 = r15; }
            // b55ae movzx r14d,BYTE PTR [rbx+0x20]
            r14 = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b55b3 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b55b6 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b55b8 jne 0xb55bf
            if fl.ne() { pc = 0xb55bf; continue; }
            pc = 0xb55ba;
        }
        0xb55ba => {
            // block b55ba
            // b55ba shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b55bd jmp 0xb55c3
            pc = 0xb55c3; continue;
        }
        0xb55bf => {
            // block b55bf
            // b55bf mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb55c3;
        }
        0xb55c3 => {
            // block b55c3
            // b55c3 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b55c6 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b55ca cmove rdi,QWORD PTR [rbp-0x40]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64; }
            // b55cf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b55d1 jne 0xb55dc
            if fl.ne() { pc = 0xb55dc; continue; }
            pc = 0xb55d3;
        }
        0xb55d3 => {
            // block b55d3
            // b55d3 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // b55d7 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // b55da jmp 0xb55e0
            pc = 0xb55e0; continue;
        }
        0xb55dc => {
            // block b55dc
            // b55dc mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb55e0;
        }
        0xb55e0 => {
            // block b55e0
            // b55e0 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b55e3 mov rdx,r13
            rdx = r13;
            // b55e6 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b55ea test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b55ed je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55ef;
        }
        0xb55ef => {
            // block b55ef
            // b55ef call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b55f4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b55f6 je 0xb55fc
            if fl.e() { pc = 0xb55fc; continue; }
            pc = 0xb55f8;
        }
        0xb55f8 => {
            // block b55f8
            // b55f8 jns 0xb560b
            if fl.ns() { pc = 0xb560b; continue; }
            pc = 0xb55fa;
        }
        0xb55fa => {
            // block b55fa
            // b55fa jmp 0xb5601
            pc = 0xb5601; continue;
        }
        0xb55fc => {
            // block b55fc
            // b55fc cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b55ff jae 0xb560b
            if fl.ae() { pc = 0xb560b; continue; }
            pc = 0xb5601;
        }
        0xb5601 => {
            // block b5601
            // b5601 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b5604 test rax,rax
            fl = logic::<64>(rax & rax);
            // b5607 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5609;
        }
        0xb5609 => {
            // block b5609
            // b5609 jmp 0xb562d
            pc = 0xb562d; continue;
        }
        0xb560b => {
            // block b560b
            // b560b mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b560f test rax,rax
            fl = logic::<64>(rax & rax);
            // b5612 jne 0xb55a0
            if fl.ne() { pc = 0xb55a0; continue; }
            pc = 0xb5614;
        }
        0xb5614 => {
            // block b5614
            // b5614 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5618 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b561b add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // b561f jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb5621 => {
            // block b5621
            // b5621 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b5625 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b5628 mov rbx,rdi
            rbx = rdi;
            // b562b jmp 0xb5634
            pc = 0xb5634; continue;
        }
        0xb562d => {
            // block b562d
            // b562d mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5631 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            pc = 0xb5634;
        }
        0xb5634 => {
            // block b5634
            // b5634 mov rax,rbx
            rax = rbx;
            // b5637 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b563b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b563c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b563e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b5640 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5642 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5644 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5645 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dd46 size=247
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dd46<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dd46;
    loop {
        match pc {
        0x2dd46 => {
            // block 2dd46
            // 2dd46 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dd47 mov rbp,rsp
            rbp = rsp;
            // 2dd4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2dd4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2dd4d mov rax,QWORD PTR [rip+0x112694] # 0x1403e8
            rax = m.r64(m.env.data_va(347061)) as u64;
            // 2dd54 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 2dd57 lea rsi,[rip+0x118d4a] # 0x146aa8
            rsi = m.env.data_va(373365);
            // 2dd5e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2dd60 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 2dd65 mov rdi,rbx
            rdi = rbx;
            // 2dd68 call 0xebf5a
            { let r = { let t = m.env.stub_va(0x727); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dd6d mov r14,rax
            r14 = rax;
            // 2dd70 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 2dd73 je 0x2de38
            if fl.e() { pc = 0x2de38; continue; }
            pc = 0x2dd79;
        }
        0x2dd79 => {
            // block 2dd79
            // 2dd79 mov rdi,rbx
            rdi = rbx;
            // 2dd7c mov rsi,r14
            rsi = r14;
            // 2dd7f call 0xebf36
            { let r = { let t = m.env.stub_va(0x703); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dd84 mov QWORD PTR [rip+0x193c95],rax # 0x1c1a20
            m.w64(m.env.data_va(877037), (rax) as u64);
            // 2dd8b test rax,rax
            fl = logic::<64>(rax & rax);
            // 2dd8e je 0x2de2c
            if fl.e() { pc = 0x2de2c; continue; }
            pc = 0x2dd94;
        }
        0x2dd94 => {
            // block 2dd94
            // 2dd94 lea rsi,[rip+0x118d2d] # 0x146ac8
            rsi = m.env.data_va(373397);
            // 2dd9b mov rdi,rax
            rdi = rax;
            // 2dd9e call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dda3 mov QWORD PTR [rip+0x193c7e],rax # 0x1c1a28
            m.w64(m.env.data_va(877045), (rax) as u64);
            // 2ddaa mov rdi,QWORD PTR [rip+0x193c6f] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddb1 lea rsi,[rip+0x118d30] # 0x146ae8
            rsi = m.env.data_va(373429);
            // 2ddb8 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddbd mov QWORD PTR [rip+0x193c6c],rax # 0x1c1a30
            m.w64(m.env.data_va(877053), (rax) as u64);
            // 2ddc4 mov rdi,QWORD PTR [rip+0x193c55] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddcb lea rsi,[rip+0x118d36] # 0x146b08
            rsi = m.env.data_va(373461);
            // 2ddd2 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddd7 mov QWORD PTR [rip+0x193c5a],rax # 0x1c1a38
            m.w64(m.env.data_va(877061), (rax) as u64);
            // 2ddde mov rdi,QWORD PTR [rip+0x193c3b] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2dde5 lea rsi,[rip+0x118d3c] # 0x146b28
            rsi = m.env.data_va(373493);
            // 2ddec call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ddf1 mov QWORD PTR [rip+0x193c48],rax # 0x1c1a40
            m.w64(m.env.data_va(877069), (rax) as u64);
            // 2ddf8 mov rdi,QWORD PTR [rip+0x193c21] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2ddff lea rsi,[rip+0x118d42] # 0x146b48
            rsi = m.env.data_va(373525);
            // 2de06 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de0b mov QWORD PTR [rip+0x193c36],rax # 0x1c1a48
            m.w64(m.env.data_va(877077), (rax) as u64);
            // 2de12 mov rdi,QWORD PTR [rip+0x193c07] # 0x1c1a20
            rdi = m.r64(m.env.data_va(877037)) as u64;
            // 2de19 lea rsi,[rip+0x118d48] # 0x146b68
            rsi = m.env.data_va(373557);
            // 2de20 call 0xebf3c
            { let r = { let t = m.env.stub_va(0x709); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de25 mov QWORD PTR [rip+0x193c24],rax # 0x1c1a50
            m.w64(m.env.data_va(877085), (rax) as u64);
            pc = 0x2de2c;
        }
        0x2de2c => {
            // block 2de2c
            // 2de2c mov rdi,r14
            rdi = r14;
            // 2de2f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2de30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2de32 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de33 jmp 0xebf42
            { let r = { let t = m.env.stub_va(0x70f); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x2de38 => {
            // block 2de38
            // 2de38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2de39 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2de3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18b50 size=253
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018b50<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18b50;
    loop {
        match pc {
        0x18b50 => {
            // block 18b50
            // 18b50 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18b51 mov rbp,rsp
            rbp = rsp;
            // 18b54 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18b56 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18b58 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 18b5a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18b5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18b5d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18b5e mov rbx,rsi
            rbx = rsi;
            // 18b61 mov r14,rdi
            r14 = rdi;
            // 18b64 mov r13,QWORD PTR [rbx]
            r13 = m.r64(rbx) as u64;
            // 18b67 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18b6b je 0x18c3b
            if fl.e() { pc = 0x18c3b; continue; }
            pc = 0x18b71;
        }
        0x18b71 => {
            // block 18b71
            // 18b71 lea r12,[rip+0x111741] # 0x12a2b9
            r12 = m.env.data_va(256646);
            // 18b78 lea r15,[rip+0x11172f] # 0x12a2ae
            r15 = m.env.data_va(256635);
            pc = 0x18b7f;
        }
        0x18b7f => {
            // block 18b7f
            // 18b7f mov eax,DWORD PTR [r13+0x0]
            rax = (m.r32(r13) as u64) as u32 as u64;
            // 18b83 cmp eax,0x6e6f696d
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696du64).1;
            // 18b88 jg 0x18ba9
            if fl.g() { pc = 0x18ba9; continue; }
            pc = 0x18b8a;
        }
        0x18b8a => {
            // block 18b8a
            // 18b8a cmp eax,0x69616c66
            fl = sub::<32>((rax as u32 as u64), 0x69616c66u64).1;
            // 18b8f jg 0x18bbd
            if fl.g() { pc = 0x18bbd; continue; }
            pc = 0x18b91;
        }
        0x18b91 => {
            // block 18b91
            // 18b91 cmp eax,0x69616474
            fl = sub::<32>((rax as u32 as u64), 0x69616474u64).1;
            // 18b96 jne 0x18bed
            if fl.ne() { pc = 0x18bed; continue; }
            pc = 0x18b98;
        }
        0x18b98 => {
            // block 18b98
            // 18b98 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18b9d mov rdi,r14
            rdi = r14;
            // 18ba0 lea rsi,[rip+0x1116f9] # 0x12a2a0
            rsi = m.env.data_va(256621);
            // 18ba7 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18ba9 => {
            // block 18ba9
            // 18ba9 cmp eax,0x6e6f696e
            fl = sub::<32>((rax as u32 as u64), 0x6e6f696eu64).1;
            // 18bae jne 0x18bd5
            if fl.ne() { pc = 0x18bd5; continue; }
            pc = 0x18bb0;
        }
        0x18bb0 => {
            // block 18bb0
            // 18bb0 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // 18bb5 mov rdi,r14
            rdi = r14;
            // 18bb8 mov rsi,r15
            rsi = r15;
            // 18bbb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bbd => {
            // block 18bbd
            // 18bbd cmp eax,0x69616c67
            fl = sub::<32>((rax as u32 as u64), 0x69616c67u64).1;
            // 18bc2 jne 0x18c05
            if fl.ne() { pc = 0x18c05; continue; }
            pc = 0x18bc4;
        }
        0x18bc4 => {
            // block 18bc4
            // 18bc4 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 18bc9 mov rdi,r14
            rdi = r14;
            // 18bcc lea rsi,[rip+0x1116d3] # 0x12a2a6
            rsi = m.env.data_va(256627);
            // 18bd3 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bd5 => {
            // block 18bd5
            // 18bd5 cmp eax,0x6f757420
            fl = sub::<32>((rax as u32 as u64), 0x6f757420u64).1;
            // 18bda jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bdc;
        }
        0x18bdc => {
            // block 18bdc
            // 18bdc mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18be1 mov rdi,r14
            rdi = r14;
            // 18be4 lea rsi,[rip+0x1116c9] # 0x12a2b4
            rsi = m.env.data_va(256641);
            // 18beb jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18bed => {
            // block 18bed
            // 18bed cmp eax,0x69616573
            fl = sub::<32>((rax as u32 as u64), 0x69616573u64).1;
            // 18bf2 jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18bf4;
        }
        0x18bf4 => {
            // block 18bf4
            // 18bf4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18bf9 mov rdi,r14
            rdi = r14;
            // 18bfc lea rsi,[rip+0x111698] # 0x12a29b
            rsi = m.env.data_va(256616);
            // 18c03 jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c05 => {
            // block 18c05
            // 18c05 cmp eax,0x69736469
            fl = sub::<32>((rax as u32 as u64), 0x69736469u64).1;
            // 18c0a jne 0x18c1d
            if fl.ne() { pc = 0x18c1d; continue; }
            pc = 0x18c0c;
        }
        0x18c0c => {
            // block 18c0c
            // 18c0c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c11 mov rdi,r14
            rdi = r14;
            // 18c14 lea rsi,[rip+0x11167b] # 0x12a296
            rsi = m.env.data_va(256611);
            // 18c1b jmp 0x18c28
            pc = 0x18c28; continue;
        }
        0x18c1d => {
            // block 18c1d
            // 18c1d mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 18c22 mov rdi,r14
            rdi = r14;
            // 18c25 mov rsi,r12
            rsi = r12;
            pc = 0x18c28;
        }
        0x18c28 => {
            // block 18c28
            // 18c28 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18c2d add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 18c31 cmp r13,QWORD PTR [rbx+0x8]
            fl = sub::<64>(r13, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // 18c35 jne 0x18b7f
            if fl.ne() { pc = 0x18b7f; continue; }
            pc = 0x18c3b;
        }
        0x18c3b => {
            // block 18c3b
            // 18c3b mov rax,r14
            rax = r14;
            // 18c3e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 18c42 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18c43 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18c45 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18c47 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18c49 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18c4b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18c4c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfd3a size=254
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fd3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xfd3a;
    loop {
        match pc {
        0xfd3a => {
            // block fd3a
            // fd3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fd3b mov rbp,rsp
            rbp = rsp;
            // fd3e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fd40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fd41 mov r14,rsi
            r14 = rsi;
            // fd44 mov rbx,rdi
            rbx = rdi;
            // fd47 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd4a movzx esi,WORD PTR [r14]
            rsi = (m.r16(r14) as u64) as u32 as u64;
            // fd4e call QWORD PTR [rax+0x1c08]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c08)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd54 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd57 movzx esi,WORD PTR [r14+0x2]
            rsi = (m.r16(r14.wrapping_add(0x2)) as u64) as u32 as u64;
            // fd5c mov rdi,rbx
            rdi = rbx;
            // fd5f call QWORD PTR [rax+0x1c18]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c18)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd68 movzx esi,WORD PTR [r14+0x4]
            rsi = (m.r16(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // fd6d mov rdi,rbx
            rdi = rbx;
            // fd70 call QWORD PTR [rax+0x1c28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd76 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd79 movzx esi,WORD PTR [r14+0x6]
            rsi = (m.r16(r14.wrapping_add(0x6)) as u64) as u32 as u64;
            // fd7e mov rdi,rbx
            rdi = rbx;
            // fd81 call QWORD PTR [rax+0x1c38]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd8a movzx esi,WORD PTR [r14+0x8]
            rsi = (m.r16(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // fd8f mov rdi,rbx
            rdi = rbx;
            // fd92 call QWORD PTR [rax+0x1c48]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd98 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd9b movzx esi,WORD PTR [r14+0xa]
            rsi = (m.r16(r14.wrapping_add(0xa)) as u64) as u32 as u64;
            // fda0 mov rdi,rbx
            rdi = rbx;
            // fda3 call QWORD PTR [rax+0x1c58]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fda9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdac movzx esi,WORD PTR [r14+0xc]
            rsi = (m.r16(r14.wrapping_add(0xc)) as u64) as u32 as u64;
            // fdb1 mov rdi,rbx
            rdi = rbx;
            // fdb4 call QWORD PTR [rax+0x1c68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c68)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdbd movzx esi,WORD PTR [r14+0xe]
            rsi = (m.r16(r14.wrapping_add(0xe)) as u64) as u32 as u64;
            // fdc2 mov rdi,rbx
            rdi = rbx;
            // fdc5 call QWORD PTR [rax+0x1c78]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c78)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdce movzx esi,WORD PTR [r14+0x10]
            rsi = (m.r16(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // fdd3 mov rdi,rbx
            rdi = rbx;
            // fdd6 call QWORD PTR [rax+0x1c88]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c88)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fddc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fddf movzx esi,WORD PTR [r14+0x12]
            rsi = (m.r16(r14.wrapping_add(0x12)) as u64) as u32 as u64;
            // fde4 mov rdi,rbx
            rdi = rbx;
            // fde7 call QWORD PTR [rax+0x1c98]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fded mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdf0 movzx esi,WORD PTR [r14+0x14]
            rsi = (m.r16(r14.wrapping_add(0x14)) as u64) as u32 as u64;
            // fdf5 mov rdi,rbx
            rdi = rbx;
            // fdf8 call QWORD PTR [rax+0x1ca8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ca8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdfe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe01 movzx esi,WORD PTR [r14+0x16]
            rsi = (m.r16(r14.wrapping_add(0x16)) as u64) as u32 as u64;
            // fe06 mov rdi,rbx
            rdi = rbx;
            // fe09 call QWORD PTR [rax+0x1cb8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cb8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe0f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe12 movzx esi,BYTE PTR [r14+0x18]
            rsi = (m.r8(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // fe17 mov rdi,rbx
            rdi = rbx;
            // fe1a call QWORD PTR [rax+0x1cc8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe20 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe23 movzx esi,BYTE PTR [r14+0x19]
            rsi = (m.r8(r14.wrapping_add(0x19)) as u64) as u32 as u64;
            // fe28 mov rdi,rbx
            rdi = rbx;
            // fe2b call QWORD PTR [rax+0x1cd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe31 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // fe33 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fe34 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fe36 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fe37 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eea0 size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eea0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2eea0;
    loop {
        match pc {
        0x2eea0 => {
            // block 2eea0
            // 2eea0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eea1 mov rbp,rsp
            rbp = rsp;
            // 2eea4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2eea5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2eea6 mov rbx,rdi
            rbx = rdi;
            // 2eea9 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2eeac je 0x2eed8
            if fl.e() { pc = 0x2eed8; continue; }
            pc = 0x2eeae;
        }
        0x2eeae => {
            // block 2eeae
            // 2eeae mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 2eeb1 mov rdi,rsi
            rdi = rsi;
            // 2eeb4 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2eeb7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2eeba je 0x2eed8
            if fl.e() { pc = 0x2eed8; continue; }
            pc = 0x2eebc;
        }
        0x2eebc => {
            // block 2eebc
            // 2eebc cmp BYTE PTR [rax],0x0
            fl = sub::<8>(m.r8(rax) as u64, 0x0u64).1;
            // 2eebf je 0x2eed8
            if fl.e() { pc = 0x2eed8; continue; }
            pc = 0x2eec1;
        }
        0x2eec1 => {
            // block 2eec1
            // 2eec1 mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 2eec5 movsxd rdx,DWORD PTR [rcx]
            rdx = sx::<32>(m.r32(rcx) as u64) as u64;
            // 2eec8 lea esi,[rdx+0x1]
            rsi = (rdx.wrapping_add(0x1)) as u32 as u64;
            // 2eecb mov DWORD PTR [rcx],esi
            m.w32(rcx, ((rsi as u32 as u64)) as u32);
            // 2eecd mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2eed1 mov rcx,QWORD PTR [rcx]
            rcx = m.r64(rcx) as u64;
            // 2eed4 mov QWORD PTR [rcx+rdx*8],rax
            m.w64(rcx.wrapping_add(rdx.wrapping_mul(8)), (rax) as u64);
            pc = 0x2eed8;
        }
        0x2eed8 => {
            // block 2eed8
            // 2eed8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2eedc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2eedd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2eede ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e45e size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e45e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e45e;
    loop {
        match pc {
        0x2e45e => {
            // block 2e45e
            // 2e45e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e45f mov rbp,rsp
            rbp = rsp;
            // 2e462 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e464 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e466 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e468 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e469 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2e46d mov r14,rsi
            r14 = rsi;
            // 2e470 mov r12,rdi
            r12 = rdi;
            // 2e473 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e477 call QWORD PTR [rax+0x70]
            { let r = { let t = m.r64(rax.wrapping_add(0x70)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e47a mov rcx,QWORD PTR [r12+0x18]
            rcx = m.r64(r12.wrapping_add(0x18)) as u64;
            // 2e47f sub rcx,QWORD PTR [r12+0x10]
            { let (r, f) = sub::<64>(rcx, m.r64(r12.wrapping_add(0x10)) as u64);
              rcx = r;
              fl = f; }
            // 2e484 mov rax,rcx
            rax = rcx;
            // 2e487 shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 2e48b mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // 2e48e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e490 jle 0x2e51b
            if fl.le() { pc = 0x2e51b; continue; }
            pc = 0x2e496;
        }
        0x2e496 => {
            // block 2e496
            // 2e496 shl rcx,0x1d
            { let (r, f) = shl::<64>(rcx, 0x1du64, fl); rcx = r; fl = f; }
            // 2e49a movabs r15,0x100000000
            r15 = 0x0_0001_0000_0000u64;
            // 2e4a4 add r15,rcx
            { let (r, f) = add::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 2e4a7 mov rax,r15
            rax = r15;
            // 2e4aa sar rax,0x20
            { let (r, f) = sar::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // 2e4ae mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 2e4b3 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // 2e4b6 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // 2e4bd cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // 2e4c1 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e4c6 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 2e4c9 sar r15,0x1d
            { let (r, f) = sar::<64>(r15, 0x1du64, fl); r15 = r; fl = f; }
            // 2e4cd mov rdi,rax
            rdi = rax;
            // 2e4d0 mov rsi,r15
            rsi = r15;
            // 2e4d3 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e4d8 mov DWORD PTR [rbp-0x3c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x0u64) as u32);
            // 2e4df mov rbx,QWORD PTR [r12+0x10]
            rbx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2e4e4 mov r15,QWORD PTR [r12+0x18]
            r15 = m.r64(r12.wrapping_add(0x18)) as u64;
            // 2e4e9 mov QWORD PTR [rbp-0x38],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r12) as u64);
            // 2e4ed mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 2e4f1 lea rax,[rbp-0x3c]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 2e4f5 mov QWORD PTR [rbp-0x28],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rax) as u64);
            // 2e4f9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e4fb cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 2e4fe je 0x2e51b
            if fl.e() { pc = 0x2e51b; continue; }
            pc = 0x2e500;
        }
        0x2e500 => {
            // block 2e500
            // 2e500 lea r14,[rbp-0x38]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2e504;
        }
        0x2e504 => {
            // block 2e504
            // 2e504 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 2e507 mov rdi,r14
            rdi = r14;
            // 2e50a call 0x2eea0
            { let r = { let t = m.env.code_va(0x2eea0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e50f add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e513 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 2e516 jne 0x2e504
            if fl.ne() { pc = 0x2e504; continue; }
            pc = 0x2e518;
        }
        0x2e518 => {
            // block 2e518
            // 2e518 mov eax,DWORD PTR [rbp-0x3c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64) as u32 as u64;
            pc = 0x2e51b;
        }
        0x2e51b => {
            // block 2e51b
            // 2e51b add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2e51f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e520 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e522 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e524 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e526 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e527 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5bbe2 size=283
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005bbe2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5bbe2;
    loop {
        match pc {
        0x5bbe2 => {
            // block 5bbe2
            // 5bbe2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5bbe3 mov rbp,rsp
            rbp = rsp;
            // 5bbe6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5bbe8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5bbea push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5bbec push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5bbee push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5bbef sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 5bbf3 mov r13d,r9d
            r13 = ((r9 as u32 as u64)) as u32 as u64;
            // 5bbf6 mov DWORD PTR [rbp-0x34],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((r8 as u32 as u64)) as u32);
            // 5bbfa movsd QWORD PTR [rbp-0x40],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x1 as u64);
            // 5bbff movapd xmm1,xmm0
            x1 = x0;
            // 5bc03 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5bc06 mov r12,rsi
            r12 = rsi;
            // 5bc09 mov rbx,rdi
            rbx = rdi;
            // 5bc0c dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bc0e mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5bc13 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5bc15 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 5bc18 je 0x5bce3
            if fl.e() { pc = 0x5bce3; continue; }
            pc = 0x5bc1e;
        }
        0x5bc1e => {
            // block 5bc1e
            // 5bc1e test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // 5bc21 je 0x5bce3
            if fl.e() { pc = 0x5bce3; continue; }
            pc = 0x5bc27;
        }
        0x5bc27 => {
            // block 5bc27
            // 5bc27 divsd xmm1,xmm2
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x2)).to_bits());
            // 5bc2b movsd QWORD PTR [rbp-0x50],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x1 as u64);
            // 5bc30 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bc35 addsd xmm0,QWORD PTR [rip+0xa4b4b] # 0x100788
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(85845)) as u128))).to_bits());
            // 5bc3d movsd QWORD PTR [rbp-0x48],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), x0 as u64);
            // 5bc42 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // 5bc46 xorps xmm0,xmm0
            x0 = 0;
            // 5bc49 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bc4e lea eax,[r13-0x1]
            rax = (r13.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bc52 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 5bc55 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 5bc59 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bc5c;
        }
        0x5bc5c => {
            // block 5bc5c
            // 5bc5c movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // 5bc61 divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 5bc65 mulsd xmm0,QWORD PTR [rip+0xa4b23] # 0x100790
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85853)) as u128))).to_bits());
            // 5bc6d call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.call(t, &[], &[x0 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 5bc72 mulsd xmm0,QWORD PTR [rbp-0x40]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u128))).to_bits());
            // 5bc77 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 5bc7b cvtss2sd xmm0,xmm0
            x0 = set_lo64(x0, (f32_of(x0) as f64).to_bits());
            // 5bc7f mulsd xmm0,QWORD PTR [rbp-0x48]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u128))).to_bits());
            // 5bc84 addsd xmm0,QWORD PTR [rip+0xa2c4c] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 5bc8c cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 5bc91 mov ecx,DWORD PTR [rbp-0x34]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // 5bc94 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 5bc96 je 0x5bc9a
            if fl.e() { pc = 0x5bc9a; continue; }
            pc = 0x5bc98;
        }
        0x5bc98 => {
            // block 5bc98
            // 5bc98 bswap eax
            rax = (rax as u32).swap_bytes() as u64;
            pc = 0x5bc9a;
        }
        0x5bc9a => {
            // block 5bc9a
            // 5bc9a movsd xmm1,QWORD PTR [rbp-0x50]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u128;
            // 5bc9f movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // 5bca4 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 5bca7 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 5bcaa mov rdx,rbx
            rdx = rbx;
            // 5bcad je 0x5bcc1
            if fl.e() { pc = 0x5bcc1; continue; }
            pc = 0x5bcaf;
        }
        0x5bcaf => {
            // block 5bcaf
            // 5bcaf mov DWORD PTR [rdx],eax
            m.w32(rdx, ((rax as u32 as u64)) as u32);
            // 5bcb1 add rdx,0x4
            { let (r, f) = add::<64>(rdx, 0x4u64);
              rdx = r;
              fl = f; }
            // 5bcb5 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bcb7 jne 0x5bcaf
            if fl.ne() { pc = 0x5bcaf; continue; }
            pc = 0x5bcb9;
        }
        0x5bcb9 => {
            // block 5bcb9
            // 5bcb9 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 5bcbd lea rbx,[rbx+rax*4]
            rbx = rbx.wrapping_add(rax.wrapping_mul(4));
            pc = 0x5bcc1;
        }
        0x5bcc1 => {
            // block 5bcc1
            // 5bcc1 addsd xmm0,QWORD PTR [rip+0xa3ebf] # 0xffb88
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(82773)) as u128))).to_bits());
            // 5bcc9 ucomisd xmm0,xmm1
            fl = ucomi(f64_of(x0), f64_of(x1));
            // 5bccd jbe 0x5bcd3
            if fl.be() { pc = 0x5bcd3; continue; }
            pc = 0x5bccf;
        }
        0x5bccf => {
            // block 5bccf
            // 5bccf subsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) - f64_of(x1)).to_bits());
            pc = 0x5bcd3;
        }
        0x5bcd3 => {
            // block 5bcd3
            // 5bcd3 inc DWORD PTR [r12]
            { let (r, f) = inc::<32>(m.r32(r12) as u64, fl); m.w32(r12, (r) as u32); fl = f; }
            // 5bcd7 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 5bcda cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 5bcdd jne 0x5bc5c
            if fl.ne() { pc = 0x5bc5c; continue; }
            pc = 0x5bce3;
        }
        0x5bce3 => {
            // block 5bce3
            // 5bce3 imul r14d,r13d
            { let (r, f) = imul::<32>((r14 as u32 as u64), (r13 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // 5bce7 shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 5bceb mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 5bcee add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 5bcf2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5bcf3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5bcf5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5bcf7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5bcf9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5bcfb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5bcfc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6250 size=284
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006250<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6250;
    loop {
        match pc {
        0x6250 => {
            // block 6250
            // 6250 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6251 mov rbp,rsp
            rbp = rsp;
            // 6254 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6256 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6258 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6259 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 625a mov r14,rdx
            r14 = rdx;
            // 625d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 625f mov r15,rdi
            r15 = rdi;
            // 6262 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6265 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 626b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 626d jne 0x6276
            if fl.ne() { pc = 0x6276; continue; }
            pc = 0x626f;
        }
        0x626f => {
            // block 626f
            // 626f mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 6271 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // 6274 jbe 0x627d
            if fl.be() { pc = 0x627d; continue; }
            pc = 0x6276;
        }
        0x6276 => {
            // block 6276
            // 6276 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6278 jmp 0x633f
            pc = 0x633f; continue;
        }
        0x627d => {
            // block 627d
            // 627d lea rcx,[rip+0xc8] # 0x634c
            rcx = m.env.code_va(0x634c);
            // 6284 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 6288 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 628b jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x628d => {
            // block 628d
            // 628d mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6290 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6295 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 629a mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 62a0 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62a5 => {
            // block 62a5
            // 62a5 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62a8 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62ad mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 62b2 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 62b8 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62ba => {
            // block 62ba
            // 62ba mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62bd mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62c2 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 62c7 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 62cd jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62cf => {
            // block 62cf
            // 62cf mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62d2 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62d7 mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 62dc mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 62e2 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62e4 => {
            // block 62e4
            // 62e4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62e7 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 62ec mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 62f1 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 62f7 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x62f9 => {
            // block 62f9
            // 62f9 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 62fc mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6301 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 6306 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 630c jmp 0x6336
            pc = 0x6336; continue;
        }
        0x630e => {
            // block 630e
            // 630e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6311 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6316 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 631b mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 6321 jmp 0x6336
            pc = 0x6336; continue;
        }
        0x6323 => {
            // block 6323
            // 6323 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 6326 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 632b mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 6330 mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x6336;
        }
        0x6336 => {
            // block 6336
            // 6336 mov rdi,r15
            rdi = r15;
            // 6339 mov rdx,r14
            rdx = r14;
            // 633c call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x633f;
        }
        0x633f => {
            // block 633f
            // 633f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6343 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6344 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6346 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6348 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6349 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5bcfd size=291
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005bcfd<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x2: u128 = fx[2] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x5bcfd;
    loop {
        match pc {
        0x5bcfd => {
            // block 5bcfd
            // 5bcfd push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5bcfe mov rbp,rsp
            rbp = rsp;
            // 5bd01 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5bd03 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5bd05 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5bd07 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5bd09 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5bd0a sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 5bd0e mov r13d,r9d
            r13 = ((r9 as u32 as u64)) as u32 as u64;
            // 5bd11 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 5bd14 movsd QWORD PTR [rbp-0x38],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), x1 as u64);
            // 5bd19 movapd xmm1,xmm0
            x1 = x0;
            // 5bd1d mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 5bd20 mov rbx,rdi
            rbx = rdi;
            // 5bd23 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bd25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5bd2a shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5bd2c test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 5bd2f je 0x5be06
            if fl.e() { pc = 0x5be06; continue; }
            pc = 0x5bd35;
        }
        0x5bd35 => {
            // block 5bd35
            // 5bd35 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // 5bd38 je 0x5be06
            if fl.e() { pc = 0x5be06; continue; }
            pc = 0x5bd3e;
        }
        0x5bd3e => {
            // block 5bd3e
            // 5bd3e divsd xmm1,xmm2
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x2)).to_bits());
            // 5bd42 movsd QWORD PTR [rbp-0x48],xmm1
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), x1 as u64);
            // 5bd47 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bd4c addsd xmm0,QWORD PTR [rip+0xa4a34] # 0x100788
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(85845)) as u128))).to_bits());
            // 5bd54 movsd QWORD PTR [rbp-0x40],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0 as u64);
            // 5bd59 mov eax,DWORD PTR [rsi]
            rax = (m.r32(rsi) as u64) as u32 as u64;
            // 5bd5b mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 5bd5f mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 5bd63 xorps xmm0,xmm0
            x0 = 0;
            // 5bd66 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 5bd6b lea eax,[r13-0x1]
            rax = (r13.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 5bd6f inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 5bd72 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 5bd76 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5bd79;
        }
        0x5bd79 => {
            // block 5bd79
            // 5bd79 movsd QWORD PTR [rbp-0x30],xmm0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0 as u64);
            // 5bd7e divsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x1)).to_bits());
            // 5bd82 mulsd xmm0,QWORD PTR [rip+0xa4a06] # 0x100790
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85853)) as u128))).to_bits());
            // 5bd8a call 0xec290
            { let r = { let t = m.env.stub_va(0xa5d); m.call(t, &[], &[x0 as u64]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 5bd8f mulsd xmm0,QWORD PTR [rbp-0x38]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u128))).to_bits());
            // 5bd94 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 5bd98 cvtss2sd xmm0,xmm0
            x0 = set_lo64(x0, (f32_of(x0) as f64).to_bits());
            // 5bd9c mulsd xmm0,QWORD PTR [rbp-0x40]
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u128))).to_bits());
            // 5bda1 addsd xmm0,QWORD PTR [rip+0xa2b2f] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 5bda9 cvttsd2si eax,xmm0
            rax = (cvtt32(f64_of(x0)) as u64) as u32 as u64;
            // 5bdad test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 5bdb0 je 0x5bdb6
            if fl.e() { pc = 0x5bdb6; continue; }
            pc = 0x5bdb2;
        }
        0x5bdb2 => {
            // block 5bdb2
            // 5bdb2 rol ax,0x8
            { let (r, f) = rol::<16>((rax as u16 as u64), 0x8u64, fl); rax = (rax & !0xffff) | ((r) as u16 as u64); fl = f; }
            pc = 0x5bdb6;
        }
        0x5bdb6 => {
            // block 5bdb6
            // 5bdb6 movsd xmm1,QWORD PTR [rbp-0x48]
            x1 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u128;
            // 5bdbb movsd xmm0,QWORD PTR [rbp-0x30]
            x0 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u128;
            // 5bdc0 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 5bdc3 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // 5bdc6 mov rdx,rbx
            rdx = rbx;
            // 5bdc9 je 0x5bdde
            if fl.e() { pc = 0x5bdde; continue; }
            pc = 0x5bdcb;
        }
        0x5bdcb => {
            // block 5bdcb
            // 5bdcb mov WORD PTR [rdx],ax
            m.w16(rdx, ((rax as u16 as u64)) as u16);
            // 5bdce add rdx,0x2
            { let (r, f) = add::<64>(rdx, 0x2u64);
              rdx = r;
              fl = f; }
            // 5bdd2 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5bdd4 jne 0x5bdcb
            if fl.ne() { pc = 0x5bdcb; continue; }
            pc = 0x5bdd6;
        }
        0x5bdd6 => {
            // block 5bdd6
            // 5bdd6 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 5bdda lea rbx,[rbx+rax*2]
            rbx = rbx.wrapping_add(rax.wrapping_mul(2));
            pc = 0x5bdde;
        }
        0x5bdde => {
            // block 5bdde
            // 5bdde addsd xmm0,QWORD PTR [rip+0xa3da2] # 0xffb88
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(82773)) as u128))).to_bits());
            // 5bde6 ucomisd xmm0,xmm1
            fl = ucomi(f64_of(x0), f64_of(x1));
            // 5bdea jbe 0x5bdf0
            if fl.be() { pc = 0x5bdf0; continue; }
            pc = 0x5bdec;
        }
        0x5bdec => {
            // block 5bdec
            // 5bdec subsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) - f64_of(x1)).to_bits());
            pc = 0x5bdf0;
        }
        0x5bdf0 => {
            // block 5bdf0
            // 5bdf0 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // 5bdf3 cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // 5bdf6 jne 0x5bd79
            if fl.ne() { pc = 0x5bd79; continue; }
            pc = 0x5bdf8;
        }
        0x5bdf8 => {
            // block 5bdf8
            // 5bdf8 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 5bdfc lea eax,[rax+r14*1]
            rax = (rax.wrapping_add(r14)) as u32 as u64;
            // 5be00 mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 5be04 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            pc = 0x5be06;
        }
        0x5be06 => {
            // block 5be06
            // 5be06 imul r14d,r13d
            { let (r, f) = imul::<32>((r14 as u32 as u64), (r13 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // 5be0a shl r14d,0x2
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0x2u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 5be0e mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 5be11 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 5be15 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5be16 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5be18 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 5be1a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5be1c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5be1e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5be1f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x657e size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000657e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x657e;
    loop {
        match pc {
        0x657e => {
            // block 657e
            // 657e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 657f mov rbp,rsp
            rbp = rsp;
            // 6582 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 6584 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 6586 cmp eax,0xf
            fl = sub::<32>((rax as u32 as u64), 0xfu64).1;
            // 6589 jbe 0x6592
            if fl.be() { pc = 0x6592; continue; }
            pc = 0x658b;
        }
        0x658b => {
            // block 658b
            // 658b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 658d jmp 0x6669
            pc = 0x6669; continue;
        }
        0x6592 => {
            // block 6592
            // 6592 lea rdx,[rip+0xd3] # 0x666c
            rdx = m.env.code_va(0x666c);
            // 6599 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // 659d add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // 65a0 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x65a2 => {
            // block 65a2
            // 65a2 mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 65a5 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 65aa mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // 65af mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 65b5 jmp 0x6663
            pc = 0x6663; continue;
        }
        0x65ba => {
            // block 65ba
            // 65ba mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 65bd add eax,0xfffffffe
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffeu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 65c0 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 65c5 mov ecx,0x200
            rcx = (0x200u64) as u32 as u64;
            // 65ca mov r8d,0x9
            r8 = (0x9u64) as u32 as u64;
            // 65d0 jmp 0x6663
            pc = 0x6663; continue;
        }
        0x65d5 => {
            // block 65d5
            // 65d5 mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 65d8 add eax,0xfffffffc
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffcu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 65db mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 65e0 mov ecx,0x400
            rcx = (0x400u64) as u32 as u64;
            // 65e5 mov r8d,0xa
            r8 = (0xau64) as u32 as u64;
            // 65eb jmp 0x6663
            pc = 0x6663; continue;
        }
        0x65ed => {
            // block 65ed
            // 65ed mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 65f0 add eax,0xfffffffa
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffau64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 65f3 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 65f8 mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // 65fd mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 6603 jmp 0x6663
            pc = 0x6663; continue;
        }
        0x6605 => {
            // block 6605
            // 6605 mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 6608 add eax,0xfffffff8
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff8u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 660b mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6610 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 6615 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 661b jmp 0x6663
            pc = 0x6663; continue;
        }
        0x661d => {
            // block 661d
            // 661d mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 6620 add eax,0xfffffff6
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff6u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6623 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6628 mov ecx,0x2000
            rcx = (0x2000u64) as u32 as u64;
            // 662d mov r8d,0xd
            r8 = (0xdu64) as u32 as u64;
            // 6633 jmp 0x6663
            pc = 0x6663; continue;
        }
        0x6635 => {
            // block 6635
            // 6635 mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 6638 add eax,0xfffffff4
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff4u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 663b mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6640 mov ecx,0x4000
            rcx = (0x4000u64) as u32 as u64;
            // 6645 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // 664b jmp 0x6663
            pc = 0x6663; continue;
        }
        0x664d => {
            // block 664d
            // 664d mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // 6650 add eax,0xfffffff2
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffff2u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6653 mov esi,0x178
            rsi = (0x178u64) as u32 as u64;
            // 6658 mov ecx,0x8000
            rcx = (0x8000u64) as u32 as u64;
            // 665d mov r8d,0xf
            r8 = (0xfu64) as u32 as u64;
            pc = 0x6663;
        }
        0x6663 => {
            // block 6663
            // 6663 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 6665 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x6669;
        }
        0x6669 => {
            // block 6669
            // 6669 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 666a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7b00 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007b00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7b00;
    loop {
        match pc {
        0x7b00 => {
            // block 7b00
            // 7b00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7b01 mov rbp,rsp
            rbp = rsp;
            // 7b04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7b06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7b08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7b0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7b0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7b0d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7b11 mov r15,rsi
            r15 = rsi;
            // 7b14 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 7b18 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 7b1b je 0x7c1b
            if fl.e() { pc = 0x7c1b; continue; }
            pc = 0x7b21;
        }
        0x7b21 => {
            // block 7b21
            // 7b21 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 7b25 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 7b2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7b2e mov r14,QWORD PTR [rbp-0x38]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7b32;
        }
        0x7b32 => {
            // block 7b32
            // 7b32 movsd xmm0,QWORD PTR [r15-0x8]
            x0 = m.r64(r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7b38 addsd xmm0,QWORD PTR [rip+0xf6d98] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b40 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 7b45 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 7b47 mov ebx,0x0
            rbx = (0x0u64) as u32 as u64;
            // 7b4c js 0x7b5d
            if fl.s() { pc = 0x7b5d; continue; }
            pc = 0x7b4e;
        }
        0x7b4e => {
            // block 7b4e
            // 7b4e cmp eax,0x3ff
            fl = sub::<32>((rax as u32 as u64), 0x3ffu64).1;
            // 7b53 mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 7b58 cmovg eax,ecx
            if fl.g() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 7b5b mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0x7b5d;
        }
        0x7b5d => {
            // block 7b5d
            // 7b5d mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7b61 lea r12,[rax+0x1ff]
            r12 = rax.wrapping_add(0x1ff);
            // 7b68 movsd xmm0,QWORD PTR [r15]
            x0 = m.r64(r15) as u128;
            // 7b6d addsd xmm0,QWORD PTR [rip+0xf6d63] # 0xfe8d8
            x0 = set_lo64(x0, (f64_of(x0) + f64_of((m.r64(m.env.data_va(77989)) as u128))).to_bits());
            // 7b75 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7b7a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7b7c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7b7e js 0x7b90
            if fl.s() { pc = 0x7b90; continue; }
            pc = 0x7b80;
        }
        0x7b80 => {
            // block 7b80
            // 7b80 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7b86 mov eax,0x3ff
            rax = (0x3ffu64) as u32 as u64;
            // 7b8b cmovg ecx,eax
            if fl.g() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7b8e mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7b90;
        }
        0x7b90 => {
            // block 7b90
            // 7b90 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7b93 shl ebx,0x6
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x6u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 7b96 add ebx,eax
            { let (r, f) = add::<32>((rbx as u32 as u64), (rax as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // 7b98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7b9b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7b9e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ba3 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7ba6 mov rdi,r14
            rdi = r14;
            // 7ba9 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bac mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bae call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bb1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bb3 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bb5;
        }
        0x7bb5 => {
            // block 7bb5
            // 7bb5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bb8 add r12,0x200
            { let (r, f) = add::<64>(r12, 0x200u64);
              r12 = r;
              fl = f; }
            // 7bbf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 7bc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7bc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7bca mov rdi,r14
            rdi = r14;
            // 7bcd mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bd0 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bd2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bd5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7bd7 je 0x7c1e
            if fl.e() { pc = 0x7c1e; continue; }
            pc = 0x7bd9;
        }
        0x7bd9 => {
            // block 7bd9
            // 7bd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 7bdc add r12d,0x200
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x200u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 7be3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7be8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7beb mov rdi,r14
            rdi = r14;
            // 7bee mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 7bf1 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 7bf3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7bf6 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 7bf9 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7bfd cmp rax,0x1ff
            fl = sub::<64>(rax, 0x1ffu64).1;
            // 7c03 ja 0x7c1e
            if fl.a() { pc = 0x7c1e; continue; }
            pc = 0x7c05;
        }
        0x7c05 => {
            // block 7c05
            // 7c05 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // 7c09 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 7c0c mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 7c10 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // 7c13 jne 0x7b32
            if fl.ne() { pc = 0x7b32; continue; }
            pc = 0x7c19;
        }
        0x7c19 => {
            // block 7c19
            // 7c19 jmp 0x7c1e
            pc = 0x7c1e; continue;
        }
        0x7c1b => {
            // block 7c1b
            // 7c1b xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x7c1e;
        }
        0x7c1e => {
            // block 7c1e
            // 7c1e mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 7c21 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7c25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7c26 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7c28 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7c2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7c2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7c2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7c2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5af44 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005af44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5af44;
    loop {
        match pc {
        0x5af44 => {
            // block 5af44
            // 5af44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5af45 mov rbp,rsp
            rbp = rsp;
            // 5af48 cmp esi,0x8
            fl = sub::<32>((rsi as u32 as u64), 0x8u64).1;
            // 5af4b je 0x5af75
            if fl.e() { pc = 0x5af75; continue; }
            pc = 0x5af4d;
        }
        0x5af4d => {
            // block 5af4d
            // 5af4d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5af4f cmp esi,0x2
            fl = sub::<32>((rsi as u32 as u64), 0x2u64).1;
            // 5af52 jne 0x5af85
            if fl.ne() { pc = 0x5af85; continue; }
            pc = 0x5af54;
        }
        0x5af54 => {
            // block 5af54
            // 5af54 cmp edi,0xd
            fl = sub::<32>((rdi as u32 as u64), 0xdu64).1;
            // 5af57 jg 0x5afb0
            if fl.g() { pc = 0x5afb0; continue; }
            pc = 0x5af59;
        }
        0x5af59 => {
            // block 5af59
            // 5af59 add edi,0xfffffffe
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfffffffeu64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 5af5c cmp edi,0x8
            fl = sub::<32>((rdi as u32 as u64), 0x8u64).1;
            // 5af5f ja 0x5b01a
            if fl.a() { pc = 0x5b01a; continue; }
            pc = 0x5af65;
        }
        0x5af65 => {
            // block 5af65
            // 5af65 lea rcx,[rip+0xb0] # 0x5b01c
            rcx = m.env.code_va(0x5b01c);
            // 5af6c movsxd rsi,DWORD PTR [rcx+rdi*4]
            rsi = sx::<32>(m.r32(rcx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // 5af70 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 5af73 jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x5af75 => {
            // block 5af75
            // 5af75 cmp edi,0x5
            fl = sub::<32>((rdi as u32 as u64), 0x5u64).1;
            // 5af78 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 5af7b movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 5af7e add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5af80 jmp 0x5b01a
            pc = 0x5b01a; continue;
        }
        0x5af85 => {
            // block 5af85
            // 5af85 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // 5af88 jne 0x5b01a
            if fl.ne() { pc = 0x5b01a; continue; }
            pc = 0x5af8e;
        }
        0x5af8e => {
            // block 5af8e
            // 5af8e dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5af90 cmp edi,0xc
            fl = sub::<32>((rdi as u32 as u64), 0xcu64).1;
            // 5af93 ja 0x5b01a
            if fl.a() { pc = 0x5b01a; continue; }
            pc = 0x5af99;
        }
        0x5af99 => {
            // block 5af99
            // 5af99 lea rcx,[rip+0xa0] # 0x5b040
            rcx = m.env.code_va(0x5b040);
            // 5afa0 movsxd rsi,DWORD PTR [rcx+rdi*4]
            rsi = sx::<32>(m.r32(rcx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // 5afa4 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 5afa7 jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x5afa9 => {
            // block 5afa9
            // 5afa9 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 5afae jmp 0x5b01a
            pc = 0x5b01a; continue;
        }
        0x5afb0 => {
            // block 5afb0
            // 5afb0 cmp edi,0xe
            fl = sub::<32>((rdi as u32 as u64), 0xeu64).1;
            // 5afb3 jne 0x5b01a
            if fl.ne() { pc = 0x5b01a; continue; }
            pc = 0x5afb5;
        }
        0x5afb5 => {
            // block 5afb5
            // 5afb5 mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // 5afba jmp 0x5b01a
            pc = 0x5b01a; continue;
        }
        0x5afbc => {
            // block 5afbc
            // 5afbc mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5afc1 jmp 0x5b01a
            pc = 0x5b01a; continue;
        }
        0x5afc3 => {
            // block 5afc3
            // 5afc3 and edx,0x1
            { let r = (rdx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5afc6 lea ecx,[rdx+rdx*2]
            rcx = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // 5afc9 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 5afce cmovne eax,ecx
            if fl.ne() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 5afd1 jmp 0x5b01a
            pc = 0x5b01a; continue;
        }
        0x5afd3 => {
            // block 5afd3
            // 5afd3 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5afd6 lea rax,[rip+0xa6ca3] # 0x101c80
            rax = m.env.data_va(91213);
            // 5afdd jmp 0x5b017
            pc = 0x5b017; continue;
        }
        0x5afdf => {
            // block 5afdf
            // 5afdf mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 5afe1 mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // 5afe6 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // 5afea shr rcx,0x22
            { let (r, f) = shr::<64>(rcx, 0x22u64, fl); rcx = r; fl = f; }
            // 5afee lea eax,[rcx+rcx*4]
            rax = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 5aff1 sub edx,eax
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5aff3 lea rax,[rip+0xa6856] # 0x101850
            rax = m.env.data_va(90141);
            // 5affa jmp 0x5b017
            pc = 0x5b017; continue;
        }
        0x5affc => {
            // block 5affc
            // 5affc mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 5affe mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // 5b003 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // 5b007 shr rcx,0x22
            { let (r, f) = shr::<64>(rcx, 0x22u64, fl); rcx = r; fl = f; }
            // 5b00b lea eax,[rcx+rcx*4]
            rax = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // 5b00e sub edx,eax
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5b010 lea rax,[rip+0xa6859] # 0x101870
            rax = m.env.data_va(90173);
            pc = 0x5b017;
        }
        0x5b017 => {
            // block 5b017
            // 5b017 mov eax,DWORD PTR [rax+rdx*4]
            rax = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5b01a;
        }
        0x5b01a => {
            // block 5b01a
            // 5b01a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b01b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eee0 size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eee0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2eee0;
    loop {
        match pc {
        0x2eee0 => {
            // block 2eee0
            // 2eee0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eee1 mov rbp,rsp
            rbp = rsp;
            // 2eee4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2eee6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2eee8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2eee9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2eeea mov r8,QWORD PTR [rdi]
            r8 = m.r64(rdi) as u64;
            // 2eeed mov r9,QWORD PTR [rdi+0x8]
            r9 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2eef1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2eef3 cmp r8,r9
            fl = sub::<64>(r8, r9).1;
            // 2eef6 je 0x2f009
            if fl.e() { pc = 0x2f009; continue; }
            pc = 0x2eefc;
        }
        0x2eefc => {
            // block 2eefc
            // 2eefc mov r15,QWORD PTR [rsi]
            r15 = m.r64(rsi) as u64;
            // 2eeff mov rsi,r15
            rsi = r15;
            // 2ef02 mov rcx,r8
            rcx = r8;
            pc = 0x2ef05;
        }
        0x2ef05 => {
            // block 2ef05
            // 2ef05 movzx ebx,BYTE PTR [rsi]
            rbx = (m.r8(rsi) as u64) as u32 as u64;
            // 2ef08 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // 2ef0b cmp edi,ebx
            fl = sub::<32>((rdi as u32 as u64), (rbx as u32 as u64)).1;
            // 2ef0d jne 0x2ef1f
            if fl.ne() { pc = 0x2ef1f; continue; }
            pc = 0x2ef0f;
        }
        0x2ef0f => {
            // block 2ef0f
            // 2ef0f inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2ef12 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 2ef15 cmp r9,rcx
            fl = sub::<64>(r9, rcx).1;
            // 2ef18 jne 0x2ef05
            if fl.ne() { pc = 0x2ef05; continue; }
            pc = 0x2ef1a;
        }
        0x2ef1a => {
            // block 2ef1a
            // 2ef1a jmp 0x2f009
            pc = 0x2f009; continue;
        }
        0x2ef1f => {
            // block 2ef1f
            // 2ef1f test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 2ef21 je 0x2f007
            if fl.e() { pc = 0x2f007; continue; }
            pc = 0x2ef27;
        }
        0x2ef27 => {
            // block 2ef27
            // 2ef27 mov rbx,r8
            rbx = r8;
            pc = 0x2ef2a;
        }
        0x2ef2a => {
            // block 2ef2a
            // 2ef2a movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 2ef2e movzx ecx,BYTE PTR [rbx]
            rcx = (m.r8(rbx) as u64) as u32 as u64;
            // 2ef31 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 2ef33 jne 0x2ef43
            if fl.ne() { pc = 0x2ef43; continue; }
            pc = 0x2ef35;
        }
        0x2ef35 => {
            // block 2ef35
            // 2ef35 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2ef38 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 2ef3b cmp r9,rbx
            fl = sub::<64>(r9, rbx).1;
            // 2ef3e jne 0x2ef2a
            if fl.ne() { pc = 0x2ef2a; continue; }
            pc = 0x2ef40;
        }
        0x2ef40 => {
            // block 2ef40
            // 2ef40 mov rbx,r9
            rbx = r9;
            pc = 0x2ef43;
        }
        0x2ef43 => {
            // block 2ef43
            // 2ef43 mov r14,rbx
            r14 = rbx;
            // 2ef46 sub r14,r8
            { let (r, f) = sub::<64>(r14, r8);
              r14 = r;
              fl = f; }
            // 2ef49 mov rdi,QWORD PTR [rip+0x111238] # 0x140188
            rdi = m.r64(m.env.data_va(346453)) as u64;
            // 2ef50 lea rsi,[rip+0xfbc3a] # 0x12ab91
            rsi = m.env.data_va(258910);
            // 2ef57 mov edx,0x23
            rdx = (0x23u64) as u32 as u64;
            // 2ef5c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef61 mov rdi,rax
            rdi = rax;
            // 2ef64 mov rsi,r14
            rsi = r14;
            // 2ef67 call 0xec0b0
            { let r = { let t = m.env.stub_va(0x87d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef6c lea rsi,[rip+0xfbc42] # 0x12abb5
            rsi = m.env.data_va(258946);
            // 2ef73 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 2ef78 mov rdi,rax
            rdi = rax;
            // 2ef7b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ef80 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2ef83 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2ef87 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2ef8b and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ef8e or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ef91 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2ef95 mov cl,BYTE PTR [rbx]
            rcx = (rcx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // 2ef97 mov BYTE PTR [rbp-0x19],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7), ((rcx as u8 as u64)) as u8);
            // 2ef9a lea rsi,[rbp-0x19]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe7);
            // 2ef9e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2efa3 mov rdi,rax
            rdi = rax;
            // 2efa6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efab lea rsi,[rip+0xfbc10] # 0x12abc2
            rsi = m.env.data_va(258959);
            // 2efb2 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 2efb7 mov rdi,rax
            rdi = rax;
            // 2efba call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efbf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2efc2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2efc6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2efca and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2efcd or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2efd0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2efd4 mov cl,BYTE PTR [r15]
            rcx = (rcx & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // 2efd7 mov BYTE PTR [rbp-0x1a],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), ((rcx as u8 as u64)) as u8);
            // 2efda lea rsi,[rbp-0x1a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // 2efde mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2efe3 mov rdi,rax
            rdi = rax;
            // 2efe6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efeb lea rsi,[rip+0xfbbdd] # 0x12abcf
            rsi = m.env.data_va(258972);
            // 2eff2 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 2eff7 mov rdi,rax
            rdi = rax;
            // 2effa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2efff mov rdi,rax
            rdi = rax;
            // 2f002 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f007;
        }
        0x2f007 => {
            // block 2f007
            // 2f007 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2f009;
        }
        0x2f009 => {
            // block 2f009
            // 2f009 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2f00d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f00e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f010 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2f012 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f013 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x62a70 size=316
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00062a70<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x62a70;
    loop {
        match pc {
        0x62a70 => {
            // block 62a70
            // 62a70 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 62a71 mov rbp,rsp
            rbp = rsp;
            // 62a74 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 62a75 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 62a76 mov rbx,rdi
            rbx = rdi;
            // 62a79 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 62a7b cmp esi,0x9
            fl = sub::<32>((rsi as u32 as u64), 0x9u64).1;
            // 62a7e jbe 0x62a9c
            if fl.be() { pc = 0x62a9c; continue; }
            pc = 0x62a80;
        }
        0x62a80 => {
            // block 62a80
            // 62a80 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 62a88 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 62a90 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 62a97 jmp 0x62b79
            pc = 0x62b79; continue;
        }
        0x62a9c => {
            // block 62a9c
            // 62a9c lea rcx,[rip+0xe1] # 0x62b84
            rcx = m.env.code_va(0x62b84);
            // 62aa3 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 62aa7 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 62aaa jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x62aac => {
            // block 62aac
            // 62aac lea rax,[rip+0xccc09] # 0x12f6bc
            rax = m.env.data_va(278153);
            // 62ab3 lea rsi,[rip+0xce4a6] # 0x130f60
            rsi = m.env.data_va(284461);
            // 62aba test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62abc cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62ac0 mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 62ac5 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 62aca jmp 0x62b6d
            pc = 0x62b6d; continue;
        }
        0x62acf => {
            // block 62acf
            // 62acf lea rax,[rip+0xccbff] # 0x12f6d5
            rax = m.env.data_va(278178);
            // 62ad6 lea rsi,[rip+0xce49b] # 0x130f78
            rsi = m.env.data_va(284485);
            // 62add jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62adf => {
            // block 62adf
            // 62adf lea rax,[rip+0xcc5c5] # 0x12f0ab
            rax = m.env.data_va(276600);
            // 62ae6 lea rsi,[rip+0xce4a1] # 0x130f8e
            rsi = m.env.data_va(284507);
            // 62aed jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62aef => {
            // block 62aef
            // 62aef lea rax,[rip+0xcc5cc] # 0x12f0c2
            rax = m.env.data_va(276623);
            // 62af6 lea rsi,[rip+0xce4a7] # 0x130fa4
            rsi = m.env.data_va(284529);
            // 62afd jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62aff => {
            // block 62aff
            // 62aff lea rax,[rip+0xcc5d3] # 0x12f0d9
            rax = m.env.data_va(276646);
            // 62b06 lea rsi,[rip+0xce4ad] # 0x130fba
            rsi = m.env.data_va(284551);
            // 62b0d jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62b0f => {
            // block 62b0f
            // 62b0f lea rax,[rip+0xcc5da] # 0x12f0f0
            rax = m.env.data_va(276669);
            // 62b16 lea rsi,[rip+0xce4b3] # 0x130fd0
            rsi = m.env.data_va(284573);
            // 62b1d jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62b1f => {
            // block 62b1f
            // 62b1f lea rax,[rip+0xcc64d] # 0x12f173
            rax = m.env.data_va(276800);
            // 62b26 lea rsi,[rip+0xce4b9] # 0x130fe6
            rsi = m.env.data_va(284595);
            // 62b2d jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62b2f => {
            // block 62b2f
            // 62b2f lea rax,[rip+0xcc654] # 0x12f18a
            rax = m.env.data_va(276823);
            // 62b36 lea rsi,[rip+0xce4bf] # 0x130ffc
            rsi = m.env.data_va(284617);
            // 62b3d jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62b3f => {
            // block 62b3f
            // 62b3f lea rax,[rip+0xcc65b] # 0x12f1a1
            rax = m.env.data_va(276846);
            // 62b46 lea rsi,[rip+0xce4c5] # 0x131012
            rsi = m.env.data_va(284639);
            // 62b4d jmp 0x62b5d
            pc = 0x62b5d; continue;
        }
        0x62b4f => {
            // block 62b4f
            // 62b4f lea rax,[rip+0xcc662] # 0x12f1b8
            rax = m.env.data_va(276869);
            // 62b56 lea rsi,[rip+0xce4cb] # 0x131028
            rsi = m.env.data_va(284661);
            pc = 0x62b5d;
        }
        0x62b5d => {
            // block 62b5d
            // 62b5d test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62b5f cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62b63 mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            // 62b68 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            pc = 0x62b6d;
        }
        0x62b6d => {
            // block 62b6d
            // 62b6d cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 62b71 mov rdi,rbx
            rdi = rbx;
            // 62b74 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x62b79;
        }
        0x62b79 => {
            // block 62b79
            // 62b79 mov rax,rbx
            rax = rbx;
            // 62b7c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 62b80 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 62b81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 62b82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25e9c size=328
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025e9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x25e9c;
    loop {
        match pc {
        0x25e9c => {
            // block 25e9c
            // 25e9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25e9d mov rbp,rsp
            rbp = rsp;
            // 25ea0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25ea2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25ea3 mov r14,rsi
            r14 = rsi;
            // 25ea6 mov rbx,rdi
            rbx = rdi;
            // 25ea9 lea rsi,[rip+0x10454f] # 0x12a3ff
            rsi = m.env.data_va(256972);
            // 25eb0 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 25eb5 mov rdi,r14
            rdi = r14;
            // 25eb8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25ebd movzx esi,WORD PTR [rbx]
            rsi = (m.r16(rbx) as u64) as u32 as u64;
            // 25ec0 mov rdi,rax
            rdi = rax;
            // 25ec3 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25ec8 lea rsi,[rip+0x104537] # 0x12a406
            rsi = m.env.data_va(256979);
            // 25ecf mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 25ed4 mov rdi,rax
            rdi = rax;
            // 25ed7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25edc movzx esi,WORD PTR [rbx+0x2]
            rsi = (m.r16(rbx.wrapping_add(0x2)) as u64) as u32 as u64;
            // 25ee0 mov rdi,rax
            rdi = rax;
            // 25ee3 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25ee8 lea rsi,[rip+0x10451e] # 0x12a40d
            rsi = m.env.data_va(256986);
            // 25eef mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 25ef4 mov rdi,rax
            rdi = rax;
            // 25ef7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25efc mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 25eff mov rdi,rax
            rdi = rax;
            // 25f02 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f07 lea rsi,[rip+0x104506] # 0x12a414
            rsi = m.env.data_va(256993);
            // 25f0e mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 25f13 mov rdi,rax
            rdi = rax;
            // 25f16 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f1b mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25f1f mov rdi,rax
            rdi = rax;
            // 25f22 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f27 lea rsi,[rip+0x1044f5] # 0x12a423
            rsi = m.env.data_va(257008);
            // 25f2e mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 25f33 mov rdi,rax
            rdi = rax;
            // 25f36 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f3b mov rsi,QWORD PTR [rbx+0x10]
            rsi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 25f3f mov rdi,rax
            rdi = rax;
            // 25f42 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f47 lea rsi,[rip+0x1044e4] # 0x12a432
            rsi = m.env.data_va(257023);
            // 25f4e mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 25f53 mov rdi,rax
            rdi = rax;
            // 25f56 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f5b movzx esi,BYTE PTR [rbx+0x18]
            rsi = (m.r8(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 25f5f mov rdi,rax
            rdi = rax;
            // 25f62 call 0xec08c
            { let r = { let t = m.env.stub_va(0x859); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f67 lea rsi,[rip+0x1044cd] # 0x12a43b
            rsi = m.env.data_va(257032);
            // 25f6e mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 25f73 mov rdi,rax
            rdi = rax;
            // 25f76 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f7b movzx esi,BYTE PTR [rbx+0x19]
            rsi = (m.r8(rbx.wrapping_add(0x19)) as u64) as u32 as u64;
            // 25f7f mov rdi,rax
            rdi = rax;
            // 25f82 call 0xec08c
            { let r = { let t = m.env.stub_va(0x859); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f87 lea rsi,[rip+0x1044b6] # 0x12a444
            rsi = m.env.data_va(257041);
            // 25f8e mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 25f93 mov rdi,rax
            rdi = rax;
            // 25f96 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25f9b movzx esi,BYTE PTR [rbx+0x1a]
            rsi = (m.r8(rbx.wrapping_add(0x1a)) as u64) as u32 as u64;
            // 25f9f mov rdi,rax
            rdi = rax;
            // 25fa2 call 0xec08c
            { let r = { let t = m.env.stub_va(0x859); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25fa7 lea rsi,[rip+0x10449e] # 0x12a44c
            rsi = m.env.data_va(257049);
            // 25fae mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 25fb3 mov rdi,rax
            rdi = rax;
            // 25fb6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25fbb movzx esi,BYTE PTR [rbx+0x1b]
            rsi = (m.r8(rbx.wrapping_add(0x1b)) as u64) as u32 as u64;
            // 25fbf mov rdi,rax
            rdi = rax;
            // 25fc2 call 0xec08c
            { let r = { let t = m.env.stub_va(0x859); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25fc7 lea rsi,[rip+0x104486] # 0x12a454
            rsi = m.env.data_va(257057);
            // 25fce mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 25fd3 mov rdi,rax
            rdi = rax;
            // 25fd6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25fdb mov rax,r14
            rax = r14;
            // 25fde pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25fdf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25fe1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25fe2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x62bac size=332
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00062bac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x62bac;
    loop {
        match pc {
        0x62bac => {
            // block 62bac
            // 62bac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 62bad mov rbp,rsp
            rbp = rsp;
            // 62bb0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 62bb1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 62bb2 mov rbx,rdi
            rbx = rdi;
            // 62bb5 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 62bb7 cmp esi,0x9
            fl = sub::<32>((rsi as u32 as u64), 0x9u64).1;
            // 62bba jbe 0x62bd8
            if fl.be() { pc = 0x62bd8; continue; }
            pc = 0x62bbc;
        }
        0x62bbc => {
            // block 62bbc
            // 62bbc mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 62bc4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 62bcc mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 62bd3 jmp 0x62cc3
            pc = 0x62cc3; continue;
        }
        0x62bd8 => {
            // block 62bd8
            // 62bd8 lea rcx,[rip+0xf1] # 0x62cd0
            rcx = m.env.code_va(0x62cd0);
            // 62bdf movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 62be3 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 62be6 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x62be8 => {
            // block 62be8
            // 62be8 lea rax,[rip+0xccacd] # 0x12f6bc
            rax = m.env.data_va(278153);
            // 62bef lea rsi,[rip+0xce448] # 0x13103e
            rsi = m.env.data_va(284683);
            // 62bf6 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62bf8 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62bfc mov eax,0x6
            rax = (0x6u64) as u32 as u64;
            // 62c01 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // 62c06 jmp 0x62cb7
            pc = 0x62cb7; continue;
        }
        0x62c0b => {
            // block 62c0b
            // 62c0b lea rax,[rip+0xccac3] # 0x12f6d5
            rax = m.env.data_va(278178);
            // 62c12 lea rsi,[rip+0xce443] # 0x13105c
            rsi = m.env.data_va(284713);
            // 62c19 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c1e => {
            // block 62c1e
            // 62c1e lea rax,[rip+0xce453] # 0x131078
            rax = m.env.data_va(284741);
            // 62c25 lea rsi,[rip+0xce452] # 0x13107e
            rsi = m.env.data_va(284747);
            // 62c2c jmp 0x62c3c
            pc = 0x62c3c; continue;
        }
        0x62c2e => {
            // block 62c2e
            // 62c2e lea rax,[rip+0xce465] # 0x13109a
            rax = m.env.data_va(284775);
            // 62c35 lea rsi,[rip+0xce464] # 0x1310a0
            rsi = m.env.data_va(284781);
            pc = 0x62c3c;
        }
        0x62c3c => {
            // block 62c3c
            // 62c3c test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62c3e cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62c42 mov eax,0x5
            rax = (0x5u64) as u32 as u64;
            // 62c47 jmp 0x62cb2
            pc = 0x62cb2; continue;
        }
        0x62c49 => {
            // block 62c49
            // 62c49 lea rax,[rip+0xcc489] # 0x12f0d9
            rax = m.env.data_va(276646);
            // 62c50 lea rsi,[rip+0xce465] # 0x1310bc
            rsi = m.env.data_va(284809);
            // 62c57 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c59 => {
            // block 62c59
            // 62c59 lea rax,[rip+0xcc490] # 0x12f0f0
            rax = m.env.data_va(276669);
            // 62c60 lea rsi,[rip+0xce471] # 0x1310d8
            rsi = m.env.data_va(284837);
            // 62c67 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c69 => {
            // block 62c69
            // 62c69 lea rax,[rip+0xcc503] # 0x12f173
            rax = m.env.data_va(276800);
            // 62c70 lea rsi,[rip+0xce47d] # 0x1310f4
            rsi = m.env.data_va(284865);
            // 62c77 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c79 => {
            // block 62c79
            // 62c79 lea rax,[rip+0xcc50a] # 0x12f18a
            rax = m.env.data_va(276823);
            // 62c80 lea rsi,[rip+0xce489] # 0x131110
            rsi = m.env.data_va(284893);
            // 62c87 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c89 => {
            // block 62c89
            // 62c89 lea rax,[rip+0xcc511] # 0x12f1a1
            rax = m.env.data_va(276846);
            // 62c90 lea rsi,[rip+0xce495] # 0x13112c
            rsi = m.env.data_va(284921);
            // 62c97 jmp 0x62ca7
            pc = 0x62ca7; continue;
        }
        0x62c99 => {
            // block 62c99
            // 62c99 lea rax,[rip+0xcc518] # 0x12f1b8
            rax = m.env.data_va(276869);
            // 62ca0 lea rsi,[rip+0xce4a1] # 0x131148
            rsi = m.env.data_va(284949);
            pc = 0x62ca7;
        }
        0x62ca7 => {
            // block 62ca7
            // 62ca7 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 62ca9 cmovne rsi,rax
            if fl.ne() { rsi = rax; }
            // 62cad mov eax,0x4
            rax = (0x4u64) as u32 as u64;
            pc = 0x62cb2;
        }
        0x62cb2 => {
            // block 62cb2
            // 62cb2 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            pc = 0x62cb7;
        }
        0x62cb7 => {
            // block 62cb7
            // 62cb7 cmovne rdx,rax
            if fl.ne() { rdx = rax; }
            // 62cbb mov rdi,rbx
            rdi = rbx;
            // 62cbe call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x62cc3;
        }
        0x62cc3 => {
            // block 62cc3
            // 62cc3 mov rax,rbx
            rax = rbx;
            // 62cc6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 62cca pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 62ccb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 62ccc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5855b size=335
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005855b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5855b;
    loop {
        match pc {
        0x5855b => {
            // block 5855b
            // 5855b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5855c mov rbp,rsp
            rbp = rsp;
            // 5855f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58561 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58563 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 58565 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 58567 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58568 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58569 mov rbx,rsi
            rbx = rsi;
            // 5856c mov r14,rdi
            r14 = rdi;
            // 5856f mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 58572 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58576 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 58579 je 0x58593
            if fl.e() { pc = 0x58593; continue; }
            pc = 0x5857b;
        }
        0x5857b => {
            // block 5857b
            // 5857b lea rsi,[rcx-0x2]
            rsi = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 5857f sub rsi,rdi
            { let (r, f) = sub::<64>(rsi, rdi);
              rsi = r;
              fl = f; }
            // 58582 not rsi
            rsi = !rsi;
            // 58585 and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 58589 add rsi,rcx
            { let (r, f) = add::<64>(rsi, rcx);
              rsi = r;
              fl = f; }
            // 5858c mov QWORD PTR [rbx+0x8],rsi
            m.w64(rbx.wrapping_add(0x8), (rsi) as u64);
            // 58590 mov rdi,rsi
            rdi = rsi;
            pc = 0x58593;
        }
        0x58593 => {
            // block 58593
            // 58593 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 58595 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 58598 je 0x5869b
            if fl.e() { pc = 0x5869b; continue; }
            pc = 0x5859e;
        }
        0x5859e => {
            // block 5859e
            // 5859e cmp edx,0x6
            fl = sub::<32>((rdx as u32 as u64), 0x6u64).1;
            // 585a1 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585a7;
        }
        0x585a7 => {
            // block 585a7
            // 585a7 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 585a9 mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 585ae imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 585b2 shr rdx,0x22
            { let (r, f) = shr::<64>(rdx, 0x22u64, fl); rdx = r; fl = f; }
            // 585b6 add edx,edx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 585b8 lea ecx,[rdx+rdx*2]
            rcx = (rdx.wrapping_add(rdx.wrapping_mul(2))) as u32 as u64;
            // 585bb shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 585be mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 585c0 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 585c3 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585c9;
        }
        0x585c9 => {
            // block 585c9
            // 585c9 mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // 585cb mov r12d,0xaaaaaaab
            r12 = (0xaaaaaaabu64) as u32 as u64;
            // 585d1 imul r12,rcx
            { let (r, f) = imul::<64>(r12, rcx); r12 = r; fl = f; }
            // 585d5 shr r12,0x22
            { let (r, f) = shr::<64>(r12, 0x22u64, fl); r12 = r; fl = f; }
            // 585d9 mov r13d,0x1
            r13 = (0x1u64) as u32 as u64;
            // 585df lea r15,[rbp-0x2a]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 585e3 jmp 0x585ec
            pc = 0x585ec; continue;
        }
        0x585e5 => {
            // block 585e5
            // 585e5 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 585e9 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x585ec;
        }
        0x585ec => {
            // block 585ec
            // 585ec mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 585f1 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 585f3 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 585f9 mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 585fd mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58601 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58604 jae 0x58613
            if fl.ae() { pc = 0x58613; continue; }
            pc = 0x58606;
        }
        0x58606 => {
            // block 58606
            // 58606 mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58609 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5860d mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58611 jmp 0x5862b
            pc = 0x5862b; continue;
        }
        0x58613 => {
            // block 58613
            // 58613 mov rdi,rbx
            rdi = rbx;
            // 58616 mov rsi,r15
            rsi = r15;
            // 58619 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5861e mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 58623 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58627 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5862b;
        }
        0x5862b => {
            // block 5862b
            // 5862b mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 5862d shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58630 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 58636 mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5863a cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 5863d jae 0x5864c
            if fl.ae() { pc = 0x5864c; continue; }
            pc = 0x5863f;
        }
        0x5863f => {
            // block 5863f
            // 5863f mov WORD PTR [rdi],si
            m.w16(rdi, ((rsi as u16 as u64)) as u16);
            // 58642 add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 58646 mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 5864a jmp 0x58665
            pc = 0x58665; continue;
        }
        0x5864c => {
            // block 5864c
            // 5864c mov rdi,rbx
            rdi = rbx;
            // 5864f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 58653 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58658 mov ecx,DWORD PTR [r14+r13*4-0x4]
            rcx = (m.r32(r14.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 5865d mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 58661 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x58665;
        }
        0x58665 => {
            // block 58665
            // 58665 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 58668 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5866e mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 58672 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // 58675 jae 0x58684
            if fl.ae() { pc = 0x58684; continue; }
            pc = 0x58677;
        }
        0x58677 => {
            // block 58677
            // 58677 mov WORD PTR [rdi],cx
            m.w16(rdi, ((rcx as u16 as u64)) as u16);
            // 5867a add rdi,0x2
            { let (r, f) = add::<64>(rdi, 0x2u64);
              rdi = r;
              fl = f; }
            // 5867e mov QWORD PTR [rbx+0x8],rdi
            m.w64(rbx.wrapping_add(0x8), (rdi) as u64);
            // 58682 jmp 0x58690
            pc = 0x58690; continue;
        }
        0x58684 => {
            // block 58684
            // 58684 mov rdi,rbx
            rdi = rbx;
            // 58687 lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 5868b call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x58690;
        }
        0x58690 => {
            // block 58690
            // 58690 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 58693 jb 0x585e5
            if fl.b() { pc = 0x585e5; continue; }
            pc = 0x58699;
        }
        0x58699 => {
            // block 58699
            // 58699 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5869b;
        }
        0x5869b => {
            // block 5869b
            // 5869b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 5869f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 586a0 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 586a2 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 586a4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 586a6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 586a8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 586a9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
