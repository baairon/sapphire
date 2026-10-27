//! Mechanically lifted functions (scripts/lift_run.py), module g020. Each one is checked against the original
//! in the world harness (tests/world_g020.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xd7110 size=600
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7110<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7110;
    loop {
        match pc {
        0xd7110 => {
            // block d7110
            // d7110 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7111 mov rbp,rsp
            rbp = rsp;
            // d7114 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7116 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7118 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d711a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d711c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d711d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d711e mov r12,rsi
            r12 = rsi;
            // d7121 mov r14,rdi
            r14 = rdi;
            // d7124 lea rsi,[rip+0xe6655] # 0x1bd780
            rsi = m.env.data_va(859981);
            // d712b mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // d7130 mov rdi,r12
            rdi = r12;
            // d7133 call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7138 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d713a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd7140;
        }
        0xd7140 => {
            // block d7140
            // d7140 movzx eax,BYTE PTR [r12+0xd]
            rax = (m.r8(r12.wrapping_add(0xd)) as u64) as u32 as u64;
            // d7146 cmp eax,0x61
            fl = sub::<32>((rax as u32 as u64), 0x61u64).1;
            // d7149 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd714f;
        }
        0xd714f => {
            // block d714f
            // d714f mov bx,WORD PTR [r12+0xe]
            rbx = (rbx & !0xffff) | ((m.r16(r12.wrapping_add(0xe)) as u64) as u16 as u64);
            // d7155 rol bx,0x8
            { let (r, f) = rol::<16>((rbx as u16 as u64), 0x8u64, fl); rbx = (rbx & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7159 lea rsi,[r12+0x10]
            rsi = r12.wrapping_add(0x10);
            // d715e lea rdi,[r14+0x2e8]
            rdi = r14.wrapping_add(0x2e8);
            // d7165 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d716a movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // d716d movzx eax,BYTE PTR [rbx+r12*1+0x10]
            rax = (m.r8(rbx.wrapping_add(r12).wrapping_add(0x10)) as u64) as u32 as u64;
            // d7173 cmp eax,0x62
            fl = sub::<32>((rax as u32 as u64), 0x62u64).1;
            // d7176 jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd717c;
        }
        0xd717c => {
            // block d717c
            // d717c mov r13w,WORD PTR [rbx+r12*1+0x11]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x11)) as u64) as u16 as u64);
            // d7182 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d7187 lea rsi,[r12+rbx*1+0x13]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x13);
            // d718c lea r15,[r14+0x300]
            r15 = r14.wrapping_add(0x300);
            // d7193 mov rdi,r15
            rdi = r15;
            // d7196 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d719b movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d719f lea rbx,[rax+rbx*1+0x13]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x13);
            // d71a4 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71a9 cmp eax,0x63
            fl = sub::<32>((rax as u32 as u64), 0x63u64).1;
            // d71ac jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71ae;
        }
        0xd71ae => {
            // block d71ae
            // d71ae mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71b4 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71b9 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71be lea rdi,[r14+0x2b8]
            rdi = r14.wrapping_add(0x2b8);
            // d71c5 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71ca movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71ce lea rbx,[rax+rbx*1+0x3]
            rbx = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d71d3 movzx eax,BYTE PTR [r12+rbx*1]
            rax = (m.r8(r12.wrapping_add(rbx)) as u64) as u32 as u64;
            // d71d8 cmp eax,0x64
            fl = sub::<32>((rax as u32 as u64), 0x64u64).1;
            // d71db jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd71dd;
        }
        0xd71dd => {
            // block d71dd
            // d71dd mov r13w,WORD PTR [rbx+r12*1+0x1]
            r13 = (r13 & !0xffff) | ((m.r16(rbx.wrapping_add(r12).wrapping_add(0x1)) as u64) as u16 as u64);
            // d71e3 rol r13w,0x8
            { let (r, f) = rol::<16>((r13 as u16 as u64), 0x8u64, fl); r13 = (r13 & !0xffff) | ((r) as u16 as u64); fl = f; }
            // d71e8 lea rsi,[r12+rbx*1+0x3]
            rsi = r12.wrapping_add(rbx).wrapping_add(0x3);
            // d71ed lea rdi,[r14+0x2d0]
            rdi = r14.wrapping_add(0x2d0);
            // d71f4 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d71f9 movzx eax,r13w
            rax = ((r13 as u16 as u64)) as u32 as u64;
            // d71fd lea rax,[rax+rbx*1+0x3]
            rax = rax.wrapping_add(rbx).wrapping_add(0x3);
            // d7202 movzx ecx,BYTE PTR [r12+rax*1]
            rcx = (m.r8(r12.wrapping_add(rax)) as u64) as u32 as u64;
            // d7207 cmp ecx,0x65
            fl = sub::<32>((rcx as u32 as u64), 0x65u64).1;
            // d720a jne 0xd722c
            if fl.ne() { pc = 0xd722c; continue; }
            pc = 0xd720c;
        }
        0xd720c => {
            // block d720c
            // d720c mov ecx,DWORD PTR [rax+r12*1+0x1]
            rcx = (m.r32(rax.wrapping_add(r12).wrapping_add(0x1)) as u64) as u32 as u64;
            // d7211 bswap ecx
            rcx = (rcx as u32).swap_bytes() as u64;
            // d7213 mov DWORD PTR [r14+0x36c],ecx
            m.w32(r14.wrapping_add(0x36c), ((rcx as u32 as u64)) as u32);
            // d721a mov cl,BYTE PTR [r14+0x300]
            rcx = (rcx & !0xff) | ((m.r8(r14.wrapping_add(0x300)) as u64) as u8 as u64);
            // d7221 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d7224 jne 0xd723d
            if fl.ne() { pc = 0xd723d; continue; }
            pc = 0xd7226;
        }
        0xd7226 => {
            // block d7226
            // d7226 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d722a jmp 0xd7244
            pc = 0xd7244; continue;
        }
        0xd722c => {
            // block d722c
            // d722c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd722e;
        }
        0xd722e => {
            // block d722e
            // d722e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7232 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7233 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7235 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7237 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7239 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d723b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d723c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd723d => {
            // block d723d
            // d723d mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7244;
        }
        0xd7244 => {
            // block d7244
            // d7244 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7247 cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d724a je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd724c;
        }
        0xd724c => {
            // block d724c
            // d724c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d724e jne 0xd7256
            if fl.ne() { pc = 0xd7256; continue; }
            pc = 0xd7250;
        }
        0xd7250 => {
            // block d7250
            // d7250 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7254 jmp 0xd725d
            pc = 0xd725d; continue;
        }
        0xd7256 => {
            // block d7256
            // d7256 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd725d;
        }
        0xd725d => {
            // block d725d
            // d725d movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7260 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d7263 je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd7265;
        }
        0xd7265 => {
            // block d7265
            // d7265 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7267 jne 0xd726f
            if fl.ne() { pc = 0xd726f; continue; }
            pc = 0xd7269;
        }
        0xd7269 => {
            // block d7269
            // d7269 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d726d jmp 0xd7276
            pc = 0xd7276; continue;
        }
        0xd726f => {
            // block d726f
            // d726f mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7276;
        }
        0xd7276 => {
            // block d7276
            // d7276 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7279 cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d727c je 0xd729b
            if fl.e() { pc = 0xd729b; continue; }
            pc = 0xd727e;
        }
        0xd727e => {
            // block d727e
            // d727e test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7280 jne 0xd7288
            if fl.ne() { pc = 0xd7288; continue; }
            pc = 0xd7282;
        }
        0xd7282 => {
            // block d7282
            // d7282 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7286 jmp 0xd728f
            pc = 0xd728f; continue;
        }
        0xd7288 => {
            // block d7288
            // d7288 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd728f;
        }
        0xd728f => {
            // block d728f
            // d728f movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d7292 cmp edx,0x78
            fl = sub::<32>((rdx as u32 as u64), 0x78u64).1;
            // d7295 jne 0xd7361
            if fl.ne() { pc = 0xd7361; continue; }
            pc = 0xd729b;
        }
        0xd729b => {
            // block d729b
            // d729b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d729d jne 0xd72a5
            if fl.ne() { pc = 0xd72a5; continue; }
            pc = 0xd729f;
        }
        0xd729f => {
            // block d729f
            // d729f lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72a3 jmp 0xd72ac
            pc = 0xd72ac; continue;
        }
        0xd72a5 => {
            // block d72a5
            // d72a5 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72ac;
        }
        0xd72ac => {
            // block d72ac
            // d72ac movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72af cmp edx,0x35
            fl = sub::<32>((rdx as u32 as u64), 0x35u64).1;
            // d72b2 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72b4;
        }
        0xd72b4 => {
            // block d72b4
            // d72b4 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72b6 jne 0xd72be
            if fl.ne() { pc = 0xd72be; continue; }
            pc = 0xd72b8;
        }
        0xd72b8 => {
            // block d72b8
            // d72b8 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72bc jmp 0xd72c5
            pc = 0xd72c5; continue;
        }
        0xd72be => {
            // block d72be
            // d72be mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72c5;
        }
        0xd72c5 => {
            // block d72c5
            // d72c5 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72c8 cmp edx,0x36
            fl = sub::<32>((rdx as u32 as u64), 0x36u64).1;
            // d72cb jne 0xd72e7
            if fl.ne() { pc = 0xd72e7; continue; }
            pc = 0xd72cd;
        }
        0xd72cd => {
            // block d72cd
            // d72cd test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72cf jne 0xd72d7
            if fl.ne() { pc = 0xd72d7; continue; }
            pc = 0xd72d1;
        }
        0xd72d1 => {
            // block d72d1
            // d72d1 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72d5 jmp 0xd72de
            pc = 0xd72de; continue;
        }
        0xd72d7 => {
            // block d72d7
            // d72d7 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72de;
        }
        0xd72de => {
            // block d72de
            // d72de movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d72e2 cmp edx,0x76
            fl = sub::<32>((rdx as u32 as u64), 0x76u64).1;
            // d72e5 je 0xd731a
            if fl.e() { pc = 0xd731a; continue; }
            pc = 0xd72e7;
        }
        0xd72e7 => {
            // block d72e7
            // d72e7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d72e9 jne 0xd72f1
            if fl.ne() { pc = 0xd72f1; continue; }
            pc = 0xd72eb;
        }
        0xd72eb => {
            // block d72eb
            // d72eb lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d72ef jmp 0xd72f8
            pc = 0xd72f8; continue;
        }
        0xd72f1 => {
            // block d72f1
            // d72f1 mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd72f8;
        }
        0xd72f8 => {
            // block d72f8
            // d72f8 movzx edx,BYTE PTR [rdx]
            rdx = (m.r8(rdx) as u64) as u32 as u64;
            // d72fb cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // d72fe jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd7300;
        }
        0xd7300 => {
            // block d7300
            // d7300 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d7302 jne 0xd730a
            if fl.ne() { pc = 0xd730a; continue; }
            pc = 0xd7304;
        }
        0xd7304 => {
            // block d7304
            // d7304 lea rdx,[r15+0x1]
            rdx = r15.wrapping_add(0x1);
            // d7308 jmp 0xd7311
            pc = 0xd7311; continue;
        }
        0xd730a => {
            // block d730a
            // d730a mov rdx,QWORD PTR [r14+0x310]
            rdx = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd7311;
        }
        0xd7311 => {
            // block d7311
            // d7311 movzx edx,BYTE PTR [rdx+0x1]
            rdx = (m.r8(rdx.wrapping_add(0x1)) as u64) as u32 as u64;
            // d7315 cmp edx,0x6b
            fl = sub::<32>((rdx as u32 as u64), 0x6bu64).1;
            // d7318 jne 0xd733a
            if fl.ne() { pc = 0xd733a; continue; }
            pc = 0xd731a;
        }
        0xd731a => {
            // block d731a
            // d731a lea rdi,[rax+r12*1+0x31]
            rdi = rax.wrapping_add(r12).wrapping_add(0x31);
            pc = 0xd731f;
        }
        0xd731f => {
            // block d731f
            // d731f lea rsi,[rip+0xe6467] # 0x1bd78d
            rsi = m.env.data_va(859994);
            // d7326 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // d732b call 0xec2c6
            { let r = { let t = m.env.stub_va(0xa93); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7330 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d7332 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d7335 jmp 0xd722e
            pc = 0xd722e; continue;
        }
        0xd733a => {
            // block d733a
            // d733a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d733c jne 0xd7343
            if fl.ne() { pc = 0xd7343; continue; }
            pc = 0xd733e;
        }
        0xd733e => {
            // block d733e
            // d733e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // d7341 jmp 0xd734a
            pc = 0xd734a; continue;
        }
        0xd7343 => {
            // block d7343
            // d7343 mov r15,QWORD PTR [r14+0x310]
            r15 = m.r64(r14.wrapping_add(0x310)) as u64;
            pc = 0xd734a;
        }
        0xd734a => {
            // block d734a
            // d734a movzx ecx,BYTE PTR [r15]
            rcx = (m.r8(r15) as u64) as u32 as u64;
            // d734e cmp ecx,0x78
            fl = sub::<32>((rcx as u32 as u64), 0x78u64).1;
            // d7351 jne 0xd735a
            if fl.ne() { pc = 0xd735a; continue; }
            pc = 0xd7353;
        }
        0xd7353 => {
            // block d7353
            // d7353 lea rdi,[rax+r12*1+0x51]
            rdi = rax.wrapping_add(r12).wrapping_add(0x51);
            // d7358 jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd735a => {
            // block d735a
            // d735a lea rdi,[rax+r12*1+0x11]
            rdi = rax.wrapping_add(r12).wrapping_add(0x11);
            // d735f jmp 0xd731f
            pc = 0xd731f; continue;
        }
        0xd7361 => {
            // block d7361
            // d7361 lea rdi,[rax+r12*1+0x5]
            rdi = rax.wrapping_add(r12).wrapping_add(0x5);
            // d7366 jmp 0xd731f
            pc = 0xd731f; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe00a1 size=867
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e00a1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe00a1;
    loop {
        match pc {
        0xe00a1 => {
            // block e00a1
            // e00a1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e00a2 mov rbp,rsp
            rbp = rsp;
            // e00a5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e00a7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e00a9 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e00ab push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e00ad push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e00ae sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e00b2 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e00b5 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // e00b7 mov QWORD PTR [rbp-0x40],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rsi) as u64);
            // e00bb mov r14,rdi
            r14 = rdi;
            // e00be lea eax,[rbx+0x4]
            rax = (rbx.wrapping_add(0x4)) as u32 as u64;
            // e00c1 movsxd rdi,eax
            rdi = sx::<32>((rax as u32 as u64)) as u64;
            // e00c4 add rdi,rdi
            { let (r, f) = add::<64>(rdi, rdi);
              rdi = r;
              fl = f; }
            // e00c7 mov r15,0xffffffffffffffff
            r15 = 0xffff_ffff_ffff_ffffu64;
            // e00ce cmovb rdi,r15
            if fl.b() { rdi = r15; }
            // e00d2 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00d7 mov r13,rax
            r13 = rax;
            // e00da mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e00dc shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00df add eax,ebx
            { let (r, f) = add::<32>((rax as u32 as u64), (rbx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e00e1 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e00e3 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e00e7 lea eax,[rax+0x4]
            rax = (rax.wrapping_add(0x4)) as u32 as u64;
            // e00ea movsxd r12,eax
            r12 = sx::<32>((rax as u32 as u64)) as u64;
            // e00ed add r12,r12
            { let (r, f) = add::<64>(r12, r12);
              r12 = r;
              fl = f; }
            // e00f0 cmovb r12,r15
            if fl.b() { r12 = r15; }
            // e00f4 mov rdi,r12
            rdi = r12;
            // e00f7 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00fc mov r15,rax
            r15 = rax;
            // e00ff mov rdi,r12
            rdi = r12;
            // e0102 call 0xec140
            { let r = { let t = m.env.stub_va(0x90d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0107 mov r12,rax
            r12 = rax;
            // e010a test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // e010c jle 0xe0159
            if fl.le() { pc = 0xe0159; continue; }
            pc = 0xe010e;
        }
        0xe010e => {
            // block e010e
            // e010e xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0110 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0112;
        }
        0xe0112 => {
            // block e0112
            // e0112 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // e0114 shr esi,0x1f
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1fu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e0117 add esi,ecx
            { let (r, f) = add::<32>((rsi as u32 as u64), (rcx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0119 sar esi,1
            { let (r, f) = sar::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // e011b inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e011d test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // e0120 movsxd rdx,eax
            rdx = sx::<32>((rax as u32 as u64)) as u64;
            // e0123 movzx edx,WORD PTR [r14+rdx*2]
            rdx = (m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u32 as u64;
            // e0128 movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e012b jne 0xe0134
            if fl.ne() { pc = 0xe0134; continue; }
            pc = 0xe012d;
        }
        0xe012d => {
            // block e012d
            // e012d mov WORD PTR [r15+rsi*2],dx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            // e0132 jmp 0xe0139
            pc = 0xe0139; continue;
        }
        0xe0134 => {
            // block e0134
            // e0134 mov WORD PTR [r12+rsi*2],dx
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rdx as u16 as u64)) as u16);
            pc = 0xe0139;
        }
        0xe0139 => {
            // block e0139
            // e0139 lea edx,[rax+0x1]
            rdx = (rax.wrapping_add(0x1)) as u32 as u64;
            // e013c movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e013f mov dx,WORD PTR [r14+rdx*2]
            rdx = (rdx & !0xffff) | ((m.r16(r14.wrapping_add(rdx.wrapping_mul(2))) as u64) as u16 as u64);
            // e0144 mov WORD PTR [r13+rax*1+0x2],dx
            m.w16(r13.wrapping_add(rax).wrapping_add(0x2), ((rdx as u16 as u64)) as u16);
            // e014a inc ecx
            { let (r, f) = inc::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // e014c add rax,0x2
            { let (r, f) = add::<64>(rax, 0x2u64);
              rax = r;
              fl = f; }
            // e0150 cmp ebx,ecx
            fl = sub::<32>((rbx as u32 as u64), (rcx as u32 as u64)).1;
            // e0152 jne 0xe0112
            if fl.ne() { pc = 0xe0112; continue; }
            pc = 0xe0154;
        }
        0xe0154 => {
            // block e0154
            // e0154 movzx eax,WORD PTR [r13+0x2]
            rax = (m.r16(r13.wrapping_add(0x2)) as u64) as u32 as u64;
            pc = 0xe0159;
        }
        0xe0159 => {
            // block e0159
            // e0159 mov WORD PTR [r13+0x0],ax
            m.w16(r13, ((rax as u16 as u64)) as u16);
            // e015e movsxd rax,ebx
            rax = sx::<32>((rbx as u32 as u64)) as u64;
            // e0161 mov ax,WORD PTR [r13+rax*2+0x0]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0167 lea ecx,[rbx+0x2]
            rcx = (rbx.wrapping_add(0x2)) as u32 as u64;
            // e016a movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e016d mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0173 lea ecx,[rbx+0x1]
            rcx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // e0176 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e0179 mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e017f shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0182 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e0184 cdq
            rdx = if (rax as u32 as i32) < 0 { 0xffff_ffff } else { 0 };
            // e0185 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0188 idiv ecx
            { let (q, r) = idiv::<32>(rdx, rax, (rcx as u32 as u64)); rax = q; rdx = r; }
            // e018a mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // e018d test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // e018f mov r14,QWORD PTR [rbp-0x40]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e0193 jle 0xe0244
            if fl.le() { pc = 0xe0244; continue; }
            pc = 0xe0199;
        }
        0xe0199 => {
            // block e0199
            // e0199 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // e019e xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e01a1 lea r9,[rip+0x47e78] # 0x128020
            r9 = m.env.data_va(247789);
            // e01a8 mov r8w,0x4
            r8 = (r8 & !0xffff) | ((0x4u64) as u16 as u64);
            // e01ad mov r10d,DWORD PTR [rbp-0x38]
            r10 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            pc = 0xe01b1;
        }
        0xe01b1 => {
            // block e01b1
            // e01b1 mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e01b4 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e01b7 mov edx,r11d
            rdx = ((r11 as u32 as u64)) as u32 as u64;
            // e01ba shr edx,0xb
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // e01bd and edx,0x1f
            { let r = (rdx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e01c0 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e01c2 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e01c4 movsx ebx,WORD PTR [r13+rax*2-0x2]
            rbx = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e01ca mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // e01cf sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01d1 imul ebx,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rbx as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rbx = (r) as u32 as u64; fl = f; }
            // e01d6 movsx edi,WORD PTR [r13+rax*2+0x0]
            rdi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e01dc mov esi,0x40
            rsi = (0x40u64) as u32 as u64;
            // e01e1 sub esi,edx
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rdx as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01e3 imul edi,DWORD PTR [r9+rsi*4]
            { let (r, f) = imul::<32>((rdi as u32 as u64), m.r32(r9.wrapping_add(rsi.wrapping_mul(4))) as u64); rdi = (r) as u32 as u64; fl = f; }
            // e01e8 add edi,ebx
            { let (r, f) = add::<32>((rdi as u32 as u64), (rbx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e01ea movsx esi,WORD PTR [r13+rax*2+0x2]
            rsi = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e01f0 mov ebx,0x60
            rbx = (0x60u64) as u32 as u64;
            // e01f5 sub ebx,edx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e01f7 imul esi,DWORD PTR [r9+rbx*4]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r9.wrapping_add(rbx.wrapping_mul(4))) as u64); rsi = (r) as u32 as u64; fl = f; }
            // e01fc add esi,edi
            { let (r, f) = add::<32>((rsi as u32 as u64), (rdi as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // e01fe movsx eax,WORD PTR [r13+rax*2+0x4]
            rax = (sx::<16>(m.r16(r13.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e0204 mov edi,0x80
            rdi = (0x80u64) as u32 as u64;
            // e0209 sub edi,edx
            { let (r, f) = sub::<32>((rdi as u32 as u64), (rdx as u32 as u64));
              rdi = (r) as u32 as u64;
              fl = f; }
            // e020b imul eax,DWORD PTR [r9+rdi*4]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(r9.wrapping_add(rdi.wrapping_mul(4))) as u64); rax = (r) as u32 as u64; fl = f; }
            // e0210 add eax,esi
            { let (r, f) = add::<32>((rax as u32 as u64), (rsi as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0212 mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0216 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e021b ja 0xe022c
            if fl.a() { pc = 0xe022c; continue; }
            pc = 0xe021d;
        }
        0xe021d => {
            // block e021d
            // e021d mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e021f shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0222 cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e0227 cmovb bx,r8w
            if fl.b() { rbx = (rbx & !0xffff) | (((r8 as u16 as u64)) as u16 as u64); }
            pc = 0xe022c;
        }
        0xe022c => {
            // block e022c
            // e022c movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e022f mov WORD PTR [r14+rcx*2],bx
            m.w16(r14.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e0234 add ecx,0x2
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x2u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0237 add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e023b dec r10d
            { let (r, f) = dec::<32>((r10 as u32 as u64), fl); r10 = (r) as u32 as u64; fl = f; }
            // e023e jne 0xe01b1
            if fl.ne() { pc = 0xe01b1; continue; }
            pc = 0xe0244;
        }
        0xe0244 => {
            // block e0244
            // e0244 mov QWORD PTR [rbp-0x60],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (r13) as u64);
            // e0248 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // e024c mov ax,WORD PTR [r15+0x2]
            rax = (rax & !0xffff) | ((m.r16(r15.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0251 mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e0255 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e0259 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e025c mov cx,WORD PTR [r15+rax*2]
            rcx = (rcx & !0xffff) | ((m.r16(r15.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e0261 lea edx,[rsi+0x2]
            rdx = (rsi.wrapping_add(0x2)) as u32 as u64;
            // e0264 movsxd rdx,edx
            rdx = sx::<32>((rdx as u32 as u64)) as u64;
            // e0267 mov WORD PTR [r15+rdx*2],cx
            m.w16(r15.wrapping_add(rdx.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e026c inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e026e movsxd rsi,esi
            rsi = sx::<32>((rsi as u32 as u64)) as u64;
            // e0271 mov WORD PTR [r15+rsi*2],cx
            m.w16(r15.wrapping_add(rsi.wrapping_mul(2)), ((rcx as u16 as u64)) as u16);
            // e0276 mov QWORD PTR [rbp-0x50],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r12) as u64);
            // e027a mov cx,WORD PTR [r12+0x2]
            rcx = (rcx & !0xffff) | ((m.r16(r12.wrapping_add(0x2)) as u64) as u16 as u64);
            // e0280 mov WORD PTR [r12],cx
            m.w16(r12, ((rcx as u16 as u64)) as u16);
            // e0285 mov ax,WORD PTR [r12+rax*2]
            rax = (rax & !0xffff) | ((m.r16(r12.wrapping_add(rax.wrapping_mul(2))) as u64) as u16 as u64);
            // e028a mov WORD PTR [r12+rdx*2],ax
            m.w16(r12.wrapping_add(rdx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e028f mov WORD PTR [r12+rsi*2],ax
            m.w16(r12.wrapping_add(rsi.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0294 mov ecx,DWORD PTR [rbp-0x38]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // e0297 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e029a jl 0xe03db
            if fl.l() { pc = 0xe03db; continue; }
            pc = 0xe02a0;
        }
        0xe02a0 => {
            // block e02a0
            // e02a0 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // e02a2 shr eax,0x1f
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1fu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a5 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e02a7 sar eax,1
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02a9 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e02ab mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // e02af mov r12d,0x2
            r12 = (0x2u64) as u32 as u64;
            // e02b5 xor r11d,r11d
            { let r = (r11 as u32 as u64) ^ (r11 as u32 as u64); let f = logic::<32>(r);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e02b8 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe02bb;
        }
        0xe02bb => {
            // block e02bb
            // e02bb mov r13,r14
            r13 = r14;
            // e02be mov eax,r11d
            rax = ((r11 as u32 as u64)) as u32 as u64;
            // e02c1 sar eax,0x10
            { let (r, f) = sar::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e02c4 mov ebx,r11d
            rbx = ((r11 as u32 as u64)) as u32 as u64;
            // e02c7 shr ebx,0xb
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0xbu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e02ca and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e02cd inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e02cf movsxd r14,eax
            r14 = sx::<32>((rax as u32 as u64)) as u64;
            // e02d2 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e02d6 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e02dc mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // e02e1 sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e02e3 lea rdx,[rip+0x47d36] # 0x128020
            rdx = m.env.data_va(247789);
            // e02ea mov rdi,rdx
            rdi = rdx;
            // e02ed mov ecx,DWORD PTR [rdi+rcx*4]
            rcx = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e02f0 mov DWORD PTR [rbp-0x38],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), ((rcx as u32 as u64)) as u32);
            // e02f3 imul eax,ecx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rcx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e02f6 movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e02fb mov edx,0x40
            rdx = (0x40u64) as u32 as u64;
            // e0300 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0302 mov edx,DWORD PTR [rdi+rdx*4]
            rdx = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0305 mov DWORD PTR [rbp-0x40],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rdx as u32 as u64)) as u32);
            // e0308 imul ecx,edx
            { let (r, f) = imul::<32>((rcx as u32 as u64), (rdx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // e030b add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e030d movsx eax,WORD PTR [rsi+r14*2+0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e0313 mov edx,0x60
            rdx = (0x60u64) as u32 as u64;
            // e0318 sub edx,ebx
            { let (r, f) = sub::<32>((rdx as u32 as u64), (rbx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e031a mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // e031e imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e0322 add eax,ecx
            { let (r, f) = add::<32>((rax as u32 as u64), (rcx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e0324 movsx r10d,WORD PTR [rsi+r14*2+0x4]
            r10 = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e032a mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // e032f sub ecx,ebx
            { let (r, f) = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0331 mov r9d,DWORD PTR [rdi+rcx*4]
            r9 = (m.r32(rdi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // e0335 imul r10d,r9d
            { let (r, f) = imul::<32>((r10 as u32 as u64), (r9 as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // e0339 add r10d,eax
            { let (r, f) = add::<32>((r10 as u32 as u64), (rax as u32 as u64));
              r10 = (r) as u32 as u64;
              fl = f; }
            // e033c mov bx,0x3fb
            rbx = (rbx & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e0340 cmp r10d,0x3fbffff
            fl = sub::<32>((r10 as u32 as u64), 0x3fbffffu64).1;
            // e0347 mov ax,0x3fb
            rax = (rax & !0xffff) | ((0x3fbu64) as u16 as u64);
            // e034b ja 0xe0362
            if fl.a() { pc = 0xe0362; continue; }
            pc = 0xe034d;
        }
        0xe034d => {
            // block e034d
            // e034d mov eax,r10d
            rax = ((r10 as u32 as u64)) as u32 as u64;
            // e0350 shr eax,0x10
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0353 cmp r10d,0x40000
            fl = sub::<32>((r10 as u32 as u64), 0x40000u64).1;
            // e035a mov cx,0x4
            rcx = (rcx & !0xffff) | ((0x4u64) as u16 as u64);
            // e035e cmovb ax,cx
            if fl.b() { rax = (rax & !0xffff) | (((rcx as u16 as u64)) as u16 as u64); }
            pc = 0xe0362;
        }
        0xe0362 => {
            // block e0362
            // e0362 lea ecx,[r12-0x2]
            rcx = (r12.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // e0367 movsxd rcx,ecx
            rcx = sx::<32>((rcx as u32 as u64)) as u64;
            // e036a mov WORD PTR [r13+rcx*2+0x0],ax
            m.w16(r13.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16 as u64)) as u16);
            // e0370 mov rsi,QWORD PTR [rbp-0x50]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e0374 movsx eax,WORD PTR [rsi+r14*2-0x2]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0xffff_ffff_ffff_fffe)) as u64) as u64) as u32 as u64;
            // e037a imul eax,DWORD PTR [rbp-0x38]
            { let (r, f) = imul::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64); rax = (r) as u32 as u64; fl = f; }
            // e037e movsx ecx,WORD PTR [rsi+r14*2]
            rcx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2))) as u64) as u64) as u32 as u64;
            // e0383 imul ecx,DWORD PTR [rbp-0x40]
            { let (r, f) = imul::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64); rcx = (r) as u32 as u64; fl = f; }
            // e0387 add ecx,eax
            { let (r, f) = add::<32>((rcx as u32 as u64), (rax as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0389 movsx edx,WORD PTR [rsi+r14*2+0x2]
            rdx = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x2)) as u64) as u64) as u32 as u64;
            // e038f imul edx,r8d
            { let (r, f) = imul::<32>((rdx as u32 as u64), (r8 as u32 as u64)); rdx = (r) as u32 as u64; fl = f; }
            // e0393 add edx,ecx
            { let (r, f) = add::<32>((rdx as u32 as u64), (rcx as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // e0395 movsx eax,WORD PTR [rsi+r14*2+0x4]
            rax = (sx::<16>(m.r16(rsi.wrapping_add(r14.wrapping_mul(2)).wrapping_add(0x4)) as u64) as u64) as u32 as u64;
            // e039b imul eax,r9d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r9 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // e039f add eax,edx
            { let (r, f) = add::<32>((rax as u32 as u64), (rdx as u32 as u64));
              rax = (r) as u32 as u64;
              fl = f; }
            // e03a1 cmp eax,0x3fbffff
            fl = sub::<32>((rax as u32 as u64), 0x3fbffffu64).1;
            // e03a6 ja 0xe03ba
            if fl.a() { pc = 0xe03ba; continue; }
            pc = 0xe03a8;
        }
        0xe03a8 => {
            // block e03a8
            // e03a8 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e03aa shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e03ad cmp eax,0x40000
            fl = sub::<32>((rax as u32 as u64), 0x40000u64).1;
            // e03b2 mov ax,0x4
            rax = (rax & !0xffff) | ((0x4u64) as u16 as u64);
            // e03b6 cmovb bx,ax
            if fl.b() { rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64); }
            pc = 0xe03ba;
        }
        0xe03ba => {
            // block e03ba
            // e03ba movsxd rax,r12d
            rax = sx::<32>((r12 as u32 as u64)) as u64;
            // e03bd mov WORD PTR [r13+rax*2+0x0],bx
            m.w16(r13.wrapping_add(rax.wrapping_mul(2)), ((rbx as u16 as u64)) as u16);
            // e03c3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e03c6 add r12d,0x4
            { let (r, f) = add::<32>((r12 as u32 as u64), 0x4u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e03ca add r11d,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((r11 as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              r11 = (r) as u32 as u64;
              fl = f; }
            // e03ce cmp r15,QWORD PTR [rbp-0x58]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64).1;
            // e03d2 mov r14,r13
            r14 = r13;
            // e03d5 jl 0xe02bb
            if fl.l() { pc = 0xe02bb; continue; }
            pc = 0xe03db;
        }
        0xe03db => {
            // block e03db
            // e03db mov rdi,QWORD PTR [rbp-0x60]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // e03df call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03e4 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e03e8 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e03ed mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // e03f1 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e03f5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e03f6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e03f8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e03fa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e03fc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e03fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e03ff jmp 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2238 size=13
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2238<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd2238;
    loop {
        match pc {
        0xd2238 => {
            // block d2238
            // d2238 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2239 mov rbp,rsp
            rbp = rsp;
            // d223c call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d2241 dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d2243 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2244 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd2245 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d2245<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd2245;
    loop {
        match pc {
        0xd2245 => {
            // block d2245
            // d2245 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d2246 mov rbp,rsp
            rbp = rsp;
            // d2249 call 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d224e add eax,0xfffffffe
            { let (r, f) = add::<32>((rax as u32 as u64), 0xfffffffeu64);
              rax = (r) as u32 as u64;
              fl = f; }
            // d2251 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d2252 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7ffc size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7ffc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7ffc;
    loop {
        match pc {
        0xb7ffc => {
            // block b7ffc
            // b7ffc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7ffd mov rbp,rsp
            rbp = rsp;
            // b8000 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8002 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8003 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8006 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8008 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b800b call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8011 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8013 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8015 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8018 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8019 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b801b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b801c jmp 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8022 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8022<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8022;
    loop {
        match pc {
        0xb8022 => {
            // block b8022
            // b8022 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8023 mov rbp,rsp
            rbp = rsp;
            // b8026 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8028 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8029 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b802c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b802e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8031 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8037 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8039 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b803b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b803e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b803f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8041 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8042 jmp 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8048 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8048<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8048;
    loop {
        match pc {
        0xb8048 => {
            // block b8048
            // b8048 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8049 mov rbp,rsp
            rbp = rsp;
            // b804c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b804e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b804f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8052 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8054 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8057 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b805d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b805f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8061 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8064 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8065 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8067 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8068 jmp 0xd2238
            { let r = { let t = m.env.code_va(0xd2238); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb806e size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b806e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb806e;
    loop {
        match pc {
        0xb806e => {
            // block b806e
            // b806e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b806f mov rbp,rsp
            rbp = rsp;
            // b8072 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8074 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8075 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8078 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b807a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b807d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8083 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8085 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8087 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b808a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b808b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b808d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b808e jmp 0xd2245
            { let r = { let t = m.env.code_va(0xd2245); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbbc2c size=111
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bbc2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbbc2c;
    loop {
        match pc {
        0xbbc2c => {
            // block bbc2c
            // bbc2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bbc2d mov rbp,rsp
            rbp = rsp;
            // bbc30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bbc31 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bbc32 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // bbc34 lea eax,[rdi-0x2]
            rax = (rdi.wrapping_add(0xffff_ffff_ffff_fffe)) as u32 as u64;
            // bbc37 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // bbc3a ja 0xbbc68
            if fl.a() { pc = 0xbbc68; continue; }
            pc = 0xbbc3c;
        }
        0xbbc3c => {
            // block bbc3c
            // bbc3c call 0x5c889
            { let r = { let t = m.env.code_va(0x5c889); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bbc41 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // bbc43 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // bbc48 cmp ecx,0x7
            fl = sub::<32>((rcx as u32 as u64), 0x7u64).1;
            // bbc4b ja 0xbbc94
            if fl.a() { pc = 0xbbc94; continue; }
            pc = 0xbbc4d;
        }
        0xbbc4d => {
            // block bbc4d
            // bbc4d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // bbc4f lea rcx,[rip+0x4707a] # 0x102cd0
            rcx = m.env.data_va(95389);
            // bbc56 lea rdx,[rip+0x47053] # 0x102cb0
            rdx = m.env.data_va(95357);
            // bbc5d test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // bbc5f cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // bbc63 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc66 jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc68 => {
            // block bbc68
            // bbc68 mov eax,0x76
            rax = (0x76u64) as u32 as u64;
            // bbc6d test edi,edi
            fl = logic::<32>((rdi as u32 as u64) & (rdi as u32 as u64));
            // bbc6f je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc71;
        }
        0xbbc71 => {
            // block bbc71
            // bbc71 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // bbc74 jne 0xbbc8f
            if fl.ne() { pc = 0xbbc8f; continue; }
            pc = 0xbbc76;
        }
        0xbbc76 => {
            // block bbc76
            // bbc76 movzx ecx,dx
            rcx = ((rdx as u16 as u64)) as u32 as u64;
            // bbc79 mov eax,0x65
            rax = (0x65u64) as u32 as u64;
            // bbc7e cmp ecx,0x63
            fl = sub::<32>((rcx as u32 as u64), 0x63u64).1;
            // bbc81 je 0xbbc94
            if fl.e() { pc = 0xbbc94; continue; }
            pc = 0xbbc83;
        }
        0xbbc83 => {
            // block bbc83
            // bbc83 lea rax,[rip+0x47016] # 0x102ca0
            rax = m.env.data_va(95341);
            // bbc8a mov eax,DWORD PTR [rax+rcx*4]
            rax = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // bbc8d jmp 0xbbc94
            pc = 0xbbc94; continue;
        }
        0xbbc8f => {
            // block bbc8f
            // bbc8f mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            pc = 0xbbc94;
        }
        0xbbc94 => {
            // block bbc94
            // bbc94 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bbc98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bbc99 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bbc9a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xeb7b0 size=131
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000eb7b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xeb7b0;
    loop {
        match pc {
        0xeb7b0 => {
            // block eb7b0
            // eb7b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // eb7b1 mov rbp,rsp
            rbp = rsp;
            // eb7b4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // eb7b6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // eb7b8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // eb7ba push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // eb7bc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // eb7bd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // eb7be mov r12,r8
            r12 = r8;
            // eb7c1 mov rbx,rcx
            rbx = rcx;
            // eb7c4 mov r13,rdx
            r13 = rdx;
            // eb7c7 mov r14,rsi
            r14 = rsi;
            // eb7ca mov r15,rdi
            r15 = rdi;
            // eb7cd jmp 0xeb7d3
            pc = 0xeb7d3; continue;
        }
        0xeb7cf => {
            // block eb7cf
            // eb7cf add r12,0x38
            { let (r, f) = add::<64>(r12, 0x38u64);
              r12 = r;
              fl = f; }
            pc = 0xeb7d3;
        }
        0xeb7d3 => {
            // block eb7d3
            // eb7d3 cmp r15,r14
            fl = sub::<64>(r15, r14).1;
            // eb7d6 je 0xeb824
            if fl.e() { pc = 0xeb824; continue; }
            pc = 0xeb7d8;
        }
        0xeb7d8 => {
            // block eb7d8
            // eb7d8 cmp r13,rbx
            fl = sub::<64>(r13, rbx).1;
            // eb7db je 0xeb809
            if fl.e() { pc = 0xeb809; continue; }
            pc = 0xeb7dd;
        }
        0xeb7dd => {
            // block eb7dd
            // eb7dd mov eax,DWORD PTR [r13+0x18]
            rax = (m.r32(r13.wrapping_add(0x18)) as u64) as u32 as u64;
            // eb7e1 cmp eax,DWORD PTR [r15+0x18]
            fl = sub::<32>((rax as u32 as u64), m.r32(r15.wrapping_add(0x18)) as u64).1;
            // eb7e5 jge 0xeb7f8
            if fl.ge() { pc = 0xeb7f8; continue; }
            pc = 0xeb7e7;
        }
        0xeb7e7 => {
            // block eb7e7
            // eb7e7 mov rdi,r12
            rdi = r12;
            // eb7ea mov rsi,r13
            rsi = r13;
            // eb7ed call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb7f2 add r13,0x38
            { let (r, f) = add::<64>(r13, 0x38u64);
              r13 = r;
              fl = f; }
            // eb7f6 jmp 0xeb7cf
            pc = 0xeb7cf; continue;
        }
        0xeb7f8 => {
            // block eb7f8
            // eb7f8 mov rdi,r12
            rdi = r12;
            // eb7fb mov rsi,r15
            rsi = r15;
            // eb7fe call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb803 add r15,0x38
            { let (r, f) = add::<64>(r15, 0x38u64);
              r15 = r;
              fl = f; }
            // eb807 jmp 0xeb7cf
            pc = 0xeb7cf; continue;
        }
        0xeb809 => {
            // block eb809
            // eb809 sub r14,r15
            { let (r, f) = sub::<64>(r14, r15);
              r14 = r;
              fl = f; }
            // eb80c xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xeb80e;
        }
        0xeb80e => {
            // block eb80e
            // eb80e lea rsi,[r15+rbx*1]
            rsi = r15.wrapping_add(rbx);
            // eb812 lea rdi,[r12+rbx*1]
            rdi = r12.wrapping_add(rbx);
            // eb816 call 0xe064a
            { let r = { let t = m.env.code_va(0xe064a); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // eb81b add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // eb81f cmp r14,rbx
            fl = sub::<64>(r14, rbx).1;
            // eb822 jne 0xeb80e
            if fl.ne() { pc = 0xeb80e; continue; }
            pc = 0xeb824;
        }
        0xeb824 => {
            // block eb824
            // eb824 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // eb828 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // eb829 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // eb82b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // eb82d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // eb82f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // eb831 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // eb832 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbc038 size=132
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bc038<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbc038;
    loop {
        match pc {
        0xbc038 => {
            // block bc038
            // bc038 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bc039 mov rbp,rsp
            rbp = rsp;
            // bc03c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // bc03e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bc040 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // bc042 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bc043 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc047 mov r14,rsi
            r14 = rsi;
            // bc04a mov r12d,edi
            r12 = ((rdi as u32 as u64)) as u32 as u64;
            // bc04d lea rbx,[r14+0x8]
            rbx = r14.wrapping_add(0x8);
            // bc051 mov rsi,QWORD PTR [r14+0x8]
            rsi = m.r64(r14.wrapping_add(0x8)) as u64;
            // bc055 mov rdi,r14
            rdi = r14;
            // bc058 call 0xebd80
            { let r = { let t = m.env.stub_va(0x54d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc05d mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // bc065 mov QWORD PTR [r14],rbx
            m.w64(r14, (rbx) as u64);
            // bc068 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // bc070 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // bc077 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // bc07c lea r15,[rbp-0x24]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            pc = 0xbc080;
        }
        0xbc080 => {
            // block bc080
            // bc080 lea esi,[rbx-0x1]
            rsi = (rbx.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // bc083 mov edi,r12d
            rdi = ((r12 as u32 as u64)) as u32 as u64;
            // bc086 call 0xca0d0
            { let r = { let t = m.env.code_va(0xca0d0); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc08b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bc08d je 0xbc09a
            if fl.e() { pc = 0xbc09a; continue; }
            pc = 0xbc08f;
        }
        0xbc08f => {
            // block bc08f
            // bc08f mov rdi,r14
            rdi = r14;
            // bc092 mov rsi,r15
            rsi = r15;
            // bc095 call 0xebd74
            { let r = { let t = m.env.stub_va(0x541); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xbc09a;
        }
        0xbc09a => {
            // block bc09a
            // bc09a mov DWORD PTR [rbp-0x24],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rbx as u32 as u64)) as u32);
            // bc09d cmp ebx,0x6c
            fl = sub::<32>((rbx as u32 as u64), 0x6cu64).1;
            // bc0a0 lea eax,[rbx+0x1]
            rax = (rbx.wrapping_add(0x1)) as u32 as u64;
            // bc0a3 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // bc0a5 jb 0xbc080
            if fl.b() { pc = 0xbc080; continue; }
            pc = 0xbc0a7;
        }
        0xbc0a7 => {
            // block bc0a7
            // bc0a7 cmp QWORD PTR [r14+0x10],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x10)) as u64, 0x0u64).1;
            // bc0ac setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // bc0af add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // bc0b3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bc0b4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // bc0b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // bc0b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // bc0ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bc0bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            // db463 mov QWORD PTR [rbx+0x6a8],rax
            m.w64(rbx.wrapping_add(0x6a8), (rax) as u64);
            pc = 0xdb46a;
        }
        0xdb46a => {
            // block db46a
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
            // eb73d mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb741;
        }
        0xeb741 => {
            // block eb741
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
            // eb762 mov r12,QWORD PTR [r14]
            r12 = m.r64(r14) as u64;
            // eb765 jmp 0xeb76b
            pc = 0xeb76b; continue;
        }
        0xeb767 => {
            // block eb767
            // eb767 mov rdi,QWORD PTR [r15+0x8]
            rdi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0xeb76b;
        }
        0xeb76b => {
            // block eb76b
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
            // ea4c3 mov r15,rax
            r15 = rax;
            // ea4c6 mov rsi,QWORD PTR [r13+0x0]
            rsi = m.r64(r13) as u64;
            // ea4ca jmp 0xea4d7
            pc = 0xea4d7; continue;
        }
        0xea4cc => {
            // block ea4cc
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
            // b94ba cmp rbx,r13
            fl = sub::<64>(rbx, r13).1;
            // b94bd je 0xb94d5
            if fl.e() { pc = 0xb94d5; continue; }
            pc = 0xb94bf;
        }
        0xb94bf => {
            // block b94bf
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
            // b94d3 jmp 0xb94eb
            pc = 0xb94eb; continue;
        }
        0xb94d5 => {
            // block b94d5
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
            // b94dc mov rsi,rbx
            rsi = rbx;
            pc = 0xb94df;
        }
        0xb94df => {
            // block b94df
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
            // b94eb mov sil,BYTE PTR [rbx+0x19]
            rsi = (rsi & !0xff) | ((m.r8(rbx.wrapping_add(0x19)) as u64) as u8 as u64);
            // b94ef mov rbx,rcx
            rbx = rcx;
            pc = 0xb94f2;
        }
        0xb94f2 => {
            // block b94f2
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
            // b9528 jmp 0xb952c
            pc = 0xb952c; continue;
        }
        0xb952a => {
            // block b952a
            // b952a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb952c;
        }
        0xb952c => {
            // block b952c
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
            // b5001 mov rbx,rax
            rbx = rax;
            // b5004 jmp 0xb502a
            pc = 0xb502a; continue;
        }
        0xb5006 => {
            // block b5006
            // b5006 mov rbx,rax
            rbx = rax;
            // b5009 jmp 0xb5075
            pc = 0xb5075; continue;
        }
        0xb500b => {
            // block b500b
            // b500b mov rbx,rax
            rbx = rax;
            // b500e jmp 0xb505e
            pc = 0xb505e; continue;
        }
        0xb5010 => {
            // block b5010
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

/// parity: P2v @0xb4896 size=1292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b4896<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb4896;
    loop {
        match pc {
        0xb4896 => {
            // block b4896
            // b4896 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b4897 mov rbp,rsp
            rbp = rsp;
            // b489a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b489c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b489e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b48a0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b48a2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b48a3 sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // b48aa mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // b48ad mov DWORD PTR [rbp-0x16c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe94), ((rcx as u32 as u64)) as u32);
            // b48b3 mov QWORD PTR [rbp-0x178],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rdi) as u64);
            // b48ba lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b48c1 lea r12,[rbp-0x160]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b48c8 mov rax,QWORD PTR [rip+0x8b901] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b48cf lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b48d3 mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // b48da add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b48de mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b48e5 mov rsi,r12
            rsi = r12;
            // b48e8 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b48ed mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // b48f5 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // b48fc mov r13,QWORD PTR [rip+0x8babd] # 0x1403c0
            r13 = m.r64(m.env.data_va(347021)) as u64;
            // b4903 lea rbx,[r13+0x18]
            rbx = r13.wrapping_add(0x18);
            // b4907 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // b490e add r13,0x40
            { let (r, f) = add::<64>(r13, 0x40u64);
              r13 = r;
              fl = f; }
            // b4912 mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b4919 mov rdi,r12
            rdi = r12;
            // b491c call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4921 mov QWORD PTR [rbp-0x180],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rbx) as u64);
            // b4928 mov rax,QWORD PTR [rip+0x8ba79] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // b492f add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // b4933 mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // b493a mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b4941 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b494c mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b4957 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // b4962 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // b496d mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // b4977 xorps xmm0,xmm0
            x0 = 0;
            // b497a movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // b497e mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b4986 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b498a mov rdi,r12
            rdi = r12;
            // b498d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4992 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b4996 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b499b mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // b499e call 0xcd01e
            { let r = { let t = m.env.code_va(0xcd01e); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b49a3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b49a5 je 0xb4c45
            if fl.e() { pc = 0xb4c45; continue; }
            pc = 0xb49ab;
        }
        0xb49ab => {
            // block b49ab
            // b49ab lea rsi,[rip+0x8094c] # 0x1352fe
            rsi = m.env.data_va(301771);
            // b49b2 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b49b9 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // b49be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b49c3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b49c6 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b49ca mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b49ce and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b49d1 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b49d4 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b49d8 mov esi,DWORD PTR [rbp-0x16c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b49de and esi,0x7fff
            { let r = (rsi as u32 as u64) & 0x7fffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b49e4 mov rdi,rax
            rdi = rax;
            // b49e7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b49ec mov rbx,rax
            rbx = rax;
            // b49ef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b49f2 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b49f6 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b49f9 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b49fd call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a02 mov rsi,QWORD PTR [rip+0x8b787] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4a09 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b4a0d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a12 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4a15 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4a19 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b4a1e mov rdi,rax
            rdi = rax;
            // b4a21 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a23 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4a26 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b4a2a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a2f movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b4a33 mov rdi,rbx
            rdi = rbx;
            // b4a36 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a3b mov rdi,rbx
            rdi = rbx;
            // b4a3e call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a43 lea rsi,[rip+0x808c1] # 0x13530b
            rsi = m.env.data_va(301784);
            // b4a4a mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // b4a4f mov rdi,rbx
            rdi = rbx;
            // b4a52 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a57 lea rcx,[rip+0x761c4] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b4a5e lea r15,[rip+0x7f2a0] # 0x133d05
            r15 = m.env.data_va(296146);
            // b4a65 test DWORD PTR [rbp-0x16c],0x10000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64 & 0x10000u64);
            // b4a6f mov rsi,r15
            rsi = r15;
            // b4a72 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b4a76 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b4a7b mov rdi,rax
            rdi = rax;
            // b4a7e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a83 mov rbx,rax
            rbx = rax;
            // b4a86 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b4a89 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4a8d add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b4a90 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b4a94 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4a99 mov r14,r12
            r14 = r12;
            // b4a9c mov rsi,QWORD PTR [rip+0x8b6ed] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4aa3 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b4aa7 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4aac mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4aaf mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4ab3 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b4ab8 mov rdi,rax
            rdi = rax;
            // b4abb call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4abd mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4ac0 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b4ac4 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ac9 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b4acd mov rdi,rbx
            rdi = rbx;
            // b4ad0 mov r12,r14
            r12 = r14;
            // b4ad3 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ad8 mov rdi,rbx
            rdi = rbx;
            // b4adb call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ae0 lea rsi,[rip+0x8082d] # 0x135314
            rsi = m.env.data_va(301793);
            // b4ae7 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // b4aec mov rdi,rbx
            rdi = rbx;
            // b4aef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4af4 test DWORD PTR [rbp-0x16c],0x100000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64 & 0x100000u64);
            // b4afe mov rsi,r15
            rsi = r15;
            // b4b01 lea rcx,[rip+0x7611a] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b4b08 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b4b0c mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b4b11 mov rdi,rax
            rdi = rax;
            // b4b14 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b19 mov rbx,rax
            rbx = rax;
            // b4b1c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b4b1f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4b23 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b4b26 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b4b2a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b2f mov r14,r12
            r14 = r12;
            // b4b32 mov rsi,QWORD PTR [rip+0x8b657] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4b39 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b4b3d call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b42 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4b45 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4b49 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b4b4e mov rdi,rax
            rdi = rax;
            // b4b51 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b53 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4b56 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b4b5a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b5f movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b4b63 mov rdi,rbx
            rdi = rbx;
            // b4b66 mov r12,r14
            r12 = r14;
            // b4b69 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b6e mov rdi,rbx
            rdi = rbx;
            // b4b71 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b76 lea rsi,[rip+0x807aa] # 0x135327
            rsi = m.env.data_va(301812);
            // b4b7d mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // b4b82 mov rdi,rbx
            rdi = rbx;
            // b4b85 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4b8a test DWORD PTR [rbp-0x16c],0x200000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64 & 0x200000u64);
            // b4b94 mov rsi,r15
            rsi = r15;
            // b4b97 lea rcx,[rip+0x76084] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b4b9e cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b4ba2 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b4ba7 mov rdi,rax
            rdi = rax;
            // b4baa call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4baf mov rbx,rax
            rbx = rax;
            // b4bb2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b4bb5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b4bb9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b4bbc lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4bc0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4bc5 mov r14,r12
            r14 = r12;
            // b4bc8 mov rsi,QWORD PTR [rip+0x8b5c1] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b4bcf lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4bd3 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4bd8 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b4bdb mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b4bdf mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b4be4 mov rdi,rax
            rdi = rax;
            // b4be7 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4be9 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b4bec lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4bf0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4bf5 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // b4bf9 mov rdi,rbx
            rdi = rbx;
            // b4bfc mov r12,r14
            r12 = r14;
            // b4bff call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c04 mov rdi,rbx
            rdi = rbx;
            // b4c07 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c0c lea rsi,[rip+0x80727] # 0x13533a
            rsi = m.env.data_va(301831);
            // b4c13 mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b4c18 mov rdi,rbx
            rdi = rbx;
            // b4c1b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c20 test DWORD PTR [rbp-0x16c],0x1000000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64 & 0x1000000u64);
            // b4c2a lea rcx,[rip+0x75ff1] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b4c31 cmovne r15,rcx
            if fl.ne() { r15 = rcx; }
            // b4c35 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b4c3a mov rdi,rax
            rdi = rax;
            // b4c3d mov rsi,r15
            rsi = r15;
            // b4c40 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4c45;
        }
        0xb4c45 => {
            // block b4c45
            // b4c45 mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b4c4c mov rsi,r12
            rsi = r12;
            // b4c4f call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c54 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b4c5b mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b4c62 mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b4c69 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // b4c70 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b4c77 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b4c7e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c83 mov rdi,r12
            rdi = r12;
            // b4c86 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4c8b mov rsi,QWORD PTR [rip+0x8b576] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b4c92 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4c96 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b4c9d call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4ca2 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b4ca9 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4cae mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b4cb5 add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // b4cbc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b4cbd pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b4cbf pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b4cc1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b4cc3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b4cc5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b4cc6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb4cc7 => {
            // block b4cc7
            // b4cc7 mov r14,r12
            r14 = r12;
            // b4cca mov rbx,rax
            rbx = rax;
            pc = 0xb4ccd;
        }
        0xb4ccd => {
            // block b4ccd
            // b4ccd mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b4cd4 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b4cdb mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b4ce2 mov rax,QWORD PTR [rbp-0x188]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64;
            // b4ce9 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b4cf0 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b4cf7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4cfc mov rdi,r14
            rdi = r14;
            // b4cff call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4d04;
        }
        0xb4d04 => {
            // block b4d04
            // b4d04 mov rsi,QWORD PTR [rip+0x8b4fd] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b4d0b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b4d0f lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b4d16 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4d1b;
        }
        0xb4d1b => {
            // block b4d1b
            // b4d1b lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b4d22 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d27 mov rdi,rbx
            rdi = rbx;
            // b4d2a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d2f mov rbx,rax
            rbx = rax;
            // b4d32 jmp 0xb4d1b
            pc = 0xb4d1b; continue;
        }
        0xb4d34 => {
            // block b4d34
            // b4d34 mov rbx,rax
            rbx = rax;
            // b4d37 jmp 0xb4d04
            pc = 0xb4d04; continue;
        }
        0xb4d39 => {
            // block b4d39
            // b4d39 mov rbx,rax
            rbx = rax;
            // b4d3c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b4d40 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d45 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b4d4c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d51 mov rdi,r12
            rdi = r12;
            // b4d54 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d59 jmp 0xb4d04
            pc = 0xb4d04; continue;
        }
        0xb4d5b => {
            // block b4d5b
            // b4d5b mov r14,r12
            r14 = r12;
            // b4d5e mov rbx,rax
            rbx = rax;
            // b4d61 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b4d65 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d6a jmp 0xb4ccd
            pc = 0xb4ccd; continue;
        }
        0xb4d6f => {
            // block b4d6f
            // b4d6f mov rbx,rax
            rbx = rax;
            // b4d72 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b4d76 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d7b jmp 0xb4ccd
            pc = 0xb4ccd; continue;
        }
        0xb4d80 => {
            // block b4d80
            // b4d80 mov rbx,rax
            rbx = rax;
            // b4d83 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b4d87 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d8c jmp 0xb4ccd
            pc = 0xb4ccd; continue;
        }
        0xb4d91 => {
            // block b4d91
            // b4d91 mov rbx,rax
            rbx = rax;
            // b4d94 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b4d98 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4d9d jmp 0xb4ccd
            pc = 0xb4ccd; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb11c0 size=1640
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b11c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb11c0;
    loop {
        match pc {
        0xb11c0 => {
            // block b11c0
            // b11c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b11c1 mov rbp,rsp
            rbp = rsp;
            // b11c4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b11c6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b11c8 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b11ca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b11cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b11cd sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b11d4 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // b11d7 mov DWORD PTR [rbp-0x16c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe94), ((rcx as u32 as u64)) as u32);
            // b11dd mov QWORD PTR [rbp-0x178],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rdi) as u64);
            // b11e4 mov al,BYTE PTR [rip+0x97786] # 0x148970
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(381245)) as u64) as u8 as u64);
            // b11ea test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b11ec jne 0xb1293
            if fl.ne() { pc = 0xb1293; continue; }
            pc = 0xb11f2;
        }
        0xb11f2 => {
            // block b11f2
            // b11f2 lea rdi,[rip+0x97777] # 0x148970
            rdi = m.env.data_va(381245);
            // b11f9 call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b11fe test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // b1200 je 0xb1293
            if fl.e() { pc = 0xb1293; continue; }
            pc = 0xb1206;
        }
        0xb1206 => {
            // block b1206
            // b1206 lea r12,[rip+0x97703] # 0x148910
            r12 = m.env.data_va(381149);
            // b120d lea rsi,[rip+0x83d0c] # 0x134f20
            rsi = m.env.data_va(300781);
            // b1214 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // b1219 mov rdi,r12
            rdi = r12;
            // b121c call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1221 lea rbx,[rip+0x97700] # 0x148928
            rbx = m.env.data_va(381173);
            // b1228 lea rsi,[rip+0x83d00] # 0x134f2f
            rsi = m.env.data_va(300796);
            // b122f mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // b1234 mov rdi,rbx
            rdi = rbx;
            // b1237 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b123c lea rbx,[rip+0x976fd] # 0x148940
            rbx = m.env.data_va(381197);
            // b1243 lea rsi,[rip+0x83cf4] # 0x134f3e
            rsi = m.env.data_va(300811);
            // b124a mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // b124f mov rdi,rbx
            rdi = rbx;
            // b1252 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1257 lea rbx,[rip+0x976fa] # 0x148958
            rbx = m.env.data_va(381221);
            // b125e lea rsi,[rip+0x83ce7] # 0x134f4c
            rsi = m.env.data_va(300825);
            // b1265 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b126a mov rdi,rbx
            rdi = rbx;
            // b126d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1272 lea rdi,[rip+0xfffffffffffb7543] # 0x687bc
            rdi = m.env.code_va(0x687bc);
            // b1279 lea rdx,[rip+0xfffffffffff4def0] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // b1280 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b1282 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1287 lea rdi,[rip+0x976e2] # 0x148970
            rdi = m.env.data_va(381245);
            // b128e call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb1293;
        }
        0xb1293 => {
            // block b1293
            // b1293 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b129a lea r12,[rbp-0x160]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b12a1 mov rax,QWORD PTR [rip+0x8ef28] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b12a8 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b12ac mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // b12b3 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b12b7 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b12be mov rsi,r12
            rsi = r12;
            // b12c1 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b12c6 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // b12ce mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // b12d5 mov r13,QWORD PTR [rip+0x8f0e4] # 0x1403c0
            r13 = m.r64(m.env.data_va(347021)) as u64;
            // b12dc lea rbx,[r13+0x18]
            rbx = r13.wrapping_add(0x18);
            // b12e0 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // b12e7 add r13,0x40
            { let (r, f) = add::<64>(r13, 0x40u64);
              r13 = r;
              fl = f; }
            // b12eb mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b12f2 mov rdi,r12
            rdi = r12;
            // b12f5 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b12fa mov QWORD PTR [rbp-0x180],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rbx) as u64);
            // b1301 mov r14,QWORD PTR [rip+0x8f0a0] # 0x1403a8
            r14 = m.r64(m.env.data_va(346997)) as u64;
            // b1308 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // b130c mov QWORD PTR [rbp-0x160],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r14) as u64);
            // b1313 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b131e mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b1329 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // b1334 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // b133f mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // b1349 xorps xmm0,xmm0
            x0 = 0;
            // b134c movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // b1350 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b1358 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b135c mov rdi,r12
            rdi = r12;
            // b135f call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1364 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b1368 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b136d mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // b1370 call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1375 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b1377 je 0xb16bc
            if fl.e() { pc = 0xb16bc; continue; }
            pc = 0xb137d;
        }
        0xb137d => {
            // block b137d
            // b137d lea rsi,[rip+0x83bcc] # 0x134f50
            rsi = m.env.data_va(300829);
            // b1384 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b138b mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // b1390 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1395 lea rdx,[rip+0x79886] # 0x12ac22
            rdx = m.env.data_va(259055);
            // b139c lea r12,[rip+0x82962] # 0x133d05
            r12 = m.env.data_va(296146);
            // b13a3 mov ecx,DWORD PTR [rbp-0x16c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b13a9 test cl,0x40
            fl = logic::<8>((rcx as u8 as u64) & 0x40u64);
            // b13ac mov rsi,r12
            rsi = r12;
            // b13af cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // b13b3 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b13b8 mov rdi,rax
            rdi = rax;
            // b13bb call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b13c0 mov rbx,rax
            rbx = rax;
            // b13c3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b13c6 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b13ca add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b13cd lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b13d1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b13d6 mov rsi,QWORD PTR [rip+0x8edb3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b13dd lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b13e1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b13e6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b13e9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b13ed mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b13f2 mov rdi,rax
            rdi = rax;
            // b13f5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b13f7 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b13fa lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b13fe call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1403 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b1407 mov rdi,rbx
            rdi = rbx;
            // b140a call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b140f mov rdi,rbx
            rdi = rbx;
            // b1412 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1417 lea rsi,[rip+0x83b52] # 0x134f70
            rsi = m.env.data_va(300861);
            // b141e mov edx,0x16
            rdx = (0x16u64) as u32 as u64;
            // b1423 mov rdi,rbx
            rdi = rbx;
            // b1426 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b142b mov ecx,DWORD PTR [rbp-0x16c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b1431 test cl,0x80
            fl = logic::<8>((rcx as u8 as u64) & 0x80u64);
            // b1434 mov rsi,r12
            rsi = r12;
            // b1437 lea rcx,[rip+0x797e4] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b143e cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b1442 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b1447 mov rdi,rax
            rdi = rax;
            // b144a call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b144f mov rbx,rax
            rbx = rax;
            // b1452 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b1455 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1459 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b145c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b1460 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1465 mov rsi,QWORD PTR [rip+0x8ed24] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b146c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b1470 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1475 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1478 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b147c mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b1481 mov rdi,rax
            rdi = rax;
            // b1484 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1486 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b1489 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b148d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1492 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b1496 mov rdi,rbx
            rdi = rbx;
            // b1499 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b149e mov rdi,rbx
            rdi = rbx;
            // b14a1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b14a6 lea rsi,[rip+0x83ada] # 0x134f87
            rsi = m.env.data_va(300884);
            // b14ad mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // b14b2 mov rdi,rbx
            rdi = rbx;
            // b14b5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b14ba mov ecx,DWORD PTR [rbp-0x16c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b14c0 test cl,0x1
            fl = logic::<8>((rcx as u8 as u64) & 0x1u64);
            // b14c3 lea rcx,[rip+0x79758] # 0x12ac22
            rcx = m.env.data_va(259055);
            // b14ca cmovne r12,rcx
            if fl.ne() { r12 = rcx; }
            // b14ce mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b14d3 mov rdi,rax
            rdi = rax;
            // b14d6 mov rsi,r12
            rsi = r12;
            // b14d9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b14de mov rbx,rax
            rbx = rax;
            // b14e1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b14e4 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b14e8 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b14eb lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b14ef call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b14f4 mov rsi,QWORD PTR [rip+0x8ec95] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b14fb lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b14ff call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1504 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1507 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b150b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b1510 mov rdi,rax
            rdi = rax;
            // b1513 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1515 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b1518 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b151c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1521 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b1525 mov rdi,rbx
            rdi = rbx;
            // b1528 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b152d mov rdi,rbx
            rdi = rbx;
            // b1530 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1535 lea rsi,[rip+0x83a60] # 0x134f9c
            rsi = m.env.data_va(300905);
            // b153c mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // b1541 mov rdi,rbx
            rdi = rbx;
            // b1544 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1549 mov ecx,DWORD PTR [rbp-0x16c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b154f shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // b1552 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // b1555 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // b1558 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b155d cmovb rdx,rcx
            if fl.b() { rdx = rcx; }
            // b1561 lea rcx,[rdx+rdx*2]
            rcx = rdx.wrapping_add(rdx.wrapping_mul(2));
            // b1565 lea rdx,[rip+0x973a4] # 0x148910
            rdx = m.env.data_va(381149);
            // b156c lea rsi,[rdx+rcx*8]
            rsi = rdx.wrapping_add(rcx.wrapping_mul(8));
            // b1570 mov rdi,rax
            rdi = rax;
            // b1573 call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1578 mov rbx,rax
            rbx = rax;
            // b157b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b157e mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1582 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b1585 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b1589 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b158e mov rsi,QWORD PTR [rip+0x8ebfb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b1595 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b1599 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b159e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b15a1 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b15a5 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b15aa mov rdi,rax
            rdi = rax;
            // b15ad call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b15af mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b15b2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b15b6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b15bb movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b15bf mov rdi,rbx
            rdi = rbx;
            // b15c2 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b15c7 mov rdi,rbx
            rdi = rbx;
            // b15ca call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b15cf lea rsi,[rip+0x839cd] # 0x134fa3
            rsi = m.env.data_va(300912);
            // b15d6 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // b15db mov rdi,rbx
            rdi = rbx;
            // b15de call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b15e3 mov esi,DWORD PTR [rbp-0x16c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe94)) as u64) as u32 as u64;
            // b15e9 shr esi,0x18
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x18u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // b15ec mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b15ef mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b15f3 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b15f7 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b15fa or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b15fd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b1601 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1604 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1608 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b1610 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1613 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1617 mov QWORD PTR [rcx+rax*1+0x18],0x2
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x2u64) as u64);
            // b1620 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1623 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1627 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b1632 movzx ebx,si
            rbx = ((rsi as u16 as u64)) as u32 as u64;
            // b1635 mov rdi,rax
            rdi = rax;
            // b1638 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b163a call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b163f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1642 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1646 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b164a and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b164d or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1650 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b1654 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1657 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b165b mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b1666 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1669 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b166d and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b1675 lea rsi,[rip+0x828eb] # 0x133f67
            rsi = m.env.data_va(296756);
            // b167c mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // b1681 mov rdi,rax
            rdi = rax;
            // b1684 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1689 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b168c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1690 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b1694 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1697 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b169a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b169e mov rdi,rax
            rdi = rax;
            // b16a1 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b16a3 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16a8 lea rsi,[rip+0x78fd3] # 0x12a682
            rsi = m.env.data_va(257615);
            // b16af mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b16b4 mov rdi,rax
            rdi = rax;
            // b16b7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb16bc;
        }
        0xb16bc => {
            // block b16bc
            // b16bc mov rdi,QWORD PTR [rbp-0x178]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b16c3 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b16ca call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16cf mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b16d6 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b16dd mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b16e4 mov QWORD PTR [rbp-0x160],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r14) as u64);
            // b16eb lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b16f2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16f7 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b16fe call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1703 mov rsi,QWORD PTR [rip+0x8eafe] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b170a add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b170e lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1715 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b171a lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b1721 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1726 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b172d add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b1734 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b1735 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b1737 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b1739 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b173b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b173d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b173e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb173f => {
            // block b173f
            // b173f mov r15,rax
            r15 = rax;
            pc = 0xb1742;
        }
        0xb1742 => {
            // block b1742
            // b1742 mov rax,QWORD PTR [rbp-0x180]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // b1749 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b1750 mov QWORD PTR [rbp-0xf8],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r13) as u64);
            // b1757 mov QWORD PTR [rbp-0x160],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r14) as u64);
            pc = 0xb175e;
        }
        0xb175e => {
            // block b175e
            // b175e lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b1765 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b176a lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b1771 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb1776;
        }
        0xb1776 => {
            // block b1776
            // b1776 mov rsi,QWORD PTR [rip+0x8ea8b] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b177d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b1781 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1788 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb178d;
        }
        0xb178d => {
            // block b178d
            // b178d lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b1794 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb1799;
        }
        0xb1799 => {
            // block b1799
            // b1799 mov rdi,r15
            rdi = r15;
            // b179c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17a1 mov r15,rax
            r15 = rax;
            // b17a4 jmp 0xb178d
            pc = 0xb178d; continue;
        }
        0xb17a6 => {
            // block b17a6
            // b17a6 mov r15,rax
            r15 = rax;
            // b17a9 jmp 0xb1776
            pc = 0xb1776; continue;
        }
        0xb17ab => {
            // block b17ab
            // b17ab mov r15,rax
            r15 = rax;
            // b17ae lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b17b2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17b7 jmp 0xb175e
            pc = 0xb175e; continue;
        }
        0xb17b9 => {
            // block b17b9
            // b17b9 mov r15,rax
            r15 = rax;
            // b17bc lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b17c0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17c5 jmp 0xb1742
            pc = 0xb1742; continue;
        }
        0xb17ca => {
            // block b17ca
            // b17ca mov r15,rax
            r15 = rax;
            // b17cd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b17d1 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17d6 jmp 0xb1742
            pc = 0xb1742; continue;
        }
        0xb17db => {
            // block b17db
            // b17db mov r15,rax
            r15 = rax;
            // b17de lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b17e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17e7 jmp 0xb1742
            pc = 0xb1742; continue;
        }
        0xb17ec => {
            // block b17ec
            // b17ec mov r15,rax
            r15 = rax;
            // b17ef lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b17f3 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17f8 jmp 0xb1742
            pc = 0xb1742; continue;
        }
        0xb17fd => {
            // block b17fd
            // b17fd mov r15,rax
            r15 = rax;
            pc = 0xb1800;
        }
        0xb1800 => {
            // block b1800
            // b1800 add rbx,0xffffffffffffffe8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ffe8u64);
              rbx = r;
              fl = f; }
            // b1804 mov rdi,rbx
            rdi = rbx;
            // b1807 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b180c cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // b180f jne 0xb1800
            if fl.ne() { pc = 0xb1800; continue; }
            pc = 0xb1811;
        }
        0xb1811 => {
            // block b1811
            // b1811 jmp 0xb1816
            pc = 0xb1816; continue;
        }
        0xb1813 => {
            // block b1813
            // b1813 mov r15,rax
            r15 = rax;
            pc = 0xb1816;
        }
        0xb1816 => {
            // block b1816
            // b1816 lea rdi,[rip+0x97153] # 0x148970
            rdi = m.env.data_va(381245);
            // b181d call 0xec16a
            { let r = { let t = m.env.stub_va(0x937); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1822 jmp 0xb1799
            pc = 0xb1799; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb0982 size=2100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b0982<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb0982;
    loop {
        match pc {
        0xb0982 => {
            // block b0982
            // b0982 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b0983 mov rbp,rsp
            rbp = rsp;
            // b0986 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b0988 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b098a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b098c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b098e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b098f sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b0996 mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // b0999 mov DWORD PTR [rbp-0x17c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe84), ((rcx as u32 as u64)) as u32);
            // b099f mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // b09a2 mov r13,rdi
            r13 = rdi;
            // b09a5 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b09ac lea rbx,[rbp-0x160]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b09b3 mov rax,QWORD PTR [rip+0x8f816] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b09ba lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b09be mov QWORD PTR [rbp-0x168],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rcx) as u64);
            // b09c5 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b09c9 mov QWORD PTR [rbp-0xf8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (rax) as u64);
            // b09d0 mov rsi,rbx
            rsi = rbx;
            // b09d3 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b09d8 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // b09e0 mov DWORD PTR [rbp-0x68],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0xffffffffu64) as u32);
            // b09e7 mov r12,QWORD PTR [rip+0x8f9d2] # 0x1403c0
            r12 = m.r64(m.env.data_va(347021)) as u64;
            // b09ee lea rax,[r12+0x18]
            rax = r12.wrapping_add(0x18);
            // b09f3 mov QWORD PTR [rbp-0x170],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rax) as u64);
            // b09fa mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b0a01 add r12,0x40
            { let (r, f) = add::<64>(r12, 0x40u64);
              r12 = r;
              fl = f; }
            // b0a05 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b0a0c mov rdi,rbx
            rdi = rbx;
            // b0a0f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a14 mov rdi,rbx
            rdi = rbx;
            // b0a17 mov rax,QWORD PTR [rip+0x8f98a] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // b0a1e add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // b0a22 mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // b0a29 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b0a30 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // b0a3b mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // b0a46 mov QWORD PTR [rbp-0x118],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (0x0u64) as u64);
            // b0a51 mov QWORD PTR [rbp-0x120],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x0u64) as u64);
            // b0a5c mov DWORD PTR [rbp-0x100],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x10u64) as u32);
            // b0a66 xorps xmm0,xmm0
            x0 = 0;
            // b0a69 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // b0a6d mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b0a75 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b0a79 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a7e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b0a82 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a87 mov edi,r14d
            rdi = ((r14 as u32 as u64)) as u32 as u64;
            // b0a8a call 0xce321
            { let r = { let t = m.env.code_va(0xce321); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0a8f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b0a91 je 0xb106f
            if fl.e() { pc = 0xb106f; continue; }
            pc = 0xb0a97;
        }
        0xb0a97 => {
            // block b0a97
            // b0a97 lea eax,[r15-0x14a]
            rax = (r15.wrapping_add(0xffff_ffff_ffff_feb6)) as u32 as u64;
            // b0a9e cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // b0aa1 jae 0xb0b90
            if fl.ae() { pc = 0xb0b90; continue; }
            pc = 0xb0aa7;
        }
        0xb0aa7 => {
            // block b0aa7
            // b0aa7 mov eax,DWORD PTR [rbp-0x17c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0aad movzx ebx,ax
            rbx = ((rax as u16 as u64)) as u32 as u64;
            // b0ab0 cmp ebx,0xc350
            fl = sub::<32>((rbx as u32 as u64), 0xc350u64).1;
            // b0ab6 ja 0xb0cde
            if fl.a() { pc = 0xb0cde; continue; }
            pc = 0xb0abc;
        }
        0xb0abc => {
            // block b0abc
            // b0abc lea rsi,[rip+0x8440d] # 0x134ed0
            rsi = m.env.data_va(300701);
            // b0ac3 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0aca mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0acf call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ad4 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // b0ad8 mulsd xmm0,QWORD PTR [rip+0x4fcc0] # 0x1007a0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85869)) as u128))).to_bits());
            // b0ae0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ae3 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ae7 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0aec and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0af0 or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0af3 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0af7 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0afa mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0afe mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0b07 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0b0a mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b0e mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0b17 mov rdi,rax
            rdi = rax;
            // b0b1a call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b1f mov rbx,rax
            rbx = rax;
            // b0b22 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0b25 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b29 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0b2d and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0b30 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0b33 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0b37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0b3a mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0b3e add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0b41 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b45 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b4a mov rsi,QWORD PTR [rip+0x8f63f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0b51 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b55 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b5a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0b5d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0b61 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0b66 mov rdi,rax
            rdi = rax;
            // b0b69 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b6b mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0b6e lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b0b72 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b77 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0b7b mov rdi,rbx
            rdi = rbx;
            // b0b7e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b83 mov rdi,rbx
            rdi = rbx;
            // b0b86 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0b8b jmp 0xb0de7
            pc = 0xb0de7; continue;
        }
        0xb0b90 => {
            // block b0b90
            // b0b90 cmp r15d,0x14e
            fl = sub::<32>((r15 as u32 as u64), 0x14eu64).1;
            // b0b97 jne 0xb0f19
            if fl.ne() { pc = 0xb0f19; continue; }
            pc = 0xb0b9d;
        }
        0xb0b9d => {
            // block b0b9d
            // b0b9d lea rsi,[rip+0x8433e] # 0x134ee2
            rsi = m.env.data_va(300719);
            // b0ba4 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0bab mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0bb0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0bb5 mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0bbb shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // b0bbe cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0bc2 mulsd xmm0,QWORD PTR [rip+0x511e6] # 0x101db0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(91517)) as u128))).to_bits());
            // b0bca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0bcd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0bd1 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0bd6 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0bda or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0bdd mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0be1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0be4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0be8 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0bf1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0bf4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0bf8 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0c01 mov rdi,rax
            rdi = rax;
            // b0c04 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c09 mov rbx,rax
            rbx = rax;
            // b0c0c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0c0f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0c13 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0c17 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0c1a or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0c1d mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0c21 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0c24 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0c28 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0c2b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c2f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c34 mov rsi,QWORD PTR [rip+0x8f555] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0c3b lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c3f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c44 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0c47 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0c4b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0c50 mov rdi,rax
            rdi = rax;
            // b0c53 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c55 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0c58 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b0c5c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c61 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0c65 mov rdi,rbx
            rdi = rbx;
            // b0c68 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c6d mov rdi,rbx
            rdi = rbx;
            // b0c70 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c75 lea rsi,[rip+0x8426c] # 0x134ee8
            rsi = m.env.data_va(300725);
            // b0c7c lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0c83 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // b0c88 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0c8d mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0c93 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // b0c96 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0c9a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0c9d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ca1 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0ca6 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0caa or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0cad mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0cb1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cb4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cb8 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0cc1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cc4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cc8 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0cd1 mov rdi,rax
            rdi = rax;
            // b0cd4 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0cd9 jmp 0xb105a
            pc = 0xb105a; continue;
        }
        0xb0cde => {
            // block b0cde
            // b0cde lea rsi,[rip+0x841eb] # 0x134ed0
            rsi = m.env.data_va(300701);
            // b0ce5 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0cec mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0cf1 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0cf6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0cf9 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0cfd mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0d01 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d04 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d07 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0d0b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d0e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d12 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0d1a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d1d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d21 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b0d2a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d2d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d31 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b0d3c mov rdi,rax
            rdi = rax;
            // b0d3f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b0d41 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0d46 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d49 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d4d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0d51 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d54 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d57 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0d5b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d5e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d62 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b0d6d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0d70 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d74 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0d7c lea rsi,[rip+0x84151] # 0x134ed4
            rsi = m.env.data_va(300705);
            // b0d83 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // b0d88 mov rdi,rax
            rdi = rax;
            // b0d8b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0d90 mov rbx,rax
            rbx = rax;
            // b0d93 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0d96 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0d9a add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0d9d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0da1 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0da6 mov rsi,QWORD PTR [rip+0x8f3e3] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0dad lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0db1 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0db6 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0db9 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0dbd mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0dc2 mov rdi,rax
            rdi = rax;
            // b0dc5 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0dc7 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0dca lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b0dce call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0dd3 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0dd7 mov rdi,rbx
            rdi = rbx;
            // b0dda call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ddf mov rdi,rbx
            rdi = rbx;
            // b0de2 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb0de7;
        }
        0xb0de7 => {
            // block b0de7
            // b0de7 mov eax,DWORD PTR [rbp-0x17c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0ded mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // b0def shr ebx,0x10
            { let (r, f) = shr::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // b0df2 cmp eax,0xc350ffff
            fl = sub::<32>((rax as u32 as u64), 0xc350ffffu64).1;
            // b0df7 ja 0xb0e61
            if fl.a() { pc = 0xb0e61; continue; }
            pc = 0xb0df9;
        }
        0xb0df9 => {
            // block b0df9
            // b0df9 lea rsi,[rip+0x840de] # 0x134ede
            rsi = m.env.data_va(300715);
            // b0e00 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0e07 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0e0c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e11 cvtsi2sd xmm0,ebx
            x0 = set_lo64(x0, (sx::<32>((rbx as u32 as u64)) as f64).to_bits());
            // b0e15 mulsd xmm0,QWORD PTR [rip+0x4f983] # 0x1007a0
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(85869)) as u128))).to_bits());
            // b0e1d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e20 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e24 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0e29 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e2d or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e30 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0e34 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e37 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e3b mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0e44 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e47 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e4b mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0e54 mov rdi,rax
            rdi = rax;
            // b0e57 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e5c jmp 0xb105a
            pc = 0xb105a; continue;
        }
        0xb0e61 => {
            // block b0e61
            // b0e61 lea rsi,[rip+0x84076] # 0x134ede
            rsi = m.env.data_va(300715);
            // b0e68 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0e6f mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // b0e74 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0e79 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e7c mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e80 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0e84 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e87 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0e8a mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0e8e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0e91 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0e95 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0e9d mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ea0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ea4 mov QWORD PTR [rcx+rax*1+0x18],0x4
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x4u64) as u64);
            // b0ead mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0eb0 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0eb4 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // b0ebf movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // b0ec2 mov rdi,rax
            rdi = rax;
            // b0ec5 call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0eca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ecd mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ed1 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0ed5 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0ed8 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0edb mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0edf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ee2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ee6 mov DWORD PTR [rcx+rax*1+0x90],0x20
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x20u64) as u32);
            // b0ef1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0ef4 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0ef8 and DWORD PTR [rcx+rax*1+0x8],0xffffbfff
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // b0f00 lea rsi,[rip+0x83fcd] # 0x134ed4
            rsi = m.env.data_va(300705);
            // b0f07 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // b0f0c mov rdi,rax
            rdi = rax;
            // b0f0f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f14 jmp 0xb106f
            pc = 0xb106f; continue;
        }
        0xb0f19 => {
            // block b0f19
            // b0f19 cmp r15d,0x14f
            fl = sub::<32>((r15 as u32 as u64), 0x14fu64).1;
            // b0f20 jne 0xb106f
            if fl.ne() { pc = 0xb106f; continue; }
            pc = 0xb0f26;
        }
        0xb0f26 => {
            // block b0f26
            // b0f26 lea rsi,[rip+0x83fc1] # 0x134eee
            rsi = m.env.data_va(300731);
            // b0f2d lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b0f34 mov edx,0x19
            rdx = (0x19u64) as u32 as u64;
            // b0f39 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f3e mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b0f44 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // b0f47 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b0f4b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f4e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f52 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b0f57 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0f5b or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0f5e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b0f62 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f65 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f69 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b0f72 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0f75 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f79 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b0f82 mov rdi,rax
            rdi = rax;
            // b0f85 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f8a mov rbx,rax
            rbx = rax;
            // b0f8d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0f90 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0f94 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // b0f98 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0f9b or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b0f9e mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b0fa2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0fa5 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b0fa9 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // b0fac lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fb0 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fb5 mov rsi,QWORD PTR [rip+0x8f1d4] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // b0fbc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fc0 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fc5 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b0fc8 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // b0fcc mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // b0fd1 mov rdi,rax
            rdi = rax;
            // b0fd4 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fd6 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b0fd9 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b0fdd call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fe2 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // b0fe6 mov rdi,rbx
            rdi = rbx;
            // b0fe9 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0fee mov rdi,rbx
            rdi = rbx;
            // b0ff1 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0ff6 lea rsi,[rip+0x83f0b] # 0x134f08
            rsi = m.env.data_va(300757);
            // b0ffd lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1004 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // b1009 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b100e mov ecx,DWORD PTR [rbp-0x17c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe84)) as u64) as u32 as u64;
            // b1014 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // b1017 cvtsi2sd xmm0,ecx
            x0 = set_lo64(x0, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // b101b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b101e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1022 mov edx,0xfffffefb
            rdx = (0xfffffefbu64) as u32 as u64;
            // b1027 and edx,DWORD PTR [rcx+rax*1+0x8]
            { let r = (rdx as u32 as u64) & m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b102b or edx,0x4
            { let r = (rdx as u32 as u64) | 0x4u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b102e mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b1032 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1035 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1039 mov QWORD PTR [rcx+rax*1+0x18],0x7
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x7u64) as u64);
            // b1042 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b1045 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1049 mov QWORD PTR [rcx+rax*1+0x10],0x5
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x10), (0x5u64) as u64);
            // b1052 mov rdi,rax
            rdi = rax;
            // b1055 call 0xec092
            { let r = { let t = m.env.stub_va(0x85f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb105a;
        }
        0xb105a => {
            // block b105a
            // b105a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b105d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b1061 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // b1065 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1068 or edx,0x2
            { let r = (rdx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b106b mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            pc = 0xb106f;
        }
        0xb106f => {
            // block b106f
            // b106f mov rdi,r13
            rdi = r13;
            // b1072 lea rsi,[rbp-0x160]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b1079 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b107e mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // b1085 mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b108c mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b1093 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b109a mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            // b10a1 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b10a8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10ad lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b10b4 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10b9 mov rsi,QWORD PTR [rip+0x8f148] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b10c0 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b10c4 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b10cb call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10d0 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b10d7 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b10dc mov rax,r13
            rax = r13;
            // b10df add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // b10e6 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b10e7 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b10e9 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b10eb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b10ed pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b10ef pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b10f0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb10f1 => {
            // block b10f1
            // b10f1 mov rbx,rax
            rbx = rax;
            pc = 0xb10f4;
        }
        0xb10f4 => {
            // block b10f4
            // b10f4 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // b10fb mov QWORD PTR [rbp-0x168],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rax) as u64);
            // b1102 mov QWORD PTR [rbp-0xf8],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (r12) as u64);
            // b1109 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // b1110 mov QWORD PTR [rbp-0x160],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rax) as u64);
            pc = 0xb1117;
        }
        0xb1117 => {
            // block b1117
            // b1117 lea rdi,[rbp-0x120]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // b111e call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1123 lea rdi,[rbp-0x160]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea0);
            // b112a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb112f;
        }
        0xb112f => {
            // block b112f
            // b112f mov rsi,QWORD PTR [rip+0x8f0d2] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b1136 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b113a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // b1141 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb1146;
        }
        0xb1146 => {
            // block b1146
            // b1146 lea rdi,[rbp-0xf8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff08);
            // b114d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1152 mov rdi,rbx
            rdi = rbx;
            // b1155 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b115a mov rbx,rax
            rbx = rax;
            // b115d jmp 0xb1146
            pc = 0xb1146; continue;
        }
        0xb115f => {
            // block b115f
            // b115f mov rbx,rax
            rbx = rax;
            // b1162 jmp 0xb112f
            pc = 0xb112f; continue;
        }
        0xb1164 => {
            // block b1164
            // b1164 mov rbx,rax
            rbx = rax;
            // b1167 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // b116b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1170 jmp 0xb1117
            pc = 0xb1117; continue;
        }
        0xb1172 => {
            // block b1172
            // b1172 mov rbx,rax
            rbx = rax;
            // b1175 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // b1179 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b117e jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb1183 => {
            // block b1183
            // b1183 mov rbx,rax
            rbx = rax;
            // b1186 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b118a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b118f jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb1194 => {
            // block b1194
            // b1194 mov rbx,rax
            rbx = rax;
            // b1197 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // b119b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b11a0 jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
        0xb11a5 => {
            // block b11a5
            // b11a5 mov rbx,rax
            rbx = rax;
            // b11a8 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b11ac call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b11b1 jmp 0xb10f4
            pc = 0xb10f4; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa5d1c size=3354
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a5d1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa5d1c;
    loop {
        match pc {
        0xa5d1c => {
            // block a5d1c
            // a5d1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a5d1d mov rbp,rsp
            rbp = rsp;
            // a5d20 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a5d22 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a5d24 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a5d26 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a5d28 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a5d29 sub rsp,0x1b8
            { let (r, f) = sub::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // a5d30 mov r13d,r8d
            r13 = ((r8 as u32 as u64)) as u32 as u64;
            // a5d33 mov DWORD PTR [rbp-0x1bc],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe44), ((rcx as u32 as u64)) as u32);
            // a5d39 mov QWORD PTR [rbp-0x1c8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe38), (rdi) as u64);
            // a5d40 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // a5d47 lea r14,[rbp-0x1b0]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // a5d4e mov rax,QWORD PTR [rip+0x9a47b] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a5d55 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a5d59 mov QWORD PTR [rbp-0x1b8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rcx) as u64);
            // a5d60 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a5d64 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // a5d6b mov rsi,r14
            rsi = r14;
            // a5d6e call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5d73 mov QWORD PTR [rbp-0xc0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u64);
            // a5d7e mov DWORD PTR [rbp-0xb8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0xffffffffu64) as u32);
            // a5d88 mov rbx,QWORD PTR [rip+0x9a631] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a5d8f lea r15,[rbx+0x18]
            r15 = rbx.wrapping_add(0x18);
            // a5d93 mov QWORD PTR [rbp-0x1b8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (r15) as u64);
            // a5d9a add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a5d9e mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // a5da5 mov rdi,r14
            rdi = r14;
            // a5da8 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5dad mov QWORD PTR [rbp-0x1d8],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe28), (r15) as u64);
            // a5db4 mov QWORD PTR [rbp-0x1d0],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe30), (rbx) as u64);
            // a5dbb mov rax,QWORD PTR [rip+0x9a5e6] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // a5dc2 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // a5dc6 mov QWORD PTR [rbp-0x1e0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe20), (rax) as u64);
            // a5dcd mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // a5dd4 mov QWORD PTR [rbp-0x158],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (0x0u64) as u64);
            // a5ddf mov QWORD PTR [rbp-0x160],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (0x0u64) as u64);
            // a5dea mov QWORD PTR [rbp-0x168],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (0x0u64) as u64);
            // a5df5 mov QWORD PTR [rbp-0x170],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (0x0u64) as u64);
            // a5e00 mov DWORD PTR [rbp-0x150],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (0x10u64) as u32);
            // a5e0a xorps xmm0,xmm0
            x0 = 0;
            // a5e0d movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // a5e14 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // a5e1f lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // a5e26 mov rdi,r14
            rdi = r14;
            // a5e29 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5e2e lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // a5e35 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5e3a lea rsi,[rip+0x8e21f] # 0x134060
            rsi = m.env.data_va(297005);
            // a5e41 lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a5e48 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // a5e4d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5e52 lea rsi,[rip+0x8461c] # 0x12a475
            rsi = m.env.data_va(257090);
            // a5e59 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // a5e5e mov rdi,rax
            rdi = rax;
            // a5e61 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5e66 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5e69 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5e6d mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a5e71 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a5e74 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a5e77 mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a5e7b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5e7e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5e82 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a5e8a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5e8d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5e91 mov QWORD PTR [rcx+rax*1+0x18],0x1
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x1u64) as u64);
            // a5e9a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5e9d mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5ea1 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // a5eac mov esi,DWORD PTR [rbp-0x1bc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a5eb2 and esi,0xf
            { let r = (rsi as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // a5eb5 mov rdi,rax
            rdi = rax;
            // a5eb8 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5ebd mov rbx,rax
            rbx = rax;
            // a5ec0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5ec3 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5ec7 mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // a5ecb and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a5ece or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a5ed1 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // a5ed5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5ed8 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5edc mov DWORD PTR [rax+rbx*1+0x90],0x20
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x20u64) as u32);
            // a5ee7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5eea mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5eee and DWORD PTR [rax+rbx*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a5ef6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5ef9 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5efd add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a5f00 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // a5f07 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f0c mov rsi,QWORD PTR [rip+0x9a27d] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a5f13 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // a5f1a call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f1f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5f22 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a5f26 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a5f2b mov rdi,rax
            rdi = rax;
            // a5f2e call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f30 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a5f33 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // a5f3a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f3f movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a5f43 mov rdi,rbx
            rdi = rbx;
            // a5f46 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f4b mov rdi,rbx
            rdi = rbx;
            // a5f4e call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f53 lea rsi,[rip+0x8e117] # 0x134071
            rsi = m.env.data_va(297022);
            // a5f5a mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // a5f5f mov rdi,rbx
            rdi = rbx;
            // a5f62 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f67 lea rsi,[rip+0x84507] # 0x12a475
            rsi = m.env.data_va(257090);
            // a5f6e mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // a5f73 mov rdi,rax
            rdi = rax;
            // a5f76 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5f7b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5f7e mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5f82 mov edx,DWORD PTR [rcx+rax*1+0x8]
            rdx = (m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64) as u32 as u64;
            // a5f86 and edx,0xffffffb5
            { let r = (rdx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a5f89 or edx,0x8
            { let r = (rdx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a5f8c mov DWORD PTR [rcx+rax*1+0x8],edx
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // a5f90 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5f93 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5f97 or DWORD PTR [rcx+rax*1+0x8],0x4000
            { let r = m.r32(rcx.wrapping_add(rax).wrapping_add(0x8)) as u64 | 0x4000u64; let f = logic::<32>(r);
              m.w32(rcx.wrapping_add(rax).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a5f9f mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5fa2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5fa6 mov QWORD PTR [rcx+rax*1+0x18],0x2
            m.w64(rcx.wrapping_add(rax).wrapping_add(0x18), (0x2u64) as u64);
            // a5faf mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5fb2 mov rcx,QWORD PTR [rcx-0x18]
            rcx = m.r64(rcx.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5fb6 mov DWORD PTR [rcx+rax*1+0x90],0x30
            m.w32(rcx.wrapping_add(rax).wrapping_add(0x90), (0x30u64) as u32);
            // a5fc1 mov esi,DWORD PTR [rbp-0x1bc]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a5fc7 and esi,0xff0
            { let r = (rsi as u32 as u64) & 0xff0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // a5fcd mov rdi,rax
            rdi = rax;
            // a5fd0 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5fd5 mov rbx,rax
            rbx = rax;
            // a5fd8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5fdb mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5fdf mov ecx,DWORD PTR [rax+rbx*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64) as u32 as u64;
            // a5fe3 and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a5fe6 or ecx,0x2
            { let r = (rcx as u32 as u64) | 0x2u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a5fe9 mov DWORD PTR [rax+rbx*1+0x8],ecx
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // a5fed mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a5ff0 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5ff4 mov DWORD PTR [rax+rbx*1+0x90],0x20
            m.w32(rax.wrapping_add(rbx).wrapping_add(0x90), (0x20u64) as u32);
            // a5fff mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6002 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6006 and DWORD PTR [rax+rbx*1+0x8],0xffffbfff
            { let r = m.r32(rax.wrapping_add(rbx).wrapping_add(0x8)) as u64 & 0xffffbfffu64; let f = logic::<32>(r);
              m.w32(rax.wrapping_add(rbx).wrapping_add(0x8), (r) as u32);
              fl = f; }
            // a600e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6011 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6015 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a6018 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // a601f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6024 mov rsi,QWORD PTR [rip+0x9a165] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a602b lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // a6032 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6037 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a603a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a603e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6043 mov rdi,rax
            rdi = rax;
            // a6046 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6048 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a604b lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // a6052 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6057 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a605b mov rdi,rbx
            rdi = rbx;
            // a605e call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6063 mov rdi,rbx
            rdi = rbx;
            // a6066 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a606b mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // a606e call 0xd0960
            { let r = { let t = m.env.code_va(0xd0960); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6073 test ax,ax
            fl = logic::<16>((rax as u16 as u64) & (rax as u16 as u64));
            // a6076 je 0xa61c5
            if fl.e() { pc = 0xa61c5; continue; }
            pc = 0xa607c;
        }
        0xa607c => {
            // block a607c
            // a607c lea rsi,[rip+0x8dffd] # 0x134080
            rsi = m.env.data_va(297037);
            // a6083 lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a608a mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // a608f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6094 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a609a mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a609c and edx,0x8000
            { let r = (rdx as u32 as u64) & 0x8000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a60a2 lea r12,[rip+0x8dfed] # 0x134096
            r12 = m.env.data_va(297059);
            // a60a9 lea r14,[rip+0x8dfed] # 0x13409d
            r14 = m.env.data_va(297066);
            // a60b0 mov rsi,r14
            rsi = r14;
            // a60b3 cmovne rsi,r12
            if fl.ne() { rsi = r12; }
            // a60b7 shr rdx,0xe
            { let (r, f) = shr::<64>(rdx, 0xeu64, fl); rdx = r; fl = f; }
            // a60bb xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a60bf add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a60c3 mov rdi,rax
            rdi = rax;
            // a60c6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a60cb mov rbx,rax
            rbx = rax;
            // a60ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a60d1 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a60d5 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a60d8 lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // a60df call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a60e4 mov r15d,r13d
            r15 = ((r13 as u32 as u64)) as u32 as u64;
            // a60e7 mov rsi,QWORD PTR [rip+0x9a0a2] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a60ee lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // a60f5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a60fa mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a60fd mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a6101 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6106 mov rdi,rax
            rdi = rax;
            // a6109 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a610b mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a610e lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // a6115 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a611a movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a611e mov rdi,rbx
            rdi = rbx;
            // a6121 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6126 mov r13d,r15d
            r13 = ((r15 as u32 as u64)) as u32 as u64;
            // a6129 mov rdi,rbx
            rdi = rbx;
            // a612c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6131 lea rsi,[rip+0x8df6e] # 0x1340a6
            rsi = m.env.data_va(297075);
            // a6138 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // a613d mov rdi,rbx
            rdi = rbx;
            // a6140 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6145 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a614b mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a614d and edx,0x1000000
            { let r = (rdx as u32 as u64) & 0x1000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a6153 cmovne r14,r12
            if fl.ne() { r14 = r12; }
            // a6157 shr rdx,0x17
            { let (r, f) = shr::<64>(rdx, 0x17u64, fl); rdx = r; fl = f; }
            // a615b xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a615f add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a6163 mov rdi,rax
            rdi = rax;
            // a6166 mov rsi,r14
            rsi = r14;
            // a6169 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a616e mov rbx,rax
            rbx = rax;
            // a6171 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6174 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6178 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a617b lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // a617f call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6184 mov rsi,QWORD PTR [rip+0x9a005] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a618b lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // a618f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6194 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a6197 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a619b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a61a0 mov rdi,rax
            rdi = rax;
            // a61a3 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a61a5 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a61a8 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // a61ac call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a61b1 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a61b5 mov rdi,rbx
            rdi = rbx;
            // a61b8 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a61bd mov rdi,rbx
            rdi = rbx;
            // a61c0 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa61c5;
        }
        0xa61c5 => {
            // block a61c5
            // a61c5 mov edi,r13d
            rdi = ((r13 as u32 as u64)) as u32 as u64;
            // a61c8 call 0xd0960
            { let r = { let t = m.env.code_va(0xd0960); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a61cd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // a61d0 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // a61d3 jb 0xa627c
            if fl.b() { pc = 0xa627c; continue; }
            pc = 0xa61d9;
        }
        0xa61d9 => {
            // block a61d9
            // a61d9 lea rsi,[rip+0x8dedc] # 0x1340bc
            rsi = m.env.data_va(297097);
            // a61e0 lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a61e7 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // a61ec call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a61f1 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a61f7 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a61f9 and edx,0x4000000
            { let r = (rdx as u32 as u64) & 0x4000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a61ff lea rcx,[rip+0x8de90] # 0x134096
            rcx = m.env.data_va(297059);
            // a6206 lea rsi,[rip+0x8de90] # 0x13409d
            rsi = m.env.data_va(297066);
            // a620d cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a6211 shr rdx,0x19
            { let (r, f) = shr::<64>(rdx, 0x19u64, fl); rdx = r; fl = f; }
            // a6215 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a6219 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a621d mov rdi,rax
            rdi = rax;
            // a6220 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6225 mov rbx,rax
            rbx = rax;
            // a6228 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a622b mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a622f add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a6232 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // a6236 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a623b mov rsi,QWORD PTR [rip+0x99f4e] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a6242 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // a6246 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a624b mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a624e mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a6252 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6257 mov rdi,rax
            rdi = rax;
            // a625a call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a625c mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a625f lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // a6263 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6268 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a626c mov rdi,rbx
            rdi = rbx;
            // a626f call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6274 mov rdi,rbx
            rdi = rbx;
            // a6277 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa627c;
        }
        0xa627c => {
            // block a627c
            // a627c lea rsi,[rip+0x8de4f] # 0x1340d2
            rsi = m.env.data_va(297119);
            // a6283 lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a628a mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // a628f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6294 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a629a mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a629c and edx,0x100000
            { let r = (rdx as u32 as u64) & 0x100000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a62a2 lea rcx,[rip+0x8dded] # 0x134096
            rcx = m.env.data_va(297059);
            // a62a9 lea r15,[rip+0x8dded] # 0x13409d
            r15 = m.env.data_va(297066);
            // a62b0 mov rsi,r15
            rsi = r15;
            // a62b3 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a62b7 shr rdx,0x13
            { let (r, f) = shr::<64>(rdx, 0x13u64, fl); rdx = r; fl = f; }
            // a62bb xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a62bf add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a62c3 mov rdi,rax
            rdi = rax;
            // a62c6 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a62cb mov rbx,rax
            rbx = rax;
            // a62ce mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a62d1 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a62d5 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a62d8 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // a62dc call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a62e1 mov rsi,QWORD PTR [rip+0x99ea8] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a62e8 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // a62ec call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a62f1 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a62f4 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a62f8 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a62fd mov rdi,rax
            rdi = rax;
            // a6300 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6302 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a6305 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // a6309 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a630e movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a6312 mov rdi,rbx
            rdi = rbx;
            // a6315 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a631a mov rdi,rbx
            rdi = rbx;
            // a631d call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6322 lea rsi,[rip+0x8ddc2] # 0x1340eb
            rsi = m.env.data_va(297144);
            // a6329 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // a632e mov rdi,rbx
            rdi = rbx;
            // a6331 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6336 lea rcx,[rip+0x8496d] # 0x12acaa
            rcx = m.env.data_va(259191);
            // a633d lea r14,[rip+0x8d7bf] # 0x133b03
            r14 = m.env.data_va(295632);
            // a6344 test DWORD PTR [rbp-0x1bc],0x200000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64 & 0x200000u64);
            // a634e mov rsi,r14
            rsi = r14;
            // a6351 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a6355 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // a635a mov rdi,rax
            rdi = rax;
            // a635d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6362 mov rbx,rax
            rbx = rax;
            // a6365 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6368 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a636c add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a636f lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // a6373 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6378 mov rsi,QWORD PTR [rip+0x99e11] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a637f lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // a6383 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6388 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a638b mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a638f mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6394 mov rdi,rax
            rdi = rax;
            // a6397 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6399 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a639c lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // a63a0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a63a5 movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a63a9 mov rdi,rbx
            rdi = rbx;
            // a63ac call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a63b1 mov rdi,rbx
            rdi = rbx;
            // a63b4 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a63b9 lea rsi,[rip+0x8dd3e] # 0x1340fe
            rsi = m.env.data_va(297163);
            // a63c0 mov edx,0x18
            rdx = (0x18u64) as u32 as u64;
            // a63c5 mov rdi,rbx
            rdi = rbx;
            // a63c8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a63cd mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a63d3 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a63d5 and edx,0x40000
            { let r = (rdx as u32 as u64) & 0x40000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a63db mov rsi,r15
            rsi = r15;
            // a63de lea rcx,[rip+0x8dcb1] # 0x134096
            rcx = m.env.data_va(297059);
            // a63e5 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a63e9 shr rdx,0x11
            { let (r, f) = shr::<64>(rdx, 0x11u64, fl); rdx = r; fl = f; }
            // a63ed xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a63f1 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a63f5 mov rdi,rax
            rdi = rax;
            // a63f8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a63fd mov rbx,rax
            rbx = rax;
            // a6400 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6403 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6407 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a640a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // a640e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6413 mov rsi,QWORD PTR [rip+0x99d76] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a641a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // a641e call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6423 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a6426 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a642a mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a642f mov rdi,rax
            rdi = rax;
            // a6432 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6434 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a6437 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // a643b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6440 movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a6444 mov rdi,rbx
            rdi = rbx;
            // a6447 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a644c mov rdi,rbx
            rdi = rbx;
            // a644f call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6454 lea rsi,[rip+0x8dcbc] # 0x134117
            rsi = m.env.data_va(297188);
            // a645b mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // a6460 mov rdi,rbx
            rdi = rbx;
            // a6463 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6468 test DWORD PTR [rbp-0x1bc],0x80000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64 & 0x80000u64);
            // a6472 mov rsi,r14
            rsi = r14;
            // a6475 lea rcx,[rip+0x8482e] # 0x12acaa
            rcx = m.env.data_va(259191);
            // a647c cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a6480 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // a6485 mov rdi,rax
            rdi = rax;
            // a6488 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a648d mov rbx,rax
            rbx = rax;
            // a6490 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6493 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6497 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a649a lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // a649e call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64a3 mov rsi,QWORD PTR [rip+0x99ce6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a64aa lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // a64ae call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64b3 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a64b6 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a64ba mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a64bf mov rdi,rax
            rdi = rax;
            // a64c2 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64c4 mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a64c7 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // a64cb call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64d0 movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a64d4 mov rdi,rbx
            rdi = rbx;
            // a64d7 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64dc mov rdi,rbx
            rdi = rbx;
            // a64df call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64e4 lea rsi,[rip+0x8dc3f] # 0x13412a
            rsi = m.env.data_va(297207);
            // a64eb mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // a64f0 mov rdi,rbx
            rdi = rbx;
            // a64f3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a64f8 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a64fe mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a6500 and edx,0x400000
            { let r = (rdx as u32 as u64) & 0x400000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a6506 mov rsi,r15
            rsi = r15;
            // a6509 lea rcx,[rip+0x8db86] # 0x134096
            rcx = m.env.data_va(297059);
            // a6510 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a6514 shr rdx,0x15
            { let (r, f) = shr::<64>(rdx, 0x15u64, fl); rdx = r; fl = f; }
            // a6518 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a651c add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a6520 mov rdi,rax
            rdi = rax;
            // a6523 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6528 mov rbx,rax
            rbx = rax;
            // a652b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a652e mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6532 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a6535 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a6539 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a653e mov rsi,QWORD PTR [rip+0x99c4b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a6545 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a6549 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a654e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a6551 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a6555 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a655a mov rdi,rax
            rdi = rax;
            // a655d call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a655f mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a6562 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a6566 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a656b movsx esi,r13b
            rsi = (sx::<8>((r13 as u8 as u64)) as u64) as u32 as u64;
            // a656f mov rdi,rbx
            rdi = rbx;
            // a6572 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6577 mov rdi,rbx
            rdi = rbx;
            // a657a call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a657f lea rsi,[rip+0x8dbbc] # 0x134142
            rsi = m.env.data_va(297231);
            // a6586 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // a658b mov rdi,rbx
            rdi = rbx;
            // a658e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6593 test DWORD PTR [rbp-0x1bc],0x800000
            fl = logic::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64 & 0x800000u64);
            // a659d lea rcx,[rip+0x84706] # 0x12acaa
            rcx = m.env.data_va(259191);
            // a65a4 cmovne r14,rcx
            if fl.ne() { r14 = rcx; }
            // a65a8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // a65ad mov rdi,rax
            rdi = rax;
            // a65b0 mov rsi,r14
            rsi = r14;
            // a65b3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a65b8 mov rbx,rax
            rbx = rax;
            // a65bb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a65be mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a65c2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a65c5 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // a65c9 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a65ce mov rsi,QWORD PTR [rip+0x99bbb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a65d5 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // a65d9 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a65de mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a65e1 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a65e5 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a65ea mov rdi,rax
            rdi = rax;
            // a65ed call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a65ef mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a65f2 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // a65f6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a65fb movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a65ff mov rdi,rbx
            rdi = rbx;
            // a6602 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6607 mov rdi,rbx
            rdi = rbx;
            // a660a call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a660f lea rsi,[rip+0x8db3e] # 0x134154
            rsi = m.env.data_va(297249);
            // a6616 mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // a661b mov rdi,rbx
            rdi = rbx;
            // a661e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6623 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a6629 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a662b and edx,0x2000000
            { let r = (rdx as u32 as u64) & 0x2000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a6631 mov rsi,r15
            rsi = r15;
            // a6634 lea rcx,[rip+0x8da5b] # 0x134096
            rcx = m.env.data_va(297059);
            // a663b cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a663f shr rdx,0x18
            { let (r, f) = shr::<64>(rdx, 0x18u64, fl); rdx = r; fl = f; }
            // a6643 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a6647 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a664b mov rdi,rax
            rdi = rax;
            // a664e call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6653 mov rbx,rax
            rbx = rax;
            // a6656 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a6659 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a665d add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a6660 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // a6664 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6669 mov rsi,QWORD PTR [rip+0x99b20] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a6670 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // a6674 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6679 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a667c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a6680 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6685 mov rdi,rax
            rdi = rax;
            // a6688 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a668a mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a668d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // a6691 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6696 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a669a mov rdi,rbx
            rdi = rbx;
            // a669d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a66a2 mov rdi,rbx
            rdi = rbx;
            // a66a5 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a66aa lea rsi,[rip+0x8dab9] # 0x13416a
            rsi = m.env.data_va(297271);
            // a66b1 mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // a66b6 mov rdi,rbx
            rdi = rbx;
            // a66b9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a66be mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a66c4 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a66c6 and edx,0x40000000
            { let r = (rdx as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a66cc mov rsi,r15
            rsi = r15;
            // a66cf lea rcx,[rip+0x8d9c0] # 0x134096
            rcx = m.env.data_va(297059);
            // a66d6 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a66da shr rdx,0x1d
            { let (r, f) = shr::<64>(rdx, 0x1du64, fl); rdx = r; fl = f; }
            // a66de xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a66e2 add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a66e6 mov rdi,rax
            rdi = rax;
            // a66e9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a66ee mov rbx,rax
            rbx = rax;
            // a66f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a66f4 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a66f8 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a66fb lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // a66ff call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6704 mov rsi,QWORD PTR [rip+0x99a85] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a670b lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // a670f call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6714 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a6717 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a671b mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a6720 mov rdi,rax
            rdi = rax;
            // a6723 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6725 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a6728 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // a672c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6731 movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a6735 mov rdi,rbx
            rdi = rbx;
            // a6738 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a673d mov rdi,rbx
            rdi = rbx;
            // a6740 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6745 lea rsi,[rip+0x8da3b] # 0x134187
            rsi = m.env.data_va(297300);
            // a674c mov edx,0x1c
            rdx = (0x1cu64) as u32 as u64;
            // a6751 mov rdi,rbx
            rdi = rbx;
            // a6754 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6759 mov ecx,DWORD PTR [rbp-0x1bc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64) as u32 as u64;
            // a675f mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // a6761 and edx,0x20000000
            { let r = (rdx as u32 as u64) & 0x20000000u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // a6767 mov rsi,r15
            rsi = r15;
            // a676a lea rcx,[rip+0x8d925] # 0x134096
            rcx = m.env.data_va(297059);
            // a6771 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a6775 shr rdx,0x1c
            { let (r, f) = shr::<64>(rdx, 0x1cu64, fl); rdx = r; fl = f; }
            // a6779 xor rdx,0x2
            { let r = rdx ^ 0x2u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a677d add rdx,0x6
            { let (r, f) = add::<64>(rdx, 0x6u64);
              rdx = r;
              fl = f; }
            // a6781 mov rdi,rax
            rdi = rax;
            // a6784 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6789 mov rbx,rax
            rbx = rax;
            // a678c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a678f mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a6793 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // a6796 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a679a call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a679f mov rsi,QWORD PTR [rip+0x999ea] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a67a6 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a67aa call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67af mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a67b2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a67b6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a67bb mov rdi,rax
            rdi = rax;
            // a67be call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67c0 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a67c3 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a67c7 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67cc movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a67d0 mov rdi,rbx
            rdi = rbx;
            // a67d3 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67d8 mov rdi,rbx
            rdi = rbx;
            // a67db call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67e0 lea rsi,[rip+0x8d9bd] # 0x1341a4
            rsi = m.env.data_va(297329);
            // a67e7 mov edx,0x1b
            rdx = (0x1bu64) as u32 as u64;
            // a67ec mov rdi,rbx
            rdi = rbx;
            // a67ef call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a67f4 cmp DWORD PTR [rbp-0x1bc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe44)) as u64, 0x0u64).1;
            // a67fb lea rcx,[rip+0x8d894] # 0x134096
            rcx = m.env.data_va(297059);
            // a6802 cmovs r15,rcx
            if fl.s() { r15 = rcx; }
            // a6806 setns cl
            rcx = (rcx & !0xff) | (((fl.ns() as u64)) as u8 as u64);
            // a6809 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // a680c lea rdx,[rcx+rcx*1+0x6]
            rdx = rcx.wrapping_add(rcx).wrapping_add(0x6);
            // a6811 mov rdi,rax
            rdi = rax;
            // a6814 mov rsi,r15
            rsi = r15;
            // a6817 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a681c mov rdi,QWORD PTR [rbp-0x1c8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // a6823 lea rsi,[rbp-0x1b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // a682a call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a682f mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // a6836 mov QWORD PTR [rbp-0x1b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rax) as u64);
            // a683d mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // a6844 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // a684b mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // a6852 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            // a6859 lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // a6860 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6865 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // a686c call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6871 mov rsi,QWORD PTR [rip+0x99990] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a6878 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a687c lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a6883 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6888 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // a688f call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6894 mov rax,QWORD PTR [rbp-0x1c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe38)) as u64;
            // a689b add rsp,0x1b8
            { let (r, f) = add::<64>(rsp, 0x1b8u64);
              rsp = r;
              fl = f; }
            // a68a2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a68a3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a68a5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a68a7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a68a9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a68ab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a68ac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa68ad => {
            // block a68ad
            // a68ad mov rbx,rax
            rbx = rax;
            pc = 0xa68b0;
        }
        0xa68b0 => {
            // block a68b0
            // a68b0 mov rax,QWORD PTR [rbp-0x1d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe28)) as u64;
            // a68b7 mov QWORD PTR [rbp-0x1b8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe48), (rax) as u64);
            // a68be mov rax,QWORD PTR [rbp-0x1d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe30)) as u64;
            // a68c5 mov QWORD PTR [rbp-0x148],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rax) as u64);
            // a68cc mov rax,QWORD PTR [rbp-0x1e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe20)) as u64;
            // a68d3 mov QWORD PTR [rbp-0x1b0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (rax) as u64);
            pc = 0xa68da;
        }
        0xa68da => {
            // block a68da
            // a68da lea rdi,[rbp-0x170]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe90);
            // a68e1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a68e6 lea rdi,[rbp-0x1b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe50);
            // a68ed call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa68f2;
        }
        0xa68f2 => {
            // block a68f2
            // a68f2 mov rsi,QWORD PTR [rip+0x9990f] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a68f9 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a68fd lea rdi,[rbp-0x1b8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe48);
            // a6904 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa6909;
        }
        0xa6909 => {
            // block a6909
            // a6909 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // a6910 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6915 mov rdi,rbx
            rdi = rbx;
            // a6918 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a691d mov rbx,rax
            rbx = rax;
            // a6920 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // a6927 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a692c jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa692e => {
            // block a692e
            // a692e mov rbx,rax
            rbx = rax;
            // a6931 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // a6938 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a693d jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6942 => {
            // block a6942
            // a6942 mov rbx,rax
            rbx = rax;
            // a6945 lea rdi,[rbp-0x70]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff90);
            // a6949 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a694e jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6953 => {
            // block a6953
            // a6953 mov rbx,rax
            rbx = rax;
            // a6956 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // a695a call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a695f jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6964 => {
            // block a6964
            // a6964 mov rbx,rax
            rbx = rax;
            // a6967 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // a696b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6970 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6975 => {
            // block a6975
            // a6975 mov rbx,rax
            rbx = rax;
            // a6978 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // a697c call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6981 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6986 => {
            // block a6986
            // a6986 mov rbx,rax
            rbx = rax;
            // a6989 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a698d call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6992 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6997 => {
            // block a6997
            // a6997 mov rbx,rax
            rbx = rax;
            // a699a lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // a699e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69a3 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa69a8 => {
            // block a69a8
            // a69a8 mov rbx,rax
            rbx = rax;
            // a69ab lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // a69af call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69b4 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa69b9 => {
            // block a69b9
            // a69b9 mov rbx,rax
            rbx = rax;
            // a69bc lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // a69c0 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69c5 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa69ca => {
            // block a69ca
            // a69ca mov rbx,rax
            rbx = rax;
            // a69cd lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a69d1 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69d6 jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa69db => {
            // block a69db
            // a69db mov rbx,rax
            rbx = rax;
            // a69de lea rdi,[rbp-0x88]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff78);
            // a69e5 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69ea jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa69ef => {
            // block a69ef
            // a69ef mov rbx,rax
            rbx = rax;
            // a69f2 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // a69f6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a69fb jmp 0xa68b0
            pc = 0xa68b0; continue;
        }
        0xa6a00 => {
            // block a6a00
            // a6a00 mov rbx,rax
            rbx = rax;
            // a6a03 jmp 0xa6909
            pc = 0xa6909; continue;
        }
        0xa6a08 => {
            // block a6a08
            // a6a08 mov rbx,rax
            rbx = rax;
            // a6a0b jmp 0xa68f2
            pc = 0xa68f2; continue;
        }
        0xa6a10 => {
            // block a6a10
            // a6a10 mov rbx,rax
            rbx = rax;
            // a6a13 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // a6a1a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6a1f jmp 0xa68da
            pc = 0xa68da; continue;
        }
        0xa6a24 => {
            // block a6a24
            // a6a24 mov rbx,rax
            rbx = rax;
            // a6a27 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // a6a2b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a6a30 jmp 0xa68b0
            pc = 0xa68b0; continue;
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
            // d6ca3 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ca8;
        }
        0xd6ca8 => {
            // block d6ca8
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
            // d6cb4 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cb9;
        }
        0xd6cb9 => {
            // block d6cb9
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
            // d6cc5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6cc8 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6ccb;
        }
        0xd6ccb => {
            // block d6ccb
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
            // d6cd7 cmp QWORD PTR [rbx+0x6a8],rdi
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rdi).1;
            // d6cde je 0xd6ce7
            if fl.e() { pc = 0xd6ce7; continue; }
            pc = 0xd6ce0;
        }
        0xd6ce0 => {
            // block d6ce0
            // d6ce0 mov QWORD PTR [rbx+0x6a8],rdi
            m.w64(rbx.wrapping_add(0x6a8), (rdi) as u64);
            pc = 0xd6ce7;
        }
        0xd6ce7 => {
            // block d6ce7
            // d6ce7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6cec;
        }
        0xd6cec => {
            // block d6cec
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
            // d738e lea rax,[r12+0x344]
            rax = r12.wrapping_add(0x344);
            // d7396 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7398 => {
            // block d7398
            // d7398 lea rax,[r12+0x340]
            rax = r12.wrapping_add(0x340);
            pc = 0xd73a0;
        }
        0xd73a0 => {
            // block d73a0
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
            // d7452 lea rax,[r12+0x354]
            rax = r12.wrapping_add(0x354);
            // d745a jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd745f => {
            // block d745f
            // d745f lea rax,[r12+0x358]
            rax = r12.wrapping_add(0x358);
            // d7467 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd746c => {
            // block d746c
            // d746c lea rax,[r12+0x348]
            rax = r12.wrapping_add(0x348);
            // d7474 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7479 => {
            // block d7479
            // d7479 lea rax,[r12+0x34c]
            rax = r12.wrapping_add(0x34c);
            // d7481 jmp 0xd73a0
            pc = 0xd73a0; continue;
        }
        0xd7486 => {
            // block d7486
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
            // d77dc mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d77df jmp 0xd784f
            pc = 0xd784f; continue;
        }
        0xd77e1 => {
            // block d77e1
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
            // d7830 lea rax,[r15+0x348]
            rax = r15.wrapping_add(0x348);
            // d7837 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7839 => {
            // block d7839
            // d7839 lea rax,[r15+0x34c]
            rax = r15.wrapping_add(0x34c);
            // d7840 jmp 0xd7849
            pc = 0xd7849; continue;
        }
        0xd7842 => {
            // block d7842
            // d7842 lea rax,[r15+0x350]
            rax = r15.wrapping_add(0x350);
            pc = 0xd7849;
        }
        0xd7849 => {
            // block d7849
            // d7849 mov DWORD PTR [rbp-0x30],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), ((rbx as u32 as u64)) as u32);
            // d784c mov r12d,DWORD PTR [rax]
            r12 = (m.r32(rax) as u64) as u32 as u64;
            pc = 0xd784f;
        }
        0xd784f => {
            // block d784f
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
            // d78d9 cmp r12d,DWORD PTR [r15+0x338]
            fl = sub::<32>((r12 as u32 as u64), m.r32(r15.wrapping_add(0x338)) as u64).1;
            // d78e0 jne 0xd7903
            if fl.ne() { pc = 0xd7903; continue; }
            pc = 0xd78e2;
        }
        0xd78e2 => {
            // block d78e2
            // d78e2 cmp DWORD PTR [r15+0x370],0x1
            fl = sub::<32>(m.r32(r15.wrapping_add(0x370)) as u64, 0x1u64).1;
            // d78ea jne 0xd78f3
            if fl.ne() { pc = 0xd78f3; continue; }
            pc = 0xd78ec;
        }
        0xd78ec => {
            // block d78ec
            // d78ec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d78f1 jmp 0xd78f8
            pc = 0xd78f8; continue;
        }
        0xd78f3 => {
            // block d78f3
            // d78f3 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd78f8;
        }
        0xd78f8 => {
            // block d78f8
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
            // d797c cmp BYTE PTR [r15+0x378],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7984 jne 0xd7994
            if fl.ne() { pc = 0xd7994; continue; }
            pc = 0xd7986;
        }
        0xd7986 => {
            // block d7986
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
            // eb18a lea r15,[rbp-0x2a0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            pc = 0xeb191;
        }
        0xeb191 => {
            // block eb191
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
            // eb219 jmp 0xeb21b
            pc = 0xeb21b; continue;
        }
        0xeb21b => {
            // block eb21b
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
            // eac53 mov DWORD PTR [r14+0x60],0x1
            m.w32(r14.wrapping_add(0x60), (0x1u64) as u32);
            pc = 0xeac5b;
        }
        0xeac5b => {
            // block eac5b
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
            // eac7b lea r13,[rbp-0x2b0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xeac82;
        }
        0xeac82 => {
            // block eac82
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
            // ead10 mov DWORD PTR [rbp-0x2b8],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fd48), ((r12 as u32 as u64)) as u32);
            // ead17 lea r12,[rbp-0x2b0]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            pc = 0xead1e;
        }
        0xead1e => {
            // block ead1e
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
            // ead56 mov ebx,DWORD PTR [r14+0x60]
            rbx = (m.r32(r14.wrapping_add(0x60)) as u64) as u32 as u64;
            // ead5a mov r12d,DWORD PTR [rbp-0x2b8]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64) as u32 as u64;
            pc = 0xead61;
        }
        0xead61 => {
            // block ead61
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
            // eaf97 lea rdi,[rbp-0x2b0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd50);
            // eaf9e call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xeafa3;
        }
        0xeafa3 => {
            // block eafa3
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
            // eafb8 jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafba => {
            // block eafba
            // eafba jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbc => {
            // block eafbc
            // eafbc jmp 0xeafbe
            pc = 0xeafbe; continue;
        }
        0xeafbe => {
            // block eafbe
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
            // d6a31 mov QWORD PTR [rbx+0x344],rax
            m.w64(rbx.wrapping_add(0x344), (rax) as u64);
            // d6a38 jmp 0xd6bad
            pc = 0xd6bad; continue;
        }
        0xd6a3d => {
            // block d6a3d
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
            // d6a49 cmp DWORD PTR [rbx+0x330],0x10220
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x330)) as u64, 0x10220u64).1;
            // d6a53 jne 0xd6b0f
            if fl.ne() { pc = 0xd6b0f; continue; }
            pc = 0xd6a59;
        }
        0xd6a59 => {
            // block d6a59
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
            // d6941 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // d6945 jmp 0xd6970
            pc = 0xd6970; continue;
        }
        0xd6947 => {
            // block d6947
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
            // d6953 cmp QWORD PTR [rbx+0x6a8],rdi
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x6a8)) as u64, rdi).1;
            // d695a je 0xd6963
            if fl.e() { pc = 0xd6963; continue; }
            pc = 0xd695c;
        }
        0xd695c => {
            // block d695c
            // d695c mov QWORD PTR [rbx+0x6a8],rdi
            m.w64(rbx.wrapping_add(0x6a8), (rdi) as u64);
            pc = 0xd6963;
        }
        0xd6963 => {
            // block d6963
            // d6963 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6968;
        }
        0xd6968 => {
            // block d6968
            // d6968 mov rdi,r12
            rdi = r12;
            // d696b call 0xe807c
            { let r = { let t = m.env.code_va(0xe807c); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd6970;
        }
        0xd6970 => {
            // block d6970
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
