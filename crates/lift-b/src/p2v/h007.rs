//! Mechanically lifted functions (scripts/lift_run.py), module h007. Each one is checked against the original
//! in the world harness (tests/world_h007.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xdb438 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000db438<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdb438;
    loop {
        match pc {
        0xdb438 => {
            // block db438
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db438 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // db439 mov rbp,rsp
            rbp = rsp;
            // db43c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // db43e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // db440 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // db442 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // db443 mov r14,rdx
            r14 = rdx;
            // db446 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // db449 mov rbx,rdi
            rbx = rdi;
            // db44c lea r15,[rbx+0x6a0]
            r15 = rbx.wrapping_add(0x6a0);
            // db453 mov rax,QWORD PTR [rbx+0x6a0]
            rax = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // db45a cmp QWORD PTR [rbx+0x6a8],rax
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rax).1;
            // db461 je 0xdb46a
            if fl.e() { pc = 0xdb46a; continue; }
            pc = 0xdb463;
        }
        0xdb463 => {
            // block db463
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db463 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db463 mov QWORD PTR [rbx+0x6a8],rax
            m.w64(rbx.wrapping_add(0x6a8), (rax) as u64);
            pc = 0xdb46a;
        }
        0xdb46a => {
            // block db46a
            #[cfg(feature = "trace")] eprintln!("f_000db438 pc db46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // db46a xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // db46c mov rdi,r15
            rdi = r15;
            // db46f call 0xebed0
            { let r = { let t = m.env.stub_va(0x69d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db474 test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // db477 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // db47a cmp r12d,0x200
            fl = sub::<32>((r12 as u32 as u64), 0x200u64).1;
            // db481 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // db484 and cl,al
            { let r = (rcx as u8 as u64) & (rax as u8 as u64); let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // db486 lea rdi,[rbx+0x380]
            rdi = rbx.wrapping_add(0x380);
            // db48d movzx r8d,cl
            r8 = ((rcx as u8 as u64)) as u32 as u64;
            // db491 movzx edx,r12w
            rdx = ((r12 as u16 as u64)) as u32 as u64;
            // db495 mov rsi,r15
            rsi = r15;
            // db498 mov rcx,r14
            rcx = r14;
            // db49b call 0xe9740
            { let r = { let t = m.env.code_va(0xe9740); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // db4a0 mov eax,DWORD PTR [rbx+0x6a8]
            rax = (m.r32(rbx.wrapping_add(0x6a8)) as u64) as u32 as u64;
            // db4a6 sub eax,DWORD PTR [rbx+0x6a0]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(0x6a0)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // db4ac mov DWORD PTR [rbx+0x338],eax
            m.w32(rbx.wrapping_add(0x338), ((rax as u32 as u64)) as u32);
            // db4b2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // db4b3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // db4b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // db4b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // db4b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // db4ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb720 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb720<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb720;
    loop {
        match pc {
        0xeb720 => {
            // block eb720
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb720 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb720 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb721 mov rbp,rsp
            rbp = rsp;
            // eb724 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb726 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb728 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb72a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb72b mov r15,rsi
            r15 = rsi;
            // eb72e mov r14,rdi
            r14 = rdi;
            // eb731 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // eb734 mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // eb738 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb73b je 0xeb767
            if fl.e() { pc = 0xeb767; continue; }
            pc = 0xeb73d;
        }
        0xeb73d => {
            // block eb73d
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb73d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb73d mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb741;
        }
        0xeb741 => {
            // block eb741
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb741 add rdi,0xffffffffffffffc8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffc8u64);
              rdi = r;
              fl = f; }
            // eb745 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // eb749 mov rsi,rbx
            rsi = rbx;
            // eb74c call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb751 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb755 add rdi,0xffffffffffffffc8
            { let (r, f) = add::<64>(rdi, 0xffff_ffff_ffff_ffc8u64);
              rdi = r;
              fl = f; }
            // eb759 mov QWORD PTR [r15+0x8],rdi
            m.w64(r15.wrapping_add(0x8), (rdi) as u64);
            // eb75d cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // eb760 jne 0xeb741
            if fl.ne() { pc = 0xeb741; continue; }
            pc = 0xeb762;
        }
        0xeb762 => {
            // block eb762
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb762 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // eb765 jmp 0xeb76b
            pc = 0xeb76b; continue;
        }
        0xeb767 => {
            // block eb767
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb767 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb76b;
        }
        0xeb76b => {
            // block eb76b
            #[cfg(feature = "trace")] eprintln!("f_000eb720 pc eb76b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb76b mov QWORD PTR [r14],rdi
            m.w64(r14, (rdi) as u64);
            // eb76e mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // eb772 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // eb776 mov rcx,QWORD PTR [r15+0x10]
            rcx = m.r64(r15.wrapping_add(0x10)) as u64;
            // eb77a mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // eb77e mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // eb782 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // eb786 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // eb78a mov QWORD PTR [r14+0x10],rcx
            m.w64(r14.wrapping_add(0x10), (rcx) as u64);
            // eb78e mov QWORD PTR [r15+0x18],rax
            m.w64(r15.wrapping_add(0x18), (rax) as u64);
            // eb792 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb796 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // eb799 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb79a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb79c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb79e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb7a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb7a1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea43e size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea43e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea43e;
    loop {
        match pc {
        0xea43e => {
            // block ea43e
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea43e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea43e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea43f mov rbp,rsp
            rbp = rsp;
            // ea442 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea444 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea446 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea448 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea44a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea44b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ea44c mov rbx,rdi
            rbx = rdi;
            // ea44f call 0xe0758
            { let r = { let t = m.env.code_va(0xe0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea454 lea rax,[rip+0x5c5a5] # 0x146a00
            rax = m.env.data_va(373197);
            // ea45b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // ea45e mov DWORD PTR [rbx+0x34],0xff
            m.w32(rbx.wrapping_add(0x34), (0xffu64) as u32);
            // ea465 lea rdi,[rbx+0x38]
            rdi = rbx.wrapping_add(0x38);
            // ea469 lea rax,[rip+0x59d58] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea470 lea r12,[rax+0x10]
            r12 = rax.wrapping_add(0x10);
            // ea474 mov QWORD PTR [rbx+0x38],r12
            m.w64(rbx.wrapping_add(0x38), (r12) as u64);
            // ea478 lea r14,[rbx+0x40]
            r14 = rbx.wrapping_add(0x40);
            // ea47c lea r13,[rbx+0x48]
            r13 = rbx.wrapping_add(0x48);
            // ea480 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // ea488 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // ea490 mov QWORD PTR [rbx+0x40],r13
            m.w64(rbx.wrapping_add(0x40), (r13) as u64);
            // ea494 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea497 call 0xec1f4
            { let r = { let t = m.env.stub_va(0x9c1); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea49c mov DWORD PTR [rbx+0x58],eax
            m.w32(rbx.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // ea49f mov DWORD PTR [rbx+0x5c],0xffffffff
            m.w32(rbx.wrapping_add(0x5c), (0xffffffffu64) as u32);
            // ea4a6 mov DWORD PTR [rbx+0x60],0x0
            m.w32(rbx.wrapping_add(0x60), (0x0u64) as u32);
            // ea4ad mov DWORD PTR [rbx+0x64],0x3
            m.w32(rbx.wrapping_add(0x64), (0x3u64) as u32);
            // ea4b4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ea4b8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea4b9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea4bb pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ea4bd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea4bf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea4c1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea4c2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xea4c3 => {
            // block ea4c3
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4c3 mov r15,rax
            r15 = rax;
            // ea4c6 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // ea4ca jmp 0xea4d7
            pc = 0xea4d7; continue;
        }
        0xea4cc => {
            // block ea4cc
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4cc mov r15,rax
            r15 = rax;
            // ea4cf mov QWORD PTR [rbx+0x38],r12
            m.w64(rbx.wrapping_add(0x38), (r12) as u64);
            // ea4d3 mov rsi,QWORD PTR [rbx+0x48]
            rsi = m.r64(rbx.wrapping_add(0x48)) as u64;
            pc = 0xea4d7;
        }
        0xea4d7 => {
            // block ea4d7
            #[cfg(feature = "trace")] eprintln!("f_000ea43e pc ea4d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4d7 mov rdi,r14
            rdi = r14;
            // ea4da call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea4df mov rdi,rbx
            rdi = rbx;
            // ea4e2 call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea4e7 mov rdi,r15
            rdi = r15;
            // ea4ea call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5740 size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5740<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5740;
    loop {
        match pc {
        0xd5740 => {
            // block d5740
            #[cfg(feature = "trace")] eprintln!("f_000d5740 pc d5740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5740 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5741 mov rbp,rsp
            rbp = rsp;
            // d5744 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d5746 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5748 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5749 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d574a mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // d574d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // d5750 mov rbx,rdi
            rbx = rdi;
            // d5753 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5756 call 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d575b mov DWORD PTR [rbx+0x24c],eax
            m.w32(rbx.wrapping_add(0x24c), ((rax as u32 as u64)) as u32);
            // d5761 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5764 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d5767 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d576a call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d576f mov DWORD PTR [rbx+0x248],eax
            m.w32(rbx.wrapping_add(0x248), ((rax as u32 as u64)) as u32);
            // d5775 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // d577c mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // d5783 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d5786 call 0xcb6fa
            { let r = { let t = m.env.code_va(0xcb6fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d578b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d578d je 0xd57ac
            if fl.e() { pc = 0xd57ac; continue; }
            pc = 0xd578f;
        }
        0xd578f => {
            // block d578f
            #[cfg(feature = "trace")] eprintln!("f_000d5740 pc d578f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d578f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5792 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d5796 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // d579b mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // d57a0 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // d57a6 mov rdi,rbx
            rdi = rbx;
            // d57a9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd57ac;
        }
        0xd57ac => {
            // block d57ac
            #[cfg(feature = "trace")] eprintln!("f_000d5740 pc d57ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d57ac mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d57af call 0xcb6a1
            { let r = { let t = m.env.code_va(0xcb6a1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d57b4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d57b6 je 0xd57d5
            if fl.e() { pc = 0xd57d5; continue; }
            pc = 0xd57b8;
        }
        0xd57b8 => {
            // block d57b8
            #[cfg(feature = "trace")] eprintln!("f_000d5740 pc d57b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d57b8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d57bb lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // d57bf mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // d57c4 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // d57c9 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // d57cf mov rdi,rbx
            rdi = rbx;
            // d57d2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd57d5;
        }
        0xd57d5 => {
            // block d57d5
            #[cfg(feature = "trace")] eprintln!("f_000d5740 pc d57d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d57d5 mov DWORD PTR [rbx+0x230],0x0
            m.w32(rbx.wrapping_add(0x230), (0x0u64) as u32);
            // d57df mov QWORD PTR [rbx+0x228],0x0
            m.w64(rbx.wrapping_add(0x228), (0x0u64) as u64);
            // d57ea mov QWORD PTR [rbx+0x220],0x0
            m.w64(rbx.wrapping_add(0x220), (0x0u64) as u64);
            // d57f5 mov QWORD PTR [rbx+0x218],0x0
            m.w64(rbx.wrapping_add(0x218), (0x0u64) as u64);
            // d5800 mov QWORD PTR [rbx+0x210],0x0
            m.w64(rbx.wrapping_add(0x210), (0x0u64) as u64);
            // d580b mov QWORD PTR [rbx+0x240],0x0
            m.w64(rbx.wrapping_add(0x240), (0x0u64) as u64);
            // d5816 mov QWORD PTR [rbx+0x238],0x0
            m.w64(rbx.wrapping_add(0x238), (0x0u64) as u64);
            // d5821 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5825 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5826 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5828 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d582a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d582b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb944a size=242
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b944a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb944a;
    loop {
        match pc {
        0xb944a => {
            // block b944a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b944a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b944a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b944b mov rbp,rsp
            rbp = rsp;
            // b944e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9450 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9452 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b9454 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b9456 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9457 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9458 mov r13,rdx
            r13 = rdx;
            // b945b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b945e mov r14,rdi
            r14 = rdi;
            // b9461 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9465 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b946a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b946c je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9472;
        }
        0xb9472 => {
            // block b9472
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9472 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9472 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9476 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b947b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b947d je 0xb952a
            if fl.e() { pc = 0xb952a; continue; }
            pc = 0xb9483;
        }
        0xb9483 => {
            // block b9483
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9483 mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9487 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b948c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // b948f movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9493 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9496 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9498 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // b949d jbe 0xb952c
            if fl.be() { pc = 0xb952c; continue; }
            pc = 0xb94a3;
        }
        0xb94a3 => {
            // block b94a3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94a3 lea rax,[rip+0x49506] # 0x1029b0
            rax = m.env.data_va(94589);
            // b94aa mov r15d,DWORD PTR [rax+rcx*4]
            r15 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b94ae mov rbx,QWORD PTR [r13+0x0]
            rbx = m.r64(r13) as u64;
            // b94b2 add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            pc = 0xb94b6;
        }
        0xb94b6 => {
            // block b94b6
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94b6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b94b8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb94ba;
        }
        0xb94ba => {
            // block b94ba
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94ba cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // b94bd je 0xb94d5
            if fl.e() { pc = 0xb94d5; continue; }
            pc = 0xb94bf;
        }
        0xb94bf => {
            // block b94bf
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94bf mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // b94c3 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94c6 je 0xb94dc
            if fl.e() { pc = 0xb94dc; continue; }
            pc = 0xb94c8;
        }
        0xb94c8 => {
            // block b94c8
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94c8 mov rcx,rsi
            rcx = rsi;
            // b94cb mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // b94ce test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b94d1 jne 0xb94c8
            if fl.ne() { pc = 0xb94c8; continue; }
            pc = 0xb94d3;
        }
        0xb94d3 => {
            // block b94d3
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d3 jmp 0xb94eb
            pc = 0xb94eb; continue;
        }
        0xb94d5 => {
            // block b94d5
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94d5 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b94d7 mov rbx,r13
            rbx = r13;
            // b94da jmp 0xb94f2
            pc = 0xb94f2; continue;
        }
        0xb94dc => {
            // block b94dc
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94dc mov rsi,rbx
            rsi = rbx;
            pc = 0xb94df;
        }
        0xb94df => {
            // block b94df
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94df mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b94e3 cmp QWORD PTR [rcx],rsi
            fl = sub::<64>(m.r64(rcx) as u64, rsi).1;
            // b94e6 mov rsi,rcx
            rsi = rcx;
            // b94e9 jne 0xb94df
            if fl.ne() { pc = 0xb94df; continue; }
            pc = 0xb94eb;
        }
        0xb94eb => {
            // block b94eb
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94eb mov sil,BYTE PTR [rbx+0x19]
            rsi = (rsi & !0xff) | ((m.r8(rbx.wrapping_add(0x19)) as u64) as u8 as u64);
            // b94ef mov rbx,rcx
            rbx = rcx;
            pc = 0xb94f2;
        }
        0xb94f2 => {
            // block b94f2
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b94f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b94f2 movzx esi,sil
            rsi = ((rsi as u8 as u64)) as u32 as u64;
            // b94f6 lea ecx,[rax*8+0x0]
            rcx = (rax.wrapping_mul(8)) as u32 as u64;
            // b94fd shl esi,cl
            { let (r, f) = shl::<32>((rsi as u32 as u64), (rcx as u8 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // b94ff or edx,esi
            { let r = (rdx as u32 as u64) | (rsi as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b9501 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // b9503 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // b9506 jne 0xb94ba
            if fl.ne() { pc = 0xb94ba; continue; }
            pc = 0xb9508;
        }
        0xb9508 => {
            // block b9508
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9508 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9508 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b950b lea esi,[r12+r15*1]
            rsi = (r12.wrapping_add(r15)) as u32 as u64;
            // b950f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9514 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9517 mov rdi,r14
            rdi = r14;
            // b951a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b951d inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // b9520 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // b9522 cmp r12d,0x5
            fl = sub::<32>((r12 as u32 as u64), 0x5u64).1;
            // b9526 jne 0xb94b6
            if fl.ne() { pc = 0xb94b6; continue; }
            pc = 0xb9528;
        }
        0xb9528 => {
            // block b9528
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b9528 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9528 jmp 0xb952c
            pc = 0xb952c; continue;
        }
        0xb952a => {
            // block b952a
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb952c;
        }
        0xb952c => {
            // block b952c
            #[cfg(feature = "trace")] eprintln!("f_000b944a pc b952c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b952c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b9530 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9531 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b9533 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9535 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9537 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9539 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b953a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7ef4 size=392
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7ef4;
    loop {
        match pc {
        0xe7ef4 => {
            // block e7ef4
            #[cfg(feature = "trace")] eprintln!("f_000e7ef4 pc e7ef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7ef5 mov rbp,rsp
            rbp = rsp;
            // e7ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e7efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e7efc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e7efe push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e7eff mov rbx,rdi
            rbx = rdi;
            // e7f02 lea rax,[rip+0x5ea27] # 0x146930
            rax = m.env.data_va(372989);
            // e7f09 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e7f0c call 0xe80a2
            { let r = { let t = m.env.code_va(0xe80a2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f11 lea rdi,[rbx+0x308]
            rdi = rbx.wrapping_add(0x308);
            // e7f18 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f1d lea rdi,[rbx+0x2f0]
            rdi = rbx.wrapping_add(0x2f0);
            // e7f24 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f29 lea rdi,[rbx+0x2d8]
            rdi = rbx.wrapping_add(0x2d8);
            // e7f30 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f35 lea rdi,[rbx+0x2c0]
            rdi = rbx.wrapping_add(0x2c0);
            // e7f3c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f41 lea rdi,[rbx+0x2a8]
            rdi = rbx.wrapping_add(0x2a8);
            // e7f48 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f4d lea rdi,[rbx+0x290]
            rdi = rbx.wrapping_add(0x290);
            // e7f54 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f59 lea rdi,[rbx+0x268]
            rdi = rbx.wrapping_add(0x268);
            // e7f60 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f65 lea rdi,[rbx+0x248]
            rdi = rbx.wrapping_add(0x248);
            // e7f6c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7f71 lea r14,[rbx+0x8]
            r14 = rbx.wrapping_add(0x8);
            // e7f75 mov rax,QWORD PTR [rip+0x58424] # 0x1403a0
            rax = m.r64(m.env.data_va(346989)) as u64;
            // e7f7c lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e7f80 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e7f84 lea r15,[rbx+0x1b0]
            r15 = rbx.wrapping_add(0x1b0);
            // e7f8b add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e7f8f mov QWORD PTR [rbx+0x1b0],rax
            m.w64(rbx.wrapping_add(0x1b0), (rax) as u64);
            // e7f96 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e7f9a mov rdi,rbx
            rdi = rbx;
            // e7f9d call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7fa2 mov rsi,QWORD PTR [rip+0x58247] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // e7fa9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e7fad mov rdi,r14
            rdi = r14;
            // e7fb0 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7fb5 mov rdi,r15
            rdi = r15;
            // e7fb8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e7fb9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e7fbb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e7fbd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e7fbf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7fc0 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xe7fc5 => {
            // block e7fc5
            #[cfg(feature = "trace")] eprintln!("f_000e7ef4 pc e7fc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7fc5 mov r14,rax
            r14 = rax;
            // e7fc8 lea rdi,[rbx+0x308]
            rdi = rbx.wrapping_add(0x308);
            // e7fcf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7fd4 lea rdi,[rbx+0x2f0]
            rdi = rbx.wrapping_add(0x2f0);
            // e7fdb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7fe0 lea rdi,[rbx+0x2d8]
            rdi = rbx.wrapping_add(0x2d8);
            // e7fe7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7fec lea rdi,[rbx+0x2c0]
            rdi = rbx.wrapping_add(0x2c0);
            // e7ff3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e7ff8 lea rdi,[rbx+0x2a8]
            rdi = rbx.wrapping_add(0x2a8);
            // e7fff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8004 lea rdi,[rbx+0x290]
            rdi = rbx.wrapping_add(0x290);
            // e800b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8010 lea rdi,[rbx+0x268]
            rdi = rbx.wrapping_add(0x268);
            // e8017 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e801c lea rdi,[rbx+0x248]
            rdi = rbx.wrapping_add(0x248);
            // e8023 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8028 mov rax,QWORD PTR [rip+0x58371] # 0x1403a0
            rax = m.r64(m.env.data_va(346989)) as u64;
            // e802f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e8033 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e8037 lea r15,[rbx+0x1b0]
            r15 = rbx.wrapping_add(0x1b0);
            // e803e add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e8042 mov QWORD PTR [rbx+0x1b0],rax
            m.w64(rbx.wrapping_add(0x1b0), (rax) as u64);
            // e8049 lea r12,[rbx+0x8]
            r12 = rbx.wrapping_add(0x8);
            // e804d add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e8051 mov rdi,rbx
            rdi = rbx;
            // e8054 call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8059 mov rsi,QWORD PTR [rip+0x58190] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // e8060 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e8064 mov rdi,r12
            rdi = r12;
            // e8067 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e806c mov rdi,r15
            rdi = r15;
            // e806f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8074 mov rdi,r14
            rdi = r14;
            // e8077 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb4dac size=734
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b4dac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb4dac;
    loop {
        match pc {
        0xb4dac => {
            // block b4dac
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b4dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4dac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b4dad mov rbp,rsp
            rbp = rsp;
            // b4db0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b4db2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b4db4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b4db6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b4db8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b4db9 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b4dc0 mov r12d,r8d
            r12 = ((r8 as u32 as u64)) as u32 as u64;
            // b4dc3 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // b4dc6 mov QWORD PTR [rbp-0x160],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rdi) as u64);
            // b4dcd lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b4dd4 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4ddb mov rax,QWORD PTR [rip+0x8b3ee] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b4de2 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b4de6 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // b4ded add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b4df1 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // b4df8 mov rsi,rbx
            rsi = rbx;
            // b4dfb call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4e00 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b4e08 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // b4e0f mov r15,QWORD PTR [rip+0x8b5aa] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // b4e16 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // b4e1a mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b4e21 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b4e28 add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // b4e2c mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b4e33 mov rdi,rbx
            rdi = rbx;
            // b4e36 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4e3b mov r13,QWORD PTR [rip+0x8b566] # 0x1403a8
            r13 = m.r64(m.env.data_va(346997)) as u64;
            // b4e42 add r13,0x10
            { let (r, f) = add::<64>(r13, 0x10u64);
              r13 = r;
              fl = f; }
            // b4e46 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b4e4d mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b4e58 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b4e63 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b4e6e mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b4e79 mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // b4e83 xorps xmm0,xmm0
            x0 = 0;
            // b4e86 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b4e8a mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b4e92 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b4e96 mov rdi,rbx
            rdi = rbx;
            // b4e99 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4e9e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b4ea2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ea7 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // b4eaa call 0xcd01e
            { let r = { let t = m.env.code_va(0xcd01e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4eaf test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b4eb1 je 0xb4f7e
            if fl.e() { pc = 0xb4f7e; continue; }
            pc = 0xb4eb7;
        }
        0xb4eb7 => {
            // block b4eb7
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b4eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4eb7 lea rsi,[rip+0x80491] # 0x13534f
            rsi = m.env.data_va(301852);
            // b4ebe lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b4ec5 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // b4eca call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ecf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4ed2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4ed6 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b4eda and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4edd or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4ee0 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b4ee4 movzx esi,r14w
            rsi = ((r14 as u16 as u64)) as u32 as u64;
            // b4ee8 mov rdi,rax
            rdi = rax;
            // b4eeb call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ef0 mov r12,rax
            r12 = rax;
            // b4ef3 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // b4ef7 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4efb add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // b4efe lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4f02 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f07 mov rsi,QWORD PTR [rip+0x8b282] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4f0e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4f12 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f17 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4f1a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4f1e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b4f23 mov rdi,rax
            rdi = rax;
            // b4f26 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f28 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4f2a lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4f2e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f33 movsx esi,bl
            rsi = (sx::<8>((rbx as u8 as u64)) as u64) as u32 as u64;
            // b4f36 mov rdi,r12
            rdi = r12;
            // b4f39 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f3e mov rdi,r12
            rdi = r12;
            // b4f41 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f46 lea rsi,[rip+0x8040b] # 0x135358
            rsi = m.env.data_va(301861);
            // b4f4d mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // b4f52 mov rdi,r12
            rdi = r12;
            // b4f55 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f5a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4f5d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4f61 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b4f65 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4f68 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b4f6b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b4f6f shr r14d,0x10
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // b4f73 mov rdi,rax
            rdi = rax;
            // b4f76 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b4f79 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4f7e;
        }
        0xb4f7e => {
            // block b4f7e
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b4f7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4f7e mov rdi,QWORD PTR [rbp-0x160]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b4f85 lea rsi,[rbp-0x150]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4f8c call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f91 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b4f98 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b4f9f mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b4fa6 mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            // b4fad lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b4fb4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4fb9 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b4fc0 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4fc5 mov rsi,QWORD PTR [rip+0x8b23c] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b4fcc add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4fd0 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b4fd7 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4fdc lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b4fe3 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4fe8 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // b4fef add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // b4ff6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b4ff7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b4ff9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b4ffb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b4ffd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b4fff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5000 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb5001 => {
            // block b5001
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b5001 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5001 mov rbx,rax
            rbx = rax;
            // b5004 jmp 0xb502a
            pc = 0xb502a; continue;
        }
        0xb5006 => {
            // block b5006
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b5006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5006 mov rbx,rax
            rbx = rax;
            // b5009 jmp 0xb5075
            pc = 0xb5075; continue;
        }
        0xb500b => {
            // block b500b
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b500b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b500b mov rbx,rax
            rbx = rax;
            // b500e jmp 0xb505e
            pc = 0xb505e; continue;
        }
        0xb5010 => {
            // block b5010
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b5010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5010 mov rbx,rax
            rbx = rax;
            // b5013 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b5017 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b501c jmp 0xb5046
            pc = 0xb5046; continue;
        }
        0xb501e => {
            // block b501e
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b501e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b501e mov rbx,rax
            rbx = rax;
            // b5021 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b5025 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb502a;
        }
        0xb502a => {
            // block b502a
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b502a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b502a mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // b5031 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // b5038 mov QWORD PTR [rbp-0xe8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (r15) as u64);
            // b503f mov QWORD PTR [rbp-0x150],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r13) as u64);
            pc = 0xb5046;
        }
        0xb5046 => {
            // block b5046
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b5046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5046 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // b504d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5052 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // b5059 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb505e;
        }
        0xb505e => {
            // block b505e
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b505e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b505e mov rsi,QWORD PTR [rip+0x8b1a3] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b5065 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b5069 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // b5070 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb5075;
        }
        0xb5075 => {
            // block b5075
            #[cfg(feature = "trace")] eprintln!("f_000b4dac pc b5075 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5075 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // b507c call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5081 mov rdi,rbx
            rdi = rbx;
            // b5084 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7f30 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7f30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7f30;
    loop {
        match pc {
        0xb7f30 => {
            // block b7f30
            #[cfg(feature = "trace")] eprintln!("f_000b7f30 pc b7f30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7f31 mov rbp,rsp
            rbp = rsp;
            // b7f34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b7f36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7f38 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7f39 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b7f3a mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // b7f3d mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // b7f40 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b7f42 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7f45 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7f4b mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b7f4d mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b7f4f mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // b7f52 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // b7f55 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b7f59 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7f5a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7f5c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7f5e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7f5f jmp 0x5b332
            { let r = { let t = m.env.code_va(0x5b332); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea3fc size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea3fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea3fc;
    loop {
        match pc {
        0xea3fc => {
            // block ea3fc
            #[cfg(feature = "trace")] eprintln!("f_000ea3fc pc ea3fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea3fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea3fd mov rbp,rsp
            rbp = rsp;
            // ea400 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea402 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea403 mov edi,0xe8
            rdi = (0xe8u64) as u32 as u64;
            // ea408 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea40d mov rbx,rax
            rbx = rax;
            // ea410 mov rdi,rbx
            rdi = rbx;
            // ea413 call 0xea43e
            { let r = { let t = m.env.code_va(0xea43e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea418 mov rax,rbx
            rax = rbx;
            // ea41b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea41c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea41e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea41f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xea420 => {
            // block ea420
            #[cfg(feature = "trace")] eprintln!("f_000ea3fc pc ea420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea420 mov r14,rax
            r14 = rax;
            // ea423 mov rdi,rbx
            rdi = rbx;
            // ea426 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea42b mov rdi,r14
            rdi = r14;
            // ea42e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6c84 size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6c84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6c84;
    loop {
        match pc {
        0xd6c84 => {
            // block d6c84
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c85 mov rbp,rsp
            rbp = rsp;
            // d6c88 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6c89 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d6c8a mov rbx,rdi
            rbx = rdi;
            // d6c8d lea rax,[rip+0x6d77c] # 0x144410
            rax = m.env.data_va(363485);
            // d6c94 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d6c97 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // d6c9e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6ca1 je 0xd6ca8
            if fl.e() { pc = 0xd6ca8; continue; }
            pc = 0xd6ca3;
        }
        0xd6ca3 => {
            // block d6ca3
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ca3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ca3 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ca8;
        }
        0xd6ca8 => {
            // block d6ca8
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ca8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ca8 mov rdi,QWORD PTR [rbx+0x290]
            rdi = m.r64(rbx.wrapping_add(0x290)) as u64;
            // d6caf test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cb2 je 0xd6cb9
            if fl.e() { pc = 0xd6cb9; continue; }
            pc = 0xd6cb4;
        }
        0xd6cb4 => {
            // block d6cb4
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cb4 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cb9;
        }
        0xd6cb9 => {
            // block d6cb9
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cb9 mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d6cc0 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cc3 je 0xd6ccb
            if fl.e() { pc = 0xd6ccb; continue; }
            pc = 0xd6cc5;
        }
        0xd6cc5 => {
            // block d6cc5
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cc5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6cc8 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ccb;
        }
        0xd6ccb => {
            // block d6ccb
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ccb mov rdi,QWORD PTR [rbx+0x6a0]
            rdi = m.r64(rbx.wrapping_add(0x6a0)) as u64;
            // d6cd2 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6cd5 je 0xd6cec
            if fl.e() { pc = 0xd6cec; continue; }
            pc = 0xd6cd7;
        }
        0xd6cd7 => {
            // block d6cd7
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cd7 cmp QWORD PTR [rbx+0x6a8],rdi
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rdi).1;
            // d6cde je 0xd6ce7
            if fl.e() { pc = 0xd6ce7; continue; }
            pc = 0xd6ce0;
        }
        0xd6ce0 => {
            // block d6ce0
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ce0 mov QWORD PTR [rbx+0x6a8],rdi
            m.w64(rbx.wrapping_add(0x6a8), (rdi) as u64);
            pc = 0xd6ce7;
        }
        0xd6ce7 => {
            // block d6ce7
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6ce7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ce7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cec;
        }
        0xd6cec => {
            // block d6cec
            #[cfg(feature = "trace")] eprintln!("f_000d6c84 pc d6cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6cec lea rdi,[rbx+0x380]
            rdi = rbx.wrapping_add(0x380);
            // d6cf3 call 0xe807c
            { let r = { let t = m.env.code_va(0xe807c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6cf8 lea rdi,[rbx+0x318]
            rdi = rbx.wrapping_add(0x318);
            // d6cff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d04 lea rdi,[rbx+0x300]
            rdi = rbx.wrapping_add(0x300);
            // d6d0b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d10 lea rdi,[rbx+0x2e8]
            rdi = rbx.wrapping_add(0x2e8);
            // d6d17 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d1c lea rdi,[rbx+0x2d0]
            rdi = rbx.wrapping_add(0x2d0);
            // d6d23 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d28 lea rdi,[rbx+0x2b8]
            rdi = rbx.wrapping_add(0x2b8);
            // d6d2f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d34 lea rdi,[rbx+0x2a0]
            rdi = rbx.wrapping_add(0x2a0);
            // d6d3b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6d40 mov rdi,rbx
            rdi = rbx;
            // d6d43 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d6d47 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6d48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d49 jmp 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd668c size=382
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d668c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd668c;
    loop {
        match pc {
        0xd668c => {
            // block d668c
            #[cfg(feature = "trace")] eprintln!("f_000d668c pc d668c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d668c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d668d mov rbp,rsp
            rbp = rsp;
            // d6690 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6692 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6694 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d6696 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6698 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6699 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d669d mov rbx,rdi
            rbx = rdi;
            // d66a0 call 0xb66ac
            { let r = { let t = m.env.code_va(0xb66ac); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d66a5 lea rax,[rip+0x6dd64] # 0x144410
            rax = m.env.data_va(363485);
            // d66ac mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d66af lea r14,[rbx+0x2a0]
            r14 = rbx.wrapping_add(0x2a0);
            // d66b6 mov DWORD PTR [rbx+0x37c],0x0
            m.w32(rbx.wrapping_add(0x37c), (0x0u64) as u32);
            // d66c0 lea r15,[rbx+0x380]
            r15 = rbx.wrapping_add(0x380);
            // d66c7 mov DWORD PTR [rbx+0x298],0x0
            m.w32(rbx.wrapping_add(0x298), (0x0u64) as u32);
            // d66d1 mov QWORD PTR [rbx+0x290],0x0
            m.w64(rbx.wrapping_add(0x290), (0x0u64) as u64);
            // d66dc mov QWORD PTR [rbx+0x288],0x0
            m.w64(rbx.wrapping_add(0x288), (0x0u64) as u64);
            // d66e7 mov esi,0x90
            rsi = (0x90u64) as u32 as u64;
            // d66ec mov rdi,r14
            rdi = r14;
            // d66ef call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d66f4 mov BYTE PTR [rbx+0x378],0x0
            m.w8(rbx.wrapping_add(0x378), (0x0u64) as u8);
            // d66fb mov DWORD PTR [rbx+0x374],0x0
            m.w32(rbx.wrapping_add(0x374), (0x0u64) as u32);
            // d6705 mov QWORD PTR [rbx+0x36c],0x0
            m.w64(rbx.wrapping_add(0x36c), (0x0u64) as u64);
            // d6710 mov QWORD PTR [rbx+0x364],0x0
            m.w64(rbx.wrapping_add(0x364), (0x0u64) as u64);
            // d671b mov QWORD PTR [rbx+0x35c],0x0
            m.w64(rbx.wrapping_add(0x35c), (0x0u64) as u64);
            // d6726 mov QWORD PTR [rbx+0x354],0x0
            m.w64(rbx.wrapping_add(0x354), (0x0u64) as u64);
            // d6731 mov QWORD PTR [rbx+0x34c],0x0
            m.w64(rbx.wrapping_add(0x34c), (0x0u64) as u64);
            // d673c mov QWORD PTR [rbx+0x344],0x0
            m.w64(rbx.wrapping_add(0x344), (0x0u64) as u64);
            // d6747 mov QWORD PTR [rbx+0x33c],0x0
            m.w64(rbx.wrapping_add(0x33c), (0x0u64) as u64);
            // d6752 mov QWORD PTR [rbx+0x334],0x0
            m.w64(rbx.wrapping_add(0x334), (0x0u64) as u64);
            // d675d mov rdi,r15
            rdi = r15;
            // d6760 call 0xe7eea
            { let r = { let t = m.env.code_va(0xe7eea); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6765 mov QWORD PTR [rbx+0x6b8],0x0
            m.w64(rbx.wrapping_add(0x6b8), (0x0u64) as u64);
            // d6770 mov QWORD PTR [rbx+0x6b0],0x0
            m.w64(rbx.wrapping_add(0x6b0), (0x0u64) as u64);
            // d677b mov QWORD PTR [rbx+0x6a8],0x0
            m.w64(rbx.wrapping_add(0x6a8), (0x0u64) as u64);
            // d6786 mov QWORD PTR [rbx+0x6a0],0x0
            m.w64(rbx.wrapping_add(0x6a0), (0x0u64) as u64);
            // d6791 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d6795 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6796 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6798 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d679a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d679c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d679e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d679f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd67a0 => {
            // block d67a0
            #[cfg(feature = "trace")] eprintln!("f_000d668c pc d67a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d67a0 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d67a4 lea rdi,[rbx+0x318]
            rdi = rbx.wrapping_add(0x318);
            // d67ab lea r12,[rbx+0x300]
            r12 = rbx.wrapping_add(0x300);
            // d67b2 lea r13,[rbx+0x2e8]
            r13 = rbx.wrapping_add(0x2e8);
            // d67b9 lea r15,[rbx+0x2d0]
            r15 = rbx.wrapping_add(0x2d0);
            // d67c0 lea rax,[rbx+0x2b8]
            rax = rbx.wrapping_add(0x2b8);
            // d67c7 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d67cb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67d0 mov rdi,r12
            rdi = r12;
            // d67d3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67d8 mov rdi,r13
            rdi = r13;
            // d67db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67e0 mov rdi,r15
            rdi = r15;
            // d67e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67e8 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d67ec call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67f1 mov rdi,r14
            rdi = r14;
            // d67f4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d67f9 mov rdi,rbx
            rdi = rbx;
            // d67fc call 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6801 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // d6805 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7368 size=328
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7368<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7368;
    loop {
        match pc {
        0xd7368 => {
            // block d7368
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7368 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7369 mov rbp,rsp
            rbp = rsp;
            // d736c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d736e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7370 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7372 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7373 mov r12,rdi
            r12 = rdi;
            // d7376 lea eax,[rsi-0x1]
            rax = (rsi.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d7379 cmp eax,0x6
            fl = sub::<32>((rax as u32 as u64), 0x6u64).1;
            // d737c ja 0xd7398
            if fl.a() { pc = 0xd7398; continue; }
            pc = 0xd737e;
        }
        0xd737e => {
            // block d737e
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d737e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d737e lea rcx,[rip+0x10f] # 0xd7494
            rcx = m.env.code_va(0xd7494);
            // d7385 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // d7389 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // d738c jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd738e => {
            // block d738e
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d738e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d738e lea rax,[r12+0x344]
            rax = r12.wrapping_add(0x344);
            // d7396 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7398 => {
            // block d7398
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7398 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7398 lea rax,[r12+0x340]
            rax = r12.wrapping_add(0x340);
            pc = 0xd73a0;
        }
        0xd73a0 => {
            // block d73a0
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d73a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d73a0 mov r15d,DWORD PTR [rax]
            r15 = (m.r32(rax) as u64) as u32 as u64;
            // d73a3 mov rdi,r12
            rdi = r12;
            // d73a6 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73ab mov edi,0x100
            rdi = (0x100u64) as u32 as u64;
            // d73b0 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73b5 mov r14,rax
            r14 = rax;
            // d73b8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd73ba;
        }
        0xd73ba => {
            // block d73ba
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d73ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d73ba mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d73be lea edx,[r15+rbx*1]
            rdx = (r15.wrapping_add(rbx)) as u32 as u64;
            // d73c2 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d73c7 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d73cc xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d73cf mov rdi,r12
            rdi = r12;
            // d73d2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73d5 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d73d9 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d73de mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // d73e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d73e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d73eb mov rdi,r12
            rdi = r12;
            // d73ee call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73f1 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d73f5 mov rdi,r12
            rdi = r12;
            // d73f8 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d73fe mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d7402 lea rdx,[r14+rbx*1]
            rdx = r14.wrapping_add(rbx);
            // d7406 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d740b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7410 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7413 mov rdi,r12
            rdi = r12;
            // d7416 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7419 add rbx,0x4
            { let (r, f) = add::<64>(rbx, 0x4u64);
              rbx = r;
              fl = f; }
            // d741d cmp rbx,0x100
            fl = sub::<64>(rbx, 0x100u64).1;
            // d7424 jne 0xd73ba
            if fl.ne() { pc = 0xd73ba; continue; }
            pc = 0xd7426;
        }
        0xd7426 => {
            // block d7426
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7426 mov rdi,r12
            rdi = r12;
            // d7429 mov rsi,r14
            rsi = r14;
            // d742c call 0xd7110
            { let r = { let t = m.env.code_va(0xd7110); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7431 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d7434 mov rdi,r14
            rdi = r14;
            // d7437 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d743c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d743e mov rdi,r12
            rdi = r12;
            // d7441 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7446 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // d7449 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d744a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d744c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d744e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7450 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7451 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd7452 => {
            // block d7452
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7452 lea rax,[r12+0x354]
            rax = r12.wrapping_add(0x354);
            // d745a jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd745f => {
            // block d745f
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d745f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d745f lea rax,[r12+0x358]
            rax = r12.wrapping_add(0x358);
            // d7467 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd746c => {
            // block d746c
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d746c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d746c lea rax,[r12+0x348]
            rax = r12.wrapping_add(0x348);
            // d7474 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7479 => {
            // block d7479
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7479 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7479 lea rax,[r12+0x34c]
            rax = r12.wrapping_add(0x34c);
            // d7481 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7486 => {
            // block d7486
            #[cfg(feature = "trace")] eprintln!("f_000d7368 pc d7486 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7486 lea rax,[r12+0x350]
            rax = r12.wrapping_add(0x350);
            // d748e jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7764 size=948
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000d7764<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7764;
    loop {
        match pc {
        0xd7764 => {
            // block d7764
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7764 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7764 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7765 mov rbp,rsp
            rbp = rsp;
            // d7768 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d776a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d776c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d776e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7770 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7771 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d7772 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d7774 mov r15,rdi
            r15 = rdi;
            // d7777 cmp QWORD PTR [r15+0x288],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x288)) as u64, 0x0u64).1;
            // d777f je 0xd7aa6
            if fl.e() { pc = 0xd7aa6; continue; }
            pc = 0xd7785;
        }
        0xd7785 => {
            // block d7785
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7785 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7788 mov rdi,r15
            rdi = r15;
            // d778b call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7791 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7793 je 0xd7ab9
            if fl.e() { pc = 0xd7ab9; continue; }
            pc = 0xd7799;
        }
        0xd7799 => {
            // block d7799
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7799 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7799 mov eax,DWORD PTR [r15+0x370]
            rax = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d77a0 lea ecx,[rax-0x1]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d77a3 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // d77a6 ja 0xd77d1
            if fl.a() { pc = 0xd77d1; continue; }
            pc = 0xd77a8;
        }
        0xd77a8 => {
            // block d77a8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77a8 lea rdx,[rip+0x34d] # 0xd7afc
            rdx = m.env.code_va(0xd7afc);
            // d77af movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // d77b3 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // d77b6 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd77b8 => {
            // block d77b8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77b8 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77bb mov r12d,DWORD PTR [r15+0x344]
            r12 = (m.r32(r15.wrapping_add(0x344)) as u64) as u32 as u64;
            // d77c2 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77c5 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d77ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // d77cf jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd77d1 => {
            // block d77d1
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77d1 mov r12d,DWORD PTR [r15+0x340]
            r12 = (m.r32(r15.wrapping_add(0x340)) as u64) as u32 as u64;
            // d77d8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d77da je 0xd77e1
            if fl.e() { pc = 0xd77e1; continue; }
            pc = 0xd77dc;
        }
        0xd77dc => {
            // block d77dc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77dc mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77df jmp 0xd784f
            pc = 0xd784f; continue;
        }
        0xd77e1 => {
            // block d77e1
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77e1 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77e4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77e7 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d77ec xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d77ee jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd77f0 => {
            // block d77f0
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d77f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d77f0 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77f3 mov r12d,DWORD PTR [r15+0x354]
            r12 = (m.r32(r15.wrapping_add(0x354)) as u64) as u32 as u64;
            // d77fa mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d77fd mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d7802 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7807 jmp 0xd7820
            pc = 0xd7820; continue;
        }
        0xd7809 => {
            // block d7809
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7809 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7809 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d780c mov r12d,DWORD PTR [r15+0x358]
            r12 = (m.r32(r15.wrapping_add(0x358)) as u64) as u32 as u64;
            // d7813 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7816 mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d781b mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            pc = 0xd7820;
        }
        0xd7820 => {
            // block d7820
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7820 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7820 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7825 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7828 mov rdi,r15
            rdi = r15;
            // d782b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d782e jmp 0xd784f
            pc = 0xd784f; continue;
        }
        0xd7830 => {
            // block d7830
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7830 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7830 lea rax,[r15+0x348]
            rax = r15.wrapping_add(0x348);
            // d7837 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7839 => {
            // block d7839
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7839 lea rax,[r15+0x34c]
            rax = r15.wrapping_add(0x34c);
            // d7840 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7842 => {
            // block d7842
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7842 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7842 lea rax,[r15+0x350]
            rax = r15.wrapping_add(0x350);
            pc = 0xd7849;
        }
        0xd7849 => {
            // block d7849
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7849 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d784c mov r12d,DWORD PTR [rax]
            r12 = (m.r32(rax) as u64) as u32 as u64;
            pc = 0xd784f;
        }
        0xd784f => {
            // block d784f
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d784f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d784f mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7856 mov rdi,r15
            rdi = r15;
            // d7859 call 0xd7b18
            { let r = { let t = m.env.code_va(0xd7b18); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d785e mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7865 mov rdi,r15
            rdi = r15;
            // d7868 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d786d mov rbx,QWORD PTR [r15+0x288]
            rbx = m.r64(r15.wrapping_add(0x288)) as u64;
            // d7874 mov r14d,0x100
            r14 = (0x100u64) as u32 as u64;
            // d787a add r14d,DWORD PTR [r15+0x298]
            { let (r, f) = add::<32>((r14 as u32 as u64), m.r32(r15.wrapping_add(0x298)) as u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d7881 shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // d7885 mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // d7889 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d788c mov esi,0x2882
            rsi = (0x2882u64) as u32 as u64;
            // d7891 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // d7896 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d789b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d789e mov rdi,r15
            rdi = r15;
            // d78a1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78a4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d78a7 mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // d78ac mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d78b1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d78b4 mov rdi,r15
            rdi = r15;
            // d78b7 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d78ba call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78bd test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // d78c0 je 0xd797c
            if fl.e() { pc = 0xd797c; continue; }
            pc = 0xd78c6;
        }
        0xd78c6 => {
            // block d78c6
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78c6 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d78c9 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd78cc;
        }
        0xd78cc => {
            // block d78cc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78cc mov edi,DWORD PTR [r15+0x10]
            rdi = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // d78d0 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d78d5 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d78d7 je 0xd7903
            if fl.e() { pc = 0xd7903; continue; }
            pc = 0xd78d9;
        }
        0xd78d9 => {
            // block d78d9
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78d9 cmp r12d,DWORD PTR [r15+0x338]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r15.wrapping_add(0x338)) as u64).1;
            // d78e0 jne 0xd7903
            if fl.ne() { pc = 0xd7903; continue; }
            pc = 0xd78e2;
        }
        0xd78e2 => {
            // block d78e2
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78e2 cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d78ea jne 0xd78f3
            if fl.ne() { pc = 0xd78f3; continue; }
            pc = 0xd78ec;
        }
        0xd78ec => {
            // block d78ec
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78ec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d78f1 jmp 0xd78f8
            pc = 0xd78f8; continue;
        }
        0xd78f3 => {
            // block d78f3
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78f3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd78f8;
        }
        0xd78f8 => {
            // block d78f8
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d78f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d78f8 mov rdi,r15
            rdi = r15;
            // d78fb call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7900 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7903;
        }
        0xd7903 => {
            // block d7903
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7903 mov rdi,r15
            rdi = r15;
            // d7906 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // d7909 mov rdx,rbx
            rdx = rbx;
            // d790c call 0xd7e14
            { let r = { let t = m.env.code_va(0xd7e14); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7911 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7914 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // d7919 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d791e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7921 mov rdi,r15
            rdi = r15;
            // d7924 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // d7927 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d792a cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7932 jne 0xd795d
            if fl.ne() { pc = 0xd795d; continue; }
            pc = 0xd7934;
        }
        0xd7934 => {
            // block d7934
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7934 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7936 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // d7939 div DWORD PTR [rbp-0x2c]
            { let (q, r) = div::<32>(rdx, rax, m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64); rax = q; rdx = r; }
            // d793c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d793e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7940 lea rdi,[rip+0x61402] # 0x138d49
            rdi = m.env.data_va(316694);
            // d7947 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d7949 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d794e mov rax,QWORD PTR [rip+0x68a8b] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d7955 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d7958 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd795d;
        }
        0xd795d => {
            // block d795d
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d795d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d795d inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // d7960 add r12d,0x100
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x100u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d7967 add rbx,0x100
            { let (r, f) = add::<64>(rbx, 0x100u64);
              rbx = r;
              fl = f; }
            // d796e add r14d,0x64
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x64u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d7972 cmp r13d,DWORD PTR [rbp-0x2c]
            fl = sub::<32>((r13 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64).1;
            // d7976 jb 0xd78cc
            if fl.b() { pc = 0xd78cc; continue; }
            pc = 0xd797c;
        }
        0xd797c => {
            // block d797c
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d797c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d797c cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7984 jne 0xd7994
            if fl.ne() { pc = 0xd7994; continue; }
            pc = 0xd7986;
        }
        0xd7986 => {
            // block d7986
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7986 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7986 lea rdi,[rip+0x613d2] # 0x138d5f
            rdi = m.env.data_va(316716);
            // d798d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d798f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd7994;
        }
        0xd7994 => {
            // block d7994
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7994 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7997 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d799c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d79a1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79a6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79a9 mov rdi,r15
            rdi = r15;
            // d79ac call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79af mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79b2 mov rdi,r15
            rdi = r15;
            // d79b5 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79bb mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79be mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d79c3 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // d79c8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79d0 mov rdi,r15
            rdi = r15;
            // d79d3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79d6 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79d9 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d79de mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d79e3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d79e8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d79eb mov rdi,r15
            rdi = r15;
            // d79ee call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79f1 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d79f4 mov rdi,r15
            rdi = r15;
            // d79f7 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d79fd mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // d7a00 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7a02 je 0xd7a25
            if fl.e() { pc = 0xd7a25; continue; }
            pc = 0xd7a04;
        }
        0xd7a04 => {
            // block d7a04
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7a04 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7a06 mov rdi,r15
            rdi = r15;
            // d7a09 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a0e mov esi,DWORD PTR [r15+0x370]
            rsi = (m.r32(r15.wrapping_add(0x370)) as u64) as u32 as u64;
            // d7a15 mov rdi,r15
            rdi = r15;
            // d7a18 call 0xd7eca
            { let r = { let t = m.env.code_va(0xd7eca); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a1d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7a1f je 0xd7acc
            if fl.e() { pc = 0xd7acc; continue; }
            pc = 0xd7a25;
        }
        0xd7a25 => {
            // block d7a25
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7a25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7a25 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a28 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7a2d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7a32 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a37 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a3a mov rdi,r15
            rdi = r15;
            // d7a3d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a40 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a43 mov rdi,r15
            rdi = r15;
            // d7a46 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a4c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a4f mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d7a54 mov edx,0x9c
            rdx = (0x9cu64) as u32 as u64;
            // d7a59 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a5e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a61 mov rdi,r15
            rdi = r15;
            // d7a64 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a67 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a6a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7a6f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d7a74 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7a79 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7a7c mov rdi,r15
            rdi = r15;
            // d7a7f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a82 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d7a85 mov rdi,r15
            rdi = r15;
            // d7a88 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7a8e xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7a90 mov rdi,r15
            rdi = r15;
            // d7a93 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7a97 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7a98 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7a9a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7a9c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7a9e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7aa0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7aa1 jmp 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xd7aa6 => {
            // block d7aa6
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7aa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7aa6 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7aab call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ab0 lea rcx,[rip+0x615e9] # 0x1390a0
            rcx = m.env.data_va(317549);
            // d7ab7 jmp 0xd7ae7
            pc = 0xd7ae7; continue;
        }
        0xd7ab9 => {
            // block d7ab9
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7ab9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ab9 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7abe call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ac3 lea rcx,[rip+0x61656] # 0x139120
            rcx = m.env.data_va(317677);
            // d7aca jmp 0xd7ae7
            pc = 0xd7ae7; continue;
        }
        0xd7acc => {
            // block d7acc
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7acc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7ace mov rdi,r15
            rdi = r15;
            // d7ad1 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ad6 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7adb call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ae0 lea rcx,[rip+0x612a1] # 0x138d88
            rcx = m.env.data_va(316757);
            pc = 0xd7ae7;
        }
        0xd7ae7 => {
            // block d7ae7
            #[cfg(feature = "trace")] eprintln!("f_000d7764 pc d7ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7ae7 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // d7aea mov rsi,QWORD PTR [rip+0x686e7] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // d7af1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7af3 mov rdi,rax
            rdi = rax;
            // d7af6 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6886 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6886<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6886;
    loop {
        match pc {
        0xb6886 => {
            // block b6886
            #[cfg(feature = "trace")] eprintln!("f_000b6886 pc b6886 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6886 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6887 mov rbp,rsp
            rbp = rsp;
            // b688a movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b688d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b688e jmp 0xb672c
            { let r = { let t = m.env.code_va(0xb672c); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeafd6 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eafd6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeafd6;
    loop {
        match pc {
        0xeafd6 => {
            // block eafd6
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eafd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafd6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eafd7 mov rbp,rsp
            rbp = rsp;
            // eafda push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eafdc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eafdd sub rsp,0x290
            { let (r, f) = sub::<64>(rsp, 0x290u64);
              rsp = r;
              fl = f; }
            // eafe4 mov rbx,rdi
            rbx = rdi;
            // eafe7 lea r14,[rbp-0x298]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eafee mov rdi,r14
            rdi = r14;
            // eaff1 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaff6 mov edi,DWORD PTR [rbx+0x5c]
            rdi = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eaff9 mov rsi,r14
            rsi = r14;
            // eaffc call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb001 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb003 je 0xeb013
            if fl.e() { pc = 0xeb013; continue; }
            pc = 0xeb005;
        }
        0xeb005 => {
            // block eb005
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb005 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb005 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eb008 mov rax,QWORD PTR [rax+0x48]
            rax = m.r64(rax.wrapping_add(0x48)) as u64;
            // eb00c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eb00e mov rdi,rbx
            rdi = rbx;
            // eb011 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb013;
        }
        0xeb013 => {
            // block eb013
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb013 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb013 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eb01a call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb01f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb021 add rsp,0x290
            { let (r, f) = add::<64>(rsp, 0x290u64);
              rsp = r;
              fl = f; }
            // eb028 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb029 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb02b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb02c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb02d => {
            // block eb02d
            #[cfg(feature = "trace")] eprintln!("f_000eafd6 pc eb02d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb02d mov rbx,rax
            rbx = rax;
            // eb030 lea rdi,[rbp-0x298]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd68);
            // eb037 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb03c mov rdi,rbx
            rdi = rbx;
            // eb03f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb122 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb122<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb122;
    loop {
        match pc {
        0xeb122 => {
            // block eb122
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb122 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb122 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb123 mov rbp,rsp
            rbp = rsp;
            // eb126 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb128 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb12a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb12b sub rsp,0x288
            { let (r, f) = sub::<64>(rsp, 0x288u64);
              rsp = r;
              fl = f; }
            // eb132 mov r14,rdi
            r14 = rdi;
            // eb135 lea rbx,[rbp-0x2a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb13c mov rdi,rbx
            rdi = rbx;
            // eb13f call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb144 mov edi,DWORD PTR [r14+0x5c]
            rdi = (m.r32(r14.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eb148 mov rsi,rbx
            rsi = rbx;
            // eb14b call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb150 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb152 je 0xeb1fd
            if fl.e() { pc = 0xeb1fd; continue; }
            pc = 0xeb158;
        }
        0xeb158 => {
            // block eb158
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb158 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb158 lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb15f call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb164 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // eb166 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb16b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eb16d je 0xeb180
            if fl.e() { pc = 0xeb180; continue; }
            pc = 0xeb16f;
        }
        0xeb16f => {
            // block eb16f
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb16f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb16f movzx esi,BYTE PTR [r14+0x68]
            rsi = (m.r8(r14.wrapping_add(0x68)) as u64) as u32 as u64;
            // eb174 lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb17b call 0xe8cc
            { let r = { let t = m.env.code_va(0xe8cc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb180;
        }
        0xeb180 => {
            // block eb180
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb180 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb180 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eb184 cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eb188 jg 0xeb1b2
            if fl.g() { pc = 0xeb1b2; continue; }
            pc = 0xeb18a;
        }
        0xeb18a => {
            // block eb18a
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb18a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb18a lea r15,[rbp-0x2a0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            pc = 0xeb191;
        }
        0xeb191 => {
            // block eb191
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb191 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // eb193 mov edx,DWORD PTR [r14+rax*4+0x6c]
            rdx = (m.r32(r14.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x6c)) as u64) as u32 as u64;
            // eb198 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // eb19d mov rdi,r15
            rdi = r15;
            // eb1a0 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // eb1a2 call 0x23fc
            { let r = { let t = m.env.code_va(0x23fc); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb1a7 cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eb1ab lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // eb1ae mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // eb1b0 jl 0xeb191
            if fl.l() { pc = 0xeb191; continue; }
            pc = 0xeb1b2;
        }
        0xeb1b2 => {
            // block eb1b2
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb1b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb1b2 lea rsi,[r14+0x38]
            rsi = r14.wrapping_add(0x38);
            // eb1b6 lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb1bd mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // eb1c2 call 0x8fb8
            { let r = { let t = m.env.code_va(0x8fb8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb1c7 mov esi,DWORD PTR [r14+0x60]
            rsi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eb1cb lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb1d2 call 0x680ca
            { let r = { let t = m.env.code_va(0x680ca); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb1d7 mov esi,DWORD PTR [r14+0x34]
            rsi = (m.r32(r14.wrapping_add(0x34)) as u64) as u32 as u64;
            // eb1db lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb1e2 call 0x0
            { let r = { let t = m.env.code_va(0x0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb1e7 mov esi,DWORD PTR [rip+0xd6803] # 0x1c19f0
            rsi = (m.r32(m.env.data_va(876989)) as u64) as u32 as u64;
            // eb1ed mov edx,DWORD PTR [r14+0x58]
            rdx = (m.r32(r14.wrapping_add(0x58)) as u64) as u32 as u64;
            // eb1f1 lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb1f8 call 0x49316
            { let r = { let t = m.env.code_va(0x49316); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeb1fd;
        }
        0xeb1fd => {
            // block eb1fd
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb1fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb1fd lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb204 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb209 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // eb20b add rsp,0x288
            { let (r, f) = add::<64>(rsp, 0x288u64);
              rsp = r;
              fl = f; }
            // eb212 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb213 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb215 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb217 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb218 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb219 => {
            // block eb219
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb219 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb219 jmp 0xeb21b
            pc = 0xeb21b; continue;
        }
        0xeb21b => {
            // block eb21b
            #[cfg(feature = "trace")] eprintln!("f_000eb122 pc eb21b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb21b mov rbx,rax
            rbx = rax;
            // eb21e lea rdi,[rbp-0x2a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // eb225 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb22a mov rdi,rbx
            rdi = rbx;
            // eb22d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeab9c size=1082
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eab9c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeab9c;
    loop {
        match pc {
        0xeab9c => {
            // block eab9c
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eab9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eab9c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eab9d mov rbp,rsp
            rbp = rsp;
            // eaba0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eaba2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eaba4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eaba6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eaba8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eaba9 sub rsp,0x2a8
            { let (r, f) = sub::<64>(rsp, 0x2a8u64);
              rsp = r;
              fl = f; }
            // eabb0 mov rbx,rsi
            rbx = rsi;
            // eabb3 mov r14,rdi
            r14 = rdi;
            // eabb6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // eabb9 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabbf mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // eabc2 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // eabc5 jne 0xeafa3
            if fl.ne() { pc = 0xeafa3; continue; }
            pc = 0xeabcb;
        }
        0xeabcb => {
            // block eabcb
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabcb mov rbx,QWORD PTR [rbx+0x30]
            rbx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // eabcf xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // eabd2 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // eabd5 je 0xeafa3
            if fl.e() { pc = 0xeafa3; continue; }
            pc = 0xeabdb;
        }
        0xeabdb => {
            // block eabdb
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabdb lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eabe2 mov rdi,r12
            rdi = r12;
            // eabe5 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabea mov edi,DWORD PTR [r14+0x5c]
            rdi = (m.r32(r14.wrapping_add(0x5c)) as u64) as u32 as u64;
            // eabee mov rsi,r12
            rsi = r12;
            // eabf1 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eabf6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eabf8 je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeabfe;
        }
        0xeabfe => {
            // block eabfe
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eabfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eabfe mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // eac00 mov DWORD PTR [rbp-0x2b4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), ((rax as u32 as u64)) as u32);
            // eac06 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // eac08 je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeac0e;
        }
        0xeac0e => {
            // block eac0e
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac0e lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac15 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac1a mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // eac1d mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac20 call 0xcc7a3
            { let r = { let t = m.env.code_va(0xcc7a3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac25 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac27 je 0xeac47
            if fl.e() { pc = 0xeac47; continue; }
            pc = 0xeac29;
        }
        0xeac29 => {
            // block eac29
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac29 lea rsi,[r14+0x68]
            rsi = r14.wrapping_add(0x68);
            // eac2d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac34 call 0xe90c
            { let r = { let t = m.env.code_va(0xe90c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac39 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eac40 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eac42 call 0xe8cc
            { let r = { let t = m.env.code_va(0xe8cc); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeac47;
        }
        0xeac47 => {
            // block eac47
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac47 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac4a call 0xcc147
            { let r = { let t = m.env.code_va(0xcc147); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac4f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac51 jne 0xeac5b
            if fl.ne() { pc = 0xeac5b; continue; }
            pc = 0xeac53;
        }
        0xeac53 => {
            // block eac53
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac53 mov DWORD PTR [r14+0x60],0x1
            m.w32(r14.wrapping_add(0x60), (0x1u64) as u32);
            pc = 0xeac5b;
        }
        0xeac5b => {
            // block eac5b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac5b mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eac5e mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eac64 call 0xc8c60
            { let r = { let t = m.env.code_va(0xc8c60); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac69 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eac6b je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeac71;
        }
        0xeac71 => {
            // block eac71
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac71 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eac75 cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eac79 jg 0xeacca
            if fl.g() { pc = 0xeacca; continue; }
            pc = 0xeac7b;
        }
        0xeac7b => {
            // block eac7b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac7b lea r13,[rbp-0x2b0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xeac82;
        }
        0xeac82 => {
            // block eac82
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eac82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eac82 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eac84 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eac86 mov rdi,r13
            rdi = r13;
            // eac89 mov esi,DWORD PTR [rbp-0x2b4]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eac8f mov r8d,ebx
            r8 = ((rbx as u32 as u64)) as u32 as u64;
            // eac92 call 0x78
            { let r = { let t = m.env.code_va(0x78); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eac97 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eac99 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eac9b mov rdi,r13
            rdi = r13;
            // eac9e mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // eaca0 call 0x50a0
            { let r = { let t = m.env.code_va(0x50a0); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaca5 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // eaca8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eacaa mov rdi,r13
            rdi = r13;
            // eacad call 0xe70c
            { let r = { let t = m.env.code_va(0xe70c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacb2 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // eacb5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eacb7 mov rdi,r13
            rdi = r13;
            // eacba call 0xe7ec
            { let r = { let t = m.env.code_va(0xe7ec); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacbf cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // eacc3 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // eacc6 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // eacc8 jl 0xeac82
            if fl.l() { pc = 0xeac82; continue; }
            pc = 0xeacca;
        }
        0xeacca => {
            // block eacca
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eacca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eacca lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eacd1 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // eacd6 call 0x1d1a
            { let r = { let t = m.env.code_va(0x1d1a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacdb mov r13d,0x2
            r13 = (0x2u64) as u32 as u64;
            // eace1 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // eace6 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eace9 call 0xc9e94
            { let r = { let t = m.env.code_va(0xc9e94); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eacf0 jne 0xead06
            if fl.ne() { pc = 0xead06; continue; }
            pc = 0xeacf2;
        }
        0xeacf2 => {
            // block eacf2
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eacf2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eacf2 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // eacf7 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eacfa call 0xc9e94
            { let r = { let t = m.env.code_va(0xc9e94); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eacff movzx r13d,al
            r13 = ((rax as u8 as u64)) as u32 as u64;
            // ead03 inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            pc = 0xead06;
        }
        0xead06 => {
            // block ead06
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead06 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead0a cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // ead0e jg 0xead61
            if fl.g() { pc = 0xead61; continue; }
            pc = 0xead10;
        }
        0xead10 => {
            // block ead10
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead10 mov DWORD PTR [rbp-0x2b8],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd48), ((r12 as u32 as u64)) as u32);
            // ead17 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xead1e;
        }
        0xead1e => {
            // block ead1e
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead1e mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // ead20 lea rdx,[r14+rax*4+0x6c]
            rdx = r14.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x6c);
            // ead25 mov rdi,r12
            rdi = r12;
            // ead28 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead2a call 0x2522
            { let r = { let t = m.env.code_va(0x2522); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead2f mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // ead34 mov rdi,r12
            rdi = r12;
            // ead37 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead39 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // ead3c call 0x23fc
            { let r = { let t = m.env.code_va(0x23fc); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead41 mov rdi,r12
            rdi = r12;
            // ead44 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead46 call 0x2964
            { let r = { let t = m.env.code_va(0x2964); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead4b cmp ebx,DWORD PTR [r14+0x64]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x64)) as u64).1;
            // ead4f lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // ead52 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // ead54 jl 0xead1e
            if fl.l() { pc = 0xead1e; continue; }
            pc = 0xead56;
        }
        0xead56 => {
            // block ead56
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead56 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead5a mov r12d,DWORD PTR [rbp-0x2b8]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            pc = 0xead61;
        }
        0xead61 => {
            // block ead61
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc ead61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ead61 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // ead68 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // ead6a call 0x6806e
            { let r = { let t = m.env.code_va(0x6806e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead6f mov edi,DWORD PTR [rbp-0x2b4]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // ead75 call 0x5a914
            { let r = { let t = m.env.code_va(0x5a914); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead7a mov DWORD PTR [rbp-0x2b4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), ((rax as u32 as u64)) as u32);
            // ead80 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // ead83 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead88 mov WORD PTR [rbp-0x2c2],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_fd3e), ((rax as u16 as u64)) as u16);
            // ead8f mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // ead92 call 0x5caed
            { let r = { let t = m.env.code_va(0x5caed); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ead97 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // ead9a mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead9e mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // eada1 call 0xcfa3b
            { let r = { let t = m.env.code_va(0xcfa3b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eada6 movzx ecx,bx
            rcx = ((rbx as u16 as u64)) as u32 as u64;
            // eada9 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // eadac cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // eadae jbe 0xeadc7
            if fl.be() { pc = 0xeadc7; continue; }
            pc = 0xeadb0;
        }
        0xeadb0 => {
            // block eadb0
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadb0 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eadb7 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadbc mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // eadc2 jmp 0xeafa3
            pc = 0xeafa3; continue;
        }
        0xeadc7 => {
            // block eadc7
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadc7 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eadcb mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // eadce xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // eadd0 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // eadd3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eadd5 call 0xbbacd
            { let r = { let t = m.env.code_va(0xbbacd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadda mov DWORD PTR [rbp-0x2c0],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd40), ((rax as u32 as u64)) as u32);
            // eade0 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eade4 movzx esi,r13b
            rsi = ((r13 as u8 as u64)) as u32 as u64;
            // eade8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // eadea call 0xbbb07
            { let r = { let t = m.env.code_va(0xbbb07); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eadef mov DWORD PTR [rbp-0x2bc],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd44), ((rax as u32 as u64)) as u32);
            // eadf5 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // eadf8 je 0xeae1b
            if fl.e() { pc = 0xeae1b; continue; }
            pc = 0xeadfa;
        }
        0xeadfa => {
            // block eadfa
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eadfa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eadfa mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eadfe xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eae00 call 0xbba5d
            { let r = { let t = m.env.code_va(0xbba5d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae05 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae0c xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eae0e mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eae10 mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae16 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeae1b;
        }
        0xeae1b => {
            // block eae1b
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae1b lea rsi,[r14+0x38]
            rsi = r14.wrapping_add(0x38);
            // eae1f lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae26 call 0x914a
            { let r = { let t = m.env.code_va(0x914a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae2b lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae32 call 0x9046
            { let r = { let t = m.env.code_va(0x9046); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae37 mov edi,DWORD PTR [r14+0x60]
            rdi = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // eae3b xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eae3d call 0xbba5d
            { let r = { let t = m.env.code_va(0xbba5d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae42 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eae49 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eae4b mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eae4d mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae53 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae58 mov BYTE PTR [rbp-0x2c3],r13b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d), ((r13 as u8 as u64)) as u8);
            // eae5f mov DWORD PTR [rbp-0x2b8],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd48), ((r12 as u32 as u64)) as u32);
            // eae66 mov ax,WORD PTR [rbp-0x2c2]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_fd3e)) as u64) as u16 as u64);
            // eae6d test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // eae70 je 0xeaee5
            if fl.e() { pc = 0xeaee5; continue; }
            pc = 0xeae72;
        }
        0xeae72 => {
            // block eae72
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae72 movzx r12d,ax
            r12 = ((rax as u16 as u64)) as u32 as u64;
            // eae76 cmp BYTE PTR [rbp-0x2c3],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d)) as u64, 0x0u64).1;
            // eae7d mov ebx,DWORD PTR [rbp-0x2bc]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eae83 cmovne ebx,DWORD PTR [rbp-0x2c0]
            if fl.ne() { rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // eae8a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // eae8d lea r14,[rbp-0x2b0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xeae94;
        }
        0xeae94 => {
            // block eae94
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eae94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eae94 mov edi,DWORD PTR [rbp-0x2b8]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            // eae9a call 0xcd5c3
            { let r = { let t = m.env.code_va(0xcd5c3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eae9f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaea1 je 0xeaeb3
            if fl.e() { pc = 0xeaeb3; continue; }
            pc = 0xeaea3;
        }
        0xeaea3 => {
            // block eaea3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaea3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // eaea8 mov rdi,r14
            rdi = r14;
            // eaeab mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // eaeae call 0xd778
            { let r = { let t = m.env.code_va(0xd778); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaeb3;
        }
        0xeaeb3 => {
            // block eaeb3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaeb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaeb3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaeb5 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // eaeb8 call 0xbbc9b
            { let r = { let t = m.env.code_va(0xbbc9b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaebd xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaebf mov rdi,r14
            rdi = r14;
            // eaec2 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaec4 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // eaec6 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaecb movzx esi,r13w
            rsi = ((r13 as u16 as u64)) as u32 as u64;
            // eaecf mov rdi,r14
            rdi = r14;
            // eaed2 mov edx,DWORD PTR [rbp-0x2b4]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64) as u32 as u64;
            // eaed8 call 0x6c96
            { let r = { let t = m.env.code_va(0x6c96); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaedd inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // eaee0 cmp r13d,r12d
            fl = sub::<32>((r13 as u32 as u64), (r12 as u32 as u64)).1;
            // eaee3 jb 0xeae94
            if fl.b() { pc = 0xeae94; continue; }
            pc = 0xeaee5;
        }
        0xeaee5 => {
            // block eaee5
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaee5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaee5 mov esi,0x20
            rsi = (0x20u64) as u32 as u64;
            // eaeea mov ebx,DWORD PTR [rbp-0x2b8]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            // eaef0 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaef2 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaef7 mov r14b,BYTE PTR [rbp-0x2c3]
            r14 = (r14 & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd3d)) as u64) as u8 as u64);
            // eaefe test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf00 je 0xeaf30
            if fl.e() { pc = 0xeaf30; continue; }
            pc = 0xeaf02;
        }
        0xeaf02 => {
            // block eaf02
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf02 xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // eaf04 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaf06 mov edx,0x63
            rdx = (0x63u64) as u32 as u64;
            // eaf0b call 0xbbc2c
            { let r = { let t = m.env.code_va(0xbbc2c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf10 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // eaf13 mov edx,DWORD PTR [rbp-0x2bc]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd44)) as u64) as u32 as u64;
            // eaf19 cmovne edx,DWORD PTR [rbp-0x2c0]
            if fl.ne() { rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // eaf20 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf27 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaf29 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaf2b call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaf30;
        }
        0xeaf30 => {
            // block eaf30
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf30 mov esi,0x23
            rsi = (0x23u64) as u32 as u64;
            // eaf35 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf37 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf3c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf3e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf40;
        }
        0xeaf40 => {
            // block eaf40
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf40 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // eaf45 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf47 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf4e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf50;
        }
        0xeaf50 => {
            // block eaf50
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf50 mov esi,0x5d
            rsi = (0x5du64) as u32 as u64;
            // eaf55 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf57 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf5c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf5e jne 0xeaf70
            if fl.ne() { pc = 0xeaf70; continue; }
            pc = 0xeaf60;
        }
        0xeaf60 => {
            // block eaf60
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf60 mov esi,0x6b
            rsi = (0x6bu64) as u32 as u64;
            // eaf65 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // eaf67 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf6c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // eaf6e je 0xeaf97
            if fl.e() { pc = 0xeaf97; continue; }
            pc = 0xeaf70;
        }
        0xeaf70 => {
            // block eaf70
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf70 mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // eaf75 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // eaf77 mov edx,0x63
            rdx = (0x63u64) as u32 as u64;
            // eaf7c call 0xbbc2c
            { let r = { let t = m.env.code_va(0xbbc2c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eaf81 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf88 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // eaf8a mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // eaf8c mov edx,DWORD PTR [rbp-0x2c0]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64) as u32 as u64;
            // eaf92 call 0x8e30
            { let r = { let t = m.env.code_va(0x8e30); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeaf97;
        }
        0xeaf97 => {
            // block eaf97
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eaf97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eaf97 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf9e call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeafa3;
        }
        0xeafa3 => {
            // block eafa3
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafa3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafa3 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // eafa6 add rsp,0x2a8
            { let (r, f) = add::<64>(rsp, 0x2a8u64);
              rsp = r;
              fl = f; }
            // eafad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eafae pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eafb0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eafb2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eafb4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eafb6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eafb7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeafb8 => {
            // block eafb8
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafb8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafb8 jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafba => {
            // block eafba
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafba jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbc => {
            // block eafbc
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafbc jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbe => {
            // block eafbe
            #[cfg(feature = "trace")] eprintln!("f_000eab9c pc eafbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eafbe mov rbx,rax
            rbx = rax;
            // eafc1 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eafc8 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eafcd mov rdi,rbx
            rdi = rbx;
            // eafd0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea4f0 size=168
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea4f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea4f0;
    loop {
        match pc {
        0xea4f0 => {
            // block ea4f0
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea4f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea4f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea4f1 mov rbp,rsp
            rbp = rsp;
            // ea4f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea4f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea4f8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea4f9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ea4fa mov r15,rdi
            r15 = rdi;
            // ea4fd lea rax,[rip+0x5c4fc] # 0x146a00
            rax = m.env.data_va(373197);
            // ea504 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // ea507 call 0xeafd6
            { let r = { let t = m.env.code_va(0xeafd6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea50c mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // ea510 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // ea514 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // ea517 je 0xea52b
            if fl.e() { pc = 0xea52b; continue; }
            pc = 0xea519;
        }
        0xea519 => {
            // block ea519
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea519 mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea51d call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea522 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // ea526 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // ea529 jne 0xea519
            if fl.ne() { pc = 0xea519; continue; }
            pc = 0xea52b;
        }
        0xea52b => {
            // block ea52b
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea52b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea52b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // ea52e mov rax,QWORD PTR [rax+0x88]
            rax = m.r64(rax.wrapping_add(0x88)) as u64;
            // ea535 mov rdi,r15
            rdi = r15;
            // ea538 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea53a lea rax,[rip+0x59c87] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea541 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ea545 mov QWORD PTR [r15+0x38],rax
            m.w64(r15.wrapping_add(0x38), (rax) as u64);
            // ea549 lea rdi,[r15+0x40]
            rdi = r15.wrapping_add(0x40);
            // ea54d mov rsi,QWORD PTR [r15+0x48]
            rsi = m.r64(r15.wrapping_add(0x48)) as u64;
            // ea551 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea556 mov rdi,r15
            rdi = r15;
            // ea559 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ea55d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea55e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea560 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea562 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea563 jmp 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xea568 => {
            // block ea568
            #[cfg(feature = "trace")] eprintln!("f_000ea4f0 pc ea568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea568 mov r14,rax
            r14 = rax;
            // ea56b lea rax,[rip+0x59c56] # 0x1441c8
            rax = m.env.data_va(362901);
            // ea572 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ea576 mov QWORD PTR [r15+0x38],rax
            m.w64(r15.wrapping_add(0x38), (rax) as u64);
            // ea57a lea rdi,[r15+0x40]
            rdi = r15.wrapping_add(0x40);
            // ea57e mov rsi,QWORD PTR [r15+0x48]
            rsi = m.r64(r15.wrapping_add(0x48)) as u64;
            // ea582 call 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea587 mov rdi,r15
            rdi = r15;
            // ea58a call 0xe07aa
            { let r = { let t = m.env.code_va(0xe07aa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea58f mov rdi,r14
            rdi = r14;
            // ea592 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd69d6 size=672
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d69d6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd69d6;
    loop {
        match pc {
        0xd69d6 => {
            // block d69d6
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69d6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d69d7 mov rbp,rsp
            rbp = rsp;
            // d69da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d69dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d69dd mov rbx,rdi
            rbx = rdi;
            // d69e0 call 0xd6ec8
            { let r = { let t = m.env.code_va(0xd6ec8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69e5 mov DWORD PTR [rbx+0x330],eax
            m.w32(rbx.wrapping_add(0x330), ((rax as u32 as u64)) as u32);
            // d69eb mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d69ee call 0xcdaff
            { let r = { let t = m.env.code_va(0xcdaff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69f3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d69f5 je 0xd6a3d
            if fl.e() { pc = 0xd6a3d; continue; }
            pc = 0xd69f7;
        }
        0xd69f7 => {
            // block d69f7
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d69f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d69f7 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6a01 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a08 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6a12 mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6a1c mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6a27 movabs rax,0xfe000000800000
            rax = 0xfe_0000_0080_0000u64;
            pc = 0xd6a31;
        }
        0xd6a31 => {
            // block d6a31
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a31 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6a38 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6a3d => {
            // block d6a3d
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a3d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a40 call 0xcdb57
            { let r = { let t = m.env.code_va(0xcdb57); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a45 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a47 je 0xd6a80
            if fl.e() { pc = 0xd6a80; continue; }
            pc = 0xd6a49;
        }
        0xd6a49 => {
            // block d6a49
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a49 cmp DWORD PTR [rbx+0x330],0x10220
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d6a53 jne 0xd6b0f
            if fl.ne() { pc = 0xd6b0f; continue; }
            pc = 0xd6a59;
        }
        0xd6a59 => {
            // block d6a59
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a59 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a63 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6a6a mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6a74 mov DWORD PTR [rbx+0x368],0x3d
            m.w32(rbx.wrapping_add(0x368), (0x3du64) as u32);
            // d6a7e jmp 0xd6ab5
            pc = 0xd6ab5; continue;
        }
        0xd6a80 => {
            // block d6a80
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a80 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6a83 call 0xcdbbf
            { let r = { let t = m.env.code_va(0xcdbbf); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6a88 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6a8a je 0xd6b63
            if fl.e() { pc = 0xd6b63; continue; }
            pc = 0xd6a90;
        }
        0xd6a90 => {
            // block d6a90
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6a90 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6a9a mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6aa1 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6aab mov DWORD PTR [rbx+0x368],0x3c
            m.w32(rbx.wrapping_add(0x368), (0x3cu64) as u32);
            pc = 0xd6ab5;
        }
        0xd6ab5 => {
            // block d6ab5
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6ab5 mov DWORD PTR [rbx+0x360],0x40
            m.w32(rbx.wrapping_add(0x360), (0x40u64) as u32);
            // d6abf mov DWORD PTR [rbx+0x364],0x40
            m.w32(rbx.wrapping_add(0x364), (0x40u64) as u32);
            // d6ac9 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ad4 mov DWORD PTR [rbx+0x354],0x0
            m.w32(rbx.wrapping_add(0x354), (0x0u64) as u32);
            // d6ade mov DWORD PTR [rbx+0x358],0x0
            m.w32(rbx.wrapping_add(0x358), (0x0u64) as u32);
            // d6ae8 movabs rax,0xf8000000000000
            rax = 0xf8_0000_0000_0000u64;
            // d6af2 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6af9 movabs rax,0xf0000000f40000
            rax = 0xf0_0000_00f4_0000u64;
            // d6b03 mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b0a jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b0f => {
            // block d6b0f
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b0f movabs rax,0x100000002000000
            rax = 0x100_0000_0200_0000u64;
            // d6b19 mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b20 mov DWORD PTR [rbx+0x35c],0x100
            m.w32(rbx.wrapping_add(0x35c), (0x100u64) as u32);
            // d6b2a mov DWORD PTR [rbx+0x368],0xff
            m.w32(rbx.wrapping_add(0x368), (0xffu64) as u32);
            // d6b34 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6b3f movabs rax,0xfe000000000000
            rax = 0xfe_0000_0000_0000u64;
            // d6b49 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6b50 movabs rax,0xfc000000fd0000
            rax = 0xfc_0000_00fd_0000u64;
            // d6b5a mov QWORD PTR [rbx+0x34c],rax
            m.w64(rbx.wrapping_add(0x34c), (rax) as u64);
            // d6b61 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6b63 => {
            // block d6b63
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b63 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d6b66 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6b6b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6b6d je 0xd6c23
            if fl.e() { pc = 0xd6c23; continue; }
            pc = 0xd6b73;
        }
        0xd6b73 => {
            // block d6b73
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6b73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6b73 movabs rax,0x100000004000000
            rax = 0x100_0000_0400_0000u64;
            // d6b7d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6b84 mov DWORD PTR [rbx+0x35c],0x80
            m.w32(rbx.wrapping_add(0x35c), (0x80u64) as u32);
            // d6b8e mov DWORD PTR [rbx+0x368],0x7f
            m.w32(rbx.wrapping_add(0x368), (0x7fu64) as u32);
            // d6b98 mov QWORD PTR [rbx+0x33c],0x40000
            m.w64(rbx.wrapping_add(0x33c), (0x40000u64) as u64);
            // d6ba3 mov DWORD PTR [rbx+0x344],0x0
            m.w32(rbx.wrapping_add(0x344), (0x0u64) as u32);
            pc = 0xd6bad;
        }
        0xd6bad => {
            // block d6bad
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bad mov rdi,QWORD PTR [rbx+0x6b8]
            rdi = m.r64(rbx.wrapping_add(0x6b8)) as u64;
            // d6bb4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6bb7 je 0xd6bca
            if fl.e() { pc = 0xd6bca; continue; }
            pc = 0xd6bb9;
        }
        0xd6bb9 => {
            // block d6bb9
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bb9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6bbc call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bbf mov QWORD PTR [rbx+0x6b8],0x0
            m.w64(rbx.wrapping_add(0x6b8), (0x0u64) as u64);
            pc = 0xd6bca;
        }
        0xd6bca => {
            // block d6bca
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6bca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6bca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bcd mov rdi,rbx
            rdi = rbx;
            // d6bd0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bd6 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // d6bd8 call 0x2f83a
            { let r = { let t = m.env.code_va(0x2f83a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bdd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6bdf je 0xd6c1c
            if fl.e() { pc = 0xd6c1c; continue; }
            pc = 0xd6be1;
        }
        0xd6be1 => {
            // block d6be1
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6be1 mov edi,0x2c0
            rdi = (0x2c0u64) as u32 as u64;
            // d6be6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6beb mov r14,rax
            r14 = rax;
            // d6bee mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6bf1 mov rax,QWORD PTR [rax+0x1a0]
            rax = m.r64(rax.wrapping_add(0x1a0)) as u64;
            // d6bf8 mov rdi,rbx
            rdi = rbx;
            // d6bfb call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6bfd movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // d6c00 cmp BYTE PTR [rbx+0x378],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d6c07 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d6c0a movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // d6c0d mov rdi,r14
            rdi = r14;
            // d6c10 call 0x2f7d2
            { let r = { let t = m.env.code_va(0x2f7d2); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6c15 mov QWORD PTR [rbx+0x6b8],r14
            m.w64(rbx.wrapping_add(0x6b8), (r14) as u64);
            pc = 0xd6c1c;
        }
        0xd6c1c => {
            // block d6c1c
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c1c mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d6c1e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6c1f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6c21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c22 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6c23 => {
            // block d6c23
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c23 movabs rax,0x100000001000000
            rax = 0x100_0000_0100_0000u64;
            // d6c2d mov QWORD PTR [rbx+0x334],rax
            m.w64(rbx.wrapping_add(0x334), (rax) as u64);
            // d6c34 mov DWORD PTR [rbx+0x35c],0x40
            m.w32(rbx.wrapping_add(0x35c), (0x40u64) as u32);
            // d6c3e mov DWORD PTR [rbx+0x368],0x3f
            m.w32(rbx.wrapping_add(0x368), (0x3fu64) as u32);
            // d6c48 mov QWORD PTR [rbx+0x33c],0x10000
            m.w64(rbx.wrapping_add(0x33c), (0x10000u64) as u64);
            // d6c53 movabs rax,0xfe000000c00000
            rax = 0xfe_0000_00c0_0000u64;
            // d6c5d jmp 0xd6a31
            pc = 0xd6a31; continue;
        }
        0xd6c62 => {
            // block d6c62
            #[cfg(feature = "trace")] eprintln!("f_000d69d6 pc d6c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c62 mov rbx,rax
            rbx = rax;
            // d6c65 mov rdi,r14
            rdi = r14;
            // d6c68 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6c6d mov rdi,rbx
            rdi = rbx;
            // d6c70 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6814 size=450
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6814<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6814;
    loop {
        match pc {
        0xd6814 => {
            // block d6814
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6814 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6815 mov rbp,rsp
            rbp = rsp;
            // d6818 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d681a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d681c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d681e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6820 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6821 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d6825 mov rbx,rdi
            rbx = rdi;
            // d6828 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d682a mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // d682f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6832 call 0xb672c
            { let r = { let t = m.env.code_va(0xb672c); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6837 lea rax,[rip+0x6dbd2] # 0x144410
            rax = m.env.data_va(363485);
            // d683e mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d6841 lea r14,[rbx+0x2a0]
            r14 = rbx.wrapping_add(0x2a0);
            // d6848 mov DWORD PTR [rbx+0x37c],0x0
            m.w32(rbx.wrapping_add(0x37c), (0x0u64) as u32);
            // d6852 lea r12,[rbx+0x380]
            r12 = rbx.wrapping_add(0x380);
            // d6859 mov DWORD PTR [rbx+0x298],0x0
            m.w32(rbx.wrapping_add(0x298), (0x0u64) as u32);
            // d6863 mov QWORD PTR [rbx+0x290],0x0
            m.w64(rbx.wrapping_add(0x290), (0x0u64) as u64);
            // d686e mov QWORD PTR [rbx+0x288],0x0
            m.w64(rbx.wrapping_add(0x288), (0x0u64) as u64);
            // d6879 mov esi,0x90
            rsi = (0x90u64) as u32 as u64;
            // d687e mov rdi,r14
            rdi = r14;
            // d6881 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6886 mov BYTE PTR [rbx+0x378],0x0
            m.w8(rbx.wrapping_add(0x378), (0x0u64) as u8);
            // d688d mov DWORD PTR [rbx+0x374],0x0
            m.w32(rbx.wrapping_add(0x374), (0x0u64) as u32);
            // d6897 mov QWORD PTR [rbx+0x36c],0x0
            m.w64(rbx.wrapping_add(0x36c), (0x0u64) as u64);
            // d68a2 mov QWORD PTR [rbx+0x364],0x0
            m.w64(rbx.wrapping_add(0x364), (0x0u64) as u64);
            // d68ad mov QWORD PTR [rbx+0x35c],0x0
            m.w64(rbx.wrapping_add(0x35c), (0x0u64) as u64);
            // d68b8 mov QWORD PTR [rbx+0x354],0x0
            m.w64(rbx.wrapping_add(0x354), (0x0u64) as u64);
            // d68c3 mov QWORD PTR [rbx+0x34c],0x0
            m.w64(rbx.wrapping_add(0x34c), (0x0u64) as u64);
            // d68ce mov QWORD PTR [rbx+0x344],0x0
            m.w64(rbx.wrapping_add(0x344), (0x0u64) as u64);
            // d68d9 mov QWORD PTR [rbx+0x33c],0x0
            m.w64(rbx.wrapping_add(0x33c), (0x0u64) as u64);
            // d68e4 mov QWORD PTR [rbx+0x334],0x0
            m.w64(rbx.wrapping_add(0x334), (0x0u64) as u64);
            // d68ef mov rdi,r12
            rdi = r12;
            // d68f2 call 0xe7eea
            { let r = { let t = m.env.code_va(0xe7eea); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d68f7 lea r15,[rbx+0x6a0]
            r15 = rbx.wrapping_add(0x6a0);
            // d68fe mov QWORD PTR [rbx+0x6b8],0x0
            m.w64(rbx.wrapping_add(0x6b8), (0x0u64) as u64);
            // d6909 mov QWORD PTR [rbx+0x6b0],0x0
            m.w64(rbx.wrapping_add(0x6b0), (0x0u64) as u64);
            // d6914 mov QWORD PTR [rbx+0x6a8],0x0
            m.w64(rbx.wrapping_add(0x6a8), (0x0u64) as u64);
            // d691f mov QWORD PTR [rbx+0x6a0],0x0
            m.w64(rbx.wrapping_add(0x6a0), (0x0u64) as u64);
            // d692a mov rdi,rbx
            rdi = rbx;
            // d692d call 0xd69d6
            { let r = { let t = m.env.code_va(0xd69d6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6932 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d6936 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6937 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6939 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d693b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d693d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d693f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6940 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6941 => {
            // block d6941
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6941 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6941 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d6945 jmp 0xd6970
            pc = 0xd6970; continue;
        }
        0xd6947 => {
            // block d6947
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6947 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d694b mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // d694e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6951 je 0xd6968
            if fl.e() { pc = 0xd6968; continue; }
            pc = 0xd6953;
        }
        0xd6953 => {
            // block d6953
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6953 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6953 cmp QWORD PTR [rbx+0x6a8],rdi
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rdi).1;
            // d695a je 0xd6963
            if fl.e() { pc = 0xd6963; continue; }
            pc = 0xd695c;
        }
        0xd695c => {
            // block d695c
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d695c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d695c mov QWORD PTR [rbx+0x6a8],rdi
            m.w64(rbx.wrapping_add(0x6a8), (rdi) as u64);
            pc = 0xd6963;
        }
        0xd6963 => {
            // block d6963
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6963 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6968;
        }
        0xd6968 => {
            // block d6968
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6968 mov rdi,r12
            rdi = r12;
            // d696b call 0xe807c
            { let r = { let t = m.env.code_va(0xe807c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6970;
        }
        0xd6970 => {
            // block d6970
            #[cfg(feature = "trace")] eprintln!("f_000d6814 pc d6970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6970 lea rax,[rbx+0x2b8]
            rax = rbx.wrapping_add(0x2b8);
            // d6977 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // d697b lea r13,[rbx+0x2d0]
            r13 = rbx.wrapping_add(0x2d0);
            // d6982 lea r15,[rbx+0x2e8]
            r15 = rbx.wrapping_add(0x2e8);
            // d6989 lea r12,[rbx+0x300]
            r12 = rbx.wrapping_add(0x300);
            // d6990 lea rdi,[rbx+0x318]
            rdi = rbx.wrapping_add(0x318);
            // d6997 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d699c mov rdi,r12
            rdi = r12;
            // d699f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69a4 mov rdi,r15
            rdi = r15;
            // d69a7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69ac mov rdi,r13
            rdi = r13;
            // d69af call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69b4 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // d69b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69bd mov rdi,r14
            rdi = r14;
            // d69c0 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69c5 mov rdi,rbx
            rdi = rbx;
            // d69c8 call 0xb6894
            { let r = { let t = m.env.code_va(0xb6894); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d69cd mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d69d1 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6c76 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6c76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6c76;
    loop {
        match pc {
        0xd6c76 => {
            // block d6c76
            #[cfg(feature = "trace")] eprintln!("f_000d6c76 pc d6c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6c76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6c77 mov rbp,rsp
            rbp = rsp;
            // d6c7a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d6c7c xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d6c7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6c7f jmp 0xd6814
            { let r = { let t = m.env.code_va(0xd6814); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6914 size=512
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6914;
    loop {
        match pc {
        0xb6914 => {
            // block b6914
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6915 mov rbp,rsp
            rbp = rsp;
            // b6918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b691a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b691c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b691d sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // b6921 mov rbx,rdi
            rbx = rdi;
            // b6924 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // b692b mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // b6931 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b6935 je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb693b;
        }
        0xb693b => {
            // block b693b
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b693b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b693b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b693e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // b6942 mov r14d,0xffffffff
            r14 = (0xffffffffu64) as u32 as u64;
            // b6948 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // b694d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b6952 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b6955 mov rdi,rbx
            rdi = rbx;
            // b6958 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b695b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b695d je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb6963;
        }
        0xb6963 => {
            // block b6963
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6963 mov r14d,DWORD PTR [rbp-0x24]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // b6967 cmp r14d,DWORD PTR [rbx+0x10]
            fl = sub::<32>((r14 as u32 as u64), m.r32(rbx.wrapping_add(0x10)) as u64).1;
            // b696b je 0xb6ad6
            if fl.e() { pc = 0xb6ad6; continue; }
            pc = 0xb6971;
        }
        0xb6971 => {
            // block b6971
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6971 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6971 mov rdi,QWORD PTR [rip+0x89808] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // b6978 lea rsi,[rip+0x813f8] # 0x137d77
            rsi = m.env.data_va(312644);
            // b697f mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // b6984 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6989 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b698c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b6990 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b6994 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6997 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b699a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b699e mov rdi,rax
            rdi = rax;
            // b69a1 mov rsi,rbx
            rsi = rbx;
            // b69a4 call 0xec086
            { let r = { let t = m.env.stub_va(0x853); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69a9 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b69ac mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b69b0 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b69b4 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69b7 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69ba mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b69be lea rsi,[rip+0x813d0] # 0x137d95
            rsi = m.env.data_va(312674);
            // b69c5 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // b69ca mov rdi,rax
            rdi = rax;
            // b69cd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69d2 mov r15,rax
            r15 = rax;
            // b69d5 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b69d9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b69db mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // b69de call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b69e3 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b69e7 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b69ea jne 0xb69f5
            if fl.ne() { pc = 0xb69f5; continue; }
            pc = 0xb69ec;
        }
        0xb69ec => {
            // block b69ec
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69ec lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b69f0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b69f3 jmp 0xb69fd
            pc = 0xb69fd; continue;
        }
        0xb69f5 => {
            // block b69f5
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69f5 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b69f9 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb69fd;
        }
        0xb69fd => {
            // block b69fd
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b69fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b69fd mov rdi,r15
            rdi = r15;
            // b6a00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a05 lea rsi,[rip+0x8139a] # 0x137da6
            rsi = m.env.data_va(312691);
            // b6a0c mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b6a11 mov rdi,rax
            rdi = rax;
            // b6a14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a19 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // b6a1e mov rdi,rax
            rdi = rax;
            // b6a21 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a26 lea rsi,[rip+0x8138e] # 0x137dbb
            rsi = m.env.data_va(312712);
            // b6a2d mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // b6a32 mov rdi,rax
            rdi = rax;
            // b6a35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a3a mov r15,rax
            r15 = rax;
            // b6a3d mov esi,DWORD PTR [rbx+0x10]
            rsi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b6a40 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6a44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b6a46 call 0x5c0a9
            { let r = { let t = m.env.code_va(0x5c0a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a4b movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // b6a4f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b6a52 jne 0xb6a5d
            if fl.ne() { pc = 0xb6a5d; continue; }
            pc = 0xb6a54;
        }
        0xb6a54 => {
            // block b6a54
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a54 lea rsi,[rbp-0x57]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // b6a58 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b6a5b jmp 0xb6a65
            pc = 0xb6a65; continue;
        }
        0xb6a5d => {
            // block b6a5d
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a5d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a5d mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b6a61 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            pc = 0xb6a65;
        }
        0xb6a65 => {
            // block b6a65
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6a65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6a65 mov rdi,r15
            rdi = r15;
            // b6a68 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a6d mov rbx,rax
            rbx = rax;
            // b6a70 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b6a73 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b6a77 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b6a7a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6a7e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a83 mov rsi,QWORD PTR [rip+0x89706] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b6a8a lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6a8e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6a93 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b6a96 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b6a9a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b6a9f mov rdi,rax
            rdi = rax;
            // b6aa2 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6aa4 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b6aa7 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6aab call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ab0 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b6ab4 mov rdi,rbx
            rdi = rbx;
            // b6ab7 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6abc mov rdi,rbx
            rdi = rbx;
            // b6abf call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6ac4 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6ac8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6acd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6ad1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6ad6;
        }
        0xb6ad6 => {
            // block b6ad6
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ad6 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // b6ad9 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // b6add pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6ade pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6ae0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6ae2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6ae3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6ae4 => {
            // block b6ae4
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ae4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ae4 mov rbx,rax
            rbx = rax;
            // b6ae7 jmp 0xb6b03
            pc = 0xb6b03; continue;
        }
        0xb6ae9 => {
            // block b6ae9
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6ae9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6ae9 mov rbx,rax
            rbx = rax;
            // b6aec jmp 0xb6afa
            pc = 0xb6afa; continue;
        }
        0xb6aee => {
            // block b6aee
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6aee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6aee mov rbx,rax
            rbx = rax;
            // b6af1 lea rdi,[rbp-0x20]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // b6af5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6afa;
        }
        0xb6afa => {
            // block b6afa
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6afa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6afa lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // b6afe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb6b03;
        }
        0xb6b03 => {
            // block b6b03
            #[cfg(feature = "trace")] eprintln!("f_000b6914 pc b6b03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6b03 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b6b07 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6b0c mov rdi,rbx
            rdi = rbx;
            // b6b0f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e8e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e8e;
    loop {
        match pc {
        0xe0e8e => {
            // block e0e8e
            #[cfg(feature = "trace")] eprintln!("f_000e0e8e pc e0e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e8f mov rbp,rsp
            rbp = rsp;
            // e0e92 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e96 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e99 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e9d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ea3 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0ea6 shr eax,0x1e
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1eu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0ea9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ead pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0eae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0cbc size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0cbc;
    loop {
        match pc {
        0xe0cbc => {
            // block e0cbc
            #[cfg(feature = "trace")] eprintln!("f_000e0cbc pc e0cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0cbd mov rbp,rsp
            rbp = rsp;
            // e0cc0 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cc4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0cc7 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0ccb call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cd1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0cd4 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0cd7 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0cda add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0cdf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e26 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e26;
    loop {
        match pc {
        0xe0e26 => {
            // block e0e26
            #[cfg(feature = "trace")] eprintln!("f_000e0e26 pc e0e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e27 mov rbp,rsp
            rbp = rsp;
            // e0e2a sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e2e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e31 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e35 call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e3b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0e3e shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0e41 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0e44 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe478a size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e478a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe478a;
    loop {
        match pc {
        0xe478a => {
            // block e478a
            #[cfg(feature = "trace")] eprintln!("f_000e478a pc e478a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e478a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e478b mov rbp,rsp
            rbp = rsp;
            // e478e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4792 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4795 mov r9,QWORD PTR [rax]
            r9 = m.r64(rax) as u64;
            // e4798 mov esi,DWORD PTR [rdi+0x30]
            rsi = (m.r32(rdi.wrapping_add(0x30)) as u64) as u32 as u64;
            // e479b add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e479e lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e47a2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e47a7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e47aa mov rdi,rax
            rdi = rax;
            // e47ad call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e47b1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e47b4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e47b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e47b9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7d52 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7d52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7d52;
    loop {
        match pc {
        0xb7d52 => {
            // block b7d52
            #[cfg(feature = "trace")] eprintln!("f_000b7d52 pc b7d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7d52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7d53 mov rbp,rsp
            rbp = rsp;
            // b7d56 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7d5a mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7d61 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7d64 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7d68 mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // b7d6d mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b7d72 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7d75 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d78 test BYTE PTR [rbp-0x4],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64 & 0x8u64);
            // b7d7c sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b7d7f add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7d83 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7d84 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7dba size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7dba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7dba;
    loop {
        match pc {
        0xb7dba => {
            // block b7dba
            #[cfg(feature = "trace")] eprintln!("f_000b7dba pc b7dba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7dba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7dbb mov rbp,rsp
            rbp = rsp;
            // b7dbe sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7dc2 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7dc9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7dcc lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7dd0 mov esi,0x51
            rsi = (0x51u64) as u32 as u64;
            // b7dd5 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // b7dda xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7ddd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7de0 cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b7de4 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b7de7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7deb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7dec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7894 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7894<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7894;
    loop {
        match pc {
        0xb7894 => {
            // block b7894
            #[cfg(feature = "trace")] eprintln!("f_000b7894 pc b7894 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7894 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7895 mov rbp,rsp
            rbp = rsp;
            // b7898 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7899 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b789a mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // b78a1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b78a4 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // b78a8 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // b78aa mov esi,0x36
            rsi = (0x36u64) as u32 as u64;
            // b78af mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b78b4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b78b7 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b78ba test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b78bc cmovne ebx,DWORD PTR [rbp-0xc]
            if fl.ne() { rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // b78c0 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // b78c2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b78c6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b78c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b78c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8154 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8154<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8154;
    loop {
        match pc {
        0xb8154 => {
            // block b8154
            #[cfg(feature = "trace")] eprintln!("f_000b8154 pc b8154 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8154 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8155 mov rbp,rsp
            rbp = rsp;
            // b8158 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b815c mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b8163 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8166 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b816a mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // b816f mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // b8174 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // b817a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b817d cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b8181 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b8184 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b8188 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8189 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
