//! Mechanically lifted functions (scripts/lift_run.py), module q005. Each one is checked against the original
//! in the world harness (tests/world_q005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x5987b size=525
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005987b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5987b;
    loop {
        match pc {
        0x5987b => {
            // block 5987b
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5987b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5987b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5987c mov rbp,rsp
            rbp = rsp;
            // 5987f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 59881 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 59883 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 59885 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 59887 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 59888 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 5988c mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 5988f mov rbx,rsi
            rbx = rsi;
            // 59892 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 59895 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 59898 je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x5989e;
        }
        0x5989e => {
            // block 5989e
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5989e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5989e test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 598a1 je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x598a7;
        }
        0x598a7 => {
            // block 598a7
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598a7 test r14w,r14w
            fl = logic::<16>((r14 as u16 as u64) & (r14 as u16 as u64));
            // 598ab je 0x59a5a
            if fl.e() { pc = 0x59a5a; continue; }
            pc = 0x598b1;
        }
        0x598b1 => {
            // block 598b1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598b1 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 598b4 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 598b6 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 598b9 ja 0x59a5a
            if fl.a() { pc = 0x59a5a; continue; }
            pc = 0x598bf;
        }
        0x598bf => {
            // block 598bf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598bf lea rcx,[rip+0x1a6] # 0x59a6c
            rcx = m.env.code_va(0x59a6c);
            // 598c6 movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // 598ca add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 598cd jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x598cf => {
            // block 598cf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598cf mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 598d1 and ecx,0xfffc
            { let r = (rcx as u32 as u64) & 0xfffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 598d7 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 598da xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 598dc test cx,cx
            fl = logic::<16>((rcx as u16 as u64) & (rcx as u16 as u64));
            // 598df je 0x59905
            if fl.e() { pc = 0x59905; continue; }
            pc = 0x598e1;
        }
        0x598e1 => {
            // block 598e1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e1 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            pc = 0x598e4;
        }
        0x598e4 => {
            // block 598e4
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e4 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 598e7 je 0x598f7
            if fl.e() { pc = 0x598f7; continue; }
            pc = 0x598e9;
        }
        0x598e9 => {
            // block 598e9
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598e9 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // 598ec je 0x598f7
            if fl.e() { pc = 0x598f7; continue; }
            pc = 0x598ee;
        }
        0x598ee => {
            // block 598ee
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598ee cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 598f1 jne 0x59a57
            if fl.ne() { pc = 0x59a57; continue; }
            pc = 0x598f7;
        }
        0x598f7 => {
            // block 598f7
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 598f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 598f7 mov BYTE PTR [rbx+rax*4],0x0
            m.w8(rbx.wrapping_add(rax.wrapping_mul(4)), (0x0u64) as u8);
            // 598fb inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 598fe movzx esi,ax
            rsi = ((rax as u16 as u64)) as u32 as u64;
            // 59901 cmp esi,ecx
            fl = sub::<32>((rsi as u32 as u64), (rcx as u32 as u64)).1;
            // 59903 jb 0x598e4
            if fl.b() { pc = 0x598e4; continue; }
            pc = 0x59905;
        }
        0x59905 => {
            // block 59905
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59905 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59905 movzx r12d,dx
            r12 = ((rdx as u16 as u64)) as u32 as u64;
            // 59909 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 5990c movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 59910 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 59913 jb 0x59a5a
            if fl.b() { pc = 0x59a5a; continue; }
            pc = 0x59919;
        }
        0x59919 => {
            // block 59919
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59919 lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 5991d dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            pc = 0x59920;
        }
        0x59920 => {
            // block 59920
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59920 mov rdi,r13
            rdi = r13;
            // 59923 mov rsi,rbx
            rsi = rbx;
            // 59926 mov rdx,r12
            rdx = r12;
            // 59929 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5992e add r13,r12
            { let (r, f) = add::<64>(r13, r12);
              r13 = r;
              fl = f; }
            // 59931 dec r14w
            { let (r, f) = dec::<16>((r14 as u16 as u64), fl); r14 = (r14 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 59935 jne 0x59920
            if fl.ne() { pc = 0x59920; continue; }
            pc = 0x59937;
        }
        0x59937 => {
            // block 59937
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59937 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59937 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x5993c => {
            // block 5993c
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5993c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5993c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 5993e and eax,0xfff0
            { let r = (rax as u32 as u64) & 0xfff0u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 59943 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 59946 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // 59949 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 5994b lea r12d,[rax+rax*2]
            r12 = (rax.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 5994f movzx eax,r12w
            rax = ((r12 as u16 as u64)) as u32 as u64;
            // 59953 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 59956 movzx eax,dx
            rax = ((rdx as u16 as u64)) as u32 as u64;
            // 59959 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 5995d and r12,0xfffe
            { let r = r12 & 0xfffeu64; let f = logic::<64>(r);
              r12 = r;
              fl = f; }
            // 59964 add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // 59967 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x5996a;
        }
        0x5996a => {
            // block 5996a
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5996a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5996a mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 5996d test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 59970 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 59975 je 0x5998d
            if fl.e() { pc = 0x5998d; continue; }
            pc = 0x59977;
        }
        0x59977 => {
            // block 59977
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59977 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59977 mov WORD PTR [rbx+rax*2],0x200
            m.w16(rbx.wrapping_add(rax.wrapping_mul(2)), (0x200u64) as u16);
            // 5997d mov WORD PTR [rbx+rax*2+0x2],0x40
            m.w16(rbx.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2), (0x40u64) as u16);
            // 59984 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 59988 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 5998b jl 0x59977
            if fl.l() { pc = 0x59977; continue; }
            pc = 0x5998d;
        }
        0x5998d => {
            // block 5998d
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 5998d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5998d mov rdi,rbx
            rdi = rbx;
            // 59990 mov rsi,rbx
            rsi = rbx;
            // 59993 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 59996 call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5999b add rbx,QWORD PTR [rbp-0x38]
            { let (r, f) = add::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64);
              rbx = r;
              fl = f; }
            // 5999f inc r13d
            { let (r, f) = inc::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // 599a2 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // 599a6 movzx ecx,r14w
            rcx = ((r14 as u16 as u64)) as u32 as u64;
            // 599aa mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 599ad cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // 599af jne 0x5996a
            if fl.ne() { pc = 0x5996a; continue; }
            pc = 0x599b1;
        }
        0x599b1 => {
            // block 599b1
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599b1 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x599b6 => {
            // block 599b6
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599b6 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 599b8 and eax,0xfffe
            { let r = (rax as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 599bd shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 599bf movzx r8d,dx
            r8 = ((rdx as u16 as u64)) as u32 as u64;
            // 599c3 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 599c5 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // 599c7 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 599c9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 599cb movzx edi,r14w
            rdi = ((r14 as u16 as u64)) as u32 as u64;
            pc = 0x599cf;
        }
        0x599cf => {
            // block 599cf
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599cf test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 599d2 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 599d7 je 0x599eb
            if fl.e() { pc = 0x599eb; continue; }
            pc = 0x599d9;
        }
        0x599d9 => {
            // block 599d9
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599d9 mov BYTE PTR [rbx+rcx*1],0x80
            m.w8(rbx.wrapping_add(rcx), (0x80u64) as u8);
            // 599dd mov BYTE PTR [rbx+rcx*1+0x1],0x10
            m.w8(rbx.wrapping_add(rcx).wrapping_add(0x1), (0x10u64) as u8);
            // 599e2 add rcx,0x2
            { let (r, f) = add::<64>(rcx, 0x2u64);
              rcx = r;
              fl = f; }
            // 599e6 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 599e9 jl 0x599d9
            if fl.l() { pc = 0x599d9; continue; }
            pc = 0x599eb;
        }
        0x599eb => {
            // block 599eb
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599eb add rbx,r8
            { let (r, f) = add::<64>(rbx, r8);
              rbx = r;
              fl = f; }
            // 599ee inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 599f0 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // 599f3 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 599f6 cmp ecx,edi
            fl = sub::<32>((rcx as u32 as u64), (rdi as u32 as u64)).1;
            // 599f8 jne 0x599cf
            if fl.ne() { pc = 0x599cf; continue; }
            pc = 0x599fa;
        }
        0x599fa => {
            // block 599fa
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599fa jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x599fc => {
            // block 599fc
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 599fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 599fc mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 599fe and eax,0xfffc
            { let r = (rax as u32 as u64) & 0xfffcu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 59a03 movzx r12d,dx
            r12 = ((rdx as u16 as u64)) as u32 as u64;
            // 59a07 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 59a0a test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // 59a0d je 0x59a2b
            if fl.e() { pc = 0x59a2b; continue; }
            pc = 0x59a0f;
        }
        0x59a0f => {
            // block 59a0f
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a0f dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 59a11 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 59a14 lea rdx,[rax*4+0x4]
            rdx = rax.wrapping_mul(4).wrapping_add(0x4);
            // 59a1c lea rsi,[rip+0xa824d] # 0x101c70
            rsi = m.env.data_va(91197);
            // 59a23 mov rdi,rbx
            rdi = rbx;
            // 59a26 call 0xec230
            { let r = { let t = m.env.stub_va(0x9fd); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x59a2b;
        }
        0x59a2b => {
            // block 59a2b
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a2b mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 59a2e movzx eax,r14w
            rax = ((r14 as u16 as u64)) as u32 as u64;
            // 59a32 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 59a35 jb 0x59a5a
            if fl.b() { pc = 0x59a5a; continue; }
            pc = 0x59a37;
        }
        0x59a37 => {
            // block 59a37
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a37 lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 59a3b dec r14d
            { let (r, f) = dec::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            pc = 0x59a3e;
        }
        0x59a3e => {
            // block 59a3e
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a3e mov rdi,r13
            rdi = r13;
            // 59a41 mov rsi,rbx
            rsi = rbx;
            // 59a44 mov rdx,r12
            rdx = r12;
            // 59a47 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59a4c add r13,r12
            { let (r, f) = add::<64>(r13, r12);
              r13 = r;
              fl = f; }
            // 59a4f dec r14w
            { let (r, f) = dec::<16>((r14 as u16 as u64), fl); r14 = (r14 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // 59a53 jne 0x59a3e
            if fl.ne() { pc = 0x59a3e; continue; }
            pc = 0x59a55;
        }
        0x59a55 => {
            // block 59a55
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a55 jmp 0x59a5a
            pc = 0x59a5a; continue;
        }
        0x59a57 => {
            // block 59a57
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a57 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x59a5a;
        }
        0x59a5a => {
            // block 59a5a
            #[cfg(feature = "trace")] eprintln!("f_0005987b pc 59a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59a5a mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 59a5d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 59a61 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 59a62 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 59a64 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 59a66 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 59a68 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 59a6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 59a6b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdee52 size=508
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dee52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdee52;
    loop {
        match pc {
        0xdee52 => {
            // block dee52
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc dee52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dee52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dee53 mov rbp,rsp
            rbp = rsp;
            // dee56 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dee58 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dee5a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dee5c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dee5d sub rsp,0x50
            { let (r, f) = sub::<64>(rsp, 0x50u64);
              rsp = r;
              fl = f; }
            // dee61 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // dee64 mov r14,rsi
            r14 = rsi;
            // dee67 mov rbx,rdi
            rbx = rdi;
            // dee6a test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // dee6d jne 0xdee87
            if fl.ne() { pc = 0xdee87; continue; }
            pc = 0xdee6f;
        }
        0xdee6f => {
            // block dee6f
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc dee6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dee6f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee72 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // dee74 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dee76 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dee78 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // dee7b mov rdi,rbx
            rdi = rbx;
            // dee7e call QWORD PTR [rax+0x19d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x19d0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee84 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xdee87;
        }
        0xdee87 => {
            // block dee87
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc dee87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dee87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dee8a mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // dee90 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // dee94 mov rdi,rbx
            rdi = rbx;
            // dee97 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dee9d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deea0 movzx esi,BYTE PTR [rbx+0x278]
            rsi = (m.r8(rbx.wrapping_add(0x278)) as u64) as u32 as u64;
            // deea7 mov rdi,rbx
            rdi = rbx;
            // deeaa call QWORD PTR [rax+0x570]
            { let r = { let t = m.r64(rax.wrapping_add(0x570)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deeb0 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // deeb3 and ecx,0xfffffffe
            { let r = (rcx as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // deeb6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deeb9 mov rax,QWORD PTR [rax+0x318]
            rax = m.r64(rax.wrapping_add(0x318)) as u64;
            // deec0 cmp ecx,0x10
            fl = sub::<32>((rcx as u32 as u64), 0x10u64).1;
            // deec3 jne 0xdef2d
            if fl.ne() { pc = 0xdef2d; continue; }
            pc = 0xdeec5;
        }
        0xdeec5 => {
            // block deec5
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc deec5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // deec5 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // deecb xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // deecd mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // deed2 mov rdi,rbx
            rdi = rbx;
            // deed5 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deed7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deeda mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // deee0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // deee5 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // deeea mov rdi,rbx
            rdi = rbx;
            // deeed call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // deef3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // deef6 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // deefc mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // def01 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // def06 mov rdi,rbx
            rdi = rbx;
            // def09 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // def0f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // def12 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // def18 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // def1d mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // def22 mov rdi,rbx
            rdi = rbx;
            // def25 call QWORD PTR [rax+0x318]
            { let r = { let t = m.r64(rax.wrapping_add(0x318)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // def2b jmp 0xdef43
            pc = 0xdef43; continue;
        }
        0xdef2d => {
            // block def2d
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc def2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // def2d mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // def33 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // def39 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // def3e mov rdi,rbx
            rdi = rbx;
            // def41 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdef43;
        }
        0xdef43 => {
            // block def43
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc def43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // def43 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // def46 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // def4c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // def4e mov rdi,rbx
            rdi = rbx;
            // def51 call QWORD PTR [rax+0x4f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4f8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // def57 cmp BYTE PTR [rbx+0x279],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x279)) as u64, 0x0u64).1;
            // def5e je 0xdf002
            if fl.e() { pc = 0xdf002; continue; }
            pc = 0xdef64;
        }
        0xdef64 => {
            // block def64
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc def64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // def64 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // def67 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // def6d lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // def71 mov rdi,rbx
            rdi = rbx;
            // def74 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // def7a mov r12d,DWORD PTR [rbp-0x28]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // def7e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // def81 mov rdi,rbx
            rdi = rbx;
            // def84 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // def8a cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // def8d jbe 0xdef96
            if fl.be() { pc = 0xdef96; continue; }
            pc = 0xdef8f;
        }
        0xdef8f => {
            // block def8f
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc def8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // def8f mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            pc = 0xdef96;
        }
        0xdef96 => {
            // block def96
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc def96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // def96 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // def99 mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // def9f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // defa3 mov rdi,rbx
            rdi = rbx;
            // defa6 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // defac mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // defaf mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // defb5 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // defb9 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // defbb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // defbe xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // defc1 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // defc6 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // defc9 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // defcc mov edx,DWORD PTR [rbp-0x68]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // defcf mov esi,0x458
            rsi = (0x458u64) as u32 as u64;
            // defd4 cmovne esi,edx
            if fl.ne() { rsi = ((rdx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // defd7 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // defda mov ecx,0x45a
            rcx = (0x45au64) as u32 as u64;
            // defdf cmovne ecx,esi
            if fl.ne() { rcx = ((rsi as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // defe2 sub ecx,edx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // defe4 imul ecx,DWORD PTR [rbp-0x60]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // defe8 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // defeb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // defee mov esi,DWORD PTR [rbp-0x28]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // deff1 mov rdi,rbx
            rdi = rbx;
            // deff4 mov rdx,r14
            rdx = r14;
            // deff7 mov r8d,r15d
            r8 = ((r15 as u32 as u64)) as u32 as u64;
            // deffa call QWORD PTR [rax+0x2b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x2b0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df000 jmp 0xdf02f
            pc = 0xdf02f; continue;
        }
        0xdf002 => {
            // block df002
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc df002 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df002 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // df00a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df00d mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df013 lea rdx,[rbp-0x70]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df017 mov rdi,rbx
            rdi = rbx;
            // df01a call QWORD PTR [rax+0xa10]
            { let r = { let t = m.r64(rax.wrapping_add(0xa10)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df020 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // df024 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // df027 mov rsi,r14
            rsi = r14;
            // df02a call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdf02f;
        }
        0xdf02f => {
            // block df02f
            #[cfg(feature = "trace")] eprintln!("f_000dee52 pc df02f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df02f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df032 mov esi,DWORD PTR [rbx+0x250]
            rsi = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df038 mov rdi,rbx
            rdi = rbx;
            // df03b call QWORD PTR [rax+0x618]
            { let r = { let t = m.r64(rax.wrapping_add(0x618)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // df041 add rsp,0x50
            { let (r, f) = add::<64>(rsp, 0x50u64);
              rsp = r;
              fl = f; }
            // df045 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df046 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // df048 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df04a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // df04c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df04d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f282 size=504
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f282<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f282;
    loop {
        match pc {
        0x2f282 => {
            // block 2f282
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f282 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f283 mov rbp,rsp
            rbp = rsp;
            // 2f286 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2f288 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f28a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2f28c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2f28e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f28f sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 2f296 mov rbx,r9
            rbx = r9;
            // 2f299 mov QWORD PTR [rbp-0xb8],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (r8) as u64);
            // 2f2a0 mov r12,rcx
            r12 = rcx;
            // 2f2a3 mov r13,rdx
            r13 = rdx;
            // 2f2a6 mov r15,rsi
            r15 = rsi;
            // 2f2a9 mov r14,rdi
            r14 = rdi;
            // 2f2ac xorps xmm0,xmm0
            x0 = 0;
            // 2f2af movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 2f2b3 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2f2bb mov BYTE PTR [rbp-0x41],0x9f
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffbf), (0x9fu64) as u8);
            // 2f2bf lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2f2c3 lea rsi,[rbp-0x41]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbf);
            // 2f2c7 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f2cc xorps xmm0,xmm0
            x0 = 0;
            // 2f2cf movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 2f2d3 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 2f2db movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 2f2df mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 2f2e7 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 2f2ee lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2f2f2 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f2f7 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 2f2fe lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2f302 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f307 lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 2f30e lea rdx,[rbp-0xb0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 2f315 lea rcx,[rbp-0x80]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2f319 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // 2f31f mov rdi,r14
            rdi = r14;
            // 2f322 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f327 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2f32a mov rdi,QWORD PTR [rbp-0xb0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 2f331 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f334 je 0x2f34b
            if fl.e() { pc = 0x2f34b; continue; }
            pc = 0x2f336;
        }
        0x2f336 => {
            // block 2f336
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f336 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f336 cmp QWORD PTR [rbp-0xa8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, rdi).1;
            // 2f33d je 0x2f346
            if fl.e() { pc = 0x2f346; continue; }
            pc = 0x2f33f;
        }
        0x2f33f => {
            // block 2f33f
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f33f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f33f mov QWORD PTR [rbp-0xa8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rdi) as u64);
            pc = 0x2f346;
        }
        0x2f346 => {
            // block 2f346
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f346 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f34b;
        }
        0x2f34b => {
            // block 2f34b
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f34b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f34b mov rdi,QWORD PTR [rbp-0x98]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 2f352 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f355 je 0x2f36c
            if fl.e() { pc = 0x2f36c; continue; }
            pc = 0x2f357;
        }
        0x2f357 => {
            // block 2f357
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f357 cmp QWORD PTR [rbp-0x90],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64, rdi).1;
            // 2f35e je 0x2f367
            if fl.e() { pc = 0x2f367; continue; }
            pc = 0x2f360;
        }
        0x2f360 => {
            // block 2f360
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f360 mov QWORD PTR [rbp-0x90],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rdi) as u64);
            pc = 0x2f367;
        }
        0x2f367 => {
            // block 2f367
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f367 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f367 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f36c;
        }
        0x2f36c => {
            // block 2f36c
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f36c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f36c test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 2f36f je 0x2f3a8
            if fl.e() { pc = 0x2f3a8; continue; }
            pc = 0x2f371;
        }
        0x2f371 => {
            // block 2f371
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f371 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f371 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 2f375 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2f379 sub rax,rdi
            { let (r, f) = sub::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 2f37c cmp rax,0x6
            fl = sub::<64>(rax, 0x6u64).1;
            // 2f380 jne 0x2f3ac
            if fl.ne() { pc = 0x2f3ac; continue; }
            pc = 0x2f382;
        }
        0x2f382 => {
            // block 2f382
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f382 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f382 mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 2f384 mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // 2f387 mov al,BYTE PTR [rdi+0x1]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x1)) as u64) as u8 as u64);
            // 2f38a mov BYTE PTR [r13+0x0],al
            m.w8(r13, ((rax as u8 as u64)) as u8);
            // 2f38e mov al,BYTE PTR [rdi+0x2]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x2)) as u64) as u8 as u64);
            // 2f391 mov BYTE PTR [r12],al
            m.w8(r12, ((rax as u8 as u64)) as u8);
            // 2f395 mov al,BYTE PTR [rdi+0x4]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x4)) as u64) as u8 as u64);
            // 2f398 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 2f39f mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 2f3a1 mov al,BYTE PTR [rdi+0x5]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x5)) as u64) as u8 as u64);
            // 2f3a4 mov BYTE PTR [rbx],al
            m.w8(rbx, ((rax as u8 as u64)) as u8);
            // 2f3a6 jmp 0x2f3b1
            pc = 0x2f3b1; continue;
        }
        0x2f3a8 => {
            // block 2f3a8
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3a8 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0x2f3ac;
        }
        0x2f3ac => {
            // block 2f3ac
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3ac test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f3af je 0x2f3c0
            if fl.e() { pc = 0x2f3c0; continue; }
            pc = 0x2f3b1;
        }
        0x2f3b1 => {
            // block 2f3b1
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3b1 cmp QWORD PTR [rbp-0x78],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, rdi).1;
            // 2f3b5 je 0x2f3bb
            if fl.e() { pc = 0x2f3bb; continue; }
            pc = 0x2f3b7;
        }
        0x2f3b7 => {
            // block 2f3b7
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3b7 mov QWORD PTR [rbp-0x78],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rdi) as u64);
            pc = 0x2f3bb;
        }
        0x2f3bb => {
            // block 2f3bb
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3bb call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f3c0;
        }
        0x2f3c0 => {
            // block 2f3c0
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3c0 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2f3c4 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f3c7 je 0x2f3d8
            if fl.e() { pc = 0x2f3d8; continue; }
            pc = 0x2f3c9;
        }
        0x2f3c9 => {
            // block 2f3c9
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3c9 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // 2f3cd je 0x2f3d3
            if fl.e() { pc = 0x2f3d3; continue; }
            pc = 0x2f3cf;
        }
        0x2f3cf => {
            // block 2f3cf
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3cf mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0x2f3d3;
        }
        0x2f3d3 => {
            // block 2f3d3
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3d3 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f3d8;
        }
        0x2f3d8 => {
            // block 2f3d8
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3d8 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 2f3db add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 2f3e2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f3e3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2f3e5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2f3e7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f3e9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2f3eb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f3ec ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2f3ed => {
            // block 2f3ed
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3ed mov rbx,rax
            rbx = rax;
            // 2f3f0 jmp 0x2f459
            pc = 0x2f459; continue;
        }
        0x2f3f2 => {
            // block 2f3f2
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3f2 mov rbx,rax
            rbx = rax;
            // 2f3f5 jmp 0x2f441
            pc = 0x2f441; continue;
        }
        0x2f3f7 => {
            // block 2f3f7
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3f7 mov rbx,rax
            rbx = rax;
            // 2f3fa jmp 0x2f420
            pc = 0x2f420; continue;
        }
        0x2f3fc => {
            // block 2f3fc
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f3fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f3fc mov rbx,rax
            rbx = rax;
            // 2f3ff mov rdi,QWORD PTR [rbp-0xb0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 2f406 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f409 je 0x2f420
            if fl.e() { pc = 0x2f420; continue; }
            pc = 0x2f40b;
        }
        0x2f40b => {
            // block 2f40b
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f40b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f40b cmp QWORD PTR [rbp-0xa8],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, rdi).1;
            // 2f412 je 0x2f41b
            if fl.e() { pc = 0x2f41b; continue; }
            pc = 0x2f414;
        }
        0x2f414 => {
            // block 2f414
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f414 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f414 mov QWORD PTR [rbp-0xa8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rdi) as u64);
            pc = 0x2f41b;
        }
        0x2f41b => {
            // block 2f41b
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f41b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f41b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f420;
        }
        0x2f420 => {
            // block 2f420
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f420 mov rdi,QWORD PTR [rbp-0x98]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 2f427 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f42a je 0x2f441
            if fl.e() { pc = 0x2f441; continue; }
            pc = 0x2f42c;
        }
        0x2f42c => {
            // block 2f42c
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f42c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f42c cmp QWORD PTR [rbp-0x90],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64, rdi).1;
            // 2f433 je 0x2f43c
            if fl.e() { pc = 0x2f43c; continue; }
            pc = 0x2f435;
        }
        0x2f435 => {
            // block 2f435
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f435 mov QWORD PTR [rbp-0x90],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rdi) as u64);
            pc = 0x2f43c;
        }
        0x2f43c => {
            // block 2f43c
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f43c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f43c call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f441;
        }
        0x2f441 => {
            // block 2f441
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f441 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 2f445 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f448 je 0x2f459
            if fl.e() { pc = 0x2f459; continue; }
            pc = 0x2f44a;
        }
        0x2f44a => {
            // block 2f44a
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f44a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f44a cmp QWORD PTR [rbp-0x78],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, rdi).1;
            // 2f44e je 0x2f454
            if fl.e() { pc = 0x2f454; continue; }
            pc = 0x2f450;
        }
        0x2f450 => {
            // block 2f450
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f450 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f450 mov QWORD PTR [rbp-0x78],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rdi) as u64);
            pc = 0x2f454;
        }
        0x2f454 => {
            // block 2f454
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f454 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f459;
        }
        0x2f459 => {
            // block 2f459
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f459 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2f45d test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f460 je 0x2f471
            if fl.e() { pc = 0x2f471; continue; }
            pc = 0x2f462;
        }
        0x2f462 => {
            // block 2f462
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f462 cmp QWORD PTR [rbp-0x38],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64, rdi).1;
            // 2f466 je 0x2f46c
            if fl.e() { pc = 0x2f46c; continue; }
            pc = 0x2f468;
        }
        0x2f468 => {
            // block 2f468
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f468 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f468 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            pc = 0x2f46c;
        }
        0x2f46c => {
            // block 2f46c
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f46c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f46c call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f471;
        }
        0x2f471 => {
            // block 2f471
            #[cfg(feature = "trace")] eprintln!("f_0002f282 pc 2f471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f471 mov rdi,rbx
            rdi = rbx;
            // 2f474 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe938c size=496
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e938c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe938c;
    loop {
        match pc {
        0xe938c => {
            // block e938c
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e938c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e938c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e938d mov rbp,rsp
            rbp = rsp;
            // e9390 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9392 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9394 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9396 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9398 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9399 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e939d mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e93a0 mov r14,rdi
            r14 = rdi;
            // e93a3 lea rsi,[rip+0x53f63] # 0x13d30d
            rsi = m.env.data_va(334554);
            // e93aa lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e93ae mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // e93b3 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e93b8 test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // e93bc jne 0xe93c7
            if fl.ne() { pc = 0xe93c7; continue; }
            pc = 0xe93be;
        }
        0xe93be => {
            // block e93be
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93be lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // e93c2 mov rdi,r12
            rdi = r12;
            // e93c5 jmp 0xe93cf
            pc = 0xe93cf; continue;
        }
        0xe93c7 => {
            // block e93c7
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93c7 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e93cb lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            pc = 0xe93cf;
        }
        0xe93cf => {
            // block e93cf
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93cf add rdi,0x9
            { let (r, f) = add::<64>(rdi, 0x9u64);
              rdi = r;
              fl = f; }
            // e93d3 movzx edx,r15w
            rdx = ((r15 as u16 as u64)) as u32 as u64;
            // e93d7 lea rsi,[rip+0x53f3f] # 0x13d31d
            rsi = m.env.data_va(334570);
            // e93de xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e93e0 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e93e2 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e93e7 mov cl,BYTE PTR [rbp-0x40]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u8 as u64);
            // e93ea and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e93ed mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e93f2 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xe93f6;
        }
        0xe93f6 => {
            // block e93f6
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e93f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e93f6 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e93f8 mov rax,rdi
            rax = rdi;
            // e93fb cmove rax,r12
            if fl.e() { rax = r12; }
            // e93ff movzx eax,BYTE PTR [rax+rsi*1]
            rax = (m.r8(rax.wrapping_add(rsi)) as u64) as u32 as u64;
            // e9403 lea eax,[rbx+rax*1+0xd0]
            rax = (rbx.wrapping_add(rax).wrapping_add(0xd0)) as u32 as u64;
            // e940a inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // e940d movzx ebx,al
            rbx = ((rax as u8 as u64)) as u32 as u64;
            // e9410 cmp rsi,0xd
            fl = sub::<64>(rsi, 0xdu64).1;
            // e9414 jne 0xe93f6
            if fl.ne() { pc = 0xe93f6; continue; }
            pc = 0xe9416;
        }
        0xe9416 => {
            // block e9416
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9416 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9416 neg eax
            { let (r, f) = neg::<32>((rax as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e9418 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e941a cmove rdi,r12
            if fl.e() { rdi = r12; }
            // e941e add rdi,0xd
            { let (r, f) = add::<64>(rdi, 0xdu64);
              rdi = r;
              fl = f; }
            // e9422 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // e9425 lea rsi,[rip+0x53ef6] # 0x13d322
            rsi = m.env.data_va(334575);
            // e942c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e942e call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9433 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e9437 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e943b lea r8,[rbp-0x60]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // e943f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9441 mov ecx,0xd
            rcx = (0xdu64) as u32 as u64;
            // e9446 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e944b mov rbx,QWORD PTR [r14+0x268]
            rbx = m.r64(r14.wrapping_add(0x268)) as u64;
            // e9452 mov rax,QWORD PTR [r14+0x270]
            rax = m.r64(r14.wrapping_add(0x270)) as u64;
            // e9459 mov QWORD PTR [r14+0x280],rax
            m.w64(r14.wrapping_add(0x280), (rax) as u64);
            // e9460 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // e9463 je 0xe952d
            if fl.e() { pc = 0xe952d; continue; }
            pc = 0xe9469;
        }
        0xe9469 => {
            // block e9469
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9469 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9469 lea r13,[rbp-0x77]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // e946d lea r15,[rbp-0x78]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e9471 lea r12,[rbp-0x80]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            pc = 0xe9475;
        }
        0xe9475 => {
            // block e9475
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9475 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9477 mov ecx,0xd
            rcx = (0xdu64) as u32 as u64;
            // e947c mov rdi,r15
            rdi = r15;
            // e947f mov rsi,rbx
            rsi = rbx;
            // e9482 mov r8,r12
            r8 = r12;
            // e9485 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e948a movzx edx,BYTE PTR [rbp-0x58]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // e948e mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e9490 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e9492 jne 0xe9499
            if fl.ne() { pc = 0xe9499; continue; }
            pc = 0xe9494;
        }
        0xe9494 => {
            // block e9494
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9494 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9494 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e9497 jmp 0xe949d
            pc = 0xe949d; continue;
        }
        0xe9499 => {
            // block e9499
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9499 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9499 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xe949d;
        }
        0xe949d => {
            // block e949d
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e949d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e949d movzx esi,BYTE PTR [rbp-0x78]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // e94a1 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // e94a4 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e94a7 jne 0xe94ae
            if fl.ne() { pc = 0xe94ae; continue; }
            pc = 0xe94a9;
        }
        0xe94a9 => {
            // block e94a9
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94a9 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // e94ac jmp 0xe94b2
            pc = 0xe94b2; continue;
        }
        0xe94ae => {
            // block e94ae
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94ae mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0xe94b2;
        }
        0xe94b2 => {
            // block e94b2
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94b2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e94b5 jne 0xe94fb
            if fl.ne() { pc = 0xe94fb; continue; }
            pc = 0xe94b7;
        }
        0xe94b7 => {
            // block e94b7
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94b7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e94b9 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e94bd cmove rsi,r13
            if fl.e() { rsi = r13; }
            // e94c1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e94c3 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e94c7 lea rax,[rbp-0x57]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa9);
            // e94cb cmove rdi,rax
            if fl.e() { rdi = rax; }
            // e94cf jne 0xe94ed
            if fl.ne() { pc = 0xe94ed; continue; }
            pc = 0xe94d1;
        }
        0xe94d1 => {
            // block e94d1
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94d1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94d4 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94d6;
        }
        0xe94d6 => {
            // block e94d6
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94d6 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // e94d9 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // e94dc cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e94de jne 0xe94fb
            if fl.ne() { pc = 0xe94fb; continue; }
            pc = 0xe94e0;
        }
        0xe94e0 => {
            // block e94e0
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94e0 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e94e3 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // e94e6 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // e94e9 jne 0xe94d6
            if fl.ne() { pc = 0xe94d6; continue; }
            pc = 0xe94eb;
        }
        0xe94eb => {
            // block e94eb
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94eb jmp 0xe9516
            pc = 0xe9516; continue;
        }
        0xe94ed => {
            // block e94ed
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94ed test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94f0 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94f2;
        }
        0xe94f2 => {
            // block e94f2
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94f2 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e94f7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e94f9 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94fb;
        }
        0xe94fb => {
            // block e94fb
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e94fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e94fb add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e94ff mov rdi,r15
            rdi = r15;
            // e9502 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9507 cmp rbx,QWORD PTR [r14+0x270]
            fl = sub::<64>(rbx, m.r64(r14.wrapping_add(0x270)) as u64).1;
            // e950e jne 0xe9475
            if fl.ne() { pc = 0xe9475; continue; }
            pc = 0xe9514;
        }
        0xe9514 => {
            // block e9514
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9514 jmp 0xe952d
            pc = 0xe952d; continue;
        }
        0xe9516 => {
            // block e9516
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9516 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9516 mov QWORD PTR [r14+0x280],rbx
            m.w64(r14.wrapping_add(0x280), (rbx) as u64);
            // e951d lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e9521 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9526 mov rbx,QWORD PTR [r14+0x270]
            rbx = m.r64(r14.wrapping_add(0x270)) as u64;
            pc = 0xe952d;
        }
        0xe952d => {
            // block e952d
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e952d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e952d cmp QWORD PTR [r14+0x280],rbx
            fl = sub::<64>(m.r64(r14.wrapping_add(0x280)) as u64, rbx).1;
            // e9534 setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e9537 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e953b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9540 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e9544 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9549 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e954b add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // e954f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9550 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9552 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9554 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9556 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9558 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9559 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe955a => {
            // block e955a
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e955a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e955a mov rbx,rax
            rbx = rax;
            // e955d lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e9561 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9566 jmp 0xe956b
            pc = 0xe956b; continue;
        }
        0xe9568 => {
            // block e9568
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e9568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9568 mov rbx,rax
            rbx = rax;
            pc = 0xe956b;
        }
        0xe956b => {
            // block e956b
            #[cfg(feature = "trace")] eprintln!("f_000e938c pc e956b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e956b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e956f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9574 mov rdi,rbx
            rdi = rbx;
            // e9577 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea028 size=488
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000ea028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea028;
    loop {
        match pc {
        0xea028 => {
            // block ea028
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea029 mov rbp,rsp
            rbp = rsp;
            // ea02c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ea02e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ea030 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ea032 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ea034 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ea035 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // ea039 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // ea03c mov rbx,rdi
            rbx = rdi;
            // ea03f mov r15d,0xffffffff
            r15 = (0xffffffffu64) as u32 as u64;
            // ea045 cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // ea04a je 0xea1e7
            if fl.e() { pc = 0xea1e7; continue; }
            pc = 0xea050;
        }
        0xea050 => {
            // block ea050
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea050 mov rdi,rbx
            rdi = rbx;
            // ea053 call 0xebf0c
            { let r = { let t = m.env.stub_va(0x6d9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea058 mov rsi,QWORD PTR [rbx+0x28]
            rsi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea05c mov rcx,QWORD PTR [rbx+0x30]
            rcx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea060 mov rdi,QWORD PTR [rbx+0x38]
            rdi = m.r64(rbx.wrapping_add(0x38)) as u64;
            // ea064 cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // ea068 je 0xea097
            if fl.e() { pc = 0xea097; continue; }
            pc = 0xea06a;
        }
        0xea06a => {
            // block ea06a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea06a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea06a test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // ea06d jne 0xea083
            if fl.ne() { pc = 0xea083; continue; }
            pc = 0xea06f;
        }
        0xea06f => {
            // block ea06f
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea06f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea06f lea rax,[rbp-0x28]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // ea073 lea rcx,[rbp-0x29]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // ea077 mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea07b mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // ea07f mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            pc = 0xea083;
        }
        0xea083 => {
            // block ea083
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea083 mov BYTE PTR [rcx],r14b
            m.w8(rcx, ((r14 as u8 as u64)) as u8);
            // ea086 mov rcx,QWORD PTR [rbx+0x30]
            rcx = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea08a inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // ea08d mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea091 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea095 jmp 0xea09a
            pc = 0xea09a; continue;
        }
        0xea097 => {
            // block ea097
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea097 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea097 mov rdx,rsi
            rdx = rsi;
            pc = 0xea09a;
        }
        0xea09a => {
            // block ea09a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea09a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea09a mov r12,rcx
            r12 = rcx;
            // ea09d sub r12,rdx
            { let (r, f) = sub::<64>(r12, rdx);
              r12 = r;
              fl = f; }
            // ea0a0 je 0xea1dc
            if fl.e() { pc = 0xea1dc; continue; }
            pc = 0xea0a6;
        }
        0xea0a6 => {
            // block ea0a6
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0a6 mov QWORD PTR [rbp-0x58],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rdi) as u64);
            // ea0aa mov QWORD PTR [rbp-0x50],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rsi) as u64);
            // ea0ae cmp BYTE PTR [rbx+0x192],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x192)) as u64, 0x0u64).1;
            // ea0b5 je 0xea0d9
            if fl.e() { pc = 0xea0d9; continue; }
            pc = 0xea0b7;
        }
        0xea0b7 => {
            // block ea0b7
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0b7 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea0bb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea0c0 mov rdi,rdx
            rdi = rdx;
            pc = 0xea0c3;
        }
        0xea0c3 => {
            // block ea0c3
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0c3 mov rdx,r12
            rdx = r12;
            // ea0c6 call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea0cb cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea0ce jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea0d4;
        }
        0xea0d4 => {
            // block ea0d4
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0d4 jmp 0xea1c8
            pc = 0xea1c8; continue;
        }
        0xea0d9 => {
            // block ea0d9
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0d9 mov r9,QWORD PTR [rbx+0x40]
            r9 = m.r64(rbx.wrapping_add(0x40)) as u64;
            // ea0dd mov QWORD PTR [rbp-0x38],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r9) as u64);
            // ea0e1 mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // ea0e8 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ea0eb je 0xea19d
            if fl.e() { pc = 0xea19d; continue; }
            pc = 0xea0f1;
        }
        0xea0f1 => {
            // block ea0f1
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0f1 lea rax,[rbx+0x88]
            rax = rbx.wrapping_add(0x88);
            // ea0f8 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // ea0fc jmp 0xea102
            pc = 0xea102; continue;
        }
        0xea0fe => {
            // block ea0fe
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea0fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea0fe mov r9,QWORD PTR [rbx+0x40]
            r9 = m.r64(rbx.wrapping_add(0x40)) as u64;
            pc = 0xea102;
        }
        0xea102 => {
            // block ea102
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea102 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea102 mov rax,QWORD PTR [rbx+0x60]
            rax = m.r64(rbx.wrapping_add(0x60)) as u64;
            // ea106 add rax,r9
            { let (r, f) = add::<64>(rax, r9);
              rax = r;
              fl = f; }
            // ea109 mov r10,QWORD PTR [rdi]
            r10 = m.r64(rdi) as u64;
            // ea10c lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // ea110 mov QWORD PTR [rsp+0x8],rsi
            m.w64(rsp.wrapping_add(0x8), (rsi) as u64);
            // ea115 mov QWORD PTR [rsp],rax
            m.w64(rsp, (rax) as u64);
            // ea119 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // ea11d lea r8,[rbp-0x40]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ea121 call QWORD PTR [r10+0x18]
            { let r = { let t = m.r64(r10.wrapping_add(0x18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea125 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // ea128 mov rdi,QWORD PTR [rbx+0x28]
            rdi = m.r64(rbx.wrapping_add(0x28)) as u64;
            // ea12c cmp QWORD PTR [rbp-0x40],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64, rdi).1;
            // ea130 je 0xea1e7
            if fl.e() { pc = 0xea1e7; continue; }
            pc = 0xea136;
        }
        0xea136 => {
            // block ea136
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea136 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea136 cmp r13d,0x3
            fl = sub::<32>((r13 as u32 as u64), 0x3u64).1;
            // ea13a je 0xea1f9
            if fl.e() { pc = 0xea1f9; continue; }
            pc = 0xea140;
        }
        0xea140 => {
            // block ea140
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea140 cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea144 ja 0xea1e7
            if fl.a() { pc = 0xea1e7; continue; }
            pc = 0xea14a;
        }
        0xea14a => {
            // block ea14a
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea14a mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ea14e mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // ea152 mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea156 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // ea159 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea15e mov rdx,r12
            rdx = r12;
            // ea161 call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea166 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea169 jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea16b;
        }
        0xea16b => {
            // block ea16b
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea16b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea16b cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea16f jne 0xea1c8
            if fl.ne() { pc = 0xea1c8; continue; }
            pc = 0xea171;
        }
        0xea171 => {
            // block ea171
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea171 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea171 mov rdx,QWORD PTR [rbp-0x40]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // ea175 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea179 mov QWORD PTR [rbx+0x28],rdx
            m.w64(rbx.wrapping_add(0x28), (rdx) as u64);
            // ea17d mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            // ea181 sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // ea183 movsxd rcx,eax
            rcx = sx::<32>((rax as u32 as u64)) as u64;
            // ea186 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // ea189 mov QWORD PTR [rbx+0x30],rcx
            m.w64(rbx.wrapping_add(0x30), (rcx) as u64);
            // ea18d mov rdi,QWORD PTR [rbx+0x80]
            rdi = m.r64(rbx.wrapping_add(0x80)) as u64;
            // ea194 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // ea197 jne 0xea0fe
            if fl.ne() { pc = 0xea0fe; continue; }
            pc = 0xea19d;
        }
        0xea19d => {
            // block ea19d
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea19d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea19d mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // ea1a2 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1a7 mov rbx,rax
            rbx = rax;
            // ea1aa mov rdi,rbx
            rdi = rbx;
            // ea1ad call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1b2 mov rsi,QWORD PTR [rip+0x56027] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // ea1b9 mov rdx,QWORD PTR [rip+0x55fe8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // ea1c0 mov rdi,rbx
            rdi = rbx;
            // ea1c3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xea1c8 => {
            // block ea1c8
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1c8 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // ea1cc mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // ea1d0 mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // ea1d4 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // ea1d8 mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            pc = 0xea1dc;
        }
        0xea1dc => {
            // block ea1dc
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1dc xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // ea1df cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // ea1e3 cmovne r15d,r14d
            if fl.ne() { r15 = ((r14 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            pc = 0xea1e7;
        }
        0xea1e7 => {
            // block ea1e7
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1e7 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // ea1ea add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // ea1ee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ea1ef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ea1f1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ea1f3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ea1f5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ea1f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea1f8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xea1f9 => {
            // block ea1f9
            #[cfg(feature = "trace")] eprintln!("f_000ea028 pc ea1f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ea1f9 mov r12,QWORD PTR [rbx+0x30]
            r12 = m.r64(rbx.wrapping_add(0x30)) as u64;
            // ea1fd mov rcx,QWORD PTR [rbx+0x78]
            rcx = m.r64(rbx.wrapping_add(0x78)) as u64;
            // ea201 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // ea204 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // ea209 jmp 0xea0c3
            pc = 0xea0c3; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfdb4 size=483
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000dfdb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdfdb4;
    loop {
        match pc {
        0xdfdb4 => {
            // block dfdb4
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfdb5 mov rbp,rsp
            rbp = rsp;
            // dfdb8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfdba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfdbc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfdbe push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfdc0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfdc1 mov eax,DWORD PTR [rdi]
            rax = (m.r32(rdi) as u64) as u32 as u64;
            // dfdc3 mov DWORD PTR [rdi-0x4],eax
            m.w32(rdi.wrapping_add(0xffff_ffff_ffff_fffc), ((rax as u32 as u64)) as u32);
            // dfdc6 movsxd rbx,r8d
            rbx = sx::<32>((r8 as u32 as u64)) as u64;
            // dfdc9 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // dfdcc movsxd r10,eax
            r10 = sx::<32>((rax as u32 as u64)) as u64;
            // dfdcf lea eax,[rbx-0x1]
            rax = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // dfdd2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // dfdd4 mov eax,DWORD PTR [rdi+rax*4]
            rax = (m.r32(rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // dfdd7 mov DWORD PTR [rdi+r10*4],eax
            m.w32(rdi.wrapping_add(r10.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfddb mov DWORD PTR [rdi+rbx*4],eax
            m.w32(rdi.wrapping_add(rbx.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // dfdde imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfde2 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dfde4 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfde5 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfde7 mov r11d,eax
            r11 = ((rax as u32 as u64)) as u32 as u64;
            // dfdea imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dfdee mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dfdf0 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfdf1 idiv ebx
            { let (q, r) = idiv::<32>(rdx, rax, (rbx as u32 as u64)); rax = q; rdx = r; }
            // dfdf3 mov r10d,eax
            r10 = ((rax as u32 as u64)) as u32 as u64;
            // dfdf6 cmp r11d,r10d
            fl = sub::<32>((r11 as u32 as u64), (r10 as u32 as u64)).1;
            // dfdf9 jge 0xdff8c
            if fl.ge() { pc = 0xdff8c; continue; }
            pc = 0xdfdff;
        }
        0xdfdff => {
            // block dfdff
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfdff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfdff shl r8d,0x10
            { let (r, f) = shl::<32>((r8 as u32 as u64), 0x10u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // dfe03 mov eax,r8d
            rax = ((r8 as u32 as u64)) as u32 as u64;
            // dfe06 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // dfe07 idiv r9d
            { let (q, r) = idiv::<32>(rdx, rax, (r9 as u32 as u64)); rax = q; rdx = r; }
            // dfe0a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // dfe0d sub r10d,r11d
            { let (r, f) = sub::<32>((r10 as u32 as u64), (r11 as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // dfe10 movsxd rcx,r11d
            rcx = sx::<32>((r11 as u32 as u64)) as u64;
            // dfe13 lea r13,[rsi+rcx*4]
            r13 = rsi.wrapping_add(rcx.wrapping_mul(4));
            // dfe17 imul r11d,eax
            { let (r, f) = imul::<32>((r11 as u32 as u64), (rax as u32 as u64)); r11 = (r) as u32 as u64; fl = f; }
            pc = 0xdfe1b;
        }
        0xdfe1b => {
            // block dfe1b
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dfe1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfe1b mov ecx,r11d
            rcx = ((r11 as u32 as u64)) as u32 as u64;
            // dfe1e sar ecx,0x10
            { let (r, f) = sar::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // dfe21 mov r9d,r11d
            r9 = ((r11 as u32 as u64)) as u32 as u64;
            // dfe24 shr r9d,0xb
            { let (r, f) = shr::<32>((r9 as u32 as u64), 0xbu64, fl); r9 = (r) as u32 as u64; fl = f; }
            // dfe28 and r9d,0x1f
            { let r = (r9 as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // dfe2c movsxd rsi,ecx
            rsi = sx::<32>((rcx as u32 as u64)) as u64;
            // dfe2f movzx r8d,BYTE PTR [rdi+rsi*4-0x2]
            r8 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u32 as u64;
            // dfe35 mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // dfe3a sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe3d lea rax,[rip+0x481dc] # 0x128020
            rax = m.env.data_va(247789);
            // dfe44 mov r12d,DWORD PTR [rax+rcx*4]
            r12 = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe48 imul r8d,r12d
            { let (r, f) = imul::<32>((r8 as u32 as u64), (r12 as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            // dfe4c movzx ebx,BYTE PTR [rdi+rsi*4+0x2]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x2)) as u64) as u32 as u64;
            // dfe51 mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // dfe56 sub ecx,r9d
            { let (r, f) = sub::<32>((rcx as u32 as u64), (r9 as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dfe59 mov ecx,DWORD PTR [rax+rcx*4]
            rcx = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe5c imul ebx,ecx
            { let (r, f) = imul::<32>((rbx as u32 as u64), (rcx as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe5f add ebx,r8d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r8 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe62 movzx r14d,BYTE PTR [rdi+rsi*4+0x6]
            r14 = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x6)) as u64) as u32 as u64;
            // dfe68 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // dfe6d sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe70 mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe74 imul r14d,r8d
            { let (r, f) = imul::<32>((r14 as u32 as u64), (r8 as u32 as u64)); r14 = (r) as u32 as u64; fl = f; }
            // dfe78 add r14d,ebx
            { let (r, f) = add::<32>((r14 as u32 as u64), (rbx as u32 as u64));
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfe7b movzx ebx,BYTE PTR [rdi+rsi*4+0xa]
            rbx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xa)) as u64) as u32 as u64;
            // dfe80 mov edx,0x80
            rdx = (0x80u64) as u32 as u64;
            // dfe85 sub edx,r9d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r9 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfe88 mov r9d,DWORD PTR [rax+rdx*4]
            r9 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // dfe8c imul ebx,r9d
            { let (r, f) = imul::<32>((rbx as u32 as u64), (r9 as u32 as u64)); rbx = (r) as u32 as u64; fl = f; }
            // dfe90 add ebx,r14d
            { let (r, f) = add::<32>((rbx as u32 as u64), (r14 as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfe93 movzx edx,BYTE PTR [rdi+rsi*4-0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffd)) as u64) as u32 as u64;
            // dfe98 imul edx,r12d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r12 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfe9c movzx eax,BYTE PTR [rdi+rsi*4+0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x1)) as u64) as u32 as u64;
            // dfea1 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfea4 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfea6 movzx edx,BYTE PTR [rdi+rsi*4+0x5]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x5)) as u64) as u32 as u64;
            // dfeab imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfeaf add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeb1 movzx eax,BYTE PTR [rdi+rsi*4+0x9]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x9)) as u64) as u32 as u64;
            // dfeb6 imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfeba add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfebc mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // dfebf shr r14d,0x8
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x8u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // dfec3 and r14d,0xff00
            { let r = (r14 as u32 as u64) & 0xff00u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // dfeca cmp eax,0xffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffu64).1;
            // dfecf mov eax,0xff00
            rax = (0xff00u64) as u32 as u64;
            // dfed4 cmova r14d,eax
            if fl.a() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            // dfed8 movzx eax,BYTE PTR [rdi+rsi*4-0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // dfedd imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfee1 movzx edx,BYTE PTR [rdi+rsi*4]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // dfee5 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfee8 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dfeea movzx eax,BYTE PTR [rdi+rsi*4+0x4]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x4)) as u64) as u32 as u64;
            // dfeef imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dfef3 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dfef5 movzx edx,BYTE PTR [rdi+rsi*4+0x8]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x8)) as u64) as u32 as u64;
            // dfefa imul edx,r9d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r9 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dfefe add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff00 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // dff02 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff05 cmp edx,0xffffff
            fl = sub::<32>((rdx as u32 as u64), 0xffffffu64).1;
            // dff0b movzx r15d,al
            r15 = ((rax as u8 as u64)) as u32 as u64;
            // dff0f mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // dff14 cmova r15d,eax
            if fl.a() { r15 = ((rax as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // dff18 movzx eax,BYTE PTR [rdi+rsi*4-0x1]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // dff1d imul eax,r12d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r12 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff21 movzx edx,BYTE PTR [rdi+rsi*4+0x3]
            rdx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x3)) as u64) as u32 as u64;
            // dff26 imul edx,ecx
            { let (r, f) = imul::<32>((rdx as u32 as u64), (rcx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // dff29 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // dff2b movzx eax,BYTE PTR [rdi+rsi*4+0x7]
            rax = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x7)) as u64) as u32 as u64;
            // dff30 imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // dff34 add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // dff36 movzx ecx,BYTE PTR [rdi+rsi*4+0xb]
            rcx = (m.r8(rdi.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0xb)) as u64) as u32 as u64;
            // dff3b imul ecx,r9d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r9 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // dff3f add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff41 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // dff43 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // dff46 and eax,0xff000000
            { let r = (rax as u32 as u64) & 0xff000000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dff4b cmp ecx,0xffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffu64).1;
            // dff51 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // dff56 cmova eax,ecx
            if fl.a() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // dff59 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // dff5b and ecx,0xff0000
            { let r = (rcx as u32 as u64) & 0xff0000u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff61 cmp ebx,0xffffff
            fl = sub::<32>((rbx as u32 as u64), 0xffffffu64).1;
            // dff67 mov edx,0xff0000
            rdx = (0xff0000u64) as u32 as u64;
            // dff6c cmova ecx,edx
            if fl.a() { rcx = ((rdx as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // dff6f or ecx,r14d
            { let r = (rcx as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff72 or ecx,r15d
            { let r = (rcx as u32 as u64) | (r15 as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff75 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // dff77 mov DWORD PTR [r13+0x0],ecx
            m.w32(r13, ((rcx as u32 as u64)) as u32);
            // dff7b add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // dff7f add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // dff83 dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // dff86 jne 0xdfe1b
            if fl.ne() { pc = 0xdfe1b; continue; }
            pc = 0xdff8c;
        }
        0xdff8c => {
            // block dff8c
            #[cfg(feature = "trace")] eprintln!("f_000dfdb4 pc dff8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dff8c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dff8d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dff8f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dff91 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dff93 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dff95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dff96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x31368 size=468
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00031368<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31368;
    loop {
        match pc {
        0x31368 => {
            // block 31368
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31368 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31368 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31369 mov rbp,rsp
            rbp = rsp;
            // 3136c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3136d sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31374 mov rbx,rdi
            rbx = rdi;
            // 31377 mov BYTE PTR [rbp-0x9],sil
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fff7), ((rsi as u8 as u64)) as u8);
            // 3137b xorps xmm0,xmm0
            x0 = 0;
            // 3137e movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 31382 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 3138a mov BYTE PTR [rbp-0x31],0x17
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x17u64) as u8);
            // 3138e lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31392 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 31396 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3139b xorps xmm0,xmm0
            x0 = 0;
            // 3139e movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 313a2 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 313aa lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 313ae lea rsi,[rbp-0x9]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff7);
            // 313b2 call 0xebb0a
            { let r = { let t = m.env.stub_va(0x2d7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 313b7 xorps xmm0,xmm0
            x0 = 0;
            // 313ba movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 313be mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 313c6 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 313cd lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 313d1 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 313d6 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 313dd lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 313e1 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 313e6 lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 313ed lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 313f4 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 313f8 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 313fe mov rdi,rbx
            rdi = rbx;
            // 31401 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31406 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31408 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 3140f test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31412 je 0x31429
            if fl.e() { pc = 0x31429; continue; }
            pc = 0x31414;
        }
        0x31414 => {
            // block 31414
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31414 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31414 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 3141b je 0x31424
            if fl.e() { pc = 0x31424; continue; }
            pc = 0x3141d;
        }
        0x3141d => {
            // block 3141d
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3141d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3141d mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x31424;
        }
        0x31424 => {
            // block 31424
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31424 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31429;
        }
        0x31429 => {
            // block 31429
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31429 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 31430 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31433 je 0x31444
            if fl.e() { pc = 0x31444; continue; }
            pc = 0x31435;
        }
        0x31435 => {
            // block 31435
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31435 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31435 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 31439 je 0x3143f
            if fl.e() { pc = 0x3143f; continue; }
            pc = 0x3143b;
        }
        0x3143b => {
            // block 3143b
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3143b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3143b mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x3143f;
        }
        0x3143f => {
            // block 3143f
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3143f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3143f call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31444;
        }
        0x31444 => {
            // block 31444
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31444 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 31448 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3144b je 0x3145c
            if fl.e() { pc = 0x3145c; continue; }
            pc = 0x3144d;
        }
        0x3144d => {
            // block 3144d
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3144d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3144d cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 31451 je 0x31457
            if fl.e() { pc = 0x31457; continue; }
            pc = 0x31453;
        }
        0x31453 => {
            // block 31453
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31453 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x31457;
        }
        0x31457 => {
            // block 31457
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31457 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31457 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3145c;
        }
        0x3145c => {
            // block 3145c
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3145c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3145c mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 31460 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31463 je 0x31474
            if fl.e() { pc = 0x31474; continue; }
            pc = 0x31465;
        }
        0x31465 => {
            // block 31465
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31465 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31465 cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 31469 je 0x3146f
            if fl.e() { pc = 0x3146f; continue; }
            pc = 0x3146b;
        }
        0x3146b => {
            // block 3146b
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3146b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3146b mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x3146f;
        }
        0x3146f => {
            // block 3146f
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3146f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3146f call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31474;
        }
        0x31474 => {
            // block 31474
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31474 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31474 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 31478 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3147b je 0x3148c
            if fl.e() { pc = 0x3148c; continue; }
            pc = 0x3147d;
        }
        0x3147d => {
            // block 3147d
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3147d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3147d cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 31481 je 0x31487
            if fl.e() { pc = 0x31487; continue; }
            pc = 0x31483;
        }
        0x31483 => {
            // block 31483
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31483 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x31487;
        }
        0x31487 => {
            // block 31487
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31487 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31487 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3148c;
        }
        0x3148c => {
            // block 3148c
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3148c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3148c mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 3148e add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31495 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 31496 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31497 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x31498 => {
            // block 31498
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31498 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31498 mov rbx,rax
            rbx = rax;
            // 3149b jmp 0x3151b
            pc = 0x3151b; continue;
        }
        0x3149d => {
            // block 3149d
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3149d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3149d mov rbx,rax
            rbx = rax;
            // 314a0 jmp 0x31503
            pc = 0x31503; continue;
        }
        0x314a2 => {
            // block 314a2
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314a2 mov rbx,rax
            rbx = rax;
            // 314a5 jmp 0x314eb
            pc = 0x314eb; continue;
        }
        0x314a7 => {
            // block 314a7
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314a7 mov rbx,rax
            rbx = rax;
            // 314aa jmp 0x314d0
            pc = 0x314d0; continue;
        }
        0x314ac => {
            // block 314ac
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314ac mov rbx,rax
            rbx = rax;
            // 314af mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 314b6 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 314b9 je 0x314d0
            if fl.e() { pc = 0x314d0; continue; }
            pc = 0x314bb;
        }
        0x314bb => {
            // block 314bb
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314bb cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 314c2 je 0x314cb
            if fl.e() { pc = 0x314cb; continue; }
            pc = 0x314c4;
        }
        0x314c4 => {
            // block 314c4
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314c4 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x314cb;
        }
        0x314cb => {
            // block 314cb
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314cb call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x314d0;
        }
        0x314d0 => {
            // block 314d0
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314d0 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 314d7 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 314da je 0x314eb
            if fl.e() { pc = 0x314eb; continue; }
            pc = 0x314dc;
        }
        0x314dc => {
            // block 314dc
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314dc cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 314e0 je 0x314e6
            if fl.e() { pc = 0x314e6; continue; }
            pc = 0x314e2;
        }
        0x314e2 => {
            // block 314e2
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314e2 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x314e6;
        }
        0x314e6 => {
            // block 314e6
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314e6 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x314eb;
        }
        0x314eb => {
            // block 314eb
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314eb mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 314ef test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 314f2 je 0x31503
            if fl.e() { pc = 0x31503; continue; }
            pc = 0x314f4;
        }
        0x314f4 => {
            // block 314f4
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314f4 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 314f8 je 0x314fe
            if fl.e() { pc = 0x314fe; continue; }
            pc = 0x314fa;
        }
        0x314fa => {
            // block 314fa
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314fa mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x314fe;
        }
        0x314fe => {
            // block 314fe
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 314fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 314fe call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31503;
        }
        0x31503 => {
            // block 31503
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31503 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31503 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 31507 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3150a je 0x3151b
            if fl.e() { pc = 0x3151b; continue; }
            pc = 0x3150c;
        }
        0x3150c => {
            // block 3150c
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3150c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3150c cmp QWORD PTR [rbp-0x48],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64, rdi).1;
            // 31510 je 0x31516
            if fl.e() { pc = 0x31516; continue; }
            pc = 0x31512;
        }
        0x31512 => {
            // block 31512
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31512 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31512 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            pc = 0x31516;
        }
        0x31516 => {
            // block 31516
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31516 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31516 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3151b;
        }
        0x3151b => {
            // block 3151b
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3151b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3151b mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 3151f test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31522 je 0x31533
            if fl.e() { pc = 0x31533; continue; }
            pc = 0x31524;
        }
        0x31524 => {
            // block 31524
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31524 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31524 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 31528 je 0x3152e
            if fl.e() { pc = 0x3152e; continue; }
            pc = 0x3152a;
        }
        0x3152a => {
            // block 3152a
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3152a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3152a mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x3152e;
        }
        0x3152e => {
            // block 3152e
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 3152e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3152e call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31533;
        }
        0x31533 => {
            // block 31533
            #[cfg(feature = "trace")] eprintln!("f_00031368 pc 31533 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31533 mov rdi,rbx
            rdi = rbx;
            // 31536 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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

/// parity: P2v @0x2e5b4 size=442
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e5b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e5b4;
    loop {
        match pc {
        0x2e5b4 => {
            // block 2e5b4
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e5b5 mov rbp,rsp
            rbp = rsp;
            // 2e5b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e5ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e5bc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2e5be push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e5c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e5c1 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e5c5 mov r15,rdx
            r15 = rdx;
            // 2e5c8 mov rbx,rsi
            rbx = rsi;
            // 2e5cb mov r13,rdi
            r13 = rdi;
            // 2e5ce test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e5d1 je 0x2e5dd
            if fl.e() { pc = 0x2e5dd; continue; }
            pc = 0x2e5d3;
        }
        0x2e5d3 => {
            // block 2e5d3
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5d3 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 2e5d8 cmp BYTE PTR [rbx],0x0
            fl = sub::<8>(m.r8(rbx) as u64, 0x0u64).1;
            // 2e5db jne 0x2e5e2
            if fl.ne() { pc = 0x2e5e2; continue; }
            pc = 0x2e5dd;
        }
        0x2e5dd => {
            // block 2e5dd
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5dd mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            pc = 0x2e5e2;
        }
        0x2e5e2 => {
            // block 2e5e2
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5e2 mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 2e5e5 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 2e5e8 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2e5eb je 0x2e632
            if fl.e() { pc = 0x2e632; continue; }
            pc = 0x2e5ed;
        }
        0x2e5ed => {
            // block 2e5ed
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5ed cmp BYTE PTR [r15],0x0
            fl = sub::<8>(m.r8(r15) as u64, 0x0u64).1;
            // 2e5f1 je 0x2e632
            if fl.e() { pc = 0x2e632; continue; }
            pc = 0x2e5f3;
        }
        0x2e5f3 => {
            // block 2e5f3
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5f3 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2e5f6 je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e5fc;
        }
        0x2e5fc => {
            // block 2e5fc
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e5fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e5fc cmp BYTE PTR [rbx],0x0
            fl = sub::<8>(m.r8(rbx) as u64, 0x0u64).1;
            // 2e5ff je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e605;
        }
        0x2e605 => {
            // block 2e605
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e605 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e605 lea rsi,[rip+0xfc574] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e60c mov rdi,rbx
            rdi = rbx;
            // 2e60f call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e614 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e616 jne 0x2e62f
            if fl.ne() { pc = 0x2e62f; continue; }
            pc = 0x2e618;
        }
        0x2e618 => {
            // block 2e618
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e618 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e618 lea rsi,[rip+0xfc561] # 0x12ab80
            rsi = m.env.data_va(258893);
            // 2e61f mov rdi,r15
            rdi = r15;
            // 2e622 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e627 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e629 je 0x2e6c8
            if fl.e() { pc = 0x2e6c8; continue; }
            pc = 0x2e62f;
        }
        0x2e62f => {
            // block 2e62f
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e62f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e62f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2e632;
        }
        0x2e632 => {
            // block 2e632
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e632 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e632 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e636 mov rdi,r13
            rdi = r13;
            // 2e639 mov rsi,rbx
            rsi = rbx;
            // 2e63c call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e642 mov r12,rax
            r12 = rax;
            pc = 0x2e645;
        }
        0x2e645 => {
            // block 2e645
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e645 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e645 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2e648 je 0x2e753
            if fl.e() { pc = 0x2e753; continue; }
            pc = 0x2e64e;
        }
        0x2e64e => {
            // block 2e64e
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e64e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e64e lea rbx,[r13+0x28]
            rbx = r13.wrapping_add(0x28);
            // 2e652 lea rsi,[rip+0x10a66f] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 2e659 mov rdi,rbx
            rdi = rbx;
            // 2e65c call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e661 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 2e664 je 0x2e67f
            if fl.e() { pc = 0x2e67f; continue; }
            pc = 0x2e666;
        }
        0x2e666 => {
            // block 2e666
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e666 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e66a mov rdi,r13
            rdi = r13;
            // 2e66d call QWORD PTR [rax+0x88]
            { let r = { let t = m.r64(rax.wrapping_add(0x88)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e673 mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // 2e67a jmp 0x2e753
            pc = 0x2e753; continue;
        }
        0x2e67f => {
            // block 2e67f
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e67f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e67f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e683 mov rdi,r12
            rdi = r12;
            // 2e686 mov rsi,r15
            rsi = r15;
            // 2e689 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e68c mov QWORD PTR [r13+0x48],rax
            m.w64(r13.wrapping_add(0x48), (rax) as u64);
            // 2e690 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e693 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2e696 je 0x2e745
            if fl.e() { pc = 0x2e745; continue; }
            pc = 0x2e69c;
        }
        0x2e69c => {
            // block 2e69c
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e69c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e69c mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 2e6a0 mov rdi,r12
            rdi = r12;
            // 2e6a3 mov rsi,rax
            rsi = rax;
            // 2e6a6 call QWORD PTR [rcx+0x40]
            { let r = { let t = m.r64(rcx.wrapping_add(0x40)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6a9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e6ab je 0x2e745
            if fl.e() { pc = 0x2e745; continue; }
            pc = 0x2e6b1;
        }
        0x2e6b1 => {
            // block 2e6b1
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6b1 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2e6b5 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // 2e6b7 call 0xec11c
            { let r = { let t = m.env.stub_va(0x8e9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6bc test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // 2e6bf jne 0x2e6f7
            if fl.ne() { pc = 0x2e6f7; continue; }
            pc = 0x2e6c1;
        }
        0x2e6c1 => {
            // block 2e6c1
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6c1 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // 2e6c6 jmp 0x2e706
            pc = 0x2e706; continue;
        }
        0x2e6c8 => {
            // block 2e6c8
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6c8 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2e6d0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2e6d4 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2e6d8 mov rdi,r13
            rdi = r13;
            // 2e6db call QWORD PTR [rax+0x10]
            { let r = { let t = m.r64(rax.wrapping_add(0x10)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e6de test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2e6e0 jle 0x2e753
            if fl.le() { pc = 0x2e753; continue; }
            pc = 0x2e6e2;
        }
        0x2e6e2 => {
            // block 2e6e2
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6e2 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 2e6e6 cmp QWORD PTR [r13+0x18],rax
            fl = sub::<64>(m.r64(r13.wrapping_add(0x18)) as u64, rax).1;
            // 2e6ea je 0x2e753
            if fl.e() { pc = 0x2e753; continue; }
            pc = 0x2e6ec;
        }
        0x2e6ec => {
            // block 2e6ec
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6ec mov r12,QWORD PTR [rax]
            r12 = m.r64(rax) as u64;
            // 2e6ef xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 2e6f2 jmp 0x2e645
            pc = 0x2e645; continue;
        }
        0x2e6f7 => {
            // block 2e6f7
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e6f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e6f7 mov rax,QWORD PTR [r13+0x38]
            rax = m.r64(r13.wrapping_add(0x38)) as u64;
            // 2e6fb mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 2e6fe mov QWORD PTR [r13+0x30],0x0
            m.w64(r13.wrapping_add(0x30), (0x0u64) as u64);
            pc = 0x2e706;
        }
        0x2e706 => {
            // block 2e706
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e706 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2e708 mov rdi,rbx
            rdi = rbx;
            // 2e70b call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e710 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2e714 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 2e718 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2e71c mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2e720 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 2e724 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2e727 xorps xmm0,xmm0
            x0 = 0;
            // 2e72a movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2e72e mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2e736 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2e73a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e73f mov r14d,0x7d0
            r14 = (0x7d0u64) as u32 as u64;
            pc = 0x2e745;
        }
        0x2e745 => {
            // block 2e745
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e745 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e745 mov DWORD PTR [rbp-0x54],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((r14 as u32 as u64)) as u32);
            // 2e749 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2e74d mov rdi,r12
            rdi = r12;
            // 2e750 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e753;
        }
        0x2e753 => {
            // block 2e753
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e753 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e753 mov eax,DWORD PTR [rbp-0x54]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 2e756 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 2e75a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e75b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e75d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2e75f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e761 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e763 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e764 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2e765 => {
            // block 2e765
            #[cfg(feature = "trace")] eprintln!("f_0002e5b4 pc 2e765 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e765 mov rdi,rax
            rdi = rax;
            // 2e768 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f47a size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f47a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f47a;
    loop {
        match pc {
        0x2f47a => {
            // block 2f47a
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f47a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f47a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f47b mov rbp,rsp
            rbp = rsp;
            // 2f47e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f480 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f481 sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f488 mov r14,rsi
            r14 = rsi;
            // 2f48b mov rbx,rdi
            rbx = rdi;
            // 2f48e xorps xmm0,xmm0
            x0 = 0;
            // 2f491 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 2f495 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 2f49d mov BYTE PTR [rbp-0x31],0x35
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x35u64) as u8);
            // 2f4a1 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f4a5 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 2f4a9 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f4ae xorps xmm0,xmm0
            x0 = 0;
            // 2f4b1 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 2f4b5 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 2f4bd movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 2f4c1 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2f4c9 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f4d0 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2f4d4 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f4d9 lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f4e0 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 2f4e4 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f4e9 lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 2f4f0 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 2f4f7 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 2f4fb mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 2f501 mov rdi,rbx
            rdi = rbx;
            // 2f504 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f509 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2f50b mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f512 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f515 je 0x2f52c
            if fl.e() { pc = 0x2f52c; continue; }
            pc = 0x2f517;
        }
        0x2f517 => {
            // block 2f517
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f517 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f517 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f51e je 0x2f527
            if fl.e() { pc = 0x2f527; continue; }
            pc = 0x2f520;
        }
        0x2f520 => {
            // block 2f520
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f520 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f520 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f527;
        }
        0x2f527 => {
            // block 2f527
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f527 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f527 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f52c;
        }
        0x2f52c => {
            // block 2f52c
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f52c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f52c mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f533 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f536 je 0x2f547
            if fl.e() { pc = 0x2f547; continue; }
            pc = 0x2f538;
        }
        0x2f538 => {
            // block 2f538
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f538 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f538 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f53c je 0x2f542
            if fl.e() { pc = 0x2f542; continue; }
            pc = 0x2f53e;
        }
        0x2f53e => {
            // block 2f53e
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f53e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f53e mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f542;
        }
        0x2f542 => {
            // block 2f542
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f542 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f542 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f547;
        }
        0x2f547 => {
            // block 2f547
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f547 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 2f549 je 0x2f55f
            if fl.e() { pc = 0x2f55f; continue; }
            pc = 0x2f54b;
        }
        0x2f54b => {
            // block 2f54b
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f54b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f54b mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f54f mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2f553 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 2f556 je 0x2f563
            if fl.e() { pc = 0x2f563; continue; }
            pc = 0x2f558;
        }
        0x2f558 => {
            // block 2f558
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f558 mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 2f55a mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 2f55d jmp 0x2f56b
            pc = 0x2f56b; continue;
        }
        0x2f55f => {
            // block 2f55f
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f55f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f55f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x2f563;
        }
        0x2f563 => {
            // block 2f563
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f563 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2f566 mov rdi,rax
            rdi = rax;
            // 2f569 je 0x2f57a
            if fl.e() { pc = 0x2f57a; continue; }
            pc = 0x2f56b;
        }
        0x2f56b => {
            // block 2f56b
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f56b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f56b cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f56f je 0x2f575
            if fl.e() { pc = 0x2f575; continue; }
            pc = 0x2f571;
        }
        0x2f571 => {
            // block 2f571
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f571 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f571 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f575;
        }
        0x2f575 => {
            // block 2f575
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f575 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f575 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f57a;
        }
        0x2f57a => {
            // block 2f57a
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f57a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f57a mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f57e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f581 je 0x2f592
            if fl.e() { pc = 0x2f592; continue; }
            pc = 0x2f583;
        }
        0x2f583 => {
            // block 2f583
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f583 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f583 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f587 je 0x2f58d
            if fl.e() { pc = 0x2f58d; continue; }
            pc = 0x2f589;
        }
        0x2f589 => {
            // block 2f589
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f589 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f589 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f58d;
        }
        0x2f58d => {
            // block 2f58d
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f58d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f58d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f592;
        }
        0x2f592 => {
            // block 2f592
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f592 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f592 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2f594 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 2f59b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f59c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f59e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f59f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2f5a0 => {
            // block 2f5a0
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5a0 mov rbx,rax
            rbx = rax;
            // 2f5a3 jmp 0x2f606
            pc = 0x2f606; continue;
        }
        0x2f5a5 => {
            // block 2f5a5
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5a5 mov rbx,rax
            rbx = rax;
            // 2f5a8 jmp 0x2f5ee
            pc = 0x2f5ee; continue;
        }
        0x2f5aa => {
            // block 2f5aa
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5aa mov rbx,rax
            rbx = rax;
            // 2f5ad jmp 0x2f5d3
            pc = 0x2f5d3; continue;
        }
        0x2f5af => {
            // block 2f5af
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5af mov rbx,rax
            rbx = rax;
            // 2f5b2 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 2f5b9 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f5bc je 0x2f5d3
            if fl.e() { pc = 0x2f5d3; continue; }
            pc = 0x2f5be;
        }
        0x2f5be => {
            // block 2f5be
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5be cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 2f5c5 je 0x2f5ce
            if fl.e() { pc = 0x2f5ce; continue; }
            pc = 0x2f5c7;
        }
        0x2f5c7 => {
            // block 2f5c7
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5c7 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x2f5ce;
        }
        0x2f5ce => {
            // block 2f5ce
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5ce call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f5d3;
        }
        0x2f5d3 => {
            // block 2f5d3
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5d3 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 2f5da test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f5dd je 0x2f5ee
            if fl.e() { pc = 0x2f5ee; continue; }
            pc = 0x2f5df;
        }
        0x2f5df => {
            // block 2f5df
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5df cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 2f5e3 je 0x2f5e9
            if fl.e() { pc = 0x2f5e9; continue; }
            pc = 0x2f5e5;
        }
        0x2f5e5 => {
            // block 2f5e5
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5e5 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x2f5e9;
        }
        0x2f5e9 => {
            // block 2f5e9
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5e9 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f5ee;
        }
        0x2f5ee => {
            // block 2f5ee
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5ee mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 2f5f2 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f5f5 je 0x2f606
            if fl.e() { pc = 0x2f606; continue; }
            pc = 0x2f5f7;
        }
        0x2f5f7 => {
            // block 2f5f7
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5f7 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 2f5fb je 0x2f601
            if fl.e() { pc = 0x2f601; continue; }
            pc = 0x2f5fd;
        }
        0x2f5fd => {
            // block 2f5fd
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f5fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f5fd mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x2f601;
        }
        0x2f601 => {
            // block 2f601
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f601 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f601 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f606;
        }
        0x2f606 => {
            // block 2f606
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f606 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 2f60a test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 2f60d je 0x2f61e
            if fl.e() { pc = 0x2f61e; continue; }
            pc = 0x2f60f;
        }
        0x2f60f => {
            // block 2f60f
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f60f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f60f cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 2f613 je 0x2f619
            if fl.e() { pc = 0x2f619; continue; }
            pc = 0x2f615;
        }
        0x2f615 => {
            // block 2f615
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f615 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f615 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x2f619;
        }
        0x2f619 => {
            // block 2f619
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f619 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f619 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2f61e;
        }
        0x2f61e => {
            // block 2f61e
            #[cfg(feature = "trace")] eprintln!("f_0002f47a pc 2f61e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f61e mov rdi,rbx
            rdi = rbx;
            // 2f621 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1044 size=418
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1044<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1044;
    loop {
        match pc {
        0xe1044 => {
            // block e1044
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1044 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1044 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1045 mov rbp,rsp
            rbp = rsp;
            // e1048 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e104a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e104c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e104e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e1050 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1051 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e1052 mov DWORD PTR [rbp-0x30],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rdx as u32 as u64)) as u32);
            // e1055 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e1058 mov r12,rdi
            r12 = rdi;
            // e105b mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e105f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e1061 call QWORD PTR [rax+0x3c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1067 mov r14,rax
            r14 = rax;
            // e106a test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // e106d je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1073;
        }
        0xe1073 => {
            // block e1073
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1073 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1073 mov rbx,r14
            rbx = r14;
            // e1076 shr rbx,0x20
            { let (r, f) = shr::<64>(rbx, 0x20u64, fl); rbx = r; fl = f; }
            // e107a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e107c je 0xe11d6
            if fl.e() { pc = 0xe11d6; continue; }
            pc = 0xe1082;
        }
        0xe1082 => {
            // block e1082
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1082 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1086 mov rdi,r12
            rdi = r12;
            // e1089 call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e108f cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e1092 jne 0xe10b4
            if fl.ne() { pc = 0xe10b4; continue; }
            pc = 0xe1094;
        }
        0xe1094 => {
            // block e1094
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1094 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e1097 cmp ebx,0x2cf
            fl = sub::<32>((rbx as u32 as u64), 0x2cfu64).1;
            // e109d jg 0xe10ea
            if fl.g() { pc = 0xe10ea; continue; }
            pc = 0xe109f;
        }
        0xe109f => {
            // block e109f
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e109f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e109f cmp ebx,0x1e6
            fl = sub::<32>((rbx as u32 as u64), 0x1e6u64).1;
            // e10a5 jne 0xe110b
            if fl.ne() { pc = 0xe110b; continue; }
            pc = 0xe10a7;
        }
        0xe10a7 => {
            // block e10a7
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10a7 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10ac mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e10b2 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10b4 => {
            // block e10b4
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10b4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e10b7 jne 0xe11d6
            if fl.ne() { pc = 0xe11d6; continue; }
            pc = 0xe10bd;
        }
        0xe10bd => {
            // block e10bd
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10bd cmp r14d,0x2d0
            fl = sub::<32>((r14 as u32 as u64), 0x2d0u64).1;
            // e10c4 je 0xe1125
            if fl.e() { pc = 0xe1125; continue; }
            pc = 0xe10c6;
        }
        0xe10c6 => {
            // block e10c6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10c6 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // e10cd je 0xe10e0
            if fl.e() { pc = 0xe10e0; continue; }
            pc = 0xe10cf;
        }
        0xe10cf => {
            // block e10cf
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10cf xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e10d2 cmp r14d,0x780
            fl = sub::<32>((r14 as u32 as u64), 0x780u64).1;
            // e10d9 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10de jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10e0;
        }
        0xe10e0 => {
            // block e10e0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10e0 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // e10e8 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10ea => {
            // block e10ea
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ea cmp ebx,0x2d0
            fl = sub::<32>((rbx as u32 as u64), 0x2d0u64).1;
            // e10f0 je 0xe10ff
            if fl.e() { pc = 0xe10ff; continue; }
            pc = 0xe10f2;
        }
        0xe10f2 => {
            // block e10f2
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10f2 cmp ebx,0x438
            fl = sub::<32>((rbx as u32 as u64), 0x438u64).1;
            // e10f8 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e10fd jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe10ff;
        }
        0xe10ff => {
            // block e10ff
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e10ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e10ff inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e1101 mov eax,0x7
            rax = (0x7u64) as u32 as u64;
            // e1106 mov r13d,ebx
            r13 = ((rbx as u32 as u64)) as u32 as u64;
            // e1109 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe110b => {
            // block e110b
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e110b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e110b cmp ebx,0x240
            fl = sub::<32>((rbx as u32 as u64), 0x240u64).1;
            // e1111 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // e1116 jne 0xe1130
            if fl.ne() { pc = 0xe1130; continue; }
            pc = 0xe1118;
        }
        0xe1118 => {
            // block e1118
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1118 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1118 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e111d mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e1123 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe1125 => {
            // block e1125
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1125 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1125 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e112a mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe1130;
        }
        0xe1130 => {
            // block e1130
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1130 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e1132 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1136 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e113a mov rdi,r12
            rdi = r12;
            // e113d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1143 mov ecx,0xc0000000
            rcx = (0xc0000000u64) as u32 as u64;
            // e1148 and ecx,DWORD PTR [rbp-0x2c]
            { let r = (rcx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e114b mov DWORD PTR [rbp-0x2c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rcx as u32 as u64)) as u32);
            // e114e imul r15d,r13d
            { let (r, f) = imul::<32>((r15 as u32 as u64), (r13 as u32 as u64)); r15 = (r) as u32 as u64; fl = f; }
            // e1152 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e1155 js 0xe1167
            if fl.s() { pc = 0xe1167; continue; }
            pc = 0xe1157;
        }
        0xe1157 => {
            // block e1157
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1157 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1157 add r15d,0x8000
            { let (r, f) = add::<32>((r15 as u32 as u64), 0x8000u64);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e115e sar r15d,0x10
            { let (r, f) = sar::<32>((r15 as u32 as u64), 0x10u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e1162 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // e1165 jmp 0xe1174
            pc = 0xe1174; continue;
        }
        0xe1167 => {
            // block e1167
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1167 mov esi,0x8000
            rsi = (0x8000u64) as u32 as u64;
            // e116c sub esi,r15d
            { let (r, f) = sub::<32>((rsi as u32 as u64), (r15 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e116f sar esi,0x10
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e1172 neg esi
            { let (r, f) = neg::<32>((rsi as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            pc = 0xe1174;
        }
        0xe1174 => {
            // block e1174
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1174 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1174 mov edi,DWORD PTR [rbp-0x30]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e1177 dec r13d
            { let (r, f) = dec::<32>((r13 as u32 as u64), fl); r13 = (r) as u32 as u64; fl = f; }
            // e117a imul r13d,edi
            { let (r, f) = imul::<32>((r13 as u32 as u64), (rdi as u32 as u64)); r13 = (r) as u32 as u64; fl = f; }
            // e117e cmp r13d,0xfffffffe
            fl = sub::<32>((r13 as u32 as u64), 0xfffffffeu64).1;
            // e1182 jg 0xe1193
            if fl.g() { pc = 0xe1193; continue; }
            pc = 0xe1184;
        }
        0xe1184 => {
            // block e1184
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1184 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1184 mov edx,0x7fff
            rdx = (0x7fffu64) as u32 as u64;
            // e1189 sub edx,r13d
            { let (r, f) = sub::<32>((rdx as u32 as u64), (r13 as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e118c sar edx,0x10
            { let (r, f) = sar::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e118f neg edx
            { let (r, f) = neg::<32>((rdx as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e1191 jmp 0xe11a1
            pc = 0xe11a1; continue;
        }
        0xe1193 => {
            // block e1193
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e1193 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1193 add r13d,0x8001
            { let (r, f) = add::<32>((r13 as u32 as u64), 0x8001u64);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e119a sar r13d,0x10
            { let (r, f) = sar::<32>((r13 as u32 as u64), 0x10u64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e119e mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            pc = 0xe11a1;
        }
        0xe11a1 => {
            // block e11a1
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11a1 mov eax,0x1fff0000
            rax = (0x1fff0000u64) as u32 as u64;
            // e11a6 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // e11a8 je 0xe11c0
            if fl.e() { pc = 0xe11c0; continue; }
            pc = 0xe11aa;
        }
        0xe11aa => {
            // block e11aa
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11aa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e11ac mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e11b1 cmovne edi,edx
            if fl.ne() { rdi = ((rdx as u32 as u64)) as u32 as u64; } else { rdi = rdi as u32 as u64; }
            // e11b4 mov eax,0x1fff
            rax = (0x1fffu64) as u32 as u64;
            // e11b9 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e11bb div edi
            { let (q, r) = div::<32>(rdx, rax, (rdi as u32 as u64)); rax = q; rdx = r; }
            // e11bd shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xe11c0;
        }
        0xe11c0 => {
            // block e11c0
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11c0 mov rdx,QWORD PTR [r12]
            rdx = m.r64(r12) as u64;
            // e11c4 add esi,ebx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rbx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11c6 shl esi,0x2
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x2u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e11c9 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cb or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e11cd mov rdi,r12
            rdi = r12;
            // e11d0 call QWORD PTR [rdx+0x1ad0]
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe11d6;
        }
        0xe11d6 => {
            // block e11d6
            #[cfg(feature = "trace")] eprintln!("f_000e1044 pc e11d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e11d6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e11da pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e11db pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e11dd pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e11df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e11e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e11e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e11e4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17332 size=414
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017332<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17332;
    loop {
        match pc {
        0x17332 => {
            // block 17332
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17332 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17333 mov rbp,rsp
            rbp = rsp;
            // 17336 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17338 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1733a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1733c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1733e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1733f sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 17343 mov r14,rsi
            r14 = rsi;
            // 17346 mov rbx,rdi
            rbx = rdi;
            // 17349 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 1734d mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 17352 call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17357 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1735b mov rsi,rbx
            rsi = rbx;
            // 1735e call 0x174d0
            { let r = { let t = m.env.code_va(0x174d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17363 mov r12,QWORD PTR [rbp-0x40]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 17367 cmp r12,QWORD PTR [rbp-0x38]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 1736b je 0x1743e
            if fl.e() { pc = 0x1743e; continue; }
            pc = 0x17371;
        }
        0x17371 => {
            // block 17371
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17371 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17371 mov QWORD PTR [rbp-0x80],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (r14) as u64);
            // 17375 lea r13,[rbp-0x5f]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa1);
            // 17379 lea r15,[rbp-0x78]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            pc = 0x1737d;
        }
        0x1737d => {
            // block 1737d
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 1737d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1737d lea rsi,[r12+0x18]
            rsi = r12.wrapping_add(0x18);
            // 17382 mov rdi,r15
            rdi = r15;
            // 17385 call 0x174d0
            { let r = { let t = m.env.code_va(0x174d0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1738a movzx r14d,BYTE PTR [rbp-0x78]
            r14 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // 1738f test r14b,0x1
            fl = logic::<8>((r14 as u8 as u64) & 0x1u64);
            // 17393 jne 0x1739e
            if fl.ne() { pc = 0x1739e; continue; }
            pc = 0x17395;
        }
        0x17395 => {
            // block 17395
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17395 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17395 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 17398 lea r8,[rbp-0x77]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // 1739c jmp 0x173a6
            pc = 0x173a6; continue;
        }
        0x1739e => {
            // block 1739e
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 1739e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1739e mov r14,QWORD PTR [rbp-0x70]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 173a2 mov r8,QWORD PTR [rbp-0x68]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x173a6;
        }
        0x173a6 => {
            // block 173a6
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173a6 movzx edx,BYTE PTR [rbp-0x60]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 173aa test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 173ad jne 0x173b7
            if fl.ne() { pc = 0x173b7; continue; }
            pc = 0x173af;
        }
        0x173af => {
            // block 173af
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173af shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 173b2 mov rsi,r13
            rsi = r13;
            // 173b5 jmp 0x173bf
            pc = 0x173bf; continue;
        }
        0x173b7 => {
            // block 173b7
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173b7 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 173bb mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x173bf;
        }
        0x173bf => {
            // block 173bf
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173bf cmp r14,rdx
            fl = sub::<64>(r14, rdx).1;
            // 173c2 jb 0x17425
            if fl.b() { pc = 0x17425; continue; }
            pc = 0x173c4;
        }
        0x173c4 => {
            // block 173c4
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173c4 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 173c7 je 0x17442
            if fl.e() { pc = 0x17442; continue; }
            pc = 0x173c9;
        }
        0x173c9 => {
            // block 173c9
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173c9 cmp r14,rdx
            fl = sub::<64>(r14, rdx).1;
            // 173cc jl 0x17425
            if fl.l() { pc = 0x17425; continue; }
            pc = 0x173ce;
        }
        0x173ce => {
            // block 173ce
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173ce mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 173d4 sub r10,rdx
            { let (r, f) = sub::<64>(r10, rdx);
              r10 = r;
              fl = f; }
            // 173d7 add r10,r14
            { let (r, f) = add::<64>(r10, r14);
              r10 = r;
              fl = f; }
            // 173da je 0x17425
            if fl.e() { pc = 0x17425; continue; }
            pc = 0x173dc;
        }
        0x173dc => {
            // block 173dc
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173dc add r14,r8
            { let (r, f) = add::<64>(r14, r8);
              r14 = r;
              fl = f; }
            // 173df add r10,r8
            { let (r, f) = add::<64>(r10, r8);
              r10 = r;
              fl = f; }
            // 173e2 mov r9b,BYTE PTR [rsi]
            r9 = (r9 & !0xff) | ((m.r8(rsi) as u64) as u8 as u64);
            // 173e5 mov rcx,r8
            rcx = r8;
            pc = 0x173e8;
        }
        0x173e8 => {
            // block 173e8
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173e8 movzx r11d,r9b
            r11 = ((r9 as u8 as u64)) as u32 as u64;
            // 173ec movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // 173ef cmp edi,r11d
            fl = sub::<32>((rdi as u32 as u64), (r11 as u32 as u64)).1;
            // 173f2 mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // 173f7 jne 0x1740d
            if fl.ne() { pc = 0x1740d; continue; }
            pc = 0x173f9;
        }
        0x173f9 => {
            // block 173f9
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173f9 cmp rdx,rdi
            fl = sub::<64>(rdx, rdi).1;
            // 173fc je 0x17417
            if fl.e() { pc = 0x17417; continue; }
            pc = 0x173fe;
        }
        0x173fe => {
            // block 173fe
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 173fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 173fe movzx eax,BYTE PTR [rsi+rdi*1]
            rax = (m.r8(rsi.wrapping_add(rdi)) as u64) as u32 as u64;
            // 17402 movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // 17406 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 17409 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // 1740b je 0x173f9
            if fl.e() { pc = 0x173f9; continue; }
            pc = 0x1740d;
        }
        0x1740d => {
            // block 1740d
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 1740d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1740d inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 17410 cmp rcx,r10
            fl = sub::<64>(rcx, r10).1;
            // 17413 jne 0x173e8
            if fl.ne() { pc = 0x173e8; continue; }
            pc = 0x17415;
        }
        0x17415 => {
            // block 17415
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17415 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17415 jmp 0x17425
            pc = 0x17425; continue;
        }
        0x17417 => {
            // block 17417
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17417 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17417 cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 1741a je 0x17425
            if fl.e() { pc = 0x17425; continue; }
            pc = 0x1741c;
        }
        0x1741c => {
            // block 1741c
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 1741c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1741c sub rcx,r8
            { let (r, f) = sub::<64>(rcx, r8);
              rcx = r;
              fl = f; }
            // 1741f cmp rcx,0xffffffffffffffff
            fl = sub::<64>(rcx, 0xffff_ffff_ffff_ffffu64).1;
            // 17423 jne 0x17442
            if fl.ne() { pc = 0x17442; continue; }
            pc = 0x17425;
        }
        0x17425 => {
            // block 17425
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17425 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17425 mov rdi,r15
            rdi = r15;
            // 17428 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1742d add r12,0xf8
            { let (r, f) = add::<64>(r12, 0xf8u64);
              r12 = r;
              fl = f; }
            // 17434 cmp r12,QWORD PTR [rbp-0x38]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 17438 jne 0x1737d
            if fl.ne() { pc = 0x1737d; continue; }
            pc = 0x1743e;
        }
        0x1743e => {
            // block 1743e
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 1743e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1743e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 17440 jmp 0x17467
            pc = 0x17467; continue;
        }
        0x17442 => {
            // block 17442
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17442 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 17446 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 17449 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 1744d movzx esi,WORD PTR [r12+0x4]
            rsi = (m.r16(r12.wrapping_add(0x4)) as u64) as u32 as u64;
            // 17453 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17455 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 17457 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1745a call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1745c mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1745e lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 17462 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17467;
        }
        0x17467 => {
            // block 17467
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17467 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1746b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17470 lea rax,[rip+0x129fa1] # 0x141418
            rax = m.env.data_va(351205);
            // 17477 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1747b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 1747f call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17484 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17486 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1748a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1748b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1748d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1748f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17491 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17493 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17494 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x17495 => {
            // block 17495
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17495 mov rbx,rax
            rbx = rax;
            pc = 0x17498;
        }
        0x17498 => {
            // block 17498
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 17498 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17498 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1749c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x174a1;
        }
        0x174a1 => {
            // block 174a1
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 174a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 174a1 lea rax,[rip+0x129f70] # 0x141418
            rax = m.env.data_va(351205);
            // 174a8 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 174ac lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 174b0 call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 174b5 mov rdi,rbx
            rdi = rbx;
            // 174b8 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 174bd mov rbx,rax
            rbx = rax;
            // 174c0 jmp 0x174a1
            pc = 0x174a1; continue;
        }
        0x174c2 => {
            // block 174c2
            #[cfg(feature = "trace")] eprintln!("f_00017332 pc 174c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 174c2 mov rbx,rax
            rbx = rax;
            // 174c5 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 174c9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 174ce jmp 0x17498
            pc = 0x17498; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8100 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000e8100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe8100;
    loop {
        match pc {
        0xe8100 => {
            // block e8100
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e8101 mov rbp,rsp
            rbp = rsp;
            // e8104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e8106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8108 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e810a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e810c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e810d sub rsp,0x498
            { let (r, f) = sub::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8114 mov r14,rsi
            r14 = rsi;
            // e8117 mov rbx,rdi
            rbx = rdi;
            // e811a mov rax,QWORD PTR [rip+0x582b7] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8121 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8124 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e8128 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e812b call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e812e lea r15,[rbx+0x290]
            r15 = rbx.wrapping_add(0x290);
            // e8135 test BYTE PTR [rbx+0x290],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x290)) as u64 & 0x1u64);
            // e813c jne 0xe8146
            if fl.ne() { pc = 0xe8146; continue; }
            pc = 0xe813e;
        }
        0xe813e => {
            // block e813e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e813e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e813e mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // e8144 jmp 0xe815b
            pc = 0xe815b; continue;
        }
        0xe8146 => {
            // block e8146
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8146 mov rax,QWORD PTR [rbx+0x2a0]
            rax = m.r64(rbx.wrapping_add(0x2a0)) as u64;
            // e814d mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e8150 mov QWORD PTR [rbx+0x298],0x0
            m.w64(rbx.wrapping_add(0x298), (0x0u64) as u64);
            pc = 0xe815b;
        }
        0xe815b => {
            // block e815b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e815b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e815b test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e815f jne 0xe8167
            if fl.ne() { pc = 0xe8167; continue; }
            pc = 0xe8161;
        }
        0xe8161 => {
            // block e8161
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8161 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8161 lea rdi,[r14+0x1]
            rdi = r14.wrapping_add(0x1);
            // e8165 jmp 0xe816b
            pc = 0xe816b; continue;
        }
        0xe8167 => {
            // block e8167
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8167 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8167 mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe816b;
        }
        0xe816b => {
            // block e816b
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e816b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e816b lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // e8172 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8177 mov eax,DWORD PTR [rbp-0x450]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64) as u32 as u64;
            // e817d mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            // e8183 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // e818a call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e818f mov r12,rax
            r12 = rax;
            // e8192 lea r13,[rbp-0x4b8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // e8199 mov rdi,r13
            rdi = r13;
            // e819c call 0xec2ea
            { let r = { let t = m.env.stub_va(0xab7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a1 mov rdi,r13
            rdi = r13;
            // e81a4 call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a9 mov rdi,rax
            rdi = rax;
            // e81ac call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81b1 mov r13,rax
            r13 = rax;
            // e81b4 mov rdi,r12
            rdi = r12;
            // e81b7 call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81bc mov rcx,rax
            rcx = rax;
            // e81bf lea rsi,[rip+0x55048] # 0x13d20e
            rsi = m.env.data_va(334299);
            // e81c6 lea r12,[rbp-0x420]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fbe0);
            // e81cd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e81cf mov rdi,r12
            rdi = r12;
            // e81d2 mov rdx,r13
            rdx = r13;
            // e81d5 call 0xec296
            { let r = { let t = m.env.stub_va(0xa63); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81da mov rdi,r15
            rdi = r15;
            // e81dd mov rsi,r12
            rsi = r12;
            // e81e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81e5 lea r15,[rbx+0x8]
            r15 = rbx.wrapping_add(0x8);
            // e81e9 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e81ed jne 0xe81f4
            if fl.ne() { pc = 0xe81f4; continue; }
            pc = 0xe81ef;
        }
        0xe81ef => {
            // block e81ef
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e81f2 jmp 0xe81f8
            pc = 0xe81f8; continue;
        }
        0xe81f4 => {
            // block e81f4
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f4 mov r14,QWORD PTR [r14+0x10]
            r14 = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe81f8;
        }
        0xe81f8 => {
            // block e81f8
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e81f8 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e81fd mov rdi,r15
            rdi = r15;
            // e8200 mov rsi,r14
            rsi = r14;
            // e8203 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8208 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e820b mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e820f test BYTE PTR [rax+r15*1+0x20],0x5
            fl = logic::<8>(m.r8(rax.wrapping_add(r15).wrapping_add(0x20)) as u64 & 0x5u64);
            // e8215 je 0xe821c
            if fl.e() { pc = 0xe821c; continue; }
            pc = 0xe8217;
        }
        0xe8217 => {
            // block e8217
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8217 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e821a jmp 0xe8269
            pc = 0xe8269; continue;
        }
        0xe821c => {
            // block e821c
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e821c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e821c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e821f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8221 mov rdi,rbx
            rdi = rbx;
            // e8224 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8227 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e822a mov rdi,rbx
            rdi = rbx;
            // e822d call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8233 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // e8236 cmp QWORD PTR [rbx+0x90],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e823e je 0xe8269
            if fl.e() { pc = 0xe8269; continue; }
            pc = 0xe8240;
        }
        0xe8240 => {
            // block e8240
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8240 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8240 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e8244 mov rdi,rbx
            rdi = rbx;
            // e8247 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e824c test rax,rax
            fl = logic::<64>(rax & rax);
            // e824f jne 0xe8269
            if fl.ne() { pc = 0xe8269; continue; }
            pc = 0xe8251;
        }
        0xe8251 => {
            // block e8251
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8251 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8251 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e8254 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e8258 lea rdi,[r15+rax*1]
            rdi = r15.wrapping_add(rax);
            // e825c mov esi,DWORD PTR [r15+rax*1+0x20]
            rsi = (m.r32(r15.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e8261 or esi,0x4
            { let r = (rsi as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8264 call 0xec0fe
            { let r = { let t = m.env.stub_va(0x8cb); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8269;
        }
        0xe8269 => {
            // block e8269
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8269 mov rax,QWORD PTR [rip+0x58168] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e8270 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e8273 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e8277 jne 0xe828e
            if fl.ne() { pc = 0xe828e; continue; }
            pc = 0xe8279;
        }
        0xe8279 => {
            // block e8279
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e8279 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8279 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // e827c add rsp,0x498
            { let (r, f) = add::<64>(rsp, 0x498u64);
              rsp = r;
              fl = f; }
            // e8283 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8284 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e8286 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e8288 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e828a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e828c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e828d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe828e => {
            // block e828e
            #[cfg(feature = "trace")] eprintln!("f_000e8100 pc e828e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e828e call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xefd0 size=398
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000efd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xefd0;
    loop {
        match pc {
        0xefd0 => {
            // block efd0
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc efd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // efd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // efd1 mov rbp,rsp
            rbp = rsp;
            // efd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // efd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // efd8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // efda push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // efdb sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // efe2 mov r15,rdi
            r15 = rdi;
            // efe5 lea r14,[rbp-0xa8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // efec mov rdi,r14
            rdi = r14;
            // efef call 0x2bf98
            { let r = { let t = m.env.code_va(0x2bf98); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eff4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eff7 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // effe mov rdi,r15
            rdi = r15;
            // f001 mov rsi,r14
            rsi = r14;
            // f004 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f006 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f008 je 0xf012
            if fl.e() { pc = 0xf012; continue; }
            pc = 0xf00a;
        }
        0xf00a => {
            // block f00a
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f00a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f00a mov eax,DWORD PTR [rbp-0x88]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // f010 jmp 0xf070
            pc = 0xf070; continue;
        }
        0xf012 => {
            // block f012
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f012 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // f015 cmp DWORD PTR [rbp-0x88],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64, 0x0u64).1;
            // f01c je 0xf106
            if fl.e() { pc = 0xf106; continue; }
            pc = 0xf022;
        }
        0xf022 => {
            // block f022
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f022 mov rbx,QWORD PTR [rbp-0x84]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64;
            // f029 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // f02d add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // f031 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xf034;
        }
        0xf034 => {
            // block f034
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f034 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // f037 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // f03b mov esi,DWORD PTR [rbx-0xc]
            rsi = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // f03e mov edx,DWORD PTR [rbx-0x8]
            rdx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64) as u32 as u64;
            // f041 mov ecx,DWORD PTR [rbx-0x4]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // f044 mov r8d,DWORD PTR [rbx]
            r8 = (m.r32(rbx) as u64) as u32 as u64;
            // f047 mov rdi,r15
            rdi = r15;
            // f04a call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f04c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // f04e jne 0xf05e
            if fl.ne() { pc = 0xf05e; continue; }
            pc = 0xf050;
        }
        0xf050 => {
            // block f050
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f050 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f050 mov eax,DWORD PTR [rbp-0x5c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // f053 lea ecx,[rax+0x1]
            rcx = (rax.wrapping_add(0x1)) as u32 as u64;
            // f056 mov DWORD PTR [rbp-0x5c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rcx as u32 as u64)) as u32);
            // f059 mov WORD PTR [r14+rax*2],r12w
            m.w16(r14.wrapping_add(rax.wrapping_mul(2)), ((r12 as u16 as u64)) as u16);
            pc = 0xf05e;
        }
        0xf05e => {
            // block f05e
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f05e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f05e inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // f061 mov eax,DWORD PTR [rbp-0x88]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // f067 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // f06b cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // f06e jb 0xf034
            if fl.b() { pc = 0xf034; continue; }
            pc = 0xf070;
        }
        0xf070 => {
            // block f070
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f070 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // f073 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // f075 je 0xf106
            if fl.e() { pc = 0xf106; continue; }
            pc = 0xf07b;
        }
        0xf07b => {
            // block f07b
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f07b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f07b cmp DWORD PTR [rbp-0x5c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64, 0x0u64).1;
            // f07f je 0xf106
            if fl.e() { pc = 0xf106; continue; }
            pc = 0xf085;
        }
        0xf085 => {
            // block f085
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f085 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f085 mov rdi,QWORD PTR [rip+0x1310f4] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // f08c lea rsi,[rip+0x11af22] # 0x129fb5
            rsi = m.env.data_va(255874);
            // f093 mov edx,0x3e
            rdx = (0x3eu64) as u32 as u64;
            // f098 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f09d lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // f0a4 mov rsi,rax
            rsi = rax;
            // f0a7 call 0x2bfcc
            { let r = { let t = m.env.code_va(0x2bfcc); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0ac mov rbx,rax
            rbx = rax;
            // f0af mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f0b2 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // f0b6 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // f0b9 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // f0bd call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0c2 mov rsi,QWORD PTR [rip+0x1310c7] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // f0c9 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // f0cd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0d2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // f0d5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // f0d9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // f0de mov rdi,rax
            rdi = rax;
            // f0e1 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0e3 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f0e6 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // f0ea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0ef movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // f0f3 mov rdi,rbx
            rdi = rbx;
            // f0f6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f0fb mov rdi,rbx
            rdi = rbx;
            // f0fe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f103 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0xf106;
        }
        0xf106 => {
            // block f106
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f106 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f106 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // f10a call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f10f lea rdi,[rbp-0x84]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff7c);
            // f116 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f11b mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // f11e add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // f125 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f126 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // f128 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f12a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // f12c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f12d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf12e => {
            // block f12e
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f12e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f12e jmp 0xf130
            pc = 0xf130; continue;
        }
        0xf130 => {
            // block f130
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f130 mov rbx,rax
            rbx = rax;
            pc = 0xf133;
        }
        0xf133 => {
            // block f133
            #[cfg(feature = "trace")] eprintln!("f_0000efd0 pc f133 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f133 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // f137 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f13c lea rdi,[rbp-0x84]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff7c);
            // f143 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f148 mov rdi,rbx
            rdi = rbx;
            // f14b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f150 mov rbx,rax
            rbx = rax;
            // f153 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // f157 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f15c jmp 0xf133
            pc = 0xf133; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9208 size=388
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9208<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9208;
    loop {
        match pc {
        0xe9208 => {
            // block e9208
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9208 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9209 mov rbp,rsp
            rbp = rsp;
            // e920c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e920e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9210 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9212 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9214 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9215 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9219 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // e921c mov r14,rdi
            r14 = rdi;
            // e921f xorps xmm0,xmm0
            x0 = 0;
            // e9222 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // e9226 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // e922e xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e9230 cmp QWORD PTR [r14+0x90],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x90)) as u64, 0x0u64).1;
            // e9238 je 0xe9353
            if fl.e() { pc = 0xe9353; continue; }
            pc = 0xe923e;
        }
        0xe923e => {
            // block e923e
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e923e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e923e lea r13,[r14+0x8]
            r13 = r14.wrapping_add(0x8);
            // e9242 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e9244 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e9246 mov rdi,r13
            rdi = r13;
            // e9249 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e924e lea rdi,[r14+0x268]
            rdi = r14.wrapping_add(0x268);
            // e9255 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // e9258 je 0xe9269
            if fl.e() { pc = 0xe9269; continue; }
            pc = 0xe925a;
        }
        0xe925a => {
            // block e925a
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e925a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e925a lea esi,[r15+0x1]
            rsi = (r15.wrapping_add(0x1)) as u32 as u64;
            // e925e mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9262 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9267 jmp 0xe927d
            pc = 0xe927d; continue;
        }
        0xe9269 => {
            // block e9269
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9269 mov r15d,0x1e8480
            r15 = (0x1e8480u64) as u32 as u64;
            // e926f mov esi,0x1e8480
            rsi = (0x1e8480u64) as u32 as u64;
            // e9274 mov QWORD PTR [rbp-0x60],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rdi) as u64);
            // e9278 call 0xebf18
            { let r = { let t = m.env.stub_va(0x6e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe927d;
        }
        0xe927d => {
            // block e927d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e927d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e927d xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // e927f call 0xec0ec
            { let r = { let t = m.env.stub_va(0x8b9); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9284 mov QWORD PTR [rbp-0x68],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r14) as u64);
            // e9288 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // e928b mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e928f mov QWORD PTR [rbp-0x70],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r15) as u64);
            // e9293 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e9296 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e929a xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // e929d jmp 0xe92a6
            pc = 0xe92a6; continue;
        }
        0xe929f => {
            // block e929f
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e929f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e929f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e92a2 add r14,0x18
            { let (r, f) = add::<64>(r14, 0x18u64);
              r14 = r;
              fl = f; }
            pc = 0xe92a6;
        }
        0xe92a6 => {
            // block e92a6
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92a6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // e92aa mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e92ae add rsi,r13
            { let (r, f) = add::<64>(rsi, r13);
              rsi = r;
              fl = f; }
            // e92b1 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92b5 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ba lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92be mov rsi,QWORD PTR [rip+0x56ecb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // e92c5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92cd mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // e92d1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // e92d6 mov rdi,rax
            rdi = rax;
            // e92d9 call rcx
            { let r = { let t = rcx; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92db mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e92de lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92e7 movsx edx,r12b
            rdx = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // e92eb mov rdi,r13
            rdi = r13;
            // e92ee mov rsi,rbx
            rsi = rbx;
            // e92f1 call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92f6 cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e92fa jae 0xe9328
            if fl.ae() { pc = 0xe9328; continue; }
            pc = 0xe92fc;
        }
        0xe92fc => {
            // block e92fc
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e92fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e92fc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92ff mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // e9303 mov eax,DWORD PTR [rcx+rax*1+0x20]
            rax = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x20)) as u64) as u32 as u64;
            // e9307 and eax,0x5
            { let r = (rax as u32 as u64) & 0x5u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e930a jne 0xe9328
            if fl.ne() { pc = 0xe9328; continue; }
            pc = 0xe930c;
        }
        0xe930c => {
            // block e930c
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e930c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e930c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e9310 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // e9313 add rdi,r14
            { let (r, f) = add::<64>(rdi, r14);
              rdi = r;
              fl = f; }
            // e9316 mov rsi,rbx
            rsi = rbx;
            // e9319 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e931e jmp 0xe929f
            pc = 0xe929f; continue;
        }
        0xe9323 => {
            // block e9323
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9323 mov rbx,rax
            rbx = rax;
            // e9326 jmp 0xe9379
            pc = 0xe9379; continue;
        }
        0xe9328 => {
            // block e9328
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9328 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // e932c cmp eax,0x1e847f
            fl = sub::<32>((rax as u32 as u64), 0x1e847fu64).1;
            // e9331 mov rbx,QWORD PTR [rbp-0x68]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e9335 ja 0xe934d
            if fl.a() { pc = 0xe934d; continue; }
            pc = 0xe9337;
        }
        0xe9337 => {
            // block e9337
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9337 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e933b add r14,QWORD PTR [rax]
            { let (r, f) = add::<64>(r14, m.r64(rax) as u64);
              r14 = r;
              fl = f; }
            // e933e lea rsi,[rip+0x4fb72] # 0x138eb7
            rsi = m.env.data_va(317060);
            // e9345 mov rdi,r14
            rdi = r14;
            // e9348 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe934d;
        }
        0xe934d => {
            // block e934d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e934d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e934d mov ebx,DWORD PTR [rbx+0x260]
            rbx = (m.r32(rbx.wrapping_add(0x260)) as u64) as u32 as u64;
            pc = 0xe9353;
        }
        0xe9353 => {
            // block e9353
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9353 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9353 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e9357 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e935c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e935e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e9362 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9363 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9365 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9367 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9369 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e936b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e936c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe936d => {
            // block e936d
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e936d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e936d mov rbx,rax
            rbx = rax;
            // e9370 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e9374 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9379;
        }
        0xe9379 => {
            // block e9379
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9379 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e937d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9382 mov rdi,rbx
            rdi = rbx;
            // e9385 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e938a jmp 0xe9323
            pc = 0xe9323; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc51ba size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c51ba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc51ba;
    loop {
        match pc {
        0xc51ba => {
            // block c51ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51ba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c51bb mov rbp,rsp
            rbp = rsp;
            // c51be push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c51c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c51c2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c51c4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c51c6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c51c7 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c51cb mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c51cf test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c51d2 je 0xc530e
            if fl.e() { pc = 0xc530e; continue; }
            pc = 0xc51d8;
        }
        0xc51d8 => {
            // block c51d8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c51d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c51d8 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // c51dc movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // c51df mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c51e2 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c51e6 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c51e9 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // c51ed mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // c51f1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // c51f5 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // c51f9 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // c51fd lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // c5201 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // c5205 jmp 0xc520a
            pc = 0xc520a; continue;
        }
        0xc5207 => {
            // block c5207
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5207 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5207 mov rbx,rax
            rbx = rax;
            pc = 0xc520a;
        }
        0xc520a => {
            // block c520a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c520a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c520a test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c520d mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c5211 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c5216 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // c521a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // c521d and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // c5221 jne 0xc522b
            if fl.ne() { pc = 0xc522b; continue; }
            pc = 0xc5223;
        }
        0xc5223 => {
            // block c5223
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5223 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5223 mov r14,rax
            r14 = rax;
            // c5226 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // c5229 jmp 0xc522f
            pc = 0xc522f; continue;
        }
        0xc522b => {
            // block c522b
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522b mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xc522f;
        }
        0xc522f => {
            // block c522f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c522f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c522f lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // c5233 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c5236 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c523a cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c523f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c5242 jne 0xc524e
            if fl.ne() { pc = 0xc524e; continue; }
            pc = 0xc5244;
        }
        0xc5244 => {
            // block c5244
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5244 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // c5248 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c524c jmp 0xc5256
            pc = 0xc5256; continue;
        }
        0xc524e => {
            // block c524e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c524e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c524e mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // c5252 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xc5256;
        }
        0xc5256 => {
            // block c5256
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5256 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5256 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // c5259 mov rdx,r13
            rdx = r13;
            // c525c cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // c5260 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c5263 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5265;
        }
        0xc5265 => {
            // block c5265
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5265 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // c5269 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c526e test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c5270 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // c5274 je 0xc527a
            if fl.e() { pc = 0xc527a; continue; }
            pc = 0xc5276;
        }
        0xc5276 => {
            // block c5276
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5276 jns 0xc528c
            if fl.ns() { pc = 0xc528c; continue; }
            pc = 0xc5278;
        }
        0xc5278 => {
            // block c5278
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5278 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5278 jmp 0xc527f
            pc = 0xc527f; continue;
        }
        0xc527a => {
            // block c527a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527a cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c527d jae 0xc528c
            if fl.ae() { pc = 0xc528c; continue; }
            pc = 0xc527f;
        }
        0xc527f => {
            // block c527f
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c527f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c527f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c5282 test rax,rax
            fl = logic::<64>(rax & rax);
            // c5285 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc5287;
        }
        0xc5287 => {
            // block c5287
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5287 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5287 jmp 0xc5323
            pc = 0xc5323; continue;
        }
        0xc528c => {
            // block c528c
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c528c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c528c test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c528f jne 0xc5299
            if fl.ne() { pc = 0xc5299; continue; }
            pc = 0xc5291;
        }
        0xc5291 => {
            // block c5291
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5291 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5291 mov r14,rbx
            r14 = rbx;
            // c5294 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // c5297 jmp 0xc52a0
            pc = 0xc52a0; continue;
        }
        0xc5299 => {
            // block c5299
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5299 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5299 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // c529d mov r14,rbx
            r14 = rbx;
            pc = 0xc52a0;
        }
        0xc52a0 => {
            // block c52a0
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52a0 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c52a4 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52a7 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c52ab cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // c52b0 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // c52b3 jne 0xc52ba
            if fl.ne() { pc = 0xc52ba; continue; }
            pc = 0xc52b5;
        }
        0xc52b5 => {
            // block c52b5
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52b5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // c52b8 jmp 0xc52be
            pc = 0xc52be; continue;
        }
        0xc52ba => {
            // block c52ba
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ba mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xc52be;
        }
        0xc52be => {
            // block c52be
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52be cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // c52c1 mov rdx,rax
            rdx = rax;
            // c52c4 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // c52c8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // c52cb je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52cd;
        }
        0xc52cd => {
            // block c52cd
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52cd test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // c52d0 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // c52d4 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // c52d9 mov r12,rax
            r12 = rax;
            // c52dc call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c52e1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // c52e3 mov rax,r12
            rax = r12;
            // c52e6 je 0xc52ec
            if fl.e() { pc = 0xc52ec; continue; }
            pc = 0xc52e8;
        }
        0xc52e8 => {
            // block c52e8
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52e8 js 0xc52f1
            if fl.s() { pc = 0xc52f1; continue; }
            pc = 0xc52ea;
        }
        0xc52ea => {
            // block c52ea
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ea jmp 0xc531a
            pc = 0xc531a; continue;
        }
        0xc52ec => {
            // block c52ec
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52ec cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // c52ef jae 0xc531a
            if fl.ae() { pc = 0xc531a; continue; }
            pc = 0xc52f1;
        }
        0xc52f1 => {
            // block c52f1
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52f1 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // c52f5 test rax,rax
            fl = logic::<64>(rax & rax);
            // c52f8 jne 0xc5207
            if fl.ne() { pc = 0xc5207; continue; }
            pc = 0xc52fe;
        }
        0xc52fe => {
            // block c52fe
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c52fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c52fe mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5302 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5305 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c5309 mov rax,r14
            rax = r14;
            // c530c jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc530e => {
            // block c530e
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c530e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c530e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c5312 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // c5315 mov rax,rdi
            rax = rdi;
            // c5318 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc531a => {
            // block c531a
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c531a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c531a mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c531e mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // c5321 jmp 0xc532d
            pc = 0xc532d; continue;
        }
        0xc5323 => {
            // block c5323
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c5323 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c5323 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // c5327 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // c532a mov rax,rbx
            rax = rbx;
            pc = 0xc532d;
        }
        0xc532d => {
            // block c532d
            #[cfg(feature = "trace")] eprintln!("f_000c51ba pc c532d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c532d add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // c5331 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c5332 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c5334 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c5336 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c5338 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c533a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c533b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
