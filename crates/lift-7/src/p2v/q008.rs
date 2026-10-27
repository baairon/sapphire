//! Mechanically lifted functions (scripts/lift_run.py), module q008. Each one is checked against the original
//! in the world harness (tests/world_q008.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe6324 size=216
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x961
pub fn f_000e6324<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe6324;
    loop {
        match pc {
        0xe6324 => {
            // block e6324
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e6324 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6324 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6325 mov rbp,rsp
            rbp = rsp;
            // e6328 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6329 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e632d mov rbx,QWORD PTR [rip+0x5a0a4] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e6334 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e6337 mov QWORD PTR [rbp-0x10],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (rbx) as u64);
            // e633b mov ecx,DWORD PTR [rdi+0x88]
            rcx = (m.r32(rdi.wrapping_add(0x88)) as u64) as u32 as u64;
            // e6341 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6343 shr edx,0x1c
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x1cu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e6346 mov DWORD PTR [rdi+0x7c],edx
            m.w32(rdi.wrapping_add(0x7c), ((rdx as u32 as u64)) as u32);
            // e6349 lea rax,[rip+0x41ff0] # 0x128340
            rax = m.env.data_va(248589);
            // e6350 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6353 mov BYTE PTR [rbp-0x1c],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u8 as u64)) as u8);
            // e6356 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6358 shr edx,0x14
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e635b and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e635e mov DWORD PTR [rdi+0x78],edx
            m.w32(rdi.wrapping_add(0x78), ((rdx as u32 as u64)) as u32);
            // e6361 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6364 mov BYTE PTR [rbp-0x1b],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe5), ((rdx as u8 as u64)) as u8);
            // e6367 mov BYTE PTR [rbp-0x1a],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe6), (0x20u64) as u8);
            // e636b mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e636d shr edx,0xc
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e6370 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e6373 mov DWORD PTR [rdi+0x74],edx
            m.w32(rdi.wrapping_add(0x74), ((rdx as u32 as u64)) as u32);
            // e6376 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6379 mov BYTE PTR [rbp-0x19],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe7), ((rdx as u8 as u64)) as u8);
            // e637c shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e637f and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e6382 mov DWORD PTR [rdi+0x70],ecx
            m.w32(rdi.wrapping_add(0x70), ((rcx as u32 as u64)) as u32);
            // e6385 mov cl,BYTE PTR [rcx+rax*1]
            rcx = (rcx & !0xff) | ((m.r8(rcx.wrapping_add(rax)) as u64) as u8 as u64);
            // e6388 mov BYTE PTR [rbp-0x18],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rcx as u8 as u64)) as u8);
            // e638b mov BYTE PTR [rbp-0x17],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffe9), (0x20u64) as u8);
            // e638f mov ecx,DWORD PTR [rdi+0x84]
            rcx = (m.r32(rdi.wrapping_add(0x84)) as u64) as u32 as u64;
            // e6395 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e6397 shr edx,0x1c
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x1cu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e639a mov DWORD PTR [rdi+0x6c],edx
            m.w32(rdi.wrapping_add(0x6c), ((rdx as u32 as u64)) as u32);
            // e639d mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63a0 mov BYTE PTR [rbp-0x16],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffea), ((rdx as u8 as u64)) as u8);
            // e63a3 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e63a5 shr edx,0x14
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x14u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e63a8 and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e63ab mov DWORD PTR [rdi+0x68],edx
            m.w32(rdi.wrapping_add(0x68), ((rdx as u32 as u64)) as u32);
            // e63ae mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63b1 mov BYTE PTR [rbp-0x15],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffeb), ((rdx as u8 as u64)) as u8);
            // e63b4 mov BYTE PTR [rbp-0x14],0x20
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x20u64) as u8);
            // e63b8 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // e63ba shr edx,0xc
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e63bd and edx,0xf
            { let r = (rdx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e63c0 mov DWORD PTR [rdi+0x64],edx
            m.w32(rdi.wrapping_add(0x64), ((rdx as u32 as u64)) as u32);
            // e63c3 mov dl,BYTE PTR [rdx+rax*1]
            rdx = (rdx & !0xff) | ((m.r8(rdx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63c6 mov BYTE PTR [rbp-0x13],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffed), ((rdx as u8 as u64)) as u8);
            // e63c9 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e63cc and ecx,0xf
            { let r = (rcx as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e63cf mov DWORD PTR [rdi+0x60],ecx
            m.w32(rdi.wrapping_add(0x60), ((rcx as u32 as u64)) as u32);
            // e63d2 mov al,BYTE PTR [rcx+rax*1]
            rax = (rax & !0xff) | ((m.r8(rcx.wrapping_add(rax)) as u64) as u8 as u64);
            // e63d5 mov BYTE PTR [rbp-0x12],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), ((rax as u8 as u64)) as u8);
            // e63d8 mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // e63dc add rdi,0x38
            { let (r, f) = add::<64>(rdi, 0x38u64);
              rdi = r;
              fl = f; }
            // e63e0 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e63e4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e63e9 cmp rbx,QWORD PTR [rbp-0x10]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64).1;
            // e63ed jne 0xe63f6
            if fl.ne() { pc = 0xe63f6; continue; }
            pc = 0xe63ef;
        }
        0xe63ef => {
            // block e63ef
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e63ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e63ef add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // e63f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e63f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e63f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe63f6 => {
            // block e63f6
            #[cfg(feature = "trace")] eprintln!("f_000e6324 pc e63f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e63f6 call 0xec194
            { let _ = { let t = m.env.stub_va(0x961); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0828 size=208
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0828<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0828;
    loop {
        match pc {
        0xe0828 => {
            // block e0828
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0828 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0828 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0829 mov rbp,rsp
            rbp = rsp;
            // e082c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e082e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0830 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e0832 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e0834 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0835 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0836 mov r14,rsi
            r14 = rsi;
            // e0839 mov rbx,rdi
            rbx = rdi;
            // e083c cmp QWORD PTR [rbx+0x28],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x28)) as u64, 0x0u64).1;
            // e0841 jne 0xe08da
            if fl.ne() { pc = 0xe08da; continue; }
            pc = 0xe0847;
        }
        0xe0847 => {
            // block e0847
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0847 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0847 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e084a mov rdi,rbx
            rdi = rbx;
            // e084d call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0850 mov r15,rax
            r15 = rax;
            // e0853 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // e0857 sub rcx,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rcx, m.r64(r15) as u64);
              rcx = r;
              fl = f; }
            // e085a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // e085e movabs rax,0x6db6db6db6db6db7
            rax = 0x6db6_db6d_b6db_6db7u64;
            // e0868 imul rax,rcx
            { let (r, f) = imul::<64>(rax, rcx); rax = r; fl = f; }
            // e086c mov DWORD PTR [rbx+0x30],eax
            m.w32(rbx.wrapping_add(0x30), ((rax as u32 as u64)) as u32);
            // e086f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0871 jle 0xe08da
            if fl.le() { pc = 0xe08da; continue; }
            pc = 0xe0873;
        }
        0xe0873 => {
            // block e0873
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e0873 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0873 mov r13,rax
            r13 = rax;
            // e0876 shl r13,0x20
            { let (r, f) = shl::<64>(r13, 0x20u64, fl); r13 = r; fl = f; }
            // e087a movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // e087d mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // e0882 mov rax,r12
            rax = r12;
            // e0885 mul rcx
            { let (lo, hi, f) = mul1::<64>(rax, rcx); rax = lo; rdx = hi; fl = f; }
            // e0888 mov rdi,0xffffffffffffffff
            rdi = 0xffff_ffff_ffff_ffffu64;
            // e088f cmovno rdi,rax
            if fl.no() { rdi = rax; }
            // e0893 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0898 mov QWORD PTR [rbx+0x28],rax
            m.w64(rbx.wrapping_add(0x28), (rax) as u64);
            // e089c mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // e089f mov rcx,QWORD PTR [rcx+0x10]
            rcx = m.r64(rcx.wrapping_add(0x10)) as u64;
            // e08a3 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // e08a6 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e08ab mov ecx,0x9
            rcx = (0x9u64) as u32 as u64;
            // e08b0 movabs rdx,0x100000001
            rdx = 0x0_0001_0000_0001u64;
            // e08ba cmp r13,rdx
            fl = sub::<64>(r13, rdx).1;
            // e08bd jl 0xe08da
            if fl.l() { pc = 0xe08da; continue; }
            pc = 0xe08bf;
        }
        0xe08bf => {
            // block e08bf
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08bf mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            // e08c3 mov rsi,QWORD PTR [r15]
            rsi = m.r64(r15) as u64;
            // e08c6 mov rsi,QWORD PTR [rsi+rcx*8]
            rsi = m.r64(rsi.wrapping_add(rcx.wrapping_mul(8))) as u64;
            // e08ca mov QWORD PTR [rdx+rax*8],rsi
            m.w64(rdx.wrapping_add(rax.wrapping_mul(8)), (rsi) as u64);
            // e08ce inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e08d1 add rcx,0x7
            { let (r, f) = add::<64>(rcx, 0x7u64);
              rcx = r;
              fl = f; }
            // e08d5 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // e08d8 jl 0xe08bf
            if fl.l() { pc = 0xe08bf; continue; }
            pc = 0xe08da;
        }
        0xe08da => {
            // block e08da
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08da test r14,r14
            fl = logic::<64>(r14 & r14);
            // e08dd je 0xe08e6
            if fl.e() { pc = 0xe08e6; continue; }
            pc = 0xe08df;
        }
        0xe08df => {
            // block e08df
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08df mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // e08e3 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0xe08e6;
        }
        0xe08e6 => {
            // block e08e6
            #[cfg(feature = "trace")] eprintln!("f_000e0828 pc e08e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e08e6 mov eax,DWORD PTR [rbx+0x30]
            rax = (m.r32(rbx.wrapping_add(0x30)) as u64) as u32 as u64;
            // e08e9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e08ed pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e08ee pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e08f0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e08f2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e08f4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e08f6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e08f7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4403e size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x337); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x337); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 440f7 mov rdi,rbx
            rdi = rbx;
            // 440fa call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x315f0 size=192
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000315f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x315f0;
    loop {
        match pc {
        0x315f0 => {
            // block 315f0
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 315f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 315f1 mov rbp,rsp
            rbp = rsp;
            // 315f4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 315f6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 315f8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 315fa push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 315fc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 315fd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 315fe mov r13,rdi
            r13 = rdi;
            // 31601 lea r14,[r13+0x18]
            r14 = r13.wrapping_add(0x18);
            // 31605 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31609 mov rdi,r14
            rdi = r14;
            // 3160c call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31612 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31614 je 0x3168f
            if fl.e() { pc = 0x3168f; continue; }
            pc = 0x31616;
        }
        0x31616 => {
            // block 31616
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31616 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31616 mov rdi,r14
            rdi = r14;
            // 31619 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3161e and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31621 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31623 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31628 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // 3162e jne 0x31692
            if fl.ne() { pc = 0x31692; continue; }
            pc = 0x31630;
        }
        0x31630 => {
            // block 31630
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31630 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31630 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 31637 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 3163b mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 31642 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 31646 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3164b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3164e mov rdi,r14
            rdi = r14;
            // 31651 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31654 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 31657 test BYTE PTR [rbp-0x2c],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x8u64);
            // 3165b jne 0x31692
            if fl.ne() { pc = 0x31692; continue; }
            pc = 0x3165d;
        }
        0x3165d => {
            // block 3165d
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3165d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3165d lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x31661;
        }
        0x31661 => {
            // block 31661
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31661 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31661 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31663 cmp ebx,0x2710
            fl = sub::<32>((rbx as u32 as u64), 0x2710u64).1;
            // 31669 ja 0x3168f
            if fl.a() { pc = 0x3168f; continue; }
            pc = 0x3166b;
        }
        0x3166b => {
            // block 3166b
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3166b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3166b mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 3166f mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 31676 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3167b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3167e mov rdi,r14
            rdi = r14;
            // 31681 mov rdx,r12
            rdx = r12;
            // 31684 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31687 test BYTE PTR [rbp-0x2c],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x8u64);
            // 3168b je 0x31661
            if fl.e() { pc = 0x31661; continue; }
            pc = 0x3168d;
        }
        0x3168d => {
            // block 3168d
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3168d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3168d jmp 0x31692
            pc = 0x31692; continue;
        }
        0x3168f => {
            // block 3168f
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 3168f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3168f xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x31692;
        }
        0x31692 => {
            // block 31692
            #[cfg(feature = "trace")] eprintln!("f_000315f0 pc 31692 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31692 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 31695 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 31699 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3169a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3169c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 3169e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 316a0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 316a2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 316a3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x136d0 size=186
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000136d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x136d0;
    loop {
        match pc {
        0x136d0 => {
            // block 136d0
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 136d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 136d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 136d1 mov rbp,rsp
            rbp = rsp;
            // 136d4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 136d6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 136d8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 136da push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 136dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 136dd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 136de mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 136e0 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 136e4 mov r13,rdi
            r13 = rdi;
            // 136e7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 136eb call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 136ee mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // 136f1 cmp BYTE PTR [r13+0x2e8],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x2e8)) as u64, 0x0u64).1;
            // 136f9 mov r14d,ebx
            r14 = ((rbx as u32 as u64)) as u32 as u64;
            // 136fc mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 13701 je 0x1377a
            if fl.e() { pc = 0x1377a; continue; }
            pc = 0x13703;
        }
        0x13703 => {
            // block 13703
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13703 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13703 lea rbx,[r13+0x8]
            rbx = r13.wrapping_add(0x8);
            // 13707 movsxd rsi,DWORD PTR [r13+0x260]
            rsi = sx::<32>(m.r32(r13.wrapping_add(0x260)) as u64) as u64;
            // 1370e xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 13711 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 13713 mov rdi,rbx
            rdi = rbx;
            // 13716 call 0xec044
            { let r = { let t = m.env.stub_va(0x811); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1371b mov esi,DWORD PTR [r13+0x2ec]
            rsi = (m.r32(r13.wrapping_add(0x2ec)) as u64) as u32 as u64;
            pc = 0x13722;
        }
        0x13722 => {
            // block 13722
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13722 cmp esi,r12d
            fl = sub::<32>((rsi as u32 as u64), (r12 as u32 as u64)).1;
            // 13725 jae 0x13762
            if fl.ae() { pc = 0x13762; continue; }
            pc = 0x13727;
        }
        0x13727 => {
            // block 13727
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13727 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13727 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1372a mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 1372e test BYTE PTR [rax+rbx*1+0x20],0x2
            fl = logic::<8>(m.r8(rax.wrapping_add(rbx).wrapping_add(0x20)) as u64 & 0x2u64);
            // 13733 jne 0x13767
            if fl.ne() { pc = 0x13767; continue; }
            pc = 0x13735;
        }
        0x13735 => {
            // block 13735
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13735 mov rdi,rbx
            rdi = rbx;
            // 13738 call 0xec03e
            { let r = { let t = m.env.stub_va(0x80b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1373d mov rcx,QWORD PTR [rbp-0x30]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 13741 mov BYTE PTR [rcx+r15*1],al
            m.w8(rcx.wrapping_add(r15), ((rax as u8 as u64)) as u8);
            // 13745 mov esi,DWORD PTR [r13+0x2ec]
            rsi = (m.r32(r13.wrapping_add(0x2ec)) as u64) as u32 as u64;
            // 1374c inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 1374e mov DWORD PTR [r13+0x2ec],esi
            m.w32(r13.wrapping_add(0x2ec), ((rsi as u32 as u64)) as u32);
            // 13755 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 13758 cmp r14d,r15d
            fl = sub::<32>((r14 as u32 as u64), (r15 as u32 as u64)).1;
            // 1375b jne 0x13722
            if fl.ne() { pc = 0x13722; continue; }
            pc = 0x1375d;
        }
        0x1375d => {
            // block 1375d
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 1375d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1375d mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 13760 jmp 0x1377a
            pc = 0x1377a; continue;
        }
        0x13762 => {
            // block 13762
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13762 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 13765 jmp 0x1377a
            pc = 0x1377a; continue;
        }
        0x13767 => {
            // block 13767
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 13767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 13767 lea rdi,[rip+0x116994] # 0x12a102
            rdi = m.env.data_va(256207);
            // 1376e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 13770 call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 13775 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0x1377a;
        }
        0x1377a => {
            // block 1377a
            #[cfg(feature = "trace")] eprintln!("f_000136d0 pc 1377a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1377a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1377e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1377f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 13781 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 13783 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 13785 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 13787 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 13788 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bbc2 size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bbc2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2bbc2;
    loop {
        match pc {
        0x2bbc2 => {
            // block 2bbc2
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bbc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbc2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bbc3 mov rbp,rsp
            rbp = rsp;
            // 2bbc6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bbc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bbca push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2bbcc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bbce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bbcf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2bbd0 mov r15,rsi
            r15 = rsi;
            // 2bbd3 mov r12,rdi
            r12 = rdi;
            // 2bbd6 lea rbx,[r15+0x8]
            rbx = r15.wrapping_add(0x8);
            // 2bbda mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2bbde mov rdi,r15
            rdi = r15;
            // 2bbe1 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbe6 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 2bbee mov QWORD PTR [r15],rbx
            m.w64(r15, (rbx) as u64);
            // 2bbf1 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 2bbf9 mov r13,QWORD PTR [r12+0x50]
            r13 = m.r64(r12.wrapping_add(0x50)) as u64;
            // 2bbfe test r13,r13
            fl = logic::<64>(r13 & r13);
            // 2bc01 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc03;
        }
        0x2bc03 => {
            // block 2bc03
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc03 mov ecx,DWORD PTR [r12+0x58]
            rcx = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 2bc08 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 2bc0a je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc0c;
        }
        0x2bc0c => {
            // block 2bc0c
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc0c mov eax,DWORD PTR [r12+0x4c]
            rax = (m.r32(r12.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bc11 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bc13 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc15;
        }
        0x2bc15 => {
            // block 2bc15
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc15 cmp eax,DWORD PTR [r12+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r12.wrapping_add(0x20)) as u64).1;
            // 2bc1a ja 0x2bc68
            if fl.a() { pc = 0x2bc68; continue; }
            pc = 0x2bc1c;
        }
        0x2bc1c => {
            // block 2bc1c
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc1c mov rsi,QWORD PTR [r12+0x78]
            rsi = m.r64(r12.wrapping_add(0x78)) as u64;
            // 2bc21 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2bc24 je 0x2bc68
            if fl.e() { pc = 0x2bc68; continue; }
            pc = 0x2bc26;
        }
        0x2bc26 => {
            // block 2bc26
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc26 mov edx,DWORD PTR [r12+0x80]
            rdx = (m.r32(r12.wrapping_add(0x80)) as u64) as u32 as u64;
            // 2bc2e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2bc30 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 2bc32 je 0x2bc6a
            if fl.e() { pc = 0x2bc6a; continue; }
            pc = 0x2bc34;
        }
        0x2bc34 => {
            // block 2bc34
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc34 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2bc36 jne 0x2bc6a
            if fl.ne() { pc = 0x2bc6a; continue; }
            pc = 0x2bc38;
        }
        0x2bc38 => {
            // block 2bc38
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc38 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bc3a;
        }
        0x2bc3a => {
            // block 2bc3a
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc3a mov eax,DWORD PTR [rsi+rbx*4]
            rax = (m.r32(rsi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bc3d mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // 2bc40 mov rdi,r15
            rdi = r15;
            // 2bc43 mov r14,rsi
            r14 = rsi;
            // 2bc46 mov rsi,r13
            rsi = r13;
            // 2bc49 call 0xeb870
            { let r = { let t = m.env.stub_va(0x3d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bc4e mov rsi,r14
            rsi = r14;
            // 2bc51 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 2bc54 mov DWORD PTR [rax],ecx
            m.w32(rax, ((rcx as u32 as u64)) as u32);
            // 2bc56 add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 2bc5a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bc5d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bc5f cmp ebx,DWORD PTR [r12+0x4c]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r12.wrapping_add(0x4c)) as u64).1;
            // 2bc64 jb 0x2bc3a
            if fl.b() { pc = 0x2bc3a; continue; }
            pc = 0x2bc66;
        }
        0x2bc66 => {
            // block 2bc66
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc66 jmp 0x2bc6a
            pc = 0x2bc6a; continue;
        }
        0x2bc68 => {
            // block 2bc68
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc68 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bc6a;
        }
        0x2bc6a => {
            // block 2bc6a
            #[cfg(feature = "trace")] eprintln!("f_0002bbc2 pc 2bc6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bc6a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2bc6e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bc6f pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bc71 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2bc73 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bc75 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bc77 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bc78 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3153c size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0003153c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3153c;
    loop {
        match pc {
        0x3153c => {
            // block 3153c
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 3153c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3153c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3153d mov rbp,rsp
            rbp = rsp;
            // 31540 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31542 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31544 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31546 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31548 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31549 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3154a mov r13,rdi
            r13 = rdi;
            // 3154d lea r14,[r13+0x18]
            r14 = r13.wrapping_add(0x18);
            // 31551 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31555 mov rdi,r14
            rdi = r14;
            // 31558 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3155e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31560 je 0x315db
            if fl.e() { pc = 0x315db; continue; }
            pc = 0x31562;
        }
        0x31562 => {
            // block 31562
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 31562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31562 mov rdi,r14
            rdi = r14;
            // 31565 call 0xb6914
            { let r = { let t = m.env.code_va(0xb6914); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3156a and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 3156d xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 3156f cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // 31574 mov r15d,0x0
            r15 = (0x0u64) as u32 as u64;
            // 3157a jne 0x315de
            if fl.ne() { pc = 0x315de; continue; }
            pc = 0x3157c;
        }
        0x3157c => {
            // block 3157c
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 3157c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3157c mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 31583 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 31587 mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 3158e lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 31592 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31597 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3159a mov rdi,r14
            rdi = r14;
            // 3159d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315a0 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 315a3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315a7 jne 0x315de
            if fl.ne() { pc = 0x315de; continue; }
            pc = 0x315a9;
        }
        0x315a9 => {
            // block 315a9
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315a9 lea r12,[rbp-0x2c]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            pc = 0x315ad;
        }
        0x315ad => {
            // block 315ad
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315ad inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 315af cmp ebx,0x2710
            fl = sub::<32>((rbx as u32 as u64), 0x2710u64).1;
            // 315b5 ja 0x315db
            if fl.a() { pc = 0x315db; continue; }
            pc = 0x315b7;
        }
        0x315b7 => {
            // block 315b7
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315b7 mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 315bb mov esi,DWORD PTR [r13+0x2a8]
            rsi = (m.r32(r13.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // 315c2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 315c7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 315ca mov rdi,r14
            rdi = r14;
            // 315cd mov rdx,r12
            rdx = r12;
            // 315d0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 315d3 test BYTE PTR [rbp-0x2c],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64 & 0x4u64);
            // 315d7 je 0x315ad
            if fl.e() { pc = 0x315ad; continue; }
            pc = 0x315d9;
        }
        0x315d9 => {
            // block 315d9
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315d9 jmp 0x315de
            pc = 0x315de; continue;
        }
        0x315db => {
            // block 315db
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315db xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x315de;
        }
        0x315de => {
            // block 315de
            #[cfg(feature = "trace")] eprintln!("f_0003153c pc 315de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 315de mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 315e1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 315e5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 315e6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 315e8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 315ea pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 315ec pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 315ee pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 315ef ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43d98 size=178
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043d98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43d98;
    loop {
        match pc {
        0x43d98 => {
            // block 43d98
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43d98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43d98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43d99 mov rbp,rsp
            rbp = rsp;
            // 43d9c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 43d9e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 43da0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 43da2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 43da4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43da5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43da6 mov r14,rdx
            r14 = rdx;
            // 43da9 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 43dac mov r13,rdi
            r13 = rdi;
            // 43daf mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 43db3 call 0xcc811
            { let r = { let t = m.env.code_va(0xcc811); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43db8 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 43dba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43dbc cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 43dc0 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43dc2;
        }
        0x43dc2 => {
            // block 43dc2
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc2 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43dc4 je 0x43e3b
            if fl.e() { pc = 0x43e3b; continue; }
            pc = 0x43dc6;
        }
        0x43dc6 => {
            // block 43dc6
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dc6 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 43dcd lea r12,[r14+0x8]
            r12 = r14.wrapping_add(0x8);
            // 43dd1 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 43dd4 lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 43dd8 shl rax,0x5
            { let (r, f) = shl::<64>(rax, 0x5u64, fl); rax = r; fl = f; }
            // 43ddc lea rcx,[rip+0xbc2dd] # 0x1000c0
            rcx = m.env.data_va(84109);
            // 43de3 lea rbx,[rax+rcx*1+0x8]
            rbx = rax.wrapping_add(rcx).wrapping_add(0x8);
            // 43de8 mov r15d,0x1
            r15 = (0x1u64) as u32 as u64;
            pc = 0x43dee;
        }
        0x43dee => {
            // block 43dee
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43dee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43dee mov rdi,r14
            rdi = r14;
            // 43df1 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 43df5 call 0xebb40
            { let r = { let t = m.env.stub_va(0x30d); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43dfa cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // 43dfd setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 43e00 mov r9,QWORD PTR [r13+0x0]
            r9 = m.r64(r13) as u64;
            // 43e04 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 43e07 mov esi,DWORD PTR [rbx-0x8]
            rsi = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fff8)) as u64) as u32 as u64;
            // 43e0a mov ecx,DWORD PTR [rbx-0x4]
            rcx = (m.r32(rbx.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // 43e0d mov r8d,DWORD PTR [rbx]
            r8 = (m.r32(rbx) as u64) as u32 as u64;
            // 43e10 mov rdi,r13
            rdi = r13;
            // 43e13 call QWORD PTR [r9+0x30]
            { let r = { let t = m.r64(r9.wrapping_add(0x30)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43e17 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 43e19 je 0x43e39
            if fl.e() { pc = 0x43e39; continue; }
            pc = 0x43e1b;
        }
        0x43e1b => {
            // block 43e1b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e1b mov DWORD PTR [rbp-0x2c],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r15 as u32 as u64)) as u32);
            // 43e1f cmp r15d,0x3f
            fl = sub::<32>((r15 as u32 as u64), 0x3fu64).1;
            // 43e23 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 43e25 ja 0x43e3b
            if fl.a() { pc = 0x43e3b; continue; }
            pc = 0x43e27;
        }
        0x43e27 => {
            // block 43e27
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e27 lea ecx,[r15-0x8]
            rcx = (r15.wrapping_add(0xffff_ffff_ffff_fff8)) as u32 as u64;
            // 43e2b inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 43e2e add rbx,0xc
            { let (r, f) = add::<64>(rbx, 0xcu64);
              rbx = r;
              fl = f; }
            // 43e32 cmp ecx,0x37
            fl = sub::<32>((rcx as u32 as u64), 0x37u64).1;
            // 43e35 ja 0x43dee
            if fl.a() { pc = 0x43dee; continue; }
            pc = 0x43e37;
        }
        0x43e37 => {
            // block 43e37
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e37 jmp 0x43e3b
            pc = 0x43e3b; continue;
        }
        0x43e39 => {
            // block 43e39
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e39 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x43e3b;
        }
        0x43e3b => {
            // block 43e3b
            #[cfg(feature = "trace")] eprintln!("f_00043d98 pc 43e3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43e3b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43e3f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43e40 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 43e42 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 43e44 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 43e46 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 43e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd386 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d386<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd386;
    loop {
        match pc {
        0xd386 => {
            // block d386
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d386 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d386 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d387 mov rbp,rsp
            rbp = rsp;
            // d38a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d38c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d38e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d390 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d391 mov r14,rdx
            r14 = rdx;
            // d394 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // d396 mov r12,rdi
            r12 = rdi;
            // d399 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // d39e call 0xd0ea0
            { let r = { let t = m.env.code_va(0xd0ea0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3a3 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // d3a6 cmp eax,ebx
            fl = sub::<32>((rax as u32 as u64), (rbx as u32 as u64)).1;
            // d3a8 jbe 0xd3cb
            if fl.be() { pc = 0xd3cb; continue; }
            pc = 0xd3aa;
        }
        0xd3aa => {
            // block d3aa
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3aa test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // d3ad je 0xd3cf
            if fl.e() { pc = 0xd3cf; continue; }
            pc = 0xd3af;
        }
        0xd3af => {
            // block d3af
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3af movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // d3b2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d3b4 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d3b7 mov esi,0xfe
            rsi = (0xfeu64) as u32 as u64;
            // d3bc cmovne esi,ecx
            if fl.ne() { rsi = ((rcx as u32 as u64)) as u32 as u64; } else { rsi = rsi as u32 as u64; }
            // d3bf mov r15d,0xff
            r15 = (0xffu64) as u32 as u64;
            // d3c5 cmovne r15d,ecx
            if fl.ne() { r15 = ((rcx as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // d3c9 jmp 0xd3da
            pc = 0xd3da; continue;
        }
        0xd3cb => {
            // block d3cb
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cb xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3cd jmp 0xd426
            pc = 0xd426; continue;
        }
        0xd3cf => {
            // block d3cf
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3cf mov r15d,0x71
            r15 = (0x71u64) as u32 as u64;
            // d3d5 mov esi,0x70
            rsi = (0x70u64) as u32 as u64;
            pc = 0xd3da;
        }
        0xd3da => {
            // block d3da
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3da mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            // d3e1 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3e3 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // d3e5 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3e7;
        }
        0xd3e7 => {
            // block d3e7
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3e7 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // d3ea je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd3ec;
        }
        0xd3ec => {
            // block d3ec
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d3ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3ec lea rdx,[r14+0x4]
            rdx = r14.wrapping_add(0x4);
            // d3f0 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d3f4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3f6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d3fb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d3fe mov rdi,r12
            rdi = r12;
            // d401 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d404 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d406 je 0xd426
            if fl.e() { pc = 0xd426; continue; }
            pc = 0xd408;
        }
        0xd408 => {
            // block d408
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d408 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d408 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // d40c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // d410 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d415 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d418 mov rdi,r12
            rdi = r12;
            // d41b mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // d41e mov rdx,r14
            rdx = r14;
            // d421 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d424 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd426;
        }
        0xd426 => {
            // block d426
            #[cfg(feature = "trace")] eprintln!("f_0000d386 pc d426 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d426 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d428 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d429 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d42b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d42d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d42f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d430 ret
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
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc4d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            { let r = { let t = m.env.stub_va(0x583); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x5ad); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc520 mov rbx,rax
            rbx = rax;
            // bc523 lea r12,[rip+0x10c166] # 0x1c8690
            r12 = m.env.data_va(904797);
            // bc52a jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc52c => {
            // block bc52c
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc52c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc52c cmp DWORD PTR [rbx+0x1c],r14d
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x1c)) as u64, (r14 as u32 as u64)).1;
            // bc530 jne 0xbc568
            if fl.ne() { pc = 0xbc568; continue; }
            pc = 0xbc532;
        }
        0xbc532 => {
            // block bc532
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc532 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc532 lea rsi,[rbx+0x20]
            rsi = rbx.wrapping_add(0x20);
            // bc536 mov rdi,r15
            rdi = r15;
            // bc539 call 0xebdaa
            { let r = { let t = m.env.stub_va(0x577); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc547 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc552 jmp 0xbc563
            pc = 0xbc563; continue;
        }
        0xbc554 => {
            // block bc554
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc554 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc554 mov rax,rbx
            rax = rbx;
            pc = 0xbc557;
        }
        0xbc557 => {
            // block bc557
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc557 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc563 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc563 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // bc566 jne 0xbc52c
            if fl.ne() { pc = 0xbc52c; continue; }
            pc = 0xbc568;
        }
        0xbc568 => {
            // block bc568
            #[cfg(feature = "trace")] eprintln!("f_000bc4d4 pc bc568 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x12041 size=167
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012041<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12041;
    loop {
        match pc {
        0x12041 => {
            // block 12041
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12041 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12041 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12042 mov rbp,rsp
            rbp = rsp;
            // 12045 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 12047 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 12049 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1204b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1204d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1204e push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 1204f mov r15,r8
            r15 = r8;
            // 12052 mov r14,rcx
            r14 = rcx;
            // 12055 mov rbx,rdx
            rbx = rdx;
            // 12058 mov r12,rsi
            r12 = rsi;
            // 1205b mov r13,rdi
            r13 = rdi;
            // 1205e jmp 0x12064
            pc = 0x12064; continue;
        }
        0x12060 => {
            // block 12060
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12060 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12060 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            pc = 0x12064;
        }
        0x12064 => {
            // block 12064
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12064 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 12067 je 0x12099
            if fl.e() { pc = 0x12099; continue; }
            pc = 0x12069;
        }
        0x12069 => {
            // block 12069
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12069 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12069 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1206c je 0x120b8
            if fl.e() { pc = 0x120b8; continue; }
            pc = 0x1206e;
        }
        0x1206e => {
            // block 1206e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1206e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1206e mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 12071 cmp eax,DWORD PTR [r13+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r13.wrapping_add(0x18)) as u64).1;
            // 12075 jge 0x12088
            if fl.ge() { pc = 0x12088; continue; }
            pc = 0x12077;
        }
        0x12077 => {
            // block 12077
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12077 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12077 mov rdi,r15
            rdi = r15;
            // 1207a mov rsi,rbx
            rsi = rbx;
            // 1207d call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12082 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12086 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12088 => {
            // block 12088
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12088 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12088 mov rdi,r15
            rdi = r15;
            // 1208b mov rsi,r13
            rsi = r13;
            // 1208e call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 12093 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 12097 jmp 0x12060
            pc = 0x12060; continue;
        }
        0x12099 => {
            // block 12099
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 12099 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12099 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1209c je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x1209e;
        }
        0x1209e => {
            // block 1209e
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 1209e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1209e mov rdi,r15
            rdi = r15;
            // 120a1 mov rsi,rbx
            rsi = rbx;
            // 120a4 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120a9 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120ad add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // 120b1 cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // 120b4 jne 0x1209e
            if fl.ne() { pc = 0x1209e; continue; }
            pc = 0x120b6;
        }
        0x120b6 => {
            // block 120b6
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b6 jmp 0x120d9
            pc = 0x120d9; continue;
        }
        0x120b8 => {
            // block 120b8
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120b8 cmp r13,r12
            fl = sub::<64>(r13, r12).1;
            // 120bb je 0x120d9
            if fl.e() { pc = 0x120d9; continue; }
            pc = 0x120bd;
        }
        0x120bd => {
            // block 120bd
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120bd sub r12,r13
            { let (r, f) = sub::<64>(r12, r13);
              r12 = r;
              fl = f; }
            // 120c0 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x120c2;
        }
        0x120c2 => {
            // block 120c2
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120c2 lea rsi,[r13+rbx*1+0x0]
            rsi = r13.wrapping_add(rbx);
            // 120c7 lea rdi,[r15+rbx*1]
            rdi = r15.wrapping_add(rbx);
            // 120cb call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 120d0 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 120d4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 120d7 jne 0x120c2
            if fl.ne() { pc = 0x120c2; continue; }
            pc = 0x120d9;
        }
        0x120d9 => {
            // block 120d9
            #[cfg(feature = "trace")] eprintln!("f_00012041 pc 120d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 120d9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 120dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 120de pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 120e0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 120e2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 120e4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 120e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 120e7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xf1d4 size=160
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000f1d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xf1d4;
    loop {
        match pc {
        0xf1d4 => {
            // block f1d4
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f1d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f1d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // f1d5 mov rbp,rsp
            rbp = rsp;
            // f1d8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // f1da push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // f1dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // f1dd sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // f1e4 mov r14,rdx
            r14 = rdx;
            // f1e7 mov rbx,rdi
            rbx = rdi;
            // f1ea lea r15,[rbp-0x98]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // f1f1 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // f1f3 mov rdi,r15
            rdi = r15;
            // f1f6 call 0x2ccf6
            { let r = { let t = m.env.code_va(0x2ccf6); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f1fb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // f1fe mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // f205 mov rdi,rbx
            rdi = rbx;
            // f208 mov rsi,r15
            rsi = r15;
            // f20b call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f20d mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // f20f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // f211 je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf213;
        }
        0xf213 => {
            // block f213
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f213 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f213 cmp QWORD PTR [rbp-0x4c],0x0
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64, 0x0u64).1;
            // f218 je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf21a;
        }
        0xf21a => {
            // block f21a
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f21a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f21a cmp DWORD PTR [rbp-0x44],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64, 0x0u64).1;
            // f21e je 0xf235
            if fl.e() { pc = 0xf235; continue; }
            pc = 0xf220;
        }
        0xf220 => {
            // block f220
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f220 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f220 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // f227 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // f229 call 0x2cd04
            { let r = { let t = m.env.code_va(0x2cd04); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f22e mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // f231 mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            pc = 0xf235;
        }
        0xf235 => {
            // block f235
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f235 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f235 lea rdi,[rbp-0x4c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // f239 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f23e lea rdi,[rbp-0x74]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // f242 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f247 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // f249 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // f250 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // f251 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // f253 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // f255 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // f256 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xf257 => {
            // block f257
            #[cfg(feature = "trace")] eprintln!("f_0000f1d4 pc f257 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // f257 mov rbx,rax
            rbx = rax;
            // f25a lea rdi,[rbp-0x4c]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb4);
            // f25e call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f263 lea rdi,[rbp-0x74]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // f267 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // f26c mov rdi,rbx
            rdi = rbx;
            // f26f call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1064 size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001064<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1064;
    loop {
        match pc {
        0x1064 => {
            // block 1064
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 1064 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1064 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1065 mov rbp,rsp
            rbp = rsp;
            // 1068 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 106a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 106c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 106e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 106f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 1073 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 1076 mov r14,rsi
            r14 = rsi;
            // 1079 mov rbx,rdi
            rbx = rdi;
            // 107c mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 1083 mov DWORD PTR [rbp-0x28],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0xcu64) as u32);
            // 108a mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // 108e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1091 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1097 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 109a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 109c cmove r15d,r12d
            if fl.e() { r15 = ((r12 as u32 as u64)) as u32 as u64; } else { r15 = r15 as u32 as u64; }
            // 10a0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10a3 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 10a7 mov rdi,rbx
            rdi = rbx;
            // 10aa mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 10ad call QWORD PTR [rax+0x3e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x3e8)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10b3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 10b5 je 0x10f0
            if fl.e() { pc = 0x10f0; continue; }
            pc = 0x10b7;
        }
        0x10b7 => {
            // block 10b7
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10b7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10ba lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 10be mov rdi,rbx
            rdi = rbx;
            // 10c1 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 10c4 call QWORD PTR [rax+0x440]
            { let r = { let t = m.r64(rax.wrapping_add(0x440)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10ca test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 10cc je 0x10ed
            if fl.e() { pc = 0x10ed; continue; }
            pc = 0x10ce;
        }
        0x10ce => {
            // block 10ce
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ce cmp DWORD PTR [rbp-0x24],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, 0x0u64).1;
            // 10d2 jne 0x10e4
            if fl.ne() { pc = 0x10e4; continue; }
            pc = 0x10d4;
        }
        0x10d4 => {
            // block 10d4
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10d4 mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 10d7 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // 10da je 0x10e4
            if fl.e() { pc = 0x10e4; continue; }
            pc = 0x10dc;
        }
        0x10dc => {
            // block 10dc
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10dc mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 10df cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // 10e2 jne 0x10f0
            if fl.ne() { pc = 0x10f0; continue; }
            pc = 0x10e4;
        }
        0x10e4 => {
            // block 10e4
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e4 mov BYTE PTR [r14],0x1
            m.w8(r14, (0x1u64) as u8);
            // 10e8 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 10eb jmp 0x10f0
            pc = 0x10f0; continue;
        }
        0x10ed => {
            // block 10ed
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ed xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x10f0;
        }
        0x10f0 => {
            // block 10f0
            #[cfg(feature = "trace")] eprintln!("f_00001064 pc 10f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f0 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 10f3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 10f7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10f8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10fc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10ff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x17202 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00017202<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x17202;
    loop {
        match pc {
        0x17202 => {
            // block 17202
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17202 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17202 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 17203 mov rbp,rsp
            rbp = rsp;
            // 17206 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 17208 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 17209 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 1720d mov r14,rsi
            r14 = rsi;
            // 17210 mov ebx,edi
            rbx = ((rdi as u32 as u64)) as u32 as u64;
            // 17212 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 17216 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 1721b call 0x16080
            { let r = { let t = m.env.code_va(0x16080); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17220 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 17222 mov rcx,QWORD PTR [rbp-0x20]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 17226 sub rcx,QWORD PTR [rbp-0x28]
            { let (r, f) = sub::<64>(rcx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = r;
              fl = f; }
            // 1722a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 1722e movabs rdx,0xef7bdef7bdef7bdf
            rdx = 0xef7b_def7_bdef_7bdfu64;
            // 17238 imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 1723c cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 1723f jae 0x1727a
            if fl.ae() { pc = 0x1727a; continue; }
            pc = 0x17241;
        }
        0x17241 => {
            // block 17241
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 17241 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17241 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17244 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 17248 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // 1724b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1724d xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 1724f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 17252 mov rdi,r14
            rdi = r14;
            // 17255 call rax
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17257 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 17259 jmp 0x1727c
            pc = 0x1727c; continue;
        }
        0x1725b => {
            // block 1725b
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1725b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1725b mov rbx,rax
            rbx = rax;
            // 1725e lea rax,[rip+0x12a1b3] # 0x141418
            rax = m.env.data_va(351205);
            // 17265 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17269 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1726d call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17272 mov rdi,rbx
            rdi = rbx;
            // 17275 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1727a;
        }
        0x1727a => {
            // block 1727a
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727a xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x1727c;
        }
        0x1727c => {
            // block 1727c
            #[cfg(feature = "trace")] eprintln!("f_00017202 pc 1727c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1727c lea rax,[rip+0x12a195] # 0x141418
            rax = m.env.data_va(351205);
            // 17283 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 17287 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 1728b call 0xeb94e
            { let r = { let t = m.env.stub_va(0x11b); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17290 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 17292 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 17296 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 17297 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 17299 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1729a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8d2c size=150
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008d2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8d2c;
    loop {
        match pc {
        0x8d2c => {
            // block 8d2c
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8d2d mov rbp,rsp
            rbp = rsp;
            // 8d30 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8d32 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8d34 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8d36 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8d37 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8d3b mov r14,rdx
            r14 = rdx;
            // 8d3e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 8d40 mov r12,rdi
            r12 = rdi;
            // 8d43 mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8d48 call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d4d mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // 8d50 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 8d57 mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 8d5e mov DWORD PTR [r14],0xff
            m.w32(r14, (0xffu64) as u32);
            // 8d65 lea rsi,[rbp-0x24]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 8d69 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 8d6d mov edi,ebx
            rdi = ((rbx as u32 as u64)) as u32 as u64;
            // 8d6f call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d74 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8d76 je 0x8db2
            if fl.e() { pc = 0x8db2; continue; }
            pc = 0x8d78;
        }
        0x8d78 => {
            // block 8d78
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d78 mov esi,DWORD PTR [rbp-0x24]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 8d7b test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 8d7d je 0x8db2
            if fl.e() { pc = 0x8db2; continue; }
            pc = 0x8d7f;
        }
        0x8d7f => {
            // block 8d7f
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d7f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 8d82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8d84 cmp esi,r15d
            fl = sub::<32>((rsi as u32 as u64), (r15 as u32 as u64)).1;
            // 8d87 ja 0x8db4
            if fl.a() { pc = 0x8db4; continue; }
            pc = 0x8d89;
        }
        0x8d89 => {
            // block 8d89
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d89 cmp edx,0x3
            fl = sub::<32>((rdx as u32 as u64), 0x3u64).1;
            // 8d8c ja 0x8db4
            if fl.a() { pc = 0x8db4; continue; }
            pc = 0x8d8e;
        }
        0x8d8e => {
            // block 8d8e
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8d8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d8e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 8d92 lea rcx,[rip+0xf68f7] # 0xff690
            rcx = m.env.data_va(81501);
            // 8d99 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d9c lea rdi,[rip+0xf68fd] # 0xff6a0
            rdi = m.env.data_va(81517);
            // 8da3 mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8da7 mov rdi,r12
            rdi = r12;
            // 8daa mov rdx,r14
            rdx = r14;
            // 8dad call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8db0 jmp 0x8db4
            pc = 0x8db4; continue;
        }
        0x8db2 => {
            // block 8db2
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8db2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x8db4;
        }
        0x8db4 => {
            // block 8db4
            #[cfg(feature = "trace")] eprintln!("f_00008d2c pc 8db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8db4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8db8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8db9 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8dbb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8dbd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8dbf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8dc0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3101e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = rax; m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.code_va(0x2eee0); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31085;
        }
        0x31085 => {
            // block 31085
            #[cfg(feature = "trace")] eprintln!("f_0003101e pc 31085 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31085 mov rdi,rbx
            rdi = rbx;
            // 31088 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x3108d;
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
            { let r = { let t = m.env.stub_va(0x907); m.set_raw([rdi, rsi, rdx, rcx, r8, r9]); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
