//! Mechanically lifted functions (scripts/lift_run.py), module p003. Each one is checked against the original
//! in the world harness (tests/world_p003.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x6911e size=296
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006911e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6911e;
    loop {
        match pc {
        0x6911e => {
            // block 6911e
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6911e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6911e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6911f mov rbp,rsp
            rbp = rsp;
            // 69122 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 69124 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 69126 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 69128 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6912a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6912b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 6912f mov r15,rsi
            r15 = rsi;
            // 69132 mov r14,rdi
            r14 = rdi;
            // 69135 cmp QWORD PTR [r15+0x1d8],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x1d8)) as u64, 0x0u64).1;
            // 6913d jne 0x691b5
            if crate::cov::brk(0x6913d, fl.ne()) { pc = 0x691b5; continue; }
            pc = 0x6913f;
        }
        0x6913f => {
            // block 6913f
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6913f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6913f mov r13,QWORD PTR [r15+0x198]
            r13 = m.r64(r15.wrapping_add(0x198)) as u64;
            // 69146 lea rax,[r15+0x1a0]
            rax = r15.wrapping_add(0x1a0);
            // 6914d mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 69151 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 69154 je 0x691b5
            if crate::cov::brk(0x69154, fl.e()) { pc = 0x691b5; continue; }
            pc = 0x69156;
        }
        0x69156 => {
            // block 69156
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69156 lea r12,[r15+0x1c8]
            r12 = r15.wrapping_add(0x1c8);
            // 6915d lea rax,[r15+0x1d0]
            rax = r15.wrapping_add(0x1d0);
            // 69164 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            pc = 0x69168;
        }
        0x69168 => {
            // block 69168
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69168 lea rbx,[r13+0x20]
            rbx = r13.wrapping_add(0x20);
            // 6916c mov rdi,r12
            rdi = r12;
            // 6916f mov rsi,rbx
            rsi = rbx;
            // 69172 call 0xebcba
            { let r = { let t = m.env.stub_va(0x487); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69177 cmp rax,QWORD PTR [rbp-0x38]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 6917b jne 0x69188
            if crate::cov::brk(0x6917b, fl.ne()) { pc = 0x69188; continue; }
            pc = 0x6917d;
        }
        0x6917d => {
            // block 6917d
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6917d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6917d mov rdi,r12
            rdi = r12;
            // 69180 mov rsi,rbx
            rsi = rbx;
            // 69183 call 0xebca8
            { let r = { let t = m.env.stub_va(0x475); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x69188;
        }
        0x69188 => {
            // block 69188
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69188 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69188 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 6918c test rax,rax
            fl = logic::<64>(rax & rax);
            // 6918f je 0x6919f
            if crate::cov::brk(0x6918f, fl.e()) { pc = 0x6919f; continue; }
            pc = 0x69191;
        }
        0x69191 => {
            // block 69191
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69191 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69191 mov r13,rax
            r13 = rax;
            // 69194 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 69198 test rax,rax
            fl = logic::<64>(rax & rax);
            // 6919b jne 0x69191
            if crate::cov::brk(0x6919b, fl.ne()) { pc = 0x69191; continue; }
            pc = 0x6919d;
        }
        0x6919d => {
            // block 6919d
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6919d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6919d jmp 0x691af
            pc = 0x691af; continue;
        }
        0x6919f => {
            // block 6919f
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6919f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6919f mov rax,r13
            rax = r13;
            pc = 0x691a2;
        }
        0x691a2 => {
            // block 691a2
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691a2 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 691a6 cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // 691aa mov rax,r13
            rax = r13;
            // 691ad jne 0x691a2
            if crate::cov::brk(0x691ad, fl.ne()) { pc = 0x691a2; continue; }
            pc = 0x691af;
        }
        0x691af => {
            // block 691af
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691af cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 691b3 jne 0x69168
            if crate::cov::brk(0x691b3, fl.ne()) { pc = 0x69168; continue; }
            pc = 0x691b5;
        }
        0x691b5 => {
            // block 691b5
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691b5 lea r12,[r14+0x8]
            r12 = r14.wrapping_add(0x8);
            // 691b9 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 691c1 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 691c9 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 691d0 mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // 691d3 mov rbx,QWORD PTR [r15+0x1c8]
            rbx = m.r64(r15.wrapping_add(0x1c8)) as u64;
            // 691da add r15,0x1d0
            { let (r, f) = add::<64>(r15, 0x1d0u64);
              r15 = r;
              fl = f; }
            // 691e1 jmp 0x69217
            pc = 0x69217; continue;
        }
        0x691e3 => {
            // block 691e3
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691e3 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 691e7 mov rdi,r14
            rdi = r14;
            // 691ea mov rsi,r12
            rsi = r12;
            // 691ed call 0xebca2
            { let r = { let t = m.env.stub_va(0x46f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 691f2 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 691f6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 691f9 je 0x69208
            if crate::cov::brk(0x691f9, fl.e()) { pc = 0x69208; continue; }
            pc = 0x691fb;
        }
        0x691fb => {
            // block 691fb
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691fb mov rbx,rax
            rbx = rax;
            // 691fe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69201 test rax,rax
            fl = logic::<64>(rax & rax);
            // 69204 jne 0x691fb
            if crate::cov::brk(0x69204, fl.ne()) { pc = 0x691fb; continue; }
            pc = 0x69206;
        }
        0x69206 => {
            // block 69206
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69206 jmp 0x69217
            pc = 0x69217; continue;
        }
        0x69208 => {
            // block 69208
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69208 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69208 mov rax,rbx
            rax = rbx;
            pc = 0x6920b;
        }
        0x6920b => {
            // block 6920b
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6920b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6920b mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 6920f cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 69212 mov rax,rbx
            rax = rbx;
            // 69215 jne 0x6920b
            if crate::cov::brk(0x69215, fl.ne()) { pc = 0x6920b; continue; }
            pc = 0x69217;
        }
        0x69217 => {
            // block 69217
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69217 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6921a jne 0x691e3
            if crate::cov::brk(0x6921a, fl.ne()) { pc = 0x691e3; continue; }
            pc = 0x6921c;
        }
        0x6921c => {
            // block 6921c
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6921c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6921c mov rax,r14
            rax = r14;
            // 6921f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69223 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69224 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69226 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69228 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6922a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6922c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6922d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6922e => {
            // block 6922e
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 6922e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6922e mov rbx,rax
            rbx = rax;
            // 69231 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 69235 mov rdi,r14
            rdi = r14;
            // 69238 call 0xebcc0
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 6923d mov rdi,rbx
            rdi = rbx;
            // 69240 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d9a size=302
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6d9a;
    loop {
        match pc {
        0xd6d9a => {
            // block d6d9a
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6d9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d9b mov rbp,rsp
            rbp = rsp;
            // d6d9e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d6da0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d6da2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d6da4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d6da6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6da7 sub rsp,0x298
            { let (r, f) = sub::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6dae mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // d6db1 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // d6db3 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // d6db6 mov r15,rdi
            r15 = rdi;
            // d6db9 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // d6dbc xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // d6dbf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d6dc1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6dc4 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d6dc6 call QWORD PTR [rbx+0x10]
            { let r = { let t = m.r64(rbx.wrapping_add(0x10)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6dc9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6dcb je 0xd6e9b
            if crate::cov::brk(0xd6dcb, fl.e()) { pc = 0xd6e9b; continue; }
            pc = 0xd6dd1;
        }
        0xd6dd1 => {
            // block d6dd1
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6dd1 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6dd8 mov rdi,r12
            rdi = r12;
            // d6ddb call 0xb6722
            { let r = { let t = m.env.code_va(0xb6722); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6de0 movzx edi,r13w
            rdi = ((r13 as u16 as u64)) as u32 as u64;
            // d6de4 mov rsi,r12
            rsi = r12;
            // d6de7 call 0x17202
            { let r = { let t = m.env.code_va(0x17202); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6dec mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6df3 mov rax,QWORD PTR [rax+0x100]
            rax = m.r64(rax.wrapping_add(0x100)) as u64;
            // d6dfa lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e01 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6e03 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d6e05 je 0xd6e68
            if crate::cov::brk(0xd6e05, fl.e()) { pc = 0xd6e68; continue; }
            pc = 0xd6e07;
        }
        0xd6e07 => {
            // block d6e07
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e07 mov DWORD PTR [rbp-0x2b4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd4c), (0x0u64) as u32);
            // d6e11 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e18 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // d6e1c lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e23 lea rdx,[rbp-0x2b4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fd4c);
            // d6e2a mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d6e2f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e34 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e37 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6e39 test BYTE PTR [rbp-0x2b4],0x4
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fd4c)) as u64 & 0x4u64);
            // d6e40 je 0xd6e68
            if crate::cov::brk(0xd6e40, fl.e()) { pc = 0xd6e68; continue; }
            pc = 0xd6e42;
        }
        0xd6e42 => {
            // block d6e42
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e42 mov rax,QWORD PTR [rbp-0x2b0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // d6e49 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e4d lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e54 mov esi,0x40002
            rsi = (0x40002u64) as u32 as u64;
            // d6e59 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d6e5e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6e63 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6e66 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd6e68;
        }
        0xd6e68 => {
            // block d6e68
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e68 mov rdi,r15
            rdi = r15;
            // d6e6b call 0xd69d6
            { let r = { let t = m.env.code_va(0xd69d6); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6e70 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d6e73 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d6e77 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d6e79 mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // d6e7e mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // d6e84 mov rdi,r15
            rdi = r15;
            // d6e87 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d6e8a call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6e8c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d6e8f lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6e96 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd6e9b;
        }
        0xd6e9b => {
            // block d6e9b
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6e9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6e9b mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // d6e9e add rsp,0x298
            { let (r, f) = add::<64>(rsp, 0x298u64);
              rsp = r;
              fl = f; }
            // d6ea5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6ea6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d6ea8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d6eaa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d6eac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d6eae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6eaf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd6eb0 => {
            // block d6eb0
            #[cfg(feature = "trace")] eprintln!("f_000d6d9a pc d6eb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6eb0 mov rbx,rax
            rbx = rax;
            // d6eb3 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // d6eba call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d6ebf mov rdi,rbx
            rdi = rbx;
            // d6ec2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15028 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15028;
    loop {
        match pc {
        0x15028 => {
            // block 15028
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15028 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15029 mov rbp,rsp
            rbp = rsp;
            // 1502c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1502e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15030 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15032 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15033 mov r15,rdx
            r15 = rdx;
            // 15036 mov r14,rsi
            r14 = rsi;
            // 15039 mov rbx,rdi
            rbx = rdi;
            // 1503c mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 15044 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 1504c mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 15054 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 1505c mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 15064 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 1506c cmp BYTE PTR [rbx+0x190],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x190)) as u64, 0x0u64).1;
            // 15073 je 0x15083
            if crate::cov::brk(0x15073, fl.e()) { pc = 0x15083; continue; }
            pc = 0x15075;
        }
        0x15075 => {
            // block 15075
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15075 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15075 mov rdi,QWORD PTR [rbx+0x40]
            rdi = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 15079 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1507c je 0x15083
            if crate::cov::brk(0x1507c, fl.e()) { pc = 0x15083; continue; }
            pc = 0x1507e;
        }
        0x1507e => {
            // block 1507e
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1507e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1507e call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x15083;
        }
        0x15083 => {
            // block 15083
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15083 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15083 cmp BYTE PTR [rbx+0x191],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x191)) as u64, 0x0u64).1;
            // 1508a je 0x1509a
            if crate::cov::brk(0x1508a, fl.e()) { pc = 0x1509a; continue; }
            pc = 0x1508c;
        }
        0x1508c => {
            // block 1508c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1508c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1508c mov rdi,QWORD PTR [rbx+0x68]
            rdi = m.r64(rbx.wrapping_add(0x68)) as u64;
            // 15090 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 15093 je 0x1509a
            if crate::cov::brk(0x15093, fl.e()) { pc = 0x1509a; continue; }
            pc = 0x15095;
        }
        0x15095 => {
            // block 15095
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15095 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15095 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x1509a;
        }
        0x1509a => {
            // block 1509a
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1509a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1509a mov QWORD PTR [rbx+0x60],r15
            m.w64(rbx.wrapping_add(0x60), (r15) as u64);
            // 1509e cmp r15,0x9
            fl = sub::<64>(r15, 0x9u64).1;
            // 150a2 jb 0x150c2
            if crate::cov::brk(0x150a2, fl.b()) { pc = 0x150c2; continue; }
            pc = 0x150a4;
        }
        0x150a4 => {
            // block 150a4
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150a4 mov r12b,BYTE PTR [rbx+0x192]
            r12 = (r12 & !0xff) | ((m.r8(rbx.wrapping_add(0x192)) as u64) as u8 as u64);
            // 150ab test r14,r14
            fl = logic::<64>(r14 & r14);
            // 150ae je 0x150e2
            if crate::cov::brk(0x150ae, fl.e()) { pc = 0x150e2; continue; }
            pc = 0x150b0;
        }
        0x150b0 => {
            // block 150b0
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150b0 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 150b3 je 0x150e2
            if crate::cov::brk(0x150b3, fl.e()) { pc = 0x150e2; continue; }
            pc = 0x150b5;
        }
        0x150b5 => {
            // block 150b5
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150b5 mov QWORD PTR [rbx+0x40],r14
            m.w64(rbx.wrapping_add(0x40), (r14) as u64);
            // 150b9 mov BYTE PTR [rbx+0x190],0x0
            m.w8(rbx.wrapping_add(0x190), (0x0u64) as u8);
            // 150c0 jmp 0x150f5
            pc = 0x150f5; continue;
        }
        0x150c2 => {
            // block 150c2
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150c2 lea rax,[rbx+0x58]
            rax = rbx.wrapping_add(0x58);
            // 150c6 mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 150ca mov QWORD PTR [rbx+0x60],0x8
            m.w64(rbx.wrapping_add(0x60), (0x8u64) as u64);
            // 150d2 mov BYTE PTR [rbx+0x190],0x0
            m.w8(rbx.wrapping_add(0x190), (0x0u64) as u8);
            // 150d9 mov r12b,BYTE PTR [rbx+0x192]
            r12 = (r12 & !0xff) | ((m.r8(rbx.wrapping_add(0x192)) as u64) as u8 as u64);
            // 150e0 jmp 0x150f5
            pc = 0x150f5; continue;
        }
        0x150e2 => {
            // block 150e2
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150e2 mov rdi,r15
            rdi = r15;
            // 150e5 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 150ea mov QWORD PTR [rbx+0x40],rax
            m.w64(rbx.wrapping_add(0x40), (rax) as u64);
            // 150ee mov BYTE PTR [rbx+0x190],0x1
            m.w8(rbx.wrapping_add(0x190), (0x1u64) as u8);
            pc = 0x150f5;
        }
        0x150f5 => {
            // block 150f5
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150f5 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 150f8 je 0x15113
            if crate::cov::brk(0x150f8, fl.e()) { pc = 0x15113; continue; }
            pc = 0x150fa;
        }
        0x150fa => {
            // block 150fa
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 150fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 150fa mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 15101 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 15109 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 15111 jmp 0x1514c
            pc = 0x1514c; continue;
        }
        0x15113 => {
            // block 15113
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15113 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15113 cmp r15,0x8
            fl = sub::<64>(r15, 0x8u64).1;
            // 15117 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // 1511c cmovge rdi,r15
            if fl.ge() { rdi = r15; }
            // 15120 mov QWORD PTR [rbx+0x70],rdi
            m.w64(rbx.wrapping_add(0x70), (rdi) as u64);
            // 15124 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 15127 je 0x1513c
            if crate::cov::brk(0x15127, fl.e()) { pc = 0x1513c; continue; }
            pc = 0x15129;
        }
        0x15129 => {
            // block 15129
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 15129 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15129 cmp rdi,0x8
            fl = sub::<64>(rdi, 0x8u64).1;
            // 1512d jb 0x1513c
            if crate::cov::brk(0x1512d, fl.b()) { pc = 0x1513c; continue; }
            pc = 0x1512f;
        }
        0x1512f => {
            // block 1512f
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1512f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1512f mov QWORD PTR [rbx+0x68],r14
            m.w64(rbx.wrapping_add(0x68), (r14) as u64);
            // 15133 mov BYTE PTR [rbx+0x191],0x0
            m.w8(rbx.wrapping_add(0x191), (0x0u64) as u8);
            // 1513a jmp 0x1514c
            pc = 0x1514c; continue;
        }
        0x1513c => {
            // block 1513c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1513c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1513c call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 15141 mov QWORD PTR [rbx+0x68],rax
            m.w64(rbx.wrapping_add(0x68), (rax) as u64);
            // 15145 mov BYTE PTR [rbx+0x191],0x1
            m.w8(rbx.wrapping_add(0x191), (0x1u64) as u8);
            pc = 0x1514c;
        }
        0x1514c => {
            // block 1514c
            #[cfg(feature = "trace")] eprintln!("f_00015028 pc 1514c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1514c mov rax,rbx
            rax = rbx;
            // 1514f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15150 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15152 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15154 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 15156 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15157 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68fea size=308
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068fea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68fea;
    loop {
        match pc {
        0x68fea => {
            // block 68fea
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 68fea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68fea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68feb mov rbp,rsp
            rbp = rsp;
            // 68fee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 68ff0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68ff2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 68ff4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 68ff6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68ff7 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 68ffb mov r12,rdx
            r12 = rdx;
            // 68ffe mov DWORD PTR [rbp-0x2c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u32 as u64)) as u32);
            // 69001 mov r15,rdi
            r15 = rdi;
            // 69004 lea rdi,[r15+0x198]
            rdi = r15.wrapping_add(0x198);
            // 6900b mov rsi,r12
            rsi = r12;
            // 6900e call 0xebbd6
            { let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 69013 mov rbx,rax
            rbx = rax;
            // 69016 add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 6901d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69020 je 0x6910d
            if crate::cov::brk(0x69020, fl.e()) { pc = 0x6910d; continue; }
            pc = 0x69026;
        }
        0x69026 => {
            // block 69026
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69026 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69026 movzx r10d,BYTE PTR [r12]
            r10 = (m.r8(r12) as u64) as u32 as u64;
            // 6902b mov r11b,r10b
            r11 = (r11 & !0xff) | (((r10 as u8 as u64)) as u8 as u64);
            // 6902e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69032 shr r10,1
            { let (r, f) = shr::<64>(r10, 0x1u64, fl); r10 = r; fl = f; }
            // 69035 mov r8,QWORD PTR [r12+0x8]
            r8 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 6903a mov r13,QWORD PTR [r12+0x10]
            r13 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 6903f lea r14,[r12+0x1]
            r14 = r12.wrapping_add(0x1);
            // 69044 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            pc = 0x69048;
        }
        0x69048 => {
            // block 69048
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69048 movzx edx,BYTE PTR [rbx+0x20]
            rdx = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 6904c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6904e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 69050 jne 0x69057
            if crate::cov::brk(0x69050, fl.ne()) { pc = 0x69057; continue; }
            pc = 0x69052;
        }
        0x69052 => {
            // block 69052
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69052 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 69055 jmp 0x6905b
            pc = 0x6905b; continue;
        }
        0x69057 => {
            // block 69057
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69057 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69057 mov rdx,QWORD PTR [rbx+0x28]
            rdx = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x6905b;
        }
        0x6905b => {
            // block 6905b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6905b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6905b test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 6905e mov rcx,r8
            rcx = r8;
            // 69061 cmove rcx,r10
            if fl.e() { rcx = r10; }
            // 69065 cmp rdx,rcx
            fl = sub::<64>(rdx, rcx).1;
            // 69068 jne 0x6910d
            if crate::cov::brk(0x69068, fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x6906e;
        }
        0x6906e => {
            // block 6906e
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6906e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6906e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69070 jne 0x6907b
            if crate::cov::brk(0x69070, fl.ne()) { pc = 0x6907b; continue; }
            pc = 0x69072;
        }
        0x69072 => {
            // block 69072
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69072 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69072 lea rdi,[rbx+0x20]
            rdi = rbx.wrapping_add(0x20);
            // 69076 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 69079 jmp 0x6907f
            pc = 0x6907f; continue;
        }
        0x6907b => {
            // block 6907b
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907b mov rdi,QWORD PTR [rbx+0x30]
            rdi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x6907f;
        }
        0x6907f => {
            // block 6907f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6907f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6907f test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // 69082 mov rsi,r13
            rsi = r13;
            // 69085 cmove rsi,r14
            if fl.e() { rsi = r14; }
            // 69089 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6908b jne 0x690a9
            if crate::cov::brk(0x6908b, fl.ne()) { pc = 0x690a9; continue; }
            pc = 0x6908d;
        }
        0x6908d => {
            // block 6908d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6908d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6908d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69090 je 0x690d5
            if crate::cov::brk(0x69090, fl.e()) { pc = 0x690d5; continue; }
            pc = 0x69092;
        }
        0x69092 => {
            // block 69092
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69092 movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 69095 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 69098 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 6909a jne 0x6910d
            if crate::cov::brk(0x6909a, fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x6909c;
        }
        0x6909c => {
            // block 6909c
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6909c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6909c inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 6909f inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 690a2 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 690a5 jne 0x69092
            if crate::cov::brk(0x690a5, fl.ne()) { pc = 0x69092; continue; }
            pc = 0x690a7;
        }
        0x690a7 => {
            // block 690a7
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a7 jmp 0x690d5
            pc = 0x690d5; continue;
        }
        0x690a9 => {
            // block 690a9
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690a9 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 690ac je 0x690d5
            if crate::cov::brk(0x690ac, fl.e()) { pc = 0x690d5; continue; }
            pc = 0x690ae;
        }
        0x690ae => {
            // block 690ae
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690ae mov r12,r8
            r12 = r8;
            // 690b1 mov QWORD PTR [rbp-0x38],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r15) as u64);
            // 690b5 mov r15,r10
            r15 = r10;
            // 690b8 mov r14b,r11b
            r14 = (r14 & !0xff) | (((r11 as u8 as u64)) as u8 as u64);
            // 690bb call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 690c0 mov r11b,r14b
            r11 = (r11 & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 690c3 mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 690c7 mov r10,r15
            r10 = r15;
            // 690ca mov r15,QWORD PTR [rbp-0x38]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 690ce mov r8,r12
            r8 = r12;
            // 690d1 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 690d3 jne 0x6910d
            if crate::cov::brk(0x690d3, fl.ne()) { pc = 0x6910d; continue; }
            pc = 0x690d5;
        }
        0x690d5 => {
            // block 690d5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690d5 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 690d7 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 690da cmp DWORD PTR [rbx+0x38],ecx
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x38)) as u64, (rcx as u32 as u64)).1;
            // 690dd je 0x6910f
            if crate::cov::brk(0x690dd, fl.e()) { pc = 0x6910f; continue; }
            pc = 0x690df;
        }
        0x690df => {
            // block 690df
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690df rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690df mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 690e3 test rax,rax
            fl = logic::<64>(rax & rax);
            // 690e6 je 0x690f5
            if crate::cov::brk(0x690e6, fl.e()) { pc = 0x690f5; continue; }
            pc = 0x690e8;
        }
        0x690e8 => {
            // block 690e8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690e8 mov rbx,rax
            rbx = rax;
            // 690eb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 690ee test rax,rax
            fl = logic::<64>(rax & rax);
            // 690f1 jne 0x690e8
            if crate::cov::brk(0x690f1, fl.ne()) { pc = 0x690e8; continue; }
            pc = 0x690f3;
        }
        0x690f3 => {
            // block 690f3
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f3 jmp 0x69104
            pc = 0x69104; continue;
        }
        0x690f5 => {
            // block 690f5
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f5 mov rax,rbx
            rax = rbx;
            pc = 0x690f8;
        }
        0x690f8 => {
            // block 690f8
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 690f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 690f8 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 690fc cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 690ff mov rax,rbx
            rax = rbx;
            // 69102 jne 0x690f8
            if crate::cov::brk(0x69102, fl.ne()) { pc = 0x690f8; continue; }
            pc = 0x69104;
        }
        0x69104 => {
            // block 69104
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69104 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69107 jne 0x69048
            if crate::cov::brk(0x69107, fl.ne()) { pc = 0x69048; continue; }
            pc = 0x6910d;
        }
        0x6910d => {
            // block 6910d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x6910f;
        }
        0x6910f => {
            // block 6910f
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6910f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6910f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 69113 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69114 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 69116 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 69118 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6911a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6911c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6911d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ca6e size=318
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ca6e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ca6e;
    loop {
        match pc {
        0x2ca6e => {
            // block 2ca6e
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2ca6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ca6e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ca6f mov rbp,rsp
            rbp = rsp;
            // 2ca72 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2ca74 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2ca76 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2ca78 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2ca7a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ca7b sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2ca7f mov r15,rsi
            r15 = rsi;
            // 2ca82 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // 2ca86 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 2ca8a sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 2ca8d sar rsi,0x4
            { let (r, f) = sar::<64>(rsi, 0x4u64, fl); rsi = r; fl = f; }
            // 2ca91 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ca96 lea rsi,[rip+0xfdf07] # 0x12a9a4
            rsi = m.env.data_va(258417);
            // 2ca9d mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // 2caa2 mov rdi,rax
            rdi = rax;
            // 2caa5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2caaa mov rbx,rax
            rbx = rax;
            // 2caad mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2cab0 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2cab4 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 2cab7 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2cabb mov rdi,r14
            rdi = r14;
            // 2cabe call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cac3 mov rsi,QWORD PTR [rip+0x1136c6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2caca mov rdi,r14
            rdi = r14;
            // 2cacd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cad2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2cad5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2cad9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2cade mov rdi,rax
            rdi = rax;
            // 2cae1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cae3 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2cae6 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2caea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2caef movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 2caf3 mov rdi,rbx
            rdi = rbx;
            // 2caf6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cafb mov rdi,rbx
            rdi = rbx;
            // 2cafe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb03 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 2cb06 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2cb0a je 0x2cb7b
            if crate::cov::brk(0x2cb0a, fl.e()) { pc = 0x2cb7b; continue; }
            pc = 0x2cb0c;
        }
        0x2cb0c => {
            // block 2cb0c
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb0c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb0c lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2cb10;
        }
        0x2cb10 => {
            // block 2cb10
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb10 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2cb14 mov rsi,rbx
            rsi = rbx;
            // 2cb17 call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb1c mov r13,rax
            r13 = rax;
            // 2cb1f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 2cb23 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2cb27 add rsi,r13
            { let (r, f) = add::<64>(rsi, r13);
              rsi = r;
              fl = f; }
            // 2cb2a mov rdi,r12
            rdi = r12;
            // 2cb2d call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb32 mov rdi,r12
            rdi = r12;
            // 2cb35 mov rsi,QWORD PTR [rip+0x113654] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 2cb3c call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb41 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2cb44 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 2cb48 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 2cb4d mov rdi,rax
            rdi = rax;
            // 2cb50 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb52 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 2cb55 mov rdi,r12
            rdi = r12;
            // 2cb58 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb5d movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 2cb61 mov rdi,r13
            rdi = r13;
            // 2cb64 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb69 mov rdi,r13
            rdi = r13;
            // 2cb6c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb71 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 2cb75 cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 2cb79 jne 0x2cb10
            if crate::cov::brk(0x2cb79, fl.ne()) { pc = 0x2cb10; continue; }
            pc = 0x2cb7b;
        }
        0x2cb7b => {
            // block 2cb7b
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb7b mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2cb7f add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2cb83 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2cb84 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2cb86 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2cb88 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2cb8a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2cb8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2cb8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2cb8e => {
            // block 2cb8e
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb8e mov rbx,rax
            rbx = rax;
            // 2cb91 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            pc = 0x2cb95;
        }
        0x2cb95 => {
            // block 2cb95
            #[cfg(feature = "trace")] eprintln!("f_0002ca6e pc 2cb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cb95 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cb9a mov rdi,rbx
            rdi = rbx;
            // 2cb9d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cba2 mov rbx,rax
            rbx = rax;
            // 2cba5 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2cba9 jmp 0x2cb95
            pc = 0x2cb95; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x586aa size=332
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000586aa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x586aa;
    loop {
        match pc {
        0x586aa => {
            // block 586aa
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586aa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 586ab mov rbp,rsp
            rbp = rsp;
            // 586ae push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 586b0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 586b2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 586b4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 586b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 586b7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 586b8 mov rbx,rdx
            rbx = rdx;
            // 586bb mov r14,rsi
            r14 = rsi;
            // 586be mov r15,rdi
            r15 = rdi;
            // 586c1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 586c4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 586c8 cmp rcx,rax
            fl = sub::<64>(rcx, rax).1;
            // 586cb je 0x586e5
            if crate::cov::brk(0x586cb, fl.e()) { pc = 0x586e5; continue; }
            pc = 0x586cd;
        }
        0x586cd => {
            // block 586cd
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586cd lea rdx,[rcx-0x2]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffe);
            // 586d1 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 586d4 not rdx
            { let v = !rdx; rdx = v; }
            // 586d7 and rdx,0xfffffffffffffffe
            { let r = rdx & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 586db add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 586de mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            // 586e2 mov rax,rdx
            rax = rdx;
            pc = 0x586e5;
        }
        0x586e5 => {
            // block 586e5
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586e5 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 586e8 je 0x58716
            if crate::cov::brk(0x586e8, fl.e()) { pc = 0x58716; continue; }
            pc = 0x586ea;
        }
        0x586ea => {
            // block 586ea
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586ea cmp DWORD PTR [r14],0x0
            fl = sub::<32>(m.r32(r14) as u64, 0x0u64).1;
            // 586ee je 0x58716
            if crate::cov::brk(0x586ee, fl.e()) { pc = 0x58716; continue; }
            pc = 0x586f0;
        }
        0x586f0 => {
            // block 586f0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f0 mov ecx,DWORD PTR [r14+0x4]
            rcx = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // 586f4 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 586f6 je 0x58716
            if crate::cov::brk(0x586f6, fl.e()) { pc = 0x58716; continue; }
            pc = 0x586f8;
        }
        0x586f8 => {
            // block 586f8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 586f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 586f8 cmp WORD PTR [r14+0x34],0x0
            fl = sub::<16>(m.r16(r14.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 586fe je 0x58716
            if crate::cov::brk(0x586fe, fl.e()) { pc = 0x58716; continue; }
            pc = 0x58700;
        }
        0x58700 => {
            // block 58700
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58700 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58700 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 58703 cmp DWORD PTR [r14+0x24],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 58708 je 0x58719
            if crate::cov::brk(0x58708, fl.e()) { pc = 0x58719; continue; }
            pc = 0x5870a;
        }
        0x5870a => {
            // block 5870a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870a cmp ecx,0x6
            fl = sub::<32>((rcx as u32 as u64), 0x6u64).1;
            // 5870d jb 0x58719
            if crate::cov::brk(0x5870d, fl.b()) { pc = 0x58719; continue; }
            pc = 0x5870f;
        }
        0x5870f => {
            // block 5870f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5870f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5870f cmp DWORD PTR [r14+0x18],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x18)) as u64, 0x0u64).1;
            // 58714 je 0x5872b
            if crate::cov::brk(0x58714, fl.e()) { pc = 0x5872b; continue; }
            pc = 0x58716;
        }
        0x58716 => {
            // block 58716
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58716 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58716 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0x58719;
        }
        0x58719 => {
            // block 58719
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58719 mov al,r12b
            rax = (rax & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // 5871c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 58720 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 58721 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 58723 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 58725 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 58727 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 58729 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5872a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5872b => {
            // block 5872b
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5872b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5872b mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // 5872e cmp DWORD PTR [r14+0x8],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x8)) as u64, 0x0u64).1;
            // 58733 je 0x58719
            if crate::cov::brk(0x58733, fl.e()) { pc = 0x58719; continue; }
            pc = 0x58735;
        }
        0x58735 => {
            // block 58735
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58735 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58735 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // 58738 jmp 0x58741
            pc = 0x58741; continue;
        }
        0x5873a => {
            // block 5873a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5873a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5873a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5873e inc r13
            { let (r, f) = inc::<64>(r13, fl); r13 = r; fl = f; }
            pc = 0x58741;
        }
        0x58741 => {
            // block 58741
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58741 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58741 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58745 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58747 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5874d mov WORD PTR [rbp-0x2a],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd6), ((rsi as u16 as u64)) as u16);
            // 58751 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 58755 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58758 jae 0x58767
            if crate::cov::brk(0x58758, fl.ae()) { pc = 0x58767; continue; }
            pc = 0x5875a;
        }
        0x5875a => {
            // block 5875a
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5875a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5875a mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 5875d add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 58761 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 58765 jmp 0x5877f
            pc = 0x5877f; continue;
        }
        0x58767 => {
            // block 58767
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58767 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58767 mov rdi,rbx
            rdi = rbx;
            // 5876a lea rsi,[rbp-0x2a]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd6);
            // 5876e call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 58773 mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 58777 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 5877b mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x5877f;
        }
        0x5877f => {
            // block 5877f
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 5877f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5877f mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 58781 shr esi,0xa
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xau64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 58784 and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 5878a mov WORD PTR [rbp-0x2c],si
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rsi as u16 as u64)) as u16);
            // 5878e cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 58791 jae 0x587a0
            if crate::cov::brk(0x58791, fl.ae()) { pc = 0x587a0; continue; }
            pc = 0x58793;
        }
        0x58793 => {
            // block 58793
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 58793 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 58793 mov WORD PTR [rax],si
            m.w16(rax, ((rsi as u16 as u64)) as u16);
            // 58796 add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 5879a mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 5879e jmp 0x587b8
            pc = 0x587b8; continue;
        }
        0x587a0 => {
            // block 587a0
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587a0 mov rdi,rbx
            rdi = rbx;
            // 587a3 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 587a7 call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 587ac mov ecx,DWORD PTR [r15+r13*4]
            rcx = (m.r32(r15.wrapping_add(r13.wrapping_mul(4))) as u64) as u32 as u64;
            // 587b0 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 587b4 mov rdx,QWORD PTR [rbx+0x10]
            rdx = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0x587b8;
        }
        0x587b8 => {
            // block 587b8
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587b8 shr ecx,0x14
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x14u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 587bb and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 587c1 mov WORD PTR [rbp-0x2e],cx
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffd2), ((rcx as u16 as u64)) as u16);
            // 587c5 cmp rax,rdx
            fl = sub::<64>(rax, rdx).1;
            // 587c8 jae 0x587d7
            if crate::cov::brk(0x587c8, fl.ae()) { pc = 0x587d7; continue; }
            pc = 0x587ca;
        }
        0x587ca => {
            // block 587ca
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587ca mov WORD PTR [rax],cx
            m.w16(rax, ((rcx as u16 as u64)) as u16);
            // 587cd add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // 587d1 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 587d5 jmp 0x587e3
            pc = 0x587e3; continue;
        }
        0x587d7 => {
            // block 587d7
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587d7 mov rdi,rbx
            rdi = rbx;
            // 587da lea rsi,[rbp-0x2e]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd2);
            // 587de call 0xebbc4
            { let r = { let t = m.env.stub_va(0x391); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x587e3;
        }
        0x587e3 => {
            // block 587e3
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587e3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587e3 lea eax,[r13+0x1]
            rax = (r13.wrapping_add(0x1)) as u32 as u64;
            // 587e7 cmp eax,DWORD PTR [r14+0x8]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x8)) as u64).1;
            // 587eb jae 0x58719
            if crate::cov::brk(0x587eb, fl.ae()) { pc = 0x58719; continue; }
            pc = 0x587f1;
        }
        0x587f1 => {
            // block 587f1
            #[cfg(feature = "trace")] eprintln!("f_000586aa pc 587f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 587f1 jmp 0x5873a
            pc = 0x5873a; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x421e0 size=340
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000421e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x421e0;
    loop {
        match pc {
        0x421e0 => {
            // block 421e0
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 421e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 421e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 421e1 mov rbp,rsp
            rbp = rsp;
            // 421e4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 421e6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 421e8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 421ea push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 421eb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 421ef mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 421f2 mov r14,rsi
            r14 = rsi;
            // 421f5 mov rbx,rdi
            rbx = rdi;
            // 421f8 mov eax,r15d
            rax = ((r15 as u32 as u64)) as u32 as u64;
            // 421fb lea rcx,[rip+0xbe26e] # 0x100470
            rcx = m.env.data_va(85053);
            // 42202 mov r12d,DWORD PTR [rcx+rax*4]
            r12 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 42206 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // 42209 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 4220e mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 42215 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // 42218 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 4221b ja 0x4223d
            if crate::cov::brk(0x4221b, fl.a()) { pc = 0x4223d; continue; }
            pc = 0x4221d;
        }
        0x4221d => {
            // block 4221d
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4221d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4221d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42220 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 42224 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 42229 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4222f mov rdi,rbx
            rdi = rbx;
            // 42232 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42235 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 42238 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x4223d => {
            // block 4223d
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4223d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4223d mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 42244 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 4224b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4224e lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 42252 mov ecx,0x10000
            rcx = (0x10000u64) as u32 as u64;
            // 42257 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // 4225d mov rdi,rbx
            rdi = rbx;
            // 42260 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42263 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 42266 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 42268 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4226a mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 4226f je 0x4228f
            if crate::cov::brk(0x4226f, fl.e()) { pc = 0x4228f; continue; }
            pc = 0x42271;
        }
        0x42271 => {
            // block 42271
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42271 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42274 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 42278 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 4227d mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            // 42283 mov rdi,rbx
            rdi = rbx;
            // 42286 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 42289 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 4228c mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            pc = 0x4228f;
        }
        0x4228f => {
            // block 4228f
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4228f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4228f mov edx,DWORD PTR [rbp-0x28]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 42292 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 42294 or esi,edx
            { let r = (rsi as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 42296 je 0x422a9
            if crate::cov::brk(0x42296, fl.e()) { pc = 0x422a9; continue; }
            pc = 0x42298;
        }
        0x42298 => {
            // block 42298
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42298 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 4229a je 0x422b2
            if crate::cov::brk(0x4229a, fl.e()) { pc = 0x422b2; continue; }
            pc = 0x4229c;
        }
        0x4229c => {
            // block 4229c
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4229c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4229c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 4229e jne 0x422b2
            if crate::cov::brk(0x4229e, fl.ne()) { pc = 0x422b2; continue; }
            pc = 0x422a0;
        }
        0x422a0 => {
            // block 422a0
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422a0 mov DWORD PTR [rbp-0x24],0x1
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x1u64) as u32);
            // 422a7 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422a9 => {
            // block 422a9
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422a9 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 422b0 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422b2 => {
            // block 422b2
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422b2 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 422b4 jne 0x422c3
            if crate::cov::brk(0x422b4, fl.ne()) { pc = 0x422c3; continue; }
            pc = 0x422b6;
        }
        0x422b6 => {
            // block 422b6
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422b6 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 422b8 je 0x422c3
            if crate::cov::brk(0x422b8, fl.e()) { pc = 0x422c3; continue; }
            pc = 0x422ba;
        }
        0x422ba => {
            // block 422ba
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422ba mov DWORD PTR [rbp-0x24],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x2u64) as u32);
            // 422c1 jmp 0x422d2
            pc = 0x422d2; continue;
        }
        0x422c3 => {
            // block 422c3
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422c3 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 422c5 je 0x422d2
            if crate::cov::brk(0x422c5, fl.e()) { pc = 0x422d2; continue; }
            pc = 0x422c7;
        }
        0x422c7 => {
            // block 422c7
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422c7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 422c9 je 0x422d2
            if crate::cov::brk(0x422c9, fl.e()) { pc = 0x422d2; continue; }
            pc = 0x422cb;
        }
        0x422cb => {
            // block 422cb
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422cb mov DWORD PTR [rbp-0x24],0x3
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x3u64) as u32);
            pc = 0x422d2;
        }
        0x422d2 => {
            // block 422d2
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422d2 cmp r15d,0x4
            fl = sub::<32>((r15 as u32 as u64), 0x4u64).1;
            // 422d6 jl 0x4230a
            if crate::cov::brk(0x422d6, fl.l()) { pc = 0x4230a; continue; }
            pc = 0x422d8;
        }
        0x422d8 => {
            // block 422d8
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422d8 mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 422db cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // 422df ja 0x4230a
            if crate::cov::brk(0x422df, fl.a()) { pc = 0x4230a; continue; }
            pc = 0x422e1;
        }
        0x422e1 => {
            // block 422e1
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422e1 lea rdx,[rip+0x3c] # 0x42324
            rdx = m.env.code_va(0x42324);
            // 422e8 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // 422ec add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // 422ef jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x422f1 => {
            // block 422f1
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422f1 mov DWORD PTR [rbp-0x24],0x4
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x4u64) as u32);
            // 422f8 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x422fa => {
            // block 422fa
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 422fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 422fa mov DWORD PTR [rbp-0x24],0x5
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x5u64) as u32);
            // 42301 jmp 0x4230a
            pc = 0x4230a; continue;
        }
        0x42303 => {
            // block 42303
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42303 mov DWORD PTR [rbp-0x24],0x6
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x6u64) as u32);
            pc = 0x4230a;
        }
        0x4230a => {
            // block 4230a
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4230a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4230a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4230c je 0x42314
            if crate::cov::brk(0x4230c, fl.e()) { pc = 0x42314; continue; }
            pc = 0x4230e;
        }
        0x4230e => {
            // block 4230e
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 4230e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4230e mov ecx,DWORD PTR [rbp-0x24]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 42311 mov DWORD PTR [r14],ecx
            m.w32(r14, ((rcx as u32 as u64)) as u32);
            pc = 0x42314;
        }
        0x42314 => {
            // block 42314
            #[cfg(feature = "trace")] eprintln!("f_000421e0 pc 42314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42314 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 42318 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42319 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 4231b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4231d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4231f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42320 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf04e size=342
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000df04e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdf04e;
    loop {
        match pc {
        0xdf04e => {
            // block df04e
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df04e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df04e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df04f mov rbp,rsp
            rbp = rsp;
            // df052 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df054 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df056 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df058 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df05a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df05b sub rsp,0x78
            { let (r, f) = sub::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df05f mov QWORD PTR [rbp-0x88],r8
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (r8) as u64);
            // df066 mov QWORD PTR [rbp-0x90],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (rcx) as u64);
            // df06d mov r12,rdx
            r12 = rdx;
            // df070 mov r13,rsi
            r13 = rsi;
            // df073 mov rbx,rdi
            rbx = rdi;
            // df076 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df079 mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df07f lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // df083 call QWORD PTR [rax+0x3f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3f0)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // df089 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df08c mov edx,DWORD PTR [rbx+0x250]
            rdx = (m.r32(rbx.wrapping_add(0x250)) as u64) as u32 as u64;
            // df092 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // df096 mov rdi,rbx
            rdi = rbx;
            // df099 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // df09f mov r14d,DWORD PTR [rbp-0x30]
            r14 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // df0a3 lea ecx,[r14-0x5]
            rcx = (r14.wrapping_add(0xffff_ffff_ffff_fffb)) as u32 as u64;
            // df0a7 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // df0aa ja 0xdf0bb
            if crate::cov::brk(0xdf0aa, fl.a()) { pc = 0xdf0bb; continue; }
            pc = 0xdf0ac;
        }
        0xdf0ac => {
            // block df0ac
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0ac mov r15d,0x121
            r15 = (0x121u64) as u32 as u64;
            // df0b2 shr r15d,cl
            { let (r, f) = shr::<32>((r15 as u32 as u64), (rcx as u8 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // df0b5 and r15d,0x1
            { let r = (r15 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // df0b9 jmp 0xdf0be
            pc = 0xdf0be; continue;
        }
        0xdf0bb => {
            // block df0bb
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0bb xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xdf0be;
        }
        0xdf0be => {
            // block df0be
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0be mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // df0c1 mov edx,DWORD PTR [rbx+0x274]
            rdx = (m.r32(rbx.wrapping_add(0x274)) as u64) as u32 as u64;
            // df0c7 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // df0cb xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // df0cd xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // df0d0 xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // df0d3 call 0x32d57
            { let r = { let t = m.env.code_va(0x32d57); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // df0d8 mov eax,DWORD PTR [rbp-0x6c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // df0db mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // df0de mov eax,DWORD PTR [rbp-0x68]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // df0e1 mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // df0e4 mov eax,DWORD PTR [rbp-0x70]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // df0e7 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // df0ea mov eax,DWORD PTR [rbp-0x64]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // df0ed mov DWORD PTR [rbp-0x94],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c), ((rax as u32 as u64)) as u32);
            // df0f3 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // df0f6 je 0xdf10c
            if crate::cov::brk(0xdf0f6, fl.e()) { pc = 0xdf10c; continue; }
            pc = 0xdf0f8;
        }
        0xdf0f8 => {
            // block df0f8
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df0f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df0f8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df0fb lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df0ff lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df103 mov rdi,rbx
            rdi = rbx;
            // df106 call QWORD PTR [rax+0x1a00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a00)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xdf10c;
        }
        0xdf10c => {
            // block df10c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df10c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df10c cmp r14d,0x11
            fl = sub::<32>((r14 as u32 as u64), 0x11u64).1;
            // df110 jne 0xdf12c
            if crate::cov::brk(0xdf110, fl.ne()) { pc = 0xdf12c; continue; }
            pc = 0xdf112;
        }
        0xdf112 => {
            // block df112
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df112 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df115 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df119 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df11d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df121 mov rdi,rbx
            rdi = rbx;
            // df124 call QWORD PTR [rax+0x1a08]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a08)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // df12a jmp 0xdf14a
            pc = 0xdf14a; continue;
        }
        0xdf12c => {
            // block df12c
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df12c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df12c cmp r14d,0x10
            fl = sub::<32>((r14 as u32 as u64), 0x10u64).1;
            // df130 jne 0xdf14a
            if crate::cov::brk(0xdf130, fl.ne()) { pc = 0xdf14a; continue; }
            pc = 0xdf132;
        }
        0xdf132 => {
            // block df132
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df132 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df132 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // df135 lea rsi,[rbp-0x74]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff8c);
            // df139 lea rdx,[rbp-0x78]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // df13d lea rcx,[rbp-0x7c]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff84);
            // df141 mov rdi,rbx
            rdi = rbx;
            // df144 call QWORD PTR [rax+0x1a10]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a10)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xdf14a;
        }
        0xdf14a => {
            // block df14a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df14a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df14a test r13,r13
            fl = logic::<64>(r13 & r13);
            // df14d mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // df154 mov rdx,QWORD PTR [rbp-0x90]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // df15b je 0xdf164
            if crate::cov::brk(0xdf15b, fl.e()) { pc = 0xdf164; continue; }
            pc = 0xdf15d;
        }
        0xdf15d => {
            // block df15d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df15d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df15d mov eax,DWORD PTR [rbp-0x74]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // df160 mov DWORD PTR [r13+0x0],eax
            m.w32(r13, ((rax as u32 as u64)) as u32);
            pc = 0xdf164;
        }
        0xdf164 => {
            // block df164
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df164 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df164 test r12,r12
            fl = logic::<64>(r12 & r12);
            // df167 je 0xdf170
            if crate::cov::brk(0xdf167, fl.e()) { pc = 0xdf170; continue; }
            pc = 0xdf169;
        }
        0xdf169 => {
            // block df169
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df169 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df169 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df16c mov DWORD PTR [r12],eax
            m.w32(r12, ((rax as u32 as u64)) as u32);
            pc = 0xdf170;
        }
        0xdf170 => {
            // block df170
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df170 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df170 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // df173 je 0xdf17d
            if crate::cov::brk(0xdf173, fl.e()) { pc = 0xdf17d; continue; }
            pc = 0xdf175;
        }
        0xdf175 => {
            // block df175
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df175 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df175 mov eax,DWORD PTR [rbp-0x78]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // df178 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df17b mov DWORD PTR [rdx],eax
            m.w32(rdx, ((rax as u32 as u64)) as u32);
            pc = 0xdf17d;
        }
        0xdf17d => {
            // block df17d
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df17d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df17d test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // df180 je 0xdf18a
            if crate::cov::brk(0xdf180, fl.e()) { pc = 0xdf18a; continue; }
            pc = 0xdf182;
        }
        0xdf182 => {
            // block df182
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df182 mov eax,DWORD PTR [rbp-0x94]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff6c)) as u64) as u32 as u64;
            // df188 mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            pc = 0xdf18a;
        }
        0xdf18a => {
            // block df18a
            #[cfg(feature = "trace")] eprintln!("f_000df04e pc df18a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // df18a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // df18d imul eax,DWORD PTR [rbp-0x78]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64); rax = (r) as u32 as u64; fl = f; }
            // df191 shl eax,0x2
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // df194 add rsp,0x78
            { let (r, f) = add::<64>(rsp, 0x78u64);
              rsp = r;
              fl = f; }
            // df198 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // df199 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // df19b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // df19d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // df19f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // df1a1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // df1a2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x645eb size=351
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000645eb<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x645eb;
    loop {
        match pc {
        0x645eb => {
            // block 645eb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645eb push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 645ec mov rbp,rsp
            rbp = rsp;
            // 645ef push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 645f0 movzx ebx,WORD PTR [rdi]
            rbx = (m.r16(rdi) as u64) as u32 as u64;
            // 645f3 cmp ebx,0xc350
            fl = sub::<32>((rbx as u32 as u64), 0xc350u64).1;
            // 645f9 ja 0x64656
            if crate::cov::brk(0x645f9, fl.a()) { pc = 0x64656; continue; }
            pc = 0x645fb;
        }
        0x645fb => {
            // block 645fb
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 645fb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 645fb movzx eax,WORD PTR [rdi+0x2]
            rax = (m.r16(rdi.wrapping_add(0x2)) as u64) as u32 as u64;
            // 645ff cmp eax,0xc350
            fl = sub::<32>((rax as u32 as u64), 0xc350u64).1;
            // 64604 ja 0x64656
            if crate::cov::brk(0x64604, fl.a()) { pc = 0x64656; continue; }
            pc = 0x64606;
        }
        0x64606 => {
            // block 64606
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64606 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64606 movzx edx,WORD PTR [rdi+0x4]
            rdx = (m.r16(rdi.wrapping_add(0x4)) as u64) as u32 as u64;
            // 6460a cmp edx,0xc350
            fl = sub::<32>((rdx as u32 as u64), 0xc350u64).1;
            // 64610 ja 0x64656
            if crate::cov::brk(0x64610, fl.a()) { pc = 0x64656; continue; }
            pc = 0x64612;
        }
        0x64612 => {
            // block 64612
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64612 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64612 movzx ecx,WORD PTR [rdi+0x6]
            rcx = (m.r16(rdi.wrapping_add(0x6)) as u64) as u32 as u64;
            // 64616 cmp ecx,0xc350
            fl = sub::<32>((rcx as u32 as u64), 0xc350u64).1;
            // 6461c ja 0x64656
            if crate::cov::brk(0x6461c, fl.a()) { pc = 0x64656; continue; }
            pc = 0x6461e;
        }
        0x6461e => {
            // block 6461e
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6461e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6461e movzx r11d,WORD PTR [rdi+0x8]
            r11 = (m.r16(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 64623 cmp r11d,0xc350
            fl = sub::<32>((r11 as u32 as u64), 0xc350u64).1;
            // 6462a ja 0x64656
            if crate::cov::brk(0x6462a, fl.a()) { pc = 0x64656; continue; }
            pc = 0x6462c;
        }
        0x6462c => {
            // block 6462c
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6462c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6462c movzx r10d,WORD PTR [rdi+0xa]
            r10 = (m.r16(rdi.wrapping_add(0xa)) as u64) as u32 as u64;
            // 64631 cmp r10d,0xc350
            fl = sub::<32>((r10 as u32 as u64), 0xc350u64).1;
            // 64638 ja 0x64656
            if crate::cov::brk(0x64638, fl.a()) { pc = 0x64656; continue; }
            pc = 0x6463a;
        }
        0x6463a => {
            // block 6463a
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6463a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6463a movzx r9d,WORD PTR [rdi+0xc]
            r9 = (m.r16(rdi.wrapping_add(0xc)) as u64) as u32 as u64;
            // 6463f cmp r9d,0xc350
            fl = sub::<32>((r9 as u32 as u64), 0xc350u64).1;
            // 64646 ja 0x64656
            if crate::cov::brk(0x64646, fl.a()) { pc = 0x64656; continue; }
            pc = 0x64648;
        }
        0x64648 => {
            // block 64648
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64648 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64648 movzx r8d,WORD PTR [rdi+0xe]
            r8 = (m.r16(rdi.wrapping_add(0xe)) as u64) as u32 as u64;
            // 6464d cmp r8d,0xc350
            fl = sub::<32>((r8 as u32 as u64), 0xc350u64).1;
            // 64654 jbe 0x6465b
            if crate::cov::brk(0x64654, fl.be()) { pc = 0x6465b; continue; }
            pc = 0x64656;
        }
        0x64656 => {
            // block 64656
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64656 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64656 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x64658;
        }
        0x64658 => {
            // block 64658
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 64658 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64658 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 64659 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6465a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6465b => {
            // block 6465b
            #[cfg(feature = "trace")] eprintln!("f_000645eb pc 6465b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6465b cvtsi2sd xmm1,ebx
            x1 = set_lo64(x1, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // 6465f movsd xmm0,QWORD PTR [rip+0x9c139] # 0x1007a0
            x0 = m.r64(m.env.data_va(85869)) as u128;
            // 64667 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6466b cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 6466f movss DWORD PTR [rsi],xmm1
            m.w32(rsi, x1 as u32);
            // 64673 xorps xmm1,xmm1
            x1 = 0;
            // 64676 cvtsi2sd xmm1,eax
            x1 = set_lo64(x1, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6467a mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 6467e cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64682 movss DWORD PTR [rsi+0x4],xmm1
            m.w32(rsi.wrapping_add(0x4), x1 as u32);
            // 64687 xorps xmm1,xmm1
            x1 = 0;
            // 6468a cvtsi2sd xmm1,edx
            x1 = set_lo64(x1, (sx::<32>((rdx as u32 as u64)) as f64).to_bits());
            // 6468e mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 64692 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 64696 movss DWORD PTR [rsi+0x8],xmm1
            m.w32(rsi.wrapping_add(0x8), x1 as u32);
            // 6469b xorps xmm1,xmm1
            x1 = 0;
            // 6469e cvtsi2sd xmm1,ecx
            x1 = set_lo64(x1, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // 646a2 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646a6 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646aa movss DWORD PTR [rsi+0xc],xmm1
            m.w32(rsi.wrapping_add(0xc), x1 as u32);
            // 646af xorps xmm1,xmm1
            x1 = 0;
            // 646b2 cvtsi2sd xmm1,r11d
            x1 = set_lo64(x1, (sx::<32>((r11 as u32 as u64)) as f64).to_bits());
            // 646b7 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646bb cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646bf movss DWORD PTR [rsi+0x10],xmm1
            m.w32(rsi.wrapping_add(0x10), x1 as u32);
            // 646c4 xorps xmm1,xmm1
            x1 = 0;
            // 646c7 cvtsi2sd xmm1,r10d
            x1 = set_lo64(x1, (sx::<32>((r10 as u32 as u64)) as f64).to_bits());
            // 646cc mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646d0 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646d4 movss DWORD PTR [rsi+0x14],xmm1
            m.w32(rsi.wrapping_add(0x14), x1 as u32);
            // 646d9 xorps xmm1,xmm1
            x1 = 0;
            // 646dc cvtsi2sd xmm1,r9d
            x1 = set_lo64(x1, (sx::<32>((r9 as u32 as u64)) as f64).to_bits());
            // 646e1 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646e5 cvtsd2ss xmm1,xmm1
            x1 = set_lo32(x1, (f64_of(x1) as f32).to_bits());
            // 646e9 movss DWORD PTR [rsi+0x18],xmm1
            m.w32(rsi.wrapping_add(0x18), x1 as u32);
            // 646ee xorps xmm1,xmm1
            x1 = 0;
            // 646f1 cvtsi2sd xmm1,r8d
            x1 = set_lo64(x1, (sx::<32>((r8 as u32 as u64)) as f64).to_bits());
            // 646f6 mulsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) * f64_of(x0)).to_bits());
            // 646fa cvtsd2ss xmm0,xmm1
            x0 = set_lo32(x0, (f64_of(x1) as f32).to_bits());
            // 646fe movss DWORD PTR [rsi+0x1c],xmm0
            m.w32(rsi.wrapping_add(0x1c), x0 as u32);
            // 64703 movzx eax,WORD PTR [rdi+0x12]
            rax = (m.r16(rdi.wrapping_add(0x12)) as u64) as u32 as u64;
            // 64707 xorps xmm0,xmm0
            x0 = 0;
            // 6470a cvtsi2sd xmm0,eax
            x0 = set_lo64(x0, (sx::<32>((rax as u32 as u64)) as f64).to_bits());
            // 6470e mulsd xmm0,QWORD PTR [rip+0x9c092] # 0x1007a8
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85877)) as u128))).to_bits());
            // 64716 cvtsd2ss xmm0,xmm0
            x0 = set_lo32(x0, (f64_of(x0) as f32).to_bits());
            // 6471a movss DWORD PTR [rsi+0x24],xmm0
            m.w32(rsi.wrapping_add(0x24), x0 as u32);
            // 6471f mov ax,WORD PTR [rdi+0x10]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x10)) as u64) as u16 as u64);
            // 64723 mov WORD PTR [rsi+0x20],ax
            m.w16(rsi.wrapping_add(0x20), ((rax as u16 as u64)) as u16);
            // 64727 mov ax,WORD PTR [rdi+0x14]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x14)) as u64) as u16 as u64);
            // 6472b mov WORD PTR [rsi+0x28],ax
            m.w16(rsi.wrapping_add(0x28), ((rax as u16 as u64)) as u16);
            // 6472f mov ax,WORD PTR [rdi+0x16]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x16)) as u64) as u16 as u64);
            // 64733 mov WORD PTR [rsi+0x2a],ax
            m.w16(rsi.wrapping_add(0x2a), ((rax as u16 as u64)) as u16);
            // 64737 mov al,BYTE PTR [rdi+0x18]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x18)) as u64) as u8 as u64);
            // 6473a mov BYTE PTR [rsi+0x2c],al
            m.w8(rsi.wrapping_add(0x2c), ((rax as u8 as u64)) as u8);
            // 6473d mov al,BYTE PTR [rdi+0x19]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x19)) as u64) as u8 as u64);
            // 64740 mov BYTE PTR [rsi+0x2d],al
            m.w8(rsi.wrapping_add(0x2d), ((rax as u8 as u64)) as u8);
            // 64743 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 64745 jmp 0x64658
            pc = 0x64658; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28551 size=356
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028551<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28551;
    loop {
        match pc {
        0x28551 => {
            // block 28551
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28551 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28551 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28552 mov rbp,rsp
            rbp = rsp;
            // 28555 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28557 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28559 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2855b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2855d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2855e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28562 mov r14,rsi
            r14 = rsi;
            // 28565 mov r15,rdi
            r15 = rdi;
            // 28568 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // 2856b lea rsi,[rip+0x1021e2] # 0x12a754
            rsi = m.env.data_va(257825);
            // 28572 mov edx,0xa
            rdx = (0xau64) as u32 as u64;
            // 28577 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2857c mov rsi,QWORD PTR [r14+0x10]
            rsi = m.r64(r14.wrapping_add(0x10)) as u64;
            // 28580 mov rdi,rax
            rdi = rax;
            // 28583 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28588 lea rsi,[rip+0x101ec7] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2858f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 28594 mov rdi,rax
            rdi = rax;
            // 28597 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2859c add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 285a0 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 285a3 je 0x2867e
            if crate::cov::brk(0x285a3, fl.e()) { pc = 0x2867e; continue; }
            pc = 0x285a9;
        }
        0x285a9 => {
            // block 285a9
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285a9 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 285ad jmp 0x285c9
            pc = 0x285c9; continue;
        }
        0x285af => {
            // block 285af
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285af mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 285b4 mov rdi,r15
            rdi = r15;
            // 285b7 lea rsi,[rip+0x1021a5] # 0x12a763
            rsi = m.env.data_va(257840);
            // 285be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 285c3 mov r12,rbx
            r12 = rbx;
            // 285c6 mov rbx,r13
            rbx = r13;
            pc = 0x285c9;
        }
        0x285c9 => {
            // block 285c9
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285c9 mov esi,DWORD PTR [r12+0x1c]
            rsi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 285ce mov r13d,DWORD PTR [r12+0x20]
            r13 = (m.r32(r12.wrapping_add(0x20)) as u64) as u32 as u64;
            // 285d3 mov rdi,rbx
            rdi = rbx;
            // 285d6 call 0x60623
            { let r = { let t = m.env.code_va(0x60623); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 285db movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 285df test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 285e2 jne 0x285ed
            if crate::cov::brk(0x285e2, fl.ne()) { pc = 0x285ed; continue; }
            pc = 0x285e4;
        }
        0x285e4 => {
            // block 285e4
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285e4 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 285e7 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 285eb jmp 0x285f5
            pc = 0x285f5; continue;
        }
        0x285ed => {
            // block 285ed
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285ed mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 285f1 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x285f5;
        }
        0x285f5 => {
            // block 285f5
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 285f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 285f5 mov rdi,r15
            rdi = r15;
            // 285f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 285fd mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // 28602 mov rdi,rax
            rdi = rax;
            // 28605 lea rsi,[rip+0x102153] # 0x12a75f
            rsi = m.env.data_va(257836);
            // 2860c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28611 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 28614 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 28618 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2861c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2861f or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28622 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 28626 mov rdi,rax
            rdi = rax;
            // 28629 mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 2862c call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28631 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 28634 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 28638 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2863c and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2863f or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28642 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 28646 mov rdi,rbx
            rdi = rbx;
            // 28649 mov r13,rbx
            r13 = rbx;
            // 2864c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28651 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 28656 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28659 je 0x28668
            if crate::cov::brk(0x28659, fl.e()) { pc = 0x28668; continue; }
            pc = 0x2865b;
        }
        0x2865b => {
            // block 2865b
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 2865b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2865b mov rbx,rax
            rbx = rax;
            // 2865e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28661 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28664 jne 0x2865b
            if crate::cov::brk(0x28664, fl.ne()) { pc = 0x2865b; continue; }
            pc = 0x28666;
        }
        0x28666 => {
            // block 28666
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28666 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28666 jmp 0x28675
            pc = 0x28675; continue;
        }
        0x28668 => {
            // block 28668
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28668 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28668 mov rbx,QWORD PTR [r12+0x10]
            rbx = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2866d cmp QWORD PTR [rbx],r12
            fl = sub::<64>(m.r64(rbx) as u64, r12).1;
            // 28670 mov r12,rbx
            r12 = rbx;
            // 28673 jne 0x28668
            if crate::cov::brk(0x28673, fl.ne()) { pc = 0x28668; continue; }
            pc = 0x28675;
        }
        0x28675 => {
            // block 28675
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 28675 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28675 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 28678 jne 0x285af
            if crate::cov::brk(0x28678, fl.ne()) { pc = 0x285af; continue; }
            pc = 0x2867e;
        }
        0x2867e => {
            // block 2867e
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 2867e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2867e lea rsi,[rip+0x101dcf] # 0x12a454
            rsi = m.env.data_va(257057);
            // 28685 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2868a mov rdi,r15
            rdi = r15;
            // 2868d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28692 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28696 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28697 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28699 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2869b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2869d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2869f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 286a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x286a1 => {
            // block 286a1
            #[cfg(feature = "trace")] eprintln!("f_00028551 pc 286a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 286a1 mov rbx,rax
            rbx = rax;
            // 286a4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 286a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 286ad mov rdi,rbx
            rdi = rbx;
            // 286b0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15158 size=360
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_00015158<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15158;
    loop {
        match pc {
        0x15158 => {
            // block 15158
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15158 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15158 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15159 mov rbp,rsp
            rbp = rsp;
            // 1515c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1515e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15160 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15162 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15164 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15165 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 15166 mov r15d,ecx
            r15 = ((rcx as u32 as u64)) as u32 as u64;
            // 15169 mov r12,rdx
            r12 = rdx;
            // 1516c mov r14,rsi
            r14 = rsi;
            // 1516f mov rbx,rdi
            rbx = rdi;
            // 15172 mov rdi,QWORD PTR [r14+0x80]
            rdi = m.r64(r14.wrapping_add(0x80)) as u64;
            // 15179 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 1517c je 0x15294
            if crate::cov::brk(0x1517c, fl.e()) { pc = 0x15294; continue; }
            pc = 0x15182;
        }
        0x15182 => {
            // block 15182
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15182 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 15185 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 15188 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 1518b cmp QWORD PTR [r14+0x78],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15190 je 0x151d1
            if crate::cov::brk(0x15190, fl.e()) { pc = 0x151d1; continue; }
            pc = 0x15192;
        }
        0x15192 => {
            // block 15192
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15192 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 15195 je 0x1519c
            if crate::cov::brk(0x15195, fl.e()) { pc = 0x1519c; continue; }
            pc = 0x15197;
        }
        0x15197 => {
            // block 15197
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15197 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15197 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 1519a jle 0x151d1
            if crate::cov::brk(0x1519a, fl.le()) { pc = 0x151d1; continue; }
            pc = 0x1519c;
        }
        0x1519c => {
            // block 1519c
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1519c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1519c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1519f mov rdi,r14
            rdi = r14;
            // 151a2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 151a5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151a7 jne 0x151d1
            if crate::cov::brk(0x151a7, fl.ne()) { pc = 0x151d1; continue; }
            pc = 0x151a9;
        }
        0x151a9 => {
            // block 151a9
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151a9 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 151ad jae 0x151d1
            if crate::cov::brk(0x151ad, fl.ae()) { pc = 0x151d1; continue; }
            pc = 0x151af;
        }
        0x151af => {
            // block 151af
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151af mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 151b3 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 151b5 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 151b8 jle 0x151c1
            if crate::cov::brk(0x151b8, fl.le()) { pc = 0x151c1; continue; }
            pc = 0x151ba;
        }
        0x151ba => {
            // block 151ba
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151ba movsxd rsi,r13d
            rsi = sx::<32>((r13 as u32 as u64)) as u64;
            // 151bd imul rsi,r12
            { let (r, f) = imul::<64>(rsi, r12); rsi = r; fl = f; }
            pc = 0x151c1;
        }
        0x151c1 => {
            // block 151c1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151c1 mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 151c4 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 151c9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151cb je 0x1526d
            if crate::cov::brk(0x151cb, fl.e()) { pc = 0x1526d; continue; }
            pc = 0x151d1;
        }
        0x151d1 => {
            // block 151d1
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151d1 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 151d9 mov QWORD PTR [rbx+0x70],0x0
            m.w64(rbx.wrapping_add(0x70), (0x0u64) as u64);
            // 151e1 mov QWORD PTR [rbx+0x68],0x0
            m.w64(rbx.wrapping_add(0x68), (0x0u64) as u64);
            // 151e9 mov QWORD PTR [rbx+0x60],0x0
            m.w64(rbx.wrapping_add(0x60), (0x0u64) as u64);
            // 151f1 mov QWORD PTR [rbx+0x58],0x0
            m.w64(rbx.wrapping_add(0x58), (0x0u64) as u64);
            // 151f9 mov QWORD PTR [rbx+0x50],0x0
            m.w64(rbx.wrapping_add(0x50), (0x0u64) as u64);
            // 15201 mov QWORD PTR [rbx+0x48],0x0
            m.w64(rbx.wrapping_add(0x48), (0x0u64) as u64);
            // 15209 mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 15211 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 15219 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // 15221 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // 15229 mov QWORD PTR [rbx+0x20],0x0
            m.w64(rbx.wrapping_add(0x20), (0x0u64) as u64);
            // 15231 mov QWORD PTR [rbx+0x18],0x0
            m.w64(rbx.wrapping_add(0x18), (0x0u64) as u64);
            // 15239 mov QWORD PTR [rbx+0x10],0x0
            m.w64(rbx.wrapping_add(0x10), (0x0u64) as u64);
            // 15241 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            // 15249 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 15250 mov QWORD PTR [rbx+0x80],0xffffffffffffffff
            m.w64(rbx.wrapping_add(0x80), (0xffff_ffff_ffff_ffffu64) as u64);
            pc = 0x1525b;
        }
        0x1525b => {
            // block 1525b
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1525b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1525b mov rax,rbx
            rax = rbx;
            // 1525e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 15262 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15263 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 15265 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 15267 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15269 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1526b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1526c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x1526d => {
            // block 1526d
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 1526d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1526d mov rdi,QWORD PTR [r14+0x78]
            rdi = m.r64(r14.wrapping_add(0x78)) as u64;
            // 15271 call 0xec1e2
            { let r = { let t = m.env.stub_va(0x9af); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 15276 add r14,0x88
            { let (r, f) = add::<64>(r14, 0x88u64);
              r14 = r;
              fl = f; }
            // 1527d mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 15282 mov rdi,rbx
            rdi = rbx;
            // 15285 mov rsi,r14
            rsi = r14;
            // 15288 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // 1528b mov QWORD PTR [rbx+0x80],rax
            m.w64(rbx.wrapping_add(0x80), (rax) as u64);
            // 15292 jmp 0x1525b
            pc = 0x1525b; continue;
        }
        0x15294 => {
            // block 15294
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15294 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15294 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // 15299 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1529e mov rbx,rax
            rbx = rax;
            // 152a1 mov rdi,rbx
            rdi = rbx;
            // 152a4 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 152a9 mov rsi,QWORD PTR [rip+0x12af30] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // 152b0 mov rdx,QWORD PTR [rip+0x12aef1] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // 152b7 mov rdi,rbx
            rdi = rbx;
            // 152ba call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe86f8 size=364
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e86f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe86f8;
    loop {
        match pc {
        0xe86f8 => {
            // block e86f8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e86f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e86f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e86f9 mov rbp,rsp
            rbp = rsp;
            // e86fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e86fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e8700 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e8702 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e8704 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e8705 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e8706 mov r15,rsi
            r15 = rsi;
            // e8709 mov r14,rdi
            r14 = rdi;
            // e870c lea rsi,[r15+0x2a8]
            rsi = r15.wrapping_add(0x2a8);
            // e8713 movzx r13d,BYTE PTR [r15+0x2a8]
            r13 = (m.r8(r15.wrapping_add(0x2a8)) as u64) as u32 as u64;
            // e871b mov r11b,r13b
            r11 = (r11 & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // e871e and r11b,0x1
            { let r = (r11 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r11 = (r11 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e8722 jne 0xe8730
            if crate::cov::brk(0xe8722, fl.ne()) { pc = 0xe8730; continue; }
            pc = 0xe8724;
        }
        0xe8724 => {
            // block e8724
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8724 lea r8,[rsi+0x1]
            r8 = rsi.wrapping_add(0x1);
            // e8728 mov rdi,r13
            rdi = r13;
            // e872b shr rdi,1
            { let (r, f) = shr::<64>(rdi, 0x1u64, fl); rdi = r; fl = f; }
            // e872e jmp 0xe873e
            pc = 0xe873e; continue;
        }
        0xe8730 => {
            // block e8730
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8730 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8730 mov rdi,QWORD PTR [r15+0x2b0]
            rdi = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e8737 mov r8,QWORD PTR [r15+0x2b8]
            r8 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe873e;
        }
        0xe873e => {
            // block e873e
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e873e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e873e mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8743 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8747 jb 0xe87bc
            if crate::cov::brk(0xe8747, fl.b()) { pc = 0xe87bc; continue; }
            pc = 0xe8749;
        }
        0xe8749 => {
            // block e8749
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8749 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e874e cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8752 jl 0xe87bc
            if crate::cov::brk(0xe8752, fl.l()) { pc = 0xe87bc; continue; }
            pc = 0xe8754;
        }
        0xe8754 => {
            // block e8754
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8754 mov rax,rdi
            rax = rdi;
            // e8757 add rax,0xfffffffffffffffd
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fffdu64);
              rax = r;
              fl = f; }
            // e875b mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8760 je 0xe87bc
            if crate::cov::brk(0xe8760, fl.e()) { pc = 0xe87bc; continue; }
            pc = 0xe8762;
        }
        0xe8762 => {
            // block e8762
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8762 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8762 lea r9,[r8+rdi*1]
            r9 = r8.wrapping_add(rdi);
            // e8766 lea r10,[r8+rdi*1-0x3]
            r10 = r8.wrapping_add(rdi).wrapping_add(0xffff_ffff_ffff_fffd);
            // e876b lea r12,[rip+0x54ad7] # 0x13d249
            r12 = m.env.data_va(334358);
            // e8772 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e8777 mov rdi,r8
            rdi = r8;
            pc = 0xe877a;
        }
        0xe877a => {
            // block e877a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e877a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e877a movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e877d cmp eax,0x44
            fl = sub::<32>((rax as u32 as u64), 0x44u64).1;
            // e8780 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // e8785 jne 0xe879d
            if crate::cov::brk(0xe8785, fl.ne()) { pc = 0xe879d; continue; }
            pc = 0xe8787;
        }
        0xe8787 => {
            // block e8787
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8787 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8787 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e878b je 0xe87a7
            if crate::cov::brk(0xe878b, fl.e()) { pc = 0xe87a7; continue; }
            pc = 0xe878d;
        }
        0xe878d => {
            // block e878d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e878d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e878d movzx ebx,BYTE PTR [rax+r12*1]
            rbx = (m.r8(rax.wrapping_add(r12)) as u64) as u32 as u64;
            // e8792 movzx ecx,BYTE PTR [rdi+rax*1]
            rcx = (m.r8(rdi.wrapping_add(rax)) as u64) as u32 as u64;
            // e8796 inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // e8799 cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // e879b je 0xe8787
            if crate::cov::brk(0xe879b, fl.e()) { pc = 0xe8787; continue; }
            pc = 0xe879d;
        }
        0xe879d => {
            // block e879d
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e879d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e879d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e87a0 cmp rdi,r10
            fl = sub::<64>(rdi, r10).1;
            // e87a3 jne 0xe877a
            if crate::cov::brk(0xe87a3, fl.ne()) { pc = 0xe877a; continue; }
            pc = 0xe87a5;
        }
        0xe87a5 => {
            // block e87a5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a5 jmp 0xe87bc
            pc = 0xe87bc; continue;
        }
        0xe87a7 => {
            // block e87a7
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87a7 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // e87ac cmp rdi,r9
            fl = sub::<64>(rdi, r9).1;
            // e87af je 0xe87bc
            if crate::cov::brk(0xe87af, fl.e()) { pc = 0xe87bc; continue; }
            pc = 0xe87b1;
        }
        0xe87b1 => {
            // block e87b1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87b1 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // e87b6 sub rdx,r8
            { let (r, f) = sub::<64>(rdx, r8);
              rdx = r;
              fl = f; }
            // e87b9 add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            pc = 0xe87bc;
        }
        0xe87bc => {
            // block e87bc
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87bc test r11b,r11b
            fl = logic::<8>((r11 as u8 as u64) & (r11 as u8 as u64));
            // e87bf jne 0xe87ca
            if crate::cov::brk(0xe87bf, fl.ne()) { pc = 0xe87ca; continue; }
            pc = 0xe87c1;
        }
        0xe87c1 => {
            // block e87c1
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87c1 lea r10,[rsi+0x1]
            r10 = rsi.wrapping_add(0x1);
            // e87c5 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // e87c8 jmp 0xe87d8
            pc = 0xe87d8; continue;
        }
        0xe87ca => {
            // block e87ca
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87ca mov r13,QWORD PTR [r15+0x2b0]
            r13 = m.r64(r15.wrapping_add(0x2b0)) as u64;
            // e87d1 mov r10,QWORD PTR [r15+0x2b8]
            r10 = m.r64(r15.wrapping_add(0x2b8)) as u64;
            pc = 0xe87d8;
        }
        0xe87d8 => {
            // block e87d8
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87d8 mov r11,0xffffffffffffffff
            r11 = 0xffff_ffff_ffff_ffffu64;
            // e87df cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e3 jb 0xe883f
            if crate::cov::brk(0xe87e3, fl.b()) { pc = 0xe883f; continue; }
            pc = 0xe87e5;
        }
        0xe87e5 => {
            // block e87e5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87e5 cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e9 jl 0xe883f
            if crate::cov::brk(0xe87e9, fl.l()) { pc = 0xe883f; continue; }
            pc = 0xe87eb;
        }
        0xe87eb => {
            // block e87eb
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87eb mov rcx,r13
            rcx = r13;
            // e87ee add rcx,0xfffffffffffffffd
            { let (r, f) = add::<64>(rcx, 0xffff_ffff_ffff_fffdu64);
              rcx = r;
              fl = f; }
            // e87f2 je 0xe883f
            if crate::cov::brk(0xe87f2, fl.e()) { pc = 0xe883f; continue; }
            pc = 0xe87f4;
        }
        0xe87f4 => {
            // block e87f4
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87f4 lea r8,[r10+r13*1]
            r8 = r10.wrapping_add(r13);
            // e87f8 lea r9,[r10+r13*1-0x3]
            r9 = r10.wrapping_add(r13).wrapping_add(0xffff_ffff_ffff_fffd);
            // e87fd lea r15,[rip+0x54a45] # 0x13d249
            r15 = m.env.data_va(334358);
            // e8804 mov rcx,r10
            rcx = r10;
            pc = 0xe8807;
        }
        0xe8807 => {
            // block e8807
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8807 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8807 movzx edi,BYTE PTR [rcx]
            rdi = (m.r8(rcx) as u64) as u32 as u64;
            // e880a cmp edi,0x44
            fl = sub::<32>((rdi as u32 as u64), 0x44u64).1;
            // e880d mov edi,0x1
            rdi = (0x1u64) as u32 as u64;
            // e8812 jne 0xe882a
            if crate::cov::brk(0xe8812, fl.ne()) { pc = 0xe882a; continue; }
            pc = 0xe8814;
        }
        0xe8814 => {
            // block e8814
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8814 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8818 je 0xe8834
            if crate::cov::brk(0xe8818, fl.e()) { pc = 0xe8834; continue; }
            pc = 0xe881a;
        }
        0xe881a => {
            // block e881a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e881a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e881a movzx eax,BYTE PTR [rdi+r15*1]
            rax = (m.r8(rdi.wrapping_add(r15)) as u64) as u32 as u64;
            // e881f movzx ebx,BYTE PTR [rcx+rdi*1]
            rbx = (m.r8(rcx.wrapping_add(rdi)) as u64) as u32 as u64;
            // e8823 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e8826 cmp ebx,eax
            fl = sub::<32>((rbx as u32 as u64), (rax as u32 as u64)).1;
            // e8828 je 0xe8814
            if crate::cov::brk(0xe8828, fl.e()) { pc = 0xe8814; continue; }
            pc = 0xe882a;
        }
        0xe882a => {
            // block e882a
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e882a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e882a inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // e882d cmp rcx,r9
            fl = sub::<64>(rcx, r9).1;
            // e8830 jne 0xe8807
            if crate::cov::brk(0xe8830, fl.ne()) { pc = 0xe8807; continue; }
            pc = 0xe8832;
        }
        0xe8832 => {
            // block e8832
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8832 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8832 jmp 0xe883f
            pc = 0xe883f; continue;
        }
        0xe8834 => {
            // block e8834
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8834 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8834 cmp rcx,r8
            fl = sub::<64>(rcx, r8).1;
            // e8837 je 0xe883f
            if crate::cov::brk(0xe8837, fl.e()) { pc = 0xe883f; continue; }
            pc = 0xe8839;
        }
        0xe8839 => {
            // block e8839
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8839 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8839 sub rcx,r10
            { let (r, f) = sub::<64>(rcx, r10);
              rcx = r;
              fl = f; }
            // e883c mov r11,rcx
            r11 = rcx;
            pc = 0xe883f;
        }
        0xe883f => {
            // block e883f
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e883f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e883f mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e8844 sub rcx,r11
            { let (r, f) = sub::<64>(rcx, r11);
              rcx = r;
              fl = f; }
            // e8847 mov rdi,r14
            rdi = r14;
            // e884a mov r8,rsi
            r8 = rsi;
            // e884d call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e8852 mov rax,r14
            rax = r14;
            // e8855 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8859 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e885a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e885c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e885e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e8860 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e8862 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8863 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc362 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc362<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc362;
    loop {
        match pc {
        0xbc362 => {
            // block bc362
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc362 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc362 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc363 mov rbp,rsp
            rbp = rsp;
            // bc366 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc368 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc36a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bc36c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc36e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc36f sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc373 mov rbx,rsi
            rbx = rsi;
            // bc376 mov r14,rdi
            r14 = rdi;
            // bc379 mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // bc37c mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc380 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bc383 je 0xbc39a
            if crate::cov::brk(0xbc383, fl.e()) { pc = 0xbc39a; continue; }
            pc = 0xbc385;
        }
        0xbc385 => {
            // block bc385
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc385 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc385 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bc389 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bc38c not rdx
            { let v = !rdx; rdx = v; }
            // bc38f and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bc393 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bc396 mov QWORD PTR [rbx+0x8],rdx
            m.w64(rbx.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbc39a;
        }
        0xbc39a => {
            // block bc39a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc39a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc39a lea r15,[rbp-0x38]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // bc39e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // bc3a6 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // bc3ae mov QWORD PTR [rbp-0x40],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r15) as u64);
            // bc3b2 mov DWORD PTR [rbp-0x44],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), (0x0u64) as u32);
            // bc3b9 mov DWORD PTR [rbp-0x48],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0x0u64) as u32);
            // bc3c0 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // bc3c3 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // bc3c7 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // bc3c9 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc3cc je 0xbc4a2
            if crate::cov::brk(0xbc3cc, fl.e()) { pc = 0xbc4a2; continue; }
            pc = 0xbc3d2;
        }
        0xbc3d2 => {
            // block bc3d2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d2 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            pc = 0xbc3d6;
        }
        0xbc3d6 => {
            // block bc3d6
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3d6 mov edi,DWORD PTR [r13+0x1c]
            rdi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bc3da lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // bc3de mov rdx,r12
            rdx = r12;
            // bc3e1 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc3e6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc3e8 je 0xbc46e
            if crate::cov::brk(0xbc3e8, fl.e()) { pc = 0xbc46e; continue; }
            pc = 0xbc3ee;
        }
        0xbc3ee => {
            // block bc3ee
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3ee mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc3f2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc3f5 mov eax,DWORD PTR [rbp-0x44]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc)) as u64) as u32 as u64;
            // bc3f8 mov rdx,r15
            rdx = r15;
            // bc3fb je 0xbc42a
            if crate::cov::brk(0xbc3fb, fl.e()) { pc = 0xbc42a; continue; }
            pc = 0xbc3fd;
        }
        0xbc3fd => {
            // block bc3fd
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc3fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc3fd mov rcx,rsi
            rcx = rsi;
            pc = 0xbc400;
        }
        0xbc400 => {
            // block bc400
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc400 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc400 cmp DWORD PTR [rcx+0x1c],eax
            fl = sub::<32>(m.r32(rcx.wrapping_add(0x1c)) as u64, (rax as u32 as u64)).1;
            // bc403 jae 0xbc410
            if crate::cov::brk(0xbc403, fl.ae()) { pc = 0xbc410; continue; }
            pc = 0xbc405;
        }
        0xbc405 => {
            // block bc405
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc405 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc405 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // bc409 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // bc40c jne 0xbc400
            if crate::cov::brk(0xbc40c, fl.ne()) { pc = 0xbc400; continue; }
            pc = 0xbc40e;
        }
        0xbc40e => {
            // block bc40e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc40e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc40e jmp 0xbc41d
            pc = 0xbc41d; continue;
        }
        0xbc410 => {
            // block bc410
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc410 mov rsi,QWORD PTR [rcx]
            rsi = m.r64(rcx) as u64;
            // bc413 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // bc416 mov rdx,rcx
            rdx = rcx;
            // bc419 jne 0xbc3fd
            if crate::cov::brk(0xbc419, fl.ne()) { pc = 0xbc3fd; continue; }
            pc = 0xbc41b;
        }
        0xbc41b => {
            // block bc41b
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41b jmp 0xbc420
            pc = 0xbc420; continue;
        }
        0xbc41d => {
            // block bc41d
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc41d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc41d mov rcx,rdx
            rcx = rdx;
            pc = 0xbc420;
        }
        0xbc420 => {
            // block bc420
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc420 cmp rcx,r15
            fl = sub::<64>(rcx, r15).1;
            // bc423 je 0xbc42a
            if crate::cov::brk(0xbc423, fl.e()) { pc = 0xbc42a; continue; }
            pc = 0xbc425;
        }
        0xbc425 => {
            // block bc425
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc425 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc425 cmp eax,DWORD PTR [rcx+0x1c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rcx.wrapping_add(0x1c)) as u64).1;
            // bc428 jae 0xbc46e
            if crate::cov::brk(0xbc428, fl.ae()) { pc = 0xbc46e; continue; }
            pc = 0xbc42a;
        }
        0xbc42a => {
            // block bc42a
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc42a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc42a mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // bc42d mov DWORD PTR [rbp-0x54],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), (0x0u64) as u32);
            // bc434 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // bc43b mov DWORD PTR [rbp-0x4c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), (0x0u64) as u32);
            // bc442 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // bc446 cmp rax,QWORD PTR [rbx+0x10]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // bc44a jae 0xbc462
            if crate::cov::brk(0xbc44a, fl.ae()) { pc = 0xbc462; continue; }
            pc = 0xbc44c;
        }
        0xbc44c => {
            // block bc44c
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc44c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc44c mov rcx,QWORD PTR [rbp-0x58]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // bc450 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // bc454 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // bc458 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // bc45b add QWORD PTR [rbx+0x8],0x10
            { let (r, f) = add::<64>(m.r64(rbx.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(rbx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // bc460 jmp 0xbc46e
            pc = 0xbc46e; continue;
        }
        0xbc462 => {
            // block bc462
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc462 mov rdi,rbx
            rdi = rbx;
            // bc465 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // bc469 call 0xebbbe
            { let r = { let t = m.env.stub_va(0x38b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbc46e;
        }
        0xbc46e => {
            // block bc46e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc46e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc46e mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bc472 test rax,rax
            fl = logic::<64>(rax & rax);
            // bc475 je 0xbc485
            if crate::cov::brk(0xbc475, fl.e()) { pc = 0xbc485; continue; }
            pc = 0xbc477;
        }
        0xbc477 => {
            // block bc477
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc477 mov r13,rax
            r13 = rax;
            // bc47a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // bc47e test rax,rax
            fl = logic::<64>(rax & rax);
            // bc481 jne 0xbc477
            if crate::cov::brk(0xbc481, fl.ne()) { pc = 0xbc477; continue; }
            pc = 0xbc483;
        }
        0xbc483 => {
            // block bc483
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc483 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc483 jmp 0xbc495
            pc = 0xbc495; continue;
        }
        0xbc485 => {
            // block bc485
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc485 mov rax,r13
            rax = r13;
            pc = 0xbc488;
        }
        0xbc488 => {
            // block bc488
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc488 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // bc48c cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // bc490 mov rax,r13
            rax = r13;
            // bc493 jne 0xbc488
            if crate::cov::brk(0xbc493, fl.ne()) { pc = 0xbc488; continue; }
            pc = 0xbc495;
        }
        0xbc495 => {
            // block bc495
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc495 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc495 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // bc498 jne 0xbc3d6
            if crate::cov::brk(0xbc498, fl.ne()) { pc = 0xbc3d6; continue; }
            pc = 0xbc49e;
        }
        0xbc49e => {
            // block bc49e
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc49e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc49e mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0xbc4a2;
        }
        0xbc4a2 => {
            // block bc4a2
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4a2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4a6 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc4ab mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bc4ad add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // bc4b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc4b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc4b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bc4b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc4b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc4ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc4bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbc4bc => {
            // block bc4bc
            #[cfg(feature = "trace")] eprintln!("f_000bc362 pc bc4bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bc4bc mov rbx,rax
            rbx = rax;
            // bc4bf mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bc4c3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bc4c7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bc4cc mov rdi,rbx
            rdi = rbx;
            // bc4cf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10de4 size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010de4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10de4;
    loop {
        match pc {
        0x10de4 => {
            // block 10de4
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10de4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10de5 mov rbp,rsp
            rbp = rsp;
            // 10de8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10dea push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10dec push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 10dee push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 10df0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 10df1 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10df5 mov r15,rdi
            r15 = rdi;
            // 10df8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10dfc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10e00 cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // 10e03 jne 0x10f3d
            if crate::cov::brk(0x10e03, fl.ne()) { pc = 0x10f3d; continue; }
            pc = 0x10e09;
        }
        0x10e09 => {
            // block 10e09
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e09 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 10e0d mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10e10 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10e14 mov rax,rbx
            rax = rbx;
            // 10e17 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e1a jbe 0x10e6f
            if crate::cov::brk(0x10e1a, fl.be()) { pc = 0x10e6f; continue; }
            pc = 0x10e1c;
        }
        0x10e1c => {
            // block 10e1c
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e1c sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 10e20 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10e24 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10e28 lea r14,[rax+rcx*1+0x1]
            r14 = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10e2d sar r14,1
            { let (r, f) = sar::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 10e30 lea rax,[r14*8+0x0]
            rax = r14.wrapping_mul(8);
            // 10e38 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e3c mov rdi,rbx
            rdi = rbx;
            // 10e3f sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 10e42 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // 10e45 mov r13,r12
            r13 = r12;
            // 10e48 shr r13,0x3
            { let (r, f) = shr::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // 10e4c mov rsi,rbx
            rsi = rbx;
            // 10e4f mov rdx,r12
            rdx = r12;
            // 10e52 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 10e57 sub r13,r14
            { let (r, f) = sub::<64>(r13, r14);
              r13 = r;
              fl = f; }
            // 10e5a lea r12,[rbx+r13*8]
            r12 = rbx.wrapping_add(r13.wrapping_mul(8));
            // 10e5e mov QWORD PTR [r15+0x10],r12
            m.w64(r15.wrapping_add(0x10), (r12) as u64);
            // 10e62 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10e66 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10e6a jmp 0x10f39
            pc = 0x10f39; continue;
        }
        0x10e6f => {
            // block 10e6f
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10e6f mov rax,r12
            rax = r12;
            // 10e72 sub rax,rcx
            { let (r, f) = sub::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // 10e75 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10e79 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 10e7f cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 10e83 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10e8d and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10e90 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 10e94 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 10e98 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10e9c mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 10ea0 lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 10ea8 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 10ead mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10eb1 mov rcx,QWORD PTR [rbp-0x60]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 10eb5 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10eb9 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ebd mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10ec1 lea rdx,[rax+r14*8]
            rdx = rax.wrapping_add(r14.wrapping_mul(8));
            // 10ec5 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10ec9 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 10ecc je 0x10f06
            if crate::cov::brk(0x10ecc, fl.e()) { pc = 0x10f06; continue; }
            pc = 0x10ece;
        }
        0x10ece => {
            // block 10ece
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ece rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ece mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 10ed1 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10ed4 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 10ed8 add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10edc mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10ee0 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 10ee4 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 10ee7 jne 0x10ece
            if crate::cov::brk(0x10ee7, fl.ne()) { pc = 0x10ece; continue; }
            pc = 0x10ee9;
        }
        0x10ee9 => {
            // block 10ee9
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10ee9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // 10eec mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10ef0 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10ef4 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10ef8 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10efc mov r13,QWORD PTR [r15+0x18]
            r13 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10f00 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10f04 jmp 0x10f0d
            pc = 0x10f0d; continue;
        }
        0x10f06 => {
            // block 10f06
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f06 mov rsi,rcx
            rsi = rcx;
            // 10f09 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x10f0d;
        }
        0x10f0d => {
            // block 10f0d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f0d mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10f10 mov QWORD PTR [rbp-0x50],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rdi) as u64);
            // 10f14 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10f18 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 10f1c mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10f20 mov QWORD PTR [rbp-0x40],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r12) as u64);
            // 10f24 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10f28 mov QWORD PTR [rbp-0x38],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r13) as u64);
            // 10f2c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10f30 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 10f35 mov r12,QWORD PTR [r15+0x10]
            r12 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10f39;
        }
        0x10f39 => {
            // block 10f39
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f39 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x10f3d;
        }
        0x10f3d => {
            // block 10f3d
            #[cfg(feature = "trace")] eprintln!("f_00010de4 pc 10f3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10f3d mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 10f40 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 10f44 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10f49 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 10f4d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 10f4e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 10f50 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 10f52 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 10f54 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 10f56 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10f57 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xed4c size=380
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ed4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xed4c;
    loop {
        match pc {
        0xed4c => {
            // block ed4c
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ed4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ed4d mov rbp,rsp
            rbp = rsp;
            // ed50 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ed52 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ed54 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ed56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ed58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ed59 sub rsp,0xb8
            { let (r, f) = sub::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // ed60 mov r15,rdx
            r15 = rdx;
            // ed63 mov r13,rsi
            r13 = rsi;
            // ed66 mov rbx,rdi
            rbx = rdi;
            // ed69 lea r14,[r15+0x8]
            r14 = r15.wrapping_add(0x8);
            // ed6d mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            // ed71 mov rdi,r15
            rdi = r15;
            // ed74 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ed79 mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            // ed81 mov QWORD PTR [r15],r14
            m.w64(r15, (r14) as u64);
            // ed84 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // ed8c cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // ed90 je 0xedd5
            if crate::cov::brk(0xed90, fl.e()) { pc = 0xedd5; continue; }
            pc = 0xed92;
        }
        0xed92 => {
            // block ed92
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ed92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ed92 lea r12,[rbp-0xd0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // ed99 mov rdi,r12
            rdi = r12;
            // ed9c mov rsi,r13
            rsi = r13;
            // ed9f call 0x2b982
            { let r = { let t = m.env.code_va(0x2b982); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eda4 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // eda7 mov rax,QWORD PTR [rax+0xd0]
            rax = m.r64(rax.wrapping_add(0xd0)) as u64;
            // edae mov rdi,rbx
            rdi = rbx;
            // edb1 mov rsi,r12
            rsi = r12;
            // edb4 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // edb6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // edb8 je 0xeddc
            if crate::cov::brk(0xedb8, fl.e()) { pc = 0xeddc; continue; }
            pc = 0xedba;
        }
        0xedba => {
            // block edba
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edba lea rdi,[rbp-0xd0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff30);
            // edc1 mov rsi,r15
            rsi = r15;
            // edc4 call 0x2bbc2
            { let r = { let t = m.env.code_va(0x2bbc2); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // edc9 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // edcb jmp 0xee6c
            pc = 0xee6c; continue;
        }
        0xedd0 => {
            // block edd0
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edd0 jmp 0xee9e
            pc = 0xee9e; continue;
        }
        0xedd5 => {
            // block edd5
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc edd5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // edd5 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // edd7 jmp 0xee8a
            pc = 0xee8a; continue;
        }
        0xeddc => {
            // block eddc
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc eddc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eddc mov r14,QWORD PTR [r13+0x0]
            r14 = m.r64(r13) as u64;
            // ede0 mov QWORD PTR [rbp-0xe0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r13) as u64);
            // ede7 lea r13,[r13+0x8]
            r13 = r13.wrapping_add(0x8);
            // edeb jmp 0xee55
            pc = 0xee55; continue;
        }
        0xeded => {
            // block eded
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc eded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eded mov DWORD PTR [rbp-0xd4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c), (0x0u64) as u32);
            // edf7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // edfa mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // edfe mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ee02 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // ee07 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ee0a mov rdi,rbx
            rdi = rbx;
            // ee0d lea rdx,[rbp-0xd4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff2c);
            // ee14 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ee16 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // ee18 je 0xee30
            if crate::cov::brk(0xee18, fl.e()) { pc = 0xee30; continue; }
            pc = 0xee1a;
        }
        0xee1a => {
            // block ee1a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee1a lea rsi,[r14+0x1c]
            rsi = r14.wrapping_add(0x1c);
            // ee1e mov r12d,DWORD PTR [rbp-0xd4]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff2c)) as u64) as u32 as u64;
            // ee25 mov rdi,r15
            rdi = r15;
            // ee28 call 0xeb870
            { let r = { let t = m.env.stub_va(0x3d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ee2d mov DWORD PTR [rax],r12d
            m.w32(rax, ((r12 as u32 as u64)) as u32);
            pc = 0xee30;
        }
        0xee30 => {
            // block ee30
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee30 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // ee34 test rax,rax
            fl = logic::<64>(rax & rax);
            // ee37 je 0xee46
            if crate::cov::brk(0xee37, fl.e()) { pc = 0xee46; continue; }
            pc = 0xee39;
        }
        0xee39 => {
            // block ee39
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee39 mov r14,rax
            r14 = rax;
            // ee3c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ee3f test rax,rax
            fl = logic::<64>(rax & rax);
            // ee42 jne 0xee39
            if crate::cov::brk(0xee42, fl.ne()) { pc = 0xee39; continue; }
            pc = 0xee44;
        }
        0xee44 => {
            // block ee44
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee44 jmp 0xee55
            pc = 0xee55; continue;
        }
        0xee46 => {
            // block ee46
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee46 mov rax,r14
            rax = r14;
            pc = 0xee49;
        }
        0xee49 => {
            // block ee49
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee49 mov r14,QWORD PTR [rax+0x10]
            r14 = m.r64(rax.wrapping_add(0x10)) as u64;
            // ee4d cmp QWORD PTR [r14],rax
            fl = sub::<64>(m.r64(r14) as u64, rax).1;
            // ee50 mov rax,r14
            rax = r14;
            // ee53 jne 0xee49
            if crate::cov::brk(0xee53, fl.ne()) { pc = 0xee49; continue; }
            pc = 0xee55;
        }
        0xee55 => {
            // block ee55
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee55 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // ee58 jne 0xeded
            if crate::cov::brk(0xee58, fl.ne()) { pc = 0xeded; continue; }
            pc = 0xee5a;
        }
        0xee5a => {
            // block ee5a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee5a mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // ee5e mov rcx,QWORD PTR [rbp-0xe0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff20)) as u64;
            // ee65 cmp rax,QWORD PTR [rcx+0x10]
            fl = sub::<64>(rax, m.r64(rcx.wrapping_add(0x10)) as u64).1;
            // ee69 sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0xee6c;
        }
        0xee6c => {
            // block ee6c
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee6c lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // ee70 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ee75 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // ee79 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // ee7e lea rdi,[rbp-0xac]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // ee85 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xee8a;
        }
        0xee8a => {
            // block ee8a
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee8a mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // ee8c add rsp,0xb8
            { let (r, f) = add::<64>(rsp, 0xb8u64);
              rsp = r;
              fl = f; }
            // ee93 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ee94 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ee96 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ee98 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ee9a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ee9c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ee9d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xee9e => {
            // block ee9e
            #[cfg(feature = "trace")] eprintln!("f_0000ed4c pc ee9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ee9e mov rbx,rax
            rbx = rax;
            // eea1 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // eea5 call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eeaa lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // eeae call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eeb3 lea rdi,[rbp-0xac]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // eeba call 0x28f0e
            { let r = { let t = m.env.code_va(0x28f0e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eebf mov rdi,rbx
            rdi = rbx;
            // eec2 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2c238 size=382
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002c238<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2c238;
    loop {
        match pc {
        0x2c238 => {
            // block 2c238
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c238 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c238 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2c239 mov rbp,rsp
            rbp = rsp;
            // 2c23c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2c23e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2c23f sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2c243 mov rbx,rsi
            rbx = rsi;
            // 2c246 mov r14,rdi
            r14 = rdi;
            // 2c249 mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 2c24b lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c24f call 0x60623
            { let r = { let t = m.env.code_va(0x60623); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c254 lea rsi,[rip+0xfe1fb] # 0x12a456
            rsi = m.env.data_va(257059);
            // 2c25b mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c260 mov rdi,r14
            rdi = r14;
            // 2c263 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c268 movzx edx,BYTE PTR [rbp-0x28]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 2c26c test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 2c26f jne 0x2c27a
            if crate::cov::brk(0x2c26f, fl.ne()) { pc = 0x2c27a; continue; }
            pc = 0x2c271;
        }
        0x2c271 => {
            // block 2c271
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c271 lea rsi,[rbp-0x27]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd9);
            // 2c275 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 2c278 jmp 0x2c282
            pc = 0x2c282; continue;
        }
        0x2c27a => {
            // block 2c27a
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c27a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c27a mov rdx,QWORD PTR [rbp-0x20]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // 2c27e mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            pc = 0x2c282;
        }
        0x2c282 => {
            // block 2c282
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c282 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c282 mov rdi,rax
            rdi = rax;
            // 2c285 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c28a lea rsi,[rip+0xfe6f5] # 0x12a986
            rsi = m.env.data_va(258387);
            // 2c291 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c296 mov rdi,rax
            rdi = rax;
            // 2c299 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c29e mov esi,DWORD PTR [rbx]
            rsi = (m.r32(rbx) as u64) as u32 as u64;
            // 2c2a0 mov rdi,rax
            rdi = rax;
            // 2c2a3 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c2a8 lea rsi,[rip+0xfe6d9] # 0x12a988
            rsi = m.env.data_va(258389);
            // 2c2af mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 2c2b4 mov rdi,rax
            rdi = rax;
            // 2c2b7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c2bc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c2bf mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c2c3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c2c7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ca or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2cd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c2d1 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2c2d4 mov rdi,rax
            rdi = rax;
            // 2c2d7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c2dc mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c2df mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c2e3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c2e7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ea or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c2ed mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c2f1 lea rsi,[rip+0xfe699] # 0x12a991
            rsi = m.env.data_va(258398);
            // 2c2f8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c2fd mov rdi,rax
            rdi = rax;
            // 2c300 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c305 mov esi,DWORD PTR [rbx+0x4]
            rsi = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 2c308 mov rdi,rax
            rdi = rax;
            // 2c30b call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c310 lea rsi,[rip+0xfe67c] # 0x12a993
            rsi = m.env.data_va(258400);
            // 2c317 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // 2c31c mov rdi,rax
            rdi = rax;
            // 2c31f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c324 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c327 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c32b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c32f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c332 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c335 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c339 mov esi,DWORD PTR [rbx+0x8]
            rsi = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c33c mov rdi,rax
            rdi = rax;
            // 2c33f call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c344 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 2c347 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 2c34b mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 2c34f and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c352 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2c355 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2c359 lea rsi,[rip+0xfe63c] # 0x12a99c
            rsi = m.env.data_va(258409);
            // 2c360 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // 2c365 mov rdi,rax
            rdi = rax;
            // 2c368 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c36d mov esi,DWORD PTR [rbx+0xc]
            rsi = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 2c370 mov rdi,rax
            rdi = rax;
            // 2c373 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c378 lea rsi,[rip+0xfe0d5] # 0x12a454
            rsi = m.env.data_va(257057);
            // 2c37f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c384 mov rdi,rax
            rdi = rax;
            // 2c387 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c38c lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c390 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c395 mov rax,r14
            rax = r14;
            // 2c398 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // 2c39c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c39d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c39f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c3a0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c3a1 => {
            // block 2c3a1
            #[cfg(feature = "trace")] eprintln!("f_0002c238 pc 2c3a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c3a1 mov rbx,rax
            rbx = rax;
            // 2c3a4 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2c3a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2c3ad mov rdi,rbx
            rdi = rbx;
            // 2c3b0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5f52 size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5f52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5f52;
    loop {
        match pc {
        0xb5f52 => {
            // block b5f52
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5f53 mov rbp,rsp
            rbp = rsp;
            // b5f56 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5f58 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5f5a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b5f5c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5f5e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5f5f sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b5f63 mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5f67 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5f6a je 0xb60a6
            if crate::cov::brk(0xb5f6a, fl.e()) { pc = 0xb60a6; continue; }
            pc = 0xb5f70;
        }
        0xb5f70 => {
            // block b5f70
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f70 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // b5f74 movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // b5f77 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b5f7a and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5f7e shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b5f81 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b5f85 mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // b5f89 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // b5f8d mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // b5f91 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // b5f95 lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // b5f99 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // b5f9d jmp 0xb5fa2
            pc = 0xb5fa2; continue;
        }
        0xb5f9f => {
            // block b5f9f
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5f9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f9f mov rbx,rax
            rbx = rax;
            pc = 0xb5fa2;
        }
        0xb5fa2 => {
            // block b5fa2
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fa2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fa2 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5fa5 mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b5fa9 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // b5fae movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b5fb2 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b5fb5 and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b5fb9 jne 0xb5fc3
            if crate::cov::brk(0xb5fb9, fl.ne()) { pc = 0xb5fc3; continue; }
            pc = 0xb5fbb;
        }
        0xb5fbb => {
            // block b5fbb
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fbb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fbb mov r14,rax
            r14 = rax;
            // b5fbe shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // b5fc1 jmp 0xb5fc7
            pc = 0xb5fc7; continue;
        }
        0xb5fc3 => {
            // block b5fc3
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fc3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fc3 mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0xb5fc7;
        }
        0xb5fc7 => {
            // block b5fc7
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fc7 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // b5fcb test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b5fce mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b5fd2 cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // b5fd7 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b5fda jne 0xb5fe6
            if crate::cov::brk(0xb5fda, fl.ne()) { pc = 0xb5fe6; continue; }
            pc = 0xb5fdc;
        }
        0xb5fdc => {
            // block b5fdc
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fdc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fdc lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // b5fe0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // b5fe4 jmp 0xb5fee
            pc = 0xb5fee; continue;
        }
        0xb5fe6 => {
            // block b5fe6
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fe6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fe6 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // b5fea mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0xb5fee;
        }
        0xb5fee => {
            // block b5fee
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5fee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5fee cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // b5ff1 mov rdx,r13
            rdx = r13;
            // b5ff4 cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // b5ff8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b5ffb je 0xb6012
            if crate::cov::brk(0xb5ffb, fl.e()) { pc = 0xb6012; continue; }
            pc = 0xb5ffd;
        }
        0xb5ffd => {
            // block b5ffd
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b5ffd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ffd mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // b6001 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b6006 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b6008 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // b600c je 0xb6012
            if crate::cov::brk(0xb600c, fl.e()) { pc = 0xb6012; continue; }
            pc = 0xb600e;
        }
        0xb600e => {
            // block b600e
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b600e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b600e jns 0xb6024
            if crate::cov::brk(0xb600e, fl.ns()) { pc = 0xb6024; continue; }
            pc = 0xb6010;
        }
        0xb6010 => {
            // block b6010
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6010 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6010 jmp 0xb6017
            pc = 0xb6017; continue;
        }
        0xb6012 => {
            // block b6012
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6012 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // b6015 jae 0xb6024
            if crate::cov::brk(0xb6015, fl.ae()) { pc = 0xb6024; continue; }
            pc = 0xb6017;
        }
        0xb6017 => {
            // block b6017
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6017 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6017 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b601a test rax,rax
            fl = logic::<64>(rax & rax);
            // b601d jne 0xb5f9f
            if crate::cov::brk(0xb601d, fl.ne()) { pc = 0xb5f9f; continue; }
            pc = 0xb601f;
        }
        0xb601f => {
            // block b601f
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b601f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b601f jmp 0xb60bb
            pc = 0xb60bb; continue;
        }
        0xb6024 => {
            // block b6024
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6024 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6024 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b6027 jne 0xb6031
            if crate::cov::brk(0xb6027, fl.ne()) { pc = 0xb6031; continue; }
            pc = 0xb6029;
        }
        0xb6029 => {
            // block b6029
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6029 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6029 mov r14,rbx
            r14 = rbx;
            // b602c shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b602f jmp 0xb6038
            pc = 0xb6038; continue;
        }
        0xb6031 => {
            // block b6031
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6031 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6031 mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // b6035 mov r14,rbx
            r14 = rbx;
            pc = 0xb6038;
        }
        0xb6038 => {
            // block b6038
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6038 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6038 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b603c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b603f mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b6043 cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // b6048 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // b604b jne 0xb6052
            if crate::cov::brk(0xb604b, fl.ne()) { pc = 0xb6052; continue; }
            pc = 0xb604d;
        }
        0xb604d => {
            // block b604d
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b604d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b604d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // b6050 jmp 0xb6056
            pc = 0xb6056; continue;
        }
        0xb6052 => {
            // block b6052
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6052 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6052 mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0xb6056;
        }
        0xb6056 => {
            // block b6056
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6056 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6056 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // b6059 mov rdx,rax
            rdx = rax;
            // b605c cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // b6060 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b6063 je 0xb6084
            if crate::cov::brk(0xb6063, fl.e()) { pc = 0xb6084; continue; }
            pc = 0xb6065;
        }
        0xb6065 => {
            // block b6065
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6065 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6065 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // b6068 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b606c cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // b6071 mov r12,rax
            r12 = rax;
            // b6074 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b6079 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b607b mov rax,r12
            rax = r12;
            // b607e je 0xb6084
            if crate::cov::brk(0xb607e, fl.e()) { pc = 0xb6084; continue; }
            pc = 0xb6080;
        }
        0xb6080 => {
            // block b6080
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6080 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6080 js 0xb6089
            if crate::cov::brk(0xb6080, fl.s()) { pc = 0xb6089; continue; }
            pc = 0xb6082;
        }
        0xb6082 => {
            // block b6082
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6082 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6082 jmp 0xb60b2
            pc = 0xb60b2; continue;
        }
        0xb6084 => {
            // block b6084
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6084 cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // b6087 jae 0xb60b2
            if crate::cov::brk(0xb6087, fl.ae()) { pc = 0xb60b2; continue; }
            pc = 0xb6089;
        }
        0xb6089 => {
            // block b6089
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6089 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6089 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // b608d test rax,rax
            fl = logic::<64>(rax & rax);
            // b6090 jne 0xb5f9f
            if crate::cov::brk(0xb6090, fl.ne()) { pc = 0xb5f9f; continue; }
            pc = 0xb6096;
        }
        0xb6096 => {
            // block b6096
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b6096 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6096 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b609a mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // b609d add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // b60a1 mov rax,r14
            rax = r14;
            // b60a4 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60a6 => {
            // block b60a6
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60a6 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // b60aa mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // b60ad mov rax,rdi
            rax = rdi;
            // b60b0 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60b2 => {
            // block b60b2
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60b2 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b60b6 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // b60b9 jmp 0xb60c5
            pc = 0xb60c5; continue;
        }
        0xb60bb => {
            // block b60bb
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60bb mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b60bf mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // b60c2 mov rax,rbx
            rax = rbx;
            pc = 0xb60c5;
        }
        0xb60c5 => {
            // block b60c5
            #[cfg(feature = "trace")] eprintln!("f_000b5f52 pc b60c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b60c5 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b60c9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b60ca pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b60cc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b60ce pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b60d0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b60d2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b60d3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6537c size=394
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006537c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6537c;
    loop {
        match pc {
        0x6537c => {
            // block 6537c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6537c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6537c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6537d mov rbp,rsp
            rbp = rsp;
            // 65380 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 65382 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 65384 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 65386 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 65388 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 65389 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 6538d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 65391 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 65394 je 0x654d0
            if crate::cov::brk(0x65394, fl.e()) { pc = 0x654d0; continue; }
            pc = 0x6539a;
        }
        0x6539a => {
            // block 6539a
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6539a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6539a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 6539e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 653a1 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 653a4 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 653a8 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 653ab mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 653af mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 653b3 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 653b7 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 653bb mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 653bf lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 653c3 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 653c7 jmp 0x653cc
            pc = 0x653cc; continue;
        }
        0x653c9 => {
            // block 653c9
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653c9 mov rbx,rax
            rbx = rax;
            pc = 0x653cc;
        }
        0x653cc => {
            // block 653cc
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653cc test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 653cf mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 653d3 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 653d8 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 653dc mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 653df and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 653e3 jne 0x653ed
            if crate::cov::brk(0x653e3, fl.ne()) { pc = 0x653ed; continue; }
            pc = 0x653e5;
        }
        0x653e5 => {
            // block 653e5
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653e5 mov r14,rax
            r14 = rax;
            // 653e8 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 653eb jmp 0x653f1
            pc = 0x653f1; continue;
        }
        0x653ed => {
            // block 653ed
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653ed mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x653f1;
        }
        0x653f1 => {
            // block 653f1
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 653f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 653f1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 653f5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 653f8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 653fc cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 65401 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65404 jne 0x65410
            if crate::cov::brk(0x65404, fl.ne()) { pc = 0x65410; continue; }
            pc = 0x65406;
        }
        0x65406 => {
            // block 65406
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65406 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65406 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 6540a mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 6540e jmp 0x65418
            pc = 0x65418; continue;
        }
        0x65410 => {
            // block 65410
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65410 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 65414 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x65418;
        }
        0x65418 => {
            // block 65418
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65418 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 6541b mov rdx,r13
            rdx = r13;
            // 6541e cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 65422 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 65425 je 0x6543c
            if crate::cov::brk(0x65425, fl.e()) { pc = 0x6543c; continue; }
            pc = 0x65427;
        }
        0x65427 => {
            // block 65427
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65427 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65427 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 6542b call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 65430 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 65432 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 65436 je 0x6543c
            if crate::cov::brk(0x65436, fl.e()) { pc = 0x6543c; continue; }
            pc = 0x65438;
        }
        0x65438 => {
            // block 65438
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65438 jns 0x6544e
            if crate::cov::brk(0x65438, fl.ns()) { pc = 0x6544e; continue; }
            pc = 0x6543a;
        }
        0x6543a => {
            // block 6543a
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6543a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6543a jmp 0x65441
            pc = 0x65441; continue;
        }
        0x6543c => {
            // block 6543c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6543c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6543c cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 6543f jae 0x6544e
            if crate::cov::brk(0x6543f, fl.ae()) { pc = 0x6544e; continue; }
            pc = 0x65441;
        }
        0x65441 => {
            // block 65441
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65441 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65441 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 65444 test rax,rax
            fl = logic::<64>(rax & rax);
            // 65447 jne 0x653c9
            if crate::cov::brk(0x65447, fl.ne()) { pc = 0x653c9; continue; }
            pc = 0x65449;
        }
        0x65449 => {
            // block 65449
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65449 jmp 0x654e5
            pc = 0x654e5; continue;
        }
        0x6544e => {
            // block 6544e
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6544e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6544e test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65451 jne 0x6545b
            if crate::cov::brk(0x65451, fl.ne()) { pc = 0x6545b; continue; }
            pc = 0x65453;
        }
        0x65453 => {
            // block 65453
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65453 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65453 mov r14,rbx
            r14 = rbx;
            // 65456 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 65459 jmp 0x65462
            pc = 0x65462; continue;
        }
        0x6545b => {
            // block 6545b
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6545b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6545b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 6545f mov r14,rbx
            r14 = rbx;
            pc = 0x65462;
        }
        0x65462 => {
            // block 65462
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65462 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65462 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 65466 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 65469 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 6546d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 65472 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 65475 jne 0x6547c
            if crate::cov::brk(0x65475, fl.ne()) { pc = 0x6547c; continue; }
            pc = 0x65477;
        }
        0x65477 => {
            // block 65477
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65477 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65477 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 6547a jmp 0x65480
            pc = 0x65480; continue;
        }
        0x6547c => {
            // block 6547c
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6547c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6547c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x65480;
        }
        0x65480 => {
            // block 65480
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 65480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 65480 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 65483 mov rdx,rax
            rdx = rax;
            // 65486 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 6548a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 6548d je 0x654ae
            if crate::cov::brk(0x6548d, fl.e()) { pc = 0x654ae; continue; }
            pc = 0x6548f;
        }
        0x6548f => {
            // block 6548f
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 6548f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6548f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 65492 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 65496 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 6549b mov r12,rax
            r12 = rax;
            // 6549e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 654a3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 654a5 mov rax,r12
            rax = r12;
            // 654a8 je 0x654ae
            if crate::cov::brk(0x654a8, fl.e()) { pc = 0x654ae; continue; }
            pc = 0x654aa;
        }
        0x654aa => {
            // block 654aa
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654aa js 0x654b3
            if crate::cov::brk(0x654aa, fl.s()) { pc = 0x654b3; continue; }
            pc = 0x654ac;
        }
        0x654ac => {
            // block 654ac
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ac jmp 0x654dc
            pc = 0x654dc; continue;
        }
        0x654ae => {
            // block 654ae
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ae cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 654b1 jae 0x654dc
            if crate::cov::brk(0x654b1, fl.ae()) { pc = 0x654dc; continue; }
            pc = 0x654b3;
        }
        0x654b3 => {
            // block 654b3
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654b3 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 654b7 test rax,rax
            fl = logic::<64>(rax & rax);
            // 654ba jne 0x653c9
            if crate::cov::brk(0x654ba, fl.ne()) { pc = 0x653c9; continue; }
            pc = 0x654c0;
        }
        0x654c0 => {
            // block 654c0
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654c0 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654c4 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 654c7 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 654cb mov rax,r14
            rax = r14;
            // 654ce jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654d0 => {
            // block 654d0
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654d0 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 654d4 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 654d7 mov rax,rdi
            rax = rdi;
            // 654da jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654dc => {
            // block 654dc
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654dc mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654e0 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 654e3 jmp 0x654ef
            pc = 0x654ef; continue;
        }
        0x654e5 => {
            // block 654e5
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654e5 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 654e9 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 654ec mov rax,rbx
            rax = rbx;
            pc = 0x654ef;
        }
        0x654ef => {
            // block 654ef
            #[cfg(feature = "trace")] eprintln!("f_0006537c pc 654ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 654ef add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 654f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 654f4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 654f6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 654f8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 654fa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 654fc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 654fd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbe30 size=400
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbe30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbe30;
    loop {
        match pc {
        0xbbe30 => {
            // block bbe30
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbe31 mov rbp,rsp
            rbp = rsp;
            // bbe34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bbe36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bbe38 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bbe3a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bbe3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbe3d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bbe41 mov r15,rsi
            r15 = rsi;
            // bbe44 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // bbe48 movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // bbe4c test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // bbe4f jne 0xbbe5a
            if crate::cov::brk(0xbbe4f, fl.ne()) { pc = 0xbbe5a; continue; }
            pc = 0xbbe51;
        }
        0xbbe51 => {
            // block bbe51
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe51 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe51 lea r14,[r15+0x1]
            r14 = r15.wrapping_add(0x1);
            // bbe55 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // bbe58 jmp 0xbbe62
            pc = 0xbbe62; continue;
        }
        0xbbe5a => {
            // block bbe5a
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe5a mov rcx,QWORD PTR [r15+0x8]
            rcx = m.r64(r15.wrapping_add(0x8)) as u64;
            // bbe5e mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xbbe62;
        }
        0xbbe62 => {
            // block bbe62
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe62 movzx r13d,BYTE PTR [rip+0x10c726] # 0x1c8590
            r13 = (m.r8(m.env.data_va(904541)) as u64) as u32 as u64;
            // bbe6a test r13b,0x1
            fl = logic::<8>((r13 as u8 as u64) & 0x1u64);
            // bbe6e jne 0xbbe7c
            if crate::cov::brk(0xbbe6e, fl.ne()) { pc = 0xbbe7c; continue; }
            pc = 0xbbe70;
        }
        0xbbe70 => {
            // block bbe70
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe70 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // bbe73 lea r12,[rip+0x10c717] # 0x1c8591
            r12 = m.env.data_va(904542);
            // bbe7a jmp 0xbbe8a
            pc = 0xbbe8a; continue;
        }
        0xbbe7c => {
            // block bbe7c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe7c mov r12,QWORD PTR [rip+0x10c71d] # 0x1c85a0
            r12 = m.r64(m.env.data_va(904557)) as u64;
            // bbe83 mov r13,QWORD PTR [rip+0x10c70e] # 0x1c8598
            r13 = m.r64(m.env.data_va(904549)) as u64;
            pc = 0xbbe8a;
        }
        0xbbe8a => {
            // block bbe8a
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe8a mov rbx,rcx
            rbx = rcx;
            // bbe8d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // bbe90 je 0xbbed5
            if crate::cov::brk(0xbbe90, fl.e()) { pc = 0xbbed5; continue; }
            pc = 0xbbe92;
        }
        0xbbe92 => {
            // block bbe92
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe92 test r13,r13
            fl = logic::<64>(r13 & r13);
            // bbe95 je 0xbbeb1
            if crate::cov::brk(0xbbe95, fl.e()) { pc = 0xbbeb1; continue; }
            pc = 0xbbe97;
        }
        0xbbe97 => {
            // block bbe97
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbe97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbe97 movzx esi,BYTE PTR [r14+rbx*1-0x1]
            rsi = (m.r8(r14.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_ffff)) as u64) as u32 as u64;
            // bbe9d mov rdi,r12
            rdi = r12;
            // bbea0 mov rdx,r13
            rdx = r13;
            // bbea3 call 0xec212
            { let r = { let t = m.env.stub_va(0x9df); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbea8 lea rcx,[rbx-0x1]
            rcx = rbx.wrapping_add(0xffff_ffff_ffff_ffff);
            // bbeac test rax,rax
            fl = logic::<64>(rax & rax);
            // bbeaf jne 0xbbe8a
            if crate::cov::brk(0xbbeaf, fl.ne()) { pc = 0xbbe8a; continue; }
            pc = 0xbbeb1;
        }
        0xbbeb1 => {
            // block bbeb1
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbeb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbeb1 lea rcx,[r14+rbx*1-0x1]
            rcx = r14.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_ffff);
            // bbeb6 sub rcx,r14
            { let (r, f) = sub::<64>(rcx, r14);
              rcx = r;
              fl = f; }
            // bbeb9 cmp rcx,0xffffffffffffffff
            fl = sub::<64>(rcx, 0xffff_ffff_ffff_ffffu64).1;
            // bbebd je 0xbbed5
            if crate::cov::brk(0xbbebd, fl.e()) { pc = 0xbbed5; continue; }
            pc = 0xbbebf;
        }
        0xbbebf => {
            // block bbebf
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbebf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbebf inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // bbec2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbec6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbec8 mov rsi,r15
            rsi = r15;
            // bbecb mov r8,r15
            r8 = r15;
            // bbece call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbed3 jmp 0xbbee7
            pc = 0xbbee7; continue;
        }
        0xbbed5 => {
            // block bbed5
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbed5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbed5 lea rsi,[rip+0x7cdec] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bbedc lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbee0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbee2 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbbee7;
        }
        0xbbee7 => {
            // block bbee7
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbee7 movzx r13d,BYTE PTR [rbp-0x40]
            r13 = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // bbeec test r13b,0x1
            fl = logic::<8>((r13 as u8 as u64) & 0x1u64);
            // bbef0 jne 0xbbefb
            if crate::cov::brk(0xbbef0, fl.ne()) { pc = 0xbbefb; continue; }
            pc = 0xbbef2;
        }
        0xbbef2 => {
            // block bbef2
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbef2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbef2 lea r14,[rbp-0x3f]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // bbef6 shr r13,1
            { let (r, f) = shr::<64>(r13, 0x1u64, fl); r13 = r; fl = f; }
            // bbef9 jmp 0xbbf03
            pc = 0xbbf03; continue;
        }
        0xbbefb => {
            // block bbefb
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbefb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbefb mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // bbeff mov r14,QWORD PTR [rbp-0x30]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xbbf03;
        }
        0xbbf03 => {
            // block bbf03
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf03 movzx r12d,BYTE PTR [rip+0x10c685] # 0x1c8590
            r12 = (m.r8(m.env.data_va(904541)) as u64) as u32 as u64;
            // bbf0b test r12b,0x1
            fl = logic::<8>((r12 as u8 as u64) & 0x1u64);
            // bbf0f jne 0xbbf1d
            if crate::cov::brk(0xbbf0f, fl.ne()) { pc = 0xbbf1d; continue; }
            pc = 0xbbf11;
        }
        0xbbf11 => {
            // block bbf11
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf11 shr r12,1
            { let (r, f) = shr::<64>(r12, 0x1u64, fl); r12 = r; fl = f; }
            // bbf14 lea r15,[rip+0x10c676] # 0x1c8591
            r15 = m.env.data_va(904542);
            // bbf1b jmp 0xbbf2b
            pc = 0xbbf2b; continue;
        }
        0xbbf1d => {
            // block bbf1d
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf1d mov r15,QWORD PTR [rip+0x10c67c] # 0x1c85a0
            r15 = m.r64(m.env.data_va(904557)) as u64;
            // bbf24 mov r12,QWORD PTR [rip+0x10c66d] # 0x1c8598
            r12 = m.r64(m.env.data_va(904549)) as u64;
            pc = 0xbbf2b;
        }
        0xbbf2b => {
            // block bbf2b
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf2b test r13,r13
            fl = logic::<64>(r13 & r13);
            // bbf2e je 0xbbf7b
            if crate::cov::brk(0xbbf2e, fl.e()) { pc = 0xbbf7b; continue; }
            pc = 0xbbf30;
        }
        0xbbf30 => {
            // block bbf30
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf30 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xbbf32;
        }
        0xbbf32 => {
            // block bbf32
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf32 test r12,r12
            fl = logic::<64>(r12 & r12);
            // bbf35 je 0xbbf56
            if crate::cov::brk(0xbbf35, fl.e()) { pc = 0xbbf56; continue; }
            pc = 0xbbf37;
        }
        0xbbf37 => {
            // block bbf37
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf37 movzx esi,BYTE PTR [r14+rbx*1]
            rsi = (m.r8(r14.wrapping_add(rbx)) as u64) as u32 as u64;
            // bbf3c mov rdi,r15
            rdi = r15;
            // bbf3f mov rdx,r12
            rdx = r12;
            // bbf42 call 0xec212
            { let r = { let t = m.env.stub_va(0x9df); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbf47 test rax,rax
            fl = logic::<64>(rax & rax);
            // bbf4a je 0xbbf56
            if crate::cov::brk(0xbbf4a, fl.e()) { pc = 0xbbf56; continue; }
            pc = 0xbbf4c;
        }
        0xbbf4c => {
            // block bbf4c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf4c inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // bbf4f cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // bbf52 jne 0xbbf32
            if crate::cov::brk(0xbbf52, fl.ne()) { pc = 0xbbf32; continue; }
            pc = 0xbbf54;
        }
        0xbbf54 => {
            // block bbf54
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf54 jmp 0xbbf7b
            pc = 0xbbf7b; continue;
        }
        0xbbf56 => {
            // block bbf56
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf56 cmp rbx,0xffffffffffffffff
            fl = sub::<64>(rbx, 0xffff_ffff_ffff_ffffu64).1;
            // bbf5a je 0xbbf7b
            if crate::cov::brk(0xbbf5a, fl.e()) { pc = 0xbbf7b; continue; }
            pc = 0xbbf5c;
        }
        0xbbf5c => {
            // block bbf5c
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf5c lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbf60 mov rcx,0xffffffffffffffff
            rcx = 0xffff_ffff_ffff_ffffu64;
            // bbf67 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // bbf6b mov rdi,r14
            rdi = r14;
            // bbf6e mov rdx,rbx
            rdx = rbx;
            // bbf71 mov r8,rsi
            r8 = rsi;
            // bbf74 call 0xec02c
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbf79 jmp 0xbbf90
            pc = 0xbbf90; continue;
        }
        0xbbf7b => {
            // block bbf7b
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf7b lea rsi,[rip+0x7cd46] # 0x138cc8
            rsi = m.env.data_va(316565);
            // bbf82 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bbf84 mov r14,QWORD PTR [rbp-0x48]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // bbf88 mov rdi,r14
            rdi = r14;
            // bbf8b call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbbf90;
        }
        0xbbf90 => {
            // block bbf90
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbf90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbf90 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbf94 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbf99 mov rax,r14
            rax = r14;
            // bbf9c add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bbfa0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbfa1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bbfa3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bbfa5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bbfa7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bbfa9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbfaa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbbfab => {
            // block bbfab
            #[cfg(feature = "trace")] eprintln!("f_000bbe30 pc bbfab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bbfab mov rbx,rax
            rbx = rax;
            // bbfae lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // bbfb2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bbfb7 mov rdi,rbx
            rdi = rbx;
            // bbfba call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x914a size=410
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000914a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x914a;
    loop {
        match pc {
        0x914a => {
            // block 914a
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 914a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 914a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 914b mov rbp,rsp
            rbp = rsp;
            // 914e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9150 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9152 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9154 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9156 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9157 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 915b mov r14,rsi
            r14 = rsi;
            // 915e mov rbx,rdi
            rbx = rdi;
            // 9161 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9164 mov rdi,r14
            rdi = r14;
            // 9167 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 916a lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 916e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 9176 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 917e mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 9182 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9185 mov rax,QWORD PTR [rax+0x198]
            rax = m.r64(rax.wrapping_add(0x198)) as u64;
            // 918c mov rdi,rbx
            rdi = rbx;
            // 918f call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9191 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 9195 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 9197 call 0xbc038
            { let r = { let t = m.env.code_va(0xbc038); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 919c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 919e je 0x9295
            if crate::cov::brk(0x919e, fl.e()) { pc = 0x9295; continue; }
            pc = 0x91a4;
        }
        0x91a4 => {
            // block 91a4
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91a4 mov r12,QWORD PTR [rbp-0x40]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 91a8 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 91ab lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 91af cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 91b2 je 0x9298
            if crate::cov::brk(0x91b2, fl.e()) { pc = 0x9298; continue; }
            pc = 0x91b8;
        }
        0x91b8 => {
            // block 91b8
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91b8 lea r15,[rbp-0x50]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 91bc lea r13,[rbp-0x58]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0x91c0;
        }
        0x91c0 => {
            // block 91c0
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91c0 mov edi,DWORD PTR [r12+0x1c]
            rdi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 91c5 mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            // 91cd mov QWORD PTR [r15],0x0
            m.w64(r15, (0x0u64) as u64);
            // 91d4 mov QWORD PTR [rbp-0x58],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (r15) as u64);
            // 91d8 mov rsi,r13
            rsi = r13;
            // 91db call 0xbc13c
            { let r = { let t = m.env.code_va(0xbc13c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 91e0 mov r13,QWORD PTR [rbp-0x58]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 91e4 jmp 0x9246
            pc = 0x9246; continue;
        }
        0x91e6 => {
            // block 91e6
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 91e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 91e6 mov DWORD PTR [rbp-0x5c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x0u64) as u32);
            // 91ed mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 91f0 mov rax,QWORD PTR [rax+0x10c8]
            rax = m.r64(rax.wrapping_add(0x10c8)) as u64;
            // 91f7 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 91fb mov rdi,rbx
            rdi = rbx;
            // 91fe lea rdx,[rbp-0x5c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffa4);
            // 9202 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 9204 mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 9207 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 9209 je 0x921f
            if crate::cov::brk(0x9209, fl.e()) { pc = 0x921f; continue; }
            pc = 0x920b;
        }
        0x920b => {
            // block 920b
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 920b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 920b xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 920d jne 0x921f
            if crate::cov::brk(0x920d, fl.ne()) { pc = 0x921f; continue; }
            pc = 0x920f;
        }
        0x920f => {
            // block 920f
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 920f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 920f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 9212 mov rax,QWORD PTR [rax+0x10]
            rax = m.r64(rax.wrapping_add(0x10)) as u64;
            // 9216 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 921a mov rdi,r14
            rdi = r14;
            // 921d call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x921f;
        }
        0x921f => {
            // block 921f
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 921f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 921f mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // 9223 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9226 je 0x9236
            if crate::cov::brk(0x9226, fl.e()) { pc = 0x9236; continue; }
            pc = 0x9228;
        }
        0x9228 => {
            // block 9228
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9228 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9228 mov r13,rax
            r13 = rax;
            // 922b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 922f test rax,rax
            fl = logic::<64>(rax & rax);
            // 9232 jne 0x9228
            if crate::cov::brk(0x9232, fl.ne()) { pc = 0x9228; continue; }
            pc = 0x9234;
        }
        0x9234 => {
            // block 9234
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9234 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9234 jmp 0x9246
            pc = 0x9246; continue;
        }
        0x9236 => {
            // block 9236
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9236 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9236 mov rax,r13
            rax = r13;
            pc = 0x9239;
        }
        0x9239 => {
            // block 9239
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9239 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9239 mov r13,QWORD PTR [rax+0x10]
            r13 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 923d cmp QWORD PTR [r13+0x0],rax
            fl = sub::<64>(m.r64(r13) as u64, rax).1;
            // 9241 mov rax,r13
            rax = r13;
            // 9244 jne 0x9239
            if crate::cov::brk(0x9244, fl.ne()) { pc = 0x9239; continue; }
            pc = 0x9246;
        }
        0x9246 => {
            // block 9246
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9246 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9246 cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 9249 jne 0x91e6
            if crate::cov::brk(0x9249, fl.ne()) { pc = 0x91e6; continue; }
            pc = 0x924b;
        }
        0x924b => {
            // block 924b
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 924b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 924b mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 924f lea r13,[rbp-0x58]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 9253 mov rdi,r13
            rdi = r13;
            // 9256 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 925b mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // 9260 test rax,rax
            fl = logic::<64>(rax & rax);
            // 9263 je 0x9273
            if crate::cov::brk(0x9263, fl.e()) { pc = 0x9273; continue; }
            pc = 0x9265;
        }
        0x9265 => {
            // block 9265
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9265 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9265 mov r12,rax
            r12 = rax;
            // 9268 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 926c test rax,rax
            fl = logic::<64>(rax & rax);
            // 926f jne 0x9265
            if crate::cov::brk(0x926f, fl.ne()) { pc = 0x9265; continue; }
            pc = 0x9271;
        }
        0x9271 => {
            // block 9271
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9271 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9271 jmp 0x9283
            pc = 0x9283; continue;
        }
        0x9273 => {
            // block 9273
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9273 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9273 mov rax,r12
            rax = r12;
            pc = 0x9276;
        }
        0x9276 => {
            // block 9276
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9276 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9276 mov r12,QWORD PTR [rax+0x10]
            r12 = m.r64(rax.wrapping_add(0x10)) as u64;
            // 927a cmp QWORD PTR [r12],rax
            fl = sub::<64>(m.r64(r12) as u64, rax).1;
            // 927e mov rax,r12
            rax = r12;
            // 9281 jne 0x9276
            if crate::cov::brk(0x9281, fl.ne()) { pc = 0x9276; continue; }
            pc = 0x9283;
        }
        0x9283 => {
            // block 9283
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9283 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9283 lea rax,[rbp-0x38]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 9287 cmp r12,rax
            fl = sub::<64>(r12, rax).1;
            // 928a jne 0x91c0
            if crate::cov::brk(0x928a, fl.ne()) { pc = 0x91c0; continue; }
            pc = 0x9290;
        }
        0x9290 => {
            // block 9290
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9290 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9290 mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // 9293 jmp 0x9298
            pc = 0x9298; continue;
        }
        0x9295 => {
            // block 9295
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9295 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9295 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x9298;
        }
        0x9298 => {
            // block 9298
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 9298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9298 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 929c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 92a0 call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 92a5 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 92a8 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 92ac pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 92ad pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 92af pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 92b1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 92b3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 92b5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 92b6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x92b7 => {
            // block 92b7
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92b7 mov rbx,rax
            rbx = rax;
            // 92ba mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 92be lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 92c2 call 0xeb888
            { let r = { let t = m.env.stub_va(0x55); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x92c7;
        }
        0x92c7 => {
            // block 92c7
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92c7 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 92cb lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 92cf call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 92d4 mov rdi,rbx
            rdi = rbx;
            // 92d7 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 92dc jmp 0x92b7
            pc = 0x92b7; continue;
        }
        0x92de => {
            // block 92de
            #[cfg(feature = "trace")] eprintln!("f_0000914a pc 92de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 92de mov rbx,rax
            rbx = rax;
            // 92e1 jmp 0x92c7
            pc = 0x92c7; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbaafa size=414
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000baafa<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbaafa;
    loop {
        match pc {
        0xbaafa => {
            // block baafa
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc baafa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baafa push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // baafb mov rbp,rsp
            rbp = rsp;
            // baafe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bab00 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bab02 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // bab04 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bab06 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bab07 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bab0b mov r13,rsi
            r13 = rsi;
            // bab0e mov r14,rdi
            r14 = rdi;
            // bab11 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bab15 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bab19 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bab1c je 0xbab33
            if crate::cov::brk(0xbab1c, fl.e()) { pc = 0xbab33; continue; }
            pc = 0xbab1e;
        }
        0xbab1e => {
            // block bab1e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab1e lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bab22 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bab25 not rdx
            { let v = !rdx; rdx = v; }
            // bab28 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bab2c add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bab2f mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbab33;
        }
        0xbab33 => {
            // block bab33
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab33 mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // bab37 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // bab3b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bab3d cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // bab40 je 0xbac80
            if crate::cov::brk(0xbab40, fl.e()) { pc = 0xbac80; continue; }
            pc = 0xbab46;
        }
        0xbab46 => {
            // block bab46
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab46 mov edi,DWORD PTR [r15+0x1c]
            rdi = (m.r32(r15.wrapping_add(0x1c)) as u64) as u32 as u64;
            // bab4a mov r12d,DWORD PTR [r15+0x20]
            r12 = (m.r32(r15.wrapping_add(0x20)) as u64) as u32 as u64;
            // bab4e mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // bab55 mov DWORD PTR [rbp-0x30],0x3e7
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x3e7u64) as u32);
            // bab5c lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // bab60 lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bab64 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bab69 mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // bab6c test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // bab6e sete bl
            rbx = (rbx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // bab71 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // bab74 cmp rdx,0x3
            fl = sub::<64>(rdx, 0x3u64).1;
            // bab78 ja 0xbac03
            if crate::cov::brk(0xbab78, fl.a()) { pc = 0xbac03; continue; }
            pc = 0xbab7e;
        }
        0xbab7e => {
            // block bab7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab7e xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab80 or al,bl
            { let r = (rax as u8 as u64) | (rbx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // bab82 jne 0xbac03
            if crate::cov::brk(0xbab82, fl.ne()) { pc = 0xbac03; continue; }
            pc = 0xbab84;
        }
        0xbab84 => {
            // block bab84
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bab84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bab84 lea rax,[rip+0x47e75] # 0x102a00
            rax = m.env.data_va(94669);
            // bab8b mov eax,DWORD PTR [rax+rdx*4]
            rax = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab8e lea rsi,[rip+0x47e7b] # 0x102a10
            rsi = m.env.data_va(94685);
            // bab95 mov edx,DWORD PTR [rsi+rdx*4]
            rdx = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // bab98 mov DWORD PTR [rbp-0x40],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rcx as u32 as u64)) as u32);
            // bab9b mov DWORD PTR [rbp-0x3c],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((r12 as u32 as u64)) as u32);
            // bab9f mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // baba2 mov DWORD PTR [rbp-0x34],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rdx as u32 as u64)) as u32);
            // baba5 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // baba9 cmp rax,QWORD PTR [r13+0x10]
            fl = sub::<64>(rax, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // babad je 0xbabc5
            if crate::cov::brk(0xbabad, fl.e()) { pc = 0xbabc5; continue; }
            pc = 0xbabaf;
        }
        0xbabaf => {
            // block babaf
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babaf mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // babb3 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // babb7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // babbb mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // babbe add QWORD PTR [r13+0x8],0x10
            { let (r, f) = add::<64>(m.r64(r13.wrapping_add(0x8)) as u64, 0x10u64);
              m.w64(r13.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // babc3 jmp 0xbabd1
            pc = 0xbabd1; continue;
        }
        0xbabc5 => {
            // block babc5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babc5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babc5 mov rdi,r13
            rdi = r13;
            // babc8 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // babcc call 0xebb2e
            { let r = { let t = m.env.stub_va(0x2fb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbabd1;
        }
        0xbabd1 => {
            // block babd1
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babd1 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // babd5 test rax,rax
            fl = logic::<64>(rax & rax);
            // babd8 je 0xbabe7
            if crate::cov::brk(0xbabd8, fl.e()) { pc = 0xbabe7; continue; }
            pc = 0xbabda;
        }
        0xbabda => {
            // block babda
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babda mov r15,rax
            r15 = rax;
            // babdd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // babe0 test rax,rax
            fl = logic::<64>(rax & rax);
            // babe3 jne 0xbabda
            if crate::cov::brk(0xbabe3, fl.ne()) { pc = 0xbabda; continue; }
            pc = 0xbabe5;
        }
        0xbabe5 => {
            // block babe5
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe5 jmp 0xbabf6
            pc = 0xbabf6; continue;
        }
        0xbabe7 => {
            // block babe7
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babe7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babe7 mov rax,r15
            rax = r15;
            pc = 0xbabea;
        }
        0xbabea => {
            // block babea
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babea mov r15,QWORD PTR [rax+0x10]
            r15 = m.r64(rax.wrapping_add(0x10)) as u64;
            // babee cmp QWORD PTR [r15],rax
            fl = sub::<64>(m.r64(r15) as u64, rax).1;
            // babf1 mov rax,r15
            rax = r15;
            // babf4 jne 0xbabea
            if crate::cov::brk(0xbabf4, fl.ne()) { pc = 0xbabea; continue; }
            pc = 0xbabf6;
        }
        0xbabf6 => {
            // block babf6
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babf6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babf6 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // babf9 jne 0xbab46
            if crate::cov::brk(0xbabf9, fl.ne()) { pc = 0xbab46; continue; }
            pc = 0xbabff;
        }
        0xbabff => {
            // block babff
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc babff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // babff mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // bac01 jmp 0xbac80
            pc = 0xbac80; continue;
        }
        0xbac03 => {
            // block bac03
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac03 mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac07 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac0b cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac0e je 0xbac7e
            if crate::cov::brk(0xbac0e, fl.e()) { pc = 0xbac7e; continue; }
            pc = 0xbac10;
        }
        0xbac10 => {
            // block bac10
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac10 lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac14 sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac17 not rdx
            { let v = !rdx; rdx = v; }
            // bac1a and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac1e add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac21 mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            // bac25 jmp 0xbac7e
            pc = 0xbac7e; continue;
        }
        0xbac27 => {
            // block bac27
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac27 mov r14,rax
            r14 = rax;
            // bac2a cmp edx,0x1
            fl = sub::<32>((rdx as u32 as u64), 0x1u64).1;
            // bac2d jne 0xbac8f
            if crate::cov::brk(0xbac2d, fl.ne()) { pc = 0xbac8f; continue; }
            pc = 0xbac2f;
        }
        0xbac2f => {
            // block bac2f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac2f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac2f mov rdi,r14
            rdi = r14;
            // bac32 call 0xec164
            { let r = { let t = m.env.stub_va(0x931); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bac37 mov rax,QWORD PTR [rip+0x8578a] # 0x1403c8
            rax = m.r64(m.env.data_va(347029)) as u64;
            // bac3e add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // bac42 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // bac46 mov rdi,r14
            rdi = r14;
            // bac49 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bac4e mov rcx,QWORD PTR [r13+0x0]
            rcx = m.r64(r13) as u64;
            // bac52 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // bac56 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // bac59 je 0xbac70
            if crate::cov::brk(0xbac59, fl.e()) { pc = 0xbac70; continue; }
            pc = 0xbac5b;
        }
        0xbac5b => {
            // block bac5b
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac5b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac5b lea rdx,[rax-0x10]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff0);
            // bac5f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // bac62 not rdx
            { let v = !rdx; rdx = v; }
            // bac65 and rdx,0xfffffffffffffff0
            { let r = rdx & 0xffff_ffff_ffff_fff0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // bac69 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // bac6c mov QWORD PTR [r13+0x8],rdx
            m.w64(r13.wrapping_add(0x8), (rdx) as u64);
            pc = 0xbac70;
        }
        0xbac70 => {
            // block bac70
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac70 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // bac74 call 0xec128
            { let r = { let t = m.env.stub_va(0x8f5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // bac79 call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbac7e;
        }
        0xbac7e => {
            // block bac7e
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac7e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xbac80;
        }
        0xbac80 => {
            // block bac80
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac80 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bac84 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bac85 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bac87 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // bac89 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bac8b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bac8d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bac8e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xbac8f => {
            // block bac8f
            #[cfg(feature = "trace")] eprintln!("f_000baafa pc bac8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // bac8f mov rdi,r14
            rdi = r14;
            // bac92 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x189c size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000189c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x189c;
    loop {
        match pc {
        0x189c => {
            // block 189c
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 189c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 189d mov rbp,rsp
            rbp = rsp;
            // 18a0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18a5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18a6 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 18a8 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 18ab mov r14,rdi
            r14 = rdi;
            // 18ae mov edi,DWORD PTR [r14+0x10]
            rdi = (m.r32(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // 18b2 call 0xcb6fa
            { let r = { let t = m.env.code_va(0xcb6fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 18b7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 18b9 je 0x192a
            if crate::cov::brk(0x18b9, fl.e()) { pc = 0x192a; continue; }
            pc = 0x18bb;
        }
        0x18bb => {
            // block 18bb
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 18bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18bb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 18be lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 18c2 mov rdi,r14
            rdi = r14;
            // 18c5 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 18c7 call QWORD PTR [rax+0x478]
            { let r = { let t = m.r64(rax.wrapping_add(0x478)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 18cd mov ecx,DWORD PTR [rbp-0x1c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // 18d0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 18d3 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 18d7 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 18d9 je 0x1931
            if crate::cov::brk(0x18d9, fl.e()) { pc = 0x1931; continue; }
            pc = 0x18db;
        }
        0x18db => {
            // block 18db
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 18db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18db test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 18de je 0x1931
            if crate::cov::brk(0x18de, fl.e()) { pc = 0x1931; continue; }
            pc = 0x18e0;
        }
        0x18e0 => {
            // block 18e0
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 18e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18e0 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 18e5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 18ea cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 18ed jg 0x1994
            if crate::cov::brk(0x18ed, fl.g()) { pc = 0x1994; continue; }
            pc = 0x18f3;
        }
        0x18f3 => {
            // block 18f3
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 18f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f3 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 18f8 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 18fe mov rdi,r14
            rdi = r14;
            // 1901 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1903 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1906 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 190b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 190d mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1912 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1918 mov rdi,r14
            rdi = r14;
            // 191b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 191e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1921 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1926 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1928 jmp 0x1984
            pc = 0x1984; continue;
        }
        0x192a => {
            // block 192a
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 192a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 192a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 192c jmp 0x1a3d
            pc = 0x1a3d; continue;
        }
        0x1931 => {
            // block 1931
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1931 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1931 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1936 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1938 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 193a je 0x19cb
            if crate::cov::brk(0x193a, fl.e()) { pc = 0x19cb; continue; }
            pc = 0x1940;
        }
        0x1940 => {
            // block 1940
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1940 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1940 cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 1943 jg 0x19dd
            if crate::cov::brk(0x1943, fl.g()) { pc = 0x19dd; continue; }
            pc = 0x1949;
        }
        0x1949 => {
            // block 1949
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1949 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1949 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 194e mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 1954 mov rdi,r14
            rdi = r14;
            // 1957 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1959 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 195c mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1961 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 1966 mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 196b mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1971 mov rdi,r14
            rdi = r14;
            // 1974 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1977 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 197a mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 197f mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x1984;
        }
        0x1984 => {
            // block 1984
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1984 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 1989 mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 198f jmp 0x1a23
            pc = 0x1a23; continue;
        }
        0x1994 => {
            // block 1994
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1994 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 1999 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 199f mov rdi,r14
            rdi = r14;
            // 19a2 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19a4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 19a7 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 19ac xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 19ae mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 19b3 mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 19b9 mov rdi,r14
            rdi = r14;
            // 19bc call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19bf mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 19c2 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 19c7 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 19c9 jmp 0x1a18
            pc = 0x1a18; continue;
        }
        0x19cb => {
            // block 19cb
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 19cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19cb cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 19ce jg 0x1a2b
            if crate::cov::brk(0x19ce, fl.g()) { pc = 0x1a2b; continue; }
            pc = 0x19d0;
        }
        0x19d0 => {
            // block 19d0
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 19d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19d0 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 19d5 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 19db jmp 0x1a36
            pc = 0x1a36; continue;
        }
        0x19dd => {
            // block 19dd
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 19dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19dd mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 19e2 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 19e8 mov rdi,r14
            rdi = r14;
            // 19eb call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19ed mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 19f0 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 19f5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 19fa mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 19ff mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 1a05 mov rdi,r14
            rdi = r14;
            // 1a08 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1a0b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1a0e mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1a13 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x1a18;
        }
        0x1a18 => {
            // block 1a18
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1a18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a18 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 1a1d mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x1a23;
        }
        0x1a23 => {
            // block 1a23
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1a23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a23 mov rdi,r14
            rdi = r14;
            // 1a26 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1a29 jmp 0x1a3d
            pc = 0x1a3d; continue;
        }
        0x1a2b => {
            // block 1a2b
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a2b mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 1a30 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            pc = 0x1a36;
        }
        0x1a36 => {
            // block 1a36
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a36 mov rdi,r14
            rdi = r14;
            // 1a39 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1a3b mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x1a3d;
        }
        0x1a3d => {
            // block 1a3d
            #[cfg(feature = "trace")] eprintln!("f_0000189c pc 1a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a3d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1a41 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a42 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a44 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a47 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x311bc size=428
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000311bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x311bc;
    loop {
        match pc {
        0x311bc => {
            // block 311bc
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 311bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 311bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 311bd mov rbp,rsp
            rbp = rsp;
            // 311c0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 311c2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 311c3 sub rsp,0x90
            { let (r, f) = sub::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 311ca mov r14,rsi
            r14 = rsi;
            // 311cd mov rbx,rdi
            rbx = rdi;
            // 311d0 xorps xmm0,xmm0
            x0 = 0;
            // 311d3 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // 311d7 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // 311df mov BYTE PTR [rbp-0x31],0x16
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x16u64) as u8);
            // 311e3 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 311e7 lea rsi,[rbp-0x31]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 311eb call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 311f0 xorps xmm0,xmm0
            x0 = 0;
            // 311f3 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // 311f7 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // 311ff movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // 31203 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // 3120b lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 31212 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 31216 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3121b lea rdi,[rbp-0xa0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 31222 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 31226 call 0xebb16
            { let r = { let t = m.env.stub_va(0x2e3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3122b lea rsi,[rbp-0x88]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // 31232 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // 31239 lea rcx,[rbp-0x70]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 3123d mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 31243 mov rdi,rbx
            rdi = rbx;
            // 31246 call 0x2fdd8
            { let r = { let t = m.env.code_va(0x2fdd8); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 3124b mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 3124d mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 31254 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31257 je 0x3126e
            if crate::cov::brk(0x31257, fl.e()) { pc = 0x3126e; continue; }
            pc = 0x31259;
        }
        0x31259 => {
            // block 31259
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31259 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31259 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 31260 je 0x31269
            if crate::cov::brk(0x31260, fl.e()) { pc = 0x31269; continue; }
            pc = 0x31262;
        }
        0x31262 => {
            // block 31262
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31262 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31262 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x31269;
        }
        0x31269 => {
            // block 31269
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31269 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31269 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x3126e;
        }
        0x3126e => {
            // block 3126e
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3126e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3126e mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 31275 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31278 je 0x31289
            if crate::cov::brk(0x31278, fl.e()) { pc = 0x31289; continue; }
            pc = 0x3127a;
        }
        0x3127a => {
            // block 3127a
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3127a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3127a cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 3127e je 0x31284
            if crate::cov::brk(0x3127e, fl.e()) { pc = 0x31284; continue; }
            pc = 0x31280;
        }
        0x31280 => {
            // block 31280
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31280 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x31284;
        }
        0x31284 => {
            // block 31284
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31284 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31284 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x31289;
        }
        0x31289 => {
            // block 31289
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31289 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31289 test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // 3128b je 0x312a1
            if crate::cov::brk(0x3128b, fl.e()) { pc = 0x312a1; continue; }
            pc = 0x3128d;
        }
        0x3128d => {
            // block 3128d
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3128d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3128d mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 31291 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 31295 cmp rax,rdi
            fl = sub::<64>(rax, rdi).1;
            // 31298 je 0x312a5
            if crate::cov::brk(0x31298, fl.e()) { pc = 0x312a5; continue; }
            pc = 0x3129a;
        }
        0x3129a => {
            // block 3129a
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3129a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3129a mov al,BYTE PTR [rdi]
            rax = (rax & !0xff) | ((m.r8(rdi) as u64) as u8 as u64);
            // 3129c mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // 3129f jmp 0x312ad
            pc = 0x312ad; continue;
        }
        0x312a1 => {
            // block 312a1
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312a1 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x312a5;
        }
        0x312a5 => {
            // block 312a5
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312a5 test rax,rax
            fl = logic::<64>(rax & rax);
            // 312a8 mov rdi,rax
            rdi = rax;
            // 312ab je 0x312bc
            if crate::cov::brk(0x312ab, fl.e()) { pc = 0x312bc; continue; }
            pc = 0x312ad;
        }
        0x312ad => {
            // block 312ad
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312ad cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 312b1 je 0x312b7
            if crate::cov::brk(0x312b1, fl.e()) { pc = 0x312b7; continue; }
            pc = 0x312b3;
        }
        0x312b3 => {
            // block 312b3
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312b3 mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x312b7;
        }
        0x312b7 => {
            // block 312b7
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312b7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x312bc;
        }
        0x312bc => {
            // block 312bc
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312bc mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 312c0 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 312c3 je 0x312d4
            if crate::cov::brk(0x312c3, fl.e()) { pc = 0x312d4; continue; }
            pc = 0x312c5;
        }
        0x312c5 => {
            // block 312c5
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312c5 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 312c9 je 0x312cf
            if crate::cov::brk(0x312c9, fl.e()) { pc = 0x312cf; continue; }
            pc = 0x312cb;
        }
        0x312cb => {
            // block 312cb
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312cb mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x312cf;
        }
        0x312cf => {
            // block 312cf
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312cf call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x312d4;
        }
        0x312d4 => {
            // block 312d4
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312d4 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 312d6 add rsp,0x90
            { let (r, f) = add::<64>(rsp, 0x90u64);
              rsp = r;
              fl = f; }
            // 312dd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 312de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 312e0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 312e1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x312e2 => {
            // block 312e2
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312e2 mov rbx,rax
            rbx = rax;
            // 312e5 jmp 0x31348
            pc = 0x31348; continue;
        }
        0x312e7 => {
            // block 312e7
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312e7 mov rbx,rax
            rbx = rax;
            // 312ea jmp 0x31330
            pc = 0x31330; continue;
        }
        0x312ec => {
            // block 312ec
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312ec mov rbx,rax
            rbx = rax;
            // 312ef jmp 0x31315
            pc = 0x31315; continue;
        }
        0x312f1 => {
            // block 312f1
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 312f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 312f1 mov rbx,rax
            rbx = rax;
            // 312f4 mov rdi,QWORD PTR [rbp-0xa0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // 312fb test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 312fe je 0x31315
            if crate::cov::brk(0x312fe, fl.e()) { pc = 0x31315; continue; }
            pc = 0x31300;
        }
        0x31300 => {
            // block 31300
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31300 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31300 cmp QWORD PTR [rbp-0x98],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64, rdi).1;
            // 31307 je 0x31310
            if crate::cov::brk(0x31307, fl.e()) { pc = 0x31310; continue; }
            pc = 0x31309;
        }
        0x31309 => {
            // block 31309
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31309 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31309 mov QWORD PTR [rbp-0x98],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rdi) as u64);
            pc = 0x31310;
        }
        0x31310 => {
            // block 31310
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31310 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31310 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x31315;
        }
        0x31315 => {
            // block 31315
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31315 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31315 mov rdi,QWORD PTR [rbp-0x88]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 3131c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3131f je 0x31330
            if crate::cov::brk(0x3131f, fl.e()) { pc = 0x31330; continue; }
            pc = 0x31321;
        }
        0x31321 => {
            // block 31321
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31321 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31321 cmp QWORD PTR [rbp-0x80],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64, rdi).1;
            // 31325 je 0x3132b
            if crate::cov::brk(0x31325, fl.e()) { pc = 0x3132b; continue; }
            pc = 0x31327;
        }
        0x31327 => {
            // block 31327
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31327 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31327 mov QWORD PTR [rbp-0x80],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rdi) as u64);
            pc = 0x3132b;
        }
        0x3132b => {
            // block 3132b
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3132b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3132b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x31330;
        }
        0x31330 => {
            // block 31330
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31330 mov rdi,QWORD PTR [rbp-0x70]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 31334 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31337 je 0x31348
            if crate::cov::brk(0x31337, fl.e()) { pc = 0x31348; continue; }
            pc = 0x31339;
        }
        0x31339 => {
            // block 31339
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31339 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31339 cmp QWORD PTR [rbp-0x68],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64, rdi).1;
            // 3133d je 0x31343
            if crate::cov::brk(0x3133d, fl.e()) { pc = 0x31343; continue; }
            pc = 0x3133f;
        }
        0x3133f => {
            // block 3133f
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3133f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3133f mov QWORD PTR [rbp-0x68],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rdi) as u64);
            pc = 0x31343;
        }
        0x31343 => {
            // block 31343
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31343 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31343 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x31348;
        }
        0x31348 => {
            // block 31348
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31348 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 3134c test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 3134f je 0x31360
            if crate::cov::brk(0x3134f, fl.e()) { pc = 0x31360; continue; }
            pc = 0x31351;
        }
        0x31351 => {
            // block 31351
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31351 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31351 cmp QWORD PTR [rbp-0x28],rdi
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64, rdi).1;
            // 31355 je 0x3135b
            if crate::cov::brk(0x31355, fl.e()) { pc = 0x3135b; continue; }
            pc = 0x31357;
        }
        0x31357 => {
            // block 31357
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31357 mov QWORD PTR [rbp-0x28],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (rdi) as u64);
            pc = 0x3135b;
        }
        0x3135b => {
            // block 3135b
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 3135b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3135b call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x31360;
        }
        0x31360 => {
            // block 31360
            #[cfg(feature = "trace")] eprintln!("f_000311bc pc 31360 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31360 mov rdi,rbx
            rdi = rbx;
            // 31363 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7a88 size=450
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7a88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7a88;
    loop {
        match pc {
        0xb7a88 => {
            // block b7a88
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7a88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7a89 mov rbp,rsp
            rbp = rsp;
            // b7a8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7a8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7a8f sub rsp,0x60
            { let (r, f) = sub::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7a93 mov rbx,rsi
            rbx = rsi;
            // b7a96 mov r14,rdi
            r14 = rdi;
            // b7a99 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7a9c call QWORD PTR [rax+0x1d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7aa2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7aa6 mov rsi,rax
            rsi = rax;
            // b7aa9 call 0xb799c
            { let r = { let t = m.env.code_va(0xb799c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7aae test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7ab1 jne 0xb7aba
            if crate::cov::brk(0xb7ab1, fl.ne()) { pc = 0xb7aba; continue; }
            pc = 0xb7ab3;
        }
        0xb7ab3 => {
            // block b7ab3
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ab3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ab3 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7ab8 jmp 0xb7ac9
            pc = 0xb7ac9; continue;
        }
        0xb7aba => {
            // block b7aba
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7aba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7aba mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7abe mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7ac1 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7ac9;
        }
        0xb7ac9 => {
            // block b7ac9
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7ac9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ac9 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7acb mov rdi,rbx
            rdi = rbx;
            // b7ace call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7ad3 mov rax,QWORD PTR [rbp-0x20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // b7ad7 mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7adb mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // b7adf mov rcx,QWORD PTR [rbp-0x28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b7ae3 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7ae7 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7aea xorps xmm0,xmm0
            x0 = 0;
            // b7aed movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // b7af1 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // b7af9 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b7afd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7b02 movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // b7b05 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // b7b07 jne 0xb7b0e
            if crate::cov::brk(0xb7b07, fl.ne()) { pc = 0xb7b0e; continue; }
            pc = 0xb7b09;
        }
        0xb7b09 => {
            // block b7b09
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b09 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // b7b0c jmp 0xb7b12
            pc = 0xb7b12; continue;
        }
        0xb7b0e => {
            // block b7b0e
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b0e mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0xb7b12;
        }
        0xb7b12 => {
            // block b7b12
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b12 test rax,rax
            fl = logic::<64>(rax & rax);
            // b7b15 je 0xb7b49
            if crate::cov::brk(0xb7b15, fl.e()) { pc = 0xb7b49; continue; }
            pc = 0xb7b17;
        }
        0xb7b17 => {
            // block b7b17
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b17 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b17 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b7b1a mov rdi,r14
            rdi = r14;
            // b7b1d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7b23 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // b7b28 jne 0xb7b60
            if crate::cov::brk(0xb7b28, fl.ne()) { pc = 0xb7b60; continue; }
            pc = 0xb7b2a;
        }
        0xb7b2a => {
            // block b7b2a
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b2a lea rsi,[rip+0x802ca] # 0x137dfb
            rsi = m.env.data_va(312776);
            // b7b31 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7b35 mov rdx,rbx
            rdx = rbx;
            // b7b38 call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7b3d test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b40 jne 0xb7b90
            if crate::cov::brk(0xb7b40, fl.ne()) { pc = 0xb7b90; continue; }
            pc = 0xb7b42;
        }
        0xb7b42 => {
            // block b7b42
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b42 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b47 jmp 0xb7b9f
            pc = 0xb7b9f; continue;
        }
        0xb7b49 => {
            // block b7b49
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b49 lea rsi,[rip+0x802a2] # 0x137df2
            rsi = m.env.data_va(312767);
            // b7b50 mov rdi,rbx
            rdi = rbx;
            // b7b53 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7b58 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7b5b jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7b60 => {
            // block b7b60
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b60 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b7b63 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7b66 cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // b7b6b jne 0xb7c25
            if crate::cov::brk(0xb7b6b, fl.ne()) { pc = 0xb7c25; continue; }
            pc = 0xb7b71;
        }
        0xb7b71 => {
            // block b7b71
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b71 lea rsi,[rip+0x7d2e8] # 0x134e60
            rsi = m.env.data_va(300589);
            // b7b78 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7b7c mov rdx,rbx
            rdx = rbx;
            // b7b7f call 0xebcf6
            { let r = { let t = m.env.stub_va(0x4c3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7b84 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // b7b87 jne 0xb7bdd
            if crate::cov::brk(0xb7b87, fl.ne()) { pc = 0xb7bdd; continue; }
            pc = 0xb7b89;
        }
        0xb7b89 => {
            // block b7b89
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b89 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // b7b8e jmp 0xb7bec
            pc = 0xb7bec; continue;
        }
        0xb7b90 => {
            // block b7b90
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b90 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7b94 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7b97 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7b9f;
        }
        0xb7b9f => {
            // block b7b9f
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7b9f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7b9f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7ba1 mov rdi,rbx
            rdi = rbx;
            // b7ba4 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7ba9 mov rax,QWORD PTR [rbp-0x40]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b7bad mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bb1 mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // b7bb5 mov rcx,QWORD PTR [rbp-0x48]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b7bb9 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7bbd mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7bc0 xorps xmm0,xmm0
            x0 = 0;
            // b7bc3 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // b7bc7 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b7bcf lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // b7bd3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7bd8 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // b7bdb jmp 0xb7c25
            pc = 0xb7c25; continue;
        }
        0xb7bdd => {
            // block b7bdd
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bdd mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b7be1 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7be4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7bec;
        }
        0xb7bec => {
            // block b7bec
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7bec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7bec xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b7bee mov rdi,rbx
            rdi = rbx;
            // b7bf1 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7bf6 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // b7bfa mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // b7bfe mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // b7c02 mov rcx,QWORD PTR [rbp-0x68]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // b7c06 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // b7c0a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b7c0d xorps xmm0,xmm0
            x0 = 0;
            // b7c10 movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // b7c14 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // b7c1c lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // b7c20 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xb7c25;
        }
        0xb7c25 => {
            // block b7c25
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c25 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b7c28 add rsp,0x60
            { let (r, f) = add::<64>(rsp, 0x60u64);
              rsp = r;
              fl = f; }
            // b7c2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7c2d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7c2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7c30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7c31 => {
            // block b7c31
            #[cfg(feature = "trace")] eprintln!("f_000b7a88 pc b7c31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7c31 mov rdi,rax
            rdi = rax;
            // b7c34 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7c39 mov rdi,rax
            rdi = rax;
            // b7c3c call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7c41 mov rdi,rax
            rdi = rax;
            // b7c44 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
