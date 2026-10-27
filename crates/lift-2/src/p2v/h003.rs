//! Mechanically lifted functions (scripts/lift_run.py), module h003. Each one is checked against the original
//! in the world harness (tests/world_h003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5855b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5857b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58593 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5859e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5859e cmp edx,0x6
            fl = sub::<32>((rdx as u32 as u64), 0x6u64).1;
            // 585a1 jb 0x5869b
            if fl.b() { pc = 0x5869b; continue; }
            pc = 0x585a7;
        }
        0x585a7 => {
            // block 585a7
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 585e5 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 585e9 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x585ec;
        }
        0x585ec => {
            // block 585ec
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 585ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5862b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5863f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5864c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58677 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58684 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58690 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 58693 jb 0x585e5
            if fl.b() { pc = 0x585e5; continue; }
            pc = 0x58699;
        }
        0x58699 => {
            // block 58699
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 58699 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58699 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5869b;
        }
        0x5869b => {
            // block 5869b
            #[cfg(feature = "trace")] eprintln!("f_0005855b pc 5869b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x32996 size=710
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032996<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x32996;
    loop {
        match pc {
        0x32996 => {
            // block 32996
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32996 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32997 mov rbp,rsp
            rbp = rsp;
            // 3299a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3299c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3299d mov QWORD PTR [rdi+0x8],0x0
            m.w64(rdi.wrapping_add(0x8), (0x0u64) as u64);
            // 329a5 mov QWORD PTR [rdi],0x0
            m.w64(rdi, (0x0u64) as u64);
            // 329ac mov DWORD PTR [rdi+0x10],0xc
            m.w32(rdi.wrapping_add(0x10), (0xcu64) as u32);
            // 329b3 mov DWORD PTR [rdi+0x14],0x0
            m.w32(rdi.wrapping_add(0x14), (0x0u64) as u32);
            // 329ba mov DWORD PTR [rdi+0x18],0x20
            m.w32(rdi.wrapping_add(0x18), (0x20u64) as u32);
            // 329c1 mov DWORD PTR [rdi+0x1c],0x3
            m.w32(rdi.wrapping_add(0x1c), (0x3u64) as u32);
            // 329c8 mov BYTE PTR [rdi+0x20],0x0
            m.w8(rdi.wrapping_add(0x20), (0x0u64) as u8);
            // 329cc mov WORD PTR [rdi+0x34],0x0
            m.w16(rdi.wrapping_add(0x34), (0x0u64) as u16);
            // 329d2 mov QWORD PTR [rdi+0x2c],0x0
            m.w64(rdi.wrapping_add(0x2c), (0x0u64) as u64);
            // 329da mov QWORD PTR [rdi+0x24],0x0
            m.w64(rdi.wrapping_add(0x24), (0x0u64) as u64);
            // 329e2 mov DWORD PTR [rdi+0x38],0x12
            m.w32(rdi.wrapping_add(0x38), (0x12u64) as u32);
            // 329e9 cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 329ec ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x329f2;
        }
        0x329f2 => {
            // block 329f2
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 329f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 329f2 cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 329f5 ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x329fb;
        }
        0x329fb => {
            // block 329fb
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 329fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 329fb mov al,r9b
            rax = (rax & !0xff) | (((r9 as u8 as u64)) as u8 as u64);
            // 329fe xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32a00 or al,cl
            { let r = (rax as u8 as u64) | (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 32a02 je 0x32c27
            if fl.e() { pc = 0x32c27; continue; }
            pc = 0x32a08;
        }
        0x32a08 => {
            // block 32a08
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a08 mov r10d,edx
            r10 = ((rdx as u32 as u64)) as u32 as u64;
            // 32a0b mov r11d,esi
            r11 = ((rsi as u32 as u64)) as u32 as u64;
            // 32a0e imul rax,r11,0x780
            { let (r, f) = imul::<64>(r11, 0x780u64); rax = r; fl = f; }
            // 32a15 lea rbx,[rip+0x18fc54] # 0x1c2670
            rbx = m.env.data_va(880189);
            // 32a1c add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 32a1f imul r14,r10,0x3c
            { let (r, f) = imul::<64>(r10, 0x3cu64); r14 = r; fl = f; }
            // 32a23 mov r10d,DWORD PTR [r14+rbx*1]
            r10 = (m.r32(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // 32a27 mov eax,DWORD PTR [r14+rbx*1+0x38]
            rax = (m.r32(r14.wrapping_add(rbx).wrapping_add(0x38)) as u64) as u32 as u64;
            // 32a2c mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32a2f mov rax,QWORD PTR [r14+rbx*1+0x30]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x30)) as u64;
            // 32a34 mov QWORD PTR [rdi+0x30],rax
            m.w64(rdi.wrapping_add(0x30), (rax) as u64);
            // 32a38 mov rax,QWORD PTR [r14+rbx*1+0x28]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x28)) as u64;
            // 32a3d mov QWORD PTR [rdi+0x28],rax
            m.w64(rdi.wrapping_add(0x28), (rax) as u64);
            // 32a41 mov rax,QWORD PTR [r14+rbx*1+0x20]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x20)) as u64;
            // 32a46 mov QWORD PTR [rdi+0x20],rax
            m.w64(rdi.wrapping_add(0x20), (rax) as u64);
            // 32a4a mov rax,QWORD PTR [r14+rbx*1+0x18]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x18)) as u64;
            // 32a4f mov QWORD PTR [rdi+0x18],rax
            m.w64(rdi.wrapping_add(0x18), (rax) as u64);
            // 32a53 mov rax,QWORD PTR [r14+rbx*1+0x10]
            rax = m.r64(r14.wrapping_add(rbx).wrapping_add(0x10)) as u64;
            // 32a58 mov QWORD PTR [rdi+0x10],rax
            m.w64(rdi.wrapping_add(0x10), (rax) as u64);
            // 32a5c mov rax,QWORD PTR [r14+rbx*1]
            rax = m.r64(r14.wrapping_add(rbx)) as u64;
            // 32a60 mov rbx,QWORD PTR [r14+rbx*1+0x8]
            rbx = m.r64(r14.wrapping_add(rbx).wrapping_add(0x8)) as u64;
            // 32a65 mov QWORD PTR [rdi+0x8],rbx
            m.w64(rdi.wrapping_add(0x8), (rbx) as u64);
            // 32a69 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 32a6c cmp esi,0xb
            fl = sub::<32>((rsi as u32 as u64), 0xbu64).1;
            // 32a6f ja 0x32ba1
            if fl.a() { pc = 0x32ba1; continue; }
            pc = 0x32a75;
        }
        0x32a75 => {
            // block 32a75
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a75 lea rax,[rip+0x1b0] # 0x32c2c
            rax = m.env.code_va(0x32c2c);
            // 32a7c movsxd rbx,DWORD PTR [rax+r11*4]
            rbx = sx::<32>(m.r32(rax.wrapping_add(r11.wrapping_mul(4))) as u64) as u64;
            // 32a80 add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 32a83 jmp rbx
            pc = (rbx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x32a85 => {
            // block 32a85
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a85 mov DWORD PTR [rdi+0x38],0x0
            m.w32(rdi.wrapping_add(0x38), (0x0u64) as u32);
            // 32a8c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32a8e je 0x32ae5
            if fl.e() { pc = 0x32ae5; continue; }
            pc = 0x32a90;
        }
        0x32a90 => {
            // block 32a90
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32a90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32a90 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32a94 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 32a9b mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32a9d test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32aa0 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32aa3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 32aa6 lea eax,[rax*4+0x4]
            rax = (rax.wrapping_mul(4).wrapping_add(0x4)) as u32 as u64;
            // 32aad mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32ab0 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32ab5 => {
            // block 32ab5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ab5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ab5 mov DWORD PTR [rdi+0x38],0xa
            m.w32(rdi.wrapping_add(0x38), (0xau64) as u32);
            // 32abc test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32abe je 0x32ae5
            if fl.e() { pc = 0x32ae5; continue; }
            pc = 0x32ac0;
        }
        0x32ac0 => {
            // block 32ac0
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ac0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ac0 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32ac4 lea eax,[rax+rax*1+0x458]
            rax = (rax.wrapping_add(rax).wrapping_add(0x458)) as u32 as u64;
            // 32acb mov DWORD PTR [rdi],eax
            m.w32(rdi, ((rax as u32 as u64)) as u32);
            // 32acd test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32ad0 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 32ad3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 32ad6 lea eax,[rax*8+0x5]
            rax = (rax.wrapping_mul(8).wrapping_add(0x5)) as u32 as u64;
            // 32add mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32ae0 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32ae5 => {
            // block 32ae5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ae5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ae5 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32ae7 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32aea mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32aed mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 32af0 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32af5 => {
            // block 32af5
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32af5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32af5 mov DWORD PTR [rdi+0x38],0x10
            m.w32(rdi.wrapping_add(0x38), (0x10u64) as u32);
            // 32afc jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b01 => {
            // block 32b01
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b01 mov DWORD PTR [rdi+0x38],0x11
            m.w32(rdi.wrapping_add(0x38), (0x11u64) as u32);
            // 32b08 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b0d => {
            // block 32b0d
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b0d mov DWORD PTR [rdi+0x38],0x1
            m.w32(rdi.wrapping_add(0x38), (0x1u64) as u32);
            // 32b14 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b16 je 0x32bc1
            if fl.e() { pc = 0x32bc1; continue; }
            pc = 0x32b1c;
        }
        0x32b1c => {
            // block 32b1c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b1c mov DWORD PTR [rdi],0x2e4
            m.w32(rdi, (0x2e4u64) as u32);
            // 32b22 mov DWORD PTR [rdi+0x38],0x9
            m.w32(rdi.wrapping_add(0x38), (0x9u64) as u32);
            // 32b29 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b2b => {
            // block 32b2b
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b2b mov DWORD PTR [rdi+0x38],0x2
            m.w32(rdi.wrapping_add(0x38), (0x2u64) as u32);
            // 32b32 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b34 je 0x32bd2
            if fl.e() { pc = 0x32bd2; continue; }
            pc = 0x32b3a;
        }
        0x32b3a => {
            // block 32b3a
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b3a test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32b3d mov eax,0x202
            rax = (0x202u64) as u32 as u64;
            // 32b42 mov ebx,0x1fc
            rbx = (0x1fcu64) as u32 as u64;
            // 32b47 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // 32b4a mov DWORD PTR [rdi],ebx
            m.w32(rdi, ((rbx as u32 as u64)) as u32);
            // 32b4c movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32b50 lea eax,[rax*8+0x6]
            rax = (rax.wrapping_mul(8).wrapping_add(0x6)) as u32 as u64;
            // 32b57 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32b5a jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b5c => {
            // block 32b5c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b5c mov DWORD PTR [rdi+0x38],0x3
            m.w32(rdi.wrapping_add(0x38), (0x3u64) as u32);
            // 32b63 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b65 je 0x32be3
            if fl.e() { pc = 0x32be3; continue; }
            pc = 0x32b67;
        }
        0x32b67 => {
            // block 32b67
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b67 test r9b,r9b
            fl = logic::<8>((r9 as u8 as u64) & (r9 as u8 as u64));
            // 32b6a mov eax,0x264
            rax = (0x264u64) as u32 as u64;
            // 32b6f mov ebx,0x256
            rbx = (0x256u64) as u32 as u64;
            // 32b74 cmovne ebx,eax
            if fl.ne() { rbx = ((rax as u32 as u64)) as u32 as u64; } else { rbx = rbx as u32 as u64; }
            // 32b77 mov DWORD PTR [rdi],ebx
            m.w32(rdi, ((rbx as u32 as u64)) as u32);
            // 32b79 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32b7d lea eax,[rax*8+0x7]
            rax = (rax.wrapping_mul(8).wrapping_add(0x7)) as u32 as u64;
            // 32b84 mov DWORD PTR [rdi+0x38],eax
            m.w32(rdi.wrapping_add(0x38), ((rax as u32 as u64)) as u32);
            // 32b87 jmp 0x32ba1
            pc = 0x32ba1; continue;
        }
        0x32b89 => {
            // block 32b89
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b89 mov DWORD PTR [rdi+0x38],0xb
            m.w32(rdi.wrapping_add(0x38), (0xbu64) as u32);
            // 32b90 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32b92 je 0x32bf4
            if fl.e() { pc = 0x32bf4; continue; }
            pc = 0x32b94;
        }
        0x32b94 => {
            // block 32b94
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32b94 mov DWORD PTR [rdi],0x634
            m.w32(rdi, (0x634u64) as u32);
            // 32b9a mov DWORD PTR [rdi+0x38],0xc
            m.w32(rdi.wrapping_add(0x38), (0xcu64) as u32);
            pc = 0x32ba1;
        }
        0x32ba1 => {
            // block 32ba1
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32ba1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ba1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32ba3 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32ba6 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32ba9 mov DWORD PTR [rdi+0x10],esi
            m.w32(rdi.wrapping_add(0x10), ((rsi as u32 as u64)) as u32);
            // 32bac mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 32baf mov BYTE PTR [rdi+0x20],r8b
            m.w8(rdi.wrapping_add(0x20), ((r8 as u8 as u64)) as u8);
            // 32bb3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bb5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 32bb7 je 0x32c0c
            if fl.e() { pc = 0x32c0c; continue; }
            pc = 0x32bb9;
        }
        0x32bb9 => {
            // block 32bb9
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bb9 movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 32bbd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 32bbf jmp 0x32c0c
            pc = 0x32c0c; continue;
        }
        0x32bc1 => {
            // block 32bc1
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bc1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bc3 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bc6 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bc9 mov DWORD PTR [rdi+0x10],0x1
            m.w32(rdi.wrapping_add(0x10), (0x1u64) as u32);
            // 32bd0 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32bd2 => {
            // block 32bd2
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bd2 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bd4 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bd7 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bda mov DWORD PTR [rdi+0x10],0x2
            m.w32(rdi.wrapping_add(0x10), (0x2u64) as u32);
            // 32be1 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32be3 => {
            // block 32be3
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32be3 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32be5 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32be8 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32beb mov DWORD PTR [rdi+0x10],0x3
            m.w32(rdi.wrapping_add(0x10), (0x3u64) as u32);
            // 32bf2 jmp 0x32c03
            pc = 0x32c03; continue;
        }
        0x32bf4 => {
            // block 32bf4
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32bf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32bf4 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // 32bf6 sub eax,r10d
            { let (r, f) = sub::<32>((rax as u32 as u64), (r10 as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 32bf9 mov DWORD PTR [rdi+0xc],eax
            m.w32(rdi.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            // 32bfc mov DWORD PTR [rdi+0x10],0x5
            m.w32(rdi.wrapping_add(0x10), (0x5u64) as u32);
            pc = 0x32c03;
        }
        0x32c03 => {
            // block 32c03
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c03 mov DWORD PTR [rdi+0x18],edx
            m.w32(rdi.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // 32c06 mov BYTE PTR [rdi+0x20],r8b
            m.w8(rdi.wrapping_add(0x20), ((r8 as u8 as u64)) as u8);
            // 32c0a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x32c0c;
        }
        0x32c0c => {
            // block 32c0c
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c0c mov DWORD PTR [rdi+0x1c],eax
            m.w32(rdi.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 32c0f cmp edx,0x1f
            fl = sub::<32>((rdx as u32 as u64), 0x1fu64).1;
            // 32c12 ja 0x32c27
            if fl.a() { pc = 0x32c27; continue; }
            pc = 0x32c14;
        }
        0x32c14 => {
            // block 32c14
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c14 mov eax,0xfc800400
            rax = (0xfc800400u64) as u32 as u64;
            // 32c19 bt eax,edx
            fl.cf = bit::<32>((rax as u32 as u64), (rdx as u32 as u64));
            // 32c1c jae 0x32c27
            if fl.ae() { pc = 0x32c27; continue; }
            pc = 0x32c1e;
        }
        0x32c1e => {
            // block 32c1e
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c1e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32c1f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32c21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32c22 jmp 0x325ca
            { let r = { let t = m.env.code_va(0x325ca); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x32c27 => {
            // block 32c27
            #[cfg(feature = "trace")] eprintln!("f_00032996 pc 32c27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32c27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32c28 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32c2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32c2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x420d2 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000420d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x420d2;
    loop {
        match pc {
        0x420d2 => {
            // block 420d2
            #[cfg(feature = "trace")] eprintln!("f_000420d2 pc 420d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 420d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 420d3 mov rbp,rsp
            rbp = rsp;
            // 420d6 mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 420d9 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 420dc mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 420e0 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 420e2 lea rdx,[rip+0xbe2d7] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 420e9 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 420ec mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // 420f1 mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // 420f7 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 420fa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 420fb jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41ea6 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041ea6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41ea6;
    loop {
        match pc {
        0x41ea6 => {
            // block 41ea6
            #[cfg(feature = "trace")] eprintln!("f_00041ea6 pc 41ea6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41ea6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41ea7 mov rbp,rsp
            rbp = rsp;
            // 41eaa mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 41ead mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 41eb0 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 41eb4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41eb6 lea rdx,[rip+0xbe503] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41ebd mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ec0 mov ecx,0x40000
            rcx = (0x40000u64) as u32 as u64;
            // 41ec5 mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // 41ecb mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 41ece pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 41ecf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15f2 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000015f2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15f2;
    loop {
        match pc {
        0x15f2 => {
            // block 15f2
            #[cfg(feature = "trace")] eprintln!("f_000015f2 pc 15f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15f3 mov rbp,rsp
            rbp = rsp;
            // 15f6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15f8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15fa push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15fb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15fc mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 15ff mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 1602 mov rbx,rdi
            rbx = rdi;
            // 1605 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1608 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 160e mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 1611 mov r9,QWORD PTR [rcx+0x30]
            r9 = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 1615 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1618 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 161a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 161c cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 1620 lea rax,[rip+0xfdc39] # 0xff260
            rax = m.env.data_va(80429);
            // 1627 mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 162a lea rax,[rip+0xfd9ef] # 0xff020
            rax = m.env.data_va(79853);
            // 1631 mov ecx,DWORD PTR [rax+rdx*4]
            rcx = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1634 lea rax,[rip+0xfda15] # 0xff050
            rax = m.env.data_va(79901);
            // 163b mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 163f mov rdi,rbx
            rdi = rbx;
            // 1642 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 1645 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1649 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 164a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 164c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 164e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 164f jmp r9
            { let r = { let t = r9; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63cb3 size=252
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63cb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63cd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63ce2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00063cb3 pc 63d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63d9b mov rbx,rax
            rbx = rax;
            // 63d9e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 63da2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63da7 mov rdi,rbx
            rdi = rbx;
            // 63daa call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28aac size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00028aac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28aac;
    loop {
        match pc {
        0x28aac => {
            // block 28aac
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28aac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28aac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28aad mov rbp,rsp
            rbp = rsp;
            // 28ab0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28ab2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28ab4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 28ab6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 28ab8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28ab9 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28abd mov r12,rsi
            r12 = rsi;
            // 28ac0 mov r14,rdi
            r14 = rdi;
            // 28ac3 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 28ac7 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 28acc call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28ad1 cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 28ad7 lea rcx,[rip+0x101d52] # 0x12a830
            rcx = m.env.data_va(258045);
            // 28ade lea rsi,[rip+0x101d5c] # 0x12a841
            rsi = m.env.data_va(258062);
            // 28ae5 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 28ae9 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28aec movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 28aef or rdx,0x10
            { let r = rdx | 0x10u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 28af3 mov rdi,rax
            rdi = rax;
            // 28af6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28afb add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 28aff cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b02 je 0x28b9a
            if fl.e() { pc = 0x28b9a; continue; }
            pc = 0x28b08;
        }
        0x28b08 => {
            // block 28b08
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b08 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28b0c lea r13,[rip+0x10198a] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x28b13;
        }
        0x28b13 => {
            // block 28b13
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b13 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 28b16 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28b18 mov rdi,r15
            rdi = r15;
            // 28b1b call 0x62a70
            { let r = { let t = m.env.code_va(0x62a70); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b20 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 28b24 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28b27 jne 0x28b32
            if fl.ne() { pc = 0x28b32; continue; }
            pc = 0x28b29;
        }
        0x28b29 => {
            // block 28b29
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b29 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 28b2c lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 28b30 jmp 0x28b3a
            pc = 0x28b3a; continue;
        }
        0x28b32 => {
            // block 28b32
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b32 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28b36 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28b3a;
        }
        0x28b3a => {
            // block 28b3a
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b3a mov rdi,r14
            rdi = r14;
            // 28b3d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b42 mov rdi,r15
            rdi = r15;
            // 28b45 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b4a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28b4e test rax,rax
            fl = logic::<64>(rax & rax);
            // 28b51 je 0x28b60
            if fl.e() { pc = 0x28b60; continue; }
            pc = 0x28b53;
        }
        0x28b53 => {
            // block 28b53
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b53 mov rbx,rax
            rbx = rax;
            // 28b56 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28b59 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28b5c jne 0x28b53
            if fl.ne() { pc = 0x28b53; continue; }
            pc = 0x28b5e;
        }
        0x28b5e => {
            // block 28b5e
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b5e jmp 0x28b6f
            pc = 0x28b6f; continue;
        }
        0x28b60 => {
            // block 28b60
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b60 mov rax,rbx
            rax = rbx;
            pc = 0x28b63;
        }
        0x28b63 => {
            // block 28b63
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b63 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28b67 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28b6a mov rax,rbx
            rax = rbx;
            // 28b6d jne 0x28b63
            if fl.ne() { pc = 0x28b63; continue; }
            pc = 0x28b6f;
        }
        0x28b6f => {
            // block 28b6f
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b6f cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b72 mov rsi,r13
            rsi = r13;
            // 28b75 lea rax,[rip+0x11014c] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28b7c cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 28b80 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28b83 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28b86 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28b89 mov rdi,r14
            rdi = r14;
            // 28b8c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28b91 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28b94 jne 0x28b13
            if fl.ne() { pc = 0x28b13; continue; }
            pc = 0x28b9a;
        }
        0x28b9a => {
            // block 28b9a
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28b9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28b9a mov rax,r14
            rax = r14;
            // 28b9d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28ba1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28ba2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28ba4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 28ba6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28ba8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28baa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28bab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x28bac => {
            // block 28bac
            #[cfg(feature = "trace")] eprintln!("f_00028aac pc 28bac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bac mov rbx,rax
            rbx = rax;
            // 28baf lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28bb3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28bb8 mov rdi,rbx
            rdi = rbx;
            // 28bbb call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
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
            #[cfg(feature = "trace")] eprintln!("f_0002eea0 pc 2eea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0002eea0 pc 2eeae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0002eea0 pc 2eebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2eebc cmp BYTE PTR [rax],0x0
            fl = sub::<8>(m.r8(rax) as u64, 0x0u64).1;
            // 2eebf je 0x2eed8
            if fl.e() { pc = 0x2eed8; continue; }
            pc = 0x2eec1;
        }
        0x2eec1 => {
            // block 2eec1
            #[cfg(feature = "trace")] eprintln!("f_0002eea0 pc 2eec1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_0002eea0 pc 2eed8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x44c68 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044c68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44c68;
    loop {
        match pc {
        0x44c68 => {
            // block 44c68
            #[cfg(feature = "trace")] eprintln!("f_00044c68 pc 44c68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44c68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44c69 mov rbp,rsp
            rbp = rsp;
            // 44c6c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44c6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44c70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44c71 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 44c72 mov r14,rsi
            r14 = rsi;
            // 44c75 mov rbx,rdi
            rbx = rdi;
            // 44c78 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // 44c7c mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 44c80 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44c85 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44c88 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 44c8b mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 44c8f mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 44c93 mov rcx,QWORD PTR [r14+0x10]
            rcx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 44c97 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 44c9b test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 44c9e je 0x44cbc
            if fl.e() { pc = 0x44cbc; continue; }
            pc = 0x44ca0;
        }
        0x44ca0 => {
            // block 44ca0
            #[cfg(feature = "trace")] eprintln!("f_00044c68 pc 44ca0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44ca0 lea rcx,[r14+0x8]
            rcx = r14.wrapping_add(0x8);
            // 44ca4 mov QWORD PTR [rax+0x10],r15
            m.w64(rax.wrapping_add(0x10), (r15) as u64);
            // 44ca8 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 44cab mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 44cb3 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 44cba jmp 0x44cbf
            pc = 0x44cbf; continue;
        }
        0x44cbc => {
            // block 44cbc
            #[cfg(feature = "trace")] eprintln!("f_00044c68 pc 44cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44cbc mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            pc = 0x44cbf;
        }
        0x44cbf => {
            // block 44cbf
            #[cfg(feature = "trace")] eprintln!("f_00044c68 pc 44cbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44cbf add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44cc3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44cc4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44cc6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44cc8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44cc9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a6dc size=47
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a6dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a6dc;
    loop {
        match pc {
        0x5a6dc => {
            // block 5a6dc
            #[cfg(feature = "trace")] eprintln!("f_0005a6dc pc 5a6dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a6dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a6dd mov rbp,rsp
            rbp = rsp;
            // 5a6e0 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5a6e2 cmp edi,0x78
            fl = sub::<32>((rdi as u32 as u64), 0x78u64).1;
            // 5a6e5 ja 0x5a707
            if fl.a() { pc = 0x5a707; continue; }
            pc = 0x5a6e7;
        }
        0x5a6e7 => {
            // block 5a6e7
            #[cfg(feature = "trace")] eprintln!("f_0005a6dc pc 5a6e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a6e7 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5a6ea lea rcx,[rip+0xa6c9f] # 0x101390
            rcx = m.env.data_va(88925);
            // 5a6f1 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5a6f4 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 5a6f6 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 5a6f9 ja 0x5a707
            if fl.a() { pc = 0x5a707; continue; }
            pc = 0x5a6fb;
        }
        0x5a6fb => {
            // block 5a6fb
            #[cfg(feature = "trace")] eprintln!("f_0005a6dc pc 5a6fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a6fb mov eax,0x7a9
            rax = (0x7a9u64) as u32 as u64;
            // 5a700 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 5a702 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5a705 jmp 0x5a709
            pc = 0x5a709; continue;
        }
        0x5a707 => {
            // block 5a707
            #[cfg(feature = "trace")] eprintln!("f_0005a6dc pc 5a707 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a707 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5a709;
        }
        0x5a709 => {
            // block 5a709
            #[cfg(feature = "trace")] eprintln!("f_0005a6dc pc 5a709 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a709 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a70a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44abc size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044abc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44abc;
    loop {
        match pc {
        0x44abc => {
            // block 44abc
            #[cfg(feature = "trace")] eprintln!("f_00044abc pc 44abc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44abc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44abd mov rbp,rsp
            rbp = rsp;
            // 44ac0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44ac2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44ac4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44ac5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 44ac6 mov r14,rsi
            r14 = rsi;
            // 44ac9 mov r15,rdi
            r15 = rdi;
            // 44acc mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 44ad1 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44ad6 mov rbx,rax
            rbx = rax;
            // 44ad9 mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 44adc mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 44adf mov rdi,r15
            rdi = r15;
            // 44ae2 mov rsi,rbx
            rsi = rbx;
            // 44ae5 call 0xebb64
            { let r = { let t = m.env.stub_va(0x331); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44aea mov r14,rax
            r14 = rax;
            // 44aed mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 44af0 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 44af4 jne 0x44afe
            if fl.ne() { pc = 0x44afe; continue; }
            pc = 0x44af6;
        }
        0x44af6 => {
            // block 44af6
            #[cfg(feature = "trace")] eprintln!("f_00044abc pc 44af6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44af6 mov rdi,rbx
            rdi = rbx;
            // 44af9 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x44afe;
        }
        0x44afe => {
            // block 44afe
            #[cfg(feature = "trace")] eprintln!("f_00044abc pc 44afe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44afe mov rax,r14
            rax = r14;
            // 44b01 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 44b04 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 44b08 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44b09 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44b0b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 44b0d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44b0e ret
            return Ret { rax, rdx, xmm0: x0 };
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
            #[cfg(feature = "trace")] eprintln!("f_0001a1e0 pc 1a1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xcec size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000cec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcec;
    loop {
        match pc {
        0xcec => {
            // block cec
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ced mov rbp,rsp
            rbp = rsp;
            // cf0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // cf2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // cf4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // cf5 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // cf9 mov r14,rsi
            r14 = rsi;
            // cfc mov rbx,rdi
            rbx = rdi;
            // cff mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d02 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d08 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // d0b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d0e lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // d12 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // d16 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d1a lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // d1e lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // d22 lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // d26 lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // d2a mov rdi,rbx
            rdi = rbx;
            // d2d call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d33 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d35 je 0xd64
            if fl.e() { pc = 0xd64; continue; }
            pc = 0xd37;
        }
        0xd37 => {
            // block d37
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3a lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // d3e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d40 mov rdi,rbx
            rdi = rbx;
            // d43 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d49 movzx eax,WORD PTR [rbp-0x32]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffce)) as u64) as u32 as u64;
            // d4d mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // d50 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // d53 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d58 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5a je 0xd68
            if fl.e() { pc = 0xd68; continue; }
            pc = 0xd5c;
        }
        0xd5c => {
            // block d5c
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // d5f sub eax,DWORD PTR [rbp-0x28]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d62 jmp 0xd6e
            pc = 0xd6e; continue;
        }
        0xd64 => {
            // block d64
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d64 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d66 jmp 0xd73
            pc = 0xd73; continue;
        }
        0xd68 => {
            // block d68
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d68 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // d6b sub eax,DWORD PTR [rbp-0x34]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6e;
        }
        0xd6e => {
            // block d6e
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d71 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd73;
        }
        0xd73 => {
            // block d73
            #[cfg(feature = "trace")] eprintln!("f_00000cec pc d73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d73 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // d77 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d78 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42360 size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042360<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42360;
    loop {
        match pc {
        0x42360 => {
            // block 42360
            #[cfg(feature = "trace")] eprintln!("f_00042360 pc 42360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42360 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42361 mov rbp,rsp
            rbp = rsp;
            // 42364 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42365 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42366 mov rbx,rsi
            rbx = rsi;
            // 42369 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 42370 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 42373 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 42375 lea rdx,[rip+0xbe0f4] # 0x100470
            rdx = m.env.data_va(85053);
            // 4237c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 4237f lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 42383 mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 42388 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 4238e call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42391 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42393 je 0x4239a
            if fl.e() { pc = 0x4239a; continue; }
            pc = 0x42395;
        }
        0x42395 => {
            // block 42395
            #[cfg(feature = "trace")] eprintln!("f_00042360 pc 42395 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42395 mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 42398 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x4239a;
        }
        0x4239a => {
            // block 4239a
            #[cfg(feature = "trace")] eprintln!("f_00042360 pc 4239a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4239a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4239e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4239f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 423a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x54cc size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000054cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x54cc;
    loop {
        match pc {
        0x54cc => {
            // block 54cc
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 54cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 54cd mov rbp,rsp
            rbp = rsp;
            // 54d0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 54d2 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 54d5 jne 0x54e0
            if fl.ne() { pc = 0x54e0; continue; }
            pc = 0x54d7;
        }
        0x54d7 => {
            // block 54d7
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 54d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54d7 mov r10b,dl
            r10 = (r10 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 54da xor r10b,0x1
            { let r = (r10 as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              r10 = (r10 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 54de je 0x5501
            if fl.e() { pc = 0x5501; continue; }
            pc = 0x54e0;
        }
        0x54e0 => {
            // block 54e0
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 54e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54e0 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // 54e3 mov r10,QWORD PTR [r10+0x5c8]
            r10 = m.r64(r10.wrapping_add(0x5c8)) as u64;
            // 54ea test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 54ed je 0x54f4
            if fl.e() { pc = 0x54f4; continue; }
            pc = 0x54ef;
        }
        0x54ef => {
            // block 54ef
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 54ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54ef movzx eax,dl
            rax = ((rdx as u8 as u64)) as u32 as u64;
            // 54f2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x54f4;
        }
        0x54f4 => {
            // block 54f4
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 54f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 54f4 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 54f6 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 54f8 mov ecx,r8d
            rcx = ((r8 as u32 as u64)) as u32 as u64;
            // 54fb mov r8d,r9d
            r8 = ((r9 as u32 as u64)) as u32 as u64;
            // 54fe call r10
            { let r = { let t = r10; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x5501;
        }
        0x5501 => {
            // block 5501
            #[cfg(feature = "trace")] eprintln!("f_000054cc pc 5501 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5501 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5502 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33b08 size=458
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033b08<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33b08;
    loop {
        match pc {
        0x33b08 => {
            // block 33b08
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b08 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33b09 mov rbp,rsp
            rbp = rsp;
            // 33b0c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 33b0e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 33b0f mov DWORD PTR [rdx],0xffffffff
            m.w32(rdx, (0xffffffffu64) as u32);
            // 33b15 cmp DWORD PTR [rdi],0x0
            fl = sub::<32>(m.r32(rdi) as u64, 0x0u64).1;
            // 33b18 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b1e;
        }
        0x33b1e => {
            // block 33b1e
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b1e cmp DWORD PTR [rdi+0x4],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33b22 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b28;
        }
        0x33b28 => {
            // block 33b28
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b28 cmp WORD PTR [rdi+0x34],0x0
            fl = sub::<16>(m.r16(rdi.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 33b2d je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b33;
        }
        0x33b33 => {
            // block 33b33
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b33 cmp DWORD PTR [rdi+0x24],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 33b37 je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33b3d;
        }
        0x33b3d => {
            // block 33b3d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b3d mov ecx,DWORD PTR [rdi+0x10]
            rcx = (m.r32(rdi.wrapping_add(0x10)) as u64) as u32 as u64;
            // 33b40 cmp rcx,0xb
            fl = sub::<64>(rcx, 0xbu64).1;
            // 33b44 ja 0x33c93
            if fl.a() { pc = 0x33c93; continue; }
            pc = 0x33b4a;
        }
        0x33b4a => {
            // block 33b4a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b4a mov r10d,DWORD PTR [rdi+0x1c]
            r10 = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33b4e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33b50 cmp r10d,0x2
            fl = sub::<32>((r10 as u32 as u64), 0x2u64).1;
            // 33b54 ja 0x33c95
            if fl.a() { pc = 0x33c95; continue; }
            pc = 0x33b5a;
        }
        0x33b5a => {
            // block 33b5a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b5a cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // 33b5d sete r8b
            r8 = (r8 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33b61 setne r9b
            r9 = (r9 & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 33b65 cmp r10d,0x2
            fl = sub::<32>((r10 as u32 as u64), 0x2u64).1;
            // 33b69 je 0x33b85
            if fl.e() { pc = 0x33b85; continue; }
            pc = 0x33b6b;
        }
        0x33b6b => {
            // block 33b6b
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b6b cmp r10d,0x1
            fl = sub::<32>((r10 as u32 as u64), 0x1u64).1;
            // 33b6f jne 0x33b99
            if fl.ne() { pc = 0x33b99; continue; }
            pc = 0x33b71;
        }
        0x33b71 => {
            // block 33b71
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b71 lea rax,[rip+0xcc3d8] # 0xfff50
            rax = m.env.data_va(83741);
            // 33b78 lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33b7c lea rbx,[rip+0xcc38d] # 0xfff10
            rbx = m.env.data_va(83677);
            // 33b83 jmp 0x33bb6
            pc = 0x33bb6; continue;
        }
        0x33b85 => {
            // block 33b85
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b85 lea rax,[rip+0xcc444] # 0xfffd0
            rax = m.env.data_va(83869);
            // 33b8c lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33b90 lea rbx,[rip+0xcc3f9] # 0xfff90
            rbx = m.env.data_va(83805);
            // 33b97 jmp 0x33bb6
            pc = 0x33bb6; continue;
        }
        0x33b99 => {
            // block 33b99
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33b99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33b99 test r10d,r10d
            fl = logic::<32>((r10 as u32 as u64) & (r10 as u32 as u64));
            // 33b9c mov r10d,0x0
            r10 = (0x0u64) as u32 as u64;
            // 33ba2 jne 0x33bc7
            if fl.ne() { pc = 0x33bc7; continue; }
            pc = 0x33ba4;
        }
        0x33ba4 => {
            // block 33ba4
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33ba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ba4 lea rax,[rip+0xcc325] # 0xffed0
            rax = m.env.data_va(83613);
            // 33bab lea rax,[rax+rcx*4]
            rax = rax.wrapping_add(rcx.wrapping_mul(4));
            // 33baf lea rbx,[rip+0xcc2da] # 0xffe90
            rbx = m.env.data_va(83549);
            pc = 0x33bb6;
        }
        0x33bb6 => {
            // block 33bb6
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bb6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bb6 lea r10,[rbx+rcx*4]
            r10 = rbx.wrapping_add(rcx.wrapping_mul(4));
            // 33bba mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 33bbc cmp eax,esi
            fl = sub::<32>((rax as u32 as u64), (rsi as u32 as u64)).1;
            // 33bbe ja 0x33c93
            if fl.a() { pc = 0x33c93; continue; }
            pc = 0x33bc4;
        }
        0x33bc4 => {
            // block 33bc4
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bc4 mov r10d,DWORD PTR [r10]
            r10 = (m.r32(r10) as u64) as u32 as u64;
            pc = 0x33bc7;
        }
        0x33bc7 => {
            // block 33bc7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bc7 lea rbx,[rip+0xcc442] # 0x100010
            rbx = m.env.data_va(83933);
            // 33bce mov r11d,DWORD PTR [rbx+rcx*4]
            r11 = (m.r32(rbx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 33bd2 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 33bd5 ja 0x33bf7
            if fl.a() { pc = 0x33bf7; continue; }
            pc = 0x33bd7;
        }
        0x33bd7 => {
            // block 33bd7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bd7 mov ebx,0x752
            rbx = (0x752u64) as u32 as u64;
            // 33bdc bt ebx,ecx
            fl.cf = bit::<32>((rbx as u32 as u64), (rcx as u32 as u64));
            // 33bdf jae 0x33bf7
            if fl.ae() { pc = 0x33bf7; continue; }
            pc = 0x33be1;
        }
        0x33be1 => {
            // block 33be1
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33be1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33be1 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33be6 bt rbx,rcx
            fl.cf = bit::<64>(rbx, rcx);
            // 33bea jae 0x33c13
            if fl.ae() { pc = 0x33c13; continue; }
            pc = 0x33bec;
        }
        0x33bec => {
            // block 33bec
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bec cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33bef jb 0x33c93
            if fl.b() { pc = 0x33c93; continue; }
            pc = 0x33bf5;
        }
        0x33bf5 => {
            // block 33bf5
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bf5 jmp 0x33c13
            pc = 0x33c13; continue;
        }
        0x33bf7 => {
            // block 33bf7
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33bf7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33bf7 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33bfc shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33bff cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c02 jae 0x33c13
            if fl.ae() { pc = 0x33c13; continue; }
            pc = 0x33c04;
        }
        0x33c04 => {
            // block 33c04
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c04 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c08 je 0x33c13
            if fl.e() { pc = 0x33c13; continue; }
            pc = 0x33c0a;
        }
        0x33c0a => {
            // block 33c0a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c0a cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c0d je 0x33c93
            if fl.e() { pc = 0x33c93; continue; }
            pc = 0x33c13;
        }
        0x33c13 => {
            // block 33c13
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c13 lea r14d,[rcx-0x1]
            r14 = (rcx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33c17 cmp r14d,0xa
            fl = sub::<32>((r14 as u32 as u64), 0xau64).1;
            // 33c1b jae 0x33c28
            if fl.ae() { pc = 0x33c28; continue; }
            pc = 0x33c1d;
        }
        0x33c1d => {
            // block 33c1d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c1d mov ebx,0x3a9
            rbx = (0x3a9u64) as u32 as u64;
            // 33c22 bt ebx,r14d
            fl.cf = bit::<32>((rbx as u32 as u64), (r14 as u32 as u64));
            // 33c26 jb 0x33c53
            if fl.b() { pc = 0x33c53; continue; }
            pc = 0x33c28;
        }
        0x33c28 => {
            // block 33c28
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c28 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33c2d shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33c30 cmp r11d,esi
            fl = sub::<32>((r11 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c33 mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c36 jae 0x33c61
            if fl.ae() { pc = 0x33c61; continue; }
            pc = 0x33c38;
        }
        0x33c38 => {
            // block 33c38
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c38 cmp r10d,esi
            fl = sub::<32>((r10 as u32 as u64), (rsi as u32 as u64)).1;
            // 33c3b mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c3e jbe 0x33c61
            if fl.be() { pc = 0x33c61; continue; }
            pc = 0x33c40;
        }
        0x33c40 => {
            // block 33c40
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c40 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c44 mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c47 je 0x33c61
            if fl.e() { pc = 0x33c61; continue; }
            pc = 0x33c49;
        }
        0x33c49 => {
            // block 33c49
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c49 cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c4c mov r11d,ecx
            r11 = ((rcx as u32 as u64)) as u32 as u64;
            // 33c4f jne 0x33c93
            if fl.ne() { pc = 0x33c93; continue; }
            pc = 0x33c51;
        }
        0x33c51 => {
            // block 33c51
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c51 jmp 0x33c61
            pc = 0x33c61; continue;
        }
        0x33c53 => {
            // block 33c53
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c53 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c53 movsxd r11,r14d
            r11 = sx::<32>((r14 as u32 as u64)) as u64;
            // 33c56 lea rbx,[rip+0xcc433] # 0x100090
            rbx = m.env.data_va(84061);
            // 33c5d mov r11d,DWORD PTR [rbx+r11*4]
            r11 = (m.r32(rbx.wrapping_add(r11.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x33c61;
        }
        0x33c61 => {
            // block 33c61
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c61 cmp r11d,0xa
            fl = sub::<32>((r11 as u32 as u64), 0xau64).1;
            // 33c65 ja 0x33c72
            if fl.a() { pc = 0x33c72; continue; }
            pc = 0x33c67;
        }
        0x33c67 => {
            // block 33c67
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c67 mov ebx,0x752
            rbx = (0x752u64) as u32 as u64;
            // 33c6c bt ebx,r11d
            fl.cf = bit::<32>((rbx as u32 as u64), (r11 as u32 as u64));
            // 33c70 jb 0x33c9a
            if fl.b() { pc = 0x33c9a; continue; }
            pc = 0x33c72;
        }
        0x33c72 => {
            // block 33c72
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c72 lea r14,[rip+0xcc3d7] # 0x100050
            r14 = m.env.data_va(83997);
            // 33c79 mov ebx,0x5f
            rbx = (0x5fu64) as u32 as u64;
            // 33c7e shr rbx,cl
            { let (r, f) = shr::<64>(rbx, (rcx as u8 as u64), fl); rbx = r; fl = f; }
            // 33c81 cmp DWORD PTR [r14+rcx*4],esi
            fl = sub::<32>(m.r32(r14.wrapping_add(rcx.wrapping_mul(4))) as u64, (rsi as u32 as u64)).1;
            // 33c85 jae 0x33c9a
            if fl.ae() { pc = 0x33c9a; continue; }
            pc = 0x33c87;
        }
        0x33c87 => {
            // block 33c87
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c87 and rbx,0x1
            { let r = rbx & 0x1u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 33c8b je 0x33c9a
            if fl.e() { pc = 0x33c9a; continue; }
            pc = 0x33c8d;
        }
        0x33c8d => {
            // block 33c8d
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c8d cmp r11d,0xb
            fl = sub::<32>((r11 as u32 as u64), 0xbu64).1;
            // 33c91 je 0x33c9a
            if fl.e() { pc = 0x33c9a; continue; }
            pc = 0x33c93;
        }
        0x33c93 => {
            // block 33c93
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c93 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33c95;
        }
        0x33c95 => {
            // block 33c95
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 33c96 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 33c98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33c99 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x33c9a => {
            // block 33c9a
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c9a cmp ecx,0xb
            fl = sub::<32>((rcx as u32 as u64), 0xbu64).1;
            // 33c9d ja 0x33cad
            if fl.a() { pc = 0x33cad; continue; }
            pc = 0x33c9f;
        }
        0x33c9f => {
            // block 33c9f
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33c9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33c9f mov ebx,0xf52
            rbx = (0xf52u64) as u32 as u64;
            // 33ca4 bt ebx,ecx
            fl.cf = bit::<32>((rbx as u32 as u64), (rcx as u32 as u64));
            // 33ca7 jae 0x33cad
            if fl.ae() { pc = 0x33cad; continue; }
            pc = 0x33ca9;
        }
        0x33ca9 => {
            // block 33ca9
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33ca9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ca9 sub esi,eax
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rax as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cab jmp 0x33cc9
            pc = 0x33cc9; continue;
        }
        0x33cad => {
            // block 33cad
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cad cmp r10d,esi
            fl = sub::<32>((r10 as u32 as u64), (rsi as u32 as u64)).1;
            // 33cb0 jbe 0x33cbe
            if fl.be() { pc = 0x33cbe; continue; }
            pc = 0x33cb2;
        }
        0x33cb2 => {
            // block 33cb2
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cb2 movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 33cb6 sub esi,eax
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rax as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cb8 add esi,esi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rsi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cba or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cbc jmp 0x33cc9
            pc = 0x33cc9; continue;
        }
        0x33cbe => {
            // block 33cbe
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cbe movzx eax,r9b
            rax = ((r9 as u8 as u64)) as u32 as u64;
            // 33cc2 sub esi,r10d
            { let (r, f) = sub::<32>((rsi as u32 as u64), (r10 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cc5 add esi,esi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rsi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // 33cc7 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x33cc9;
        }
        0x33cc9 => {
            // block 33cc9
            #[cfg(feature = "trace")] eprintln!("f_00033b08 pc 33cc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33cc9 mov DWORD PTR [rdx],esi
            m.w32(rdx, ((rsi as u32 as u64)) as u32);
            // 33ccb cmp esi,DWORD PTR [rdi]
            fl = sub::<32>((rsi as u32 as u64), m.r32(rdi) as u64).1;
            // 33ccd setb al
            rax = (rax & !0xff) | (((fl.b() as u64)) as u8 as u64);
            // 33cd0 jmp 0x33c95
            pc = 0x33c95; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x26f9d size=284
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00026f9d<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x26f9d;
    loop {
        match pc {
        0x26f9d => {
            // block 26f9d
            #[cfg(feature = "trace")] eprintln!("f_00026f9d pc 26f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26f9d push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 26f9e mov rbp,rsp
            rbp = rsp;
            // 26fa1 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 26fa2 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 26fa3 mov rbx,rsi
            rbx = rsi;
            // 26fa6 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // 26fa8 and eax,DWORD PTR [rbx+0x4]
            { let r = (rax as u32 as u64) & m.r32(rbx.wrapping_add(0x4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 26fab cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // 26fae jne 0x26fba
            if fl.ne() { pc = 0x26fba; continue; }
            pc = 0x26fb0;
        }
        0x26fb0 => {
            // block 26fb0
            #[cfg(feature = "trace")] eprintln!("f_00026f9d pc 26fb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26fb0 cmp DWORD PTR [rbx+0x8],0xffffffff
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x8)) as u64, 0xffffffffu64).1;
            // 26fb4 je 0x270ab
            if fl.e() { pc = 0x270ab; continue; }
            pc = 0x26fba;
        }
        0x26fba => {
            // block 26fba
            #[cfg(feature = "trace")] eprintln!("f_00026f9d pc 26fba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 26fba lea rsi,[rip+0x1034be] # 0x12a47f
            rsi = m.env.data_va(257100);
            // 26fc1 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 26fc6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26fcb mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26fce mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26fd2 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26fd6 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26fd9 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26fdc mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26fe0 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 26fe2 mov rdi,rax
            rdi = rax;
            // 26fe5 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 26fea mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 26fed mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 26ff1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 26ff5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26ff8 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 26ffb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 26fff lea rsi,[rip+0x10347e] # 0x12a484
            rsi = m.env.data_va(257105);
            // 27006 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 2700b mov rdi,rax
            rdi = rax;
            // 2700e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27013 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27016 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2701a mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2701e and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27021 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27024 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27028 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2702b mov rdi,rax
            rdi = rax;
            // 2702e call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 27033 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 27036 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2703a mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2703e and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27041 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 27044 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27048 lea rsi,[rip+0x10343a] # 0x12a489
            rsi = m.env.data_va(257110);
            // 2704f mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // 27054 mov rdi,rax
            rdi = rax;
            // 27057 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2705c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2705f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27063 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27067 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2706a or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2706d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27071 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 27074 mov rdi,rax
            rdi = rax;
            // 27077 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2707c mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2707f mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 27083 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 27087 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2708a or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2708d mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 27091 lea rsi,[rip+0x1033f6] # 0x12a48e
            rsi = m.env.data_va(257115);
            // 27098 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2709d mov rdi,rax
            rdi = rax;
            pc = 0x270a0;
        }
        0x270a0 => {
            // block 270a0
            #[cfg(feature = "trace")] eprintln!("f_00026f9d pc 270a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 270a0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 270a4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 270a5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 270a6 jmp 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
        0x270ab => {
            // block 270ab
            #[cfg(feature = "trace")] eprintln!("f_00026f9d pc 270ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 270ab lea rsi,[rip+0x1033de] # 0x12a490
            rsi = m.env.data_va(257117);
            // 270b2 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 270b7 jmp 0x270a0
            pc = 0x270a0; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28ce size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000028ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28ce;
    loop {
        match pc {
        0x28ce => {
            // block 28ce
            #[cfg(feature = "trace")] eprintln!("f_000028ce pc 28ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28cf mov rbp,rsp
            rbp = rsp;
            // 28d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28d5 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 28d8 mov rbx,rdi
            rbx = rdi;
            // 28db mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28de call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28e4 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 28e6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 28e8 cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // 28ec ja 0x290f
            if fl.a() { pc = 0x290f; continue; }
            pc = 0x28ee;
        }
        0x28ee => {
            // block 28ee
            #[cfg(feature = "trace")] eprintln!("f_000028ce pc 28ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ee xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 28f1 je 0x290f
            if fl.e() { pc = 0x290f; continue; }
            pc = 0x28f3;
        }
        0x28f3 => {
            // block 28f3
            #[cfg(feature = "trace")] eprintln!("f_000028ce pc 28f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28f6 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 28fb mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 2900 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 2906 mov rdi,rbx
            rdi = rbx;
            // 2909 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 290c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x290f;
        }
        0x290f => {
            // block 290f
            #[cfg(feature = "trace")] eprintln!("f_000028ce pc 290f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2910 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2912 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2913 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1100 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1100;
    loop {
        match pc {
        0x1100 => {
            // block 1100
            #[cfg(feature = "trace")] eprintln!("f_00001100 pc 1100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1101 mov rbp,rsp
            rbp = rsp;
            // 1104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1108 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 110a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 110b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 110f mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 1112 mov r14,rsi
            r14 = rsi;
            // 1115 mov rbx,rdi
            rbx = rdi;
            // 1118 mov DWORD PTR [rbp-0x24],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xcu64) as u32);
            // 111f mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 1123 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1126 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 112c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 112f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1131 cmove r12d,r15d
            if fl.e() { r12 = ((r15 as u32 as u64)) as u32 as u64; } else { r12 = r12 as u32 as u64; }
            // 1135 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1138 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 113c mov rdi,rbx
            rdi = rbx;
            // 113f mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // 1142 call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1148 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 114a je 0x115e
            if fl.e() { pc = 0x115e; continue; }
            pc = 0x114c;
        }
        0x114c => {
            // block 114c
            #[cfg(feature = "trace")] eprintln!("f_00001100 pc 114c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 114c mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 114f and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1152 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 1155 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 1158 jne 0x115e
            if fl.ne() { pc = 0x115e; continue; }
            pc = 0x115a;
        }
        0x115a => {
            // block 115a
            #[cfg(feature = "trace")] eprintln!("f_00001100 pc 115a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 115a mov BYTE PTR [r14],0x1
            m.w8(r14, (0x1u64) as u8);
            pc = 0x115e;
        }
        0x115e => {
            // block 115e
            #[cfg(feature = "trace")] eprintln!("f_00001100 pc 115e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 115e mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 1161 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1165 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1166 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1168 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 116a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 116c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 116d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3101e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0003101e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3101e;
    loop {
        match pc {
        0x3101e => {
            // block 3101e
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3101e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3101e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3101f mov rbp,rsp
            rbp = rsp;
            // 31022 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31024 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31025 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 31029 mov r14,rdx
            r14 = rdx;
            // 3102c mov rbx,rdi
            rbx = rdi;
            // 3102f xorps xmm0,xmm0
            x0 = 0;
            // 31032 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 31036 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 3103e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31041 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 31045 mov ecx,DWORD PTR [r14+0x8]
            rcx = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // 31049 sub ecx,DWORD PTR [r14]
            { let (r, f) = sub::<32>((rcx as u32 as u64), m.r32(r14) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3104c lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31050 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31052 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31054 je 0x3108d
            if fl.e() { pc = 0x3108d; continue; }
            pc = 0x31056;
        }
        0x31056 => {
            // block 31056
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31056 movzx edx,BYTE PTR [rbx+0x8]
            rdx = (m.r8(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 3105a lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 3105e mov rdi,r14
            rdi = r14;
            // 31061 call 0x2eee0
            { let r = { let t = m.env.code_va(0x2eee0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31066 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31068 jmp 0x3108f
            pc = 0x3108f; continue;
        }
        0x3106a => {
            // block 3106a
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3106a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3106a mov rbx,rax
            rbx = rax;
            // 3106d mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31071 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31074 je 0x31085
            if fl.e() { pc = 0x31085; continue; }
            pc = 0x31076;
        }
        0x31076 => {
            // block 31076
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31076 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31076 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3107a je 0x31080
            if fl.e() { pc = 0x31080; continue; }
            pc = 0x3107c;
        }
        0x3107c => {
            // block 3107c
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3107c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3107c mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x31080;
        }
        0x31080 => {
            // block 31080
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31080 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31085;
        }
        0x31085 => {
            // block 31085
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31085 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31085 mov rdi,rbx
            rdi = rbx;
            // 31088 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x3108d => {
            // block 3108d
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3108d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3108d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x3108f;
        }
        0x3108f => {
            // block 3108f
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3108f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3108f mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31093 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31096 je 0x310a7
            if fl.e() { pc = 0x310a7; continue; }
            pc = 0x31098;
        }
        0x31098 => {
            // block 31098
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31098 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 3109c je 0x310a2
            if fl.e() { pc = 0x310a2; continue; }
            pc = 0x3109e;
        }
        0x3109e => {
            // block 3109e
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 3109e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3109e mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x310a2;
        }
        0x310a2 => {
            // block 310a2
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 310a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 310a2 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x310a7;
        }
        0x310a7 => {
            // block 310a7
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 310a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 310a7 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 310a9 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 310ad pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 310ae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 310b0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 310b1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
