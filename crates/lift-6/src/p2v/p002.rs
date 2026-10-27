//! Mechanically lifted functions (scripts/lift_run.py), module p002. Each one is checked against the original
//! in the world harness (tests/world_p002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e08f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            if crate::cov::brk(0xe090e, fl.e()) { pc = 0xe0997; continue; }
            pc = 0xe0914;
        }
        0xe0914 => {
            // block e0914
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0914 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0917 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0xe0929, fl.e()) { pc = 0xe0997; continue; }
            pc = 0xe092b;
        }
        0xe092b => {
            // block e092b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e092b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0947 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e094b mov rsi,r14
            rsi = r14;
            // e094e call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e0953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e0955 je 0xe096d
            if crate::cov::brk(0xe0955, fl.e()) { pc = 0xe096d; continue; }
            pc = 0xe0957;
        }
        0xe0957 => {
            // block e0957
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0957 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0957 inc r12
            { let (r, f) = inc::<64>(r12, fl); r12 = r; fl = f; }
            // e095a add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e095e cmp r12,r13
            fl = sub::<64>(r12, r13).1;
            // e0961 jb 0xe0947
            if crate::cov::brk(0xe0961, fl.b()) { pc = 0xe0947; continue; }
            pc = 0xe0963;
        }
        0xe0963 => {
            // block e0963
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0963 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0965 cmp QWORD PTR [rbp-0x30],r15
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, r15).1;
            // e0969 jne 0xe097b
            if crate::cov::brk(0xe0969, fl.ne()) { pc = 0xe097b; continue; }
            pc = 0xe096b;
        }
        0xe096b => {
            // block e096b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096b jmp 0xe0997
            pc = 0xe0997; continue;
        }
        0xe096d => {
            // block e096d
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e096d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e096d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // e0970 mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e0974 jne 0xe0997
            if crate::cov::brk(0xe0974, fl.ne()) { pc = 0xe0997; continue; }
            pc = 0xe0976;
        }
        0xe0976 => {
            // block e0976
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0976 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0976 cmp rax,r15
            fl = sub::<64>(rax, r15).1;
            // e0979 je 0xe0997
            if crate::cov::brk(0xe0979, fl.e()) { pc = 0xe0997; continue; }
            pc = 0xe097b;
        }
        0xe097b => {
            // block e097b
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e097b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e097b lea rsi,[rip+0x4a1fe] # 0x12ab80
            rsi = m.env.data_va(258893);
            // e0982 mov rdi,r14
            rdi = r14;
            // e0985 call 0xec2ae
            { let r = { let t = m.env.stub_va(0xa7b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e0997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0997 mov rax,rbx
            rax = rbx;
            pc = 0xe099a;
        }
        0xe099a => {
            // block e099a
            #[cfg(feature = "trace")] eprintln!("f_000e08f8 pc e099a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x11e28 size=180
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011e28<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11e28;
    loop {
        match pc {
        0x11e28 => {
            // block 11e28
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e28 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11e29 mov rbp,rsp
            rbp = rsp;
            // 11e2c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11e2e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11e30 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11e32 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11e34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11e35 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11e39 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // 11e3d mov r13,rdi
            r13 = rdi;
            // 11e40 cmp r13,rsi
            fl = sub::<64>(r13, rsi).1;
            // 11e43 je 0x11eb7
            if crate::cov::brk(0x11e43, fl.e()) { pc = 0x11eb7; continue; }
            pc = 0x11e45;
        }
        0x11e45 => {
            // block 11e45
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e45 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e45 lea rax,[r13+0x38]
            rax = r13.wrapping_add(0x38);
            // 11e49 cmp rax,QWORD PTR [rbp-0x68]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11e4d je 0x11eb7
            if crate::cov::brk(0x11e4d, fl.e()) { pc = 0x11eb7; continue; }
            pc = 0x11e4f;
        }
        0x11e4f => {
            // block 11e4f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e4f lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e53 mov r12,r13
            r12 = r13;
            pc = 0x11e56;
        }
        0x11e56 => {
            // block 11e56
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e56 mov r15,rax
            r15 = rax;
            // 11e59 mov rdi,rbx
            rdi = rbx;
            // 11e5c mov rsi,r15
            rsi = r15;
            // 11e5f call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11e64 cmp r15,r13
            fl = sub::<64>(r15, r13).1;
            // 11e67 mov rbx,r15
            rbx = r15;
            // 11e6a mov r14,r15
            r14 = r15;
            // 11e6d je 0x11e90
            if crate::cov::brk(0x11e6d, fl.e()) { pc = 0x11e90; continue; }
            pc = 0x11e6f;
        }
        0x11e6f => {
            // block 11e6f
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e6f mov eax,DWORD PTR [rbp-0x48]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 11e72 cmp eax,DWORD PTR [r14-0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // 11e76 jge 0x11e90
            if crate::cov::brk(0x11e76, fl.ge()) { pc = 0x11e90; continue; }
            pc = 0x11e78;
        }
        0x11e78 => {
            // block 11e78
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e78 add r14,0xffffffffffffffc8
            { let (r, f) = add::<64>(r14, 0xffff_ffff_ffff_ffc8u64);
              r14 = r;
              fl = f; }
            // 11e7c mov rdi,rbx
            rdi = rbx;
            // 11e7f mov rsi,r14
            rsi = r14;
            // 11e82 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11e87 add rbx,0xffffffffffffffc8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffc8u64);
              rbx = r;
              fl = f; }
            // 11e8b cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 11e8e jne 0x11e6f
            if crate::cov::brk(0x11e8e, fl.ne()) { pc = 0x11e6f; continue; }
            pc = 0x11e90;
        }
        0x11e90 => {
            // block 11e90
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11e90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11e90 mov rdi,rbx
            rdi = rbx;
            // 11e93 lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11e97 mov rsi,rbx
            rsi = rbx;
            // 11e9a call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11e9f mov rdi,rbx
            rdi = rbx;
            // 11ea2 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11ea7 add r12,0x70
            { let (r, f) = add::<64>(r12, 0x70u64);
              r12 = r;
              fl = f; }
            // 11eab cmp r12,QWORD PTR [rbp-0x68]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64).1;
            // 11eaf mov rax,r12
            rax = r12;
            // 11eb2 mov r12,r15
            r12 = r15;
            // 11eb5 jne 0x11e56
            if crate::cov::brk(0x11eb5, fl.ne()) { pc = 0x11e56; continue; }
            pc = 0x11eb7;
        }
        0x11eb7 => {
            // block 11eb7
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11eb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11eb7 add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 11ebb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11ebc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11ebe pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11ec0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11ec2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11ec4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11ec5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11ec6 => {
            // block 11ec6
            #[cfg(feature = "trace")] eprintln!("f_00011e28 pc 11ec6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11ec6 mov rbx,rax
            rbx = rax;
            // 11ec9 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 11ecd call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11ed2 mov rdi,rbx
            rdi = rbx;
            // 11ed5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 11eda jmp 0x11ec6
            pc = 0x11ec6; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb52d0 size=184
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b52d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb52d0;
    loop {
        match pc {
        0xb52d0 => {
            // block b52d0
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b52d1 mov rbp,rsp
            rbp = rsp;
            // b52d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b52d5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b52d6 mov rax,rdi
            rax = rdi;
            // b52d9 cmp rsi,rax
            fl = sub::<64>(rsi, rax).1;
            // b52dc sete BYTE PTR [rsi+0x18]
            m.w8(rsi.wrapping_add(0x18), ((fl.e() as u64)) as u8);
            // b52e0 je 0xb532b
            if crate::cov::brk(0xb52e0, fl.e()) { pc = 0xb532b; continue; }
            pc = 0xb52e2;
        }
        0xb52e2 => {
            // block b52e2
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52e2 mov rbx,QWORD PTR [rsi+0x10]
            rbx = m.r64(rsi.wrapping_add(0x10)) as u64;
            // b52e6 cmp BYTE PTR [rbx+0x18],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b52ea jne 0xb532b
            if crate::cov::brk(0xb52ea, fl.ne()) { pc = 0xb532b; continue; }
            pc = 0xb52ec;
        }
        0xb52ec => {
            // block b52ec
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52ec mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b52f0 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // b52f3 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // b52f6 je 0xb5305
            if crate::cov::brk(0xb52f6, fl.e()) { pc = 0xb5305; continue; }
            pc = 0xb52f8;
        }
        0xb52f8 => {
            // block b52f8
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52f8 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b52fb je 0xb5332
            if crate::cov::brk(0xb52fb, fl.e()) { pc = 0xb5332; continue; }
            pc = 0xb52fd;
        }
        0xb52fd => {
            // block b52fd
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b52fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b52fd cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b5301 je 0xb5314
            if crate::cov::brk(0xb5301, fl.e()) { pc = 0xb5314; continue; }
            pc = 0xb5303;
        }
        0xb5303 => {
            // block b5303
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5303 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5303 jmp 0xb5332
            pc = 0xb5332; continue;
        }
        0xb5305 => {
            // block b5305
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5305 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5305 mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // b5309 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b530c je 0xb535a
            if crate::cov::brk(0xb530c, fl.e()) { pc = 0xb535a; continue; }
            pc = 0xb530e;
        }
        0xb530e => {
            // block b530e
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b530e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b530e cmp BYTE PTR [rcx+0x18],0x0
            fl = sub::<8>(m.r8(rcx.wrapping_add(0x18)) as u64, 0x0u64).1;
            // b5312 jne 0xb535a
            if crate::cov::brk(0xb5312, fl.ne()) { pc = 0xb535a; continue; }
            pc = 0xb5314;
        }
        0xb5314 => {
            // block b5314
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5314 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5314 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b5318 cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // b531b sete BYTE PTR [rdi+0x18]
            m.w8(rdi.wrapping_add(0x18), ((fl.e() as u64)) as u8);
            // b531f mov BYTE PTR [rcx+0x18],0x1
            m.w8(rcx.wrapping_add(0x18), (0x1u64) as u8);
            // b5323 cmp rdi,rax
            fl = sub::<64>(rdi, rax).1;
            // b5326 mov rsi,rdi
            rsi = rdi;
            // b5329 jne 0xb52e2
            if crate::cov::brk(0xb5329, fl.ne()) { pc = 0xb52e2; continue; }
            pc = 0xb532b;
        }
        0xb532b => {
            // block b532b
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b532b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b532b add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b532f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5330 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5331 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb5332 => {
            // block b5332
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5332 cmp QWORD PTR [rbx],rsi
            fl = sub::<64>(m.r64(rbx) as u64, rsi).1;
            // b5335 jne 0xb5347
            if crate::cov::brk(0xb5335, fl.ne()) { pc = 0xb5347; continue; }
            pc = 0xb5337;
        }
        0xb5337 => {
            // block b5337
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5337 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5337 mov rdi,rbx
            rdi = rbx;
            // b533a call 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b533f mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b5343 mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xb5347;
        }
        0xb5347 => {
            // block b5347
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b5347 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5347 mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b534b mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // b534f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b5353 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5354 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5355 jmp 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb535a => {
            // block b535a
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b535a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b535a cmp QWORD PTR [rbx],rsi
            fl = sub::<64>(m.r64(rbx) as u64, rsi).1;
            // b535d je 0xb536f
            if crate::cov::brk(0xb535d, fl.e()) { pc = 0xb536f; continue; }
            pc = 0xb535f;
        }
        0xb535f => {
            // block b535f
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b535f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b535f mov rdi,rbx
            rdi = rbx;
            // b5362 call 0xebaaa
            { let r = { let t = m.env.stub_va(0x277); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b5367 mov rbx,QWORD PTR [rbx+0x10]
            rbx = m.r64(rbx.wrapping_add(0x10)) as u64;
            // b536b mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            pc = 0xb536f;
        }
        0xb536f => {
            // block b536f
            #[cfg(feature = "trace")] eprintln!("f_000b52d0 pc b536f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b536f mov BYTE PTR [rbx+0x18],0x1
            m.w8(rbx.wrapping_add(0x18), (0x1u64) as u8);
            // b5373 mov BYTE PTR [rdi+0x18],0x0
            m.w8(rdi.wrapping_add(0x18), (0x0u64) as u8);
            // b5377 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b537b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b537c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b537d jmp 0xeb86a
            { let r = { let t = m.env.stub_va(0x37); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8e30 size=188
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008e30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8e30;
    loop {
        match pc {
        0x8e30 => {
            // block 8e30
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8e31 mov rbp,rsp
            rbp = rsp;
            // 8e34 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8e36 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8e38 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 8e3a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8e3c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8e3d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 8e41 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 8e44 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 8e47 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 8e4a mov r13,rdi
            r13 = rdi;
            // 8e4d mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // 8e51 call 0xceac8
            { let r = { let t = m.env.code_va(0xceac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8e56 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 8e58 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // 8e5f mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // 8e66 mov BYTE PTR [rbp-0x31],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf), (0x1u64) as u8);
            // 8e6a lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 8e6e lea rdx,[rbp-0x30]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 8e72 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // 8e75 call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8e7a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8e7c je 0x8edb
            if crate::cov::brk(0x8e7c, fl.e()) { pc = 0x8edb; continue; }
            pc = 0x8e7e;
        }
        0x8e7e => {
            // block 8e7e
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e7e mov ecx,DWORD PTR [rbp-0x2c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8e81 dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // 8e83 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8e85 cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // 8e87 jae 0x8edd
            if crate::cov::brk(0x8e87, fl.ae()) { pc = 0x8edd; continue; }
            pc = 0x8e89;
        }
        0x8e89 => {
            // block 8e89
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e89 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e89 cmp DWORD PTR [rbp-0x30],0x3
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64, 0x3u64).1;
            // 8e8d ja 0x8edd
            if crate::cov::brk(0x8e8d, fl.a()) { pc = 0x8edd; continue; }
            pc = 0x8e8f;
        }
        0x8e8f => {
            // block 8e8f
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e8f test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 8e92 je 0x8eb1
            if crate::cov::brk(0x8e92, fl.e()) { pc = 0x8eb1; continue; }
            pc = 0x8e94;
        }
        0x8e94 => {
            // block 8e94
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e94 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8e98 lea rcx,[rbp-0x31]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffcf);
            // 8e9c mov rdi,r13
            rdi = r13;
            // 8e9f mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8ea2 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 8ea5 call QWORD PTR [rax+0x10f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10f8)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8eab cmp BYTE PTR [rbp-0x31],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffcf)) as u64, 0x0u64).1;
            // 8eaf je 0x8edb
            if crate::cov::brk(0x8eaf, fl.e()) { pc = 0x8edb; continue; }
            pc = 0x8eb1;
        }
        0x8eb1 => {
            // block 8eb1
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8eb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8eb1 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 8eb5 mov esi,DWORD PTR [rbp-0x2c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // 8eb8 mov edx,DWORD PTR [rbp-0x30]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 8ebb lea rcx,[rip+0xf67ce] # 0xff690
            rcx = m.env.data_va(81501);
            // 8ec2 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8ec5 lea rdi,[rip+0xf67d4] # 0xff6a0
            rdi = m.env.data_va(81517);
            // 8ecc mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8ed0 mov rdi,r13
            rdi = r13;
            // 8ed3 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 8ed6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 8ed9 jmp 0x8edd
            pc = 0x8edd; continue;
        }
        0x8edb => {
            // block 8edb
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8edb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8edb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x8edd;
        }
        0x8edd => {
            // block 8edd
            #[cfg(feature = "trace")] eprintln!("f_00008e30 pc 8edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8edd add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 8ee1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8ee2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8ee4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 8ee6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8ee8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8eea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8eeb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b562 size=192
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002b562<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2b562;
    loop {
        match pc {
        0x2b562 => {
            // block 2b562
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b562 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b562 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b563 mov rbp,rsp
            rbp = rsp;
            // 2b566 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b568 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b56a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2b56c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2b56e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b56f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2b573 mov r12,rsi
            r12 = rsi;
            // 2b576 mov DWORD PTR [rbp-0x38],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0xffffffffu64) as u32);
            // 2b57d mov DWORD PTR [rbp-0x34],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), (0xffffffffu64) as u32);
            // 2b584 mov DWORD PTR [rbp-0x30],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0xffffffffu64) as u32);
            // 2b58b mov rbx,QWORD PTR [rdi+0xc0]
            rbx = m.r64(rdi.wrapping_add(0xc0)) as u64;
            // 2b592 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2b595 je 0x2b611
            if crate::cov::brk(0x2b595, fl.e()) { pc = 0x2b611; continue; }
            pc = 0x2b597;
        }
        0x2b597 => {
            // block 2b597
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b597 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b597 mov eax,DWORD PTR [rdi+0xc8]
            rax = (m.r32(rdi.wrapping_add(0xc8)) as u64) as u32 as u64;
            // 2b59d mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 2b59f mov edx,0xaaaaaaab
            rdx = (0xaaaaaaabu64) as u32 as u64;
            // 2b5a4 imul rdx,rcx
            { let (r, f) = imul::<64>(rdx, rcx); rdx = r; fl = f; }
            // 2b5a8 shr rdx,0x23
            { let (r, f) = shr::<64>(rdx, 0x23u64, fl); rdx = r; fl = f; }
            // 2b5ac cmp eax,0x14f
            fl = sub::<32>((rax as u32 as u64), 0x14fu64).1;
            // 2b5b1 mov r13d,0x1b
            r13 = (0x1bu64) as u32 as u64;
            // 2b5b7 cmovbe r13d,edx
            if fl.be() { r13 = ((rdx as u32 as u64)) as u32 as u64; } else { r13 = r13 as u32 as u64; }
            // 2b5bb mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b5bd test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 2b5c0 je 0x2b613
            if crate::cov::brk(0x2b5c0, fl.e()) { pc = 0x2b613; continue; }
            pc = 0x2b5c2;
        }
        0x2b5c2 => {
            // block 2b5c2
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b5c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b5c2 lea r14,[r12+0x8]
            r14 = r12.wrapping_add(0x8);
            // 2b5c7 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2b5c9 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b5cc;
        }
        0x2b5cc => {
            // block 2b5cc
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b5cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b5cc mov DWORD PTR [rbp-0x3c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), ((rax as u32 as u64)) as u32);
            // 2b5cf mov rdi,r12
            rdi = r12;
            // 2b5d2 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // 2b5d6 call 0xeba92
            { let r = { let t = m.env.stub_va(0x25f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2b5db movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 2b5df lea rcx,[rcx+rcx*2]
            rcx = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 2b5e3 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 2b5e6 lea rax,[rax+0x20]
            rax = rax.wrapping_add(0x20);
            // 2b5ea lea rdx,[rbp-0x38]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 2b5ee cmove rax,rdx
            if fl.e() { rax = rdx; }
            // 2b5f2 mov edx,DWORD PTR [rax+0x8]
            rdx = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 2b5f5 mov DWORD PTR [rbx+rcx*4+0x8],edx
            m.w32(rbx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 2b5f9 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 2b5fc mov QWORD PTR [rbx+rcx*4],rax
            m.w64(rbx.wrapping_add(rcx.wrapping_mul(4)), (rax) as u64);
            // 2b600 lea r15d,[r15+0x1]
            r15 = (r15.wrapping_add(0x1)) as u32 as u64;
            // 2b604 movzx eax,r15w
            rax = ((r15 as u16 as u64)) as u32 as u64;
            // 2b608 cmp eax,r13d
            fl = sub::<32>((rax as u32 as u64), (r13 as u32 as u64)).1;
            // 2b60b jb 0x2b5cc
            if crate::cov::brk(0x2b60b, fl.b()) { pc = 0x2b5cc; continue; }
            pc = 0x2b60d;
        }
        0x2b60d => {
            // block 2b60d
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b60d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b60d mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2b60f jmp 0x2b613
            pc = 0x2b613; continue;
        }
        0x2b611 => {
            // block 2b611
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b611 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b611 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2b613;
        }
        0x2b613 => {
            // block 2b613
            #[cfg(feature = "trace")] eprintln!("f_0002b562 pc 2b613 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b613 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2b617 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b618 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2b61a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2b61c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b61e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b620 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b621 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5d4e size=196
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5d4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5d4e;
    loop {
        match pc {
        0xd5d4e => {
            // block d5d4e
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5d4f mov rbp,rsp
            rbp = rsp;
            // d5d52 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d5d54 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5d56 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d5d58 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5d59 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d5d5d mov r15,rdx
            r15 = rdx;
            // d5d60 mov rbx,rsi
            rbx = rsi;
            // d5d63 mov r14,rdi
            r14 = rdi;
            // d5d66 mov r12,r15
            r12 = r15;
            // d5d69 sub r12,rbx
            { let (r, f) = sub::<64>(r12, rbx);
              r12 = r;
              fl = f; }
            // d5d6c mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            // d5d6f mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // d5d74 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5d76 je 0xd5d85
            if crate::cov::brk(0xd5d76, fl.e()) { pc = 0xd5d85; continue; }
            pc = 0xd5d78;
        }
        0xd5d78 => {
            // block d5d78
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d78 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // d5d7b mov rsi,rax
            rsi = rax;
            // d5d7e and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // d5d82 dec rsi
            { let (r, f) = dec::<64>(rsi, fl); rsi = r; fl = f; }
            pc = 0xd5d85;
        }
        0xd5d85 => {
            // block d5d85
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d85 mov rdx,r12
            rdx = r12;
            // d5d88 sub rdx,rsi
            { let (r, f) = sub::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // d5d8b jbe 0xd5db6
            if crate::cov::brk(0xd5d8b, fl.be()) { pc = 0xd5db6; continue; }
            pc = 0xd5d8d;
        }
        0xd5d8d => {
            // block d5d8d
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d8d test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5d8f jne 0xd5d99
            if crate::cov::brk(0xd5d8f, fl.ne()) { pc = 0xd5d99; continue; }
            pc = 0xd5d91;
        }
        0xd5d91 => {
            // block d5d91
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d91 movzx ecx,al
            rcx = ((rax as u8 as u64)) as u32 as u64;
            // d5d94 shr rcx,1
            { let (r, f) = shr::<64>(rcx, 0x1u64, fl); rcx = r; fl = f; }
            // d5d97 jmp 0xd5d9d
            pc = 0xd5d9d; continue;
        }
        0xd5d99 => {
            // block d5d99
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d99 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            pc = 0xd5d9d;
        }
        0xd5d9d => {
            // block d5d9d
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5d9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d9d mov QWORD PTR [rsp],0x0
            m.w64(rsp, (0x0u64) as u64);
            // d5da5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5da8 mov rdi,r14
            rdi = r14;
            // d5dab mov r9,rcx
            r9 = rcx;
            // d5dae call 0xec01a
            { let r = { let t = m.env.stub_va(0x7e7); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d5db3 mov al,BYTE PTR [r14]
            rax = (rax & !0xff) | ((m.r8(r14) as u64) as u8 as u64);
            pc = 0xd5db6;
        }
        0xd5db6 => {
            // block d5db6
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5db6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5db6 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // d5db8 jne 0xd5dc0
            if crate::cov::brk(0xd5db8, fl.ne()) { pc = 0xd5dc0; continue; }
            pc = 0xd5dba;
        }
        0xd5dba => {
            // block d5dba
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dba lea rax,[r14+0x1]
            rax = r14.wrapping_add(0x1);
            // d5dbe jmp 0xd5dc4
            pc = 0xd5dc4; continue;
        }
        0xd5dc0 => {
            // block d5dc0
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc0 mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xd5dc4;
        }
        0xd5dc4 => {
            // block d5dc4
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc4 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // d5dc7 je 0xd5dde
            if crate::cov::brk(0xd5dc7, fl.e()) { pc = 0xd5dde; continue; }
            pc = 0xd5dc9;
        }
        0xd5dc9 => {
            // block d5dc9
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dc9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dc9 mov rcx,rax
            rcx = rax;
            pc = 0xd5dcc;
        }
        0xd5dcc => {
            // block d5dcc
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dcc mov dl,BYTE PTR [rbx]
            rdx = (rdx & !0xff) | ((m.r8(rbx) as u64) as u8 as u64);
            // d5dce mov BYTE PTR [rcx],dl
            m.w8(rcx, ((rdx as u8 as u64)) as u8);
            // d5dd0 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // d5dd3 inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // d5dd6 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // d5dd9 jne 0xd5dcc
            if crate::cov::brk(0xd5dd9, fl.ne()) { pc = 0xd5dcc; continue; }
            pc = 0xd5ddb;
        }
        0xd5ddb => {
            // block d5ddb
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5ddb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ddb add rax,r12
            { let (r, f) = add::<64>(rax, r12);
              rax = r;
              fl = f; }
            pc = 0xd5dde;
        }
        0xd5dde => {
            // block d5dde
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5dde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5dde mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // d5de1 test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // d5de5 jne 0xd5def
            if crate::cov::brk(0xd5de5, fl.ne()) { pc = 0xd5def; continue; }
            pc = 0xd5de7;
        }
        0xd5de7 => {
            // block d5de7
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5de7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5de7 add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // d5dea mov BYTE PTR [r14],r12b
            m.w8(r14, ((r12 as u8 as u64)) as u8);
            // d5ded jmp 0xd5df3
            pc = 0xd5df3; continue;
        }
        0xd5def => {
            // block d5def
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5def mov QWORD PTR [r14+0x8],r12
            m.w64(r14.wrapping_add(0x8), (r12) as u64);
            pc = 0xd5df3;
        }
        0xd5df3 => {
            // block d5df3
            #[cfg(feature = "trace")] eprintln!("f_000d5d4e pc d5df3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5df3 mov rax,r14
            rax = r14;
            // d5df6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d5dfa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5dfb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d5dfd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5dff pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d5e01 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5e02 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7e5e size=210
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7e5e;
    loop {
        match pc {
        0xb7e5e => {
            // block b7e5e
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7e5f mov rbp,rsp
            rbp = rsp;
            // b7e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7e65 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7e69 mov rbx,rdi
            rbx = rdi;
            // b7e6c mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // b7e73 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e76 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7e7c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e7f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7e81 je 0xb7f23
            if crate::cov::brk(0xb7e81, fl.e()) { pc = 0xb7f23; continue; }
            pc = 0xb7e87;
        }
        0xb7e87 => {
            // block b7e87
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e8a lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // b7e8e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e91 mov esi,0x18
            rsi = (0x18u64) as u32 as u64;
            // b7e96 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b7e9b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7e9e mov rdi,rbx
            rdi = rbx;
            // b7ea1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b7ea4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7ea6 je 0xb7f23
            if crate::cov::brk(0xb7ea6, fl.e()) { pc = 0xb7f23; continue; }
            pc = 0xb7ea8;
        }
        0xb7ea8 => {
            // block b7ea8
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ea8 mov eax,DWORD PTR [rbx+0x10]
            rax = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b7eab cmp eax,0x104782ff
            fl = sub::<32>((rax as u32 as u64), 0x104782ffu64).1;
            // b7eb0 jg 0xb7ed2
            if crate::cov::brk(0xb7eb0, fl.g()) { pc = 0xb7ed2; continue; }
            pc = 0xb7eb2;
        }
        0xb7eb2 => {
            // block b7eb2
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb2 cmp eax,0x1032294f
            fl = sub::<32>((rax as u32 as u64), 0x1032294fu64).1;
            // b7eb7 jg 0xb7ee9
            if crate::cov::brk(0xb7eb7, fl.g()) { pc = 0xb7ee9; continue; }
            pc = 0xb7eb9;
        }
        0xb7eb9 => {
            // block b7eb9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb9 cmp eax,0x10266400
            fl = sub::<32>((rax as u32 as u64), 0x10266400u64).1;
            // b7ebe jne 0xb7f09
            if crate::cov::brk(0xb7ebe, fl.ne()) { pc = 0xb7f09; continue; }
            pc = 0xb7ec0;
        }
        0xb7ec0 => {
            // block b7ec0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ec0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ec0 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7ec3 and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7ec5 shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7ec8 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7ecb lea r14d,[rax+rax*4+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1)) as u32 as u64;
            // b7ed0 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7ed2 => {
            // block b7ed2
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed2 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // b7ed7 je 0xb7f10
            if crate::cov::brk(0xb7ed7, fl.e()) { pc = 0xb7f10; continue; }
            pc = 0xb7ed9;
        }
        0xb7ed9 => {
            // block b7ed9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed9 cmp eax,0x10518400
            fl = sub::<32>((rax as u32 as u64), 0x10518400u64).1;
            // b7ede je 0xb7f10
            if crate::cov::brk(0xb7ede, fl.e()) { pc = 0xb7f10; continue; }
            pc = 0xb7ee0;
        }
        0xb7ee0 => {
            // block b7ee0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee0 cmp eax,0x10518450
            fl = sub::<32>((rax as u32 as u64), 0x10518450u64).1;
            // b7ee5 jne 0xb7f23
            if crate::cov::brk(0xb7ee5, fl.ne()) { pc = 0xb7f23; continue; }
            pc = 0xb7ee7;
        }
        0xb7ee7 => {
            // block b7ee7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee7 jmp 0xb7f10
            pc = 0xb7f10; continue;
        }
        0xb7ee9 => {
            // block b7ee9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee9 cmp eax,0x10322950
            fl = sub::<32>((rax as u32 as u64), 0x10322950u64).1;
            // b7eee je 0xb7f10
            if crate::cov::brk(0xb7eee, fl.e()) { pc = 0xb7f10; continue; }
            pc = 0xb7ef0;
        }
        0xb7ef0 => {
            // block b7ef0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ef0 cmp eax,0x10352300
            fl = sub::<32>((rax as u32 as u64), 0x10352300u64).1;
            // b7ef5 jne 0xb7f23
            if crate::cov::brk(0xb7ef5, fl.ne()) { pc = 0xb7f23; continue; }
            pc = 0xb7ef7;
        }
        0xb7ef7 => {
            // block b7ef7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ef7 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7efa and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7efc shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7eff movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7f02 lea r14d,[rax+rax*2+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x1)) as u32 as u64;
            // b7f07 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7f09 => {
            // block b7f09
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f09 cmp eax,0x10294700
            fl = sub::<32>((rax as u32 as u64), 0x10294700u64).1;
            // b7f0e jne 0xb7f23
            if crate::cov::brk(0xb7f0e, fl.ne()) { pc = 0xb7f23; continue; }
            pc = 0xb7f10;
        }
        0xb7f10 => {
            // block b7f10
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f10 test BYTE PTR [rbp-0x11],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64 & 0x8u64);
            // b7f14 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // b7f19 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // b7f1f cmove r14d,eax
            if fl.e() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            pc = 0xb7f23;
        }
        0xb7f23 => {
            // block b7f23
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f23 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // b7f26 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7f2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7f2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7f2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7f2e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x523cc size=218
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000523cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x523cc;
    loop {
        match pc {
        0x523cc => {
            // block 523cc
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 523cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 523cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 523cd mov rbp,rsp
            rbp = rsp;
            // 523d0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 523d2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 523d3 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 523d7 mov QWORD PTR [rip+0x175eb2],rdi # 0x1c8290
            m.w64(m.env.data_va(903773), (rdi) as u64);
            // 523de mov rdi,QWORD PTR [rip+0xedd9b] # 0x140180
            rdi = m.r64(m.env.data_va(346445)) as u64;
            // 523e5 lea rsi,[rip+0xd8bfa] # 0x12afe6
            rsi = m.env.data_va(260019);
            // 523ec mov edx,0x2b
            rdx = (0x2bu64) as u32 as u64;
            // 523f1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 523f6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 523f9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 523fd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52401 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52404 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 52407 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // 5240b mov rsi,QWORD PTR [rip+0x175e7e] # 0x1c8290
            rsi = m.r64(m.env.data_va(903773)) as u64;
            // 52412 mov rdi,rax
            rdi = rax;
            // 52415 call 0xec0b6
            { let r = { let t = m.env.stub_va(0x883); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5241a mov rbx,rax
            rbx = rax;
            // 5241d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52420 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52424 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // 52428 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242b or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5242e mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // 52432 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 52435 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 52439 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 5243c lea r14,[rbp-0x18]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52440 mov rdi,r14
            rdi = r14;
            // 52443 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52448 mov rsi,QWORD PTR [rip+0xedd41] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 5244f mov rdi,r14
            rdi = r14;
            // 52452 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52457 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 5245a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5245e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 52463 mov rdi,rax
            rdi = rax;
            // 52466 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52468 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 5246b lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 5246f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52474 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // 52478 mov rdi,rbx
            rdi = rbx;
            // 5247b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52480 mov rdi,rbx
            rdi = rbx;
            // 52483 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 52488 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5248c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5248d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5248f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 52490 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x52491 => {
            // block 52491
            #[cfg(feature = "trace")] eprintln!("f_000523cc pc 52491 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 52491 mov rbx,rax
            rbx = rax;
            // 52494 lea rdi,[rbp-0x18]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 52498 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5249d mov rdi,rbx
            rdi = rbx;
            // 524a0 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
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
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            if crate::cov::brk(0xd1a5, fl.e()) { pc = 0xd216; continue; }
            pc = 0xd1a7;
        }
        0xd1a7 => {
            // block d1a7
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1a7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1a7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1aa mov rdi,r15
            rdi = r15;
            // d1ad call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d1b3 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // d1b8 je 0xd206
            if crate::cov::brk(0xd1b8, fl.e()) { pc = 0xd206; continue; }
            pc = 0xd1ba;
        }
        0xd1ba => {
            // block d1ba
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1ba mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1bd mov rdi,r15
            rdi = r15;
            // d1c0 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d1c6 cmp eax,0x10478350
            fl = sub::<32>((rax as u32 as u64), 0x10478350u64).1;
            // d1cb je 0xd206
            if crate::cov::brk(0xd1cb, fl.e()) { pc = 0xd206; continue; }
            pc = 0xd1cd;
        }
        0xd1cd => {
            // block d1cd
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1cd mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1d0 mov rdi,r15
            rdi = r15;
            // d1d3 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d1d9 cmp eax,0x10710800
            fl = sub::<32>((rax as u32 as u64), 0x10710800u64).1;
            // d1de je 0xd206
            if crate::cov::brk(0xd1de, fl.e()) { pc = 0xd206; continue; }
            pc = 0xd1e0;
        }
        0xd1e0 => {
            // block d1e0
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1e0 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1e3 mov rdi,r15
            rdi = r15;
            // d1e6 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d1ec cmp eax,0x10710850
            fl = sub::<32>((rax as u32 as u64), 0x10710850u64).1;
            // d1f1 je 0xd206
            if crate::cov::brk(0xd1f1, fl.e()) { pc = 0xd206; continue; }
            pc = 0xd1f3;
        }
        0xd1f3 => {
            // block d1f3
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d1f3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d1f3 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d1f6 mov rdi,r15
            rdi = r15;
            // d1f9 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // d1ff cmp eax,0x10710851
            fl = sub::<32>((rax as u32 as u64), 0x10710851u64).1;
            // d204 jne 0xd216
            if crate::cov::brk(0xd204, fl.ne()) { pc = 0xd216; continue; }
            pc = 0xd206;
        }
        0xd206 => {
            // block d206
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d206 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d209 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // d20d mov rdi,r15
            rdi = r15;
            // d210 call QWORD PTR [rax+0x328]
            { let r = { let t = m.r64(rax.wrapping_add(0x328)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xd216;
        }
        0xd216 => {
            // block d216
            #[cfg(feature = "trace")] eprintln!("f_0000d192 pc d216 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d216 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // d219 mov rdi,r15
            rdi = r15;
            // d21c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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

/// parity: P2v @0xbaa18 size=226
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000baa18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbaa18;
    loop {
        match pc {
        0xbaa18 => {
            // block baa18
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // baa19 mov rbp,rsp
            rbp = rsp;
            // baa1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // baa1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // baa20 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // baa22 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // baa24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // baa25 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // baa29 mov rbx,rdx
            rbx = rdx;
            // baa2c mov r14,rsi
            r14 = rsi;
            // baa2f mov r15,rdi
            r15 = rdi;
            // baa32 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // baa35 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // baa38 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // baa3b add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // baa3f cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // baa42 je 0xbaae8
            if crate::cov::brk(0xbaa42, fl.e()) { pc = 0xbaae8; continue; }
            pc = 0xbaa48;
        }
        0xbaa48 => {
            // block baa48
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa48 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // baa4c lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xbaa50;
        }
        0xbaa50 => {
            // block baa50
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa50 mov DWORD PTR [rbp-0x2c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), (0x0u64) as u32);
            // baa57 mov DWORD PTR [rbp-0x30],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u32);
            // baa5e mov edi,DWORD PTR [r12+0x1c]
            rdi = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // baa63 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // baa67 mov rdx,r13
            rdx = r13;
            // baa6a call 0x6858c
            { let r = { let t = m.env.code_va(0x6858c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // baa6f mov edi,DWORD PTR [rbp-0x2c]
            rdi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // baa72 mov rsi,rbx
            rsi = rbx;
            // baa75 call 0x2ca4c
            { let r = { let t = m.env.code_va(0x2ca4c); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // baa7a cmp rax,QWORD PTR [rbx+0x8]
            fl = sub::<64>(rax, m.r64(rbx.wrapping_add(0x8)) as u64).1;
            // baa7e je 0xbaab7
            if crate::cov::brk(0xbaa7e, fl.e()) { pc = 0xbaab7; continue; }
            pc = 0xbaa80;
        }
        0xbaa80 => {
            // block baa80
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baa80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baa80 mov ecx,DWORD PTR [rbp-0x30]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // baa83 lea rdx,[rip+0x47f76] # 0x102a00
            rdx = m.env.data_va(94669);
            // baa8a mov edx,DWORD PTR [rdx+rcx*4]
            rdx = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // baa8d and edx,DWORD PTR [rax+0x4]
            { let r = (rdx as u32 as u64) & m.r32(rax.wrapping_add(0x4)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // baa90 lea rax,[rip+0x47f79] # 0x102a10
            rax = m.env.data_va(94685);
            // baa97 mov cl,BYTE PTR [rax+rcx*4]
            rcx = (rcx & !0xff) | ((m.r8(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u8 as u64);
            // baa9a shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // baa9c test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // baa9e je 0xbaab7
            if crate::cov::brk(0xbaa9e, fl.e()) { pc = 0xbaab7; continue; }
            pc = 0xbaaa0;
        }
        0xbaaa0 => {
            // block baaa0
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baaa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baaa0 mov eax,DWORD PTR [r12+0x1c]
            rax = (m.r32(r12.wrapping_add(0x1c)) as u64) as u32 as u64;
            // baaa5 mov DWORD PTR [rbp-0x38],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rax as u32 as u64)) as u32);
            // baaa8 mov DWORD PTR [rbp-0x34],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rdx as u32 as u64)) as u32);
            // baaab mov rdi,r15
            rdi = r15;
            // baaae lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // baab2 call 0xebe04
            { let r = { let t = m.env.stub_va(0x5d1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xbaab7;
        }
        0xbaab7 => {
            // block baab7
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baab7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baab7 mov rax,QWORD PTR [r12+0x8]
            rax = m.r64(r12.wrapping_add(0x8)) as u64;
            // baabc test rax,rax
            fl = logic::<64>(rax & rax);
            // baabf je 0xbaacf
            if crate::cov::brk(0xbaabf, fl.e()) { pc = 0xbaacf; continue; }
            pc = 0xbaac1;
        }
        0xbaac1 => {
            // block baac1
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baac1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baac1 mov r12,rax
            r12 = rax;
            // baac4 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // baac8 test rax,rax
            fl = logic::<64>(rax & rax);
            // baacb jne 0xbaac1
            if crate::cov::brk(0xbaacb, fl.ne()) { pc = 0xbaac1; continue; }
            pc = 0xbaacd;
        }
        0xbaacd => {
            // block baacd
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baacd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baacd jmp 0xbaadf
            pc = 0xbaadf; continue;
        }
        0xbaacf => {
            // block baacf
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baacf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baacf mov rax,r12
            rax = r12;
            pc = 0xbaad2;
        }
        0xbaad2 => {
            // block baad2
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baad2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baad2 mov r12,QWORD PTR [rax+0x10]
            r12 = m.r64(rax.wrapping_add(0x10)) as u64;
            // baad6 cmp QWORD PTR [r12],rax
            fl = sub::<64>(m.r64(r12) as u64, rax).1;
            // baada mov rax,r12
            rax = r12;
            // baadd jne 0xbaad2
            if crate::cov::brk(0xbaadd, fl.ne()) { pc = 0xbaad2; continue; }
            pc = 0xbaadf;
        }
        0xbaadf => {
            // block baadf
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baadf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baadf cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // baae2 jne 0xbaa50
            if crate::cov::brk(0xbaae2, fl.ne()) { pc = 0xbaa50; continue; }
            pc = 0xbaae8;
        }
        0xbaae8 => {
            // block baae8
            #[cfg(feature = "trace")] eprintln!("f_000baa18 pc baae8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // baae8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // baaea add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // baaee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // baaef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // baaf1 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // baaf3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // baaf5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // baaf7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // baaf8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2cd5c size=230
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002cd5c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2cd5c;
    loop {
        match pc {
        0x2cd5c => {
            // block 2cd5c
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2cd5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cd5c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2cd5d mov rbp,rsp
            rbp = rsp;
            // 2cd60 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2cd62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2cd64 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2cd66 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2cd67 mov r15,rsi
            r15 = rsi;
            // 2cd6a mov rbx,rdi
            rbx = rdi;
            // 2cd6d mov r12,QWORD PTR [rbx+0x24]
            r12 = m.r64(rbx.wrapping_add(0x24)) as u64;
            // 2cd71 mov r14,QWORD PTR [rbx+0x4c]
            r14 = m.r64(rbx.wrapping_add(0x4c)) as u64;
            // 2cd75 call 0x26172
            { let r = { let t = m.env.code_va(0x26172); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cd7a lea rsi,[rip+0xfd513] # 0x12a294
            rsi = m.env.data_va(256609);
            // 2cd81 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2cd86 mov rdi,rax
            rdi = rax;
            // 2cd89 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cd8e cmp DWORD PTR [rbx+0x20],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x20)) as u64, 0x0u64).1;
            // 2cd92 lea rcx,[rip+0xfdc12] # 0x12a9ab
            rcx = m.env.data_va(258424);
            // 2cd99 lea rsi,[rip+0xfdc0d] # 0x12a9ad
            rsi = m.env.data_va(258426);
            // 2cda0 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // 2cda4 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2cda9 mov rdi,rax
            rdi = rax;
            // 2cdac call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cdb1 lea rsi,[rip+0xfdbf7] # 0x12a9af
            rsi = m.env.data_va(258428);
            // 2cdb8 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 2cdbd mov rdi,rax
            rdi = rax;
            // 2cdc0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cdc5 cmp QWORD PTR [rbx+0x24],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 2cdca je 0x2cddf
            if crate::cov::brk(0x2cdca, fl.e()) { pc = 0x2cddf; continue; }
            pc = 0x2cdcc;
        }
        0x2cdcc => {
            // block 2cdcc
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2cdcc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cdcc cmp DWORD PTR [rbx+0x2c],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x2c)) as u64, 0x0u64).1;
            // 2cdd0 je 0x2cddf
            if crate::cov::brk(0x2cdd0, fl.e()) { pc = 0x2cddf; continue; }
            pc = 0x2cdd2;
        }
        0x2cdd2 => {
            // block 2cdd2
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2cdd2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cdd2 mov rdi,r15
            rdi = r15;
            // 2cdd5 mov rsi,r12
            rsi = r12;
            // 2cdd8 call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2cddd jmp 0x2cdf3
            pc = 0x2cdf3; continue;
        }
        0x2cddf => {
            // block 2cddf
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2cddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cddf lea rsi,[rip+0xfdb9e] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2cde6 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2cdeb mov rdi,r15
            rdi = r15;
            // 2cdee call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x2cdf3;
        }
        0x2cdf3 => {
            // block 2cdf3
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2cdf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2cdf3 lea rsi,[rip+0xfdbc3] # 0x12a9bd
            rsi = m.env.data_va(258442);
            // 2cdfa mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 2cdff mov rdi,r15
            rdi = r15;
            // 2ce02 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ce07 cmp QWORD PTR [rbx+0x4c],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x4c)) as u64, 0x0u64).1;
            // 2ce0c je 0x2ce21
            if crate::cov::brk(0x2ce0c, fl.e()) { pc = 0x2ce21; continue; }
            pc = 0x2ce0e;
        }
        0x2ce0e => {
            // block 2ce0e
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2ce0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ce0e cmp DWORD PTR [rbx+0x54],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x54)) as u64, 0x0u64).1;
            // 2ce12 je 0x2ce21
            if crate::cov::brk(0x2ce12, fl.e()) { pc = 0x2ce21; continue; }
            pc = 0x2ce14;
        }
        0x2ce14 => {
            // block 2ce14
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2ce14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ce14 mov rdi,r15
            rdi = r15;
            // 2ce17 mov rsi,r14
            rsi = r14;
            // 2ce1a call 0x2c238
            { let r = { let t = m.env.code_va(0x2c238); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2ce1f jmp 0x2ce35
            pc = 0x2ce35; continue;
        }
        0x2ce21 => {
            // block 2ce21
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2ce21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ce21 lea rsi,[rip+0xfdb5c] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2ce28 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2ce2d mov rdi,r15
            rdi = r15;
            // 2ce30 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x2ce35;
        }
        0x2ce35 => {
            // block 2ce35
            #[cfg(feature = "trace")] eprintln!("f_0002cd5c pc 2ce35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ce35 mov rax,r15
            rax = r15;
            // 2ce38 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ce39 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ce3b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ce3d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ce3f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ce40 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5d10 size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00005d10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5d10;
    loop {
        match pc {
        0x5d10 => {
            // block 5d10
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5d11 mov rbp,rsp
            rbp = rsp;
            // 5d14 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 5d16 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5d18 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5d1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 5d1b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5d1f mov r14,rdx
            r14 = rdx;
            // 5d22 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 5d25 mov rbx,rdi
            rbx = rdi;
            // 5d28 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d2b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d33 jne 0x5ddf
            if crate::cov::brk(0x5d33, fl.ne()) { pc = 0x5ddf; continue; }
            pc = 0x5d39;
        }
        0x5d39 => {
            // block 5d39
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d39 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d3c lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 5d40 mov rdi,rbx
            rdi = rbx;
            // 5d43 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 5d46 call QWORD PTR [rax+0x538]
            { let r = { let t = m.r64(rax.wrapping_add(0x538)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d4c mov r12d,DWORD PTR [rbp-0x24]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 5d50 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d53 mov rdi,rbx
            rdi = rbx;
            // 5d56 call QWORD PTR [rax+0x128]
            { let r = { let t = m.r64(rax.wrapping_add(0x128)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d5c cmp r12d,eax
            fl = sub::<32>((r12 as u32 as u64), (rax as u32 as u64)).1;
            // 5d5f jbe 0x5d68
            if crate::cov::brk(0x5d5f, fl.be()) { pc = 0x5d68; continue; }
            pc = 0x5d61;
        }
        0x5d61 => {
            // block 5d61
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d61 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d61 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            pc = 0x5d68;
        }
        0x5d68 => {
            // block 5d68
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d68 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d6b mov rdi,rbx
            rdi = rbx;
            // 5d6e call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d74 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 5d76 call 0xcdd26
            { let r = { let t = m.env.code_va(0xcdd26); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d7b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d7d je 0x5db7
            if crate::cov::brk(0x5d7d, fl.e()) { pc = 0x5db7; continue; }
            pc = 0x5d7f;
        }
        0x5d7f => {
            // block 5d7f
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d7f mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            // 5d86 test rax,rax
            fl = logic::<64>(rax & rax);
            // 5d89 jne 0x5da2
            if crate::cov::brk(0x5d89, fl.ne()) { pc = 0x5da2; continue; }
            pc = 0x5d8b;
        }
        0x5d8b => {
            // block 5d8b
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d8b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5d8e mov rdi,rbx
            rdi = rbx;
            // 5d91 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5d97 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5d99 je 0x5ddf
            if crate::cov::brk(0x5d99, fl.e()) { pc = 0x5ddf; continue; }
            pc = 0x5d9b;
        }
        0x5d9b => {
            // block 5d9b
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5d9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5d9b mov rax,QWORD PTR [rbx+0x210]
            rax = m.r64(rbx.wrapping_add(0x210)) as u64;
            pc = 0x5da2;
        }
        0x5da2 => {
            // block 5da2
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5da2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5da2 mov ecx,DWORD PTR [rbx+0x24c]
            rcx = (m.r32(rbx.wrapping_add(0x24c)) as u64) as u32 as u64;
            // 5da8 imul ecx,DWORD PTR [rbp-0x24]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // 5dac and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 5daf add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 5db2 mov QWORD PTR [r14],rcx
            m.w64(r14, (rcx) as u64);
            // 5db5 jmp 0x5ded
            pc = 0x5ded; continue;
        }
        0x5db7 => {
            // block 5db7
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5db7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5db7 cmp QWORD PTR [rbx+0x218],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x218)) as u64, 0x0u64).1;
            // 5dbf jne 0x5dd1
            if crate::cov::brk(0x5dbf, fl.ne()) { pc = 0x5dd1; continue; }
            pc = 0x5dc1;
        }
        0x5dc1 => {
            // block 5dc1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dc1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 5dc4 mov rdi,rbx
            rdi = rbx;
            // 5dc7 call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 5dcd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5dcf je 0x5ddf
            if crate::cov::brk(0x5dcf, fl.e()) { pc = 0x5ddf; continue; }
            pc = 0x5dd1;
        }
        0x5dd1 => {
            // block 5dd1
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd1 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 5dd4 je 0x5de3
            if crate::cov::brk(0x5dd4, fl.e()) { pc = 0x5de3; continue; }
            pc = 0x5dd6;
        }
        0x5dd6 => {
            // block 5dd6
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dd6 mov rax,QWORD PTR [rbx+0x220]
            rax = m.r64(rbx.wrapping_add(0x220)) as u64;
            // 5ddd jmp 0x5dea
            pc = 0x5dea; continue;
        }
        0x5ddf => {
            // block 5ddf
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ddf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ddf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5de1 jmp 0x5def
            pc = 0x5def; continue;
        }
        0x5de3 => {
            // block 5de3
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5de3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5de3 mov rax,QWORD PTR [rbx+0x218]
            rax = m.r64(rbx.wrapping_add(0x218)) as u64;
            pc = 0x5dea;
        }
        0x5dea => {
            // block 5dea
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5dea mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            pc = 0x5ded;
        }
        0x5ded => {
            // block 5ded
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5ded rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ded mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x5def;
        }
        0x5def => {
            // block 5def
            #[cfg(feature = "trace")] eprintln!("f_00005d10 pc 5def rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5def add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 5df3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5df4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 5df6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 5df8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 5dfa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5dfb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8f6a size=241
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8f6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8f6a;
    loop {
        match pc {
        0xb8f6a => {
            // block b8f6a
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8f6b mov rbp,rsp
            rbp = rsp;
            // b8f6e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8f70 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8f72 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8f74 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8f76 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8f77 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8f7b mov r12,rsi
            r12 = rsi;
            // b8f7e mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // b8f82 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8f86 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8f8b cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8f8e je 0xb9034
            if crate::cov::brk(0xb8f8e, fl.e()) { pc = 0xb9034; continue; }
            pc = 0xb8f94;
        }
        0xb8f94 => {
            // block b8f94
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f94 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8f98 mov r14,rcx
            r14 = rcx;
            // b8f9b jmp 0xb8fa4
            pc = 0xb8fa4; continue;
        }
        0xb8f9d => {
            // block b8f9d
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8f9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f9d mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8fa1 mov r14,rax
            r14 = rax;
            pc = 0xb8fa4;
        }
        0xb8fa4 => {
            // block b8fa4
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8fa4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8fa4 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8fa7 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8faa lea rsi,[rip+0x7fd17] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8fb1 lea rcx,[rip+0x714e5] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8fb8 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8fbc movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8fbf add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8fc2 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8fc6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b8fcb mov rbx,rax
            rbx = rax;
            // b8fce mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8fd2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8fd7 mov rdi,r15
            rdi = r15;
            // b8fda call 0x61146
            { let r = { let t = m.env.code_va(0x61146); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b8fdf movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8fe3 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8fe6 jne 0xb8ff1
            if crate::cov::brk(0xb8fe6, fl.ne()) { pc = 0xb8ff1; continue; }
            pc = 0xb8fe8;
        }
        0xb8fe8 => {
            // block b8fe8
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8fe8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8fe8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8feb lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8fef jmp 0xb8ff9
            pc = 0xb8ff9; continue;
        }
        0xb8ff1 => {
            // block b8ff1
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8ff1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ff1 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8ff5 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8ff9;
        }
        0xb8ff9 => {
            // block b8ff9
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b8ff9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ff9 mov rdi,rbx
            rdi = rbx;
            // b8ffc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b9001 mov rdi,r15
            rdi = r15;
            // b9004 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b9009 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b900d test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b9010 je 0xb901f
            if crate::cov::brk(0xb9010, fl.e()) { pc = 0xb901f; continue; }
            pc = 0xb9012;
        }
        0xb9012 => {
            // block b9012
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9012 mov rax,rcx
            rax = rcx;
            // b9015 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b9018 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b901b jne 0xb9012
            if crate::cov::brk(0xb901b, fl.ne()) { pc = 0xb9012; continue; }
            pc = 0xb901d;
        }
        0xb901d => {
            // block b901d
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b901d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b901d jmp 0xb902b
            pc = 0xb902b; continue;
        }
        0xb901f => {
            // block b901f
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b901f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b901f mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b9023 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b9026 mov r14,rax
            r14 = rax;
            // b9029 jne 0xb901f
            if crate::cov::brk(0xb9029, fl.ne()) { pc = 0xb901f; continue; }
            pc = 0xb902b;
        }
        0xb902b => {
            // block b902b
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b902b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b902b cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b902e jne 0xb8f9d
            if crate::cov::brk(0xb902e, fl.ne()) { pc = 0xb8f9d; continue; }
            pc = 0xb9034;
        }
        0xb9034 => {
            // block b9034
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9034 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b9038 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b903c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b903d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b903f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b9041 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9043 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9045 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9046 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb9047 => {
            // block b9047
            #[cfg(feature = "trace")] eprintln!("f_000b8f6a pc b9047 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9047 mov rbx,rax
            rbx = rax;
            // b904a lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b904e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // b9053 mov rdi,rbx
            rdi = rbx;
            // b9056 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x33a14 size=244
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00033a14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x33a14;
    loop {
        match pc {
        0x33a14 => {
            // block 33a14
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 33a15 mov rbp,rsp
            rbp = rsp;
            // 33a18 mov r8,rdx
            r8 = rdx;
            // 33a1b mov BYTE PTR [rcx],0x0
            m.w8(rcx, (0x0u64) as u8);
            // 33a1e mov DWORD PTR [r8],0x0
            m.w32(r8, (0x0u64) as u32);
            // 33a25 mov edx,DWORD PTR [rdi]
            rdx = (m.r32(rdi) as u64) as u32 as u64;
            // 33a27 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // 33a29 je 0x33a7b
            if crate::cov::brk(0x33a29, fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a2b;
        }
        0x33a2b => {
            // block 33a2b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a2b cmp DWORD PTR [rdi+0x4],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x4)) as u64, 0x0u64).1;
            // 33a2f je 0x33a7b
            if crate::cov::brk(0x33a2f, fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a31;
        }
        0x33a31 => {
            // block 33a31
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a31 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a31 cmp WORD PTR [rdi+0x34],0x0
            fl = sub::<16>(m.r16(rdi.wrapping_add(0x34)) as u64, 0x0u64).1;
            // 33a36 je 0x33a7b
            if crate::cov::brk(0x33a36, fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a38;
        }
        0x33a38 => {
            // block 33a38
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a38 cmp DWORD PTR [rdi+0x24],0x0
            fl = sub::<32>(m.r32(rdi.wrapping_add(0x24)) as u64, 0x0u64).1;
            // 33a3c je 0x33a7b
            if crate::cov::brk(0x33a3c, fl.e()) { pc = 0x33a7b; continue; }
            pc = 0x33a3e;
        }
        0x33a3e => {
            // block 33a3e
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a3e mov r9d,DWORD PTR [rdi+0x10]
            r9 = (m.r32(rdi.wrapping_add(0x10)) as u64) as u32 as u64;
            // 33a42 cmp r9,0xb
            fl = sub::<64>(r9, 0xbu64).1;
            // 33a46 ja 0x33a7b
            if crate::cov::brk(0x33a46, fl.a()) { pc = 0x33a7b; continue; }
            pc = 0x33a48;
        }
        0x33a48 => {
            // block 33a48
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a48 mov edi,DWORD PTR [rdi+0x1c]
            rdi = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 33a4b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a4d cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33a50 ja 0x33a7d
            if crate::cov::brk(0x33a50, fl.a()) { pc = 0x33a7d; continue; }
            pc = 0x33a52;
        }
        0x33a52 => {
            // block 33a52
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a52 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // 33a54 jbe 0x33a7d
            if crate::cov::brk(0x33a54, fl.be()) { pc = 0x33a7d; continue; }
            pc = 0x33a56;
        }
        0x33a56 => {
            // block 33a56
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a56 lea eax,[r9-0x1]
            rax = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a5a cmp eax,0xb
            fl = sub::<32>((rax as u32 as u64), 0xbu64).1;
            // 33a5d jae 0x33a69
            if crate::cov::brk(0x33a5d, fl.ae()) { pc = 0x33a69; continue; }
            pc = 0x33a5f;
        }
        0x33a5f => {
            // block 33a5f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a5f mov edx,0x7a9
            rdx = (0x7a9u64) as u32 as u64;
            // 33a64 bt edx,eax
            fl.cf = bit::<32>((rdx as u32 as u64), (rax as u32 as u64));
            // 33a67 jb 0x33a7f
            if crate::cov::brk(0x33a67, fl.b()) { pc = 0x33a7f; continue; }
            pc = 0x33a69;
        }
        0x33a69 => {
            // block 33a69
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a69 cmp r9d,0x2
            fl = sub::<32>((r9 as u32 as u64), 0x2u64).1;
            // 33a6d sete dl
            rdx = (rdx & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 33a70 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33a72 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 33a75 xor al,dl
            { let r = (rax as u8 as u64) ^ (rdx as u8 as u64); let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 33a77 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // 33a79 jmp 0x33a81
            pc = 0x33a81; continue;
        }
        0x33a7b => {
            // block 33a7b
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a7d;
        }
        0x33a7d => {
            // block 33a7d
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 33a7e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x33a7f => {
            // block 33a7f
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a7f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x33a81;
        }
        0x33a81 => {
            // block 33a81
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a81 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a81 lea ecx,[r9-0x1]
            rcx = (r9.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // 33a85 cmp ecx,0xa
            fl = sub::<32>((rcx as u32 as u64), 0xau64).1;
            // 33a88 jae 0x33a9a
            if crate::cov::brk(0x33a88, fl.ae()) { pc = 0x33a9a; continue; }
            pc = 0x33a8a;
        }
        0x33a8a => {
            // block 33a8a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a8a mov r10d,0x1
            r10 = (0x1u64) as u32 as u64;
            // 33a90 mov edx,0x3a9
            rdx = (0x3a9u64) as u32 as u64;
            // 33a95 bt edx,ecx
            fl.cf = bit::<32>((rdx as u32 as u64), (rcx as u32 as u64));
            // 33a98 jb 0x33aa8
            if crate::cov::brk(0x33a98, fl.b()) { pc = 0x33aa8; continue; }
            pc = 0x33a9a;
        }
        0x33a9a => {
            // block 33a9a
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33a9a cmp r9d,0xb
            fl = sub::<32>((r9 as u32 as u64), 0xbu64).1;
            // 33a9e setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 33aa1 movzx r10d,cl
            r10 = ((rcx as u8 as u64)) as u32 as u64;
            // 33aa5 inc r10d
            { let (r, f) = inc::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            pc = 0x33aa8;
        }
        0x33aa8 => {
            // block 33aa8
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aa8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aa8 cmp edi,0x2
            fl = sub::<32>((rdi as u32 as u64), 0x2u64).1;
            // 33aab je 0x33ac4
            if crate::cov::brk(0x33aab, fl.e()) { pc = 0x33ac4; continue; }
            pc = 0x33aad;
        }
        0x33aad => {
            // block 33aad
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aad cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // 33ab0 jne 0x33ad6
            if crate::cov::brk(0x33ab0, fl.ne()) { pc = 0x33ad6; continue; }
            pc = 0x33ab2;
        }
        0x33ab2 => {
            // block 33ab2
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ab2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ab4 lea rax,[rip+0xcc495] # 0xfff50
            rax = m.env.data_va(83741);
            // 33abb lea rcx,[rip+0xcc44e] # 0xfff10
            rcx = m.env.data_va(83677);
            // 33ac2 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ac4 => {
            // block 33ac4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ac4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ac4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ac6 lea rax,[rip+0xcc503] # 0xfffd0
            rax = m.env.data_va(83869);
            // 33acd lea rcx,[rip+0xcc4bc] # 0xfff90
            rcx = m.env.data_va(83805);
            // 33ad4 jmp 0x33aec
            pc = 0x33aec; continue;
        }
        0x33ad6 => {
            // block 33ad6
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33ad6 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 33ad8 test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // 33ada jne 0x33af4
            if crate::cov::brk(0x33ada, fl.ne()) { pc = 0x33af4; continue; }
            pc = 0x33adc;
        }
        0x33adc => {
            // block 33adc
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33adc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33adc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 33ade lea rax,[rip+0xcc3eb] # 0xffed0
            rax = m.env.data_va(83613);
            // 33ae5 lea rcx,[rip+0xcc3a4] # 0xffe90
            rcx = m.env.data_va(83549);
            pc = 0x33aec;
        }
        0x33aec => {
            // block 33aec
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33aec cmove rcx,rax
            if fl.e() { rcx = rax; }
            // 33af0 mov ecx,DWORD PTR [rcx+r9*4]
            rcx = (m.r32(rcx.wrapping_add(r9.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x33af4;
        }
        0x33af4 => {
            // block 33af4
            #[cfg(feature = "trace")] eprintln!("f_00033a14 pc 33af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 33af4 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 33af6 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 33af8 div r10d
            { let (q, r) = div::<32>(rdx, rax, (r10 as u32 as u64)); rax = q; rdx = r; }
            // 33afb add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // 33afd mov DWORD PTR [r8],eax
            m.w32(r8, ((rax as u32 as u64)) as u32);
            // 33b00 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 33b02 jmp 0x33a7d
            pc = 0x33a7d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2156 size=254
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002156<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2156;
    loop {
        match pc {
        0x2156 => {
            // block 2156
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2156 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2156 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2157 mov rbp,rsp
            rbp = rsp;
            // 215a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 215c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 215e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2160 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2161 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2165 mov r14,rdx
            r14 = rdx;
            // 2168 mov r15,rsi
            r15 = rsi;
            // 216b mov r12,rdi
            r12 = rdi;
            // 216e mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2172 call QWORD PTR [rax+0x418]
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2178 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 217a je 0x21c6
            if crate::cov::brk(0x217a, fl.e()) { pc = 0x21c6; continue; }
            pc = 0x217c;
        }
        0x217c => {
            // block 217c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 217c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 217c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2180 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 2184 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 2186 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 218b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2190 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2193 mov rdi,r12
            rdi = r12;
            // 2196 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2199 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 219b je 0x2244
            if crate::cov::brk(0x219b, fl.e()) { pc = 0x2244; continue; }
            pc = 0x21a1;
        }
        0x21a1 => {
            // block 21a1
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21a1 mov eax,0x300000
            rax = (0x300000u64) as u32 as u64;
            // 21a6 and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 21a9 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // 21ac cmp eax,0x300000
            fl = sub::<32>((rax as u32 as u64), 0x300000u64).1;
            // 21b1 je 0x21ca
            if crate::cov::brk(0x21b1, fl.e()) { pc = 0x21ca; continue; }
            pc = 0x21b3;
        }
        0x21b3 => {
            // block 21b3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21b3 cmp eax,0x200000
            fl = sub::<32>((rax as u32 as u64), 0x200000u64).1;
            // 21b8 jne 0x21d3
            if crate::cov::brk(0x21b8, fl.ne()) { pc = 0x21d3; continue; }
            pc = 0x21ba;
        }
        0x21ba => {
            // block 21ba
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ba mov eax,0x800000
            rax = (0x800000u64) as u32 as u64;
            // 21bf mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 21c4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21c6 => {
            // block 21c6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21c6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21c8 jmp 0x2244
            pc = 0x2244; continue;
        }
        0x21ca => {
            // block 21ca
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21ca mov eax,0x1000000
            rax = (0x1000000u64) as u32 as u64;
            // 21cf xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21d1 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21d3 => {
            // block 21d3
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21d3 cmp eax,0x100000
            fl = sub::<32>((rax as u32 as u64), 0x100000u64).1;
            // 21d8 jne 0x21e6
            if crate::cov::brk(0x21d8, fl.ne()) { pc = 0x21e6; continue; }
            pc = 0x21da;
        }
        0x21da => {
            // block 21da
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21da mov eax,0x400000
            rax = (0x400000u64) as u32 as u64;
            // 21df mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 21e4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21e6 => {
            // block 21e6
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21e6 mov eax,0x200000
            rax = (0x200000u64) as u32 as u64;
            // 21eb mov ebx,0x4
            rbx = (0x4u64) as u32 as u64;
            pc = 0x21f0;
        }
        0x21f0 => {
            // block 21f0
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 21f3 je 0x21f8
            if crate::cov::brk(0x21f3, fl.e()) { pc = 0x21f8; continue; }
            pc = 0x21f5;
        }
        0x21f5 => {
            // block 21f5
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f5 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            pc = 0x21f8;
        }
        0x21f8 => {
            // block 21f8
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21f8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 21fb je 0x2215
            if crate::cov::brk(0x21fb, fl.e()) { pc = 0x2215; continue; }
            pc = 0x21fd;
        }
        0x21fd => {
            // block 21fd
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 21fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 21fd mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2202 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2207 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2209 je 0x2210
            if crate::cov::brk(0x2209, fl.e()) { pc = 0x2210; continue; }
            pc = 0x220b;
        }
        0x220b => {
            // block 220b
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 220b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 220b imul eax,ebx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rbx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 220e jmp 0x2212
            pc = 0x2212; continue;
        }
        0x2210 => {
            // block 2210
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2210 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x2212;
        }
        0x2212 => {
            // block 2212
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2212 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2212 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x2215;
        }
        0x2215 => {
            // block 2215
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2215 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2215 mov DWORD PTR [rbp-0x28],0x12
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x12u64) as u32);
            // 221c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 2220 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2224 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2226 mov rdi,r12
            rdi = r12;
            // 2229 call QWORD PTR [rax+0x388]
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 222f mov eax,DWORD PTR [rbp-0x28]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // 2232 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2235 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // 2237 cmp eax,0x10
            fl = sub::<32>((rax as u32 as u64), 0x10u64).1;
            // 223a jne 0x2244
            if crate::cov::brk(0x223a, fl.ne()) { pc = 0x2244; continue; }
            pc = 0x223c;
        }
        0x223c => {
            // block 223c
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 223c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 223c shl DWORD PTR [r15],0x2
            { let (r, f) = shl::<32>(m.r32(r15) as u64, 0x2u64, fl); m.w32(r15, (r) as u32); fl = f; }
            // 2240 shr DWORD PTR [r14],0x2
            { let (r, f) = shr::<32>(m.r32(r14) as u64, 0x2u64, fl); m.w32(r14, (r) as u32); fl = f; }
            pc = 0x2244;
        }
        0x2244 => {
            // block 2244
            #[cfg(feature = "trace")] eprintln!("f_00002156 pc 2244 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2244 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 2246 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 224a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 224b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 224d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 224f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2251 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2252 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x593a3 size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000593a3<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x593a3;
    loop {
        match pc {
        0x593a3 => {
            // block 593a3
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593a3 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 593a4 mov rbp,rsp
            rbp = rsp;
            // 593a7 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 593a9 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 593ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 593ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 593ae sub rsp,0x2050
            { let (r, f) = sub::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 593b5 mov r14,rcx
            r14 = rcx;
            // 593b8 mov r12,rdi
            r12 = rdi;
            // 593bb mov al,BYTE PTR [rbp+0x10]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0x10)) as u64) as u8 as u64);
            // 593be mov r15,QWORD PTR [rip+0xe7013] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // 593c5 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // 593c8 mov QWORD PTR [rbp-0x28],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (r15) as u64);
            // 593cc movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 593cf movzx ecx,r8b
            rcx = ((r8 as u8 as u64)) as u32 as u64;
            // 593d3 movzx r8d,r9b
            r8 = ((r9 as u8 as u64)) as u32 as u64;
            // 593d7 lea rdi,[rbp-0x2070]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_df90);
            // 593de mov r9d,eax
            r9 = ((rax as u32 as u64)) as u32 as u64;
            // 593e1 call 0x32c5c
            { let r = { let t = m.env.code_va(0x32c5c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 593e6 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 593ec test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 593ef je 0x5943b
            if crate::cov::brk(0x593ef, fl.e()) { pc = 0x5943b; continue; }
            pc = 0x593f1;
        }
        0x593f1 => {
            // block 593f1
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 593f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 593f1 movabs rax,0xffffffffffff
            rax = 0x0_ffff_ffff_ffffu64;
            // 593fb and rax,r14
            { let r = rax & r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 593fe mov rcx,rax
            rcx = rax;
            // 59401 shr rcx,0x10
            { let (r, f) = shr::<64>(rcx, 0x10u64, fl); rcx = r; fl = f; }
            // 59405 shr rax,0x20
            { let (r, f) = shr::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // 59409 movzx esi,dx
            rsi = ((rdx as u16 as u64)) as u32 as u64;
            // 5940c add rsi,rsi
            { let (r, f) = add::<64>(rsi, rsi);
              rsi = r;
              fl = f; }
            // 5940f xor edi,edi
            { let r = (rdi as u32 as u64) ^ (rdi as u32 as u64); let f = logic::<32>(r);
              rdi = (r) as u32 as u64;
              fl = f; }
            pc = 0x59411;
        }
        0x59411 => {
            // block 59411
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59411 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59411 mov WORD PTR [rbp+rdi*2-0x2030],r14w
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd0), ((r14 as u16 as u64)) as u16);
            // 5941a mov WORD PTR [rbp+rdi*2-0x202e],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd2), ((rcx as u16 as u64)) as u16);
            // 59422 mov WORD PTR [rbp+rdi*2-0x202c],ax
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd4), ((rax as u16 as u64)) as u16);
            // 5942a mov WORD PTR [rbp+rdi*2-0x202a],cx
            m.w16(rbp.wrapping_add(rdi.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_dfd6), ((rcx as u16 as u64)) as u16);
            // 59432 add rdi,0x4
            { let (r, f) = add::<64>(rdi, 0x4u64);
              rdi = r;
              fl = f; }
            // 59436 cmp rdi,rsi
            fl = sub::<64>(rdi, rsi).1;
            // 59439 jl 0x59411
            if crate::cov::brk(0x59439, fl.l()) { pc = 0x59411; continue; }
            pc = 0x5943b;
        }
        0x5943b => {
            // block 5943b
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5943b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5943b cmp DWORD PTR [rbp-0x2070],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x0u64).1;
            // 59442 je 0x59490
            if crate::cov::brk(0x59442, fl.e()) { pc = 0x59490; continue; }
            pc = 0x59444;
        }
        0x59444 => {
            // block 59444
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59444 lea rdi,[rbp-0x2030]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            // 5944b mov rsi,r12
            rsi = r12;
            // 5944e call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 59453 cmp DWORD PTR [rbp-0x2070],0x2
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64, 0x2u64).1;
            // 5945a jb 0x59490
            if crate::cov::brk(0x5945a, fl.b()) { pc = 0x59490; continue; }
            pc = 0x5945c;
        }
        0x5945c => {
            // block 5945c
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 5945c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5945c mov bx,0x2
            rbx = (rbx & !0xffff) | ((0x2u64) as u16 as u64);
            // 59460 lea r14,[rbp-0x2030]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_dfd0);
            pc = 0x59467;
        }
        0x59467 => {
            // block 59467
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59467 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59467 mov edx,DWORD PTR [rbp-0x206c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df94)) as u64) as u32 as u64;
            // 5946d mov eax,DWORD PTR [rbp-0x2068]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df98)) as u64) as u32 as u64;
            // 59473 lea r12,[r12+rax*4]
            r12 = r12.wrapping_add(rax.wrapping_mul(4));
            // 59477 mov rdi,r14
            rdi = r14;
            // 5947a mov rsi,r12
            rsi = r12;
            // 5947d call 0x59033
            { let r = { let t = m.env.code_va(0x59033); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 59482 movzx eax,bx
            rax = ((rbx as u16 as u64)) as u32 as u64;
            // 59485 lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 59488 cmp eax,DWORD PTR [rbp-0x2070]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_df90)) as u64).1;
            // 5948e jb 0x59467
            if crate::cov::brk(0x5948e, fl.b()) { pc = 0x59467; continue; }
            pc = 0x59490;
        }
        0x59490 => {
            // block 59490
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59490 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59490 cmp r15,QWORD PTR [rbp-0x28]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64).1;
            // 59494 jne 0x594a6
            if crate::cov::brk(0x59494, fl.ne()) { pc = 0x594a6; continue; }
            pc = 0x59496;
        }
        0x59496 => {
            // block 59496
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 59496 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59496 add rsp,0x2050
            { let (r, f) = add::<64>(rsp, 0x2050u64);
              rsp = r;
              fl = f; }
            // 5949d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 5949e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 594a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 594a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 594a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 594a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x594a6 => {
            // block 594a6
            #[cfg(feature = "trace")] eprintln!("f_000593a3 pc 594a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 594a6 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
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
            if crate::cov::brk(0x15e36, fl.ns()) { pc = 0x15e44; continue; }
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
            if crate::cov::brk(0x15e59, fl.ae()) { pc = 0x15e87; continue; }
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
            if crate::cov::brk(0x15e83, fl.ne()) { pc = 0x15ea4; continue; }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
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
            if crate::cov::brk(0x15ef4, fl.e()) { pc = 0x15f0c; continue; }
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

/// parity: P2v @0x12824 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00012824<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x12824;
    loop {
        match pc {
        0x12824 => {
            // block 12824
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12824 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12824 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 12825 mov rbp,rsp
            rbp = rsp;
            // 12828 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1282a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1282c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1282e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 12830 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 12831 sub rsp,0x88
            { let (r, f) = sub::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12838 mov r14,rdx
            r14 = rdx;
            // 1283b mov r15,rsi
            r15 = rsi;
            // 1283e mov r13,rdi
            r13 = rdi;
            // 12841 lea r12,[rbp-0x60]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 12845 mov rbx,r15
            rbx = r15;
            // 12848 jmp 0x12851
            pc = 0x12851; continue;
        }
        0x1284a => {
            // block 1284a
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1284a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1284a cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 1284d cmove r15,rbx
            if fl.e() { r15 = rbx; }
            pc = 0x12851;
        }
        0x12851 => {
            // block 12851
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12851 mov rdi,r12
            rdi = r12;
            // 12854 mov rsi,r13
            rsi = r13;
            // 12857 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1285c mov rdi,r13
            rdi = r13;
            // 1285f mov rsi,rbx
            rsi = rbx;
            // 12862 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 12867 mov rdi,rbx
            rdi = rbx;
            // 1286a mov rsi,r12
            rsi = r12;
            // 1286d call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 12872 mov rdi,r12
            rdi = r12;
            // 12875 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1287a add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // 1287e add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 12882 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 12885 jne 0x1284a
            if crate::cov::brk(0x12885, fl.ne()) { pc = 0x1284a; continue; }
            pc = 0x12887;
        }
        0x12887 => {
            // block 12887
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12887 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12887 mov QWORD PTR [rbp-0xa0],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (r14) as u64);
            // 1288e cmp r13,r15
            fl = sub::<64>(r13, r15).1;
            // 12891 je 0x128fe
            if crate::cov::brk(0x12891, fl.e()) { pc = 0x128fe; continue; }
            pc = 0x12893;
        }
        0x12893 => {
            // block 12893
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12893 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12893 lea r14,[rbp-0x98]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 1289a mov rbx,r13
            rbx = r13;
            // 1289d mov r12,r15
            r12 = r15;
            // 128a0 jmp 0x128b1
            pc = 0x128b1; continue;
        }
        0x128a2 => {
            // block 128a2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128a2 cmp QWORD PTR [rbp-0xa8],rbx
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, rbx).1;
            // 128a9 cmove r15,r12
            if fl.e() { r15 = r12; }
            // 128ad add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            pc = 0x128b1;
        }
        0x128b1 => {
            // block 128b1
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128b1 lea rax,[r15-0x38]
            rax = r15.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 128b5 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            pc = 0x128bc;
        }
        0x128bc => {
            // block 128bc
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128bc mov rdi,r14
            rdi = r14;
            // 128bf mov rsi,rbx
            rsi = rbx;
            // 128c2 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 128c7 mov rdi,rbx
            rdi = rbx;
            // 128ca mov rsi,r12
            rsi = r12;
            // 128cd call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 128d2 mov rdi,r12
            rdi = r12;
            // 128d5 mov rsi,r14
            rsi = r14;
            // 128d8 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 128dd mov rdi,r14
            rdi = r14;
            // 128e0 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 128e5 add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            // 128e9 cmp r12,QWORD PTR [rbp-0xa0]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64).1;
            // 128f0 jne 0x128a2
            if crate::cov::brk(0x128f0, fl.ne()) { pc = 0x128a2; continue; }
            pc = 0x128f2;
        }
        0x128f2 => {
            // block 128f2
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128f2 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // 128f6 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 128f9 mov r12,r15
            r12 = r15;
            // 128fc jne 0x128bc
            if crate::cov::brk(0x128fc, fl.ne()) { pc = 0x128bc; continue; }
            pc = 0x128fe;
        }
        0x128fe => {
            // block 128fe
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 128fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 128fe mov rax,r13
            rax = r13;
            // 12901 add rsp,0x88
            { let (r, f) = add::<64>(rsp, 0x88u64);
              rsp = r;
              fl = f; }
            // 12908 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 12909 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1290b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1290d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1290f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 12911 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 12912 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x12913 => {
            // block 12913
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12913 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12913 mov rbx,rax
            rbx = rax;
            // 12916 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1291a jmp 0x12926
            pc = 0x12926; continue;
        }
        0x1291c => {
            // block 1291c
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 1291c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1291c mov rbx,rax
            rbx = rax;
            // 1291f lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            pc = 0x12926;
        }
        0x12926 => {
            // block 12926
            #[cfg(feature = "trace")] eprintln!("f_00012824 pc 12926 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 12926 call 0xe05fa
            { let r = { let t = m.env.code_va(0xe05fa); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 1292b mov rdi,rbx
            rdi = rbx;
            // 1292e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2893b size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002893b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2893b;
    loop {
        match pc {
        0x2893b => {
            // block 2893b
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 2893b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2893b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2893c mov rbp,rsp
            rbp = rsp;
            // 2893f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28941 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28943 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 28945 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 28947 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28948 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 2894c mov r12,rsi
            r12 = rsi;
            // 2894f mov r14,rdi
            r14 = rdi;
            // 28952 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 28956 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 2895b call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28960 cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 28966 lea rcx,[rip+0x101ea8] # 0x12a815
            rcx = m.env.data_va(258018);
            // 2896d lea rsi,[rip+0x101eae] # 0x12a822
            rsi = m.env.data_va(258031);
            // 28974 cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 28978 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 2897b movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 2897e or rdx,0xc
            { let r = rdx | 0xcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 28982 mov rdi,rax
            rdi = rax;
            // 28985 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2898a add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 2898e cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28991 je 0x28a29
            if crate::cov::brk(0x28991, fl.e()) { pc = 0x28a29; continue; }
            pc = 0x28997;
        }
        0x28997 => {
            // block 28997
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28997 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2899b lea r13,[rip+0x101afb] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x289a2;
        }
        0x289a2 => {
            // block 289a2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289a2 mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 289a5 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 289a7 mov rdi,r15
            rdi = r15;
            // 289aa call 0x5b5c9
            { let r = { let t = m.env.code_va(0x5b5c9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 289af movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 289b3 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 289b6 jne 0x289c1
            if crate::cov::brk(0x289b6, fl.ne()) { pc = 0x289c1; continue; }
            pc = 0x289b8;
        }
        0x289b8 => {
            // block 289b8
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289b8 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 289bb lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 289bf jmp 0x289c9
            pc = 0x289c9; continue;
        }
        0x289c1 => {
            // block 289c1
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289c1 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 289c5 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x289c9;
        }
        0x289c9 => {
            // block 289c9
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289c9 mov rdi,r14
            rdi = r14;
            // 289cc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 289d1 mov rdi,r15
            rdi = r15;
            // 289d4 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 289d9 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 289dd test rax,rax
            fl = logic::<64>(rax & rax);
            // 289e0 je 0x289ef
            if crate::cov::brk(0x289e0, fl.e()) { pc = 0x289ef; continue; }
            pc = 0x289e2;
        }
        0x289e2 => {
            // block 289e2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289e2 mov rbx,rax
            rbx = rax;
            // 289e5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 289e8 test rax,rax
            fl = logic::<64>(rax & rax);
            // 289eb jne 0x289e2
            if crate::cov::brk(0x289eb, fl.ne()) { pc = 0x289e2; continue; }
            pc = 0x289ed;
        }
        0x289ed => {
            // block 289ed
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289ed jmp 0x289fe
            pc = 0x289fe; continue;
        }
        0x289ef => {
            // block 289ef
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289ef mov rax,rbx
            rax = rbx;
            pc = 0x289f2;
        }
        0x289f2 => {
            // block 289f2
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289f2 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 289f6 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 289f9 mov rax,rbx
            rax = rbx;
            // 289fc jne 0x289f2
            if crate::cov::brk(0x289fc, fl.ne()) { pc = 0x289f2; continue; }
            pc = 0x289fe;
        }
        0x289fe => {
            // block 289fe
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 289fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 289fe cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28a01 mov rsi,r13
            rsi = r13;
            // 28a04 lea rax,[rip+0x1102bd] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28a0b cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 28a0f setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 28a12 movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28a15 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28a18 mov rdi,r14
            rdi = r14;
            // 28a1b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28a20 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 28a23 jne 0x289a2
            if crate::cov::brk(0x28a23, fl.ne()) { pc = 0x289a2; continue; }
            pc = 0x28a29;
        }
        0x28a29 => {
            // block 28a29
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28a29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a29 mov rax,r14
            rax = r14;
            // 28a2c add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 28a30 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28a31 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 28a33 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 28a35 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28a37 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28a39 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28a3a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x28a3b => {
            // block 28a3b
            #[cfg(feature = "trace")] eprintln!("f_0002893b pc 28a3b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28a3b mov rbx,rax
            rbx = rax;
            // 28a3e lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28a42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28a47 mov rdi,rbx
            rdi = rbx;
            // 28a4a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x286b5 size=277
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000286b5<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x286b5;
    loop {
        match pc {
        0x286b5 => {
            // block 286b5
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 286b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 286b5 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 286b6 mov rbp,rsp
            rbp = rsp;
            // 286b9 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 286bb push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 286bd push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 286bf push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 286c1 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 286c2 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 286c6 mov r12,rsi
            r12 = rsi;
            // 286c9 mov r14,rdi
            r14 = rdi;
            // 286cc mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 286d0 mov rsi,QWORD PTR [r12+0x10]
            rsi = m.r64(r12.wrapping_add(0x10)) as u64;
            // 286d5 call 0xec0a4
            { let r = { let t = m.env.stub_va(0x871); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 286da cmp QWORD PTR [r12+0x10],0x1
            fl = sub::<64>(m.r64(r12.wrapping_add(0x10)) as u64, 0x1u64).1;
            // 286e0 lea rcx,[rip+0x1020e6] # 0x12a7cd
            rcx = m.env.data_va(257946);
            // 286e7 lea rsi,[rip+0x1020f0] # 0x12a7de
            rsi = m.env.data_va(257963);
            // 286ee cmove rsi,rcx
            if fl.e() { rsi = rcx; }
            // 286f2 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 286f5 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 286f8 lea rdx,[rcx+rcx*2+0x10]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x10);
            // 286fd mov rdi,rax
            rdi = rax;
            // 28700 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28705 add r12,0x8
            { let (r, f) = add::<64>(r12, 0x8u64);
              r12 = r;
              fl = f; }
            // 28709 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2870c je 0x287a4
            if crate::cov::brk(0x2870c, fl.e()) { pc = 0x287a4; continue; }
            pc = 0x28712;
        }
        0x28712 => {
            // block 28712
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28712 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28712 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28716 lea r13,[rip+0x101d80] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x2871d;
        }
        0x2871d => {
            // block 2871d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2871d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2871d mov esi,DWORD PTR [rbx+0x1c]
            rsi = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 28720 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 28722 mov rdi,r15
            rdi = r15;
            // 28725 call 0x61a4c
            { let r = { let t = m.env.code_va(0x61a4c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2872a movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 2872e test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 28731 jne 0x2873c
            if crate::cov::brk(0x28731, fl.ne()) { pc = 0x2873c; continue; }
            pc = 0x28733;
        }
        0x28733 => {
            // block 28733
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28733 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 28736 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 2873a jmp 0x28744
            pc = 0x28744; continue;
        }
        0x2873c => {
            // block 2873c
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2873c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2873c mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28740 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28744;
        }
        0x28744 => {
            // block 28744
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28744 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28744 mov rdi,r14
            rdi = r14;
            // 28747 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2874c mov rdi,r15
            rdi = r15;
            // 2874f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 28754 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28758 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2875b je 0x2876a
            if crate::cov::brk(0x2875b, fl.e()) { pc = 0x2876a; continue; }
            pc = 0x2875d;
        }
        0x2875d => {
            // block 2875d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2875d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2875d mov rbx,rax
            rbx = rax;
            // 28760 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28763 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28766 jne 0x2875d
            if crate::cov::brk(0x28766, fl.ne()) { pc = 0x2875d; continue; }
            pc = 0x28768;
        }
        0x28768 => {
            // block 28768
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28768 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28768 jmp 0x28779
            pc = 0x28779; continue;
        }
        0x2876a => {
            // block 2876a
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2876a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2876a mov rax,rbx
            rax = rbx;
            pc = 0x2876d;
        }
        0x2876d => {
            // block 2876d
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 2876d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2876d mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28771 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28774 mov rax,rbx
            rax = rbx;
            // 28777 jne 0x2876d
            if crate::cov::brk(0x28777, fl.ne()) { pc = 0x2876d; continue; }
            pc = 0x28779;
        }
        0x28779 => {
            // block 28779
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 28779 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28779 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2877c mov rsi,r13
            rsi = r13;
            // 2877f lea rax,[rip+0x110542] # 0x138cc8
            rax = m.env.data_va(316565);
            // 28786 cmove rsi,rax
            if fl.e() { rsi = rax; }
            // 2878a setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 2878d movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // 28790 add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 28793 mov rdi,r14
            rdi = r14;
            // 28796 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 2879b cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2879e jne 0x2871d
            if crate::cov::brk(0x2879e, fl.ne()) { pc = 0x2871d; continue; }
            pc = 0x287a4;
        }
        0x287a4 => {
            // block 287a4
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 287a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 287a4 mov rax,r14
            rax = r14;
            // 287a7 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 287ab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 287ac pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 287ae pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 287b0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 287b2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 287b4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 287b5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x287b6 => {
            // block 287b6
            #[cfg(feature = "trace")] eprintln!("f_000286b5 pc 287b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 287b6 mov rbx,rax
            rbx = rax;
            // 287b9 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 287bd call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 287c2 mov rdi,rbx
            rdi = rbx;
            // 287c5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb608 size=280
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000eb608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb608;
    loop {
        match pc {
        0xeb608 => {
            // block eb608
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb608 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb609 mov rbp,rsp
            rbp = rsp;
            // eb60c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb60e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb610 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb612 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb614 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb615 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb619 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // eb61d mov r15,rdi
            r15 = rdi;
            // eb620 movabs r12,0x492492492492492
            r12 = 0x492_4924_9249_2492u64;
            // eb62a mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // eb62d mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb631 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // eb634 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // eb638 movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // eb642 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // eb646 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // eb649 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // eb64c jbe 0xeb659
            if crate::cov::brk(0xeb64c, fl.be()) { pc = 0xeb659; continue; }
            pc = 0xeb64e;
        }
        0xeb64e => {
            // block eb64e
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb64e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb64e mov rdi,r15
            rdi = r15;
            // eb651 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0xeb659 => {
            // block eb659
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb659 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb659 lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // eb65d mov rdx,QWORD PTR [r15+0x10]
            rdx = m.r64(r15.wrapping_add(0x10)) as u64;
            // eb661 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // eb664 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // eb668 imul rdx,r13
            { let (r, f) = imul::<64>(rdx, r13); rdx = r; fl = f; }
            // eb66c movabs rsi,0x249249249249248
            rsi = 0x249_2492_4924_9248u64;
            // eb676 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // eb679 ja 0xeb688
            if crate::cov::brk(0xeb679, fl.a()) { pc = 0xeb688; continue; }
            pc = 0xeb67b;
        }
        0xeb67b => {
            // block eb67b
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb67b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb67b add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // eb67e cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // eb681 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // eb685 mov r12,rdx
            r12 = rdx;
            pc = 0xeb688;
        }
        0xeb688 => {
            // block eb688
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb688 mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // eb68c sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // eb68f sar r14,0x3
            { let (r, f) = sar::<64>(r14, 0x3u64, fl); r14 = r; fl = f; }
            // eb693 imul r14,r13
            { let (r, f) = imul::<64>(r14, r13); r14 = r; fl = f; }
            // eb697 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // eb69f mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // eb6a3 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // eb6a5 test r12,r12
            fl = logic::<64>(r12 & r12);
            // eb6a8 je 0xeb6b6
            if crate::cov::brk(0xeb6a8, fl.e()) { pc = 0xeb6b6; continue; }
            pc = 0xeb6aa;
        }
        0xeb6aa => {
            // block eb6aa
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb6aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb6aa imul rdi,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rdi = r; fl = f; }
            // eb6ae call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eb6b3 mov rbx,rax
            rbx = rax;
            pc = 0xeb6b6;
        }
        0xeb6b6 => {
            // block eb6b6
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb6b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb6b6 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // eb6ba imul r14,r14,0x38
            { let (r, f) = imul::<64>(r14, 0x38u64); r14 = r; fl = f; }
            // eb6be lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // eb6c2 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // eb6c6 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // eb6ca imul rax,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rax = r; fl = f; }
            // eb6ce add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // eb6d1 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // eb6d5 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // eb6d9 call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eb6de lea rax,[r14+rbx*1+0x38]
            rax = r14.wrapping_add(rbx).wrapping_add(0x38);
            // eb6e3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // eb6e7 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6eb mov rdi,r15
            rdi = r15;
            // eb6ee call 0xebf30
            { let r = { let t = m.env.stub_va(0x6fd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eb6f3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb6f7 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eb6fc add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // eb700 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb701 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb703 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb705 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb707 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb709 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb70a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xeb70b => {
            // block eb70b
            #[cfg(feature = "trace")] eprintln!("f_000eb608 pc eb70b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // eb70b mov rbx,rax
            rbx = rax;
            // eb70e lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // eb712 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // eb717 mov rdi,rbx
            rdi = rbx;
            // eb71a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2346 size=288
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2346<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2346;
    loop {
        match pc {
        0xe2346 => {
            // block e2346
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2346 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2346 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2347 mov rbp,rsp
            rbp = rsp;
            // e234a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e234c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e234e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e2350 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2352 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2353 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2357 mov r13,r9
            r13 = r9;
            // e235a mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e235d mov DWORD PTR [rbp-0x6c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rcx as u32 as u64)) as u32);
            // e2360 mov DWORD PTR [rbp-0x5c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rdx as u32 as u64)) as u32);
            // e2363 mov QWORD PTR [rbp-0x68],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rsi) as u64);
            // e2367 mov r15,rdi
            r15 = rdi;
            // e236a call 0xe49c4
            { let r = { let t = m.env.code_va(0xe49c4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e236f mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2371 test al,0x4
            fl = logic::<8>((rax as u8 as u64) & 0x4u64);
            // e2373 je 0xe242c
            if crate::cov::brk(0xe2373, fl.e()) { pc = 0xe242c; continue; }
            pc = 0xe2379;
        }
        0xe2379 => {
            // block e2379
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2379 mov rdi,r15
            rdi = r15;
            // e237c call 0xe494e
            { let r = { let t = m.env.code_va(0xe494e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2381 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2383 je 0xe242a
            if crate::cov::brk(0xe2383, fl.e()) { pc = 0xe242a; continue; }
            pc = 0xe2389;
        }
        0xe2389 => {
            // block e2389
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2389 mov DWORD PTR [rbp-0x70],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rbx as u32 as u64)) as u32);
            // e238c mov r12d,0xffffffe2
            r12 = (0xffffffe2u64) as u32 as u64;
            // e2392 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2396 lea r14,[rbp-0x58]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe239a;
        }
        0xe239a => {
            // block e239a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e239a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e239a mov rdi,rbx
            rdi = rbx;
            // e239d mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23a1 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23a6 mov rdi,r15
            rdi = r15;
            // e23a9 mov rsi,rbx
            rsi = rbx;
            // e23ac mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23af call 0xe2466
            { let r = { let t = m.env.code_va(0xe2466); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23b4 mov rdi,rbx
            rdi = rbx;
            // e23b7 mov bl,al
            rbx = (rbx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e23b9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23be test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e23c0 je 0xe242a
            if crate::cov::brk(0xe23c0, fl.e()) { pc = 0xe242a; continue; }
            pc = 0xe23c2;
        }
        0xe23c2 => {
            // block e23c2
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e23c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e23c2 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e23c5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e23c8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e23ca mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // e23cf call QWORD PTR [rax+0xbd0]
            { let r = { let t = m.r64(rax.wrapping_add(0xbd0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23d5 mov rdi,r14
            rdi = r14;
            // e23d8 mov rsi,QWORD PTR [rbp-0x68]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // e23dc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23e1 mov rdi,r15
            rdi = r15;
            // e23e4 mov rsi,r14
            rsi = r14;
            // e23e7 mov edx,DWORD PTR [rbp-0x5c]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // e23ea mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // e23ed mov r8d,DWORD PTR [rbp-0x70]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // e23f1 mov r9,r13
            r9 = r13;
            // e23f4 mov rbx,r13
            rbx = r13;
            // e23f7 call 0xe26e8
            { let r = { let t = m.env.code_va(0xe26e8); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e23fc mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // e23ff mov rdi,r14
            rdi = r14;
            // e2402 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2407 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // e240a je 0xe2422
            if crate::cov::brk(0xe240a, fl.e()) { pc = 0xe2422; continue; }
            pc = 0xe240c;
        }
        0xe240c => {
            // block e240c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e240c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e240c cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // e2410 je 0xe243d
            if crate::cov::brk(0xe2410, fl.e()) { pc = 0xe243d; continue; }
            pc = 0xe2412;
        }
        0xe2412 => {
            // block e2412
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2412 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2412 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // e2415 mov r13,rbx
            r13 = rbx;
            // e2418 lea rbx,[rbp-0x40]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e241c jne 0xe239a
            if crate::cov::brk(0xe241c, fl.ne()) { pc = 0xe239a; continue; }
            pc = 0xe2422;
        }
        0xe2422 => {
            // block e2422
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2422 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2422 mov rdi,r15
            rdi = r15;
            // e2425 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0xe242a;
        }
        0xe242a => {
            // block e242a
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242a xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe242c;
        }
        0xe242c => {
            // block e242c
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e242c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e242c mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // e242e add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // e2432 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2433 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2435 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e2437 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2439 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e243b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e243c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe243d => {
            // block e243d
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e243d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e243d mov rdi,r15
            rdi = r15;
            // e2440 call 0xe49f0
            { let r = { let t = m.env.code_va(0xe49f0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e2445 mov cl,0x1
            rcx = (rcx & !0xff) | ((0x1u64) as u8 as u64);
            // e2447 jmp 0xe242c
            pc = 0xe242c; continue;
        }
        0xe2449 => {
            // block e2449
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2449 mov rbx,rax
            rbx = rax;
            // e244c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e2450 jmp 0xe2459
            pc = 0xe2459; continue;
        }
        0xe2452 => {
            // block e2452
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2452 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2452 mov rbx,rax
            rbx = rax;
            // e2455 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            pc = 0xe2459;
        }
        0xe2459 => {
            // block e2459
            #[cfg(feature = "trace")] eprintln!("f_000e2346 pc e2459 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2459 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // e245e mov rdi,rbx
            rdi = rbx;
            // e2461 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x72d00 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00072d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x72d00;
    loop {
        match pc {
        0x72d00 => {
            // block 72d00
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 72d01 mov rbp,rsp
            rbp = rsp;
            // 72d04 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 72d06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 72d08 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 72d0a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 72d0c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 72d0d sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 72d11 mov rbx,rdx
            rbx = rdx;
            // 72d14 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 72d17 mov r15,rdi
            r15 = rdi;
            // 72d1a mov DWORD PTR [rbp-0x2c],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((r14 as u32 as u64)) as u32);
            // 72d1e movzx eax,BYTE PTR [rbx]
            rax = (m.r8(rbx) as u64) as u32 as u64;
            // 72d21 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 72d23 jne 0x72d2a
            if crate::cov::brk(0x72d23, fl.ne()) { pc = 0x72d2a; continue; }
            pc = 0x72d25;
        }
        0x72d25 => {
            // block 72d25
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d25 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 72d28 jmp 0x72d2e
            pc = 0x72d2e; continue;
        }
        0x72d2a => {
            // block 72d2a
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d2a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            pc = 0x72d2e;
        }
        0x72d2e => {
            // block 72d2e
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d2e test rax,rax
            fl = logic::<64>(rax & rax);
            // 72d31 je 0x72dec
            if crate::cov::brk(0x72d31, fl.e()) { pc = 0x72dec; continue; }
            pc = 0x72d37;
        }
        0x72d37 => {
            // block 72d37
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d37 lea r12,[r15+0x168]
            r12 = r15.wrapping_add(0x168);
            // 72d3e lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 72d42 mov rdi,r12
            rdi = r12;
            // 72d45 call 0xebc66
            { let r = { let t = m.env.stub_va(0x433); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72d4a lea rcx,[r15+0x170]
            rcx = r15.wrapping_add(0x170);
            // 72d51 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 72d54 jne 0x72dec
            if crate::cov::brk(0x72d54, fl.ne()) { pc = 0x72dec; continue; }
            pc = 0x72d5a;
        }
        0x72d5a => {
            // block 72d5a
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72d5a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72d5a mov DWORD PTR [rbp-0x50],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((r14 as u32 as u64)) as u32);
            // 72d5e lea r13,[rbp-0x48]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 72d62 mov rdi,r13
            rdi = r13;
            // 72d65 mov rsi,rbx
            rsi = rbx;
            // 72d68 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72d6d lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 72d71 mov rdi,r12
            rdi = r12;
            // 72d74 call 0xebc54
            { let r = { let t = m.env.stub_va(0x421); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72d79 mov rdi,r13
            rdi = r13;
            // 72d7c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72d81 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 72d88 mov rsi,rbx
            rsi = rbx;
            // 72d8b call 0xb5094
            { let r = { let t = m.env.code_va(0xb5094); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72d90 mov rax,QWORD PTR [rbp-0x80]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 72d94 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 72d98 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 72d9f mov rcx,QWORD PTR [rbp-0x88]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 72da6 mov QWORD PTR [rbp-0x68],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rcx) as u64);
            // 72daa mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 72dae xorps xmm0,xmm0
            x0 = 0;
            // 72db1 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // 72db8 mov QWORD PTR [rbp-0x80],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (0x0u64) as u64);
            // 72dc0 mov DWORD PTR [rbp-0x58],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((r14 as u32 as u64)) as u32);
            // 72dc4 add r15,0x1b0
            { let (r, f) = add::<64>(r15, 0x1b0u64);
              r15 = r;
              fl = f; }
            // 72dcb lea rsi,[rbp-0x70]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72dcf mov rdi,r15
            rdi = r15;
            // 72dd2 call 0xebc24
            { let r = { let t = m.env.stub_va(0x3f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72dd7 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72ddb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72de0 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 72de7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x72dec;
        }
        0x72dec => {
            // block 72dec
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72dec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72dec add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 72df0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 72df1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 72df3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 72df5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 72df7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 72df9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 72dfa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x72dfb => {
            // block 72dfb
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72dfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72dfb mov rbx,rax
            rbx = rax;
            // 72dfe mov rdi,r13
            rdi = r13;
            // 72e01 jmp 0x72e16
            pc = 0x72e16; continue;
        }
        0x72e03 => {
            // block 72e03
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72e03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72e03 mov rbx,rax
            rbx = rax;
            // 72e06 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // 72e0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72e0f lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x72e16;
        }
        0x72e16 => {
            // block 72e16
            #[cfg(feature = "trace")] eprintln!("f_00072d00 pc 72e16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 72e16 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 72e1b mov rdi,rbx
            rdi = rbx;
            // 72e1e call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19dea size=294
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00019dea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19dea;
    loop {
        match pc {
        0x19dea => {
            // block 19dea
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19dea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19deb mov rbp,rsp
            rbp = rsp;
            // 19dee push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 19df0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 19df2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 19df4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 19df6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 19df7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 19df8 mov r12,rdx
            r12 = rdx;
            // 19dfb mov r15,rdi
            r15 = rdi;
            // 19dfe mov r14,r12
            r14 = r12;
            // 19e01 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 19e04 mov rbx,r14
            rbx = r14;
            // 19e07 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19e0b mov r13,QWORD PTR [r15]
            r13 = m.r64(r15) as u64;
            // 19e0e mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e12 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e15 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e19 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e1c jbe 0x19e63
            if crate::cov::brk(0x19e1c, fl.be()) { pc = 0x19e63; continue; }
            pc = 0x19e1e;
        }
        0x19e1e => {
            // block 19e1e
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e1e mov r12,rsi
            r12 = rsi;
            // 19e21 mov rdi,r15
            rdi = r15;
            // 19e24 call 0xeba2c
            { let r = { let t = m.env.stub_va(0x1f9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19e29 mov rax,rbx
            rax = rbx;
            // 19e2c shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 19e30 je 0x19e3a
            if crate::cov::brk(0x19e30, fl.e()) { pc = 0x19e3a; continue; }
            pc = 0x19e32;
        }
        0x19e32 => {
            // block 19e32
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e32 mov rdi,r15
            rdi = r15;
            // 19e35 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call(t, &[rdi], &[]) }; return Ret::default(); }
        }
        0x19e3a => {
            // block 19e3a
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e3a mov rsi,QWORD PTR [r15+0x10]
            rsi = m.r64(r15.wrapping_add(0x10)) as u64;
            // 19e3e sub rsi,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rsi, m.r64(r15) as u64);
              rsi = r;
              fl = f; }
            // 19e41 mov rax,rsi
            rax = rsi;
            // 19e44 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e48 movabs rcx,0x1ffffffffffffffe
            rcx = 0x1fff_ffff_ffff_fffeu64;
            // 19e52 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 19e55 ja 0x19e7c
            if crate::cov::brk(0x19e55, fl.a()) { pc = 0x19e7c; continue; }
            pc = 0x19e57;
        }
        0x19e57 => {
            // block 19e57
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e57 sar rsi,1
            { let (r, f) = sar::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 19e5a cmp rsi,rbx
            fl = sub::<64>(rsi, rbx).1;
            // 19e5d cmovb rsi,rbx
            if fl.b() { rsi = rbx; }
            // 19e61 jmp 0x19e86
            pc = 0x19e86; continue;
        }
        0x19e63 => {
            // block 19e63
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e63 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e67 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19e6a sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 19e6e cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 19e71 jbe 0x19ea3
            if crate::cov::brk(0x19e71, fl.be()) { pc = 0x19ea3; continue; }
            pc = 0x19e73;
        }
        0x19e73 => {
            // block 19e73
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e73 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e73 lea rbx,[rsi+rax*4]
            rbx = rsi.wrapping_add(rax.wrapping_mul(4));
            // 19e77 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 19e7a jmp 0x19ea9
            pc = 0x19ea9; continue;
        }
        0x19e7c => {
            // block 19e7c
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e7c movabs rsi,0x3fffffffffffffff
            rsi = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x19e86;
        }
        0x19e86 => {
            // block 19e86
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19e86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19e86 mov rdi,r15
            rdi = r15;
            // 19e89 call 0xeba44
            { let r = { let t = m.env.stub_va(0x211); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19e8e mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19e92 mov rsi,r12
            rsi = r12;
            // 19e95 mov rdx,r14
            rdx = r14;
            // 19e98 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19e9d shl rbx,0x2
            { let (r, f) = shl::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 19ea1 jmp 0x19eda
            pc = 0x19eda; continue;
        }
        0x19ea3 => {
            // block 19ea3
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea3 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 19ea6 mov rbx,r12
            rbx = r12;
            pc = 0x19ea9;
        }
        0x19ea9 => {
            // block 19ea9
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ea9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ea9 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // 19ead sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 19eb0 mov rdi,r13
            rdi = r13;
            // 19eb3 mov rdx,rbx
            rdx = rbx;
            // 19eb6 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            // 19ebb mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            // 19ebf test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // 19ec2 je 0x19ee0
            if crate::cov::brk(0x19ec2, fl.e()) { pc = 0x19ee0; continue; }
            pc = 0x19ec4;
        }
        0x19ec4 => {
            // block 19ec4
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ec4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ec4 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 19ec8 sub r12,rsi
            { let (r, f) = sub::<64>(r12, rsi);
              r12 = r;
              fl = f; }
            // 19ecb mov rbx,r12
            rbx = r12;
            // 19ece and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ed2 mov rdx,r12
            rdx = r12;
            // 19ed5 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = m.cl[0]; rsi = m.cl[1]; rdi = m.cl[2]; r8 = m.cl[3]; r9 = m.cl[4]; r10 = m.cl[5]; r11 = m.cl[6];  }
            pc = 0x19eda;
        }
        0x19eda => {
            // block 19eda
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eda add QWORD PTR [r15+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rbx);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 19ede jmp 0x19f01
            pc = 0x19f01; continue;
        }
        0x19ee0 => {
            // block 19ee0
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19ee0 and rbx,0xfffffffffffffffc
            { let r = rbx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 19ee4 add r13,rbx
            { let (r, f) = add::<64>(r13, rbx);
              r13 = r;
              fl = f; }
            // 19ee7 cmp rdi,r13
            fl = sub::<64>(rdi, r13).1;
            // 19eea je 0x19f01
            if crate::cov::brk(0x19eea, fl.e()) { pc = 0x19f01; continue; }
            pc = 0x19eec;
        }
        0x19eec => {
            // block 19eec
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19eec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19eec lea rax,[rdi-0x4]
            rax = rdi.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19ef0 sub rax,r13
            { let (r, f) = sub::<64>(rax, r13);
              rax = r;
              fl = f; }
            // 19ef3 not rax
            { let v = !rax; rax = v; }
            // 19ef6 and rax,0xfffffffffffffffc
            { let r = rax & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 19efa add rax,rdi
            { let (r, f) = add::<64>(rax, rdi);
              rax = r;
              fl = f; }
            // 19efd mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            pc = 0x19f01;
        }
        0x19f01 => {
            // block 19f01
            #[cfg(feature = "trace")] eprintln!("f_00019dea pc 19f01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19f01 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 19f05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 19f06 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 19f08 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 19f0a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 19f0c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 19f0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19f0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
