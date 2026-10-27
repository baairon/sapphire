//! Mechanically lifted functions (scripts/lift_run.py), module q009. Each one is checked against the original
//! in the world harness (tests/world_q009.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xeb40 size=138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000eb40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb40;
    loop {
        match pc {
        0xeb40 => {
            // block eb40
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb41 mov rbp,rsp
            rbp = rsp;
            // eb44 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb46 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb49 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // eb4a mov r14,rdx
            r14 = rdx;
            // eb4d mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // eb4f mov r15,rdi
            r15 = rdi;
            // eb52 mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // eb56 call 0xd2f1a
            { let r = { let t = m.env.code_va(0xd2f1a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb5b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb5d je 0xebbc
            if fl.e() { pc = 0xebbc; continue; }
            pc = 0xeb5f;
        }
        0xeb5f => {
            // block eb5f
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb5f mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // eb66 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eb69 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // eb6b lea rdx,[rip+0xf0be2] # 0xff754
            rdx = m.env.data_va(81697);
            // eb72 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // eb75 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // eb79 mov ecx,0x70000
            rcx = (0x70000u64) as u32 as u64;
            // eb7e mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // eb84 mov rdi,r15
            rdi = r15;
            // eb87 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb8a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb8c je 0xebbc
            if fl.e() { pc = 0xebbc; continue; }
            pc = 0xeb8e;
        }
        0xeb8e => {
            // block eb8e
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb8e mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // eb91 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // eb94 je 0xeba4
            if fl.e() { pc = 0xeba4; continue; }
            pc = 0xeb96;
        }
        0xeb96 => {
            // block eb96
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb96 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // eb99 jne 0xebad
            if fl.ne() { pc = 0xebad; continue; }
            pc = 0xeb9b;
        }
        0xeb9b => {
            // block eb9b
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb9b mov DWORD PTR [r14],0x4b00
            m.w32(r14, (0x4b00u64) as u32);
            // eba2 jmp 0xebb8
            pc = 0xebb8; continue;
        }
        0xeba4 => {
            // block eba4
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc eba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eba4 mov DWORD PTR [r14],0x2580
            m.w32(r14, (0x2580u64) as u32);
            // ebab jmp 0xebb8
            pc = 0xebb8; continue;
        }
        0xebad => {
            // block ebad
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebad test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // ebaf jne 0xebbc
            if fl.ne() { pc = 0xebbc; continue; }
            pc = 0xebb1;
        }
        0xebb1 => {
            // block ebb1
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebb1 mov DWORD PTR [r14],0x9600
            m.w32(r14, (0x9600u64) as u32);
            pc = 0xebb8;
        }
        0xebb8 => {
            // block ebb8
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebb8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // ebba jmp 0xebbe
            pc = 0xebbe; continue;
        }
        0xebbc => {
            // block ebbc
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebbc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xebbe;
        }
        0xebbe => {
            // block ebbe
            #[cfg(feature = "trace")] eprintln!("f_0000eb40 pc ebbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ebbe add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ebc2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ebc3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ebc5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ebc7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ebc8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x581da size=130
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000581da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x581da;
    loop {
        match pc {
        0x581da => {
            // block 581da
            #[cfg(feature = "trace")] eprintln!("f_000581da pc 581da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 581da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 581db mov rbp,rsp
            rbp = rsp;
            // 581de push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 581e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 581e2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 581e3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 581e4 mov rax,QWORD PTR [rip+0xe81c5] # 0x1403b0
            rax = m.r64(m.env.data_va(347005)) as u64;
            // 581eb lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 581ef mov QWORD PTR [rdi-0x10],rcx
            m.w64(rdi.wrapping_add(0xffff_ffff_ffff_fff0), (rcx) as u64);
            // 581f3 lea rcx,[rax+0x68]
            rcx = rax.wrapping_add(0x68);
            // 581f7 mov QWORD PTR [rdi+0x70],rcx
            m.w64(rdi.wrapping_add(0x70), (rcx) as u64);
            // 581fb add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 581ff mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 58202 mov rax,QWORD PTR [rip+0xe819f] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // 58209 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 5820d mov QWORD PTR [rdi+0x8],rax
            m.w64(rdi.wrapping_add(0x8), (rax) as u64);
            // 58211 lea r15,[rdi-0x10]
            r15 = rdi.wrapping_add(0xffff_ffff_ffff_fff0);
            // 58215 lea r14,[rdi+0x70]
            r14 = rdi.wrapping_add(0x70);
            // 58219 lea rbx,[rdi+0x8]
            rbx = rdi.wrapping_add(0x8);
            // 5821d lea rdi,[rdi+0x48]
            rdi = rdi.wrapping_add(0x48);
            // 58221 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58226 mov rdi,rbx
            rdi = rbx;
            // 58229 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5822e mov rsi,QWORD PTR [rip+0xe7fc3] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 58235 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 58239 mov rdi,r15
            rdi = r15;
            // 5823c call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58241 mov rdi,r14
            rdi = r14;
            // 58244 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 58249 mov rdi,r15
            rdi = r15;
            // 5824c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58250 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58251 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58253 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58255 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 58256 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14e6c size=128
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014e6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14e6c;
    loop {
        match pc {
        0x14e6c => {
            // block 14e6c
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14e6d mov rbp,rsp
            rbp = rsp;
            // 14e70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14e72 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14e73 mov rbx,rdi
            rbx = rdi;
            // 14e76 mov rax,QWORD PTR [rip+0x12b51b] # 0x140398
            rax = m.r64(m.env.data_va(346981)) as u64;
            // 14e7d add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 14e81 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 14e84 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14e89;
        }
        0x14e89 => {
            // block 14e89
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e89 cmp BYTE PTR [rbx+0x190],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x190)) as u64, 0x0u64).1;
            // 14e90 je 0x14ea0
            if fl.e() { pc = 0x14ea0; continue; }
            pc = 0x14e92;
        }
        0x14e92 => {
            // block 14e92
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e92 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 14e96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14e99 je 0x14ea0
            if fl.e() { pc = 0x14ea0; continue; }
            pc = 0x14e9b;
        }
        0x14e9b => {
            // block 14e9b
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14e9b call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14ea0;
        }
        0x14ea0 => {
            // block 14ea0
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ea0 cmp BYTE PTR [rbx+0x191],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x191)) as u64, 0x0u64).1;
            // 14ea7 je 0x14eb7
            if fl.e() { pc = 0x14eb7; continue; }
            pc = 0x14ea9;
        }
        0x14ea9 => {
            // block 14ea9
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ea9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ea9 mov rdi,QWORD PTR [rbx+0x68]
            rdi = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 14ead test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 14eb0 je 0x14eb7
            if fl.e() { pc = 0x14eb7; continue; }
            pc = 0x14eb2;
        }
        0x14eb2 => {
            // block 14eb2
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14eb2 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x14eb7;
        }
        0x14eb7 => {
            // block 14eb7
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14eb7 mov rdi,rbx
            rdi = rbx;
            // 14eba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14ebb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14ebd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14ebe jmp 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x14ec3 => {
            // block 14ec3
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ec3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ec3 mov rdi,rax
            rdi = rax;
            // 14ec6 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ecb call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14ed0 jmp 0x14e89
            pc = 0x14e89; continue;
        }
        0x14ed2 => {
            // block 14ed2
            #[cfg(feature = "trace")] eprintln!("f_00014e6c pc 14ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14ed2 mov r14,rax
            r14 = rax;
            // 14ed5 mov rdi,rbx
            rdi = rbx;
            // 14ed8 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14edd mov rdi,r14
            rdi = r14;
            // 14ee0 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42d7c size=124
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042d7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42d7c;
    loop {
        match pc {
        0x42d7c => {
            // block 42d7c
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42d7d mov rbp,rsp
            rbp = rsp;
            // 42d80 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42d82 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42d84 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 42d86 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42d87 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42d8a mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // 42d8d mov rbx,rdi
            rbx = rdi;
            // 42d90 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d93 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42d99 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42d9c cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42da3 ja 0x42deb
            if fl.a() { pc = 0x42deb; continue; }
            pc = 0x42da5;
        }
        0x42da5 => {
            // block 42da5
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42da5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42da5 cmp r12d,0x1
            fl = sub::<32>((r12 as u32 as u64), 0x1u64).1;
            // 42da9 jg 0x42deb
            if fl.g() { pc = 0x42deb; continue; }
            pc = 0x42dab;
        }
        0x42dab => {
            // block 42dab
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dab xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42dad jne 0x42deb
            if fl.ne() { pc = 0x42deb; continue; }
            pc = 0x42daf;
        }
        0x42daf => {
            // block 42daf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42daf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42db2 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42db5 mov esi,0x902
            rsi = (0x902u64) as u32 as u64;
            // 42dba mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42dbf xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42dc2 mov rdi,rbx
            rdi = rbx;
            // 42dc5 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42dc8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42dcb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42dcd je 0x42deb
            if fl.e() { pc = 0x42deb; continue; }
            pc = 0x42dcf;
        }
        0x42dcf => {
            // block 42dcf
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42dcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42dd2 mov esi,0x906
            rsi = (0x906u64) as u32 as u64;
            // 42dd7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42ddc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42ddf mov rdi,rbx
            rdi = rbx;
            // 42de2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42de5 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42de8 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x42deb;
        }
        0x42deb => {
            // block 42deb
            #[cfg(feature = "trace")] eprintln!("f_00042d7c pc 42deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42deb mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42dee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42def pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 42df1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42df3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42df5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42df6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc58c2 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c58c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc58c2;
    loop {
        match pc {
        0xc58c2 => {
            // block c58c2
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c58c3 mov rbp,rsp
            rbp = rsp;
            // c58c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c58c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c58ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c58cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c58cc mov rbx,rsi
            rbx = rsi;
            // c58cf mov r14,rdi
            r14 = rdi;
            // c58d2 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c58d7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c58dc mov r15,rax
            r15 = rax;
            // c58df mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // c58e1 mov DWORD PTR [r15+0x1c],eax
            m.w32(r15.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c58e5 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // c58e8 mov DWORD PTR [r15+0x20],ecx
            m.w32(r15.wrapping_add(0x20), ((rcx as u32 as u64)) as u32);
            // c58ec mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // c58f0 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // c58f3 jne 0xc58fe
            if fl.ne() { pc = 0xc58fe; continue; }
            pc = 0xc58f5;
        }
        0xc58f5 => {
            // block c58f5
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58f5 lea rsi,[r14+0x8]
            rsi = r14.wrapping_add(0x8);
            // c58f9 jmp 0xc591c
            pc = 0xc591c; continue;
        }
        0xc58fb => {
            // block c58fb
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58fb mov rsi,rcx
            rsi = rcx;
            pc = 0xc58fe;
        }
        0xc58fe => {
            // block c58fe
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c58fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c58fe cmp eax,DWORD PTR [rsi+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rsi.wrapping_add(0x1c)) as u64).1;
            // c5901 jge 0xc590d
            if fl.ge() { pc = 0xc590d; continue; }
            pc = 0xc5903;
        }
        0xc5903 => {
            // block c5903
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c5903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5903 mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // c5906 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c5909 jne 0xc58fb
            if fl.ne() { pc = 0xc58fb; continue; }
            pc = 0xc590b;
        }
        0xc590b => {
            // block c590b
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c590b jmp 0xc591c
            pc = 0xc591c; continue;
        }
        0xc590d => {
            // block c590d
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c590d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c590d mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // c5911 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // c5914 jne 0xc58fb
            if fl.ne() { pc = 0xc58fb; continue; }
            pc = 0xc5916;
        }
        0xc5916 => {
            // block c5916
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c5916 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5916 lea rdx,[rsi+0x8]
            rdx = rsi.wrapping_add(0x8);
            // c591a jmp 0xc591f
            pc = 0xc591f; continue;
        }
        0xc591c => {
            // block c591c
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c591c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c591c mov rdx,rsi
            rdx = rsi;
            pc = 0xc591f;
        }
        0xc591f => {
            // block c591f
            #[cfg(feature = "trace")] eprintln!("f_000c58c2 pc c591f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c591f mov rdi,r14
            rdi = r14;
            // c5922 mov rcx,r15
            rcx = r15;
            // c5925 call 0xebdda
            { let r = { let t = m.env.stub_va(0x5a7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c592a mov rax,r15
            rax = r15;
            // c592d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c5931 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5932 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5934 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c5936 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5937 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe46ec size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e46ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe46ec;
    loop {
        match pc {
        0xe46ec => {
            // block e46ec
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e46ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e46ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e46ed mov rbp,rsp
            rbp = rsp;
            // e46f0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e46f2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e46f4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e46f5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e46f6 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e46f9 mov rbx,rdi
            rbx = rdi;
            // e46fc call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4701 mov QWORD PTR [rbx+0x2040],rax
            m.w64(rbx.wrapping_add(0x2040), (rax) as u64);
            // e4708 mov r15d,r14d
            r15 = ((r14 as u32 as u64)) as u32 as u64;
            // e470b lea r14,[rbp-0x1c]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            pc = 0xe470f;
        }
        0xe470f => {
            // block e470f
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e470f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e470f mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e4712 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4715 mov esi,DWORD PTR [rbx+0x30]
            rsi = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e4718 add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e471b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e4720 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4723 mov rdx,r14
            rdx = r14;
            // e4726 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4729 mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e472c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e472e test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // e4731 je 0xe4749
            if fl.e() { pc = 0xe4749; continue; }
            pc = 0xe4733;
        }
        0xe4733 => {
            // block e4733
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4733 mov rdi,rbx
            rdi = rbx;
            // e4736 call 0xe4a46
            { let r = { let t = m.env.code_va(0xe4a46); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e473b sub rax,QWORD PTR [rbx+0x2040]
            { let (r, f) = sub::<64>(rax, m.r64(rbx.wrapping_add(0x2040)) as u64);
              rax = r;
              fl = f; }
            // e4742 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e4745 jbe 0xe470f
            if fl.be() { pc = 0xe470f; continue; }
            pc = 0xe4747;
        }
        0xe4747 => {
            // block e4747
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4747 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4747 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xe4749;
        }
        0xe4749 => {
            // block e4749
            #[cfg(feature = "trace")] eprintln!("f_000e46ec pc e4749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4749 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e474d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e474e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4750 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4752 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4753 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6602 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6602<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6602;
    loop {
        match pc {
        0xb6602 => {
            // block b6602
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6602 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6602 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6603 mov rbp,rsp
            rbp = rsp;
            // b6606 mov r8,QWORD PTR [rdi+0x8]
            r8 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b660a add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b660e test r8,r8
            fl = logic::<64>(r8 & r8);
            // b6611 je 0xb665e
            if fl.e() { pc = 0xb665e; continue; }
            pc = 0xb6613;
        }
        0xb6613 => {
            // block b6613
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6613 mov edx,DWORD PTR [rsi]
            rdx = (m.r32(rsi) as u64) as u32 as u64;
            // b6615 mov ecx,DWORD PTR [rsi+0x4]
            rcx = (m.r32(rsi.wrapping_add(0x4)) as u64) as u32 as u64;
            // b6618 mov rax,rdi
            rax = rdi;
            pc = 0xb661b;
        }
        0xb661b => {
            // block b661b
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b661b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b661b mov r9,rax
            r9 = rax;
            // b661e mov rax,r8
            rax = r8;
            pc = 0xb6621;
        }
        0xb6621 => {
            // block b6621
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6621 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6621 mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b6624 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6626 jb 0xb6631
            if fl.b() { pc = 0xb6631; continue; }
            pc = 0xb6628;
        }
        0xb6628 => {
            // block b6628
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6628 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6628 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b662a jb 0xb663c
            if fl.b() { pc = 0xb663c; continue; }
            pc = 0xb662c;
        }
        0xb662c => {
            // block b662c
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b662c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b662c cmp DWORD PTR [rax+0x20],ecx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x20)) as u64, (rcx as u32 as u64)).1;
            // b662f jae 0xb663c
            if fl.ae() { pc = 0xb663c; continue; }
            pc = 0xb6631;
        }
        0xb6631 => {
            // block b6631
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6631 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6631 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // b6635 test rax,rax
            fl = logic::<64>(rax & rax);
            // b6638 jne 0xb6621
            if fl.ne() { pc = 0xb6621; continue; }
            pc = 0xb663a;
        }
        0xb663a => {
            // block b663a
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b663a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b663a jmp 0xb6646
            pc = 0xb6646; continue;
        }
        0xb663c => {
            // block b663c
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b663c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b663c mov r8,QWORD PTR [rax]
            r8 = m.r64(rax) as u64;
            // b663f test r8,r8
            fl = logic::<64>(r8 & r8);
            // b6642 jne 0xb661b
            if fl.ne() { pc = 0xb661b; continue; }
            pc = 0xb6644;
        }
        0xb6644 => {
            // block b6644
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6644 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6644 jmp 0xb6649
            pc = 0xb6649; continue;
        }
        0xb6646 => {
            // block b6646
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6646 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6646 mov rax,r9
            rax = r9;
            pc = 0xb6649;
        }
        0xb6649 => {
            // block b6649
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6649 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6649 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // b664c je 0xb665e
            if fl.e() { pc = 0xb665e; continue; }
            pc = 0xb664e;
        }
        0xb664e => {
            // block b664e
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b664e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b664e mov esi,DWORD PTR [rax+0x1c]
            rsi = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b6651 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // b6653 jb 0xb665e
            if fl.b() { pc = 0xb665e; continue; }
            pc = 0xb6655;
        }
        0xb6655 => {
            // block b6655
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6655 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6655 cmp esi,edx
            fl = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64)).1;
            // b6657 jb 0xb6661
            if fl.b() { pc = 0xb6661; continue; }
            pc = 0xb6659;
        }
        0xb6659 => {
            // block b6659
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6659 cmp ecx,DWORD PTR [rax+0x20]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rax.wrapping_add(0x20)) as u64).1;
            // b665c jae 0xb6661
            if fl.ae() { pc = 0xb6661; continue; }
            pc = 0xb665e;
        }
        0xb665e => {
            // block b665e
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b665e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b665e mov rax,rdi
            rax = rdi;
            pc = 0xb6661;
        }
        0xb6661 => {
            // block b6661
            #[cfg(feature = "trace")] eprintln!("f_000b6602 pc b6661 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6661 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6662 ret
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
            { let r = { let t = m.env.stub_va(0xa57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xc43c4 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c43c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc43c4;
    loop {
        match pc {
        0xc43c4 => {
            // block c43c4
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c43c5 mov rbp,rsp
            rbp = rsp;
            // c43c8 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c43cc test rax,rax
            fl = logic::<64>(rax & rax);
            // c43cf je 0xc4402
            if fl.e() { pc = 0xc4402; continue; }
            pc = 0xc43d1;
        }
        0xc43d1 => {
            // block c43d1
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d1 mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c43d3 jmp 0xc43d8
            pc = 0xc43d8; continue;
        }
        0xc43d5 => {
            // block c43d5
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d5 mov rax,rdx
            rax = rdx;
            pc = 0xc43d8;
        }
        0xc43d8 => {
            // block c43d8
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43d8 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c43db cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c43dd jae 0xc43e9
            if fl.ae() { pc = 0xc43e9; continue; }
            pc = 0xc43df;
        }
        0xc43df => {
            // block c43df
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43df mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c43e2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c43e5 jne 0xc43d5
            if fl.ne() { pc = 0xc43d5; continue; }
            pc = 0xc43e7;
        }
        0xc43e7 => {
            // block c43e7
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43e7 jmp 0xc440e
            pc = 0xc440e; continue;
        }
        0xc43e9 => {
            // block c43e9
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43e9 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c43eb jae 0xc4416
            if fl.ae() { pc = 0xc4416; continue; }
            pc = 0xc43ed;
        }
        0xc43ed => {
            // block c43ed
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43ed mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c43f1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c43f4 jne 0xc43d5
            if fl.ne() { pc = 0xc43d5; continue; }
            pc = 0xc43f6;
        }
        0xc43f6 => {
            // block c43f6
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c43f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c43f6 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c43f9 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c43fd mov rsi,rax
            rsi = rax;
            // c4400 jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc4402 => {
            // block c4402
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4402 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c4406 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c4409 mov rsi,rdi
            rsi = rdi;
            // c440c jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc440e => {
            // block c440e
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c440e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c440e mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c4411 mov rsi,rax
            rsi = rax;
            // c4414 jmp 0xc4419
            pc = 0xc4419; continue;
        }
        0xc4416 => {
            // block c4416
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4416 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc4419;
        }
        0xc4419 => {
            // block c4419
            #[cfg(feature = "trace")] eprintln!("f_000c43c4 pc c4419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4419 mov rax,rsi
            rax = rsi;
            // c441c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c441d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc56c0 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c56c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc56c0;
    loop {
        match pc {
        0xc56c0 => {
            // block c56c0
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c56c1 mov rbp,rsp
            rbp = rsp;
            // c56c4 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c56c8 test rax,rax
            fl = logic::<64>(rax & rax);
            // c56cb je 0xc56fe
            if fl.e() { pc = 0xc56fe; continue; }
            pc = 0xc56cd;
        }
        0xc56cd => {
            // block c56cd
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56cd mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // c56cf jmp 0xc56d4
            pc = 0xc56d4; continue;
        }
        0xc56d1 => {
            // block c56d1
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56d1 mov rax,rdx
            rax = rdx;
            pc = 0xc56d4;
        }
        0xc56d4 => {
            // block c56d4
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56d4 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c56d7 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c56d9 jae 0xc56e5
            if fl.ae() { pc = 0xc56e5; continue; }
            pc = 0xc56db;
        }
        0xc56db => {
            // block c56db
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56db mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // c56de test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c56e1 jne 0xc56d1
            if fl.ne() { pc = 0xc56d1; continue; }
            pc = 0xc56e3;
        }
        0xc56e3 => {
            // block c56e3
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e3 jmp 0xc570a
            pc = 0xc570a; continue;
        }
        0xc56e5 => {
            // block c56e5
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e5 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // c56e7 jae 0xc5712
            if fl.ae() { pc = 0xc5712; continue; }
            pc = 0xc56e9;
        }
        0xc56e9 => {
            // block c56e9
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56e9 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // c56ed test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c56f0 jne 0xc56d1
            if fl.ne() { pc = 0xc56d1; continue; }
            pc = 0xc56f2;
        }
        0xc56f2 => {
            // block c56f2
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56f2 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c56f5 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c56f9 mov rsi,rax
            rsi = rax;
            // c56fc jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc56fe => {
            // block c56fe
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c56fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c56fe add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c5702 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c5705 mov rsi,rdi
            rsi = rdi;
            // c5708 jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc570a => {
            // block c570a
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c570a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c570a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // c570d mov rsi,rax
            rsi = rax;
            // c5710 jmp 0xc5715
            pc = 0xc5715; continue;
        }
        0xc5712 => {
            // block c5712
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c5712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5712 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0xc5715;
        }
        0xc5715 => {
            // block c5715
            #[cfg(feature = "trace")] eprintln!("f_000c56c0 pc c5715 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5715 mov rax,rsi
            rax = rsi;
            // c5718 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c5719 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x31164 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00031164<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31164;
    loop {
        match pc {
        0x31164 => {
            // block 31164
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31164 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31165 mov rbp,rsp
            rbp = rsp;
            // 31168 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3116a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3116b mov rbx,rdi
            rbx = rdi;
            // 3116e lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 31172 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 31176 mov rdi,r14
            rdi = r14;
            // 31179 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3117f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31181 je 0x311b4
            if fl.e() { pc = 0x311b4; continue; }
            pc = 0x31183;
        }
        0x31183 => {
            // block 31183
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31183 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31183 mov rdi,r14
            rdi = r14;
            // 31186 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3118b and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3118e cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31193 jne 0x311b4
            if fl.ne() { pc = 0x311b4; continue; }
            pc = 0x31195;
        }
        0x31195 => {
            // block 31195
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 31195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31195 mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 31199 mov esi,DWORD PTR [rbx+0x2a4]
            rsi = (m.r32(rbx.wrapping_add(0x2a4)) as u64) as u32 as u64;
            // 3119f mov edx,0xe6
            rdx = (0xe6u64) as u32 as u64;
            // 311a4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 311a9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 311ac mov rdi,r14
            rdi = r14;
            // 311af call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 311b2 jmp 0x311b6
            pc = 0x311b6; continue;
        }
        0x311b4 => {
            // block 311b4
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 311b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311b4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x311b6;
        }
        0x311b6 => {
            // block 311b6
            #[cfg(feature = "trace")] eprintln!("f_00031164 pc 311b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311b6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 311b7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 311b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 311ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4383a size=82
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004383a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4383a;
    loop {
        match pc {
        0x4383a => {
            // block 4383a
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4383a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4383a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4383b mov rbp,rsp
            rbp = rsp;
            // 4383e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43840 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43841 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43845 mov r14,rdx
            r14 = rdx;
            // 43848 mov rbx,rsi
            rbx = rsi;
            // 4384b mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 43852 mov DWORD PTR [rbp-0x18],0x8
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x8u64) as u32);
            // 43859 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4385c lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 43860 lea rdx,[rbp-0x18]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 43864 call QWORD PTR [rax+0x890]
            { let r = { let t = m.r64(rax.wrapping_add(0x890)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4386a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4386c je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4386e;
        }
        0x4386e => {
            // block 4386e
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4386e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4386e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 43871 je 0x43878
            if fl.e() { pc = 0x43878; continue; }
            pc = 0x43873;
        }
        0x43873 => {
            // block 43873
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43873 mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // 43876 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x43878;
        }
        0x43878 => {
            // block 43878
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43878 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43878 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 4387b je 0x43883
            if fl.e() { pc = 0x43883; continue; }
            pc = 0x4387d;
        }
        0x4387d => {
            // block 4387d
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 4387d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4387d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // 43880 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x43883;
        }
        0x43883 => {
            // block 43883
            #[cfg(feature = "trace")] eprintln!("f_0004383a pc 43883 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43883 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 43887 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43888 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4388a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4388b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28e5e size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28e5e;
    loop {
        match pc {
        0x28e5e => {
            // block 28e5e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28e5f mov rbp,rsp
            rbp = rsp;
            // 28e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28e65 mov rbx,rsi
            rbx = rsi;
            // 28e68 mov r14,rdi
            r14 = rdi;
            // 28e6b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 28e6e je 0x28e9f
            if fl.e() { pc = 0x28e9f; continue; }
            pc = 0x28e70;
        }
        0x28e70 => {
            // block 28e70
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e70 cmp QWORD PTR [rbx],0x0
            fl = sub::<64>(m.r64(rbx) as u64, 0x0u64).1;
            // 28e74 je 0x28e93
            if fl.e() { pc = 0x28e93; continue; }
            pc = 0x28e76;
        }
        0x28e76 => {
            // block 28e76
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e76 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28e79 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 28e7c je 0x28e93
            if fl.e() { pc = 0x28e93; continue; }
            pc = 0x28e7e;
        }
        0x28e7e => {
            // block 28e7e
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e7e mov rdi,r14
            rdi = r14;
            // 28e81 call 0x28cf2
            { let r = { let t = m.env.code_va(0x28cf2); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e86 mov rdi,r14
            rdi = r14;
            // 28e89 mov rsi,rbx
            rsi = rbx;
            // 28e8c call 0x28dca
            { let r = { let t = m.env.code_va(0x28dca); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e91 jmp 0x28e9f
            pc = 0x28e9f; continue;
        }
        0x28e93 => {
            // block 28e93
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e93 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 28e95 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28e97 mov rdi,r14
            rdi = r14;
            // 28e9a call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28e9f;
        }
        0x28e9f => {
            // block 28e9f
            #[cfg(feature = "trace")] eprintln!("f_00028e5e pc 28e9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28e9f mov rax,r14
            rax = r14;
            // 28ea2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28ea3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28ea5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28ea6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4acc size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00004acc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4acc;
    loop {
        match pc {
        0x4acc => {
            // block 4acc
            #[cfg(feature = "trace")] eprintln!("f_00004acc pc 4acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4acc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4acd mov rbp,rsp
            rbp = rsp;
            // 4ad0 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4ad4 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // 4adb mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4ade lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // 4ae2 call QWORD PTR [rax+0x1e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1e0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4ae8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4aea je 0x4b07
            if fl.e() { pc = 0x4b07; continue; }
            pc = 0x4aec;
        }
        0x4aec => {
            // block 4aec
            #[cfg(feature = "trace")] eprintln!("f_00004acc pc 4aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4aec mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 4aee mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 4af1 lea edx,[rcx-0xeb0b]
            rdx = (rcx.wrapping_add(0xffff_ffff_ffff_14f5)) as u32 as u64;
            // 4af7 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // 4afa jb 0x4b09
            if fl.b() { pc = 0x4b09; continue; }
            pc = 0x4afc;
        }
        0x4afc => {
            // block 4afc
            #[cfg(feature = "trace")] eprintln!("f_00004acc pc 4afc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4afc add ecx,0xffff24f9
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xffff24f9u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 4b02 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 4b05 jb 0x4b09
            if fl.b() { pc = 0x4b09; continue; }
            pc = 0x4b07;
        }
        0x4b07 => {
            // block 4b07
            #[cfg(feature = "trace")] eprintln!("f_00004acc pc 4b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b07 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x4b09;
        }
        0x4b09 => {
            // block 4b09
            #[cfg(feature = "trace")] eprintln!("f_00004acc pc 4b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4b09 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4b0d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4b0e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba49e size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba49e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba49e;
    loop {
        match pc {
        0xba49e => {
            // block ba49e
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba49e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba49f mov rbp,rsp
            rbp = rsp;
            // ba4a2 mov rax,rdi
            rax = rdi;
            // ba4a5 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // ba4a8 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ba4ab je 0xba4d1
            if fl.e() { pc = 0xba4d1; continue; }
            pc = 0xba4ad;
        }
        0xba4ad => {
            // block ba4ad
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4ad mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // ba4b1 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // ba4b4 je 0xba4cb
            if fl.e() { pc = 0xba4cb; continue; }
            pc = 0xba4b6;
        }
        0xba4b6 => {
            // block ba4b6
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4b6 lea rdx,[rcx-0x8]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fff8);
            // ba4ba sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // ba4bd not rdx
            rdx = !rdx;
            // ba4c0 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // ba4c4 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // ba4c7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0xba4cb;
        }
        0xba4cb => {
            // block ba4cb
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4cc jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xba4d1 => {
            // block ba4d1
            #[cfg(feature = "trace")] eprintln!("f_000ba49e pc ba4d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba4d2 ret
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
            { let r = { let t = m.env.stub_va(0x6a3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
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
