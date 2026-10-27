//! Mechanically lifted functions (scripts/lift_run.py), module hb004. Each one is checked against the original
//! in the world harness (tests/world_hb004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2224e size=7693
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002224e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2224e;
    loop {
        match pc {
        0x2224e => {
            // block 2224e
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 2224e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2224e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2224f mov rbp,rsp
            rbp = rsp;
            // 22252 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 22254 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 22256 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 22258 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2225a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2225b sub rsp,0x1a8
            { let (r, f) = sub::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 22262 mov rbx,r9
            rbx = r9;
            // 22265 mov QWORD PTR [rbp-0x130],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rbx) as u64);
            // 2226c mov r15,r8
            r15 = r8;
            // 2226f mov r12,rcx
            r12 = rcx;
            // 22272 mov QWORD PTR [rbp-0x138],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rdx) as u64);
            // 22279 mov QWORD PTR [rbp-0x160],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rsi) as u64);
            // 22280 mov r13,rdi
            r13 = rdi;
            // 22283 mov r8,rbx
            r8 = rbx;
            // 22286 call 0xeba7a
            { let r = { let t = m.env.stub_va(0x247); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2228b mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // 2228e mov rdi,r15
            rdi = r15;
            // 22291 mov rsi,r12
            rsi = r12;
            // 22294 call QWORD PTR [rbx]
            { let r = { let t = m.r64(rbx) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22296 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 22298 je 0x24006
            if fl.e() { pc = 0x24006; continue; }
            pc = 0x2229e;
        }
        0x2229e => {
            // block 2229e
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 2229e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2229e mov DWORD PTR [rbp-0x124],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fedc), ((r14 as u32 as u64)) as u32);
            // 222a5 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 222aa mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 222b1 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 222b5 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 222ba mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 222c1 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 222c8 lea r14,[r12+0x18]
            r14 = r12.wrapping_add(0x18);
            // 222cd mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 222d2 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 222d9 mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 222de mov rcx,QWORD PTR [r12+0x20]
            rcx = m.r64(r12.wrapping_add(0x20)) as u64;
            // 222e3 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 222ea mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 222f1 mov QWORD PTR [r12+0x28],0x0
            m.w64(r12.wrapping_add(0x28), (0x0u64) as u64);
            // 222fa mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            // 22303 mov QWORD PTR [r12+0x18],0x0
            m.w64(r12.wrapping_add(0x18), (0x0u64) as u64);
            // 2230c mov ax,WORD PTR [r12+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 22312 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 22319 mov eax,DWORD PTR [r12+0x58]
            rax = (m.r32(r12.wrapping_add(0x58)) as u64) as u32 as u64;
            // 2231e mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 22324 mov rax,QWORD PTR [r12+0x50]
            rax = m.r64(r12.wrapping_add(0x50)) as u64;
            // 22329 mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 22330 mov rax,QWORD PTR [r12+0x48]
            rax = m.r64(r12.wrapping_add(0x48)) as u64;
            // 22335 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 2233c mov rax,QWORD PTR [r12+0x40]
            rax = m.r64(r12.wrapping_add(0x40)) as u64;
            // 22341 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 22348 mov rax,QWORD PTR [r12+0x30]
            rax = m.r64(r12.wrapping_add(0x30)) as u64;
            // 2234d mov rcx,QWORD PTR [r12+0x38]
            rcx = m.r64(r12.wrapping_add(0x38)) as u64;
            // 22352 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 22359 mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 22360 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 2236b mov rax,QWORD PTR [r12+0x60]
            rax = m.r64(r12.wrapping_add(0x60)) as u64;
            // 22370 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 22377 movups xmm0,XMMWORD PTR [r12+0x68]
            x0 = m.r128(r12.wrapping_add(0x68));
            // 2237d movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 22384 mov QWORD PTR [r12+0x70],0x0
            m.w64(r12.wrapping_add(0x70), (0x0u64) as u64);
            // 2238d mov QWORD PTR [r12+0x68],0x0
            m.w64(r12.wrapping_add(0x68), (0x0u64) as u64);
            // 22396 mov QWORD PTR [r12+0x60],0x0
            m.w64(r12.wrapping_add(0x60), (0x0u64) as u64);
            // 2239f mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 223aa movups xmm0,XMMWORD PTR [r12+0x78]
            x0 = m.r128(r12.wrapping_add(0x78));
            // 223b0 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 223b7 mov rax,QWORD PTR [r12+0x88]
            rax = m.r64(r12.wrapping_add(0x88)) as u64;
            // 223bf mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 223c6 mov QWORD PTR [r12+0x88],0x0
            m.w64(r12.wrapping_add(0x88), (0x0u64) as u64);
            // 223d2 mov QWORD PTR [r12+0x80],0x0
            m.w64(r12.wrapping_add(0x80), (0x0u64) as u64);
            // 223de mov QWORD PTR [r12+0x78],0x0
            m.w64(r12.wrapping_add(0x78), (0x0u64) as u64);
            // 223e7 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 223f2 movups xmm0,XMMWORD PTR [r12+0x90]
            x0 = m.r128(r12.wrapping_add(0x90));
            // 223fb movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 22402 mov rax,QWORD PTR [r12+0xa0]
            rax = m.r64(r12.wrapping_add(0xa0)) as u64;
            // 2240a mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 2240e mov QWORD PTR [r12+0xa0],0x0
            m.w64(r12.wrapping_add(0xa0), (0x0u64) as u64);
            // 2241a mov QWORD PTR [r12+0x98],0x0
            m.w64(r12.wrapping_add(0x98), (0x0u64) as u64);
            // 22426 mov QWORD PTR [r12+0x90],0x0
            m.w64(r12.wrapping_add(0x90), (0x0u64) as u64);
            // 22432 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 2243a movups xmm0,XMMWORD PTR [r12+0xa8]
            x0 = m.r128(r12.wrapping_add(0xa8));
            // 22443 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 22447 mov rax,QWORD PTR [r12+0xb8]
            rax = m.r64(r12.wrapping_add(0xb8)) as u64;
            // 2244f mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 22453 mov QWORD PTR [r12+0xb8],0x0
            m.w64(r12.wrapping_add(0xb8), (0x0u64) as u64);
            // 2245f mov QWORD PTR [r12+0xb0],0x0
            m.w64(r12.wrapping_add(0xb0), (0x0u64) as u64);
            // 2246b mov QWORD PTR [r12+0xa8],0x0
            m.w64(r12.wrapping_add(0xa8), (0x0u64) as u64);
            // 22477 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2247f mov rax,QWORD PTR [r12+0xc0]
            rax = m.r64(r12.wrapping_add(0xc0)) as u64;
            // 22487 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 2248b movups xmm0,XMMWORD PTR [r12+0xc8]
            x0 = m.r128(r12.wrapping_add(0xc8));
            // 22494 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 22498 mov QWORD PTR [r12+0xd0],0x0
            m.w64(r12.wrapping_add(0xd0), (0x0u64) as u64);
            // 224a4 mov QWORD PTR [r12+0xc8],0x0
            m.w64(r12.wrapping_add(0xc8), (0x0u64) as u64);
            // 224b0 mov QWORD PTR [r12+0xc0],0x0
            m.w64(r12.wrapping_add(0xc0), (0x0u64) as u64);
            // 224bc mov eax,DWORD PTR [r12+0xf0]
            rax = (m.r32(r12.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 224c4 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 224c7 mov rax,QWORD PTR [r12+0xe8]
            rax = m.r64(r12.wrapping_add(0xe8)) as u64;
            // 224cf mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 224d3 mov rax,QWORD PTR [r12+0xd8]
            rax = m.r64(r12.wrapping_add(0xd8)) as u64;
            // 224db mov rcx,QWORD PTR [r12+0xe0]
            rcx = m.r64(r12.wrapping_add(0xe0)) as u64;
            // 224e3 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 224e7 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 224eb mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 224ef mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 224f4 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 224f7 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 224fb mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 22500 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 22504 test BYTE PTR [r12+0x18],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x18)) as u64 & 0x1u64);
            // 2250a jne 0x22514
            if fl.ne() { pc = 0x22514; continue; }
            pc = 0x2250c;
        }
        0x2250c => {
            // block 2250c
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 2250c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2250c mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 22512 jmp 0x22525
            pc = 0x22525; continue;
        }
        0x22514 => {
            // block 22514
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22514 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22514 mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 22519 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 2251c mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x22525;
        }
        0x22525 => {
            // block 22525
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22525 mov QWORD PTR [rbp-0x1a8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (r13) as u64);
            // 2252c xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2252e mov rdi,r14
            rdi = r14;
            // 22531 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22536 lea rdx,[r12+0x30]
            rdx = r12.wrapping_add(0x30);
            // 2253b mov QWORD PTR [rbp-0x190],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rdx) as u64);
            // 22542 lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 22547 mov QWORD PTR [rbp-0x188],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rdi) as u64);
            // 2254e lea rax,[r12+0x78]
            rax = r12.wrapping_add(0x78);
            // 22553 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 2255a lea rax,[r12+0x90]
            rax = r12.wrapping_add(0x90);
            // 22562 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 22569 lea rax,[r12+0xa8]
            rax = r12.wrapping_add(0xa8);
            // 22571 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 22578 lea rax,[r12+0xc0]
            rax = r12.wrapping_add(0xc0);
            // 22580 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 22587 lea rbx,[r12+0xd8]
            rbx = r12.wrapping_add(0xd8);
            // 2258f mov QWORD PTR [rbp-0x180],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rbx) as u64);
            // 22596 lea r13,[r15+0x18]
            r13 = r15.wrapping_add(0x18);
            // 2259a mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 2259e mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 225a2 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 225a6 mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 225aa mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 225ae mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 225b1 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 225b9 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 225c1 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 225c9 mov ax,WORD PTR [r15+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 225ce mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 225d2 mov eax,DWORD PTR [r15+0x58]
            rax = (m.r32(r15.wrapping_add(0x58)) as u64) as u32 as u64;
            // 225d6 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 225d9 mov rax,QWORD PTR [r15+0x50]
            rax = m.r64(r15.wrapping_add(0x50)) as u64;
            // 225dd mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 225e1 mov rax,QWORD PTR [r15+0x48]
            rax = m.r64(r15.wrapping_add(0x48)) as u64;
            // 225e5 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 225e9 mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // 225ed mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 225f1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 225f5 mov rcx,QWORD PTR [r15+0x38]
            rcx = m.r64(r15.wrapping_add(0x38)) as u64;
            // 225f9 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 225fd mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 22600 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22605 movups xmm0,XMMWORD PTR [r15+0x60]
            x0 = m.r128(r15.wrapping_add(0x60));
            // 2260a movups XMMWORD PTR [r12+0x60],xmm0
            m.w128(r12.wrapping_add(0x60), x0);
            // 22610 mov rax,QWORD PTR [r15+0x70]
            rax = m.r64(r15.wrapping_add(0x70)) as u64;
            // 22614 mov QWORD PTR [r12+0x70],rax
            m.w64(r12.wrapping_add(0x70), (rax) as u64);
            // 22619 mov QWORD PTR [r15+0x70],0x0
            m.w64(r15.wrapping_add(0x70), (0x0u64) as u64);
            // 22621 mov QWORD PTR [r15+0x68],0x0
            m.w64(r15.wrapping_add(0x68), (0x0u64) as u64);
            // 22629 mov QWORD PTR [r15+0x60],0x0
            m.w64(r15.wrapping_add(0x60), (0x0u64) as u64);
            // 22631 mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 22638 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2263d movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 22642 movups XMMWORD PTR [r12+0x78],xmm0
            m.w128(r12.wrapping_add(0x78), x0);
            // 22648 mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 2264f mov QWORD PTR [r12+0x88],rax
            m.w64(r12.wrapping_add(0x88), (rax) as u64);
            // 22657 mov QWORD PTR [r15+0x88],0x0
            m.w64(r15.wrapping_add(0x88), (0x0u64) as u64);
            // 22662 mov QWORD PTR [r15+0x80],0x0
            m.w64(r15.wrapping_add(0x80), (0x0u64) as u64);
            // 2266d mov QWORD PTR [r15+0x78],0x0
            m.w64(r15.wrapping_add(0x78), (0x0u64) as u64);
            // 22675 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 2267c call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22681 movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 22689 movups XMMWORD PTR [r12+0x90],xmm0
            m.w128(r12.wrapping_add(0x90), x0);
            // 22692 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 22699 mov QWORD PTR [r12+0xa0],rax
            m.w64(r12.wrapping_add(0xa0), (rax) as u64);
            // 226a1 mov QWORD PTR [r15+0xa0],0x0
            m.w64(r15.wrapping_add(0xa0), (0x0u64) as u64);
            // 226ac mov QWORD PTR [r15+0x98],0x0
            m.w64(r15.wrapping_add(0x98), (0x0u64) as u64);
            // 226b7 mov QWORD PTR [r15+0x90],0x0
            m.w64(r15.wrapping_add(0x90), (0x0u64) as u64);
            // 226c2 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 226c9 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 226ce movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 226d6 movups XMMWORD PTR [r12+0xa8],xmm0
            m.w128(r12.wrapping_add(0xa8), x0);
            // 226df mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 226e6 mov QWORD PTR [r12+0xb8],rax
            m.w64(r12.wrapping_add(0xb8), (rax) as u64);
            // 226ee mov QWORD PTR [r15+0xb8],0x0
            m.w64(r15.wrapping_add(0xb8), (0x0u64) as u64);
            // 226f9 mov QWORD PTR [r15+0xb0],0x0
            m.w64(r15.wrapping_add(0xb0), (0x0u64) as u64);
            // 22704 mov QWORD PTR [r15+0xa8],0x0
            m.w64(r15.wrapping_add(0xa8), (0x0u64) as u64);
            // 2270f mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 22716 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2271b movups xmm0,XMMWORD PTR [r15+0xc0]
            x0 = m.r128(r15.wrapping_add(0xc0));
            // 22723 movups XMMWORD PTR [r12+0xc0],xmm0
            m.w128(r12.wrapping_add(0xc0), x0);
            // 2272c mov rax,QWORD PTR [r15+0xd0]
            rax = m.r64(r15.wrapping_add(0xd0)) as u64;
            // 22733 mov QWORD PTR [r12+0xd0],rax
            m.w64(r12.wrapping_add(0xd0), (rax) as u64);
            // 2273b mov QWORD PTR [r15+0xd0],0x0
            m.w64(r15.wrapping_add(0xd0), (0x0u64) as u64);
            // 22746 mov QWORD PTR [r15+0xc8],0x0
            m.w64(r15.wrapping_add(0xc8), (0x0u64) as u64);
            // 22751 mov QWORD PTR [r15+0xc0],0x0
            m.w64(r15.wrapping_add(0xc0), (0x0u64) as u64);
            // 2275c mov eax,DWORD PTR [r15+0xf0]
            rax = (m.r32(r15.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 22763 mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 22766 mov rax,QWORD PTR [r15+0xe8]
            rax = m.r64(r15.wrapping_add(0xe8)) as u64;
            // 2276d mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 22771 mov rax,QWORD PTR [r15+0xd8]
            rax = m.r64(r15.wrapping_add(0xd8)) as u64;
            // 22778 mov rcx,QWORD PTR [r15+0xe0]
            rcx = m.r64(r15.wrapping_add(0xe0)) as u64;
            // 2277f mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 22783 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 22786 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 2278d mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 22791 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 22798 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 2279f mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 227a3 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 227a6 test BYTE PTR [r15+0x18],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x18)) as u64 & 0x1u64);
            // 227ab jne 0x227b6
            if fl.ne() { pc = 0x227b6; continue; }
            pc = 0x227ad;
        }
        0x227ad => {
            // block 227ad
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 227ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 227ad mov WORD PTR [r13+0x0],0x0
            m.w16(r13, (0x0u64) as u16);
            // 227b4 jmp 0x227c5
            pc = 0x227c5; continue;
        }
        0x227b6 => {
            // block 227b6
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 227b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 227b6 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 227ba mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 227bd mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x227c5;
        }
        0x227c5 => {
            // block 227c5
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 227c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 227c5 mov eax,DWORD PTR [rbp-0x124]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 227cb mov QWORD PTR [rbp-0x198],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (r14) as u64);
            // 227d2 mov DWORD PTR [rbp-0x124],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fedc), ((rax as u32 as u64)) as u32);
            // 227d8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 227da mov rdi,r13
            rdi = r13;
            // 227dd call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 227e2 lea rbx,[rbp-0x108]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 227e9 lea rsi,[rbp-0xf0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 227f0 lea r14,[rbp-0xa8]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 227f7 lea rax,[r15+0x30]
            rax = r15.wrapping_add(0x30);
            // 227fb lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 227ff lea rcx,[r15+0x78]
            rcx = r15.wrapping_add(0x78);
            // 22803 mov QWORD PTR [rbp-0x178],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rcx) as u64);
            // 2280a mov rcx,QWORD PTR [rbx+0x10]
            rcx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 2280e mov QWORD PTR [r13+0x10],rcx
            m.w64(r13.wrapping_add(0x10), (rcx) as u64);
            // 22812 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 22815 mov rdx,QWORD PTR [rbx+0x8]
            rdx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 22819 mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // 2281d lea rdx,[r15+0x90]
            rdx = r15.wrapping_add(0x90);
            // 22824 mov QWORD PTR [rbp-0x170],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdx) as u64);
            // 2282b mov QWORD PTR [r13+0x0],rcx
            m.w64(r13, (rcx) as u64);
            // 2282f mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 22837 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 2283f mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 22846 mov cx,WORD PTR [rsi+0x2c]
            rcx = (rcx & !0xffff) | ((m.r16(rsi.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 2284a mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 2284e mov ecx,DWORD PTR [rsi+0x28]
            rcx = (m.r32(rsi.wrapping_add(0x28)) as u64) as u32 as u64;
            // 22851 mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 22854 mov rcx,QWORD PTR [rsi+0x20]
            rcx = m.r64(rsi.wrapping_add(0x20)) as u64;
            // 22858 mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 2285c mov rcx,QWORD PTR [rsi+0x18]
            rcx = m.r64(rsi.wrapping_add(0x18)) as u64;
            // 22860 mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 22864 mov rcx,QWORD PTR [rsi+0x10]
            rcx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // 22868 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2286c mov rcx,QWORD PTR [rsi]
            rcx = m.r64(rsi) as u64;
            // 2286f mov rdx,QWORD PTR [rsi+0x8]
            rdx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 22873 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 22877 lea r13,[r15+0xa8]
            r13 = r15.wrapping_add(0xa8);
            // 2287e mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 22881 lea rax,[r15+0xc0]
            rax = r15.wrapping_add(0xc0);
            // 22888 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 2288f lea rbx,[r15+0xd8]
            rbx = r15.wrapping_add(0xd8);
            // 22896 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2289b movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 228a2 movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 228a7 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 228ae mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 228b2 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 228b9 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 228c1 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 228c9 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 228d0 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 228d7 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 228dc movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 228e3 movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 228e8 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 228ef mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 228f6 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 228fe mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 22906 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 2290d mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 22914 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22919 lea r14,[rbp-0x90]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 22920 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 22927 movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 2292f mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 22933 mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 2293a lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 22941 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22949 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 22951 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22958 mov rdi,r13
            rdi = r13;
            // 2295b call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22960 lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 22964 movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 22968 movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 22970 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 22974 mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 2297b lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 2297f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22987 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2298f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22996 mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 2299d call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 229a2 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 229a6 movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 229aa movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 229b2 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 229b6 mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 229bd lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 229c1 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 229c9 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 229d1 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 229d8 lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 229dc mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 229df mov DWORD PTR [rbx+0x18],eax
            m.w32(rbx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 229e2 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 229e6 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 229ea mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 229ed mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 229f1 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 229f5 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 229f8 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 229fd mov rdi,r13
            rdi = r13;
            // 22a00 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a05 mov rdi,r14
            rdi = r14;
            // 22a08 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a0d lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 22a14 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a19 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 22a20 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a25 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 22a2c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a31 mov rdi,r12
            rdi = r12;
            // 22a34 mov r13,QWORD PTR [rbp-0x138]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 22a3b mov rsi,r13
            rsi = r13;
            // 22a3e mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 22a45 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22a47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 22a49 je 0x22c91
            if fl.e() { pc = 0x22c91; continue; }
            pc = 0x22a4f;
        }
        0x22a4f => {
            // block 22a4f
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22a4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22a4f mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 22a53 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 22a5a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 22a5e mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 22a62 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 22a69 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 22a70 lea r15,[r13+0x18]
            r15 = r13.wrapping_add(0x18);
            // 22a74 mov rax,QWORD PTR [r13+0x28]
            rax = m.r64(r13.wrapping_add(0x28)) as u64;
            // 22a78 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 22a7f mov rax,QWORD PTR [r13+0x18]
            rax = m.r64(r13.wrapping_add(0x18)) as u64;
            // 22a83 mov rcx,QWORD PTR [r13+0x20]
            rcx = m.r64(r13.wrapping_add(0x20)) as u64;
            // 22a87 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 22a8e mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 22a95 mov QWORD PTR [r13+0x28],0x0
            m.w64(r13.wrapping_add(0x28), (0x0u64) as u64);
            // 22a9d mov QWORD PTR [r13+0x20],0x0
            m.w64(r13.wrapping_add(0x20), (0x0u64) as u64);
            // 22aa5 mov QWORD PTR [r13+0x18],0x0
            m.w64(r13.wrapping_add(0x18), (0x0u64) as u64);
            // 22aad mov ax,WORD PTR [r13+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 22ab2 mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 22ab9 mov eax,DWORD PTR [r13+0x58]
            rax = (m.r32(r13.wrapping_add(0x58)) as u64) as u32 as u64;
            // 22abd mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 22ac3 mov rax,QWORD PTR [r13+0x50]
            rax = m.r64(r13.wrapping_add(0x50)) as u64;
            // 22ac7 mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 22ace mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 22ad2 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 22ad9 mov rax,QWORD PTR [r13+0x40]
            rax = m.r64(r13.wrapping_add(0x40)) as u64;
            // 22add mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 22ae4 mov rax,QWORD PTR [r13+0x30]
            rax = m.r64(r13.wrapping_add(0x30)) as u64;
            // 22ae8 mov rcx,QWORD PTR [r13+0x38]
            rcx = m.r64(r13.wrapping_add(0x38)) as u64;
            // 22aec mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 22af3 mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 22afa mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 22b05 mov rax,QWORD PTR [r13+0x60]
            rax = m.r64(r13.wrapping_add(0x60)) as u64;
            // 22b09 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 22b10 movups xmm0,XMMWORD PTR [r13+0x68]
            x0 = m.r128(r13.wrapping_add(0x68));
            // 22b15 movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 22b1c mov QWORD PTR [r13+0x70],0x0
            m.w64(r13.wrapping_add(0x70), (0x0u64) as u64);
            // 22b24 mov QWORD PTR [r13+0x68],0x0
            m.w64(r13.wrapping_add(0x68), (0x0u64) as u64);
            // 22b2c mov QWORD PTR [r13+0x60],0x0
            m.w64(r13.wrapping_add(0x60), (0x0u64) as u64);
            // 22b34 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 22b3f movups xmm0,XMMWORD PTR [r13+0x78]
            x0 = m.r128(r13.wrapping_add(0x78));
            // 22b44 movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 22b4b mov rax,QWORD PTR [r13+0x88]
            rax = m.r64(r13.wrapping_add(0x88)) as u64;
            // 22b52 mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 22b59 mov QWORD PTR [r13+0x88],0x0
            m.w64(r13.wrapping_add(0x88), (0x0u64) as u64);
            // 22b64 mov QWORD PTR [r13+0x80],0x0
            m.w64(r13.wrapping_add(0x80), (0x0u64) as u64);
            // 22b6f mov QWORD PTR [r13+0x78],0x0
            m.w64(r13.wrapping_add(0x78), (0x0u64) as u64);
            // 22b77 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 22b82 movups xmm0,XMMWORD PTR [r13+0x90]
            x0 = m.r128(r13.wrapping_add(0x90));
            // 22b8a movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 22b91 mov rax,QWORD PTR [r13+0xa0]
            rax = m.r64(r13.wrapping_add(0xa0)) as u64;
            // 22b98 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 22b9c mov QWORD PTR [r13+0xa0],0x0
            m.w64(r13.wrapping_add(0xa0), (0x0u64) as u64);
            // 22ba7 mov QWORD PTR [r13+0x98],0x0
            m.w64(r13.wrapping_add(0x98), (0x0u64) as u64);
            // 22bb2 mov QWORD PTR [r13+0x90],0x0
            m.w64(r13.wrapping_add(0x90), (0x0u64) as u64);
            // 22bbd mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 22bc5 movups xmm0,XMMWORD PTR [r13+0xa8]
            x0 = m.r128(r13.wrapping_add(0xa8));
            // 22bcd movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 22bd1 mov rax,QWORD PTR [r13+0xb8]
            rax = m.r64(r13.wrapping_add(0xb8)) as u64;
            // 22bd8 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 22bdc mov QWORD PTR [r13+0xb8],0x0
            m.w64(r13.wrapping_add(0xb8), (0x0u64) as u64);
            // 22be7 mov QWORD PTR [r13+0xb0],0x0
            m.w64(r13.wrapping_add(0xb0), (0x0u64) as u64);
            // 22bf2 mov QWORD PTR [r13+0xa8],0x0
            m.w64(r13.wrapping_add(0xa8), (0x0u64) as u64);
            // 22bfd mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 22c05 mov rax,QWORD PTR [r13+0xc0]
            rax = m.r64(r13.wrapping_add(0xc0)) as u64;
            // 22c0c mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 22c10 movups xmm0,XMMWORD PTR [r13+0xc8]
            x0 = m.r128(r13.wrapping_add(0xc8));
            // 22c18 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 22c1c mov QWORD PTR [r13+0xd0],0x0
            m.w64(r13.wrapping_add(0xd0), (0x0u64) as u64);
            // 22c27 mov QWORD PTR [r13+0xc8],0x0
            m.w64(r13.wrapping_add(0xc8), (0x0u64) as u64);
            // 22c32 mov QWORD PTR [r13+0xc0],0x0
            m.w64(r13.wrapping_add(0xc0), (0x0u64) as u64);
            // 22c3d mov eax,DWORD PTR [r13+0xf0]
            rax = (m.r32(r13.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 22c44 mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 22c47 mov rax,QWORD PTR [r13+0xe8]
            rax = m.r64(r13.wrapping_add(0xe8)) as u64;
            // 22c4e mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 22c52 mov rax,QWORD PTR [r13+0xd8]
            rax = m.r64(r13.wrapping_add(0xd8)) as u64;
            // 22c59 mov rcx,QWORD PTR [r13+0xe0]
            rcx = m.r64(r13.wrapping_add(0xe0)) as u64;
            // 22c60 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 22c64 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 22c68 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 22c6d mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 22c71 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 22c75 mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 22c7a mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 22c7e mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 22c82 test BYTE PTR [r13+0x18],0x1
            fl = logic::<8>(m.r8(r13.wrapping_add(0x18)) as u64 & 0x1u64);
            // 22c87 jne 0x22ca0
            if fl.ne() { pc = 0x22ca0; continue; }
            pc = 0x22c89;
        }
        0x22c89 => {
            // block 22c89
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22c89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22c89 mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 22c8f jmp 0x22caf
            pc = 0x22caf; continue;
        }
        0x22c91 => {
            // block 22c91
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22c91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22c91 mov r14d,DWORD PTR [rbp-0x124]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 22c98 inc r14d
            { let (r, f) = inc::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // 22c9b jmp 0x24006
            pc = 0x24006; continue;
        }
        0x22ca0 => {
            // block 22ca0
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22ca0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22ca0 mov rax,QWORD PTR [r13+0x28]
            rax = m.r64(r13.wrapping_add(0x28)) as u64;
            // 22ca4 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 22ca7 mov QWORD PTR [r13+0x20],0x0
            m.w64(r13.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x22caf;
        }
        0x22caf => {
            // block 22caf
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22caf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22caf mov rbx,QWORD PTR [rbp-0x160]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 22cb6 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 22cb8 mov rdi,r15
            rdi = r15;
            // 22cbb call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22cc0 mov rdx,r15
            rdx = r15;
            // 22cc3 lea rsi,[r13+0x30]
            rsi = r13.wrapping_add(0x30);
            // 22cc7 mov QWORD PTR [rbp-0x1c8],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rsi) as u64);
            // 22cce lea rdi,[r13+0x60]
            rdi = r13.wrapping_add(0x60);
            // 22cd2 mov QWORD PTR [rbp-0x1b8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rdi) as u64);
            // 22cd9 lea r14,[r13+0x78]
            r14 = r13.wrapping_add(0x78);
            // 22cdd mov QWORD PTR [rbp-0x1b0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (r14) as u64);
            // 22ce4 lea rax,[r13+0x90]
            rax = r13.wrapping_add(0x90);
            // 22ceb mov QWORD PTR [rbp-0x1a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe60), (rax) as u64);
            // 22cf2 lea rax,[r13+0xa8]
            rax = r13.wrapping_add(0xa8);
            // 22cf9 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 22d00 lea rax,[r13+0xc0]
            rax = r13.wrapping_add(0xc0);
            // 22d07 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // 22d0e lea rax,[r13+0xd8]
            rax = r13.wrapping_add(0xd8);
            // 22d15 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // 22d1c mov r15,QWORD PTR [rbp-0x198]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 22d23 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 22d27 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 22d2b mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 22d2e mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 22d32 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 22d36 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 22d39 mov QWORD PTR [rbp-0x1c0],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe40), (rdx) as u64);
            // 22d40 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 22d48 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 22d50 mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 22d57 mov rcx,QWORD PTR [rbp-0x190]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 22d5e mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 22d62 mov WORD PTR [rsi+0x2c],ax
            m.w16(rsi.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 22d66 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 22d69 mov DWORD PTR [rsi+0x28],eax
            m.w32(rsi.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 22d6c mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 22d70 mov QWORD PTR [rsi+0x20],rax
            m.w64(rsi.wrapping_add(0x20), (rax) as u64);
            // 22d74 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 22d78 mov QWORD PTR [rsi+0x18],rax
            m.w64(rsi.wrapping_add(0x18), (rax) as u64);
            // 22d7c mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 22d80 mov QWORD PTR [rsi+0x10],rax
            m.w64(rsi.wrapping_add(0x10), (rax) as u64);
            // 22d84 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 22d87 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 22d8b mov QWORD PTR [rsi+0x8],rcx
            m.w64(rsi.wrapping_add(0x8), (rcx) as u64);
            // 22d8f mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 22d92 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22d97 movups xmm0,XMMWORD PTR [r12+0x60]
            x0 = m.r128(r12.wrapping_add(0x60));
            // 22d9d movups XMMWORD PTR [r13+0x60],xmm0
            m.w128(r13.wrapping_add(0x60), x0);
            // 22da2 mov rax,QWORD PTR [r12+0x70]
            rax = m.r64(r12.wrapping_add(0x70)) as u64;
            // 22da7 mov QWORD PTR [r13+0x70],rax
            m.w64(r13.wrapping_add(0x70), (rax) as u64);
            // 22dab mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 22db2 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22dba mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 22dc2 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22dc9 mov rdi,r14
            rdi = r14;
            // 22dcc call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22dd1 movups xmm0,XMMWORD PTR [r12+0x78]
            x0 = m.r128(r12.wrapping_add(0x78));
            // 22dd7 movups XMMWORD PTR [r13+0x78],xmm0
            m.w128(r13.wrapping_add(0x78), x0);
            // 22ddc mov rax,QWORD PTR [r12+0x88]
            rax = m.r64(r12.wrapping_add(0x88)) as u64;
            // 22de4 mov QWORD PTR [r13+0x88],rax
            m.w64(r13.wrapping_add(0x88), (rax) as u64);
            // 22deb mov r14,QWORD PTR [rbp-0x158]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 22df2 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 22dfa mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 22e02 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 22e09 mov r14,r13
            r14 = r13;
            // 22e0c mov rdi,QWORD PTR [rbp-0x1a0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 22e13 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22e18 movups xmm0,XMMWORD PTR [r12+0x90]
            x0 = m.r128(r12.wrapping_add(0x90));
            // 22e21 movups XMMWORD PTR [r14+0x90],xmm0
            m.w128(r14.wrapping_add(0x90), x0);
            // 22e29 mov rax,QWORD PTR [r12+0xa0]
            rax = m.r64(r12.wrapping_add(0xa0)) as u64;
            // 22e31 mov QWORD PTR [r14+0xa0],rax
            m.w64(r14.wrapping_add(0xa0), (rax) as u64);
            // 22e38 mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 22e3f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22e47 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 22e4f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22e56 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 22e5d call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22e62 movups xmm0,XMMWORD PTR [r12+0xa8]
            x0 = m.r128(r12.wrapping_add(0xa8));
            // 22e6b movups XMMWORD PTR [r14+0xa8],xmm0
            m.w128(r14.wrapping_add(0xa8), x0);
            // 22e73 mov rax,QWORD PTR [r12+0xb8]
            rax = m.r64(r12.wrapping_add(0xb8)) as u64;
            // 22e7b mov QWORD PTR [r14+0xb8],rax
            m.w64(r14.wrapping_add(0xb8), (rax) as u64);
            // 22e82 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 22e89 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22e91 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 22e99 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22ea0 mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 22ea7 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22eac movups xmm0,XMMWORD PTR [r12+0xc0]
            x0 = m.r128(r12.wrapping_add(0xc0));
            // 22eb5 movups XMMWORD PTR [r14+0xc0],xmm0
            m.w128(r14.wrapping_add(0xc0), x0);
            // 22ebd mov rax,QWORD PTR [r12+0xd0]
            rax = m.r64(r12.wrapping_add(0xd0)) as u64;
            // 22ec5 mov QWORD PTR [r14+0xd0],rax
            m.w64(r14.wrapping_add(0xd0), (rax) as u64);
            // 22ecc mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 22ed3 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 22edb mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 22ee3 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 22eea mov rcx,QWORD PTR [rbp-0x180]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 22ef1 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 22ef4 mov rdx,QWORD PTR [rbp-0x168]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 22efb mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 22efe mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 22f02 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 22f06 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 22f09 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 22f0d mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 22f11 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 22f14 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 22f1b mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 22f20 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 22f27 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 22f2e mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 22f33 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 22f37 test BYTE PTR [r12+0x18],0x1
            fl = logic::<8>(m.r8(r12.wrapping_add(0x18)) as u64 & 0x1u64);
            // 22f3d jne 0x22f47
            if fl.ne() { pc = 0x22f47; continue; }
            pc = 0x22f3f;
        }
        0x22f3f => {
            // block 22f3f
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22f3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22f3f mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 22f45 jmp 0x22f58
            pc = 0x22f58; continue;
        }
        0x22f47 => {
            // block 22f47
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22f47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22f47 mov rax,QWORD PTR [r12+0x28]
            rax = m.r64(r12.wrapping_add(0x28)) as u64;
            // 22f4c mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 22f4f mov QWORD PTR [r12+0x20],0x0
            m.w64(r12.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x22f58;
        }
        0x22f58 => {
            // block 22f58
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 22f58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 22f58 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 22f5a mov rdi,r15
            rdi = r15;
            // 22f5d call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22f62 lea r13,[rbp-0x108]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 22f69 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 22f6d mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 22f71 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 22f75 mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 22f79 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 22f7d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 22f80 mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 22f88 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 22f90 mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 22f98 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 22f9f mov rcx,rax
            rcx = rax;
            // 22fa2 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 22fa6 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 22fad mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 22fb1 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 22fb4 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 22fb7 mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 22fbb mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 22fbf mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 22fc3 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 22fc7 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 22fcb mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 22fcf mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 22fd2 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 22fd6 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 22fda mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 22fdd mov rdi,QWORD PTR [rbp-0x188]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 22fe4 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 22fe9 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 22ff0 movups XMMWORD PTR [r12+0x60],xmm0
            m.w128(r12.wrapping_add(0x60), x0);
            // 22ff6 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 22ffd mov QWORD PTR [r12+0x70],rax
            m.w64(r12.wrapping_add(0x70), (rax) as u64);
            // 23002 lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 23009 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23011 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23019 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23020 mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 23027 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2302c movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 23033 movups XMMWORD PTR [r12+0x78],xmm0
            m.w128(r12.wrapping_add(0x78), x0);
            // 23039 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 23040 mov QWORD PTR [r12+0x88],rax
            m.w64(r12.wrapping_add(0x88), (rax) as u64);
            // 23048 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 2304f mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23057 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2305f mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23066 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 2306d call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23072 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 23079 movups XMMWORD PTR [r12+0x90],xmm0
            m.w128(r12.wrapping_add(0x90), x0);
            // 23082 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 23086 mov QWORD PTR [r12+0xa0],rax
            m.w64(r12.wrapping_add(0xa0), (rax) as u64);
            // 2308e lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 23095 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2309d mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 230a5 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 230ac mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 230b3 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 230b8 lea r15,[rbp-0x78]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 230bc movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 230c0 movups XMMWORD PTR [r12+0xa8],xmm0
            m.w128(r12.wrapping_add(0xa8), x0);
            // 230c9 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 230cd mov QWORD PTR [r12+0xb8],rax
            m.w64(r12.wrapping_add(0xb8), (rax) as u64);
            // 230d5 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 230d9 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 230e1 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 230e9 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 230f0 mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 230f7 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 230fc lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 23100 movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 23104 movups XMMWORD PTR [r12+0xc0],xmm0
            m.w128(r12.wrapping_add(0xc0), x0);
            // 2310d mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 23111 mov QWORD PTR [r12+0xd0],rax
            m.w64(r12.wrapping_add(0xd0), (rax) as u64);
            // 23119 lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2311d mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23125 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2312d mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23134 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 23138 mov rcx,rax
            rcx = rax;
            // 2313b mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 2313e mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 23145 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 23148 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 2314c mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 23150 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 23153 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23157 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 2315b mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 2315e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23163 mov rdi,r15
            rdi = r15;
            // 23166 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2316b lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 23172 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23177 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 2317e call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23183 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 2318a call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2318f mov rdi,r13
            rdi = r13;
            // 23192 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23197 mov rdi,r14
            rdi = r14;
            // 2319a mov rsi,rbx
            rsi = rbx;
            // 2319d mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 231a4 call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 231a6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 231a8 je 0x233e3
            if fl.e() { pc = 0x233e3; continue; }
            pc = 0x231ae;
        }
        0x231ae => {
            // block 231ae
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 231ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 231ae mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 231b2 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 231b9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 231bc mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 231c0 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 231c7 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 231ce lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // 231d2 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 231d6 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 231dd mov rax,QWORD PTR [rbx+0x18]
            rax = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 231e1 mov rcx,QWORD PTR [rbx+0x20]
            rcx = m.r64(rbx.wrapping_add(0x20)) as u64;
            // 231e5 mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 231ec mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 231f3 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 231fb mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 23203 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 2320b mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 2320f mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 23216 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 23219 mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 2321f mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 23223 mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 2322a mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 2322e mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 23235 mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 23239 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 23240 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 23244 mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 23248 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 2324f mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 23256 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 23261 mov rax,QWORD PTR [rbx+0x60]
            rax = m.r64(rbx.wrapping_add(0x60)) as u64;
            // 23265 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 2326c movups xmm0,XMMWORD PTR [rbx+0x68]
            x0 = m.r128(rbx.wrapping_add(0x68));
            // 23270 movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 23277 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 2327f mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 23287 mov QWORD PTR [rbx+0x60],0x0
            m.w64(rbx.wrapping_add(0x60), (0x0u64) as u64);
            // 2328f mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 2329a movups xmm0,XMMWORD PTR [rbx+0x78]
            x0 = m.r128(rbx.wrapping_add(0x78));
            // 2329e movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 232a5 mov rax,QWORD PTR [rbx+0x88]
            rax = m.r64(rbx.wrapping_add(0x88)) as u64;
            // 232ac mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 232b3 mov QWORD PTR [rbx+0x88],0x0
            m.w64(rbx.wrapping_add(0x88), (0x0u64) as u64);
            // 232be mov QWORD PTR [rbx+0x80],0x0
            m.w64(rbx.wrapping_add(0x80), (0x0u64) as u64);
            // 232c9 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 232d1 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 232dc movups xmm0,XMMWORD PTR [rbx+0x90]
            x0 = m.r128(rbx.wrapping_add(0x90));
            // 232e3 movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 232ea mov rax,QWORD PTR [rbx+0xa0]
            rax = m.r64(rbx.wrapping_add(0xa0)) as u64;
            // 232f1 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 232f5 mov QWORD PTR [rbx+0xa0],0x0
            m.w64(rbx.wrapping_add(0xa0), (0x0u64) as u64);
            // 23300 mov QWORD PTR [rbx+0x98],0x0
            m.w64(rbx.wrapping_add(0x98), (0x0u64) as u64);
            // 2330b mov QWORD PTR [rbx+0x90],0x0
            m.w64(rbx.wrapping_add(0x90), (0x0u64) as u64);
            // 23316 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 2331e movups xmm0,XMMWORD PTR [rbx+0xa8]
            x0 = m.r128(rbx.wrapping_add(0xa8));
            // 23325 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 23329 mov rax,QWORD PTR [rbx+0xb8]
            rax = m.r64(rbx.wrapping_add(0xb8)) as u64;
            // 23330 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 23334 mov QWORD PTR [rbx+0xb8],0x0
            m.w64(rbx.wrapping_add(0xb8), (0x0u64) as u64);
            // 2333f mov QWORD PTR [rbx+0xb0],0x0
            m.w64(rbx.wrapping_add(0xb0), (0x0u64) as u64);
            // 2334a mov QWORD PTR [rbx+0xa8],0x0
            m.w64(rbx.wrapping_add(0xa8), (0x0u64) as u64);
            // 23355 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 2335d mov rax,QWORD PTR [rbx+0xc0]
            rax = m.r64(rbx.wrapping_add(0xc0)) as u64;
            // 23364 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 23368 movups xmm0,XMMWORD PTR [rbx+0xc8]
            x0 = m.r128(rbx.wrapping_add(0xc8));
            // 2336f movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 23373 mov QWORD PTR [rbx+0xd0],0x0
            m.w64(rbx.wrapping_add(0xd0), (0x0u64) as u64);
            // 2337e mov QWORD PTR [rbx+0xc8],0x0
            m.w64(rbx.wrapping_add(0xc8), (0x0u64) as u64);
            // 23389 mov QWORD PTR [rbx+0xc0],0x0
            m.w64(rbx.wrapping_add(0xc0), (0x0u64) as u64);
            // 23394 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 2339a mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 2339d mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 233a4 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 233a8 mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 233af mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 233b6 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 233ba mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 233be mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 233c2 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 233c6 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 233c9 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 233cd mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 233d1 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 233d4 test BYTE PTR [rbx+0x18],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x18)) as u64 & 0x1u64);
            // 233d8 jne 0x233f3
            if fl.ne() { pc = 0x233f3; continue; }
            pc = 0x233da;
        }
        0x233da => {
            // block 233da
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 233da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 233da mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // 233e1 jmp 0x23402
            pc = 0x23402; continue;
        }
        0x233e3 => {
            // block 233e3
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 233e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 233e3 mov r14d,DWORD PTR [rbp-0x124]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 233ea add r14d,0x2
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x2u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 233ee jmp 0x24006
            pc = 0x24006; continue;
        }
        0x233f3 => {
            // block 233f3
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 233f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 233f3 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 233f7 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 233fa mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x23402;
        }
        0x23402 => {
            // block 23402
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23402 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23402 mov r15,QWORD PTR [rbp-0x1c0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe40)) as u64;
            // 23409 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2340b mov rdi,r12
            rdi = r12;
            // 2340e call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23413 lea rsi,[rbx+0x30]
            rsi = rbx.wrapping_add(0x30);
            // 23417 mov QWORD PTR [rbp-0x190],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (rsi) as u64);
            // 2341e lea rdi,[rbx+0x60]
            rdi = rbx.wrapping_add(0x60);
            // 23422 mov QWORD PTR [rbp-0x188],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rdi) as u64);
            // 23429 lea rax,[rbx+0x78]
            rax = rbx.wrapping_add(0x78);
            // 2342d mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // 23434 lea rax,[rbx+0x90]
            rax = rbx.wrapping_add(0x90);
            // 2343b mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 23442 lea rax,[rbx+0xa8]
            rax = rbx.wrapping_add(0xa8);
            // 23449 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // 23450 lea rax,[rbx+0xc0]
            rax = rbx.wrapping_add(0xc0);
            // 23457 mov QWORD PTR [rbp-0x140],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rax) as u64);
            // 2345e lea r13,[rbx+0xd8]
            r13 = rbx.wrapping_add(0xd8);
            // 23465 mov QWORD PTR [rbp-0x180],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (r13) as u64);
            // 2346c mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 23470 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 23475 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 23478 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2347c mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 23481 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 23485 mov QWORD PTR [rbp-0x198],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe68), (r12) as u64);
            // 2348c mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // 23494 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 2349c mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 234a3 mov r12,QWORD PTR [rbp-0x1c8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // 234aa mov ax,WORD PTR [r12+0x2c]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 234b0 mov WORD PTR [rsi+0x2c],ax
            m.w16(rsi.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 234b4 mov eax,DWORD PTR [r12+0x28]
            rax = (m.r32(r12.wrapping_add(0x28)) as u64) as u32 as u64;
            // 234b9 mov DWORD PTR [rsi+0x28],eax
            m.w32(rsi.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 234bc mov rax,QWORD PTR [r12+0x20]
            rax = m.r64(r12.wrapping_add(0x20)) as u64;
            // 234c1 mov QWORD PTR [rsi+0x20],rax
            m.w64(rsi.wrapping_add(0x20), (rax) as u64);
            // 234c5 mov rax,QWORD PTR [r12+0x18]
            rax = m.r64(r12.wrapping_add(0x18)) as u64;
            // 234ca mov QWORD PTR [rsi+0x18],rax
            m.w64(rsi.wrapping_add(0x18), (rax) as u64);
            // 234ce mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // 234d3 mov QWORD PTR [rsi+0x10],rax
            m.w64(rsi.wrapping_add(0x10), (rax) as u64);
            // 234d7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 234db mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 234e0 mov QWORD PTR [rsi+0x8],rcx
            m.w64(rsi.wrapping_add(0x8), (rcx) as u64);
            // 234e4 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 234e7 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 234ec movups xmm0,XMMWORD PTR [r14+0x60]
            x0 = m.r128(r14.wrapping_add(0x60));
            // 234f1 movups XMMWORD PTR [rbx+0x60],xmm0
            m.w128(rbx.wrapping_add(0x60), x0);
            // 234f5 mov rax,QWORD PTR [r14+0x70]
            rax = m.r64(r14.wrapping_add(0x70)) as u64;
            // 234f9 mov QWORD PTR [rbx+0x70],rax
            m.w64(rbx.wrapping_add(0x70), (rax) as u64);
            // 234fd mov rax,QWORD PTR [rbp-0x1b8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 23504 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2350c mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23514 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2351b mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 23522 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23527 movups xmm0,XMMWORD PTR [r14+0x78]
            x0 = m.r128(r14.wrapping_add(0x78));
            // 2352c movups XMMWORD PTR [rbx+0x78],xmm0
            m.w128(rbx.wrapping_add(0x78), x0);
            // 23530 mov rax,QWORD PTR [r14+0x88]
            rax = m.r64(r14.wrapping_add(0x88)) as u64;
            // 23537 mov QWORD PTR [rbx+0x88],rax
            m.w64(rbx.wrapping_add(0x88), (rax) as u64);
            // 2353e mov rax,QWORD PTR [rbp-0x1b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 23545 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2354d mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23555 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2355c mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 23563 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23568 movups xmm0,XMMWORD PTR [r14+0x90]
            x0 = m.r128(r14.wrapping_add(0x90));
            // 23570 movups XMMWORD PTR [rbx+0x90],xmm0
            m.w128(rbx.wrapping_add(0x90), x0);
            // 23577 mov rax,QWORD PTR [r14+0xa0]
            rax = m.r64(r14.wrapping_add(0xa0)) as u64;
            // 2357e mov QWORD PTR [rbx+0xa0],rax
            m.w64(rbx.wrapping_add(0xa0), (rax) as u64);
            // 23585 mov rax,QWORD PTR [rbp-0x1a0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 2358c mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23594 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2359c mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 235a3 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 235aa call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 235af movups xmm0,XMMWORD PTR [r14+0xa8]
            x0 = m.r128(r14.wrapping_add(0xa8));
            // 235b7 movups XMMWORD PTR [rbx+0xa8],xmm0
            m.w128(rbx.wrapping_add(0xa8), x0);
            // 235be mov rax,QWORD PTR [r14+0xb8]
            rax = m.r64(r14.wrapping_add(0xb8)) as u64;
            // 235c5 mov QWORD PTR [rbx+0xb8],rax
            m.w64(rbx.wrapping_add(0xb8), (rax) as u64);
            // 235cc mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 235d3 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 235db mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 235e3 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 235ea mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 235f1 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 235f6 movups xmm0,XMMWORD PTR [r14+0xc0]
            x0 = m.r128(r14.wrapping_add(0xc0));
            // 235fe movups XMMWORD PTR [rbx+0xc0],xmm0
            m.w128(rbx.wrapping_add(0xc0), x0);
            // 23605 mov rax,QWORD PTR [r14+0xd0]
            rax = m.r64(r14.wrapping_add(0xd0)) as u64;
            // 2360c mov QWORD PTR [rbx+0xd0],rax
            m.w64(rbx.wrapping_add(0xd0), (rax) as u64);
            // 23613 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 2361a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23622 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2362a mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23631 mov rcx,QWORD PTR [rbp-0x168]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 23638 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 2363b mov DWORD PTR [r13+0x18],eax
            m.w32(r13.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 2363f mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23643 mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 23647 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 2364a mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 2364e mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 23652 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 23656 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 2365d mov QWORD PTR [r14+0x10],rax
            m.w64(r14.wrapping_add(0x10), (rax) as u64);
            // 23661 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 23668 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 2366f mov QWORD PTR [r14+0x8],rcx
            m.w64(r14.wrapping_add(0x8), (rcx) as u64);
            // 23673 mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 23676 test BYTE PTR [r14+0x18],0x1
            fl = logic::<8>(m.r8(r14.wrapping_add(0x18)) as u64 & 0x1u64);
            // 2367b jne 0x23685
            if fl.ne() { pc = 0x23685; continue; }
            pc = 0x2367d;
        }
        0x2367d => {
            // block 2367d
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 2367d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2367d mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // 23683 jmp 0x23694
            pc = 0x23694; continue;
        }
        0x23685 => {
            // block 23685
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23685 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23685 mov rax,QWORD PTR [r14+0x28]
            rax = m.r64(r14.wrapping_add(0x28)) as u64;
            // 23689 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 2368c mov QWORD PTR [r14+0x20],0x0
            m.w64(r14.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x23694;
        }
        0x23694 => {
            // block 23694
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23694 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23694 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 23696 mov rdi,r15
            rdi = r15;
            // 23699 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2369e lea r13,[rbp-0x108]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 236a5 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 236a9 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 236ad mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 236b1 mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 236b5 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 236b9 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 236bc mov QWORD PTR [r13+0x10],0x0
            m.w64(r13.wrapping_add(0x10), (0x0u64) as u64);
            // 236c4 mov QWORD PTR [r13+0x8],0x0
            m.w64(r13.wrapping_add(0x8), (0x0u64) as u64);
            // 236cc mov QWORD PTR [r13+0x0],0x0
            m.w64(r13, (0x0u64) as u64);
            // 236d4 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 236db mov rcx,rax
            rcx = rax;
            // 236de mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 236e2 mov WORD PTR [r12+0x2c],ax
            m.w16(r12.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 236e8 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 236eb mov DWORD PTR [r12+0x28],eax
            m.w32(r12.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 236f0 mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 236f4 mov QWORD PTR [r12+0x20],rax
            m.w64(r12.wrapping_add(0x20), (rax) as u64);
            // 236f9 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 236fd mov QWORD PTR [r12+0x18],rax
            m.w64(r12.wrapping_add(0x18), (rax) as u64);
            // 23702 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23706 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2370b mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 2370e mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23712 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 23717 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 2371b mov rdi,QWORD PTR [rbp-0x1b8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe48)) as u64;
            // 23722 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23727 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 2372e movups XMMWORD PTR [r14+0x60],xmm0
            m.w128(r14.wrapping_add(0x60), x0);
            // 23733 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 2373a mov QWORD PTR [r14+0x70],rax
            m.w64(r14.wrapping_add(0x70), (rax) as u64);
            // 2373e lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 23745 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2374d mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23755 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2375c mov rdi,QWORD PTR [rbp-0x1b0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe50)) as u64;
            // 23763 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23768 movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 2376f movups XMMWORD PTR [r14+0x78],xmm0
            m.w128(r14.wrapping_add(0x78), x0);
            // 23774 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 2377b mov QWORD PTR [r14+0x88],rax
            m.w64(r14.wrapping_add(0x88), (rax) as u64);
            // 23782 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 23789 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23791 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23799 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 237a0 mov rdi,QWORD PTR [rbp-0x1a0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 237a7 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 237ac lea r15,[rbp-0x90]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 237b3 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 237ba movups XMMWORD PTR [r14+0x90],xmm0
            m.w128(r14.wrapping_add(0x90), x0);
            // 237c2 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 237c6 mov QWORD PTR [r14+0xa0],rax
            m.w64(r14.wrapping_add(0xa0), (rax) as u64);
            // 237cd lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 237d4 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 237dc mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 237e4 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 237eb mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 237f2 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 237f7 lea rbx,[rbp-0x78]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 237fb movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 237ff movups XMMWORD PTR [r14+0xa8],xmm0
            m.w128(r14.wrapping_add(0xa8), x0);
            // 23807 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 2380b mov QWORD PTR [r14+0xb8],rax
            m.w64(r14.wrapping_add(0xb8), (rax) as u64);
            // 23812 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 23816 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2381e mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23826 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2382d mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 23834 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23839 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2383d movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 23841 movups XMMWORD PTR [r14+0xc0],xmm0
            m.w128(r14.wrapping_add(0xc0), x0);
            // 23849 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 2384d mov QWORD PTR [r14+0xd0],rax
            m.w64(r14.wrapping_add(0xd0), (rax) as u64);
            // 23854 lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 23858 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23860 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23868 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2386f lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 23873 mov rcx,rax
            rcx = rax;
            // 23876 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 23879 mov r12,QWORD PTR [rbp-0x168]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 23880 mov DWORD PTR [r12+0x18],eax
            m.w32(r12.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 23885 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23889 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 2388e mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 23891 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23895 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 2389a mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 2389e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238a3 mov rdi,rbx
            rdi = rbx;
            // 238a6 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238ab mov rdi,r15
            rdi = r15;
            // 238ae call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238b3 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 238ba call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238bf lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 238c6 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238cb mov rdi,r13
            rdi = r13;
            // 238ce call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238d3 mov rbx,QWORD PTR [rbp-0x160]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 238da mov rdi,rbx
            rdi = rbx;
            // 238dd mov r15,QWORD PTR [rbp-0x1a8]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe58)) as u64;
            // 238e4 mov rsi,r15
            rsi = r15;
            // 238e7 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 238ee call QWORD PTR [rax]
            { let r = { let t = m.r64(rax) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 238f0 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 238f2 je 0x23b35
            if fl.e() { pc = 0x23b35; continue; }
            pc = 0x238f8;
        }
        0x238f8 => {
            // block 238f8
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 238f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 238f8 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 238fc mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 23903 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 23906 mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2390a mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 23911 mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 23918 lea r14,[r15+0x18]
            r14 = r15.wrapping_add(0x18);
            // 2391c mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 23920 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // 23927 mov rax,QWORD PTR [r15+0x18]
            rax = m.r64(r15.wrapping_add(0x18)) as u64;
            // 2392b mov rcx,QWORD PTR [r15+0x20]
            rcx = m.r64(r15.wrapping_add(0x20)) as u64;
            // 2392f mov QWORD PTR [rbp-0x100],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (rcx) as u64);
            // 23936 mov QWORD PTR [rbp-0x108],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (rax) as u64);
            // 2393d mov QWORD PTR [r15+0x28],0x0
            m.w64(r15.wrapping_add(0x28), (0x0u64) as u64);
            // 23945 mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // 2394d mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // 23955 mov ax,WORD PTR [r15+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 2395a mov WORD PTR [rbp-0xc4],ax
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff3c), ((rax as u16 as u64)) as u16);
            // 23961 mov eax,DWORD PTR [r15+0x58]
            rax = (m.r32(r15.wrapping_add(0x58)) as u64) as u32 as u64;
            // 23965 mov DWORD PTR [rbp-0xc8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff38), ((rax as u32 as u64)) as u32);
            // 2396b mov rax,QWORD PTR [r15+0x50]
            rax = m.r64(r15.wrapping_add(0x50)) as u64;
            // 2396f mov QWORD PTR [rbp-0xd0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff30), (rax) as u64);
            // 23976 mov rax,QWORD PTR [r15+0x48]
            rax = m.r64(r15.wrapping_add(0x48)) as u64;
            // 2397a mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // 23981 mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // 23985 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 2398c mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 23990 mov rcx,QWORD PTR [r15+0x38]
            rcx = m.r64(r15.wrapping_add(0x38)) as u64;
            // 23994 mov QWORD PTR [rbp-0xe8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rcx) as u64);
            // 2399b mov QWORD PTR [rbp-0xf0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (rax) as u64);
            // 239a2 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // 239ad mov rax,QWORD PTR [r15+0x60]
            rax = m.r64(r15.wrapping_add(0x60)) as u64;
            // 239b1 mov QWORD PTR [rbp-0xc0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rax) as u64);
            // 239b8 movups xmm0,XMMWORD PTR [r15+0x68]
            x0 = m.r128(r15.wrapping_add(0x68));
            // 239bd movups XMMWORD PTR [rbp-0xb8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff48), x0);
            // 239c4 mov QWORD PTR [r15+0x70],0x0
            m.w64(r15.wrapping_add(0x70), (0x0u64) as u64);
            // 239cc mov QWORD PTR [r15+0x68],0x0
            m.w64(r15.wrapping_add(0x68), (0x0u64) as u64);
            // 239d4 mov QWORD PTR [r15+0x60],0x0
            m.w64(r15.wrapping_add(0x60), (0x0u64) as u64);
            // 239dc mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 239e7 movups xmm0,XMMWORD PTR [r15+0x78]
            x0 = m.r128(r15.wrapping_add(0x78));
            // 239ec movups XMMWORD PTR [rbp-0xa8],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff58), x0);
            // 239f3 mov rax,QWORD PTR [r15+0x88]
            rax = m.r64(r15.wrapping_add(0x88)) as u64;
            // 239fa mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 23a01 mov QWORD PTR [r15+0x88],0x0
            m.w64(r15.wrapping_add(0x88), (0x0u64) as u64);
            // 23a0c mov QWORD PTR [r15+0x80],0x0
            m.w64(r15.wrapping_add(0x80), (0x0u64) as u64);
            // 23a17 mov QWORD PTR [r15+0x78],0x0
            m.w64(r15.wrapping_add(0x78), (0x0u64) as u64);
            // 23a1f mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 23a2a movups xmm0,XMMWORD PTR [r15+0x90]
            x0 = m.r128(r15.wrapping_add(0x90));
            // 23a32 movups XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 23a39 mov rax,QWORD PTR [r15+0xa0]
            rax = m.r64(r15.wrapping_add(0xa0)) as u64;
            // 23a40 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 23a44 mov QWORD PTR [r15+0xa0],0x0
            m.w64(r15.wrapping_add(0xa0), (0x0u64) as u64);
            // 23a4f mov QWORD PTR [r15+0x98],0x0
            m.w64(r15.wrapping_add(0x98), (0x0u64) as u64);
            // 23a5a mov QWORD PTR [r15+0x90],0x0
            m.w64(r15.wrapping_add(0x90), (0x0u64) as u64);
            // 23a65 mov QWORD PTR [rbp-0x78],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (0x0u64) as u64);
            // 23a6d movups xmm0,XMMWORD PTR [r15+0xa8]
            x0 = m.r128(r15.wrapping_add(0xa8));
            // 23a75 movups XMMWORD PTR [rbp-0x78],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff88), x0);
            // 23a79 mov rax,QWORD PTR [r15+0xb8]
            rax = m.r64(r15.wrapping_add(0xb8)) as u64;
            // 23a80 mov QWORD PTR [rbp-0x68],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rax) as u64);
            // 23a84 mov QWORD PTR [r15+0xb8],0x0
            m.w64(r15.wrapping_add(0xb8), (0x0u64) as u64);
            // 23a8f mov QWORD PTR [r15+0xb0],0x0
            m.w64(r15.wrapping_add(0xb0), (0x0u64) as u64);
            // 23a9a mov QWORD PTR [r15+0xa8],0x0
            m.w64(r15.wrapping_add(0xa8), (0x0u64) as u64);
            // 23aa5 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 23aad mov rax,QWORD PTR [r15+0xc0]
            rax = m.r64(r15.wrapping_add(0xc0)) as u64;
            // 23ab4 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 23ab8 movups xmm0,XMMWORD PTR [r15+0xc8]
            x0 = m.r128(r15.wrapping_add(0xc8));
            // 23ac0 movups XMMWORD PTR [rbp-0x58],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), x0);
            // 23ac4 mov QWORD PTR [r15+0xd0],0x0
            m.w64(r15.wrapping_add(0xd0), (0x0u64) as u64);
            // 23acf mov QWORD PTR [r15+0xc8],0x0
            m.w64(r15.wrapping_add(0xc8), (0x0u64) as u64);
            // 23ada mov QWORD PTR [r15+0xc0],0x0
            m.w64(r15.wrapping_add(0xc0), (0x0u64) as u64);
            // 23ae5 mov eax,DWORD PTR [r15+0xf0]
            rax = (m.r32(r15.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 23aec mov DWORD PTR [rbp-0x30],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rax as u32 as u64)) as u32);
            // 23aef mov rax,QWORD PTR [r15+0xe8]
            rax = m.r64(r15.wrapping_add(0xe8)) as u64;
            // 23af6 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 23afa mov rax,QWORD PTR [r15+0xd8]
            rax = m.r64(r15.wrapping_add(0xd8)) as u64;
            // 23b01 mov rcx,QWORD PTR [r15+0xe0]
            rcx = m.r64(r15.wrapping_add(0xe0)) as u64;
            // 23b08 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 23b0c mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 23b10 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 23b14 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 23b18 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 23b1b mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 23b1f mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 23b23 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 23b26 test BYTE PTR [r15+0x18],0x1
            fl = logic::<8>(m.r8(r15.wrapping_add(0x18)) as u64 & 0x1u64);
            // 23b2b jne 0x23b45
            if fl.ne() { pc = 0x23b45; continue; }
            pc = 0x23b2d;
        }
        0x23b2d => {
            // block 23b2d
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23b2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23b2d mov WORD PTR [r14],0x0
            m.w16(r14, (0x0u64) as u16);
            // 23b33 jmp 0x23b54
            pc = 0x23b54; continue;
        }
        0x23b35 => {
            // block 23b35
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23b35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23b35 mov r14d,DWORD PTR [rbp-0x124]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 23b3c add r14d,0x3
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x3u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 23b40 jmp 0x24006
            pc = 0x24006; continue;
        }
        0x23b45 => {
            // block 23b45
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23b45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23b45 mov rax,QWORD PTR [r15+0x28]
            rax = m.r64(r15.wrapping_add(0x28)) as u64;
            // 23b49 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 23b4c mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x23b54;
        }
        0x23b54 => {
            // block 23b54
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23b54 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 23b56 mov rdi,r14
            rdi = r14;
            // 23b59 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23b5e lea rax,[r15+0x30]
            rax = r15.wrapping_add(0x30);
            // 23b62 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 23b66 lea rcx,[r15+0x78]
            rcx = r15.wrapping_add(0x78);
            // 23b6a mov QWORD PTR [rbp-0x170],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rcx) as u64);
            // 23b71 lea rcx,[r15+0x90]
            rcx = r15.wrapping_add(0x90);
            // 23b78 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // 23b7f lea rcx,[r15+0xa8]
            rcx = r15.wrapping_add(0xa8);
            // 23b86 mov QWORD PTR [rbp-0x138],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rcx) as u64);
            // 23b8d lea rcx,[r15+0xc0]
            rcx = r15.wrapping_add(0xc0);
            // 23b94 mov QWORD PTR [rbp-0x130],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (rcx) as u64);
            // 23b9b lea r13,[r15+0xd8]
            r13 = r15.wrapping_add(0xd8);
            // 23ba2 mov r12,QWORD PTR [rbp-0x198]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 23ba9 mov rcx,QWORD PTR [r12+0x10]
            rcx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 23bae mov QWORD PTR [r14+0x10],rcx
            m.w64(r14.wrapping_add(0x10), (rcx) as u64);
            // 23bb2 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // 23bb6 mov rdx,QWORD PTR [r12+0x8]
            rdx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 23bbb mov QWORD PTR [r14+0x8],rdx
            m.w64(r14.wrapping_add(0x8), (rdx) as u64);
            // 23bbf mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 23bc2 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 23bcb mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 23bd4 mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // 23bdc mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 23be3 mov cx,WORD PTR [rdx+0x2c]
            rcx = (rcx & !0xffff) | ((m.r16(rdx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 23be7 mov WORD PTR [rax+0x2c],cx
            m.w16(rax.wrapping_add(0x2c), ((rcx as u16 as u64)) as u16);
            // 23beb mov ecx,DWORD PTR [rdx+0x28]
            rcx = (m.r32(rdx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 23bee mov DWORD PTR [rax+0x28],ecx
            m.w32(rax.wrapping_add(0x28), ((rcx as u32 as u64)) as u32);
            // 23bf1 mov rcx,QWORD PTR [rdx+0x20]
            rcx = m.r64(rdx.wrapping_add(0x20)) as u64;
            // 23bf5 mov QWORD PTR [rax+0x20],rcx
            m.w64(rax.wrapping_add(0x20), (rcx) as u64);
            // 23bf9 mov rcx,QWORD PTR [rdx+0x18]
            rcx = m.r64(rdx.wrapping_add(0x18)) as u64;
            // 23bfd mov QWORD PTR [rax+0x18],rcx
            m.w64(rax.wrapping_add(0x18), (rcx) as u64);
            // 23c01 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 23c05 mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 23c09 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            // 23c0c mov rdx,QWORD PTR [rdx+0x8]
            rdx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 23c10 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 23c14 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 23c17 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23c1c movups xmm0,XMMWORD PTR [rbx+0x60]
            x0 = m.r128(rbx.wrapping_add(0x60));
            // 23c20 movups XMMWORD PTR [r15+0x60],xmm0
            m.w128(r15.wrapping_add(0x60), x0);
            // 23c25 mov rax,QWORD PTR [rbx+0x70]
            rax = m.r64(rbx.wrapping_add(0x70)) as u64;
            // 23c29 mov QWORD PTR [r15+0x70],rax
            m.w64(r15.wrapping_add(0x70), (rax) as u64);
            // 23c2d mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 23c34 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23c3c mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23c44 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23c4b mov rdi,QWORD PTR [rbp-0x170]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 23c52 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23c57 movups xmm0,XMMWORD PTR [rbx+0x78]
            x0 = m.r128(rbx.wrapping_add(0x78));
            // 23c5b movups XMMWORD PTR [r15+0x78],xmm0
            m.w128(r15.wrapping_add(0x78), x0);
            // 23c60 mov rax,QWORD PTR [rbx+0x88]
            rax = m.r64(rbx.wrapping_add(0x88)) as u64;
            // 23c67 mov QWORD PTR [r15+0x88],rax
            m.w64(r15.wrapping_add(0x88), (rax) as u64);
            // 23c6e mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 23c75 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23c7d mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23c85 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23c8c mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 23c93 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23c98 movups xmm0,XMMWORD PTR [rbx+0x90]
            x0 = m.r128(rbx.wrapping_add(0x90));
            // 23c9f movups XMMWORD PTR [r15+0x90],xmm0
            m.w128(r15.wrapping_add(0x90), x0);
            // 23ca7 mov rax,QWORD PTR [rbx+0xa0]
            rax = m.r64(rbx.wrapping_add(0xa0)) as u64;
            // 23cae mov QWORD PTR [r15+0xa0],rax
            m.w64(r15.wrapping_add(0xa0), (rax) as u64);
            // 23cb5 mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 23cbc mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23cc4 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23ccc mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23cd3 mov rdi,QWORD PTR [rbp-0x138]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 23cda call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23cdf movups xmm0,XMMWORD PTR [rbx+0xa8]
            x0 = m.r128(rbx.wrapping_add(0xa8));
            // 23ce6 movups XMMWORD PTR [r15+0xa8],xmm0
            m.w128(r15.wrapping_add(0xa8), x0);
            // 23cee mov rax,QWORD PTR [rbx+0xb8]
            rax = m.r64(rbx.wrapping_add(0xb8)) as u64;
            // 23cf5 mov QWORD PTR [r15+0xb8],rax
            m.w64(r15.wrapping_add(0xb8), (rax) as u64);
            // 23cfc mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 23d03 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23d0b mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23d13 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23d1a mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 23d21 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23d26 movups xmm0,XMMWORD PTR [rbx+0xc0]
            x0 = m.r128(rbx.wrapping_add(0xc0));
            // 23d2d movups XMMWORD PTR [r15+0xc0],xmm0
            m.w128(r15.wrapping_add(0xc0), x0);
            // 23d35 mov rax,QWORD PTR [rbx+0xd0]
            rax = m.r64(rbx.wrapping_add(0xd0)) as u64;
            // 23d3c mov QWORD PTR [r15+0xd0],rax
            m.w64(r15.wrapping_add(0xd0), (rax) as u64);
            // 23d43 mov rax,QWORD PTR [rbp-0x140]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 23d4a mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23d52 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23d5a mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23d61 mov rcx,QWORD PTR [rbp-0x180]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 23d68 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 23d6b mov DWORD PTR [r13+0x18],eax
            m.w32(r13.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 23d6f mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23d73 mov QWORD PTR [r13+0x10],rax
            m.w64(r13.wrapping_add(0x10), (rax) as u64);
            // 23d77 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 23d7a mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23d7e mov QWORD PTR [r13+0x8],rcx
            m.w64(r13.wrapping_add(0x8), (rcx) as u64);
            // 23d82 mov QWORD PTR [r13+0x0],rax
            m.w64(r13, (rax) as u64);
            // 23d86 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 23d8d mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // 23d91 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 23d98 mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 23d9f mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // 23da3 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 23da6 test BYTE PTR [rbx+0x18],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x18)) as u64 & 0x1u64);
            // 23daa jne 0x23db5
            if fl.ne() { pc = 0x23db5; continue; }
            pc = 0x23dac;
        }
        0x23dac => {
            // block 23dac
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23dac mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // 23db3 jmp 0x23dc4
            pc = 0x23dc4; continue;
        }
        0x23db5 => {
            // block 23db5
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23db5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23db5 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 23db9 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // 23dbc mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            pc = 0x23dc4;
        }
        0x23dc4 => {
            // block 23dc4
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 23dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 23dc4 mov r15,r12
            r15 = r12;
            // 23dc7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 23dc9 mov rdi,r15
            rdi = r15;
            // 23dcc call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23dd1 lea r14,[rbp-0x108]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 23dd8 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // 23ddc mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 23de0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 23de3 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // 23de7 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 23deb mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 23dee mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 23df6 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 23dfe mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 23e05 lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 23e0c mov rcx,rax
            rcx = rax;
            // 23e0f mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 23e13 mov rdx,QWORD PTR [rbp-0x190]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 23e1a mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 23e1e mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 23e21 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 23e24 mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 23e28 mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 23e2c mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 23e30 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 23e34 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23e38 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 23e3c mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 23e3f mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23e43 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 23e47 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 23e4a mov rdi,QWORD PTR [rbp-0x188]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // 23e51 call 0xeba08
            { let r = { let t = m.env.stub_va(0x1d5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23e56 movups xmm0,XMMWORD PTR [rbp-0xc0]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff40));
            // 23e5d movups XMMWORD PTR [rbx+0x60],xmm0
            m.w128(rbx.wrapping_add(0x60), x0);
            // 23e61 mov rax,QWORD PTR [rbp-0xb0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 23e68 mov QWORD PTR [rbx+0x70],rax
            m.w64(rbx.wrapping_add(0x70), (rax) as u64);
            // 23e6c lea rax,[rbp-0xc0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 23e73 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23e7b mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23e83 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23e8a mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 23e91 call 0xeba50
            { let r = { let t = m.env.stub_va(0x21d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23e96 lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 23e9d movups xmm0,XMMWORD PTR [rbp-0xa8]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff58));
            // 23ea4 movups XMMWORD PTR [rbx+0x78],xmm0
            m.w128(rbx.wrapping_add(0x78), x0);
            // 23ea8 mov rax,QWORD PTR [rbp-0x98]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            // 23eaf mov QWORD PTR [rbx+0x88],rax
            m.w64(rbx.wrapping_add(0x88), (rax) as u64);
            // 23eb6 lea rax,[rbp-0xa8]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 23ebd mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23ec5 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23ecd mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23ed4 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 23edb call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23ee0 lea r12,[rbp-0x90]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 23ee7 movups xmm0,XMMWORD PTR [rbp-0x90]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff70));
            // 23eee movups XMMWORD PTR [rbx+0x90],xmm0
            m.w128(rbx.wrapping_add(0x90), x0);
            // 23ef5 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 23ef9 mov QWORD PTR [rbx+0xa0],rax
            m.w64(rbx.wrapping_add(0xa0), (rax) as u64);
            // 23f00 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 23f07 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23f0f mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23f17 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23f1e mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 23f25 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23f2a lea r13,[rbp-0x78]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 23f2e movups xmm0,XMMWORD PTR [rbp-0x78]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ff88));
            // 23f32 movups XMMWORD PTR [rbx+0xa8],xmm0
            m.w128(rbx.wrapping_add(0xa8), x0);
            // 23f39 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 23f3d mov QWORD PTR [rbx+0xb8],rax
            m.w64(rbx.wrapping_add(0xb8), (rax) as u64);
            // 23f44 lea rax,[rbp-0x78]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 23f48 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23f50 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23f58 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23f5f mov rdi,QWORD PTR [rbp-0x140]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 23f66 call 0xeb9e4
            { let r = { let t = m.env.stub_va(0x1b1); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23f6b lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 23f6f movups xmm0,XMMWORD PTR [rbp-0x60]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0));
            // 23f73 movups XMMWORD PTR [rbx+0xc0],xmm0
            m.w128(rbx.wrapping_add(0xc0), x0);
            // 23f7a mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 23f7e mov QWORD PTR [rbx+0xd0],rax
            m.w64(rbx.wrapping_add(0xd0), (rax) as u64);
            // 23f85 lea rax,[rbp-0x60]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 23f89 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 23f91 mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 23f99 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 23fa0 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 23fa4 mov rcx,rax
            rcx = rax;
            // 23fa7 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 23faa mov rdx,QWORD PTR [rbp-0x180]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // 23fb1 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 23fb4 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 23fb8 mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 23fbc mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 23fbf mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 23fc3 mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 23fc7 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 23fca call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23fcf mov rdi,r13
            rdi = r13;
            // 23fd2 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23fd7 mov rdi,r12
            rdi = r12;
            // 23fda call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23fdf mov rdi,r15
            rdi = r15;
            // 23fe2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23fe7 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 23fee call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23ff3 mov rdi,r14
            rdi = r14;
            // 23ff6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 23ffb mov r14d,DWORD PTR [rbp-0x124]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fedc)) as u64) as u32 as u64;
            // 24002 add r14d,0x4
            { let (r, f) = add::<32>((r14 as u32 as u64), 0x4u64);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x24006;
        }
        0x24006 => {
            // block 24006
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 24006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 24006 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 24009 add rsp,0x1a8
            { let (r, f) = add::<64>(rsp, 0x1a8u64);
              rsp = r;
              fl = f; }
            // 24010 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 24011 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 24013 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 24015 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 24017 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 24019 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2401a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2401b => {
            // block 2401b
            #[cfg(feature = "trace")] eprintln!("f_0002224e pc 2401b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2401b mov rdi,rax
            rdi = rax;
            // 2401e call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 24023 mov rdi,rax
            rdi = rax;
            // 24026 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2402b mov rdi,rax
            rdi = rax;
            // 2402e call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 24033 mov rdi,rax
            rdi = rax;
            // 24036 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2403b mov rdi,rax
            rdi = rax;
            // 2403e call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 24043 mov rdi,rax
            rdi = rax;
            // 24046 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2404b mov rdi,rax
            rdi = rax;
            // 2404e call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 24053 mov rdi,rax
            rdi = rax;
            // 24056 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
