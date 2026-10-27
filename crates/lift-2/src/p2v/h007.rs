//! Mechanically lifted functions (scripts/lift_run.py), module h007. Each one is checked against the original
//! in the world harness (tests/world_h007.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x6d8 size=732
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000006d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6d8;
    loop {
        match pc {
        0x6d8 => {
            // block 6d8
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 6d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6d9 mov rbp,rsp
            rbp = rsp;
            // 6dc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6de mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 6e1 cmp esi,0x9
            fl = sub::<32>((rsi as u32 as u64), 0x9u64).1;
            // 6e4 ja 0x7de
            if fl.a() { pc = 0x7de; continue; }
            pc = 0x6ea;
        }
        0x6ea => {
            // block 6ea
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 6ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6ea lea r10,[rip+0x257] # 0x948
            r10 = m.env.code_va(0x948);
            // 6f1 movsxd rsi,DWORD PTR [r10+r9*4]
            rsi = sx::<32>(m.r32(r10.wrapping_add(r9.wrapping_mul(4))) as u64) as u64;
            // 6f5 add rsi,r10
            { let (r, f) = add::<64>(rsi, r10);
              rsi = r;
              fl = f; }
            // 6f8 jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x6fa => {
            // block 6fa
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 6fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6fa add edi,0xfffffffd
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfffffffdu64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 6fd cmp edi,0x4
            fl = sub::<32>((rdi as u32 as u64), 0x4u64).1;
            // 700 ja 0x7de
            if fl.a() { pc = 0x7de; continue; }
            pc = 0x706;
        }
        0x706 => {
            // block 706
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 706 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 706 lea rax,[rip+0x293] # 0x9a0
            rax = m.env.code_va(0x9a0);
            // 70d movsxd rsi,DWORD PTR [rax+rdi*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // 711 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 714 jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x716 => {
            // block 716
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 716 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 718 je 0x728
            if fl.e() { pc = 0x728; continue; }
            pc = 0x71a;
        }
        0x71a => {
            // block 71a
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 71a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 71a mov eax,0x14
            rax = (0x14u64) as u32 as u64;
            // 71f test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 722 jne 0x7de
            if fl.ne() { pc = 0x7de; continue; }
            pc = 0x728;
        }
        0x728 => {
            // block 728
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 728 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 72b mov ecx,0x1e
            rcx = (0x1eu64) as u32 as u64;
            pc = 0x730;
        }
        0x730 => {
            // block 730
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 730 mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // 735 cmovne eax,ecx
            if fl.ne() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 738 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x73d => {
            // block 73d
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 73d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 73d dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 73f cmp edi,0xb
            fl = sub::<32>((rdi as u32 as u64), 0xbu64).1;
            // 742 ja 0x7de
            if fl.a() { pc = 0x7de; continue; }
            pc = 0x748;
        }
        0x748 => {
            // block 748
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 748 lea rax,[rip+0x221] # 0x970
            rax = m.env.code_va(0x970);
            // 74f movsxd rdx,DWORD PTR [rax+rdi*4]
            rdx = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // 753 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 756 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x758 => {
            // block 758
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 758 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 758 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 75b cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 75e mov ecx,0x6e
            rcx = (0x6eu64) as u32 as u64;
            // 763 mov eax,0x19
            rax = (0x19u64) as u32 as u64;
            // 768 cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 76b jmp 0x7de
            pc = 0x7de; continue;
        }
        0x76d => {
            // block 76d
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 76d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 76d dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 76f cmp edi,0x8
            fl = sub::<32>((rdi as u32 as u64), 0x8u64).1;
            // 772 jae 0x7de
            if fl.ae() { pc = 0x7de; continue; }
            pc = 0x774;
        }
        0x774 => {
            // block 774
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 774 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 774 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 777 lea rcx,[rip+0xfefe2] # 0xff760
            rcx = m.env.data_va(81709);
            // 77e jmp 0x7db
            pc = 0x7db; continue;
        }
        0x780 => {
            // block 780
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 780 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 780 cmp edi,0x7
            fl = sub::<32>((rdi as u32 as u64), 0x7u64).1;
            // 783 je 0x7e0
            if fl.e() { pc = 0x7e0; continue; }
            pc = 0x785;
        }
        0x785 => {
            // block 785
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 785 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 785 cmp edi,0x6
            fl = sub::<32>((rdi as u32 as u64), 0x6u64).1;
            // 788 jne 0x7e7
            if fl.ne() { pc = 0x7e7; continue; }
            pc = 0x78a;
        }
        0x78a => {
            // block 78a
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 78a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 78a mov eax,0x23
            rax = (0x23u64) as u32 as u64;
            // 78f jmp 0x7de
            pc = 0x7de; continue;
        }
        0x791 => {
            // block 791
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 791 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 791 cmp edi,0x5
            fl = sub::<32>((rdi as u32 as u64), 0x5u64).1;
            // 794 jne 0x7de
            if fl.ne() { pc = 0x7de; continue; }
            pc = 0x796;
        }
        0x796 => {
            // block 796
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 796 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 796 movzx eax,r8b
            rax = ((r8 as u8 as u64)) as u32 as u64;
            // 79a lea eax,[rax*4+0x21]
            rax = (rax.wrapping_mul(4).wrapping_add(0x21)) as u32 as u64;
            pc = 0x7a1;
        }
        0x7a1 => {
            // block 7a1
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a1 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x7a3 => {
            // block 7a3
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7a3 add edi,0xfffffffb
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xfffffffbu64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 7a6 cmp edi,0xa
            fl = sub::<32>((rdi as u32 as u64), 0xau64).1;
            // 7a9 jae 0x7de
            if fl.ae() { pc = 0x7de; continue; }
            pc = 0x7ab;
        }
        0x7ab => {
            // block 7ab
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ab movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 7ae lea rcx,[rip+0xfefcb] # 0xff780
            rcx = m.env.data_va(81741);
            // 7b5 jmp 0x7db
            pc = 0x7db; continue;
        }
        0x7b7 => {
            // block 7b7
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7b7 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 7b9 cmp edi,0x8
            fl = sub::<32>((rdi as u32 as u64), 0x8u64).1;
            // 7bc jae 0x7de
            if fl.ae() { pc = 0x7de; continue; }
            pc = 0x7be;
        }
        0x7be => {
            // block 7be
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7be movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            pc = 0x7c1;
        }
        0x7c1 => {
            // block 7c1
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7c1 lea rcx,[rip+0xfefe8] # 0xff7b0
            rcx = m.env.data_va(81789);
            // 7c8 jmp 0x7db
            pc = 0x7db; continue;
        }
        0x7ca => {
            // block 7ca
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ca dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 7cc cmp edi,0x8
            fl = sub::<32>((rdi as u32 as u64), 0x8u64).1;
            // 7cf jae 0x7de
            if fl.ae() { pc = 0x7de; continue; }
            pc = 0x7d1;
        }
        0x7d1 => {
            // block 7d1
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d1 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 7d4 lea rcx,[rip+0xfeff5] # 0xff7d0
            rcx = m.env.data_va(81821);
            pc = 0x7db;
        }
        0x7db => {
            // block 7db
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7db mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x7de;
        }
        0x7de => {
            // block 7de
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7de pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7df ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x7e0 => {
            // block 7e0
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e0 mov eax,0x22
            rax = (0x22u64) as u32 as u64;
            // 7e5 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x7e7 => {
            // block 7e7
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e7 cmp edi,0x4
            fl = sub::<32>((rdi as u32 as u64), 0x4u64).1;
            // 7ea jne 0x7de
            if fl.ne() { pc = 0x7de; continue; }
            pc = 0x7ec;
        }
        0x7ec => {
            // block 7ec
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec movzx eax,r8b
            rax = ((r8 as u8 as u64)) as u32 as u64;
            // 7f0 lea eax,[rax*4+0x20]
            rax = (rax.wrapping_mul(4).wrapping_add(0x20)) as u32 as u64;
            // 7f7 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x7f9 => {
            // block 7f9
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f9 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 7fb je 0x807
            if fl.e() { pc = 0x807; continue; }
            pc = 0x7fd;
        }
        0x7fd => {
            // block 7fd
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 7fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fd mov eax,0x13
            rax = (0x13u64) as u32 as u64;
            // 802 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 805 jne 0x7de
            if fl.ne() { pc = 0x7de; continue; }
            pc = 0x807;
        }
        0x807 => {
            // block 807
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 807 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 80a mov ecx,0x1d
            rcx = (0x1du64) as u32 as u64;
            // 80f mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // 814 cmovne eax,ecx
            if fl.ne() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 817 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x819 => {
            // block 819
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 819 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 819 test dl,dl
            fl = logic::<8>((rdx as u8 as u64) & (rdx as u8 as u64));
            // 81b je 0x858
            if fl.e() { pc = 0x858; continue; }
            pc = 0x81d;
        }
        0x81d => {
            // block 81d
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 81d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 81d test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 820 je 0x858
            if fl.e() { pc = 0x858; continue; }
            pc = 0x822;
        }
        0x822 => {
            // block 822
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 822 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 825 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 828 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 82b movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 82e lea eax,[rax+rax*8+0x12]
            rax = (rax.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x12)) as u32 as u64;
            // 832 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x834 => {
            // block 834
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 834 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 837 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 83a sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 83d movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 840 lea eax,[rax+rax*8+0x7]
            rax = (rax.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x7)) as u32 as u64;
            // 844 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x846 => {
            // block 846
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 846 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 846 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 849 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 84c sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 84f movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 852 lea eax,[rax+rax*8+0x6]
            rax = (rax.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x6)) as u32 as u64;
            pc = 0x856;
        }
        0x856 => {
            // block 856
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 856 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 856 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x858 => {
            // block 858
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 858 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 858 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 85b mov eax,0x1b
            rax = (0x1bu64) as u32 as u64;
            // 860 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // 863 je 0x7de
            if fl.e() { pc = 0x7de; continue; }
            pc = 0x869;
        }
        0x869 => {
            // block 869
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 869 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // 86c mov ecx,0x1c
            rcx = (0x1cu64) as u32 as u64;
            // 871 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // 876 cmovne eax,ecx
            if fl.ne() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 879 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x87e => {
            // block 87e
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 87e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 87e movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 881 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 884 mov ecx,0x6f
            rcx = (0x6fu64) as u32 as u64;
            // 889 mov eax,0x18
            rax = (0x18u64) as u32 as u64;
            // 88e cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 891 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x896 => {
            // block 896
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 896 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 896 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 899 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 89c mov ecx,0x71
            rcx = (0x71u64) as u32 as u64;
            // 8a1 mov eax,0x9
            rax = (0x9u64) as u32 as u64;
            // 8a6 cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 8a9 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x8ae => {
            // block 8ae
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 8ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ae movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 8b1 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 8b4 mov ecx,0x70
            rcx = (0x70u64) as u32 as u64;
            // 8b9 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // 8be cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 8c1 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x8c6 => {
            // block 8c6
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 8c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8c6 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 8c9 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 8cc mov ecx,0x1a
            rcx = (0x1au64) as u32 as u64;
            // 8d1 mov eax,0xa
            rax = (0xau64) as u32 as u64;
            // 8d6 cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 8d9 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x8de => {
            // block 8de
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 8de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8de movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 8e1 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 8e4 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 8e7 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 8ea lea eax,[rax+rax*1+0xc]
            rax = (rax.wrapping_add(rax).wrapping_add(0xc)) as u32 as u64;
            // 8ee jmp 0x7de
            pc = 0x7de; continue;
        }
        0x8f3 => {
            // block 8f3
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 8f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8f3 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 8f6 cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 8f9 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 8fc movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 8ff lea eax,[rax+rax*1+0xb]
            rax = (rax.wrapping_add(rax).wrapping_add(0xb)) as u32 as u64;
            // 903 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x908 => {
            // block 908
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 908 movzx eax,cl
            rax = ((rcx as u8 as u64)) as u32 as u64;
            // 90b cmp eax,0x8
            fl = sub::<32>((rax as u32 as u64), 0x8u64).1;
            // 90e mov ecx,0x72
            rcx = (0x72u64) as u32 as u64;
            // 913 mov eax,0x17
            rax = (0x17u64) as u32 as u64;
            // 918 cmove eax,ecx
            if fl.e() { rax = ((rcx as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // 91b jmp 0x7de
            pc = 0x7de; continue;
        }
        0x920 => {
            // block 920
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 920 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 920 mov eax,0x74
            rax = (0x74u64) as u32 as u64;
            // 925 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x92a => {
            // block 92a
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 92a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92a mov eax,0x73
            rax = (0x73u64) as u32 as u64;
            // 92f jmp 0x7de
            pc = 0x7de; continue;
        }
        0x934 => {
            // block 934
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 934 mov eax,0x6d
            rax = (0x6du64) as u32 as u64;
            // 939 jmp 0x7de
            pc = 0x7de; continue;
        }
        0x93e => {
            // block 93e
            #[cfg(feature = "trace")] eprintln!("f_000006d8 pc 93e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 93e mov eax,0x6c
            rax = (0x6cu64) as u32 as u64;
            // 943 jmp 0x7de
            pc = 0x7de; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10f58 size=366
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010f58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10f58;
    loop {
        match pc {
        0x10f58 => {
            // block 10f58
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10f59 mov rbp,rsp
            rbp = rsp;
            // 10f5c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10f5e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10f60 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10f62 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10f64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10f65 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f69 mov r14,rsi
            r14 = rsi;
            // 10f6c mov r15,rdi
            r15 = rdi;
            // 10f6f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 10f72 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10f76 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 10f79 jne 0x110ab
            if fl.ne() { pc = 0x110ab; continue; }
            pc = 0x10f7f;
        }
        0x10f7f => {
            // block 10f7f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f7f mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10f83 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f87 cmp r12,rcx
            fl = sub::<64>(r12, rcx).1;
            // 10f8a jae 0x10fd9
            if fl.ae() { pc = 0x10fd9; continue; }
            pc = 0x10f8c;
        }
        0x10f8c => {
            // block 10f8c
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10f8c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f8c sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 10f8f sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 10f93 lea rax,[rcx+0x1]
            rax = rcx.wrapping_add(0x1);
            // 10f97 shr rax,0x3f
            { let (r, f) = shr::<64>(rax, 0x3fu64, fl); rax = r; fl = f; }
            // 10f9b lea rbx,[rcx+rax*1+0x1]
            rbx = rcx.wrapping_add(rax).wrapping_add(0x1);
            // 10fa0 sar rbx,1
            { let (r, f) = sar::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 10fa3 mov rdx,r12
            rdx = r12;
            // 10fa6 sub rdx,r13
            { let (r, f) = sub::<64>(rdx, r13);
              rdx = r;
              fl = f; }
            // 10fa9 mov rax,rdx
            rax = rdx;
            // 10fac shr rax,0x3
            { let (r, f) = shr::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10fb0 mov rcx,rbx
            rcx = rbx;
            // 10fb3 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 10fb6 lea r12,[r12+rcx*8]
            r12 = r12.wrapping_add(rcx.wrapping_mul(8));
            // 10fba mov rdi,r12
            rdi = r12;
            // 10fbd mov rsi,r13
            rsi = r13;
            // 10fc0 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10fc5 mov QWORD PTR [r15+0x8],r12
            m.w64(r15.wrapping_add(0x8), (r12) as u64);
            // 10fc9 shl rbx,0x3
            { let (r, f) = shl::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 10fcd add QWORD PTR [r15+0x10],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, rbx);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10fd1 mov r13,r12
            r13 = r12;
            // 10fd4 jmp 0x110ab
            pc = 0x110ab; continue;
        }
        0x10fd9 => {
            // block 10fd9
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 10fd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10fd9 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10fdd mov QWORD PTR [rbp-0x58],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r14) as u64);
            // 10fe1 mov rax,rcx
            rax = rcx;
            // 10fe4 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 10fe7 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10feb mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10ff1 cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10ff5 lea rbx,[r14+r14*1+0x6]
            rbx = r14.wrapping_add(r14).wrapping_add(0x6);
            // 10ffa and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 10ffe mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 11002 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 11006 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 1100e mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 11012 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11017 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 1101b add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 1101e mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 11022 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 11026 lea rcx,[rax+r14*8]
            rcx = rax.wrapping_add(r14.wrapping_mul(8));
            // 1102a mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 1102e cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 11031 je 0x11070
            if fl.e() { pc = 0x11070; continue; }
            pc = 0x11033;
        }
        0x11033 => {
            // block 11033
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11033 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11033 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x11037;
        }
        0x11037 => {
            // block 11037
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11037 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11037 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1103b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1103e mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 11042 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 11046 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 1104a add r13,0x8
            { let (r, f) = add::<64>(r13, 0x8u64);
              r13 = r;
              fl = f; }
            // 1104e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 11051 jne 0x11037
            if fl.ne() { pc = 0x11037; continue; }
            pc = 0x11053;
        }
        0x11053 => {
            // block 11053
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11053 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11053 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 11056 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1105a mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 1105e mov rdx,QWORD PTR [rbp-0x48]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 11062 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 11066 mov rsi,QWORD PTR [r15+0x18]
            rsi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1106a mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1106e jmp 0x1107f
            pc = 0x1107f; continue;
        }
        0x11070 => {
            // block 11070
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 11070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11070 mov rdx,rbx
            rdx = rbx;
            // 11073 mov r14,QWORD PTR [rbp-0x58]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 11077 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1107b mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            pc = 0x1107f;
        }
        0x1107f => {
            // block 1107f
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 1107f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1107f mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 11082 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 11086 mov QWORD PTR [r15+0x8],rdx
            m.w64(r15.wrapping_add(0x8), (rdx) as u64);
            // 1108a mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 1108e mov QWORD PTR [r15+0x10],rbx
            m.w64(r15.wrapping_add(0x10), (rbx) as u64);
            // 11092 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 11096 mov QWORD PTR [r15+0x18],rcx
            m.w64(r15.wrapping_add(0x18), (rcx) as u64);
            // 1109a mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 1109e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 110a2 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 110a7 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x110ab;
        }
        0x110ab => {
            // block 110ab
            #[cfg(feature = "trace")] eprintln!("f_00010f58 pc 110ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 110ab mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 110ae mov QWORD PTR [r13-0x8],rax
            m.w64(r13.wrapping_add(0xffff_ffff_ffff_fff8), (rax) as u64);
            // 110b2 add QWORD PTR [r15+0x8],0xfffffffffffffff8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xffff_ffff_ffff_fff8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 110b7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 110bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 110bc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 110be pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 110c0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 110c2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 110c4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 110c5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xda88 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000da88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xda88;
    loop {
        match pc {
        0xda88 => {
            // block da88
            #[cfg(feature = "trace")] eprintln!("f_0000da88 pc da88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // da88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // da89 mov rbp,rsp
            rbp = rsp;
            // da8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // da8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // da8f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // da92 mov rbx,rdi
            rbx = rdi;
            // da95 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // da98 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // da9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // daa0 je 0xdaa6
            if fl.e() { pc = 0xdaa6; continue; }
            pc = 0xdaa2;
        }
        0xdaa2 => {
            // block daa2
            #[cfg(feature = "trace")] eprintln!("f_0000da88 pc daa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // daa2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // daa4 jmp 0xdade
            pc = 0xdade; continue;
        }
        0xdaa6 => {
            // block daa6
            #[cfg(feature = "trace")] eprintln!("f_0000da88 pc daa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // daa6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // daa9 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // daae xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // dab0 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // dab5 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // dabb mov rdi,rbx
            rdi = rbx;
            // dabe call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dac1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dac4 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // dac8 mov esi,0xa9
            rsi = (0xa9u64) as u32 as u64;
            // dacd mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // dad2 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // dad8 mov rdi,rbx
            rdi = rbx;
            // dadb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xdade;
        }
        0xdade => {
            // block dade
            #[cfg(feature = "trace")] eprintln!("f_0000da88 pc dade rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dade pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dadf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dae1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dae2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a270 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0001a270<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1a270;
    loop {
        match pc {
        0x1a270 => {
            // block 1a270
            #[cfg(feature = "trace")] eprintln!("f_0001a270 pc 1a270 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a270 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a271 mov rbp,rsp
            rbp = rsp;
            // 1a274 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a276 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a277 mov r14,rsi
            r14 = rsi;
            // 1a27a mov rbx,rdi
            rbx = rdi;
            // 1a27d mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1a285 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 1a28d mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 1a294 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a298 sub rsi,QWORD PTR [r14]
            { let (r, f) = sub::<64>(rsi, m.r64(r14) as u64);
              rsi = r;
              fl = f; }
            // 1a29b sar rsi,0x2
            { let (r, f) = sar::<64>(rsi, 0x2u64, fl); rsi = r; fl = f; }
            // 1a29f je 0x1a2c7
            if fl.e() { pc = 0x1a2c7; continue; }
            pc = 0x1a2a1;
        }
        0x1a2a1 => {
            // block 1a2a1
            #[cfg(feature = "trace")] eprintln!("f_0001a270 pc 1a2a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a2a1 mov rdi,rbx
            rdi = rbx;
            // 1a2a4 call 0xeba20
            { let r = { let t = m.env.stub_va(0x1ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2a9 mov rsi,QWORD PTR [r14]
            rsi = m.r64(r14) as u64;
            // 1a2ac mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 1a2b0 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // 1a2b3 mov r14,rdx
            r14 = rdx;
            // 1a2b6 and r14,0xfffffffffffffffc
            { let r = r14 & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              r14 = r;
              fl = f; }
            // 1a2ba mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 1a2be call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2c3 add QWORD PTR [rbx+0x8],r14
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, r14);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x1a2c7;
        }
        0x1a2c7 => {
            // block 1a2c7
            #[cfg(feature = "trace")] eprintln!("f_0001a270 pc 1a2c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a2c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a2c8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a2ca pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a2cb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1a2cc => {
            // block 1a2cc
            #[cfg(feature = "trace")] eprintln!("f_0001a270 pc 1a2cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a2cc mov r14,rax
            r14 = rax;
            // 1a2cf mov rdi,rbx
            rdi = rbx;
            // 1a2d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a2d7 mov rdi,r14
            rdi = r14;
            // 1a2da call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43ade size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043ade<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43ade;
    loop {
        match pc {
        0x43ade => {
            // block 43ade
            #[cfg(feature = "trace")] eprintln!("f_00043ade pc 43ade rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ade push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43adf mov rbp,rsp
            rbp = rsp;
            // 43ae2 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43ae5 jle 0x43aeb
            if fl.le() { pc = 0x43aeb; continue; }
            pc = 0x43ae7;
        }
        0x43ae7 => {
            // block 43ae7
            #[cfg(feature = "trace")] eprintln!("f_00043ade pc 43ae7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ae7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43ae9 jmp 0x43b10
            pc = 0x43b10; continue;
        }
        0x43aeb => {
            // block 43aeb
            #[cfg(feature = "trace")] eprintln!("f_00043ade pc 43aeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43aeb mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 43aed lea rcx,[rip+0xbcacc] # 0x1005c0
            rcx = m.env.data_va(85389);
            // 43af4 mov ecx,DWORD PTR [rcx+rax*4]
            rcx = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 43af7 lea rsi,[rip+0xbcaf2] # 0x1005f0
            rsi = m.env.data_va(85437);
            // 43afe mov r8d,DWORD PTR [rsi+rax*4]
            r8 = (m.r32(rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b02 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43b05 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43b08 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 43b0d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43b10;
        }
        0x43b10 => {
            // block 43b10
            #[cfg(feature = "trace")] eprintln!("f_00043ade pc 43b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b10 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43b11 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11bb4 size=61
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011bb4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11bb4;
    loop {
        match pc {
        0x11bb4 => {
            // block 11bb4
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11bb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bb4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11bb5 mov rbp,rsp
            rbp = rsp;
            // 11bb8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11bba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11bbb mov rbx,rdi
            rbx = rdi;
            // 11bbe mov r14,QWORD PTR [rbx+0x8]
            r14 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 11bc2 jmp 0x11bd2
            pc = 0x11bd2; continue;
        }
        0x11bc4 => {
            // block 11bc4
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11bc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bc4 lea rdi,[rax-0x38]
            rdi = rax.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 11bc8 mov QWORD PTR [rbx+0x10],rdi
            m.w64(rbx.wrapping_add(0x10), (rdi) as u64);
            // 11bcc mov rax,QWORD PTR [rax-0x38]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 11bd0 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11bd2;
        }
        0x11bd2 => {
            // block 11bd2
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11bd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bd2 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 11bd6 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 11bd9 jne 0x11bc4
            if fl.ne() { pc = 0x11bc4; continue; }
            pc = 0x11bdb;
        }
        0x11bdb => {
            // block 11bdb
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11bdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bdb mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 11bde test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11be1 je 0x11bec
            if fl.e() { pc = 0x11bec; continue; }
            pc = 0x11be3;
        }
        0x11be3 => {
            // block 11be3
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11be3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11be3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11be4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11be6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11be7 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x11bec => {
            // block 11bec
            #[cfg(feature = "trace")] eprintln!("f_00011bb4 pc 11bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11bec pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11bed pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11bef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11bf0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3dfe size=246
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003dfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3dfe;
    loop {
        match pc {
        0x3dfe => {
            // block 3dfe
            #[cfg(feature = "trace")] eprintln!("f_00003dfe pc 3dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3dfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3dff mov rbp,rsp
            rbp = rsp;
            // 3e02 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 3e04 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3e06 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3e08 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3e09 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3e0d mov r14,rcx
            r14 = rcx;
            // 3e10 mov r15,rdx
            r15 = rdx;
            // 3e13 mov r12,rsi
            r12 = rsi;
            // 3e16 mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 3e1c mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 3e22 mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // 3e29 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 3e30 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 3e33 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 3e37 mov esi,0x58
            rsi = (0x58u64) as u32 as u64;
            // 3e3c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3e41 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3e44 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3e47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 3e49 je 0x3ee4
            if fl.e() { pc = 0x3ee4; continue; }
            pc = 0x3e4f;
        }
        0x3e4f => {
            // block 3e4f
            #[cfg(feature = "trace")] eprintln!("f_00003dfe pc 3e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3e4f mov ebx,DWORD PTR [rbp-0x24]
            rbx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 3e52 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3e54 shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 3e57 mov ecx,ebx
            rcx = ((rbx as u32 as u64)) as u32 as u64;
            // 3e59 shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 3e5c mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 3e5e shr esi,0x1c
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1cu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 3e61 imul esi,esi,0x3e8
            { let (r, f) = imul::<32>((rsi as u32 as u64), 0x3e8u64); rsi = (r) as u32 as u64; fl = f; }
            // 3e67 mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 3e69 shr edi,0x18
            { let (r, f) = shr::<32>((rdi as u32 as u64), 0x18u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // 3e6c and edi,0xf
            { let r = (rdi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            // 3e6f imul edi,edi,0x64
            { let (r, f) = imul::<32>((rdi as u32 as u64), 0x64u64); rdi = (r) as u32 as u64; fl = f; }
            // 3e72 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 3e74 shr eax,0x14
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x14u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 3e77 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3e7a lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // 3e7d and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3e80 add edx,esi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rsi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3e82 add edx,edi
            { let (r, f) = add::<32>((rdx as u32 as u64), (rdi as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 3e84 lea eax,[rdx+rax*2]
            rax = (rdx.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 3e87 mov WORD PTR [r12],ax
            m.w16(r12, ((rax as u16 as u64)) as u16);
            // 3e8c mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 3e8e shr eax,0xc
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xcu64, fl); rax = (r) as u32 as u64; fl = f; }
            // 3e91 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3e94 lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // 3e97 and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3e9a lea eax,[rcx+rax*2]
            rax = (rcx.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 3e9d mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // 3ea1 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 3ea3 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 3ea6 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3ea9 lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // 3eac and ebx,0xf
            { let r = (rbx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3eaf lea eax,[rbx+rax*2]
            rax = (rbx.wrapping_add(rax.wrapping_mul(2))) as u32 as u64;
            // 3eb2 mov WORD PTR [r14],ax
            m.w16(r14, ((rax as u16 as u64)) as u16);
            // 3eb6 movzx ecx,WORD PTR [r12]
            rcx = (m.r16(r12) as u64) as u32 as u64;
            // 3ebb add ecx,0xfffff825
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xfffff825u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3ec1 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // 3ec4 cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 3ec7 ja 0x3ee4
            if fl.a() { pc = 0x3ee4; continue; }
            pc = 0x3ec9;
        }
        0x3ec9 => {
            // block 3ec9
            #[cfg(feature = "trace")] eprintln!("f_00003dfe pc 3ec9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ec9 movzx ecx,WORD PTR [r15]
            rcx = (m.r16(r15) as u64) as u32 as u64;
            // 3ecd dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 3ecf movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // 3ed2 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // 3ed5 sbb cl,cl
            { let (r, f) = sbb::<8>((rcx as u8 as u64), (rcx as u8 as u64), fl.cf);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3ed7 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 3ed9 cmp eax,0x1f
            fl = sub::<32>((rax as u32 as u64), 0x1fu64).1;
            // 3edc sbb al,al
            { let (r, f) = sbb::<8>((rax as u8 as u64), (rax as u8 as u64), fl.cf);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3ede and al,cl
            { let r = (rax as u8 as u64) & (rcx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3ee0 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 3ee2 jmp 0x3ee6
            pc = 0x3ee6; continue;
        }
        0x3ee4 => {
            // block 3ee4
            #[cfg(feature = "trace")] eprintln!("f_00003dfe pc 3ee4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ee4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3ee6;
        }
        0x3ee6 => {
            // block 3ee6
            #[cfg(feature = "trace")] eprintln!("f_00003dfe pc 3ee6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3ee6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3eea pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3eeb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3eed pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3eef pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 3ef1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3ef2 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 159f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a31 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // 15a34 jmp 0x15a3a
            pc = 0x15a3a; continue;
        }
        0x15a36 => {
            // block 15a36
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a36 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            pc = 0x15a3a;
        }
        0x15a3a => {
            // block 15a3a
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a44 shr rbx,1
            { let (r, f) = shr::<64>(rbx, 0x1u64, fl); rbx = r; fl = f; }
            // 15a47 jmp 0x15a4d
            pc = 0x15a4d; continue;
        }
        0x15a49 => {
            // block 15a49
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a49 mov rbx,QWORD PTR [r15+0x28]
            rbx = m.r64(r15.wrapping_add(0x28)) as u64;
            pc = 0x15a4d;
        }
        0x15a4d => {
            // block 15a4d
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a4d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a4d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 15a4f jne 0x15a56
            if fl.ne() { pc = 0x15a56; continue; }
            pc = 0x15a51;
        }
        0x15a51 => {
            // block 15a51
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a51 inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            // 15a54 jmp 0x15a5a
            pc = 0x15a5a; continue;
        }
        0x15a56 => {
            // block 15a56
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a56 mov r13,QWORD PTR [r13+0x10]
            r13 = m.r64(r13.wrapping_add(0x10)) as u64;
            pc = 0x15a5a;
        }
        0x15a5a => {
            // block 15a5a
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a5a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 15a5c jne 0x15a63
            if fl.ne() { pc = 0x15a63; continue; }
            pc = 0x15a5e;
        }
        0x15a5e => {
            // block 15a5e
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a5e inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 15a61 jmp 0x15a67
            pc = 0x15a67; continue;
        }
        0x15a63 => {
            // block 15a63
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a63 mov rsi,QWORD PTR [r15+0x30]
            rsi = m.r64(r15.wrapping_add(0x30)) as u64;
            pc = 0x15a67;
        }
        0x15a67 => {
            // block 15a67
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a82 jns 0x15a8e
            if fl.ns() { pc = 0x15a8e; continue; }
            pc = 0x15a84;
        }
        0x15a84 => {
            // block 15a84
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a84 jmp 0x15a8b
            pc = 0x15a8b; continue;
        }
        0x15a86 => {
            // block 15a86
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a86 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 15a89 jae 0x15a8e
            if fl.ae() { pc = 0x15a8e; continue; }
            pc = 0x15a8b;
        }
        0x15a8b => {
            // block 15a8b
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15a8b mov r15,r14
            r15 = r14;
            pc = 0x15a8e;
        }
        0x15a8e => {
            // block 15a8e
            #[cfg(feature = "trace")] eprintln!("f_000159f8 pc 15a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00006f36 pc 6f36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00006f36 pc 6f60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6f60 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 6f63 jmp 0x6fb1
            pc = 0x6fb1; continue;
        }
        0x6f65 => {
            // block 6f65
            #[cfg(feature = "trace")] eprintln!("f_00006f36 pc 6f65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00006f36 pc 6f8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00006f36 pc 6fb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x43b12 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043b12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43b12;
    loop {
        match pc {
        0x43b12 => {
            // block 43b12
            #[cfg(feature = "trace")] eprintln!("f_00043b12 pc 43b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43b13 mov rbp,rsp
            rbp = rsp;
            // 43b16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43b17 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43b18 mov rbx,rdx
            rbx = rdx;
            // 43b1b mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // 43b1e cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43b21 jle 0x43b27
            if fl.le() { pc = 0x43b27; continue; }
            pc = 0x43b23;
        }
        0x43b23 => {
            // block 43b23
            #[cfg(feature = "trace")] eprintln!("f_00043b12 pc 43b23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b23 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43b25 jmp 0x43b5b
            pc = 0x43b5b; continue;
        }
        0x43b27 => {
            // block 43b27
            #[cfg(feature = "trace")] eprintln!("f_00043b12 pc 43b27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b27 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43b2e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43b31 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 43b33 lea rcx,[rip+0xbca86] # 0x1005c0
            rcx = m.env.data_va(85389);
            // 43b3a mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b3d lea rsi,[rip+0xbcaac] # 0x1005f0
            rsi = m.env.data_va(85437);
            // 43b44 mov r8d,DWORD PTR [rsi+rdx*4]
            r8 = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43b48 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43b4c mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 43b51 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43b54 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43b58 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43b5b;
        }
        0x43b5b => {
            // block 43b5b
            #[cfg(feature = "trace")] eprintln!("f_00043b12 pc 43b5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43b5b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43b5f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43b60 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43b61 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4403e size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_0004403e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4403e;
    loop {
        match pc {
        0x4403e => {
            // block 4403e
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4403e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4403e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4403f mov rbp,rsp
            rbp = rsp;
            // 44042 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44044 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44046 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 44048 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44049 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 4404d mov r15,rcx
            r15 = rcx;
            // 44050 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 44052 lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 44056 mov QWORD PTR [rbp-0x28],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u64);
            // 4405e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 44066 mov QWORD PTR [rbp-0x38],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r12) as u64);
            // 4406a mov BYTE PTR [r15],0x0
            m.w8(r15, (0x0u64) as u8);
            // 4406e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44071 mov rax,QWORD PTR [rax+0x810]
            rax = m.r64(rax.wrapping_add(0x810)) as u64;
            // 44078 lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 4407c call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4407e mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 44082 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44084 je 0x440b4
            if fl.e() { pc = 0x440b4; continue; }
            pc = 0x44086;
        }
        0x44086 => {
            // block 44086
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44086 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 44089 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 4408c je 0x440cc
            if fl.e() { pc = 0x440cc; continue; }
            pc = 0x4408e;
        }
        0x4408e => {
            // block 4408e
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4408e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4408e mov rcx,r12
            rcx = r12;
            // 44091 mov rdx,rsi
            rdx = rsi;
            pc = 0x44094;
        }
        0x44094 => {
            // block 44094
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44094 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44094 mov rax,rdx
            rax = rdx;
            pc = 0x44097;
        }
        0x44097 => {
            // block 44097
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 44097 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44097 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 4409a jge 0x440a7
            if fl.ge() { pc = 0x440a7; continue; }
            pc = 0x4409c;
        }
        0x4409c => {
            // block 4409c
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 4409c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4409c mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 440a0 test rax,rax
            fl = logic::<64>(rax & rax);
            // 440a3 jne 0x44097
            if fl.ne() { pc = 0x44097; continue; }
            pc = 0x440a5;
        }
        0x440a5 => {
            // block 440a5
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440a5 jmp 0x440b9
            pc = 0x440b9; continue;
        }
        0x440a7 => {
            // block 440a7
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440a7 mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 440aa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 440ad mov rcx,rax
            rcx = rax;
            // 440b0 jne 0x44094
            if fl.ne() { pc = 0x44094; continue; }
            pc = 0x440b2;
        }
        0x440b2 => {
            // block 440b2
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b2 jmp 0x440bc
            pc = 0x440bc; continue;
        }
        0x440b4 => {
            // block 440b4
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b4 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 440b7 jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440b9 => {
            // block 440b9
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440b9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440b9 mov rax,rcx
            rax = rcx;
            pc = 0x440bc;
        }
        0x440bc => {
            // block 440bc
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440bc cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 440bf je 0x440ce
            if fl.e() { pc = 0x440ce; continue; }
            pc = 0x440c1;
        }
        0x440c1 => {
            // block 440c1
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440c1 cmp DWORD PTR [rax+0x1c],ebx
            fl = sub::<32>(m.r32(rax.wrapping_add(0x1c)) as u64, (rbx as u32 as u64)).1;
            // 440c4 jg 0x440ce
            if fl.g() { pc = 0x440ce; continue; }
            pc = 0x440c6;
        }
        0x440c6 => {
            // block 440c6
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440c6 mov BYTE PTR [r15],0x1
            m.w8(r15, (0x1u64) as u8);
            // 440ca jmp 0x440ce
            pc = 0x440ce; continue;
        }
        0x440cc => {
            // block 440cc
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440cc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0x440ce;
        }
        0x440ce => {
            // block 440ce
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440ce lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440d2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440d7 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 440da add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 440de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 440df pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 440e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 440e3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 440e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 440e6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x440e7 => {
            // block 440e7
            #[cfg(feature = "trace")] eprintln!("f_0004403e pc 440e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 440e7 mov rbx,rax
            rbx = rax;
            // 440ea mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 440ee lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 440f2 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440f7 mov rdi,rbx
            rdi = rbx;
            // 440fa call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x442e6 size=58
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000442e6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x442e6;
    loop {
        match pc {
        0x442e6 => {
            // block 442e6
            #[cfg(feature = "trace")] eprintln!("f_000442e6 pc 442e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 442e6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 442e7 mov rbp,rsp
            rbp = rsp;
            // 442ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 442eb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 442ec mov rbx,rsi
            rbx = rsi;
            // 442ef mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 442f6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 442f9 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 442fd mov esi,0x2886
            rsi = (0x2886u64) as u32 as u64;
            // 44302 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44307 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4430a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4430d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4430f je 0x44318
            if fl.e() { pc = 0x44318; continue; }
            pc = 0x44311;
        }
        0x44311 => {
            // block 44311
            #[cfg(feature = "trace")] eprintln!("f_000442e6 pc 44311 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44311 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 44315 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x44318;
        }
        0x44318 => {
            // block 44318
            #[cfg(feature = "trace")] eprintln!("f_000442e6 pc 44318 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44318 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 4431c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 4431d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4431e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15e10 size=268
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00015e10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15e10;
    loop {
        match pc {
        0x15e10 => {
            // block 15e10
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15e11 mov rbp,rsp
            rbp = rsp;
            // 15e14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15e16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15e18 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15e1a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15e1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15e1d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15e21 mov r14,rsi
            r14 = rsi;
            // 15e24 mov r12,rdi
            r12 = rdi;
            // 15e27 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 15e2b mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e30 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e33 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 15e36 jns 0x15e44
            if fl.ns() { pc = 0x15e44; continue; }
            pc = 0x15e38;
        }
        0x15e38 => {
            // block 15e38
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e38 mov rdi,r12
            rdi = r12;
            // 15e3b call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x15e44 => {
            // block 15e44
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e44 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 15e49 sub r15,rcx
            { let (r, f) = sub::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 15e4c movabs rax,0x3fffffffffffffff
            rax = 0x3fff_ffff_ffff_ffffu64;
            // 15e56 cmp r15,rax
            fl = sub::<64>(r15, rax).1;
            // 15e59 jae 0x15e87
            if fl.ae() { pc = 0x15e87; continue; }
            pc = 0x15e5b;
        }
        0x15e5b => {
            // block 15e5b
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e5b mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e5f add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // 15e62 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 15e65 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 15e69 mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e6e mov rbx,r13
            rbx = r13;
            // 15e71 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e74 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e78 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 15e7b test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15e7e mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 15e83 jne 0x15ea4
            if fl.ne() { pc = 0x15ea4; continue; }
            pc = 0x15e85;
        }
        0x15e85 => {
            // block 15e85
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e85 jmp 0x15eaf
            pc = 0x15eaf; continue;
        }
        0x15e87 => {
            // block 15e87
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15e87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15e87 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 15e8b mov r13,QWORD PTR [r12+0x8]
            r13 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 15e90 mov rbx,r13
            rbx = r13;
            // 15e93 sub rbx,rcx
            { let (r, f) = sub::<64>(rbx, rcx);
              rbx = r;
              fl = f; }
            // 15e96 mov QWORD PTR [rbp-0x38],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rcx) as u64);
            // 15e9a movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0x15ea4;
        }
        0x15ea4 => {
            // block 15ea4
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ea4 mov rdi,r15
            rdi = r15;
            // 15ea7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15eac mov r14,r15
            r14 = r15;
            pc = 0x15eaf;
        }
        0x15eaf => {
            // block 15eaf
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15eaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15eaf add r14,rax
            { let (r, f) = add::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 15eb2 mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15eb6 mov cl,BYTE PTR [rcx]
            rcx = (rcx & !0xff) | ((m.r8(rcx) as u64) as u8 as u64);
            // 15eb8 mov BYTE PTR [rax+rbx*1],cl
            m.w8(rax.wrapping_add(rbx), ((rcx as u8 as u64)) as u8);
            // 15ebb lea rcx,[rax+rbx*1+0x1]
            rcx = rax.wrapping_add(rbx).wrapping_add(0x1);
            // 15ec0 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 15ec4 mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15ec8 sub r13,r15
            { let (r, f) = sub::<64>(r13, r15);
              r13 = r;
              fl = f; }
            // 15ecb sub rbx,r13
            { let (r, f) = sub::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // 15ece add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 15ed1 mov rdi,rbx
            rdi = rbx;
            // 15ed4 mov rsi,r15
            rsi = r15;
            // 15ed7 mov rdx,r13
            rdx = r13;
            // 15eda call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15edf mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 15ee3 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 15ee7 mov QWORD PTR [r12+0x8],rax
            m.w64(r12.wrapping_add(0x8), (rax) as u64);
            // 15eec mov QWORD PTR [r12+0x10],r14
            m.w64(r12.wrapping_add(0x10), (r14) as u64);
            // 15ef1 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 15ef4 je 0x15f0c
            if fl.e() { pc = 0x15f0c; continue; }
            pc = 0x15ef6;
        }
        0x15ef6 => {
            // block 15ef6
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15ef6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ef6 mov rdi,r15
            rdi = r15;
            // 15ef9 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15efd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15efe pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f00 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f02 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f04 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f06 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f07 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0x15f0c => {
            // block 15f0c
            #[cfg(feature = "trace")] eprintln!("f_00015e10 pc 15f0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15f0c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 15f10 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15f11 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15f13 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15f15 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15f17 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15f19 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15f1a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17062 size=416
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017062<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17062;
    loop {
        match pc {
        0x17062 => {
            // block 17062
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17062 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17062 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17063 mov rbp,rsp
            rbp = rsp;
            // 17066 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 17068 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1706a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1706c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1706e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1706f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 17070 mov r15,rdx
            r15 = rdx;
            // 17073 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 17076 mov rbx,rdi
            rbx = rdi;
            // 17079 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 1707b je 0x17086
            if fl.e() { pc = 0x17086; continue; }
            pc = 0x1707d;
        }
        0x1707d => {
            // block 1707d
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1707d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1707d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17080 mov rdi,rbx
            rdi = rbx;
            // 17083 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x17086;
        }
        0x17086 => {
            // block 17086
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 17086 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17086 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 17089 mov rdi,rbx
            rdi = rbx;
            // 1708c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1708f mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 17092 mov rbx,QWORD PTR [rax]
            rbx = m.r64(rax) as u64;
            // 17095 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            // 17099 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 1709c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 170a0 movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 170aa imul rdx,rax
            { let (r, f) = imul::<64>(rdx, rax); rdx = r; fl = f; }
            // 170ae cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 170b1 jae 0x171f1
            if fl.ae() { pc = 0x171f1; continue; }
            pc = 0x170b7;
        }
        0x170b7 => {
            // block 170b7
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 170b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 170b7 imul r12,rcx,0xf8
            { let (r, f) = imul::<64>(rcx, 0xf8u64); r12 = r; fl = f; }
            // 170be lea r13,[rbx+r12*1]
            r13 = rbx.wrapping_add(r12);
            // 170c2 mov rax,QWORD PTR [rbx+r12*1+0x10]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64;
            // 170c7 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 170cb mov rax,QWORD PTR [rbx+r12*1]
            rax = m.r64(rbx.wrapping_add(r12)) as u64;
            // 170cf mov rcx,QWORD PTR [rbx+r12*1+0x8]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x8)) as u64;
            // 170d4 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 170d8 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 170db lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 170df lea rsi,[rbx+r12*1+0x18]
            rsi = rbx.wrapping_add(r12).wrapping_add(0x18);
            // 170e4 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 170e9 mov ax,WORD PTR [rbx+r12*1+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x5c)) as u64) as u16 as u64);
            // 170ef mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 170f4 mov eax,DWORD PTR [rbx+r12*1+0x58]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0x58)) as u64) as u32 as u64;
            // 170f9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 170fd mov rax,QWORD PTR [rbx+r12*1+0x50]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x50)) as u64;
            // 17102 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 17106 mov rax,QWORD PTR [rbx+r12*1+0x48]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x48)) as u64;
            // 1710b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1710f mov rax,QWORD PTR [rbx+r12*1+0x40]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x40)) as u64;
            // 17114 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 17118 mov rax,QWORD PTR [rbx+r12*1+0x30]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0x30)) as u64;
            // 1711d mov rcx,QWORD PTR [rbx+r12*1+0x38]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x38)) as u64;
            // 17122 mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 17126 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 1712a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1712d je 0x171ac
            if fl.e() { pc = 0x171ac; continue; }
            pc = 0x1712f;
        }
        0x1712f => {
            // block 1712f
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 1712f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1712f lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 17133 mov rsi,QWORD PTR [rbx+r12*1+0x60]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x60)) as u64;
            // 17138 mov rdx,QWORD PTR [rbx+r12*1+0x68]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x68)) as u64;
            // 1713d call 0xeba1a
            { let r = { let t = m.env.stub_va(0x1e7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17142 lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 17146 mov rsi,QWORD PTR [rbx+r12*1+0x78]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x78)) as u64;
            // 1714b mov rdx,QWORD PTR [rbx+r12*1+0x80]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x80)) as u64;
            // 17153 call 0xeba62
            { let r = { let t = m.env.stub_va(0x22f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17158 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1715f mov rsi,QWORD PTR [rbx+r12*1+0x90]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0x90)) as u64;
            // 17167 mov rdx,QWORD PTR [rbx+r12*1+0x98]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0x98)) as u64;
            // 1716f call 0xeba3e
            { let r = { let t = m.env.stub_va(0x20b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17174 lea rdi,[r15+0xa8]
            rdi = r15.wrapping_add(0xa8);
            // 1717b mov rsi,QWORD PTR [rbx+r12*1+0xa8]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xa8)) as u64;
            // 17183 mov rdx,QWORD PTR [rbx+r12*1+0xb0]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xb0)) as u64;
            // 1718b call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17190 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 17197 mov rsi,QWORD PTR [rbx+r12*1+0xc0]
            rsi = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc0)) as u64;
            // 1719f mov rdx,QWORD PTR [rbx+r12*1+0xc8]
            rdx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xc8)) as u64;
            // 171a7 call 0xeb9f6
            { let r = { let t = m.env.stub_va(0x1c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x171ac;
        }
        0x171ac => {
            // block 171ac
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171ac mov eax,DWORD PTR [rbx+r12*1+0xf0]
            rax = (m.r32(rbx.wrapping_add(r12).wrapping_add(0xf0)) as u64) as u32 as u64;
            // 171b4 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 171bb mov rax,QWORD PTR [rbx+r12*1+0xe8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe8)) as u64;
            // 171c3 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 171ca mov rax,QWORD PTR [rbx+r12*1+0xd8]
            rax = m.r64(rbx.wrapping_add(r12).wrapping_add(0xd8)) as u64;
            // 171d2 mov rcx,QWORD PTR [rbx+r12*1+0xe0]
            rcx = m.r64(rbx.wrapping_add(r12).wrapping_add(0xe0)) as u64;
            // 171da mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 171e1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 171e8 cmp DWORD PTR [r15+0x4],r14d
            fl = sub::<32>(m.r32(r15.wrapping_add(0x4)) as u64, (r14 as u32 as u64)).1;
            // 171ec sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 171ef jmp 0x171f3
            pc = 0x171f3; continue;
        }
        0x171f1 => {
            // block 171f1
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x171f3;
        }
        0x171f3 => {
            // block 171f3
            #[cfg(feature = "trace")] eprintln!("f_00017062 pc 171f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 171f3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 171f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 171f8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 171fa pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 171fc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 171fe pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 17200 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 17201 ret
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29700 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29748 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 2974b je 0x297a8
            if fl.e() { pc = 0x297a8; continue; }
            pc = 0x2974d;
        }
        0x2974d => {
            // block 2974d
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 2974d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29771 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29771 jmp 0x297ab
            pc = 0x297ab; continue;
        }
        0x29773 => {
            // block 29773
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29773 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 2977b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2977b cmp rcx,QWORD PTR [r14+0x10]
            fl = sub::<64>(rcx, m.r64(r14.wrapping_add(0x10)) as u64).1;
            // 2977f je 0x29797
            if fl.e() { pc = 0x29797; continue; }
            pc = 0x29781;
        }
        0x29781 => {
            // block 29781
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29781 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 29797 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 297a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 297a2 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 297a4 jne 0x29773
            if fl.ne() { pc = 0x29773; continue; }
            pc = 0x297a6;
        }
        0x297a6 => {
            // block 297a6
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 297a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 297a6 jmp 0x297ab
            pc = 0x297ab; continue;
        }
        0x297a8 => {
            // block 297a8
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 297a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 297a8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x297ab;
        }
        0x297ab => {
            // block 297ab
            #[cfg(feature = "trace")] eprintln!("f_00029700 pc 297ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x3cdc size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003cdc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3cdc;
    loop {
        match pc {
        0x3cdc => {
            // block 3cdc
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3cdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3cdc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3cdd mov rbp,rsp
            rbp = rsp;
            // 3ce0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 3ce2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3ce3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3ce7 mov rbx,rdi
            rbx = rdi;
            // 3cea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3ced lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 3cf1 mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // 3cf6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3cfb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3cfe call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d01 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // 3d08 lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            pc = 0x3d0c;
        }
        0x3d0c => {
            // block 3d0c
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d0c mov DWORD PTR [rbp-0x18],0x100
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0x100u64) as u32);
            // 3d13 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3d16 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 3d1b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3d20 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3d23 mov rdi,rbx
            rdi = rbx;
            // 3d26 mov rdx,r14
            rdx = r14;
            // 3d29 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3d2c test BYTE PTR [rbp-0x17],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9)) as u64 & 0x1u64);
            // 3d30 jne 0x3d0c
            if fl.ne() { pc = 0x3d0c; continue; }
            pc = 0x3d32;
        }
        0x3d32 => {
            // block 3d32
            #[cfg(feature = "trace")] eprintln!("f_00003cdc pc 3d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3d32 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 3d34 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 3d38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3d39 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3d3b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3d3c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6858c size=87
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006858c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6858c;
    loop {
        match pc {
        0x6858c => {
            // block 6858c
            #[cfg(feature = "trace")] eprintln!("f_0006858c pc 6858c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6858c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6858d mov rbp,rsp
            rbp = rsp;
            // 68590 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68592 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68593 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 68597 mov r14,rdx
            r14 = rdx;
            // 6859a mov rbx,rsi
            rbx = rsi;
            // 6859d mov DWORD PTR [rbp-0x14],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdi as u32 as u64)) as u32);
            // 685a0 mov DWORD PTR [rbx],0xffffffff
            m.w32(rbx, (0xffffffffu64) as u32);
            // 685a6 mov DWORD PTR [r14],0xffffffff
            m.w32(r14, (0xffffffffu64) as u32);
            // 685ad lea rdi,[rip+0x15ffac] # 0x1c8560
            rdi = m.env.data_va(904493);
            // 685b4 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // 685b8 call 0xebbca
            { let r = { let t = m.env.stub_va(0x397); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 685bd lea rcx,[rip+0x15ffa4] # 0x1c8568
            rcx = m.env.data_va(904501);
            // 685c4 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 685c7 je 0x685d8
            if fl.e() { pc = 0x685d8; continue; }
            pc = 0x685c9;
        }
        0x685c9 => {
            // block 685c9
            #[cfg(feature = "trace")] eprintln!("f_0006858c pc 685c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 685c9 mov ecx,DWORD PTR [rax+0x20]
            rcx = (m.r32(rax.wrapping_add(0x20)) as u64) as u32 as u64;
            // 685cc mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            // 685ce mov eax,DWORD PTR [rax+0x24]
            rax = (m.r32(rax.wrapping_add(0x24)) as u64) as u32 as u64;
            // 685d1 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // 685d4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 685d6 jmp 0x685da
            pc = 0x685da; continue;
        }
        0x685d8 => {
            // block 685d8
            #[cfg(feature = "trace")] eprintln!("f_0006858c pc 685d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 685d8 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x685da;
        }
        0x685da => {
            // block 685da
            #[cfg(feature = "trace")] eprintln!("f_0006858c pc 685da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 685da add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 685de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 685df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 685e1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 685e2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6819c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006819c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6819c;
    loop {
        match pc {
        0x6819c => {
            // block 6819c
            #[cfg(feature = "trace")] eprintln!("f_0006819c pc 6819c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6819c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6819d mov rbp,rsp
            rbp = rsp;
            // 681a0 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 681a2 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 681a5 mov rcx,QWORD PTR [rcx+0xbb8]
            rcx = m.r64(rcx.wrapping_add(0xbb8)) as u64;
            // 681ac mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 681ae lea rsi,[rip+0x99adb] # 0x101c90
            rsi = m.env.data_va(91229);
            // 681b5 mov esi,DWORD PTR [rsi+rdx*4]
            rsi = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 681b8 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 681ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 681bb jmp rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
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
            #[cfg(feature = "trace")] eprintln!("f_00029e48 pc 29e48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029e48 pc 29e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029e48 pc 29e96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29e96 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 29e99 je 0x29ec4
            if fl.e() { pc = 0x29ec4; continue; }
            pc = 0x29e9b;
        }
        0x29e9b => {
            // block 29e9b
            #[cfg(feature = "trace")] eprintln!("f_00029e48 pc 29e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_00029e48 pc 29ec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29ec4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29ec5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x52232 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00052232<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x52232;
    loop {
        match pc {
        0x52232 => {
            // block 52232
            #[cfg(feature = "trace")] eprintln!("f_00052232 pc 52232 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52232 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 52233 mov rbp,rsp
            rbp = rsp;
            // 52236 mov rdx,rsi
            rdx = rsi;
            // 52239 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 5223c mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 52240 mov esi,0x2841
            rsi = (0x2841u64) as u32 as u64;
            // 52245 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 5224a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 5224d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5224e jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
