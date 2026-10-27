//! Mechanically lifted functions (scripts/lift_run.py), module s6a. Each one is checked against the original
//! in the world harness (tests/world_s6a.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe4a46 size=144
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4a46<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x3: u128 = fx[3] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4a46;
    loop {
        match pc {
        0xe4a46 => {
            // block e4a46
            #[cfg(feature = "trace")] eprintln!("f_000e4a46 pc e4a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4a46 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4a47 mov rbp,rsp
            rbp = rsp;
            // e4a4a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4a4b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e4a4c call 0xec200
            { let r = { let t = m.env.stub_va(0x9cd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6]; x1 = m.xcl[1] as u128; x2 = m.xcl[2] as u128; x3 = m.xcl[3] as u128; }
            // e4a51 mov rbx,rax
            rbx = rax;
            // e4a54 call 0xe4ae0
            { let r = { let t = m.env.code_va(0xe4ae0); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6]; x1 = m.xcl[1] as u128; x2 = m.xcl[2] as u128; x3 = m.xcl[3] as u128; }
            // e4a59 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e4a5b test rax,rax
            fl = logic::<64>(rax & rax);
            // e4a5e je 0xe4acc
            if crate::cov::brk(0xe4a5e, fl.e()) { pc = 0xe4acc; continue; }
            pc = 0xe4a60;
        }
        0xe4a60 => {
            // block e4a60
            #[cfg(feature = "trace")] eprintln!("f_000e4a46 pc e4a60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4a60 movq xmm0,rbx
            x0 = (rbx) as u128;
            // e4a65 movdqa xmm1,XMMWORD PTR [rip+0x43863] # 0x1282d0
            x1 = m.r128(m.env.data_va(248477));
            // e4a6d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e4a71 movapd xmm2,XMMWORD PTR [rip+0x43867] # 0x1282e0
            x2 = m.r128(m.env.data_va(248493));
            // e4a79 subpd xmm0,xmm2
            { let a = x0; let b = x2; x0 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // e4a7d haddpd xmm0,xmm0
            x0 = haddpd(x0, x0);
            // e4a81 movq xmm3,rax
            x3 = (rax) as u128;
            // e4a86 punpckldq xmm3,xmm1
            { let a = x3; let b = x1; x3 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // e4a8a subpd xmm3,xmm2
            { let a = x3; let b = x2; x3 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // e4a8e haddpd xmm3,xmm3
            x3 = haddpd(x3, x3);
            // e4a92 divsd xmm0,xmm3
            x0 = set_lo64(x0, (f64_of(x0) / f64_of(x3)).to_bits());
            // e4a96 mulsd xmm0,QWORD PTR [rip+0x43852] # 0x1282f0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(248509)) as u128))).to_bits());
            // e4a9e movsd xmm1,QWORD PTR [rip+0x43852] # 0x1282f8
            x1 = m.r64(m.env.data_va(248517)) as u128;
            // e4aa6 movaps xmm2,xmm0
            x2 = x0;
            // e4aa9 subsd xmm2,xmm1
            x2 = set_lo64(x2, (f64_of(x2) - f64_of(x1)).to_bits());
            // e4aad cvttsd2si rax,xmm2
            rax = cvtt64(f64_of(x2)) as u64;
            // e4ab2 movabs rdx,0x8000000000000000
            rdx = 0x8000_0000_0000_0000u64;
            // e4abc xor rdx,rax
            { let r = rdx ^ rax; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // e4abf cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // e4ac4 ucomisd xmm0,xmm1
            fl = ucomi(f64_of(x0), f64_of(x1));
            // e4ac8 cmovae rcx,rdx
            if fl.ae() { rcx = rdx; }
            pc = 0xe4acc;
        }
        0xe4acc => {
            // block e4acc
            #[cfg(feature = "trace")] eprintln!("f_000e4a46 pc e4acc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4acc mov rax,rcx
            rax = rcx;
            // e4acf add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e4ad3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4ad4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4ad5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4fc6 size=622
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4fc6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x3: u128 = fx[3] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4fc6;
    loop {
        match pc {
        0xe4fc6 => {
            // block e4fc6
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e4fc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4fc6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4fc7 mov rbp,rsp
            rbp = rsp;
            // e4fca push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4fcc push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4fce push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e4fd0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e4fd2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4fd3 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e4fd7 mov r14,rsi
            r14 = rsi;
            // e4fda mov rbx,rdi
            rbx = rdi;
            // e4fdd mov rax,QWORD PTR [rip+0x5b3f4] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e4fe4 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // e4fe7 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // e4feb test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e4fed je 0xe4ff2
            if crate::cov::brk(0xe4fed, fl.e()) { pc = 0xe4ff2; continue; }
            pc = 0xe4fef;
        }
        0xe4fef => {
            // block e4fef
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e4fef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4fef mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            pc = 0xe4ff2;
        }
        0xe4ff2 => {
            // block e4ff2
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e4ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ff2 cmp DWORD PTR [r14],0xffffffff
            fl = sub::<32>(m.r32(r14) as u64, 0xffffffffu64).1;
            // e4ff6 je 0xe5219
            if crate::cov::brk(0xe4ff6, fl.e()) { pc = 0xe5219; continue; }
            pc = 0xe4ffc;
        }
        0xe4ffc => {
            // block e4ffc
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e4ffc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4ffc mov r12d,DWORD PTR [r14+0x4]
            r12 = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // e5000 mov r13d,DWORD PTR [r14+0x8]
            r13 = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // e5004 mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // e5007 shr r15d,0xa
            { let (r, f) = shr::<32>((r15 as u32 as u64), 0xau64, fl); r15 = (r) as u32 as u64; fl = f; }
            // e500b and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e500f mov BYTE PTR [rbx+0x14],r15b
            m.w8(rbx.wrapping_add(0x14), ((r15 as u8 as u64)) as u8);
            // e5013 mov eax,DWORD PTR [rbx+0x8]
            rax = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e5016 lea ecx,[rax-0x5]
            rcx = (rax.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // e5019 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e501c ja 0xe5061
            if crate::cov::brk(0xe501c, fl.a()) { pc = 0xe5061; continue; }
            pc = 0xe501e;
        }
        0xe501e => {
            // block e501e
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e501e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e501e and eax,0xfffffffb
            { let r = (rax as u32 as u64) & 0xfffffffbu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5021 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e5024 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e5027 cmove eax,r13d
            if fl.e() { rax = ((r13 as u32 as u64)) as u32 as u64; } else { rax = rax as u32 as u64; }
            // e502b mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // e502e shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e5031 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5034 lea ecx,[rcx+rcx*4]
            rcx = (rcx.wrapping_add(rcx.wrapping_mul(4))) as u32 as u64;
            // e5037 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // e503a and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e503d lea ecx,[rdx+rcx*2]
            rcx = (rdx.wrapping_add(rcx.wrapping_mul(2))) as u32 as u64;
            // e5040 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e5042 shr eax,0x1b
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1bu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5045 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5048 or eax,ecx
            { let r = (rax as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e504a mov ecx,0xcccccccd
            rcx = (0xcccccccdu64) as u32 as u64;
            // e504f imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e5053 shr rcx,0x23
            { let (r, f) = shr::<64>(rcx, 0x23u64, fl); rcx = r; fl = f; }
            // e5057 lea edx,[rcx+rcx*1]
            rdx = (rcx.wrapping_add(rcx)) as u32 as u64;
            // e505a lea edx,[rdx+rdx*4]
            rdx = (rdx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // e505d sub eax,edx
            { let (r, f) = sub::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e505f jmp 0xe5070
            pc = 0xe5070; continue;
        }
        0xe5061 => {
            // block e5061
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e5061 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5061 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e5064 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e5067 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // e506a shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e506d and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe5070;
        }
        0xe5070 => {
            // block e5070
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e5070 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5070 mov eax,eax
            rax = ((rax as u32 as u64)) as u32 as u64;
            // e5072 mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // e5074 lea rdx,[rip+0x432b5] # 0x128330
            rdx = m.env.data_va(248573);
            // e507b movzx r8d,BYTE PTR [rax+rdx*1]
            r8 = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // e5080 movzx r9d,BYTE PTR [rcx+rdx*1]
            r9 = (m.r8(rcx.wrapping_add(rdx)) as u64) as u32 as u64;
            // e5085 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e5088 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e508b and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e508e movzx r10d,BYTE PTR [rax+rdx*1]
            r10 = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // e5093 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e5096 shr eax,0x18
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x18u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e5099 and eax,0x7
            { let r = (rax as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e509c movzx r11d,BYTE PTR [rax+rdx*1]
            r11 = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // e50a1 mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e50a4 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e50a7 movzx edi,BYTE PTR [rax+rdx*1]
            rdi = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // e50ab mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e50ae shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e50b1 and eax,0x7
            { let r = (rax as u32 as u64) & 0x7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e50b4 movzx eax,BYTE PTR [rax+rdx*1]
            rax = (m.r8(rax.wrapping_add(rdx)) as u64) as u32 as u64;
            // e50b8 mov ecx,r13d
            rcx = ((r13 as u32 as u64)) as u32 as u64;
            // e50bb shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e50be and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e50c1 movzx ecx,BYTE PTR [rcx+rdx*1]
            rcx = (m.r8(rcx.wrapping_add(rdx)) as u64) as u32 as u64;
            // e50c5 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // e50c8 shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e50cb and esi,0x3
            { let r = (rsi as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e50ce movzx edx,BYTE PTR [rsi+rdx*1]
            rdx = (m.r8(rsi.wrapping_add(rdx)) as u64) as u32 as u64;
            // e50d2 pinsrw xmm0,r8d,0x0
            x0 = pinsrw(x0, (r8 as u32 as u64), 0x0);
            // e50d8 pinsrw xmm0,r10d,0x2
            x0 = pinsrw(x0, (r10 as u32 as u64), 0x2);
            // e50de pinsrw xmm0,edi,0x4
            x0 = pinsrw(x0, (rdi as u32 as u64), 0x4);
            // e50e3 pinsrw xmm0,ecx,0x6
            x0 = pinsrw(x0, (rcx as u32 as u64), 0x6);
            // e50e8 pslld xmm0,0x18
            x0 = pslld(x0, 0x18u64);
            // e50ed psrad xmm0,0x18
            x0 = psrad(x0, 0x18u64);
            // e50f2 pinsrw xmm1,r9d,0x0
            x1 = pinsrw(x1, (r9 as u32 as u64), 0x0);
            // e50f8 pinsrw xmm1,r11d,0x2
            x1 = pinsrw(x1, (r11 as u32 as u64), 0x2);
            // e50fe pinsrw xmm1,eax,0x4
            x1 = pinsrw(x1, (rax as u32 as u64), 0x4);
            // e5103 pinsrw xmm1,edx,0x6
            x1 = pinsrw(x1, (rdx as u32 as u64), 0x6);
            // e5108 pslld xmm1,0x18
            x1 = pslld(x1, 0x18u64);
            // e510d psrad xmm1,0x18
            x1 = psrad(x1, 0x18u64);
            // e5112 movdqa xmm2,XMMWORD PTR [rip+0x431e6] # 0x128300
            x2 = m.r128(m.env.data_va(248525));
            // e511a pshufd xmm3,xmm1,0xf5
            x3 = pshufd(x1, 0xf5);
            // e511f pmuludq xmm1,xmm2
            x1 = pmuludq(x1, x2);
            // e5123 pmuludq xmm3,xmm2
            x3 = pmuludq(x3, x2);
            // e5127 shufps xmm1,xmm3,0x88
            x1 = shufps(x1, x3, 0x88);
            // e512b shufps xmm1,xmm1,0xd8
            x1 = shufps(x1, x1, 0xd8);
            // e512f paddd xmm1,xmm0
            x1 = paddd(x1, x0);
            // e5133 paddd xmm1,XMMWORD PTR [rip+0x431d5] # 0x128310
            x1 = paddd(x1, m.r128(m.env.data_va(248541)));
            // e513b movdqu XMMWORD PTR [rbx+0x50],xmm1
            m.w128(rbx.wrapping_add(0x50), x1);
            // e5140 mov BYTE PTR [rbp-0x3c],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rdx as u8 as u64)) as u8);
            // e5143 mov BYTE PTR [rbp-0x3b],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc5), ((rcx as u8 as u64)) as u8);
            // e5146 mov BYTE PTR [rbp-0x3a],0x3a
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc6), (0x3au64) as u8);
            // e514a mov BYTE PTR [rbp-0x39],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc7), ((rax as u8 as u64)) as u8);
            // e514d mov BYTE PTR [rbp-0x38],dil
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rdi as u8 as u64)) as u8);
            // e5151 mov BYTE PTR [rbp-0x37],0x3a
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffc9), (0x3au64) as u8);
            // e5155 mov BYTE PTR [rbp-0x36],r11b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffca), ((r11 as u8 as u64)) as u8);
            // e5159 mov BYTE PTR [rbp-0x35],r10b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcb), ((r10 as u8 as u64)) as u8);
            // e515d mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // e5160 or al,0x3a
            { let r = (rax as u8 as u64) | 0x3au64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e5162 mov BYTE PTR [rbp-0x34],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u8 as u64)) as u8);
            // e5165 mov BYTE PTR [rbp-0x33],r9b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcd), ((r9 as u8 as u64)) as u8);
            // e5169 mov BYTE PTR [rbp-0x32],r8b
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffce), ((r8 as u8 as u64)) as u8);
            // e516d mov BYTE PTR [rbp-0x31],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x0u64) as u8);
            // e5171 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // e5175 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // e5179 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6]; x1 = m.xcl[1] as u128; x2 = m.xcl[2] as u128; x3 = m.xcl[3] as u128; }
            // e517e mov rdi,rbx
            rdi = rbx;
            // e5181 call 0xe5608
            { let r = { let t = m.env.code_va(0xe5608); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6]; x1 = m.xcl[1] as u128; x2 = m.xcl[2] as u128; x3 = m.xcl[3] as u128; }
            // e5186 mov eax,DWORD PTR [r14+0x8]
            rax = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // e518a mov DWORD PTR [rbx+0x88],eax
            m.w32(rbx.wrapping_add(0x88), ((rax as u32 as u64)) as u32);
            // e5190 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e5193 mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // e519a mov BYTE PTR [rbx+0xc],0x1
            m.w8(rbx.wrapping_add(0xc), (0x1u64) as u8);
            // e519e mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // e51a1 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e51a3 test eax,0x60000
            fl = logic::<32>((rax as u32 as u64) & 0x60000u64);
            // e51a8 jne 0xe51b4
            if crate::cov::brk(0xe51a8, fl.ne()) { pc = 0xe51b4; continue; }
            pc = 0xe51aa;
        }
        0xe51aa => {
            // block e51aa
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e51aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e51aa and eax,0x80000
            { let r = (rax as u32 as u64) & 0x80000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e51af shr eax,0x13
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x13u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e51b2 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xe51b4;
        }
        0xe51b4 => {
            // block e51b4
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e51b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e51b4 mov BYTE PTR [rbx+0xd],cl
            m.w8(rbx.wrapping_add(0xd), ((rcx as u8 as u64)) as u8);
            // e51b7 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e51ba shr al,0x5
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x5u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // e51bd and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e51bf mov BYTE PTR [rbx+0xe],al
            m.w8(rbx.wrapping_add(0xe), ((rax as u8 as u64)) as u8);
            // e51c2 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e51c5 shr al,0x4
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x4u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // e51c8 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e51ca mov BYTE PTR [rbx+0xf],al
            m.w8(rbx.wrapping_add(0xf), ((rax as u8 as u64)) as u8);
            // e51cd mov BYTE PTR [rbx+0x14],r15b
            m.w8(rbx.wrapping_add(0x14), ((r15 as u8 as u64)) as u8);
            // e51d1 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e51d4 shr eax,0xb
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xbu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e51d7 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e51d9 mov BYTE PTR [rbx+0x15],al
            m.w8(rbx.wrapping_add(0x15), ((rax as u8 as u64)) as u8);
            // e51dc mov eax,DWORD PTR [rbx+0x6c]
            rax = (m.r32(rbx.wrapping_add(0x6c)) as u64) as u32 as u64;
            // e51df lea eax,[rax+rax*4]
            rax = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e51e2 add eax,eax
            { let (r, f) = add::<32>((rax as u32 as u64), (rax as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e51e4 add eax,DWORD PTR [rbx+0x68]
            { let (r, f) = add::<32>((rax as u32 as u64), m.r32(rbx.wrapping_add(0x68)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e51e7 mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // e51ea mov eax,DWORD PTR [rbx+0x8]
            rax = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e51ed and eax,0xfffffffb
            { let r = (rax as u32 as u64) & 0xfffffffbu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e51f0 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e51f3 jne 0xe5203
            if crate::cov::brk(0xe51f3, fl.ne()) { pc = 0xe5203; continue; }
            pc = 0xe51f5;
        }
        0xe51f5 => {
            // block e51f5
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e51f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e51f5 shr r13d,0x1b
            { let (r, f) = shr::<32>((r13 as u32 as u64), 0x1bu64, fl); r13 = (r) as u32 as u64; fl = f; }
            // e51f9 and r13d,0x1
            { let r = (r13 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e51fd mov DWORD PTR [rbx+0x10],r13d
            m.w32(rbx.wrapping_add(0x10), ((r13 as u32 as u64)) as u32);
            // e5201 jmp 0xe520f
            pc = 0xe520f; continue;
        }
        0xe5203 => {
            // block e5203
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e5203 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5203 shr r12d,0x1b
            { let (r, f) = shr::<32>((r12 as u32 as u64), 0x1bu64, fl); r12 = (r) as u32 as u64; fl = f; }
            // e5207 and r12d,0x1
            { let r = (r12 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e520b mov DWORD PTR [rbx+0x10],r12d
            m.w32(rbx.wrapping_add(0x10), ((r12 as u32 as u64)) as u32);
            pc = 0xe520f;
        }
        0xe520f => {
            // block e520f
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e520f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e520f mov rax,QWORD PTR [rip+0x5b1c2] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // e5216 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            pc = 0xe5219;
        }
        0xe5219 => {
            // block e5219
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e5219 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5219 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e521d jne 0xe522e
            if crate::cov::brk(0xe521d, fl.ne()) { pc = 0xe522e; continue; }
            pc = 0xe521f;
        }
        0xe521f => {
            // block e521f
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e521f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e521f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e5223 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e5224 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e5226 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e5228 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e522a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e522c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e522d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe522e => {
            // block e522e
            #[cfg(feature = "trace")] eprintln!("f_000e4fc6 pc e522e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e522e call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6]; x1 = m.xcl[1] as u128; x2 = m.xcl[2] as u128; x3 = m.xcl[3] as u128; }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f0b2 size=326
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f0b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x2f0b2;
    loop {
        match pc {
        0x2f0b2 => {
            // block 2f0b2
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f0b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f0b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f0b3 mov rbp,rsp
            rbp = rsp;
            // 2f0b6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2f0b8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2f0ba push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f0bb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2f0bc mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2f0bf mov rbx,rdi
            rbx = rdi;
            // 2f0c2 mov BYTE PTR [rbx+0x8],dl
            m.w8(rbx.wrapping_add(0x8), ((rdx as u8 as u64)) as u8);
            // 2f0c5 lea rax,[rip+0x1125c4] # 0x141690
            rax = m.env.data_va(351837);
            // 2f0cc mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2f0cf mov QWORD PTR [rbx+0xc],0x300000
            m.w64(rbx.wrapping_add(0xc), (0x300000u64) as u64);
            // 2f0d7 mov DWORD PTR [rbx+0x14],0x0
            m.w32(rbx.wrapping_add(0x14), (0x0u64) as u32);
            // 2f0de lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // 2f0e2 mov rdi,r14
            rdi = r14;
            // 2f0e5 call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f0ea mov eax,DWORD PTR [rbx+0xc]
            rax = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 2f0ed movd xmm0,eax
            x0 = ((rax as u32 as u64)) as u32 as u128;
            // 2f0f1 pshufd xmm0,xmm0,0x0
            x0 = pshufd(x0, 0x0);
            // 2f0f6 paddd xmm0,XMMWORD PTR [rip+0xd0cc2] # 0xffdc0
            x0 = paddd(x0, m.r128(m.env.data_va(83341)));
            // 2f0fe psrld xmm0,0x2
            x0 = psrld(x0, 0x2u64);
            // 2f103 movdqu XMMWORD PTR [rbx+0x2a0],xmm0
            m.w128(rbx.wrapping_add(0x2a0), x0);
            // 2f10b lea ecx,[rax+0x6c]
            rcx = (rax.wrapping_add(0x6c)) as u32 as u64;
            // 2f10e shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 2f111 mov DWORD PTR [rbx+0x2b0],ecx
            m.w32(rbx.wrapping_add(0x2b0), ((rcx as u32 as u64)) as u32);
            // 2f117 lea ecx,[rax+0x70]
            rcx = (rax.wrapping_add(0x70)) as u32 as u64;
            // 2f11a shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 2f11d mov DWORD PTR [rbx+0x2b4],ecx
            m.w32(rbx.wrapping_add(0x2b4), ((rcx as u32 as u64)) as u32);
            // 2f123 add eax,0x1c
            { let (r, f) = add::<32>((rax as u32 as u64), 0x1cu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2f126 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 2f129 mov DWORD PTR [rbx+0x2b8],eax
            m.w32(rbx.wrapping_add(0x2b8), ((rax as u32 as u64)) as u32);
            // 2f12f mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2f133 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 2f137 movzx esi,r15w
            rsi = ((r15 as u16 as u64)) as u32 as u64;
            // 2f13b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2f13d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 2f13f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2f142 mov rdi,r14
            rdi = r14;
            // 2f145 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f147 mov rdi,rbx
            rdi = rbx;
            // 2f14a call 0x2f1f8
            { let r = { let t = m.env.code_va(0x2f1f8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f14f lea rsi,[rbp-0x19]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe7);
            // 2f153 lea rdx,[rbp-0x1a]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe6);
            // 2f157 lea rcx,[rbp-0x1b]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffe5);
            // 2f15b lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 2f15f lea r9,[rbp-0x1d]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffe3);
            // 2f163 mov rdi,rbx
            rdi = rbx;
            // 2f166 call 0x2f282
            { let r = { let t = m.env.code_va(0x2f282); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f16b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2f16d je 0x2f1bc
            if crate::cov::brk(0x2f16d, fl.e()) { pc = 0x2f1bc; continue; }
            pc = 0x2f16f;
        }
        0x2f16f => {
            // block 2f16f
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f16f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f16f movzx eax,BYTE PTR [rbp-0x1b]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe5)) as u64) as u32 as u64;
            // 2f173 cmp eax,0x18
            fl = sub::<32>((rax as u32 as u64), 0x18u64).1;
            // 2f176 jne 0x2f181
            if crate::cov::brk(0x2f176, fl.ne()) { pc = 0x2f181; continue; }
            pc = 0x2f178;
        }
        0x2f178 => {
            // block 2f178
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f178 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f178 mov DWORD PTR [rbx+0x10],0x1000000
            m.w32(rbx.wrapping_add(0x10), (0x1000000u64) as u32);
            // 2f17f jmp 0x2f196
            pc = 0x2f196; continue;
        }
        0x2f181 => {
            // block 2f181
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f181 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f181 cmp eax,0x19
            fl = sub::<32>((rax as u32 as u64), 0x19u64).1;
            // 2f184 jne 0x2f18f
            if crate::cov::brk(0x2f184, fl.ne()) { pc = 0x2f18f; continue; }
            pc = 0x2f186;
        }
        0x2f186 => {
            // block 2f186
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f186 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f186 mov DWORD PTR [rbx+0x10],0x2000000
            m.w32(rbx.wrapping_add(0x10), (0x2000000u64) as u32);
            // 2f18d jmp 0x2f196
            pc = 0x2f196; continue;
        }
        0x2f18f => {
            // block 2f18f
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f18f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f18f mov DWORD PTR [rbx+0x10],0x0
            m.w32(rbx.wrapping_add(0x10), (0x0u64) as u32);
            pc = 0x2f196;
        }
        0x2f196 => {
            // block 2f196
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f196 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f196 movzx eax,BYTE PTR [rbp-0x1c]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 2f19a cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 2f19d jne 0x2f1a8
            if crate::cov::brk(0x2f19d, fl.ne()) { pc = 0x2f1a8; continue; }
            pc = 0x2f19f;
        }
        0x2f19f => {
            // block 2f19f
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f19f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f19f mov DWORD PTR [rbx+0x14],0x10000
            m.w32(rbx.wrapping_add(0x14), (0x10000u64) as u32);
            // 2f1a6 jmp 0x2f1bc
            pc = 0x2f1bc; continue;
        }
        0x2f1a8 => {
            // block 2f1a8
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1a8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2f1aa jne 0x2f1b5
            if crate::cov::brk(0x2f1aa, fl.ne()) { pc = 0x2f1b5; continue; }
            pc = 0x2f1ac;
        }
        0x2f1ac => {
            // block 2f1ac
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1ac mov DWORD PTR [rbx+0x14],0x40000
            m.w32(rbx.wrapping_add(0x14), (0x40000u64) as u32);
            // 2f1b3 jmp 0x2f1bc
            pc = 0x2f1bc; continue;
        }
        0x2f1b5 => {
            // block 2f1b5
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1b5 mov DWORD PTR [rbx+0x14],0x0
            m.w32(rbx.wrapping_add(0x14), (0x0u64) as u32);
            pc = 0x2f1bc;
        }
        0x2f1bc => {
            // block 2f1bc
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1bc lea rsi,[rbp-0x1e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // 2f1c0 mov rdi,rbx
            rdi = rbx;
            // 2f1c3 call 0x2f47a
            { let r = { let t = m.env.code_va(0x2f47a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f1c8 lea rsi,[rbp-0x1f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe1);
            // 2f1cc mov rdi,rbx
            rdi = rbx;
            // 2f1cf call 0x2f626
            { let r = { let t = m.env.code_va(0x2f626); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f1d4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2f1d8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f1d9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2f1db pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2f1dd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f1de ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2f1df => {
            // block 2f1df
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1df mov rbx,rax
            rbx = rax;
            // 2f1e2 mov rdi,r14
            rdi = r14;
            // 2f1e5 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2f1ea jmp 0x2f1ef
            pc = 0x2f1ef; continue;
        }
        0x2f1ec => {
            // block 2f1ec
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1ec mov rbx,rax
            rbx = rax;
            pc = 0x2f1ef;
        }
        0x2f1ef => {
            // block 2f1ef
            #[cfg(feature = "trace")] eprintln!("f_0002f0b2 pc 2f1ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f1ef mov rdi,rbx
            rdi = rbx;
            // 2f1f2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c77be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            if crate::cov::brk(0xc77d2, fl.e()) { pc = 0xc7952; continue; }
            pc = 0xc77d8;
        }
        0xc77d8 => {
            // block c77d8
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c77d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c77ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c77ef cmp r8d,0x2
            fl = sub::<32>((r8 as u32 as u64), 0x2u64).1;
            // c77f3 je 0xc7802
            if crate::cov::brk(0xc77f3, fl.e()) { pc = 0xc7802; continue; }
            pc = 0xc77f5;
        }
        0xc77f5 => {
            // block c77f5
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c77f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c77f5 cmp r8d,0x3
            fl = sub::<32>((r8 as u32 as u64), 0x3u64).1;
            // c77f9 jne 0xc7813
            if crate::cov::brk(0xc77f9, fl.ne()) { pc = 0xc7813; continue; }
            pc = 0xc77fb;
        }
        0xc77fb => {
            // block c77fb
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c77fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c77fb test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c77fe jne 0xc7807
            if crate::cov::brk(0xc77fe, fl.ne()) { pc = 0xc7807; continue; }
            pc = 0xc7800;
        }
        0xc7800 => {
            // block c7800
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7800 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7800 jmp 0xc7829
            pc = 0xc7829; continue;
        }
        0xc7802 => {
            // block c7802
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7802 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7802 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7805 jne 0xc7829
            if crate::cov::brk(0xc7805, fl.ne()) { pc = 0xc7829; continue; }
            pc = 0xc7807;
        }
        0xc7807 => {
            // block c7807
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7807 lea rcx,[rdi+rax*8+0x2]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c780c lea rdx,[rdi+rax*8+0x4]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            // c7811 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc7813 => {
            // block c7813
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7813 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7813 cmp r8d,0x4
            fl = sub::<32>((r8 as u32 as u64), 0x4u64).1;
            // c7817 jne 0xc782f
            if crate::cov::brk(0xc7817, fl.ne()) { pc = 0xc782f; continue; }
            pc = 0xc7819;
        }
        0xc7819 => {
            // block c7819
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7819 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7819 test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c781c jne 0xc783e
            if crate::cov::brk(0xc781c, fl.ne()) { pc = 0xc783e; continue; }
            pc = 0xc781e;
        }
        0xc781e => {
            // block c781e
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c781e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c781e lea rcx,[rdi+rax*8+0x2]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c7823 lea rdx,[rdi+rax*8]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c7827 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc7829 => {
            // block c7829
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7829 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7829 lea rcx,[rdi+rax*8]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c782d jmp 0xc7843
            pc = 0xc7843; continue;
        }
        0xc782f => {
            // block c782f
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c782f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c782f test r14d,r14d
            fl = logic::<32>((r14 as u32 as u64) & (r14 as u32 as u64));
            // c7832 jne 0xc784a
            if crate::cov::brk(0xc7832, fl.ne()) { pc = 0xc784a; continue; }
            pc = 0xc7834;
        }
        0xc7834 => {
            // block c7834
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7834 lea rcx,[rdi+rax*8+0x4]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            // c7839 mov rdx,r9
            rdx = r9;
            // c783c jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc783e => {
            // block c783e
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c783e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c783e lea rcx,[rdi+rax*8+0x4]
            rcx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4);
            pc = 0xc7843;
        }
        0xc7843 => {
            // block c7843
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7843 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7843 lea rdx,[rdi+rax*8+0x2]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x2);
            // c7848 jmp 0xc7851
            pc = 0xc7851; continue;
        }
        0xc784a => {
            // block c784a
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c784a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c784a lea rdx,[rdi+rax*8]
            rdx = rdi.wrapping_add(rax.wrapping_mul(8));
            // c784e mov rcx,r9
            rcx = r9;
            pc = 0xc7851;
        }
        0xc7851 => {
            // block c7851
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7851 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // c7854 cmp r11d,0x6
            fl = sub::<32>((r11 as u32 as u64), 0x6u64).1;
            // c7858 ja 0xc7940
            if crate::cov::brk(0xc7858, fl.a()) { pc = 0xc7940; continue; }
            pc = 0xc785e;
        }
        0xc785e => {
            // block c785e
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c785e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            pc = (rbx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc7870 => {
            // block c7870
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7870 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7870 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c7872 jne 0xc78ca
            if crate::cov::brk(0xc7872, fl.ne()) { pc = 0xc78ca; continue; }
            pc = 0xc7874;
        }
        0xc7874 => {
            // block c7874
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7886 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7886 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c7888 jne 0xc78e9
            if crate::cov::brk(0xc7888, fl.ne()) { pc = 0xc78e9; continue; }
            pc = 0xc788a;
        }
        0xc788a => {
            // block c788a
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c788a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c78a0 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c78a2 jne 0xc790b
            if crate::cov::brk(0xc78a2, fl.ne()) { pc = 0xc790b; continue; }
            pc = 0xc78a4;
        }
        0xc78a4 => {
            // block c78a4
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c78ba test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // c78bc jne 0xc792d
            if crate::cov::brk(0xc78bc, fl.ne()) { pc = 0xc792d; continue; }
            pc = 0xc78be;
        }
        0xc78be => {
            // block c78be
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c78f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c790b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c791b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c792d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7934 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7940 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xc7942;
        }
        0xc7942 => {
            // block c7942
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c7942 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // c7945 add r9,0x8
            { let (r, f) = add::<64>(r9, 0x8u64);
              r9 = r;
              fl = f; }
            // c7949 cmp ecx,r15d
            fl = sub::<32>((rcx as u32 as u64), (r15 as u32 as u64)).1;
            // c794c jne 0xc77ef
            if crate::cov::brk(0xc794c, fl.ne()) { pc = 0xc77ef; continue; }
            pc = 0xc7952;
        }
        0xc7952 => {
            // block c7952
            #[cfg(feature = "trace")] eprintln!("f_000c77be pc c7952 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xdc0da size=282
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000dc0da<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0xdc0da;
    loop {
        match pc {
        0xdc0da => {
            // block dc0da
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc0da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc0da push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dc0db mov rbp,rsp
            rbp = rsp;
            // dc0de push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dc0e0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dc0e2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dc0e4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dc0e6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dc0e7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // dc0e8 mov r13,rsi
            r13 = rsi;
            // dc0eb mov r14,rdi
            r14 = rdi;
            // dc0ee mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc0f2 mov r15,QWORD PTR [r14+0x10]
            r15 = m.r64(r14.wrapping_add(0x10)) as u64;
            // dc0f6 mov rax,r15
            rax = r15;
            // dc0f9 sub rax,rbx
            { let (r, f) = sub::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // dc0fc cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // dc0ff jae 0xdc14d
            if crate::cov::brk(0xdc0ff, fl.ae()) { pc = 0xdc14d; continue; }
            pc = 0xdc101;
        }
        0xdc101 => {
            // block dc101
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc101 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc101 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // dc104 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // dc107 add rbx,r13
            { let (r, f) = add::<64>(rbx, r13);
              rbx = r;
              fl = f; }
            // dc10a jns 0xdc11b
            if crate::cov::brk(0xdc10a, fl.ns()) { pc = 0xdc11b; continue; }
            pc = 0xdc10c;
        }
        0xdc10c => {
            // block dc10c
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc10c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc10c mov rdi,r14
            rdi = r14;
            // dc10f call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xdc11b => {
            // block dc11b
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc11b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc11b sub r15,rax
            { let (r, f) = sub::<64>(r15, rax);
              r15 = r;
              fl = f; }
            // dc11e movabs rcx,0x3fffffffffffffff
            rcx = 0x3fff_ffff_ffff_ffffu64;
            // dc128 cmp r15,rcx
            fl = sub::<64>(r15, rcx).1;
            // dc12b jae 0xdc165
            if crate::cov::brk(0xdc12b, fl.ae()) { pc = 0xdc165; continue; }
            pc = 0xdc12d;
        }
        0xdc12d => {
            // block dc12d
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc12d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc12d add r15,r15
            { let (r, f) = add::<64>(r15, r15);
              r15 = r;
              fl = f; }
            // dc130 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // dc133 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // dc137 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc13b sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // dc13e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // dc140 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dc143 mov r12d,0x0
            r12 = (0x0u64) as u32 as u64;
            // dc149 jne 0xdc176
            if crate::cov::brk(0xdc149, fl.ne()) { pc = 0xdc176; continue; }
            pc = 0xdc14b;
        }
        0xdc14b => {
            // block dc14b
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc14b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc14b jmp 0xdc18a
            pc = 0xdc18a; continue;
        }
        0xdc14d => {
            // block dc14d
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc14d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc14d mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // dc150 mov rbx,QWORD PTR [r14+0x8]
            rbx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc154 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dc157 mov QWORD PTR [r14+0x8],rbx
            m.w64(r14.wrapping_add(0x8), (rbx) as u64);
            // dc15b dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // dc15e jne 0xdc14d
            if crate::cov::brk(0xdc15e, fl.ne()) { pc = 0xdc14d; continue; }
            pc = 0xdc160;
        }
        0xdc160 => {
            // block dc160
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc160 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc160 jmp 0xdc1e5
            pc = 0xdc1e5; continue;
        }
        0xdc165 => {
            // block dc165
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc165 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc165 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc169 sub rcx,rax
            { let (r, f) = sub::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // dc16c movabs r15,0x7fffffffffffffff
            r15 = 0x7fff_ffff_ffff_ffffu64;
            pc = 0xdc176;
        }
        0xdc176 => {
            // block dc176
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc176 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc176 mov rdi,r15
            rdi = r15;
            // dc179 mov rbx,rcx
            rbx = rcx;
            // dc17c call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc181 mov rcx,rbx
            rcx = rbx;
            // dc184 mov r12,rax
            r12 = rax;
            // dc187 mov rax,r15
            rax = r15;
            pc = 0xdc18a;
        }
        0xdc18a => {
            // block dc18a
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc18a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc18a lea rbx,[r12+rcx*1]
            rbx = r12.wrapping_add(rcx);
            // dc18e add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            // dc191 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            pc = 0xdc195;
        }
        0xdc195 => {
            // block dc195
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc195 mov BYTE PTR [rbx],0x0
            m.w8(rbx, (0x0u64) as u8);
            // dc198 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dc19b dec r13
            { let (r, f) = dec::<64>(r13, fl); r13 = r; fl = f; }
            // dc19e jne 0xdc195
            if crate::cov::brk(0xdc19e, fl.ne()) { pc = 0xdc195; continue; }
            pc = 0xdc1a0;
        }
        0xdc1a0 => {
            // block dc1a0
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc1a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc1a0 mov r15,QWORD PTR [r14]
            r15 = m.r64(r14) as u64;
            // dc1a3 mov rdx,QWORD PTR [r14+0x8]
            rdx = m.r64(r14.wrapping_add(0x8)) as u64;
            // dc1a7 sub rdx,r15
            { let (r, f) = sub::<64>(rdx, r15);
              rdx = r;
              fl = f; }
            // dc1aa sub rcx,rdx
            { let (r, f) = sub::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // dc1ad add r12,rcx
            { let (r, f) = add::<64>(r12, rcx);
              r12 = r;
              fl = f; }
            // dc1b0 mov rdi,r12
            rdi = r12;
            // dc1b3 mov rsi,r15
            rsi = r15;
            // dc1b6 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // dc1bb mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // dc1be mov QWORD PTR [r14+0x8],rbx
            m.w64(r14.wrapping_add(0x8), (rbx) as u64);
            // dc1c2 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // dc1c6 mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // dc1ca test r15,r15
            fl = logic::<64>(r15 & r15);
            // dc1cd je 0xdc1e5
            if crate::cov::brk(0xdc1cd, fl.e()) { pc = 0xdc1e5; continue; }
            pc = 0xdc1cf;
        }
        0xdc1cf => {
            // block dc1cf
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc1cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc1cf mov rdi,r15
            rdi = r15;
            // dc1d2 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc1d6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc1d7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc1d9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc1db pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc1dd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc1df pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc1e0 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xdc1e5 => {
            // block dc1e5
            #[cfg(feature = "trace")] eprintln!("f_000dc0da pc dc1e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dc1e5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // dc1e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dc1ea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dc1ec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dc1ee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dc1f0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dc1f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dc1f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11c65 size=451
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_00011c65_rec)
pub fn f_00011c65<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_00011c65_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_00011c65_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11c65;
    loop {
        match pc {
        0x11c65 => {
            // block 11c65
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11c65 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c65 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11c66 mov rbp,rsp
            rbp = rsp;
            // 11c69 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11c6b push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11c6d push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11c6f push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11c71 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11c72 sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 11c76 mov r15,r9
            r15 = r9;
            // 11c79 mov rbx,r8
            rbx = r8;
            // 11c7c mov r14,rcx
            r14 = rcx;
            // 11c7f mov r13,rsi
            r13 = rsi;
            // 11c82 mov rcx,rdi
            rcx = rdi;
            // 11c85 cmp r14,0x2
            fl = sub::<64>(r14, 0x2u64).1;
            // 11c89 jb 0x11de0
            if crate::cov::brk(0x11c89, fl.b()) { pc = 0x11de0; continue; }
            pc = 0x11c8f;
        }
        0x11c8f => {
            // block 11c8f
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11c8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c8f jne 0x11cd6
            if crate::cov::brk(0x11c8f, fl.ne()) { pc = 0x11cd6; continue; }
            pc = 0x11c91;
        }
        0x11c91 => {
            // block 11c91
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11c91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c91 mov eax,DWORD PTR [r13-0x20]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // 11c95 cmp eax,DWORD PTR [rcx+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x18)) as u64).1;
            // 11c98 jge 0x11de0
            if crate::cov::brk(0x11c98, fl.ge()) { pc = 0x11de0; continue; }
            pc = 0x11c9e;
        }
        0x11c9e => {
            // block 11c9e
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11c9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11c9e add r13,0xffffffffffffffc8
            { let (r, f) = add::<64>(r13, 0xffff_ffff_ffff_ffc8u64);
              r13 = r;
              fl = f; }
            // 11ca2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11ca6 mov rsi,rcx
            rsi = rcx;
            // 11ca9 mov rbx,rcx
            rbx = rcx;
            // 11cac call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11cb1 mov rdi,rbx
            rdi = rbx;
            // 11cb4 mov rsi,r13
            rsi = r13;
            // 11cb7 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11cbc lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11cc0 mov rdi,r13
            rdi = r13;
            // 11cc3 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11cc8 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11ccc call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11cd1 jmp 0x11de0
            pc = 0x11de0; continue;
        }
        0x11cd6 => {
            // block 11cd6
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11cd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11cd6 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 11cd9 jle 0x11d56
            if crate::cov::brk(0x11cd9, fl.le()) { pc = 0x11d56; continue; }
            pc = 0x11cdb;
        }
        0x11cdb => {
            // block 11cdb
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11cdb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11cdb mov r12,r14
            r12 = r14;
            // 11cde shr r12,0x3f
            { let (r, f) = shr::<64>(r12, 0x3fu64, fl); r12 = r; fl = f; }
            // 11ce2 add r12,r14
            { let (r, f) = add::<64>(r12, r14);
              r12 = r;
              fl = f; }
            // 11ce5 sar r12,1
            { let (r, f) = sar::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // 11ce8 imul rax,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rax = r; fl = f; }
            // 11cec lea rsi,[rcx+rax*1]
            rsi = rcx.wrapping_add(rax);
            // 11cf0 cmp r14,r15
            fl = sub::<64>(r14, r15).1;
            // 11cf3 jle 0x11d6f
            if crate::cov::brk(0x11cf3, fl.le()) { pc = 0x11d6f; continue; }
            pc = 0x11cf5;
        }
        0x11cf5 => {
            // block 11cf5
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11cf5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11cf5 mov rdi,rcx
            rdi = rcx;
            // 11cf8 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // 11cfc mov QWORD PTR [rbp-0x70],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdx) as u64);
            // 11d00 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 11d04 mov rcx,r12
            rcx = r12;
            // 11d07 mov r8,rbx
            r8 = rbx;
            // 11d0a mov r9,r15
            r9 = r15;
            // 11d0d call 0x11c65
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x10011d12); let r = f_00011c65_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11d12 sub r14,r12
            { let (r, f) = sub::<64>(r14, r12);
              r14 = r;
              fl = f; }
            // 11d15 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 11d19 mov rsi,r13
            rsi = r13;
            // 11d1c mov rdx,QWORD PTR [rbp-0x70]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 11d20 mov rcx,r14
            rcx = r14;
            // 11d23 mov r8,rbx
            r8 = rbx;
            // 11d26 mov r9,r15
            r9 = r15;
            // 11d29 call 0x11c65
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x10011d2e); let r = f_00011c65_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11d2e mov QWORD PTR [rsp+0x8],r15
            m.w64(rsp.wrapping_add(0x8), (r15) as u64);
            // 11d33 mov QWORD PTR [rsp],rbx
            m.w64(rsp, (rbx) as u64);
            // 11d37 mov rdi,QWORD PTR [rbp-0x78]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 11d3b mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 11d3f mov rdx,r13
            rdx = r13;
            // 11d42 mov rcx,QWORD PTR [rbp-0x70]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 11d46 mov r8,r12
            r8 = r12;
            // 11d49 mov r9,r14
            r9 = r14;
            // 11d4c call 0xeb8b2
            { let r = { let t = m.env.stub_va(0x7f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11d51 jmp 0x11de0
            pc = 0x11de0; continue;
        }
        0x11d56 => {
            // block 11d56
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11d56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11d56 mov rdi,rcx
            rdi = rcx;
            // 11d59 mov rsi,r13
            rsi = r13;
            // 11d5c add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 11d60 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11d61 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11d63 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11d65 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11d67 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11d69 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11d6a jmp 0xeb8b8
            { let r = { let t = m.env.stub_va(0x85); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0x11d6f => {
            // block 11d6f
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11d6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11d6f mov r15,rax
            r15 = rax;
            // 11d72 mov rdi,rcx
            rdi = rcx;
            // 11d75 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 11d79 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // 11d7d mov QWORD PTR [rbp-0x70],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rdx) as u64);
            // 11d81 mov rcx,r12
            rcx = r12;
            // 11d84 mov r8,rbx
            r8 = rbx;
            // 11d87 call 0xeb8c4
            { let r = { let t = m.env.stub_va(0x91); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11d8c mov rcx,r14
            rcx = r14;
            // 11d8f sub rcx,r12
            { let (r, f) = sub::<64>(rcx, r12);
              rcx = r;
              fl = f; }
            // 11d92 add r15,rbx
            { let (r, f) = add::<64>(r15, rbx);
              r15 = r;
              fl = f; }
            // 11d95 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 11d99 mov rsi,r13
            rsi = r13;
            // 11d9c mov r13,QWORD PTR [rbp-0x70]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 11da0 mov rdx,r13
            rdx = r13;
            // 11da3 mov r8,r15
            r8 = r15;
            // 11da6 call 0xeb8c4
            { let r = { let t = m.env.stub_va(0x91); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11dab imul rcx,r14,0x38
            { let (r, f) = imul::<64>(r14, 0x38u64); rcx = r; fl = f; }
            // 11daf mov r12,r14
            r12 = r14;
            // 11db2 add rcx,rbx
            { let (r, f) = add::<64>(rcx, rbx);
              rcx = r;
              fl = f; }
            // 11db5 mov rdi,rbx
            rdi = rbx;
            // 11db8 mov rsi,r15
            rsi = r15;
            // 11dbb mov rdx,r15
            rdx = r15;
            // 11dbe mov r8,QWORD PTR [rbp-0x78]
            r8 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 11dc2 mov r9,r13
            r9 = r13;
            // 11dc5 call 0xeb8ca
            { let r = { let t = m.env.stub_va(0x97); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11dca test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 11dcd je 0x11de0
            if crate::cov::brk(0x11dcd, fl.e()) { pc = 0x11de0; continue; }
            pc = 0x11dcf;
        }
        0x11dcf => {
            // block 11dcf
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11dcf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11dcf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 11dd2 mov rdi,rbx
            rdi = rbx;
            // 11dd5 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11dd7 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 11ddb dec r14
            { let (r, f) = dec::<64>(r14, fl); r14 = r; fl = f; }
            // 11dde jne 0x11dcf
            if crate::cov::brk(0x11dde, fl.ne()) { pc = 0x11dcf; continue; }
            pc = 0x11de0;
        }
        0x11de0 => {
            // block 11de0
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11de0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11de0 add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 11de4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11de5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11de7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11de9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11deb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11ded pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11dee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11def => {
            // block 11def
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11def mov r14,rax
            r14 = rax;
            // 11df2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11df6 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11dfb jmp 0x11e20
            pc = 0x11e20; continue;
        }
        0x11dfd => {
            // block 11dfd
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11dfd mov r14,rax
            r14 = rax;
            // 11e00 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 11e03 je 0x11e20
            if crate::cov::brk(0x11e03, fl.e()) { pc = 0x11e20; continue; }
            pc = 0x11e05;
        }
        0x11e05 => {
            // block 11e05
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11e05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e05 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 11e08 je 0x11e20
            if crate::cov::brk(0x11e08, fl.e()) { pc = 0x11e20; continue; }
            pc = 0x11e0a;
        }
        0x11e0a => {
            // block 11e0a
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e0a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 11e0d mov rdi,rbx
            rdi = rbx;
            // 11e10 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11e12 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 11e16 dec r12
            { let (r, f) = dec::<64>(r12, fl); r12 = r; fl = f; }
            // 11e19 jne 0x11e0a
            if crate::cov::brk(0x11e19, fl.ne()) { pc = 0x11e0a; continue; }
            pc = 0x11e1b;
        }
        0x11e1b => {
            // block 11e1b
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e1b jmp 0x11e20
            pc = 0x11e20; continue;
        }
        0x11e1d => {
            // block 11e1d
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11e1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e1d mov r14,rax
            r14 = rax;
            pc = 0x11e20;
        }
        0x11e20 => {
            // block 11e20
            #[cfg(feature = "trace")] eprintln!("f_00011c65 pc 11e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e20 mov rdi,r14
            rdi = r14;
            // 11e23 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d31f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d327b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d327d mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3282 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3284 je 0xd32a1
            if crate::cov::brk(0xd3284, fl.e()) { pc = 0xd32a1; continue; }
            pc = 0xd3286;
        }
        0xd3286 => {
            // block d3286
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d3286 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3286 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3289 jne 0xd3292
            if crate::cov::brk(0xd3289, fl.ne()) { pc = 0xd3292; continue; }
            pc = 0xd328b;
        }
        0xd328b => {
            // block d328b
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d328b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d328b mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d3290 jmp 0xd32a1
            pc = 0xd32a1; continue;
        }
        0xd3292 => {
            // block d3292
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d3292 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd32a1;
        }
        0xd32a1 => {
            // block d32a1
            #[cfg(feature = "trace")] eprintln!("f_000d31f1 pc d32a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d32af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.stub_va(0xa51); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d333a mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d333c mov eax,0xfffffffe
            rax = (0xfffffffeu64) as u32 as u64;
            // d3341 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3343 je 0xd3361
            if crate::cov::brk(0xd3343, fl.e()) { pc = 0xd3361; continue; }
            pc = 0xd3345;
        }
        0xd3345 => {
            // block d3345
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3345 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3345 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // d3348 jne 0xd3351
            if crate::cov::brk(0xd3348, fl.ne()) { pc = 0xd3351; continue; }
            pc = 0xd334a;
        }
        0xd334a => {
            // block d334a
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d334a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d334a mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // d334f jmp 0xd3361
            pc = 0xd3361; continue;
        }
        0xd3351 => {
            // block d3351
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3351 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.stub_va(0xa45); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd3361;
        }
        0xd3361 => {
            // block d3361
            #[cfg(feature = "trace")] eprintln!("f_000d32af pc d3361 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
