//! Mechanically lifted functions (scripts/lift_run.py), module g024. Each one is checked against the original
//! in the world harness (tests/world_g024.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            // 3ee4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x3ee6;
        }
        0x3ee6 => {
            // block 3ee6
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

/// parity: P2v @0xb7c4a size=264
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7c4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7c4a;
    loop {
        match pc {
        0xb7c4a => {
            // block b7c4a
            // b7c4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7c4b mov rbp,rsp
            rbp = rsp;
            // b7c4e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b7c50 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7c52 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b7c54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b7c56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7c57 sub rsp,0x468
            { let (r, f) = sub::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7c5e mov r15,rcx
            r15 = rcx;
            // b7c61 mov r12,rdx
            r12 = rdx;
            // b7c64 mov rbx,rdi
            rbx = rdi;
            // b7c67 mov rax,QWORD PTR [rip+0x8876a] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7c6e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7c71 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // b7c75 test BYTE PTR [r12],0x1
            fl = logic::<8>(m.r8(r12) as u64 & 0x1u64);
            // b7c7a jne 0xb7c85
            if fl.ne() { pc = 0xb7c85; continue; }
            pc = 0xb7c7c;
        }
        0xb7c7c => {
            // block b7c7c
            // b7c7c mov WORD PTR [r12],0x0
            m.w16(r12, (0x0u64) as u16);
            // b7c83 jmp 0xb7c96
            pc = 0xb7c96; continue;
        }
        0xb7c85 => {
            // block b7c85
            // b7c85 mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // b7c8a mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7c8d mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7c96;
        }
        0xb7c96 => {
            // block b7c96
            // b7c96 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // b7c9a jne 0xb7ca4
            if fl.ne() { pc = 0xb7ca4; continue; }
            pc = 0xb7c9c;
        }
        0xb7c9c => {
            // block b7c9c
            // b7c9c mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // b7ca2 jmp 0xb7cb3
            pc = 0xb7cb3; continue;
        }
        0xb7ca4 => {
            // block b7ca4
            // b7ca4 mov rax,QWORD PTR [r15+0x10]
            rax = m.r64(r15.wrapping_add(0x10)) as u64;
            // b7ca8 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // b7cab mov QWORD PTR [r15+0x8],0x0
            m.w64(r15.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xb7cb3;
        }
        0xb7cb3 => {
            // block b7cb3
            // b7cb3 mov DWORD PTR [rsi],0x0
            m.w32(rsi, (0x0u64) as u32);
            // b7cb9 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b7cbd je 0xb7d24
            if fl.e() { pc = 0xb7d24; continue; }
            pc = 0xb7cbf;
        }
        0xb7cbf => {
            // block b7cbf
            // b7cbf mov QWORD PTR [rbp-0x490],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fb70), (rsi) as u64);
            // b7cc6 lea r14,[rbp-0x488]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fb78);
            // b7ccd mov esi,0x454
            rsi = (0x454u64) as u32 as u64;
            // b7cd2 mov rdi,r14
            rdi = r14;
            // b7cd5 call 0xec188
            { let r = { let t = m.env.stub_va(0x955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cda mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7cdd xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // b7ce0 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7ce2 mov rdi,rbx
            rdi = rbx;
            // b7ce5 mov rsi,r14
            rsi = r14;
            // b7ce8 call QWORD PTR [rax+0x140]
            { let r = { let t = m.r64(rax.wrapping_add(0x140)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7cee test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7cf0 je 0xb7d27
            if fl.e() { pc = 0xb7d27; continue; }
            pc = 0xb7cf2;
        }
        0xb7cf2 => {
            // block b7cf2
            // b7cf2 lea rsi,[rbp-0x478]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb88);
            // b7cf9 mov rdi,r12
            rdi = r12;
            // b7cfc call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d01 lea rsi,[rbp-0x468]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb98);
            // b7d08 mov rdi,r15
            rdi = r15;
            // b7d0b call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7d10 mov eax,DWORD PTR [rbp-0x47c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fb84)) as u64) as u32 as u64;
            // b7d16 mov rcx,QWORD PTR [rbp-0x490]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fb70)) as u64;
            // b7d1d mov DWORD PTR [rcx],eax
            m.w32(rcx, ((rax as u32 as u64)) as u32);
            // b7d1f mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            // b7d22 jmp 0xb7d27
            pc = 0xb7d27; continue;
        }
        0xb7d24 => {
            // block b7d24
            // b7d24 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7d27;
        }
        0xb7d27 => {
            // block b7d27
            // b7d27 mov rax,QWORD PTR [rip+0x886aa] # 0x1403d8
            rax = m.r64(m.env.data_va(347045)) as u64;
            // b7d2e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // b7d31 cmp rax,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // b7d35 jne 0xb7d4c
            if fl.ne() { pc = 0xb7d4c; continue; }
            pc = 0xb7d37;
        }
        0xb7d37 => {
            // block b7d37
            // b7d37 mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // b7d3a add rsp,0x468
            { let (r, f) = add::<64>(rsp, 0x468u64);
              rsp = r;
              fl = f; }
            // b7d41 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7d42 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b7d44 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b7d46 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7d48 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7d4a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7d4b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7d4c => {
            // block b7d4c
            // b7d4c call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdf88 size=266
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000df88<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xdf88;
    loop {
        match pc {
        0xdf88 => {
            // block df88
            // df88 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // df89 mov rbp,rsp
            rbp = rsp;
            // df8c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // df8e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // df90 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // df92 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // df94 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // df95 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // df99 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // df9d mov rbx,rdi
            rbx = rdi;
            // dfa0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfa3 lea rsi,[rbp-0x48]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // dfa7 call QWORD PTR [rax+0x16d8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16d8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfad mov BYTE PTR [rbp-0x49],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb7), ((rax as u8 as u64)) as u8);
            // dfb0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfb3 lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // dfb7 mov rdi,rbx
            rdi = rbx;
            // dfba call QWORD PTR [rax+0x16e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16e8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfc0 mov BYTE PTR [rbp-0x4a],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb6), ((rax as u8 as u64)) as u8);
            // dfc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfc6 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // dfca mov rdi,rbx
            rdi = rbx;
            // dfcd call QWORD PTR [rax+0x16c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfd3 mov BYTE PTR [rbp-0x4b],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffb5), ((rax as u8 as u64)) as u8);
            // dfd6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfd9 lea rsi,[rbp-0x3c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            // dfdd mov rdi,rbx
            rdi = rbx;
            // dfe0 call QWORD PTR [rax+0x16d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16d0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfe6 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // dfe9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfec lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // dff0 mov rdi,rbx
            rdi = rbx;
            // dff3 call QWORD PTR [rax+0x16c0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16c0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dff9 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // dffc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // dfff lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // e003 mov rdi,rbx
            rdi = rbx;
            // e006 call QWORD PTR [rax+0x16f8]
            { let r = { let t = m.r64(rax.wrapping_add(0x16f8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e00c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e00f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e012 lea rsi,[rbp-0x33]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcd);
            // e016 mov rdi,rbx
            rdi = rbx;
            // e019 call QWORD PTR [rax+0x1708]
            { let r = { let t = m.r64(rax.wrapping_add(0x1708)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e01f mov r13b,al
            r13 = (r13 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e022 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e025 lea rsi,[rbp-0x30]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e029 mov rdi,rbx
            rdi = rbx;
            // e02c call QWORD PTR [rax+0x1718]
            { let r = { let t = m.r64(rax.wrapping_add(0x1718)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e032 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e034 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e036 cmp BYTE PTR [rbp-0x49],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb7)) as u64, 0x0u64).1;
            // e03a je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe03c;
        }
        0xe03c => {
            // block e03c
            // e03c cmp BYTE PTR [rbp-0x4a],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb6)) as u64, 0x0u64).1;
            // e040 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe042;
        }
        0xe042 => {
            // block e042
            // e042 cmp BYTE PTR [rbp-0x4b],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb5)) as u64, 0x0u64).1;
            // e046 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe048;
        }
        0xe048 => {
            // block e048
            // e048 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // e04b je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe04d;
        }
        0xe04d => {
            // block e04d
            // e04d test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // e050 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe052;
        }
        0xe052 => {
            // block e052
            // e052 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // e055 je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe057;
        }
        0xe057 => {
            // block e057
            // e057 test r13b,r13b
            fl = logic::<8>((r13 as u8 as u64) & (r13 as u8 as u64));
            // e05a je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe05c;
        }
        0xe05c => {
            // block e05c
            // e05c test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // e05e je 0xe083
            if fl.e() { pc = 0xe083; continue; }
            pc = 0xe060;
        }
        0xe060 => {
            // block e060
            // e060 mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // e063 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e067 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // e06a mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // e06e mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // e072 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // e076 mov rcx,QWORD PTR [rbp-0x40]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // e07a mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // e07e mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // e081 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe083;
        }
        0xe083 => {
            // block e083
            // e083 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e087 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e088 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e08a pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e08c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e08e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e090 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e091 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1710 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001710<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1710;
    loop {
        match pc {
        0x1710 => {
            // block 1710
            // 1710 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1711 mov rbp,rsp
            rbp = rsp;
            // 1714 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1716 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1717 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 171b mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 171d mov r14,rdi
            r14 = rdi;
            // 1720 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 1722 je 0x1750
            if fl.e() { pc = 0x1750; continue; }
            pc = 0x1724;
        }
        0x1724 => {
            // block 1724
            // 1724 mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 1728 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 172b lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 172f mov rdi,r14
            rdi = r14;
            // 1732 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 1734 call QWORD PTR [rax+0x4a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4a8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 173a cmp BYTE PTR [rbp-0x11],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64, 0x0u64).1;
            // 173e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1741 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1745 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 174a je 0x17a5
            if fl.e() { pc = 0x17a5; continue; }
            pc = 0x174c;
        }
        0x174c => {
            // block 174c
            // 174c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 174e jmp 0x17aa
            pc = 0x17aa; continue;
        }
        0x1750 => {
            // block 1750
            // 1750 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1753 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1757 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 175c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 175e cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 1761 jg 0x17d0
            if fl.g() { pc = 0x17d0; continue; }
            pc = 0x1763;
        }
        0x1763 => {
            // block 1763
            // 1763 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 1768 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 176e mov rdi,r14
            rdi = r14;
            // 1771 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1773 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1776 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 177b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 177d mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1782 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1788 mov rdi,r14
            rdi = r14;
            // 178b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 178e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1791 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1796 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1798 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 179d mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 17a3 jmp 0x1810
            pc = 0x1810; continue;
        }
        0x17a5 => {
            // block 17a5
            // 17a5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x17aa;
        }
        0x17aa => {
            // block 17aa
            // 17aa cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 17ad jg 0x17bc
            if fl.g() { pc = 0x17bc; continue; }
            pc = 0x17af;
        }
        0x17af => {
            // block 17af
            // 17af mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 17b4 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 17ba jmp 0x17c7
            pc = 0x17c7; continue;
        }
        0x17bc => {
            // block 17bc
            // 17bc mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 17c1 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            pc = 0x17c7;
        }
        0x17c7 => {
            // block 17c7
            // 17c7 mov rdi,r14
            rdi = r14;
            // 17ca call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cc mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 17ce jmp 0x1816
            pc = 0x1816; continue;
        }
        0x17d0 => {
            // block 17d0
            // 17d0 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 17d5 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 17db mov rdi,r14
            rdi = r14;
            // 17de call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17e3 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 17e8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17ea mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 17ef mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 17f5 mov rdi,r14
            rdi = r14;
            // 17f8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17fe mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1803 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1805 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 180a mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x1810;
        }
        0x1810 => {
            // block 1810
            // 1810 mov rdi,r14
            rdi = r14;
            // 1813 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1816;
        }
        0x1816 => {
            // block 1816
            // 1816 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 181a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 181b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 181d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 181e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28aac size=276
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            // 28b08 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28b0c lea r13,[rip+0x10198a] # 0x12a49d
            r13 = m.env.data_va(257130);
            pc = 0x28b13;
        }
        0x28b13 => {
            // block 28b13
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
            // 28b29 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 28b2c lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // 28b30 jmp 0x28b3a
            pc = 0x28b3a; continue;
        }
        0x28b32 => {
            // block 28b32
            // 28b32 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 28b36 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0x28b3a;
        }
        0x28b3a => {
            // block 28b3a
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
            // 28b5e jmp 0x28b6f
            pc = 0x28b6f; continue;
        }
        0x28b60 => {
            // block 28b60
            // 28b60 mov rax,rbx
            rax = rbx;
            pc = 0x28b63;
        }
        0x28b63 => {
            // block 28b63
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
            // 28bac mov rbx,rax
            rbx = rax;
            // 28baf lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 28bb3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28bb8 mov rdi,rbx
            rdi = rbx;
            // 28bbb call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9608 size=312
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9608<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9608;
    loop {
        match pc {
        0xe9608 => {
            // block e9608
            // e9608 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9609 mov rbp,rsp
            rbp = rsp;
            // e960c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e960e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9610 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9611 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e9612 mov r14,rsi
            r14 = rsi;
            // e9615 mov rbx,rdi
            rbx = rdi;
            // e9618 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e961f movzx eax,BYTE PTR [rdi]
            rax = (m.r8(rdi) as u64) as u32 as u64;
            // e9622 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e9624 jne 0xe962b
            if fl.ne() { pc = 0xe962b; continue; }
            pc = 0xe9626;
        }
        0xe9626 => {
            // block e9626
            // e9626 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // e9629 jmp 0xe962f
            pc = 0xe962f; continue;
        }
        0xe962b => {
            // block e962b
            // e962b mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            pc = 0xe962f;
        }
        0xe962f => {
            // block e962f
            // e962f test rax,rax
            fl = logic::<64>(rax & rax);
            // e9632 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9634;
        }
        0xe9634 => {
            // block e9634
            // e9634 lea rsi,[rip+0x407ed] # 0x129e28
            rsi = m.env.data_va(255477);
            // e963b call 0xebfae
            { let r = { let t = m.env.stub_va(0x77b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9640 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9642 je 0xe9662
            if fl.e() { pc = 0xe9662; continue; }
            pc = 0xe9644;
        }
        0xe9644 => {
            // block e9644
            // e9644 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e964b mov WORD PTR [rbp-0x1e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ffe2), (0x0u64) as u16);
            // e9651 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e9658 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e965b jne 0xe966e
            if fl.ne() { pc = 0xe966e; continue; }
            pc = 0xe965d;
        }
        0xe965d => {
            // block e965d
            // e965d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9660 jmp 0xe9672
            pc = 0xe9672; continue;
        }
        0xe9662 => {
            // block e9662
            // e9662 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e966a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e966c jmp 0xe96ca
            pc = 0xe96ca; continue;
        }
        0xe966e => {
            // block e966e
            // e966e mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9672;
        }
        0xe9672 => {
            // block e9672
            // e9672 lea r15,[r14+0x2]
            r15 = r14.wrapping_add(0x2);
            // e9676 lea rsi,[rip+0x53caa] # 0x13d327
            rsi = m.env.data_va(334580);
            // e967d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9681 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9685 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9687 mov rcx,r15
            rcx = r15;
            // e968a call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e968f mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9692 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9695 mov DWORD PTR [r14+0x4],0x0
            m.w32(r14.wrapping_add(0x4), (0x0u64) as u32);
            // e969d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // e96a0 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e96a4 ja 0xe96c0
            if fl.a() { pc = 0xe96c0; continue; }
            pc = 0xe96a6;
        }
        0xe96a6 => {
            // block e96a6
            // e96a6 lea rcx,[rip+0x7f] # 0xe972c
            rcx = m.env.code_va(0xe972c);
            // e96ad movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // e96b1 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // e96b4 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xe96b6 => {
            // block e96b6
            // e96b6 mov DWORD PTR [r14+0x8],0x1
            m.w32(r14.wrapping_add(0x8), (0x1u64) as u32);
            // e96be jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96c0 => {
            // block e96c0
            // e96c0 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            pc = 0xe96c8;
        }
        0xe96c8 => {
            // block e96c8
            // e96c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe96ca;
        }
        0xe96ca => {
            // block e96ca
            // e96ca add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e96ce pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e96cf pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e96d1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e96d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e96d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe96d5 => {
            // block e96d5
            // e96d5 mov DWORD PTR [r14+0x8],0x2
            m.w32(r14.wrapping_add(0x8), (0x2u64) as u32);
            // e96dd jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96df => {
            // block e96df
            // e96df mov DWORD PTR [r14+0x8],0x3
            m.w32(r14.wrapping_add(0x8), (0x3u64) as u32);
            // e96e7 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
        0xe96e9 => {
            // block e96e9
            // e96e9 mov DWORD PTR [r14+0x8],0x4
            m.w32(r14.wrapping_add(0x8), (0x4u64) as u32);
            // e96f1 mov rdi,QWORD PTR [rbx+0x288]
            rdi = m.r64(rbx.wrapping_add(0x288)) as u64;
            // e96f8 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e96fb jne 0xe9702
            if fl.ne() { pc = 0xe9702; continue; }
            pc = 0xe96fd;
        }
        0xe96fd => {
            // block e96fd
            // e96fd inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e9700 jmp 0xe9706
            pc = 0xe9706; continue;
        }
        0xe9702 => {
            // block e9702
            // e9702 mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe9706;
        }
        0xe9706 => {
            // block e9706
            // e9706 lea rsi,[rip+0x53c2a] # 0x13d337
            rsi = m.env.data_va(334596);
            // e970d lea rdx,[rbp-0x1e]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe2);
            // e9711 lea r8,[rbp-0x1c]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e9715 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e9717 mov r9,r15
            r9 = r15;
            // e971a mov rcx,r15
            rcx = r15;
            // e971d call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9722 mov al,BYTE PTR [rbp-0x1e]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffe2)) as u64) as u8 as u64);
            // e9725 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e9728 jmp 0xe96c8
            pc = 0xe96c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcf3c size=372
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cf3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcf3c;
    loop {
        match pc {
        0xcf3c => {
            // block cf3c
            // cf3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cf3d mov rbp,rsp
            rbp = rsp;
            // cf40 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // cf42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // cf43 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // cf47 mov r14,rsi
            r14 = rsi;
            // cf4a mov rbx,rdi
            rbx = rdi;
            // cf4d mov DWORD PTR [rbp-0x18],0xc
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), (0xcu64) as u32);
            // cf54 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf57 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // cf5b call QWORD PTR [rax+0x14c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14c8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cf61 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf63 je 0xcf75
            if fl.e() { pc = 0xcf75; continue; }
            pc = 0xcf65;
        }
        0xcf65 => {
            // block cf65
            // cf65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cf68 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // cf6c mov rdi,rbx
            rdi = rbx;
            // cf6f call QWORD PTR [rax+0x14b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14b8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xcf75;
        }
        0xcf75 => {
            // block cf75
            // cf75 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // cf77 je 0xd033
            if fl.e() { pc = 0xd033; continue; }
            pc = 0xcf7d;
        }
        0xcf7d => {
            // block cf7d
            // cf7d mov ecx,DWORD PTR [rbp-0x18]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64) as u32 as u64;
            // cf80 cmp rcx,0x3
            fl = sub::<64>(rcx, 0x3u64).1;
            // cf84 ja 0xd033
            if fl.a() { pc = 0xd033; continue; }
            pc = 0xcf8a;
        }
        0xcf8a => {
            // block cf8a
            // cf8a lea rdx,[rip+0xdf] # 0xd070
            rdx = m.env.code_va(0xd070);
            // cf91 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // cf95 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // cf98 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcf9a => {
            // block cf9a
            // cf9a mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cf9d cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfa0 jne 0xd000
            if fl.ne() { pc = 0xd000; continue; }
            pc = 0xcfa2;
        }
        0xcfa2 => {
            // block cfa2
            // cfa2 mov DWORD PTR [r14],0xc
            m.w32(r14, (0xcu64) as u32);
            // cfa9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfae => {
            // block cfae
            // cfae mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfb1 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cfb4 jne 0xd00e
            if fl.ne() { pc = 0xd00e; continue; }
            pc = 0xcfb6;
        }
        0xcfb6 => {
            // block cfb6
            // cfb6 mov DWORD PTR [r14],0xe
            m.w32(r14, (0xeu64) as u32);
            // cfbd jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfc2 => {
            // block cfc2
            // cfc2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // cfc4 mov edx,DWORD PTR [rbp-0x14]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfc7 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // cfc9 cmp edx,0xb
            fl = sub::<32>((rdx as u32 as u64), 0xbu64).1;
            // cfcc ja 0xd064
            if fl.a() { pc = 0xd064; continue; }
            pc = 0xcfd2;
        }
        0xcfd2 => {
            // block cfd2
            // cfd2 lea rsi,[rip+0xa7] # 0xd080
            rsi = m.env.code_va(0xd080);
            // cfd9 movsxd rdx,DWORD PTR [rsi+rdx*4]
            rdx = sx::<32>(m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // cfdd add rdx,rsi
            { let (r, f) = add::<64>(rdx, rsi);
              rdx = r;
              fl = f; }
            // cfe0 jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xcfe2 => {
            // block cfe2
            // cfe2 mov DWORD PTR [r14],0x4
            m.w32(r14, (0x4u64) as u32);
            // cfe9 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xcfeb => {
            // block cfeb
            // cfeb mov ecx,DWORD PTR [rbp-0x14]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // cfee test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // cff0 je 0xd01c
            if fl.e() { pc = 0xd01c; continue; }
            pc = 0xcff2;
        }
        0xcff2 => {
            // block cff2
            // cff2 cmp ecx,0x8
            fl = sub::<32>((rcx as u32 as u64), 0x8u64).1;
            // cff5 jne 0xd025
            if fl.ne() { pc = 0xd025; continue; }
            pc = 0xcff7;
        }
        0xcff7 => {
            // block cff7
            // cff7 mov DWORD PTR [r14],0x7
            m.w32(r14, (0x7u64) as u32);
            // cffe jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd000 => {
            // block d000
            // d000 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d003 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd005;
        }
        0xd005 => {
            // block d005
            // d005 mov DWORD PTR [r14],0x9
            m.w32(r14, (0x9u64) as u32);
            // d00c jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd00e => {
            // block d00e
            // d00e cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d011 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd013;
        }
        0xd013 => {
            // block d013
            // d013 mov DWORD PTR [r14],0xb
            m.w32(r14, (0xbu64) as u32);
            // d01a jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd01c => {
            // block d01c
            // d01c mov DWORD PTR [r14],0x8
            m.w32(r14, (0x8u64) as u32);
            // d023 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd025 => {
            // block d025
            // d025 cmp ecx,0xc
            fl = sub::<32>((rcx as u32 as u64), 0xcu64).1;
            // d028 jne 0xd033
            if fl.ne() { pc = 0xd033; continue; }
            pc = 0xd02a;
        }
        0xd02a => {
            // block d02a
            // d02a mov DWORD PTR [r14],0x6
            m.w32(r14, (0x6u64) as u32);
            // d031 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd033 => {
            // block d033
            // d033 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d035 jmp 0xd064
            pc = 0xd064; continue;
        }
        0xd037 => {
            // block d037
            // d037 mov DWORD PTR [r14],0x5
            m.w32(r14, (0x5u64) as u32);
            // d03e jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd040 => {
            // block d040
            // d040 mov DWORD PTR [r14],0x1
            m.w32(r14, (0x1u64) as u32);
            // d047 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd049 => {
            // block d049
            // d049 mov DWORD PTR [r14],0x2
            m.w32(r14, (0x2u64) as u32);
            // d050 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd052 => {
            // block d052
            // d052 mov DWORD PTR [r14],0x3
            m.w32(r14, (0x3u64) as u32);
            // d059 jmp 0xd062
            pc = 0xd062; continue;
        }
        0xd05b => {
            // block d05b
            // d05b mov DWORD PTR [r14],0x0
            m.w32(r14, (0x0u64) as u32);
            pc = 0xd062;
        }
        0xd062 => {
            // block d062
            // d062 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0xd064;
        }
        0xd064 => {
            // block d064
            // d064 mov al,cl
            rax = (rax & !0xff) | (((rcx as u8 as u64)) as u8 as u64);
            // d066 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d06a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d06b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d06d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d06e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe8100 size=404
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
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
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e813e mov WORD PTR [r15],0x0
            m.w16(r15, (0x0u64) as u16);
            // e8144 jmp 0xe815b
            pc = 0xe815b; continue;
        }
        0xe8146 => {
            // block e8146
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
            // e815b test BYTE PTR [r14],0x1
            fl = logic::<8>(m.r8(r14) as u64 & 0x1u64);
            // e815f jne 0xe8167
            if fl.ne() { pc = 0xe8167; continue; }
            pc = 0xe8161;
        }
        0xe8161 => {
            // block e8161
            // e8161 lea rdi,[r14+0x1]
            rdi = r14.wrapping_add(0x1);
            // e8165 jmp 0xe816b
            pc = 0xe816b; continue;
        }
        0xe8167 => {
            // block e8167
            // e8167 mov rdi,QWORD PTR [r14+0x10]
            rdi = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe816b;
        }
        0xe816b => {
            // block e816b
            // e816b lea rsi,[rbp-0x4b0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fb50);
            // e8172 call 0xec2a2
            { let r = { let t = m.env.stub_va(0xa6f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8177 mov eax,DWORD PTR [rbp-0x450]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fbb0)) as u64) as u32 as u64;
            // e817d mov DWORD PTR [rbx+0x260],eax
            m.w32(rbx.wrapping_add(0x260), ((rax as u32 as u64)) as u32);
            // e8183 lea rdi,[rbp-0x470]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fb90);
            // e818a call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e818f mov r12,rax
            r12 = rax;
            // e8192 lea r13,[rbp-0x4b8]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fb48);
            // e8199 mov rdi,r13
            rdi = r13;
            // e819c call 0xec2ea
            { let r = { let t = m.env.stub_va(0xab7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a1 mov rdi,r13
            rdi = r13;
            // e81a4 call 0xec1fa
            { let r = { let t = m.env.stub_va(0x9c7); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81a9 mov rdi,rax
            rdi = rax;
            // e81ac call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81b1 mov r13,rax
            r13 = rax;
            // e81b4 mov rdi,r12
            rdi = r12;
            // e81b7 call 0xec1a0
            { let r = { let t = m.env.stub_va(0x96d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e81da mov rdi,r15
            rdi = r15;
            // e81dd mov rsi,r12
            rsi = r12;
            // e81e0 call 0xec002
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e81ef inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // e81f2 jmp 0xe81f8
            pc = 0xe81f8; continue;
        }
        0xe81f4 => {
            // block e81f4
            // e81f4 mov r14,QWORD PTR [r14+0x10]
            r14 = m.r64(r14.wrapping_add(0x10)) as u64;
            pc = 0xe81f8;
        }
        0xe81f8 => {
            // block e81f8
            // e81f8 mov edx,0x8
            rdx = (0x8u64) as u32 as u64;
            // e81fd mov rdi,r15
            rdi = r15;
            // e8200 mov rsi,r14
            rsi = r14;
            // e8203 call 0xeb918
            { let r = { let t = m.env.stub_va(0xe5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e8217 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // e821a jmp 0xe8269
            pc = 0xe8269; continue;
        }
        0xe821c => {
            // block e821c
            // e821c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e821f xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e8221 mov rdi,rbx
            rdi = rbx;
            // e8224 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8227 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e822a mov rdi,rbx
            rdi = rbx;
            // e822d call QWORD PTR [rax+0xa0]
            { let r = { let t = m.r64(rax.wrapping_add(0xa0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e8240 add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e8244 mov rdi,rbx
            rdi = rbx;
            // e8247 call 0xebf12
            { let r = { let t = m.env.stub_va(0x6df); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e824c test rax,rax
            fl = logic::<64>(rax & rax);
            // e824f jne 0xe8269
            if fl.ne() { pc = 0xe8269; continue; }
            pc = 0xe8251;
        }
        0xe8251 => {
            // block e8251
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
            { let r = { let t = m.env.stub_va(0x8cb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe8269;
        }
        0xe8269 => {
            // block e8269
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
            // e828e call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x3c0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e1082 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1086 mov rdi,r12
            rdi = r12;
            // e1089 call QWORD PTR [rax+0x1a90]
            { let r = { let t = m.r64(rax.wrapping_add(0x1a90)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e108f cmp eax,0x3
            fl = sub::<32>((rax as u32 as u64), 0x3u64).1;
            // e1092 jne 0xe10b4
            if fl.ne() { pc = 0xe10b4; continue; }
            pc = 0xe1094;
        }
        0xe1094 => {
            // block e1094
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
            // e109f cmp ebx,0x1e6
            fl = sub::<32>((rbx as u32 as u64), 0x1e6u64).1;
            // e10a5 jne 0xe110b
            if fl.ne() { pc = 0xe110b; continue; }
            pc = 0xe10a7;
        }
        0xe10a7 => {
            // block e10a7
            // e10a7 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10ac mov r13d,0xf4
            r13 = (0xf4u64) as u32 as u64;
            // e10b2 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10b4 => {
            // block e10b4
            // e10b4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // e10b7 jne 0xe11d6
            if fl.ne() { pc = 0xe11d6; continue; }
            pc = 0xe10bd;
        }
        0xe10bd => {
            // block e10bd
            // e10bd cmp r14d,0x2d0
            fl = sub::<32>((r14 as u32 as u64), 0x2d0u64).1;
            // e10c4 je 0xe1125
            if fl.e() { pc = 0xe1125; continue; }
            pc = 0xe10c6;
        }
        0xe10c6 => {
            // block e10c6
            // e10c6 cmp r14d,0x500
            fl = sub::<32>((r14 as u32 as u64), 0x500u64).1;
            // e10cd je 0xe10e0
            if fl.e() { pc = 0xe10e0; continue; }
            pc = 0xe10cf;
        }
        0xe10cf => {
            // block e10cf
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
            // e10e0 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e10e5 mov r13d,r14d
            r13 = ((r14 as u32 as u64)) as u32 as u64;
            // e10e8 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe10ea => {
            // block e10ea
            // e10ea cmp ebx,0x2d0
            fl = sub::<32>((rbx as u32 as u64), 0x2d0u64).1;
            // e10f0 je 0xe10ff
            if fl.e() { pc = 0xe10ff; continue; }
            pc = 0xe10f2;
        }
        0xe10f2 => {
            // block e10f2
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
            // e1118 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e111d mov r13d,0x121
            r13 = (0x121u64) as u32 as u64;
            // e1123 jmp 0xe1130
            pc = 0xe1130; continue;
        }
        0xe1125 => {
            // block e1125
            // e1125 mov eax,0x8
            rax = (0x8u64) as u32 as u64;
            // e112a mov r13d,0x5a0
            r13 = (0x5a0u64) as u32 as u64;
            pc = 0xe1130;
        }
        0xe1130 => {
            // block e1130
            // e1130 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e1132 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // e1136 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // e113a mov rdi,r12
            rdi = r12;
            // e113d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.r64(rdx.wrapping_add(0x1ad0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe11d6;
        }
        0xe11d6 => {
            // block e11d6
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
            // ea050 mov rdi,rbx
            rdi = rbx;
            // ea053 call 0xebf0c
            { let r = { let t = m.env.stub_va(0x6d9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // ea06a test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // ea06d jne 0xea083
            if fl.ne() { pc = 0xea083; continue; }
            pc = 0xea06f;
        }
        0xea06f => {
            // block ea06f
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
            // ea097 mov rdx,rsi
            rdx = rsi;
            pc = 0xea09a;
        }
        0xea09a => {
            // block ea09a
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
            // ea0c3 mov rdx,r12
            rdx = r12;
            // ea0c6 call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea0cb cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea0ce jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea0d4;
        }
        0xea0d4 => {
            // block ea0d4
            // ea0d4 jmp 0xea1c8
            pc = 0xea1c8; continue;
        }
        0xea0d9 => {
            // block ea0d9
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
            // ea0f1 lea rax,[rbx+0x88]
            rax = rbx.wrapping_add(0x88);
            // ea0f8 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // ea0fc jmp 0xea102
            pc = 0xea102; continue;
        }
        0xea0fe => {
            // block ea0fe
            // ea0fe mov r9,QWORD PTR [rbx+0x40]
            r9 = m.r64(rbx.wrapping_add(0x40)) as u64;
            pc = 0xea102;
        }
        0xea102 => {
            // block ea102
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
            { let r = { let t = m.r64(r10.wrapping_add(0x18)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // ea136 cmp r13d,0x3
            fl = sub::<32>((r13 as u32 as u64), 0x3u64).1;
            // ea13a je 0xea1f9
            if fl.e() { pc = 0xea1f9; continue; }
            pc = 0xea140;
        }
        0xea140 => {
            // block ea140
            // ea140 cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea144 ja 0xea1e7
            if fl.a() { pc = 0xea1e7; continue; }
            pc = 0xea14a;
        }
        0xea14a => {
            // block ea14a
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
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea166 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // ea169 jne 0xea1e7
            if fl.ne() { pc = 0xea1e7; continue; }
            pc = 0xea16b;
        }
        0xea16b => {
            // block ea16b
            // ea16b cmp r13d,0x1
            fl = sub::<32>((r13 as u32 as u64), 0x1u64).1;
            // ea16f jne 0xea1c8
            if fl.ne() { pc = 0xea1c8; continue; }
            pc = 0xea171;
        }
        0xea171 => {
            // block ea171
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
            // ea19d mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // ea1a2 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1a7 mov rbx,rax
            rbx = rax;
            // ea1aa mov rdi,rbx
            rdi = rbx;
            // ea1ad call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ea1b2 mov rsi,QWORD PTR [rip+0x56027] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // ea1b9 mov rdx,QWORD PTR [rip+0x55fe8] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // ea1c0 mov rdi,rbx
            rdi = rbx;
            // ea1c3 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
        0xea1c8 => {
            // block ea1c8
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
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e93b8 test BYTE PTR [rbp-0x40],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64 & 0x1u64);
            // e93bc jne 0xe93c7
            if fl.ne() { pc = 0xe93c7; continue; }
            pc = 0xe93be;
        }
        0xe93be => {
            // block e93be
            // e93be lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // e93c2 mov rdi,r12
            rdi = r12;
            // e93c5 jmp 0xe93cf
            pc = 0xe93cf; continue;
        }
        0xe93c7 => {
            // block e93c7
            // e93c7 mov rdi,QWORD PTR [rbp-0x30]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e93cb lea r12,[rbp-0x3f]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            pc = 0xe93cf;
        }
        0xe93cf => {
            // block e93cf
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
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0xa63); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e9494 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // e9497 jmp 0xe949d
            pc = 0xe949d; continue;
        }
        0xe9499 => {
            // block e9499
            // e9499 mov rdx,QWORD PTR [rbp-0x50]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0xe949d;
        }
        0xe949d => {
            // block e949d
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
            // e94a9 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // e94ac jmp 0xe94b2
            pc = 0xe94b2; continue;
        }
        0xe94ae => {
            // block e94ae
            // e94ae mov rsi,QWORD PTR [rbp-0x70]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0xe94b2;
        }
        0xe94b2 => {
            // block e94b2
            // e94b2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e94b5 jne 0xe94fb
            if fl.ne() { pc = 0xe94fb; continue; }
            pc = 0xe94b7;
        }
        0xe94b7 => {
            // block e94b7
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
            // e94d1 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94d4 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94d6;
        }
        0xe94d6 => {
            // block e94d6
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
            // e94eb jmp 0xe9516
            pc = 0xe9516; continue;
        }
        0xe94ed => {
            // block e94ed
            // e94ed test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // e94f0 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94f2;
        }
        0xe94f2 => {
            // block e94f2
            // e94f2 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e94f7 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e94f9 je 0xe9516
            if fl.e() { pc = 0xe9516; continue; }
            pc = 0xe94fb;
        }
        0xe94fb => {
            // block e94fb
            // e94fb add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // e94ff mov rdi,r15
            rdi = r15;
            // e9502 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9507 cmp rbx,QWORD PTR [r14+0x270]
            fl = sub::<64>(rbx, m.r64(r14.wrapping_add(0x270)) as u64).1;
            // e950e jne 0xe9475
            if fl.ne() { pc = 0xe9475; continue; }
            pc = 0xe9514;
        }
        0xe9514 => {
            // block e9514
            // e9514 jmp 0xe952d
            pc = 0xe952d; continue;
        }
        0xe9516 => {
            // block e9516
            // e9516 mov QWORD PTR [r14+0x280],rbx
            m.w64(r14.wrapping_add(0x280), (rbx) as u64);
            // e951d lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // e9521 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9526 mov rbx,QWORD PTR [r14+0x270]
            rbx = m.r64(r14.wrapping_add(0x270)) as u64;
            pc = 0xe952d;
        }
        0xe952d => {
            // block e952d
            // e952d cmp QWORD PTR [r14+0x280],rbx
            fl = sub::<64>(m.r64(r14.wrapping_add(0x280)) as u64, rbx).1;
            // e9534 setne bl
            rbx = (rbx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e9537 lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e953b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9540 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e9544 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            // e955a mov rbx,rax
            rbx = rax;
            // e955d lea rdi,[rbp-0x58]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // e9561 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9566 jmp 0xe956b
            pc = 0xe956b; continue;
        }
        0xe9568 => {
            // block e9568
            // e9568 mov rbx,rax
            rbx = rax;
            pc = 0xe956b;
        }
        0xe956b => {
            // block e956b
            // e956b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // e956f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9574 mov rdi,rbx
            rdi = rbx;
            // e9577 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9b24 size=604
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000e9b24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9b24;
    loop {
        match pc {
        0xe9b24 => {
            // block e9b24
            // e9b24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9b25 mov rbp,rsp
            rbp = rsp;
            // e9b28 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9b2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9b2c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e9b2e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e9b30 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9b31 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9b38 mov r15,rdi
            r15 = rdi;
            // e9b3b mov rbx,QWORD PTR [rip+0x56896] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9b42 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9b45 mov QWORD PTR [rbp-0x30],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rbx) as u64);
            // e9b49 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            // e9b4c cmp QWORD PTR [r15+0x78],0x0
            fl = sub::<64>(m.r64(r15.wrapping_add(0x78)) as u64, 0x0u64).1;
            // e9b51 je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9b57;
        }
        0xe9b57 => {
            // block e9b57
            // e9b57 mov rax,QWORD PTR [r15+0x80]
            rax = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9b5e test rax,rax
            fl = logic::<64>(rax & rax);
            // e9b61 je 0xe9d55
            if fl.e() { pc = 0xe9d55; continue; }
            pc = 0xe9b67;
        }
        0xe9b67 => {
            // block e9b67
            // e9b67 mov ecx,DWORD PTR [r15+0x18c]
            rcx = (m.r32(r15.wrapping_add(0x18c)) as u64) as u32 as u64;
            // e9b6e test cl,0x10
            fl = logic::<8>((rcx as u8 as u64) & 0x10u64);
            // e9b71 jne 0xe9bad
            if fl.ne() { pc = 0xe9bad; continue; }
            pc = 0xe9b73;
        }
        0xe9b73 => {
            // block e9b73
            // e9b73 test cl,0x8
            fl = logic::<8>((rcx as u8 as u64) & 0x8u64);
            // e9b76 je 0xe9d32
            if fl.e() { pc = 0xe9d32; continue; }
            pc = 0xe9b7c;
        }
        0xe9b7c => {
            // block e9b7c
            // e9b7c lea rsi,[r15+0x108]
            rsi = r15.wrapping_add(0x108);
            // e9b83 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9b8a mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9b8f rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            // e9b92 cmp BYTE PTR [r15+0x192],0x0
            fl = sub::<8>(m.r8(r15.wrapping_add(0x192)) as u64, 0x0u64).1;
            // e9b9a je 0xe9c42
            if fl.e() { pc = 0xe9c42; continue; }
            pc = 0xe9ba0;
        }
        0xe9ba0 => {
            // block e9ba0
            // e9ba0 mov r14,QWORD PTR [r15+0x20]
            r14 = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9ba4 sub r14,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x18)) as u64);
              r14 = r;
              fl = f; }
            // e9ba8 jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9bad => {
            // block e9bad
            // e9bad mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // e9bb1 cmp rax,QWORD PTR [r15+0x28]
            fl = sub::<64>(rax, m.r64(r15.wrapping_add(0x28)) as u64).1;
            // e9bb5 je 0xe9bd4
            if fl.e() { pc = 0xe9bd4; continue; }
            pc = 0xe9bb7;
        }
        0xe9bb7 => {
            // block e9bb7
            // e9bb7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e9bba mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9bc0 mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // e9bc5 mov rdi,r15
            rdi = r15;
            // e9bc8 call QWORD PTR [rax+0x68]
            { let r = { let t = m.r64(rax.wrapping_add(0x68)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9bcb cmp eax,0xffffffff
            fl = sub::<32>((rax as u32 as u64), 0xffffffffu64).1;
            // e9bce je 0xe9d35
            if fl.e() { pc = 0xe9d35; continue; }
            pc = 0xe9bd4;
        }
        0xe9bd4 => {
            // block e9bd4
            // e9bd4 lea r14,[r15+0x88]
            r14 = r15.wrapping_add(0x88);
            // e9bdb mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            pc = 0xe9be1;
        }
        0xe9be1 => {
            // block e9be1
            // e9be1 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9be5 mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9bec mov rcx,QWORD PTR [r15+0x60]
            rcx = m.r64(r15.wrapping_add(0x60)) as u64;
            // e9bf0 add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // e9bf3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9bf6 mov rsi,r14
            rsi = r14;
            // e9bf9 lea r8,[rbp-0xb8]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // e9c00 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c03 mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // e9c05 mov r12,QWORD PTR [rbp-0xb8]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // e9c0c mov rdi,QWORD PTR [r15+0x40]
            rdi = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c10 mov rcx,QWORD PTR [r15+0x78]
            rcx = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c14 sub r12,rdi
            { let (r, f) = sub::<64>(r12, rdi);
              r12 = r;
              fl = f; }
            // e9c17 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // e9c1c mov rdx,r12
            rdx = r12;
            // e9c1f call 0xec1e8
            { let r = { let t = m.env.stub_va(0x9b5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c24 cmp rax,r12
            fl = sub::<64>(rax, r12).1;
            // e9c27 jne 0xe9c33
            if fl.ne() { pc = 0xe9c33; continue; }
            pc = 0xe9c29;
        }
        0xe9c29 => {
            // block e9c29
            // e9c29 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // e9c2c je 0xe9be1
            if fl.e() { pc = 0xe9be1; continue; }
            pc = 0xe9c2e;
        }
        0xe9c2e => {
            // block e9c2e
            // e9c2e cmp ebx,0x2
            fl = sub::<32>((rbx as u32 as u64), 0x2u64).1;
            // e9c31 jne 0xe9c6d
            if fl.ne() { pc = 0xe9c6d; continue; }
            pc = 0xe9c33;
        }
        0xe9c33 => {
            // block e9c33
            // e9c33 mov rbx,QWORD PTR [rip+0x5679e] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c3a mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c3d jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9c42 => {
            // block e9c42
            // e9c42 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e9c45 mov rdi,rax
            rdi = rax;
            // e9c48 call QWORD PTR [rcx+0x30]
            { let r = { let t = m.r64(rcx.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c4b mov rcx,QWORD PTR [r15+0x48]
            rcx = m.r64(r15.wrapping_add(0x48)) as u64;
            // e9c4f mov r14,QWORD PTR [r15+0x50]
            r14 = m.r64(r15.wrapping_add(0x50)) as u64;
            // e9c53 sub r14,rcx
            { let (r, f) = sub::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c56 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c58 jle 0xe9c8d
            if fl.le() { pc = 0xe9c8d; continue; }
            pc = 0xe9c5a;
        }
        0xe9c5a => {
            // block e9c5a
            // e9c5a cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9c5c mov rcx,QWORD PTR [r15+0x20]
            rcx = m.r64(r15.wrapping_add(0x20)) as u64;
            // e9c60 sub rcx,QWORD PTR [r15+0x18]
            { let (r, f) = sub::<64>(rcx, m.r64(r15.wrapping_add(0x18)) as u64);
              rcx = r;
              fl = f; }
            // e9c64 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // e9c68 add r14,rcx
            { let (r, f) = add::<64>(r14, rcx);
              r14 = r;
              fl = f; }
            // e9c6b jmp 0xe9cc5
            pc = 0xe9cc5; continue;
        }
        0xe9c6d => {
            // block e9c6d
            // e9c6d mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9c71 call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9c76 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9c78 mov rbx,QWORD PTR [rip+0x56759] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // e9c7f mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // e9c82 jne 0xe9d35
            if fl.ne() { pc = 0xe9d35; continue; }
            pc = 0xe9c88;
        }
        0xe9c88 => {
            // block e9c88
            // e9c88 jmp 0xe9d32
            pc = 0xe9d32; continue;
        }
        0xe9c8d => {
            // block e9c8d
            // e9c8d mov r8,QWORD PTR [r15+0x18]
            r8 = m.r64(r15.wrapping_add(0x18)) as u64;
            // e9c91 cmp r8,QWORD PTR [r15+0x20]
            fl = sub::<64>(r8, m.r64(r15.wrapping_add(0x20)) as u64).1;
            // e9c95 je 0xe9cc5
            if fl.e() { pc = 0xe9cc5; continue; }
            pc = 0xe9c97;
        }
        0xe9c97 => {
            // block e9c97
            // e9c97 mov rdx,QWORD PTR [r15+0x40]
            rdx = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9c9b mov rdi,QWORD PTR [r15+0x80]
            rdi = m.r64(r15.wrapping_add(0x80)) as u64;
            // e9ca2 sub r8,QWORD PTR [r15+0x10]
            { let (r, f) = sub::<64>(r8, m.r64(r15.wrapping_add(0x10)) as u64);
              r8 = r;
              fl = f; }
            // e9ca6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e9ca9 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cb0 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cb3 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // e9cb5 sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // e9cb8 add r14,QWORD PTR [r15+0x48]
            { let (r, f) = add::<64>(r14, m.r64(r15.wrapping_add(0x48)) as u64);
              r14 = r;
              fl = f; }
            // e9cbc sub r14,QWORD PTR [r15+0x40]
            { let (r, f) = sub::<64>(r14, m.r64(r15.wrapping_add(0x40)) as u64);
              r14 = r;
              fl = f; }
            // e9cc0 mov r12b,0x1
            r12 = (r12 & !0xff) | ((0x1u64) as u8 as u64);
            // e9cc3 jmp 0xe9cc8
            pc = 0xe9cc8; continue;
        }
        0xe9cc5 => {
            // block e9cc5
            // e9cc5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9cc8;
        }
        0xe9cc8 => {
            // block e9cc8
            // e9cc8 mov rdi,QWORD PTR [r15+0x78]
            rdi = m.r64(r15.wrapping_add(0x78)) as u64;
            // e9ccc neg r14
            { let (r, f) = neg::<64>(r14); r14 = r; fl = f; }
            // e9ccf mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // e9cd4 mov rsi,r14
            rsi = r14;
            // e9cd7 call 0xec1dc
            { let r = { let t = m.env.stub_va(0x9a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9cdc test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // e9cde je 0xe9ce8
            if fl.e() { pc = 0xe9ce8; continue; }
            pc = 0xe9ce0;
        }
        0xe9ce0 => {
            // block e9ce0
            // e9ce0 mov r13d,0xffffffff
            r13 = (0xffffffffu64) as u32 as u64;
            // e9ce6 jmp 0xe9d35
            pc = 0xe9d35; continue;
        }
        0xe9ce8 => {
            // block e9ce8
            // e9ce8 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // e9ceb je 0xe9d03
            if fl.e() { pc = 0xe9d03; continue; }
            pc = 0xe9ced;
        }
        0xe9ced => {
            // block e9ced
            // e9ced lea rdi,[r15+0x88]
            rdi = r15.wrapping_add(0x88);
            // e9cf4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // e9cfb mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e9d00 rep movs QWORD PTR [rdi],QWORD PTR [rsi]
            while rcx != 0 { let v = m.r64(rsi); m.w64(rdi, v); rsi = rsi.wrapping_add(8); rdi = rdi.wrapping_add(8); rcx = rcx.wrapping_sub(1); }
            pc = 0xe9d03;
        }
        0xe9d03 => {
            // block e9d03
            // e9d03 mov rax,QWORD PTR [r15+0x40]
            rax = m.r64(r15.wrapping_add(0x40)) as u64;
            // e9d07 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // e9d0b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // e9d0f mov DWORD PTR [r15+0x18c],0x0
            m.w32(r15.wrapping_add(0x18c), (0x0u64) as u32);
            // e9d1a mov QWORD PTR [r15+0x20],0x0
            m.w64(r15.wrapping_add(0x20), (0x0u64) as u64);
            // e9d22 mov QWORD PTR [r15+0x18],0x0
            m.w64(r15.wrapping_add(0x18), (0x0u64) as u64);
            // e9d2a mov QWORD PTR [r15+0x10],0x0
            m.w64(r15.wrapping_add(0x10), (0x0u64) as u64);
            pc = 0xe9d32;
        }
        0xe9d32 => {
            // block e9d32
            // e9d32 xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0xe9d35;
        }
        0xe9d35 => {
            // block e9d35
            // e9d35 cmp rbx,QWORD PTR [rbp-0x30]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // e9d39 jne 0xe9d50
            if fl.ne() { pc = 0xe9d50; continue; }
            pc = 0xe9d3b;
        }
        0xe9d3b => {
            // block e9d3b
            // e9d3b mov eax,r13d
            rax = ((r13 as u32 as u64)) as u32 as u64;
            // e9d3e add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9d45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9d46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e9d48 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e9d4a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9d4c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e9d4e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9d4f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe9d50 => {
            // block e9d50
            // e9d50 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9d55;
        }
        0xe9d55 => {
            // block e9d55
            // e9d55 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // e9d5a call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d5f mov rbx,rax
            rbx = rax;
            // e9d62 mov rdi,rbx
            rdi = rbx;
            // e9d65 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9d6a mov rsi,QWORD PTR [rip+0x5646f] # 0x1401e0
            rsi = m.r64(m.env.data_va(346541)) as u64;
            // e9d71 mov rdx,QWORD PTR [rip+0x56430] # 0x1401a8
            rdx = m.r64(m.env.data_va(346485)) as u64;
            // e9d78 mov rdi,rbx
            rdi = rbx;
            // e9d7b call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68d40 size=682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068d40<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68d40;
    loop {
        match pc {
        0x68d40 => {
            // block 68d40
            // 68d40 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68d41 mov rbp,rsp
            rbp = rsp;
            // 68d44 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 68d46 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 68d48 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 68d4a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 68d4c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68d4d sub rsp,0x168
            { let (r, f) = sub::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 68d54 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 68d57 mov r12d,ecx
            r12 = ((rcx as u32 as u64)) as u32 as u64;
            // 68d5a mov DWORD PTR [rbp-0x180],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe80), ((rdx as u32 as u64)) as u32);
            // 68d60 mov r13,rsi
            r13 = rsi;
            // 68d63 mov QWORD PTR [rbp-0x170],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rdi) as u64);
            // 68d6a mov DWORD PTR [rbp-0x44],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rdx as u32 as u64)) as u32);
            // 68d6d lea rdi,[r13+0x180]
            rdi = r13.wrapping_add(0x180);
            // 68d74 lea rsi,[rbp-0x44]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffbc);
            // 68d78 call 0xebbe8
            { let r = { let t = m.env.stub_va(0x3b5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68d7d mov r14,rax
            r14 = rax;
            // 68d80 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 68d87 lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 68d8e mov rax,QWORD PTR [rip+0xd743b] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 68d95 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 68d99 mov QWORD PTR [rbp-0x150],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rcx) as u64);
            // 68da0 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 68da4 mov QWORD PTR [rbp-0xe0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (rax) as u64);
            // 68dab mov rsi,rbx
            rsi = rbx;
            // 68dae call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68db3 mov DWORD PTR [rbp-0x188],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe78), ((r12 as u32 as u64)) as u32);
            // 68dba mov DWORD PTR [rbp-0x184],r15d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe7c), ((r15 as u32 as u64)) as u32);
            // 68dc1 mov QWORD PTR [rbp-0x58],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0x0u64) as u64);
            // 68dc9 mov DWORD PTR [rbp-0x50],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0xffffffffu64) as u32);
            // 68dd0 mov r15,QWORD PTR [rip+0xd75e9] # 0x1403c0
            r15 = m.r64(m.env.data_va(347021)) as u64;
            // 68dd7 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 68ddb mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // 68de2 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 68de9 add r15,0x40
            { let (r, f) = add::<64>(r15, 0x40u64);
              r15 = r;
              fl = f; }
            // 68ded mov QWORD PTR [rbp-0xe0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r15) as u64);
            // 68df4 mov rdi,rbx
            rdi = rbx;
            // 68df7 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68dfc mov rdi,rbx
            rdi = rbx;
            // 68dff mov r12,QWORD PTR [rip+0xd75a2] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // 68e06 add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // 68e0a mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 68e11 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // 68e1c mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // 68e27 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // 68e32 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // 68e3d mov DWORD PTR [rbp-0xe8],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x10u64) as u32);
            // 68e47 xorps xmm0,xmm0
            x0 = 0;
            // 68e4a movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 68e4e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 68e56 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 68e5a call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68e5f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 68e63 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68e68 add r13,0x188
            { let (r, f) = add::<64>(r13, 0x188u64);
              r13 = r;
              fl = f; }
            // 68e6f cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 68e72 mov r13,QWORD PTR [rbp-0x170]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // 68e79 je 0x68ee4
            if fl.e() { pc = 0x68ee4; continue; }
            pc = 0x68e7b;
        }
        0x68e7b => {
            // block 68e7b
            // 68e7b mov rsi,QWORD PTR [r14+0x28]
            rsi = m.r64(r14.wrapping_add(0x28)) as u64;
            // 68e7f test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 68e82 je 0x68ee4
            if fl.e() { pc = 0x68ee4; continue; }
            pc = 0x68e84;
        }
        0x68e84 => {
            // block 68e84
            // 68e84 mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // 68e87 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 68e8a lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 68e91 mov edx,DWORD PTR [rbp-0x180]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64) as u32 as u64;
            // 68e97 mov ecx,DWORD PTR [rbp-0x188]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe78)) as u64) as u32 as u64;
            // 68e9d mov r8d,DWORD PTR [rbp-0x184]
            r8 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fe7c)) as u64) as u32 as u64;
            // 68ea4 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68ea6 movzx edx,BYTE PTR [rbp-0x168]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64) as u32 as u64;
            // 68ead test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // 68eb0 jne 0x68ebe
            if fl.ne() { pc = 0x68ebe; continue; }
            pc = 0x68eb2;
        }
        0x68eb2 => {
            // block 68eb2
            // 68eb2 lea rsi,[rbp-0x167]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe99);
            // 68eb9 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 68ebc jmp 0x68ecc
            pc = 0x68ecc; continue;
        }
        0x68ebe => {
            // block 68ebe
            // 68ebe mov rdx,QWORD PTR [rbp-0x160]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 68ec5 mov rsi,QWORD PTR [rbp-0x158]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            pc = 0x68ecc;
        }
        0x68ecc => {
            // block 68ecc
            // 68ecc lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 68ed3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68ed8 lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 68edf call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x68ee4;
        }
        0x68ee4 => {
            // block 68ee4
            // 68ee4 mov rdi,r13
            rdi = r13;
            // 68ee7 lea rbx,[rbp-0x148]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 68eee mov rsi,rbx
            rsi = rbx;
            // 68ef1 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68ef6 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 68efd mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 68f04 mov QWORD PTR [rbp-0xe0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r15) as u64);
            // 68f0b mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 68f12 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 68f19 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68f1e mov rdi,rbx
            rdi = rbx;
            // 68f21 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68f26 mov rsi,QWORD PTR [rip+0xd72db] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 68f2d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 68f31 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 68f38 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68f3d lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 68f44 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68f49 mov rax,r13
            rax = r13;
            // 68f4c add rsp,0x168
            { let (r, f) = add::<64>(rsp, 0x168u64);
              rsp = r;
              fl = f; }
            // 68f53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68f54 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 68f56 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 68f58 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 68f5a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 68f5c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68f5d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x68f5e => {
            // block 68f5e
            // 68f5e mov r14,rax
            r14 = rax;
            // 68f61 jmp 0x68f8a
            pc = 0x68f8a; continue;
        }
        0x68f63 => {
            // block 68f63
            // 68f63 mov r14,rax
            r14 = rax;
            // 68f66 jmp 0x68fd5
            pc = 0x68fd5; continue;
        }
        0x68f68 => {
            // block 68f68
            // 68f68 mov r14,rax
            r14 = rax;
            // 68f6b jmp 0x68fbe
            pc = 0x68fbe; continue;
        }
        0x68f6d => {
            // block 68f6d
            // 68f6d mov r14,rax
            r14 = rax;
            // 68f70 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 68f74 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68f79 jmp 0x68fa6
            pc = 0x68fa6; continue;
        }
        0x68f7b => {
            // block 68f7b
            // 68f7b mov r14,rax
            r14 = rax;
            // 68f7e lea rdi,[rbp-0x168]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe98);
            // 68f85 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x68f8a;
        }
        0x68f8a => {
            // block 68f8a
            // 68f8a mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // 68f91 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // 68f98 mov QWORD PTR [rbp-0xe0],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (r15) as u64);
            // 68f9f mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            pc = 0x68fa6;
        }
        0x68fa6 => {
            // block 68fa6
            // 68fa6 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 68fad call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68fb2 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // 68fb9 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x68fbe;
        }
        0x68fbe => {
            // block 68fbe
            // 68fbe mov rsi,QWORD PTR [rip+0xd7243] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // 68fc5 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 68fc9 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // 68fd0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x68fd5;
        }
        0x68fd5 => {
            // block 68fd5
            // 68fd5 lea rdi,[rbp-0xe0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff20);
            // 68fdc call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68fe1 mov rdi,r14
            rdi = r14;
            // 68fe4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba4 size=146
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000ba4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba4;
    loop {
        match pc {
        0xba4 => {
            // block ba4
            // ba4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba5 mov rbp,rsp
            rbp = rsp;
            // ba8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // baa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // bac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bad sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // bb1 mov r14,rsi
            r14 = rsi;
            // bb4 mov rbx,rdi
            rbx = rdi;
            // bb7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bba call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bc0 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // bc3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bc6 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // bca mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // bce lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // bd2 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // bd6 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // bda lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // bde lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // be2 mov rdi,rbx
            rdi = rbx;
            // be5 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // beb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // bed je 0xc1c
            if fl.e() { pc = 0xc1c; continue; }
            pc = 0xbef;
        }
        0xbef => {
            // block bef
            // bef mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // bf2 lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // bf6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bf8 mov rdi,rbx
            rdi = rbx;
            // bfb call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c01 movzx eax,WORD PTR [rbp-0x34]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // c05 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // c08 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // c0b call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c10 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c12 je 0xc20
            if fl.e() { pc = 0xc20; continue; }
            pc = 0xc14;
        }
        0xc14 => {
            // block c14
            // c14 mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // c17 sub eax,DWORD PTR [rbp-0x1c]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1a jmp 0xc26
            pc = 0xc26; continue;
        }
        0xc1c => {
            // block c1c
            // c1c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1e jmp 0xc2b
            pc = 0xc2b; continue;
        }
        0xc20 => {
            // block c20
            // c20 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // c23 sub eax,DWORD PTR [rbp-0x34]
            { let (r, f) = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xc26;
        }
        0xc26 => {
            // block c26
            // c26 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // c29 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xc2b;
        }
        0xc2b => {
            // block c2b
            // c2b add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c2f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c30 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c32 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c35 ret
            return Ret { rax, rdx, xmm0: x0 };
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
            // d64 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d66 jmp 0xd73
            pc = 0xd73; continue;
        }
        0xd68 => {
            // block d68
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
            // d6e mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            // d71 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd73;
        }
        0xd73 => {
            // block d73
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

/// parity: P2v @0xc36 size=182
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000c36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc36;
    loop {
        match pc {
        0xc36 => {
            // block c36
            // c36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c37 mov rbp,rsp
            rbp = rsp;
            // c3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3f sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // c43 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // c46 mov rbx,rdi
            rbx = rdi;
            // c49 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4c call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c52 mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // c55 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c58 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // c5c mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // c60 lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // c64 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // c68 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // c6c lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c70 lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // c74 mov rdi,rbx
            rdi = rbx;
            // c77 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c7d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c7f je 0xcde
            if fl.e() { pc = 0xcde; continue; }
            pc = 0xc81;
        }
        0xc81 => {
            // block c81
            // c81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // c84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // c8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // c94 add ecx,DWORD PTR [rbp-0x28]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c97 mov DWORD PTR [rbp-0x28],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rcx as u32 as u64)) as u32);
            // c9a mov eax,DWORD PTR [rbp-0x30]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // c9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // c9f jg 0xca8
            if fl.g() { pc = 0xca8; continue; }
            pc = 0xca1;
        }
        0xca1 => {
            // block ca1
            // ca1 mov eax,DWORD PTR [rbp-0x2c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64) as u32 as u64;
            // ca4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // ca6 jge 0xcab
            if fl.ge() { pc = 0xcab; continue; }
            pc = 0xca8;
        }
        0xca8 => {
            // block ca8
            // ca8 mov DWORD PTR [rbp-0x28],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rax as u32 as u64)) as u32);
            pc = 0xcab;
        }
        0xcab => {
            // block cab
            // cab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // cae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // cb2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // cb4 mov rdi,rbx
            rdi = rbx;
            // cb7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cbd movzx eax,WORD PTR [rbp-0x34]
            rax = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // cc1 mov esi,DWORD PTR [rbp-0x28]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u32 as u64;
            // cc4 shl esi,0x10
            { let (r, f) = shl::<32>((rsi as u32 as u64), 0x10u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // cc7 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // cc9 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ccc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ccf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // cd1 mov rdi,rbx
            rdi = rbx;
            // cd4 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // cda mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // cdc jmp 0xce0
            pc = 0xce0; continue;
        }
        0xcde => {
            // block cde
            // cde xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xce0;
        }
        0xce0 => {
            // block ce0
            // ce0 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // ce4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ce5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ce7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ce9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa32 size=370
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00000a32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa32;
    loop {
        match pc {
        0xa32 => {
            // block a32
            // a32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a33 mov rbp,rsp
            rbp = rsp;
            // a36 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a38 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a3a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a3b sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // a3f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // a42 mov rbx,rdi
            rbx = rdi;
            // a45 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a48 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a4e mov r15d,eax
            r15 = ((rax as u32 as u64)) as u32 as u64;
            // a51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // a54 lea rcx,[rbp-0x30]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a58 mov QWORD PTR [rsp],rcx
            m.w64(rsp, (rcx) as u64);
            // a5c lea rsi,[rbp-0x1c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // a60 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // a64 lea rcx,[rbp-0x24]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // a68 lea r8,[rbp-0x28]
            r8 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // a6c lea r9,[rbp-0x2c]
            r9 = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // a70 mov rdi,rbx
            rdi = rbx;
            // a73 call QWORD PTR [rax+0xd10]
            { let r = { let t = m.r64(rax.wrapping_add(0xd10)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a79 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a7b je 0xb26
            if fl.e() { pc = 0xb26; continue; }
            pc = 0xa81;
        }
        0xa81 => {
            // block a81
            // a81 mov edi,r15d
            rdi = ((r15 as u32 as u64)) as u32 as u64;
            // a84 call 0xce033
            { let r = { let t = m.env.code_va(0xce033); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a89 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // a8c neg ecx
            { let (r, f) = neg::<32>((rcx as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // a8e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // a90 cmovne ecx,r14d
            if fl.ne() { rcx = ((r14 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // a94 add ecx,DWORD PTR [rbp-0x1c]
            { let (r, f) = add::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a97 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // a9a mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // a9d cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // a9f jg 0xaa8
            if fl.g() { pc = 0xaa8; continue; }
            pc = 0xaa1;
        }
        0xaa1 => {
            // block aa1
            // aa1 mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // aa4 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // aa6 jge 0xaab
            if fl.ge() { pc = 0xaab; continue; }
            pc = 0xaa8;
        }
        0xaa8 => {
            // block aa8
            // aa8 mov DWORD PTR [rbp-0x1c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rax as u32 as u64)) as u32);
            pc = 0xaab;
        }
        0xaab => {
            // block aab
            // aab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // aae lea rsi,[rbp-0x34]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffcc);
            // ab2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab4 mov rdi,rbx
            rdi = rbx;
            // ab7 call QWORD PTR [rax+0xd60]
            { let r = { let t = m.r64(rax.wrapping_add(0xd60)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abd mov eax,DWORD PTR [rbp-0x34]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // ac0 movzx ecx,ax
            rcx = ((rax as u16 as u64)) as u32 as u64;
            // ac3 mov esi,DWORD PTR [rbp-0x1c]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // ac6 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // ac9 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // acb je 0xb95
            if fl.e() { pc = 0xb95; continue; }
            pc = 0xad1;
        }
        0xad1 => {
            // block ad1
            // ad1 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // ad4 cmp edx,esi
            fl = sub::<32>((rdx as u32 as u64), (rsi as u32 as u64)).1;
            // ad6 jne 0xb2b
            if fl.ne() { pc = 0xb2b; continue; }
            pc = 0xad8;
        }
        0xad8 => {
            // block ad8
            // ad8 and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // add add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae0 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // ae2 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // ae5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // ae8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aea mov rdi,rbx
            rdi = rbx;
            // aed call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // af6 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // afa mov rdi,rbx
            rdi = rbx;
            // afd call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b03 lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb07;
        }
        0xb07 => {
            // block b07
            // b07 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b0a mov rdi,rbx
            rdi = rbx;
            // b0d mov rsi,r15
            rsi = r15;
            // b10 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b16 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b19 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b1c jne 0xb07
            if fl.ne() { pc = 0xb07; continue; }
            pc = 0xb1e;
        }
        0xb1e => {
            // block b1e
            // b1e mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b21 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b24 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb26 => {
            // block b26
            // b26 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b29 jmp 0xb95
            pc = 0xb95; continue;
        }
        0xb2b => {
            // block b2b
            // b2b dec ecx
            { let (r, f) = dec::<32>((rcx as u32 as u64), fl); rcx = (r) as u32 as u64; fl = f; }
            // b2d and eax,0xffff0000
            { let r = (rax as u32 as u64) & 0xffff0000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b32 mov DWORD PTR [rbp-0x34],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rax as u32 as u64)) as u32);
            // b35 cmp ecx,esi
            fl = sub::<32>((rcx as u32 as u64), (rsi as u32 as u64)).1;
            // b37 jne 0xb82
            if fl.ne() { pc = 0xb82; continue; }
            pc = 0xb39;
        }
        0xb39 => {
            // block b39
            // b39 add esi,0xfffffffe
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xfffffffeu64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3c or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b3e mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b41 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b44 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b46 mov rdi,rbx
            rdi = rbx;
            // b49 call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b4f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b52 lea rsi,[rbp-0x38]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // b56 mov rdi,rbx
            rdi = rbx;
            // b59 call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f lea r15,[rbp-0x3c]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc4);
            pc = 0xb63;
        }
        0xb63 => {
            // block b63
            // b63 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b66 mov rdi,rbx
            rdi = rbx;
            // b69 mov rsi,r15
            rsi = r15;
            // b6c call QWORD PTR [rax+0x678]
            { let r = { let t = m.r64(rax.wrapping_add(0x678)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b72 mov eax,DWORD PTR [rbp-0x38]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // b75 cmp eax,DWORD PTR [rbp-0x3c]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4)) as u64).1;
            // b78 jne 0xb63
            if fl.ne() { pc = 0xb63; continue; }
            pc = 0xb7a;
        }
        0xb7a => {
            // block b7a
            // b7a mov esi,DWORD PTR [rbp-0x34]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u32 as u64;
            // b7d add esi,0x2
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x2u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // b80 jmp 0xb84
            pc = 0xb84; continue;
        }
        0xb82 => {
            // block b82
            // b82 or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xb84;
        }
        0xb84 => {
            // block b84
            // b84 mov DWORD PTR [rbp-0x34],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffcc), ((rsi as u32 as u64)) as u32);
            // b87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b8a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b8c mov rdi,rbx
            rdi = rbx;
            // b8f call QWORD PTR [rax+0xd50]
            { let r = { let t = m.r64(rax.wrapping_add(0xd50)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb95;
        }
        0xb95 => {
            // block b95
            // b95 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // b98 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // b9c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa8e86 size=888
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a8e86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa8e86;
    loop {
        match pc {
        0xa8e86 => {
            // block a8e86
            // a8e86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a8e87 mov rbp,rsp
            rbp = rsp;
            // a8e8a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a8e8c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a8e8e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a8e90 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a8e92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a8e93 sub rsp,0x158
            { let (r, f) = sub::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // a8e9a mov DWORD PTR [rbp-0x15c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fea4), ((rcx as u32 as u64)) as u32);
            // a8ea0 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // a8ea3 mov r12,rdi
            r12 = rdi;
            // a8ea6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a8ead lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a8eb4 mov rax,QWORD PTR [rip+0x97315] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a8ebb lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a8ebf mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a8ec6 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a8eca mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a8ed1 mov rsi,r15
            rsi = r15;
            // a8ed4 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8ed9 mov QWORD PTR [rbp-0x180],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (r12) as u64);
            // a8ee0 mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a8ee8 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a8eef mov rbx,QWORD PTR [rip+0x974ca] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a8ef6 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // a8efa mov QWORD PTR [rbp-0x158],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r12) as u64);
            // a8f01 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a8f05 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // a8f0c mov rdi,r15
            rdi = r15;
            // a8f0f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f14 mov QWORD PTR [rbp-0x170],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (r12) as u64);
            // a8f1b mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // a8f22 mov rax,QWORD PTR [rip+0x9747f] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // a8f29 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // a8f2d mov QWORD PTR [rbp-0x178],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe88), (rax) as u64);
            // a8f34 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a8f3b lea rbx,[rbp-0x110]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a8f42 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a8f4d mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a8f58 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a8f63 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a8f6e mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a8f78 xorps xmm0,xmm0
            x0 = 0;
            // a8f7b movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a8f7f mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a8f87 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a8f8b mov rdi,r15
            rdi = r15;
            // a8f8e call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f93 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a8f97 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8f9c cmp r14d,0x17
            fl = sub::<32>((r14 as u32 as u64), 0x17u64).1;
            // a8fa0 je 0xa8faf
            if fl.e() { pc = 0xa8faf; continue; }
            pc = 0xa8fa2;
        }
        0xa8fa2 => {
            // block a8fa2
            // a8fa2 cmp r14d,0x11a
            fl = sub::<32>((r14 as u32 as u64), 0x11au64).1;
            // a8fa9 jne 0xa90c1
            if fl.ne() { pc = 0xa90c1; continue; }
            pc = 0xa8faf;
        }
        0xa8faf => {
            // block a8faf
            // a8faf xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // a8fb2 lea r14,[rbp-0x158]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a8fb9 lea r13,[rbp-0x30]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            pc = 0xa8fbd;
        }
        0xa8fbd => {
            // block a8fbd
            // a8fbd mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // a8fc2 mov rdi,r14
            rdi = r14;
            // a8fc5 lea rsi,[rip+0x8b47a] # 0x134446
            rsi = m.env.data_va(298003);
            // a8fcc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8fd1 mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // a8fd4 and esi,0xfffe
            { let r = (rsi as u32 as u64) & 0xfffeu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // a8fda shr esi,1
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0x1u64, fl); rsi = (r) as u32 as u64; fl = f; }
            // a8fdc mov rdi,rax
            rdi = rax;
            // a8fdf call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8fe4 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // a8fe9 mov rdi,rax
            rdi = rax;
            // a8fec lea rsi,[rip+0x8b45a] # 0x13444d
            rsi = m.env.data_va(298010);
            // a8ff3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a8ff8 mov ecx,r12d
            rcx = ((r12 as u32 as u64)) as u32 as u64;
            // a8ffb and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // a8ffe lea rsi,[rip+0x8b453] # 0x134458
            rsi = m.env.data_va(298021);
            // a9005 lea rcx,[rip+0x8b446] # 0x134452
            rcx = m.env.data_va(298015);
            // a900c cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // a9010 mov edx,0x5
            rdx = (0x5u64) as u32 as u64;
            // a9015 mov rdi,rax
            rdi = rax;
            // a9018 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a901d mov ecx,DWORD PTR [rbp-0x15c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fea4)) as u64) as u32 as u64;
            // a9023 bt ecx,r12d
            fl.cf = bit::<32>((rcx as u32 as u64), (r12 as u32 as u64));
            // a9027 lea rsi,[rip+0x8b438] # 0x134466
            rsi = m.env.data_va(298035);
            // a902e lea rcx,[rip+0x8b429] # 0x13445e
            rcx = m.env.data_va(298027);
            // a9035 cmovb rsi,rcx
            if fl.b() { rsi = rcx; }
            // a9039 sbb rdx,rdx
            { let (r, f) = sbb::<64>(rdx, rdx, fl.cf);
              rdx = r;
              fl = f; }
            // a903c and rdx,0x1
            { let r = rdx & 0x1u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a9040 or rdx,0x6
            { let r = rdx | 0x6u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // a9044 mov rdi,rax
            rdi = rax;
            // a9047 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a904c inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // a904f cmp r12d,0x7
            fl = sub::<32>((r12 as u32 as u64), 0x7u64).1;
            // a9053 ja 0xa90c1
            if fl.a() { pc = 0xa90c1; continue; }
            pc = 0xa9055;
        }
        0xa9055 => {
            // block a9055
            // a9055 mov rax,QWORD PTR [rbp-0x158]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // a905c mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a9060 add rsi,r14
            { let (r, f) = add::<64>(rsi, r14);
              rsi = r;
              fl = f; }
            // a9063 mov rdi,r13
            rdi = r13;
            // a9066 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a906b mov rdi,r13
            rdi = r13;
            // a906e mov rsi,QWORD PTR [rip+0x9711b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a9075 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a907a mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a907d mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a9081 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a9086 mov rdi,rax
            rdi = rax;
            // a9089 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a908b mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a908e mov rdi,r13
            rdi = r13;
            // a9091 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9096 movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // a909a mov rdi,r14
            rdi = r14;
            // a909d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90a2 mov rdi,r14
            rdi = r14;
            // a90a5 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90aa jmp 0xa8fbd
            pc = 0xa8fbd; continue;
        }
        0xa90af => {
            // block a90af
            // a90af mov r15,rbx
            r15 = rbx;
            // a90b2 mov r14,rax
            r14 = rax;
            // a90b5 lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a90bc jmp 0xa9165
            pc = 0xa9165; continue;
        }
        0xa90c1 => {
            // block a90c1
            // a90c1 mov r15,rbx
            r15 = rbx;
            // a90c4 mov r14,QWORD PTR [rbp-0x180]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe80)) as u64;
            // a90cb mov rdi,r14
            rdi = r14;
            // a90ce lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a90d5 mov rsi,rbx
            rsi = rbx;
            // a90d8 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a90dd mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a90e4 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a90eb mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a90f2 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a90f9 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a9100 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a9107 mov rdi,r15
            rdi = r15;
            // a910a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a910f mov rdi,rbx
            rdi = rbx;
            // a9112 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9117 mov rsi,QWORD PTR [rip+0x970ea] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a911e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a9122 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a9129 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a912e lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a9135 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a913a mov rax,r14
            rax = r14;
            // a913d add rsp,0x158
            { let (r, f) = add::<64>(rsp, 0x158u64);
              rsp = r;
              fl = f; }
            // a9144 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a9145 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a9147 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a9149 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a914b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a914d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a914e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa914f => {
            // block a914f
            // a914f mov r15,rbx
            r15 = rbx;
            // a9152 mov r14,rax
            r14 = rax;
            // a9155 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a9159 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a915e lea rbx,[rbp-0x150]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            pc = 0xa9165;
        }
        0xa9165 => {
            // block a9165
            // a9165 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a916c mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a9173 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a917a mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a9181 mov rax,QWORD PTR [rbp-0x178]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe88)) as u64;
            // a9188 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a918f mov rdi,r15
            rdi = r15;
            // a9192 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a9197 mov rdi,rbx
            rdi = rbx;
            // a919a call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa919f;
        }
        0xa919f => {
            // block a919f
            // a919f mov rsi,QWORD PTR [rip+0x97062] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a91a6 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a91aa lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a91b1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa91b6;
        }
        0xa91b6 => {
            // block a91b6
            // a91b6 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a91bd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91c2 mov rdi,r14
            rdi = r14;
            // a91c5 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91ca mov r14,rax
            r14 = rax;
            // a91cd jmp 0xa91b6
            pc = 0xa91b6; continue;
        }
        0xa91cf => {
            // block a91cf
            // a91cf mov r14,rax
            r14 = rax;
            // a91d2 jmp 0xa919f
            pc = 0xa919f; continue;
        }
        0xa91d4 => {
            // block a91d4
            // a91d4 mov r14,rax
            r14 = rax;
            // a91d7 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a91db call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91e0 mov rdi,rbx
            rdi = rbx;
            // a91e3 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91e8 lea rdi,[rbp-0x150]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a91ef call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a91f4 jmp 0xa919f
            pc = 0xa919f; continue;
        }
        0xa91f6 => {
            // block a91f6
            // a91f6 mov r14,rax
            r14 = rax;
            // a91f9 jmp 0xa9165
            pc = 0xa9165; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3192a size=930
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0003192a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3192a;
    loop {
        match pc {
        0x3192a => {
            // block 3192a
            // 3192a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3192b mov rbp,rsp
            rbp = rsp;
            // 3192e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31930 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31932 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31934 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31936 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31937 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3193e mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 31941 mov rbx,rsi
            rbx = rsi;
            // 31944 mov r15,rdi
            r15 = rdi;
            // 31947 mov r12,QWORD PTR [rip+0x10ea8a] # 0x1403d8
            r12 = m.r64(m.env.data_va(347045)) as u64;
            // 3194e mov r12,QWORD PTR [r12]
            r12 = m.r64(r12) as u64;
            // 31952 mov QWORD PTR [rbp-0x30],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r12) as u64);
            // 31956 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31959 mov rdi,rbx
            rdi = rbx;
            // 3195c call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31962 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31964 je 0x31c8f
            if fl.e() { pc = 0x31c8f; continue; }
            pc = 0x3196a;
        }
        0x3196a => {
            // block 3196a
            // 3196a mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // 3196d lea rcx,[rip+0xce4ec] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31974 mov r13d,DWORD PTR [rcx+rax*4]
            r13 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3197b lea rdx,[rbp-0x80]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 3197f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 31982 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 31987 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3198a mov rdi,rbx
            rdi = rbx;
            // 3198d mov esi,r13d
            rsi = ((r13 as u32 as u64)) as u32 as u64;
            // 31990 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31993 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31995 je 0x31c92
            if fl.e() { pc = 0x31c92; continue; }
            pc = 0x3199b;
        }
        0x3199b => {
            // block 3199b
            // 3199b mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 319a2 mov eax,0xffffcccc
            rax = (0xffffccccu64) as u32 as u64;
            // 319a7 and eax,DWORD PTR [rbp-0x80]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319aa mov ecx,0xffffcccf
            rcx = (0xffffcccfu64) as u32 as u64;
            // 319af and ecx,DWORD PTR [r15+0x8]
            { let r = (rcx as u32 as u64) & m.r32(r15.wrapping_add(0x8)) as u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 319b3 or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 319b5 mov eax,DWORD PTR [r15+0xc]
            rax = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 319b9 mov edx,DWORD PTR [r15+0x10]
            rdx = (m.r32(r15.wrapping_add(0x10)) as u64) as u32 as u64;
            // 319bd shl eax,0x4
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 319c0 and eax,0xffffccf0
            { let r = (rax as u32 as u64) & 0xffffccf0u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319c5 or eax,ecx
            { let r = (rax as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319c7 shl edx,0xc
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xcu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 319ca or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 319cc mov eax,DWORD PTR [r15+0x14]
            rax = (m.r32(r15.wrapping_add(0x14)) as u64) as u32 as u64;
            // 319d0 shl eax,0x8
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 319d3 or eax,edx
            { let r = (rax as u32 as u64) | (rdx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 319d5 mov DWORD PTR [rbp-0x80],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff80), ((rax as u32 as u64)) as u32);
            // 319d8 lea r12,[r15+0x30]
            r12 = r15.wrapping_add(0x30);
            // 319dc mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 319e0 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 319e5 mov rdi,r12
            rdi = r12;
            // 319e8 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 319eb mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 319ee shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 319f1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 319f5 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 319f8 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 319fa mov rdi,r12
            rdi = r12;
            // 319fd call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a00 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31a01 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31a03 mov DWORD PTR [rbp-0x7c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff84), ((rax as u32 as u64)) as u32);
            // 31a06 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a0a mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31a0f mov rdi,r12
            rdi = r12;
            // 31a12 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a15 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31a16 mov DWORD PTR [rbp-0x78],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff88), ((rax as u32 as u64)) as u32);
            // 31a19 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a1d xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31a1f mov rdi,r12
            rdi = r12;
            // 31a22 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a25 mulsd xmm0,QWORD PTR [rip+0xce403] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a2d cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a32 mov DWORD PTR [rbp-0x74],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c), ((rax as u32 as u64)) as u32);
            // 31a35 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a39 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31a3e mov rdi,r12
            rdi = r12;
            // 31a41 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a44 mulsd xmm0,QWORD PTR [rip+0xce3e4] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a4c cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a51 mov DWORD PTR [rbp-0x70],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff90), ((rax as u32 as u64)) as u32);
            // 31a54 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a58 mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31a5d mov rdi,r12
            rdi = r12;
            // 31a60 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a63 mulsd xmm0,QWORD PTR [rip+0xce3c5] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a6b cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a70 mov DWORD PTR [rbp-0x6c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rax as u32 as u64)) as u32);
            // 31a73 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a77 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31a7c mov rdi,r12
            rdi = r12;
            // 31a7f call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31a82 mulsd xmm0,QWORD PTR [rip+0xce3a6] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31a8a cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31a8f mov DWORD PTR [rbp-0x68],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), ((rax as u32 as u64)) as u32);
            // 31a92 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31a96 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31a9b mov rdi,r12
            rdi = r12;
            // 31a9e call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31aa1 mulsd xmm0,QWORD PTR [rip+0xce387] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31aa9 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31aae mov DWORD PTR [rbp-0x64],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((rax as u32 as u64)) as u32);
            // 31ab1 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31ab5 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31aba mov rdi,r12
            rdi = r12;
            // 31abd call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31ac0 mulsd xmm0,QWORD PTR [rip+0xce368] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31ac8 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31acd mov DWORD PTR [rbp-0x60],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), ((rax as u32 as u64)) as u32);
            // 31ad0 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31ad4 mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31ad9 mov rdi,r12
            rdi = r12;
            // 31adc call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31adf mulsd xmm0,QWORD PTR [rip+0xce349] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31ae7 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31aec mov DWORD PTR [rbp-0x5c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), ((rax as u32 as u64)) as u32);
            // 31aef mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31af3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31af8 mov rdi,r12
            rdi = r12;
            // 31afb call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31afe mulsd xmm0,QWORD PTR [rip+0xce32a] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31b06 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31b0b mov DWORD PTR [rbp-0x58],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), ((rax as u32 as u64)) as u32);
            // 31b0e mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b12 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31b17 mov rdi,r12
            rdi = r12;
            // 31b1a call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b1d mulsd xmm0,QWORD PTR [rip+0xce30b] # 0xffe30
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83453)) as u128))).to_bits());
            // 31b25 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31b2a mov DWORD PTR [rbp-0x54],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffac), ((rax as u32 as u64)) as u32);
            // 31b2d mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b31 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31b36 mov rdi,r12
            rdi = r12;
            // 31b39 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b3c mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // 31b3f shl ebx,0x10
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x10u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // 31b42 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b46 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31b4b mov rdi,r12
            rdi = r12;
            // 31b4e call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b51 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31b52 or eax,ebx
            { let r = (rax as u32 as u64) | (rbx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b54 mov DWORD PTR [rbp-0x50],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), ((rax as u32 as u64)) as u32);
            // 31b57 mov rax,QWORD PTR [r15+0x30]
            rax = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31b5b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31b60 mov rdi,r12
            rdi = r12;
            // 31b63 call QWORD PTR [rax+0x28]
            { let r = { let t = m.r64(rax.wrapping_add(0x28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31b66 cwde
            rax = (rax as u16 as i16 as i32 as u32) as u64;
            // 31b67 mov DWORD PTR [rbp-0x4c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb4), ((rax as u32 as u64)) as u32);
            // 31b6a mov eax,DWORD PTR [r15+0x18]
            rax = (m.r32(r15.wrapping_add(0x18)) as u64) as u32 as u64;
            // 31b6e mov ecx,DWORD PTR [r15+0x1c]
            rcx = (m.r32(r15.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 31b72 and ecx,0x10
            { let r = (rcx as u32 as u64) & 0x10u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31b75 shl ecx,0x4
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31b78 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b7b or eax,ecx
            { let r = (rax as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31b7d mov DWORD PTR [rbp-0x48],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), ((rax as u32 as u64)) as u32);
            // 31b80 movzx eax,WORD PTR [r15+0x22]
            rax = (m.r16(r15.wrapping_add(0x22)) as u64) as u32 as u64;
            // 31b85 shl eax,0x10
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x10u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 31b88 movsx ecx,WORD PTR [r15+0x20]
            rcx = (sx::<16>(m.r16(r15.wrapping_add(0x20)) as u64) as u64) as u32 as u64;
            // 31b8d or ecx,eax
            { let r = (rcx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31b8f mov DWORD PTR [rbp-0x44],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffbc), ((rcx as u32 as u64)) as u32);
            // 31b92 movsd xmm0,QWORD PTR [r15+0x28]
            x0 = m.r64(r15.wrapping_add(0x28)) as u128;
            // 31b98 mulsd xmm0,QWORD PTR [rip+0xce298] # 0xffe38
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83461)) as u128))).to_bits());
            // 31ba0 cvttsd2si rax,xmm0
            rax = cvtt64(f64_of(x0)) as u64;
            // 31ba5 mov DWORD PTR [rbp-0x40],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), ((rax as u32 as u64)) as u32);
            // 31ba8 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31bb3 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 31bb8 mov QWORD PTR [rbp-0x88],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rax) as u64);
            // 31bbf xorps xmm0,xmm0
            x0 = 0;
            // 31bc2 movaps XMMWORD PTR [rbp-0xb0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff50), x0);
            // 31bc9 mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // 31bd4 lea r15,[rbp-0xb0]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31bdb lea r12,[rbp-0x90]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31be2 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 31be4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31be6 jmp 0x31bf6
            pc = 0x31bf6; continue;
        }
        0x31be8 => {
            // block 31be8
            // 31be8 mov rcx,QWORD PTR [rbp-0xa8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // 31bef mov r14,QWORD PTR [rbp-0xa0]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x31bf6;
        }
        0x31bf6 => {
            // block 31bf6
            // 31bf6 lea eax,[r13+rbx*1+0x0]
            rax = (r13.wrapping_add(rbx)) as u32 as u64;
            // 31bfb mov DWORD PTR [rbp-0x90],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff70), ((rax as u32 as u64)) as u32);
            // 31c01 mov eax,DWORD PTR [rbp+rbx*4-0x80]
            rax = (m.r32(rbp.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31c05 mov DWORD PTR [rbp-0x8c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff74), ((rax as u32 as u64)) as u32);
            // 31c0b mov DWORD PTR [rbp-0x88],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0xffffffffu64) as u32);
            // 31c15 mov DWORD PTR [rbp-0x84],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c), (0x0u64) as u32);
            // 31c1f cmp rcx,r14
            fl = sub::<64>(rcx, r14).1;
            // 31c22 je 0x31c43
            if fl.e() { pc = 0x31c43; continue; }
            pc = 0x31c24;
        }
        0x31c24 => {
            // block 31c24
            // 31c24 mov rax,QWORD PTR [rbp-0x90]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 31c2b mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 31c32 mov QWORD PTR [rcx+0x8],rdx
            m.w64(rcx.wrapping_add(0x8), (rdx) as u64);
            // 31c36 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 31c39 add QWORD PTR [rbp-0xa8],0x10
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x10u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (r) as u64);
              fl = f; }
            // 31c41 jmp 0x31c4e
            pc = 0x31c4e; continue;
        }
        0x31c43 => {
            // block 31c43
            // 31c43 mov rdi,r15
            rdi = r15;
            // 31c46 mov rsi,r12
            rsi = r12;
            // 31c49 call 0xebb2e
            { let r = { let t = m.env.stub_va(0x2fb); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31c4e;
        }
        0x31c4e => {
            // block 31c4e
            // 31c4e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 31c51 cmp rbx,0x10
            fl = sub::<64>(rbx, 0x10u64).1;
            // 31c55 jle 0x31be8
            if fl.le() { pc = 0x31be8; continue; }
            pc = 0x31c57;
        }
        0x31c57 => {
            // block 31c57
            // 31c57 mov rdi,QWORD PTR [rbp-0xb8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31c5e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 31c61 mov rax,QWORD PTR [rax+0xca0]
            rax = m.r64(rax.wrapping_add(0xca0)) as u64;
            // 31c68 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31c6f call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31c71 mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 31c74 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31c7b call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31c80 mov rbx,QWORD PTR [rip+0x10e751] # 0x1403d8
            rbx = m.r64(m.env.data_va(347045)) as u64;
            // 31c87 mov rbx,QWORD PTR [rbx]
            rbx = m.r64(rbx) as u64;
            // 31c8a mov r12,rbx
            r12 = rbx;
            // 31c8d jmp 0x31c92
            pc = 0x31c92; continue;
        }
        0x31c8f => {
            // block 31c8f
            // 31c8f xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x31c92;
        }
        0x31c92 => {
            // block 31c92
            // 31c92 cmp r12,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 31c96 jne 0x31cc4
            if fl.ne() { pc = 0x31cc4; continue; }
            pc = 0x31c98;
        }
        0x31c98 => {
            // block 31c98
            // 31c98 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 31c9b add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ca2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 31ca3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 31ca5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 31ca7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 31ca9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 31cab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31cac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x31cad => {
            // block 31cad
            // 31cad mov rbx,rax
            rbx = rax;
            // 31cb0 lea rdi,[rbp-0xb0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31cb7 call 0xebb7c
            { let r = { let t = m.env.stub_va(0x349); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31cbc mov rdi,rbx
            rdi = rbx;
            // 31cbf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31cc4;
        }
        0x31cc4 => {
            // block 31cc4
            // 31cc4 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31cc9 jmp 0x31cad
            pc = 0x31cad; continue;
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
            { let r = { let t = m.r64(rax.wrapping_add(0x418)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2178 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 217a je 0x21c6
            if fl.e() { pc = 0x21c6; continue; }
            pc = 0x217c;
        }
        0x217c => {
            // block 217c
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
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2199 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 219b je 0x2244
            if fl.e() { pc = 0x2244; continue; }
            pc = 0x21a1;
        }
        0x21a1 => {
            // block 21a1
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
            if fl.e() { pc = 0x21ca; continue; }
            pc = 0x21b3;
        }
        0x21b3 => {
            // block 21b3
            // 21b3 cmp eax,0x200000
            fl = sub::<32>((rax as u32 as u64), 0x200000u64).1;
            // 21b8 jne 0x21d3
            if fl.ne() { pc = 0x21d3; continue; }
            pc = 0x21ba;
        }
        0x21ba => {
            // block 21ba
            // 21ba mov eax,0x800000
            rax = (0x800000u64) as u32 as u64;
            // 21bf mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 21c4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21c6 => {
            // block 21c6
            // 21c6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 21c8 jmp 0x2244
            pc = 0x2244; continue;
        }
        0x21ca => {
            // block 21ca
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
            // 21d3 cmp eax,0x100000
            fl = sub::<32>((rax as u32 as u64), 0x100000u64).1;
            // 21d8 jne 0x21e6
            if fl.ne() { pc = 0x21e6; continue; }
            pc = 0x21da;
        }
        0x21da => {
            // block 21da
            // 21da mov eax,0x400000
            rax = (0x400000u64) as u32 as u64;
            // 21df mov ebx,0x2
            rbx = (0x2u64) as u32 as u64;
            // 21e4 jmp 0x21f0
            pc = 0x21f0; continue;
        }
        0x21e6 => {
            // block 21e6
            // 21e6 mov eax,0x200000
            rax = (0x200000u64) as u32 as u64;
            // 21eb mov ebx,0x4
            rbx = (0x4u64) as u32 as u64;
            pc = 0x21f0;
        }
        0x21f0 => {
            // block 21f0
            // 21f0 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 21f3 je 0x21f8
            if fl.e() { pc = 0x21f8; continue; }
            pc = 0x21f5;
        }
        0x21f5 => {
            // block 21f5
            // 21f5 mov DWORD PTR [r15],eax
            m.w32(r15, ((rax as u32 as u64)) as u32);
            pc = 0x21f8;
        }
        0x21f8 => {
            // block 21f8
            // 21f8 test r14,r14
            fl = logic::<64>(r14 & r14);
            // 21fb je 0x2215
            if fl.e() { pc = 0x2215; continue; }
            pc = 0x21fd;
        }
        0x21fd => {
            // block 21fd
            // 21fd mov edi,DWORD PTR [r12+0x10]
            rdi = (m.r32(r12.wrapping_add(0x10)) as u64) as u32 as u64;
            // 2202 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2207 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // 2209 je 0x2210
            if fl.e() { pc = 0x2210; continue; }
            pc = 0x220b;
        }
        0x220b => {
            // block 220b
            // 220b imul eax,ebx
            { let (r, f) = imul::<32>((rax as u32 as u64), (rbx as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // 220e jmp 0x2212
            pc = 0x2212; continue;
        }
        0x2210 => {
            // block 2210
            // 2210 shr eax,1
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0x2212;
        }
        0x2212 => {
            // block 2212
            // 2212 mov DWORD PTR [r14],eax
            m.w32(r14, ((rax as u32 as u64)) as u32);
            pc = 0x2215;
        }
        0x2215 => {
            // block 2215
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
            { let r = { let t = m.r64(rax.wrapping_add(0x388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x2244; continue; }
            pc = 0x223c;
        }
        0x223c => {
            // block 223c
            // 223c shl DWORD PTR [r15],0x2
            { let (r, f) = shl::<32>(m.r32(r15) as u64, 0x2u64, fl); m.w32(r15, (r) as u32); fl = f; }
            // 2240 shr DWORD PTR [r14],0x2
            { let (r, f) = shr::<32>(m.r32(r14) as u64, 0x2u64, fl); m.w32(r14, (r) as u32); fl = f; }
            pc = 0x2244;
        }
        0x2244 => {
            // block 2244
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

/// parity: P2v @0xd3b22 size=690
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d3b22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd3b22;
    loop {
        match pc {
        0xd3b22 => {
            // block d3b22
            // d3b22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d3b23 mov rbp,rsp
            rbp = rsp;
            // d3b26 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d3b28 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d3b2a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d3b2c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d3b2e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d3b2f sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3b36 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d3b39 mov rbx,rdi
            rbx = rdi;
            // d3b3c lea rax,[rbp-0xa0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3b43 mov QWORD PTR [rbp-0x98],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u64);
            // d3b4e mov QWORD PTR [rbp-0xa0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u64);
            // d3b59 mov QWORD PTR [rbp-0xa8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (rax) as u64);
            // d3b60 mov DWORD PTR [rbp-0xac],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff54), (0x0u64) as u32);
            // d3b6a mov DWORD PTR [rbp-0xb0],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u32);
            // d3b74 mov DWORD PTR [rcx],0x0
            m.w32(rcx, (0x0u64) as u32);
            // d3b7a mov DWORD PTR [rdx],0x0
            m.w32(rdx, (0x0u64) as u32);
            // d3b80 test r14b,r14b
            fl = logic::<8>((r14 as u8 as u64) & (r14 as u8 as u64));
            // d3b83 je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b89;
        }
        0xd3b89 => {
            // block d3b89
            // d3b89 mov al,BYTE PTR [rbx+0xa]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xa)) as u64) as u8 as u64);
            // d3b8c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3b8e je 0xd3d7f
            if fl.e() { pc = 0xd3d7f; continue; }
            pc = 0xd3b94;
        }
        0xd3b94 => {
            // block d3b94
            // d3b94 mov QWORD PTR [rbp-0xc8],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff38), (rdx) as u64);
            // d3b9b mov QWORD PTR [rbp-0xc0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (rcx) as u64);
            // d3ba2 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3ba5 mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bac lea rsi,[rbp-0xac]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff54);
            // d3bb3 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d3bb5 mov rdi,rbx
            rdi = rbx;
            // d3bb8 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3bbd mov rax,QWORD PTR [rax+0x470]
            rax = m.r64(rax.wrapping_add(0x470)) as u64;
            // d3bc4 lea rsi,[rbp-0xb0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // d3bcb mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // d3bd0 mov rdi,rbx
            rdi = rbx;
            // d3bd3 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3bd5 mov DWORD PTR [rbp-0xb8],r14d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), ((r14 as u32 as u64)) as u32);
            // d3bdc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d3bdf lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            pc = 0xd3be6;
        }
        0xd3be6 => {
            // block d3be6
            // d3be6 lea rax,[rip+0x2f313] # 0x102f00
            rax = m.env.data_va(95949);
            // d3bed mov esi,DWORD PTR [rax+r14*4]
            rsi = (m.r32(rax.wrapping_add(r14.wrapping_mul(4))) as u64) as u32 as u64;
            // d3bf1 xorps xmm0,xmm0
            x0 = 0;
            // d3bf4 movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // d3bf8 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // d3bfc movaps XMMWORD PTR [rbp-0x70],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff90), x0);
            // d3c00 movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // d3c04 movaps XMMWORD PTR [rbp-0x90],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff70), x0);
            // d3c0b mov DWORD PTR [rbp-0x38],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u32);
            // d3c12 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // d3c1a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d3c1d mov rax,QWORD PTR [rax+0xc18]
            rax = m.r64(rax.wrapping_add(0xc18)) as u64;
            // d3c24 mov rdi,rbx
            rdi = rbx;
            // d3c27 lea rdx,[rbp-0x90]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // d3c2e call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c30 cmp DWORD PTR [rbp-0x8c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff74)) as u64, 0x0u64).1;
            // d3c37 je 0xd3cb2
            if fl.e() { pc = 0xd3cb2; continue; }
            pc = 0xd3c39;
        }
        0xd3c39 => {
            // block d3c39
            // d3c39 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // d3c3b jne 0xd3cb2
            if fl.ne() { pc = 0xd3cb2; continue; }
            pc = 0xd3c3d;
        }
        0xd3c3d => {
            // block d3c3d
            // d3c3d mov r13d,DWORD PTR [rbp-0x84]
            r13 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff7c)) as u64) as u32 as u64;
            // d3c44 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // d3c47 and eax,0xfffffff7
            { let r = (rax as u32 as u64) & 0xfffffff7u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d3c4a jne 0xd3c5f
            if fl.ne() { pc = 0xd3c5f; continue; }
            pc = 0xd3c4c;
        }
        0xd3c4c => {
            // block d3c4c
            // d3c4c mov ecx,DWORD PTR [rbp-0xac]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff54)) as u64) as u32 as u64;
            // d3c52 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // d3c54 je 0xd3c5f
            if fl.e() { pc = 0xd3c5f; continue; }
            pc = 0xd3c56;
        }
        0xd3c56 => {
            // block d3c56
            // d3c56 mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c5d jmp 0xd3c75
            pc = 0xd3c75; continue;
        }
        0xd3c5f => {
            // block d3c5f
            // d3c5f mov r12d,DWORD PTR [rbp-0x88]
            r12 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64) as u32 as u64;
            // d3c66 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // d3c69 jne 0xd3c7d
            if fl.ne() { pc = 0xd3c7d; continue; }
            pc = 0xd3c6b;
        }
        0xd3c6b => {
            // block d3c6b
            // d3c6b mov eax,DWORD PTR [rbp-0xb0]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64) as u32 as u64;
            // d3c71 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // d3c73 je 0xd3c7d
            if fl.e() { pc = 0xd3c7d; continue; }
            pc = 0xd3c75;
        }
        0xd3c75 => {
            // block d3c75
            // d3c75 sub r13d,r12d
            { let (r, f) = sub::<32>((r13 as u32 as u64), (r12 as u32 as u64));
              r13 = (r) as u32 as u64;
              fl = f; }
            // d3c78 lea r13d,[r12+r13*4+0x3]
            r13 = (r12.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x3)) as u32 as u64;
            pc = 0xd3c7d;
        }
        0xd3c7d => {
            // block d3c7d
            // d3c7d mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3c84 cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3c87 jg 0xd3cb2
            if fl.g() { pc = 0xd3cb2; continue; }
            pc = 0xd3c89;
        }
        0xd3c89 => {
            // block d3c89
            // d3c89 inc r12d
            { let (r, f) = inc::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            pc = 0xd3c8c;
        }
        0xd3c8c => {
            // block d3c8c
            // d3c8c mov rdi,r15
            rdi = r15;
            // d3c8f lea rsi,[rbp-0xb4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff4c);
            // d3c96 call 0xebea0
            { let r = { let t = m.env.stub_va(0x66d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3c9b mov DWORD PTR [rbp-0xb4],r12d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff4c), ((r12 as u32 as u64)) as u32);
            // d3ca2 lea eax,[r12+0x1]
            rax = (r12.wrapping_add(0x1)) as u32 as u64;
            // d3ca7 dec r12d
            { let (r, f) = dec::<32>((r12 as u32 as u64), fl); r12 = (r) as u32 as u64; fl = f; }
            // d3caa cmp r12d,r13d
            fl = sub::<32>((r12 as u32 as u64), (r13 as u32 as u64)).1;
            // d3cad mov r12d,eax
            r12 = ((rax as u32 as u64)) as u32 as u64;
            // d3cb0 jl 0xd3c8c
            if fl.l() { pc = 0xd3c8c; continue; }
            pc = 0xd3cb2;
        }
        0xd3cb2 => {
            // block d3cb2
            // d3cb2 inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // d3cb5 cmp r14,0x10
            fl = sub::<64>(r14, 0x10u64).1;
            // d3cb9 jb 0xd3be6
            if fl.b() { pc = 0xd3be6; continue; }
            pc = 0xd3cbf;
        }
        0xd3cbf => {
            // block d3cbf
            // d3cbf mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d3cc2 call 0xd19e0
            { let r = { let t = m.env.code_va(0xd19e0); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3cc7 mov ecx,DWORD PTR [rbp-0xb8]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // d3ccd dec eax
            { let (r, f) = dec::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d3ccf movzx r8d,cl
            r8 = ((rcx as u8 as u64)) as u32 as u64;
            // d3cd3 mov rdi,QWORD PTR [rbp-0xa8]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64;
            // d3cda lea r9d,[r8-0x1]
            r9 = (r8.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d3cde xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3ce1 lea rdx,[rbp-0xa0]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // d3ce8 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3ceb je 0xd3d54
            if fl.e() { pc = 0xd3d54; continue; }
            pc = 0xd3ced;
        }
        0xd3ced => {
            // block d3ced
            // d3ced xor r10d,r10d
            { let r = (r10 as u32 as u64) ^ (r10 as u32 as u64); let f = logic::<32>(r);
              r10 = (r) as u32 as u64;
              fl = f; }
            // d3cf0 mov r11d,r8d
            r11 = ((r8 as u32 as u64)) as u32 as u64;
            pc = 0xd3cf3;
        }
        0xd3cf3 => {
            // block d3cf3
            // d3cf3 mov r14d,DWORD PTR [rdi+0x1c]
            r14 = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3cf7 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0xd3cfa;
        }
        0xd3cfa => {
            // block d3cfa
            // d3cfa mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d3cfc mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // d3d00 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d03 je 0xd3d12
            if fl.e() { pc = 0xd3d12; continue; }
            pc = 0xd3d05;
        }
        0xd3d05 => {
            // block d3d05
            // d3d05 mov rdi,rcx
            rdi = rcx;
            // d3d08 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // d3d0b test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // d3d0e jne 0xd3d05
            if fl.ne() { pc = 0xd3d05; continue; }
            pc = 0xd3d10;
        }
        0xd3d10 => {
            // block d3d10
            // d3d10 jmp 0xd3d21
            pc = 0xd3d21; continue;
        }
        0xd3d12 => {
            // block d3d12
            // d3d12 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d15;
        }
        0xd3d15 => {
            // block d3d15
            // d3d15 mov rdi,QWORD PTR [rcx+0x10]
            rdi = m.r64(rcx.wrapping_add(0x10)) as u64;
            // d3d19 cmp QWORD PTR [rdi],rcx
            fl = sub::<64>(m.r64(rdi) as u64, rcx).1;
            // d3d1c mov rcx,rdi
            rcx = rdi;
            // d3d1f jne 0xd3d15
            if fl.ne() { pc = 0xd3d15; continue; }
            pc = 0xd3d21;
        }
        0xd3d21 => {
            // block d3d21
            // d3d21 cmp rdi,rdx
            fl = sub::<64>(rdi, rdx).1;
            // d3d24 mov rcx,rdx
            rcx = rdx;
            // d3d27 je 0xd3d36
            if fl.e() { pc = 0xd3d36; continue; }
            pc = 0xd3d29;
        }
        0xd3d29 => {
            // block d3d29
            // d3d29 mov ecx,DWORD PTR [rdi+0x1c]
            rcx = (m.r32(rdi.wrapping_add(0x1c)) as u64) as u32 as u64;
            // d3d2c lea ebx,[rsi+0x1]
            rbx = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d2f cmp ecx,ebx
            fl = sub::<32>((rcx as u32 as u64), (rbx as u32 as u64)).1;
            // d3d31 je 0xd3cfa
            if fl.e() { pc = 0xd3cfa; continue; }
            pc = 0xd3d33;
        }
        0xd3d33 => {
            // block d3d33
            // d3d33 mov rcx,rdi
            rcx = rdi;
            pc = 0xd3d36;
        }
        0xd3d36 => {
            // block d3d36
            // d3d36 cmp r10d,r14d
            fl = sub::<32>((r10 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d39 jge 0xd3d40
            if fl.ge() { pc = 0xd3d40; continue; }
            pc = 0xd3d3b;
        }
        0xd3d3b => {
            // block d3d3b
            // d3d3b cmp r11d,r14d
            fl = sub::<32>((r11 as u32 as u64), (r14 as u32 as u64)).1;
            // d3d3e jle 0xd3d54
            if fl.le() { pc = 0xd3d54; continue; }
            pc = 0xd3d40;
        }
        0xd3d40 => {
            // block d3d40
            // d3d40 lea r10d,[rsi+0x1]
            r10 = (rsi.wrapping_add(0x1)) as u32 as u64;
            // d3d44 lea r11d,[rsi+r8*1+0x1]
            r11 = (rsi.wrapping_add(r8).wrapping_add(0x1)) as u32 as u64;
            // d3d49 add esi,r8d
            { let (r, f) = add::<32>((rsi as u32 as u64), (r8 as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // d3d4c cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // d3d4f mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // d3d52 jne 0xd3cf3
            if fl.ne() { pc = 0xd3cf3; continue; }
            pc = 0xd3d54;
        }
        0xd3d54 => {
            // block d3d54
            // d3d54 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d5b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d3d5d cmp r10d,eax
            fl = sub::<32>((r10 as u32 as u64), (rax as u32 as u64)).1;
            // d3d60 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d62;
        }
        0xd3d62 => {
            // block d3d62
            // d3d62 cmp r9d,eax
            fl = sub::<32>((r9 as u32 as u64), (rax as u32 as u64)).1;
            // d3d65 jg 0xd3d88
            if fl.g() { pc = 0xd3d88; continue; }
            pc = 0xd3d67;
        }
        0xd3d67 => {
            // block d3d67
            // d3d67 mov rax,QWORD PTR [rbp-0xc8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff38)) as u64;
            // d3d6e mov DWORD PTR [rax],r10d
            m.w32(rax, ((r10 as u32 as u64)) as u32);
            // d3d71 mov rax,QWORD PTR [rbp-0xc0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // d3d78 mov DWORD PTR [rax],r9d
            m.w32(rax, ((r9 as u32 as u64)) as u32);
            // d3d7b mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            // d3d7d jmp 0xd3d88
            pc = 0xd3d88; continue;
        }
        0xd3d7f => {
            // block d3d7f
            // d3d7f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3d86 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3d88;
        }
        0xd3d88 => {
            // block d3d88
            // d3d88 mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3d8f call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3d94 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // d3d96 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // d3d9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d3d9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d3da0 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d3da2 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d3da4 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d3da6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d3da7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xd3da8 => {
            // block d3da8
            // d3da8 mov r14,rax
            r14 = rax;
            pc = 0xd3dab;
        }
        0xd3dab => {
            // block d3dab
            // d3dab mov rsi,QWORD PTR [rbp-0xa0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            // d3db2 mov rdi,r15
            rdi = r15;
            // d3db5 call 0xebeac
            { let r = { let t = m.env.stub_va(0x679); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dba mov rdi,r14
            rdi = r14;
            // d3dbd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3dc2 mov r14,rax
            r14 = rax;
            // d3dc5 jmp 0xd3dab
            pc = 0xd3dab; continue;
        }
        0xd3dc7 => {
            // block d3dc7
            // d3dc7 mov r14,rax
            r14 = rax;
            // d3dca lea r15,[rbp-0xa8]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // d3dd1 jmp 0xd3dab
            pc = 0xd3dab; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x31ccc size=1046
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00031ccc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut fl = Fl::default();
    let mut pc: u32 = 0x31ccc;
    loop {
        match pc {
        0x31ccc => {
            // block 31ccc
            // 31ccc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31ccd mov rbp,rsp
            rbp = rsp;
            // 31cd0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 31cd2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 31cd4 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 31cd6 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 31cd8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 31cd9 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 31ce0 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 31ce3 mov r15,rsi
            r15 = rsi;
            // 31ce6 mov r13,rdi
            r13 = rdi;
            // 31ce9 mov r14,QWORD PTR [rip+0x10e6e8] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31cf0 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31cf3 mov QWORD PTR [rbp-0x30],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (r14) as u64);
            // 31cf7 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31cfa mov rdi,r15
            rdi = r15;
            // 31cfd call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d03 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31d05 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31d07 je 0x3208c
            if fl.e() { pc = 0x3208c; continue; }
            pc = 0x31d0d;
        }
        0x31d0d => {
            // block 31d0d
            // 31d0d mov QWORD PTR [rbp-0xc0],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (r13) as u64);
            // 31d14 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 31d17 lea rcx,[rip+0xce142] # 0xffe60
            rcx = m.env.data_va(83501);
            // 31d1e mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 31d22 lea rax,[rbp-0x90]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 31d29 mov QWORD PTR [rbp-0x88],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0x0u64) as u64);
            // 31d34 mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 31d3f mov QWORD PTR [rbp-0x98],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (rax) as u64);
            // 31d46 lea r12,[rbp-0x98]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31d4d lea r13,[rbp-0x9c]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_ff64);
            pc = 0x31d54;
        }
        0x31d54 => {
            // block 31d54
            // 31d54 lea eax,[r14+rbx*1]
            rax = (r14.wrapping_add(rbx)) as u32 as u64;
            // 31d58 mov DWORD PTR [rbp-0x9c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), ((rax as u32 as u64)) as u32);
            // 31d5e mov rdi,r12
            rdi = r12;
            // 31d61 mov rsi,r13
            rsi = r13;
            // 31d64 call 0xebb22
            { let r = { let t = m.env.stub_va(0x2ef); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31d69 inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 31d6b cmp ebx,0x11
            fl = sub::<32>((rbx as u32 as u64), 0x11u64).1;
            // 31d6e jl 0x31d54
            if fl.l() { pc = 0x31d54; continue; }
            pc = 0x31d70;
        }
        0x31d70 => {
            // block 31d70
            // 31d70 lea rbx,[rbp-0xb0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff50);
            // 31d77 mov QWORD PTR [rbp-0xa8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff58), (0x0u64) as u64);
            // 31d82 mov QWORD PTR [rbp-0xb0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff50), (0x0u64) as u64);
            // 31d8d mov QWORD PTR [rbp-0xb8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (rbx) as u64);
            // 31d94 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 31d97 mov rax,QWORD PTR [rax+0xc90]
            rax = m.r64(rax.wrapping_add(0xc90)) as u64;
            // 31d9e lea rsi,[rbp-0x98]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 31da5 lea rdx,[rbp-0xb8]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 31dac mov rdi,r15
            rdi = r15;
            // 31daf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31db1 mov r14,QWORD PTR [rip+0x10e620] # 0x1403d8
            r14 = m.r64(m.env.data_va(347045)) as u64;
            // 31db8 mov r14,QWORD PTR [r14]
            r14 = m.r64(r14) as u64;
            // 31dbb mov r15,QWORD PTR [rbp-0xc0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64;
            // 31dc2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 31dc4 je 0x31e4f
            if fl.e() { pc = 0x31e4f; continue; }
            pc = 0x31dca;
        }
        0x31dca => {
            // block 31dca
            // 31dca cmp QWORD PTR [rbp-0xa8],0x11
            fl = sub::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64, 0x11u64).1;
            // 31dd2 jne 0x31e4f
            if fl.ne() { pc = 0x31e4f; continue; }
            pc = 0x31dd4;
        }
        0x31dd4 => {
            // block 31dd4
            // 31dd4 mov rcx,QWORD PTR [rbp-0xb8]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            // 31ddb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 31ddd cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31de0 je 0x31e56
            if fl.e() { pc = 0x31e56; continue; }
            pc = 0x31de2;
        }
        0x31de2 => {
            // block 31de2
            // 31de2 mov edx,DWORD PTR [rcx+0x20]
            rdx = (m.r32(rcx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 31de5 mov DWORD PTR [rbp+rax*4-0x80],edx
            m.w32(rbp.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0xffff_ffff_ffff_ff80), ((rdx as u32 as u64)) as u32);
            // 31de9 mov rdx,QWORD PTR [rcx+0x8]
            rdx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 31ded test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31df0 je 0x31dff
            if fl.e() { pc = 0x31dff; continue; }
            pc = 0x31df2;
        }
        0x31df2 => {
            // block 31df2
            // 31df2 mov rcx,rdx
            rcx = rdx;
            // 31df5 mov rdx,QWORD PTR [rcx]
            rdx = m.r64(rcx) as u64;
            // 31df8 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 31dfb jne 0x31df2
            if fl.ne() { pc = 0x31df2; continue; }
            pc = 0x31dfd;
        }
        0x31dfd => {
            // block 31dfd
            // 31dfd jmp 0x31e0e
            pc = 0x31e0e; continue;
        }
        0x31dff => {
            // block 31dff
            // 31dff mov rdx,rcx
            rdx = rcx;
            pc = 0x31e02;
        }
        0x31e02 => {
            // block 31e02
            // 31e02 mov rcx,QWORD PTR [rdx+0x10]
            rcx = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 31e06 cmp QWORD PTR [rcx],rdx
            fl = sub::<64>(m.r64(rcx) as u64, rdx).1;
            // 31e09 mov rdx,rcx
            rdx = rcx;
            // 31e0c jne 0x31e02
            if fl.ne() { pc = 0x31e02; continue; }
            pc = 0x31e0e;
        }
        0x31e0e => {
            // block 31e0e
            // 31e0e inc rax
            { let (r, f) = inc::<64>(rax, fl); rax = r; fl = f; }
            // 31e11 cmp rcx,rbx
            fl = sub::<64>(rcx, rbx).1;
            // 31e14 jne 0x31de2
            if fl.ne() { pc = 0x31de2; continue; }
            pc = 0x31e16;
        }
        0x31e16 => {
            // block 31e16
            // 31e16 mov ecx,DWORD PTR [rbp-0x80]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64) as u32 as u64;
            // 31e19 mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            // 31e1c mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 31e1e shr edx,0x4
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x4u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 31e21 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 31e23 shr esi,0xc
            { let (r, f) = shr::<32>((rsi as u32 as u64), 0xcu64, fl); rsi = (r) as u32 as u64; fl = f; }
            // 31e26 movd xmm0,ecx
            x0 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e2a shr ecx,0x8
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x8u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 31e2d movd xmm1,ecx
            x1 = ((rcx as u32 as u64)) as u32 as u128;
            // 31e31 movd xmm2,edx
            x2 = ((rdx as u32 as u64)) as u32 as u128;
            // 31e35 punpckldq xmm2,xmm1
            { let a = x2; let b = x1; x2 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e39 movd xmm1,esi
            x1 = ((rsi as u32 as u64)) as u32 as u128;
            // 31e3d punpckldq xmm0,xmm1
            { let a = x0; let b = x1; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e41 punpckldq xmm0,xmm2
            { let a = x0; let b = x2; x0 = (a & 0xffff_ffff) | ((b & 0xffff_ffff) << 32) | (((a >> 32) & 0xffff_ffff) << 64) | (((b >> 32) & 0xffff_ffff) << 96); }
            // 31e45 pand xmm0,XMMWORD PTR [rip+0xce003] # 0xffe50
            x0 &= m.r128(m.env.data_va(83485));
            // 31e4d jmp 0x31e5d
            pc = 0x31e5d; continue;
        }
        0x31e4f => {
            // block 31e4f
            // 31e4f xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 31e51 jmp 0x32066
            pc = 0x32066; continue;
        }
        0x31e56 => {
            // block 31e56
            // 31e56 pxor xmm0,xmm0
            x0 = 0;
            // 31e5a mov eax,DWORD PTR [rbp-0x7c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff84)) as u64) as u32 as u64;
            pc = 0x31e5d;
        }
        0x31e5d => {
            // block 31e5d
            // 31e5d movdqu XMMWORD PTR [r15+0x8],xmm0
            m.w128(r15.wrapping_add(0x8), x0);
            // 31e63 lea rbx,[r15+0x30]
            rbx = r15.wrapping_add(0x30);
            // 31e67 mov rcx,QWORD PTR [r15+0x30]
            rcx = m.r64(r15.wrapping_add(0x30)) as u64;
            // 31e6b mov rcx,QWORD PTR [rcx+0x30]
            rcx = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 31e6f movsx edx,ax
            rdx = (sx::<16>((rax as u16 as u64)) as u64) as u32 as u64;
            // 31e72 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31e74 mov rdi,rbx
            rdi = rbx;
            // 31e77 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e79 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e7c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e80 movsx edx,WORD PTR [rbp-0x7a]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff86)) as u64) as u64) as u32 as u64;
            // 31e84 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e89 mov rdi,rbx
            rdi = rbx;
            // 31e8c call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31e8e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31e91 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31e95 movsx edx,WORD PTR [rbp-0x78]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u64) as u32 as u64;
            // 31e99 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31e9e mov rdi,rbx
            rdi = rbx;
            // 31ea1 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ea3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ea6 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eaa mov ecx,DWORD PTR [rbp-0x74]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff8c)) as u64) as u32 as u64;
            // 31ead cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31eb2 mulsd xmm0,QWORD PTR [rip+0xcdf86] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eba xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 31ebc mov rdi,rbx
            rdi = rbx;
            // 31ebf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ec1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ec4 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31ec8 mov ecx,DWORD PTR [rbp-0x70]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64) as u32 as u64;
            // 31ecb xorps xmm0,xmm0
            x0 = 0;
            // 31ece cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ed3 mulsd xmm0,QWORD PTR [rip+0xcdf65] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31edb mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 31ee0 mov rdi,rbx
            rdi = rbx;
            // 31ee3 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ee5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ee8 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31eec mov ecx,DWORD PTR [rbp-0x6c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 31eef xorps xmm0,xmm0
            x0 = 0;
            // 31ef2 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31ef7 mulsd xmm0,QWORD PTR [rip+0xcdf41] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31eff mov esi,0x2
            rsi = (0x2u64) as u32 as u64;
            // 31f04 mov rdi,rbx
            rdi = rbx;
            // 31f07 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f09 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f0c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f10 mov ecx,DWORD PTR [rbp-0x68]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64) as u32 as u64;
            // 31f13 xorps xmm0,xmm0
            x0 = 0;
            // 31f16 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f1b mulsd xmm0,QWORD PTR [rip+0xcdf1d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f23 mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31f28 mov rdi,rbx
            rdi = rbx;
            // 31f2b call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f30 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f34 mov ecx,DWORD PTR [rbp-0x64]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c)) as u64) as u32 as u64;
            // 31f37 xorps xmm0,xmm0
            x0 = 0;
            // 31f3a cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f3f mulsd xmm0,QWORD PTR [rip+0xcdef9] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f47 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 31f4c mov rdi,rbx
            rdi = rbx;
            // 31f4f call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f51 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f54 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f58 mov ecx,DWORD PTR [rbp-0x60]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 31f5b xorps xmm0,xmm0
            x0 = 0;
            // 31f5e cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f63 mulsd xmm0,QWORD PTR [rip+0xcded5] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f6b mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 31f70 mov rdi,rbx
            rdi = rbx;
            // 31f73 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f75 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f78 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31f7c mov ecx,DWORD PTR [rbp-0x5c]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4)) as u64) as u32 as u64;
            // 31f7f xorps xmm0,xmm0
            x0 = 0;
            // 31f82 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31f87 mulsd xmm0,QWORD PTR [rip+0xcdeb1] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31f8f mov esi,0x6
            rsi = (0x6u64) as u32 as u64;
            // 31f94 mov rdi,rbx
            rdi = rbx;
            // 31f97 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31f99 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31f9c mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fa0 mov ecx,DWORD PTR [rbp-0x58]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64) as u32 as u64;
            // 31fa3 xorps xmm0,xmm0
            x0 = 0;
            // 31fa6 cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fab mulsd xmm0,QWORD PTR [rip+0xcde8d] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fb3 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 31fb8 mov rdi,rbx
            rdi = rbx;
            // 31fbb call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fbd mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fc0 mov rax,QWORD PTR [rax+0x20]
            rax = m.r64(rax.wrapping_add(0x20)) as u64;
            // 31fc4 mov ecx,DWORD PTR [rbp-0x54]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffac)) as u64) as u32 as u64;
            // 31fc7 xorps xmm0,xmm0
            x0 = 0;
            // 31fca cvtsi2sd xmm0,rcx
            x0 = set_lo64(x0, (sx::<64>(rcx) as f64).to_bits());
            // 31fcf mulsd xmm0,QWORD PTR [rip+0xcde69] # 0xffe40
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83469)) as u128))).to_bits());
            // 31fd7 mov esi,0x8
            rsi = (0x8u64) as u32 as u64;
            // 31fdc mov rdi,rbx
            rdi = rbx;
            // 31fdf call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31fe1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31fe4 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31fe8 movsx edx,WORD PTR [rbp-0x50]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64) as u64) as u32 as u64;
            // 31fec mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // 31ff1 mov rdi,rbx
            rdi = rbx;
            // 31ff4 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 31ff6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 31ff9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 31ffd movsx edx,WORD PTR [rbp-0x4e]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb2)) as u64) as u64) as u32 as u64;
            // 32001 mov esi,0x4
            rsi = (0x4u64) as u32 as u64;
            // 32006 mov rdi,rbx
            rdi = rbx;
            // 32009 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 3200b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 3200e mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 32012 movsx edx,WORD PTR [rbp-0x4c]
            rdx = (sx::<16>(m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ffb4)) as u64) as u64) as u32 as u64;
            // 32016 mov esi,0x5
            rsi = (0x5u64) as u32 as u64;
            // 3201b mov rdi,rbx
            rdi = rbx;
            // 3201e call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32020 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 32024 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32026 shr ecx,0x4
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x4u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // 32029 and ecx,0x1
            { let r = (rcx as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 3202c mov DWORD PTR [r15+0x1c],ecx
            m.w32(r15.wrapping_add(0x1c), ((rcx as u32 as u64)) as u32);
            // 32030 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 32032 and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // 32035 mov DWORD PTR [r15+0x18],ecx
            m.w32(r15.wrapping_add(0x18), ((rcx as u32 as u64)) as u32);
            // 32039 mov rcx,rax
            rcx = rax;
            // 3203c shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // 32040 mov WORD PTR [r15+0x20],cx
            m.w16(r15.wrapping_add(0x20), ((rcx as u16 as u64)) as u16);
            // 32045 shr rax,0x30
            { let (r, f) = shr::<64>(rax, 0x30u64, fl); rax = r; fl = f; }
            // 32049 mov WORD PTR [r15+0x22],ax
            m.w16(r15.wrapping_add(0x22), ((rax as u16 as u64)) as u16);
            // 3204e mov eax,DWORD PTR [rbp-0x40]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // 32051 cvtsi2sd xmm0,rax
            x0 = set_lo64(x0, (sx::<64>(rax) as f64).to_bits());
            // 32056 mulsd xmm0,QWORD PTR [rip+0xcddea] # 0xffe48
            x0 = set_lo64(x0, (f64_of(x0) * f64_of((m.r64(m.env.data_va(83477)) as u128))).to_bits());
            // 3205e movsd QWORD PTR [r15+0x28],xmm0
            m.w64(r15.wrapping_add(0x28), x0 as u64);
            // 32064 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x32066;
        }
        0x32066 => {
            // block 32066
            // 32066 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 3206d lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 32074 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 32079 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 32080 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 32087 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            pc = 0x3208c;
        }
        0x3208c => {
            // block 3208c
            // 3208c cmp r14,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r14, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 32090 jne 0x320dc
            if fl.ne() { pc = 0x320dc; continue; }
            pc = 0x32092;
        }
        0x32092 => {
            // block 32092
            // 32092 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // 32094 add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // 3209b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3209c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 3209e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 320a0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 320a2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 320a4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 320a5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x320a6 => {
            // block 320a6
            // 320a6 mov rbx,rax
            rbx = rax;
            pc = 0x320a9;
        }
        0x320a9 => {
            // block 320a9
            // 320a9 mov rsi,QWORD PTR [rbp-0x90]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 320b0 lea rdi,[rbp-0x98]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff68);
            // 320b7 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320bc mov rdi,rbx
            rdi = rbx;
            // 320bf call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320c4 mov rbx,rax
            rbx = rax;
            // 320c7 mov rsi,QWORD PTR [rbp-0xb0]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            // 320ce lea rdi,[rbp-0xb8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff48);
            // 320d5 call 0xeb894
            { let r = { let t = m.env.stub_va(0x61); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            // 320da jmp 0x320a9
            pc = 0x320a9; continue;
        }
        0x320dc => {
            // block 320dc
            // 320dc call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; x2 = POISON as u128; }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10690 size=1138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10690;
    loop {
        match pc {
        0x10690 => {
            // block 10690
            // 10690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10691 mov rbp,rsp
            rbp = rsp;
            // 10694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10698 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1069a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1069c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1069d sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 106a1 mov r15,rdi
            r15 = rdi;
            // 106a4 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 106a8 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 106ae jb 0x10752
            if fl.b() { pc = 0x10752; continue; }
            pc = 0x106b4;
        }
        0x106b4 => {
            // block 106b4
            // 106b4 add rax,0xfffffffffffffc00
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fc00u64);
              rax = r;
              fl = f; }
            // 106ba mov QWORD PTR [r15+0x20],rax
            m.w64(r15.wrapping_add(0x20), (rax) as u64);
            // 106be mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 106c2 mov r12,QWORD PTR [rbx]
            r12 = m.r64(rbx) as u64;
            // 106c5 lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 106c9 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 106cd mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 106d1 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 106d5 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 106d8 jne 0x109e6
            if fl.ne() { pc = 0x109e6; continue; }
            pc = 0x106de;
        }
        0x106de => {
            // block 106de
            // 106de mov QWORD PTR [rbp-0x70],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r12) as u64);
            // 106e2 mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 106e5 mov rax,r13
            rax = r13;
            // 106e8 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 106eb jbe 0x10859
            if fl.be() { pc = 0x10859; continue; }
            pc = 0x106f1;
        }
        0x106f1 => {
            // block 106f1
            // 106f1 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 106f5 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 106f9 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 106fd lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10702 sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 10705 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10709 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 10711 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 10715 mov rdi,rbx
            rdi = rbx;
            // 10718 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 1071b add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 1071f sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 10722 mov r12,r14
            r12 = r14;
            // 10725 shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10729 mov rsi,r13
            rsi = r13;
            // 1072c mov rdx,r14
            rdx = r14;
            // 1072f call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10734 sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10738 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 1073d mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 10741 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10745 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10749 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1074d jmp 0x109e6
            pc = 0x109e6; continue;
        }
        0x10752 => {
            // block 10752
            // 10752 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10756 mov r12,QWORD PTR [r15+0x18]
            r12 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1075a mov rbx,r14
            rbx = r14;
            // 1075d sub rbx,QWORD PTR [r15+0x8]
            { let (r, f) = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64);
              rbx = r;
              fl = f; }
            // 10761 mov rax,r12
            rax = r12;
            // 10764 sub rax,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rax, m.r64(r15) as u64);
              rax = r;
              fl = f; }
            // 10767 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 1076a jae 0x10794
            if fl.ae() { pc = 0x10794; continue; }
            pc = 0x1076c;
        }
        0x1076c => {
            // block 1076c
            // 1076c mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 10771 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10776 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 10779 je 0x10908
            if fl.e() { pc = 0x10908; continue; }
            pc = 0x1077f;
        }
        0x1077f => {
            // block 1077f
            // 1077f mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 10783 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 10787 mov rdi,r15
            rdi = r15;
            // 1078a call 0xeb846
            { let r = { let t = m.env.stub_va(0x13); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1078f jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10794 => {
            // block 10794
            // 10794 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10798 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 1079e cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 107a2 and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 107a6 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 107aa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 107ae lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 107b6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107bb mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 107bf add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 107c2 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 107c6 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 107ca lea rax,[rax+r14*8]
            rax = rax.wrapping_add(r14.wrapping_mul(8));
            // 107ce mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 107d2 mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 107d7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107dc mov rbx,rax
            rbx = rax;
            // 107df mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // 107e3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 107e7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 107eb call 0xeb858
            { let r = { let t = m.env.stub_va(0x25); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107f0 mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 107f4 lea r14,[rbp-0x50]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            pc = 0x107f8;
        }
        0x107f8 => {
            // block 107f8
            // 107f8 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 107fc cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 107ff je 0x10826
            if fl.e() { pc = 0x10826; continue; }
            pc = 0x10801;
        }
        0x10801 => {
            // block 10801
            // 10801 add rbx,0xfffffffffffffff8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff8u64);
              rbx = r;
              fl = f; }
            // 10805 mov rdi,r14
            rdi = r14;
            // 10808 mov rsi,rbx
            rsi = rbx;
            // 1080b call 0xeb852
            { let r = { let t = m.env.stub_va(0x1f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10810 jmp 0x107f8
            pc = 0x107f8; continue;
        }
        0x10812 => {
            // block 10812
            // 10812 mov r14,rax
            r14 = rax;
            pc = 0x10815;
        }
        0x10815 => {
            // block 10815
            // 10815 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10819 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1081e mov rdi,r14
            rdi = r14;
            // 10821 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            pc = 0x10826;
        }
        0x10826 => {
            // block 10826
            // 10826 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10829 movaps xmm0,XMMWORD PTR [rbp-0x50]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0));
            // 1082d mov QWORD PTR [rbp-0x50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rcx) as u64);
            // 10831 movups XMMWORD PTR [r15],xmm0
            m.w128(r15, x0);
            // 10835 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 10839 movaps xmm0,XMMWORD PTR [rbp-0x40]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0));
            // 1083d movups xmm1,XMMWORD PTR [r15+0x10]
            x1 = m.r128(r15.wrapping_add(0x10));
            // 10842 movups XMMWORD PTR [r15+0x10],xmm0
            m.w128(r15.wrapping_add(0x10), x0);
            // 10847 movaps XMMWORD PTR [rbp-0x40],xmm1
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x1);
            // 1084b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1084f call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10854 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10859 => {
            // block 10859
            // 10859 mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 1085d mov rax,r14
            rax = r14;
            // 10860 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10863 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10867 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 1086d cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10871 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 1087b and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 1087e mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10882 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10886 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1088a lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10892 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10899 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1089e mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 108a2 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 108a6 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 108aa mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108ae mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 108b2 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 108b6 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 108ba cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 108bd je 0x109a8
            if fl.e() { pc = 0x109a8; continue; }
            pc = 0x108c3;
        }
        0x108c3 => {
            // block 108c3
            // 108c3 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x108c7;
        }
        0x108c7 => {
            // block 108c7
            // 108c7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 108cb mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 108ce add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 108d2 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108d6 mov rax,rbx
            rax = rbx;
            // 108d9 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 108dd cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 108e0 mov rbx,r13
            rbx = r13;
            // 108e3 mov r13,rax
            r13 = rax;
            // 108e6 jne 0x108c7
            if fl.ne() { pc = 0x108c7; continue; }
            pc = 0x108e8;
        }
        0x108e8 => {
            // block 108e8
            // 108e8 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 108eb mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 108ef mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 108f3 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 108f7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 108fb mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 108ff mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10903 jmp 0x109ba
            pc = 0x109ba; continue;
        }
        0x10908 => {
            // block 10908
            // 10908 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1090c lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 10910 mov rdi,r15
            rdi = r15;
            // 10913 call 0xeb840
            { let r = { let t = m.env.stub_va(0xd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10918 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1091c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1091f lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 10923 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 10927 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1092b mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1092f cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 10932 jne 0x10adf
            if fl.ne() { pc = 0x10adf; continue; }
            pc = 0x10938;
        }
        0x10938 => {
            // block 10938
            // 10938 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 1093c mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 1093f mov rax,r13
            rax = r13;
            // 10942 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10945 jbe 0x109fd
            if fl.be() { pc = 0x109fd; continue; }
            pc = 0x1094b;
        }
        0x1094b => {
            // block 1094b
            // 1094b sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 1094f lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10953 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10957 lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 1095c sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1095f mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10963 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 1096b mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 1096f mov rdi,rbx
            rdi = rbx;
            // 10972 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 10975 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 10979 sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 1097c mov r12,r14
            r12 = r14;
            // 1097f shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10983 mov rsi,r13
            rsi = r13;
            // 10986 mov rdx,r14
            rdx = r14;
            // 10989 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1098e sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10992 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 10997 mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 1099b mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1099f sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 109a3 jmp 0x10adb
            pc = 0x10adb; continue;
        }
        0x109a8 => {
            // block 109a8
            // 109a8 mov rsi,rcx
            rsi = rcx;
            // 109ab mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 109af mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 109b3 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x109ba;
        }
        0x109ba => {
            // block 109ba
            // 109ba mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 109bd mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 109c1 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 109c5 mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 109c9 mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 109cd mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 109d1 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 109d5 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 109d9 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 109dd call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 109e2 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x109e6;
        }
        0x109e6 => {
            // block 109e6
            // 109e6 mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // 109e9 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            pc = 0x109ee;
        }
        0x109ee => {
            // block 109ee
            // 109ee add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 109f2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 109f3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 109f5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 109f7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 109f9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 109fb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 109fc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x109fd => {
            // block 109fd
            // 109fd mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 10a01 mov rax,r14
            rax = r14;
            // 10a04 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10a07 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10a0b mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 10a11 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10a15 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10a1f and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10a22 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10a26 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10a2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10a2e lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10a36 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10a3d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10a42 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10a46 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10a4a lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10a4e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a52 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10a56 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 10a5a mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10a5e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 10a61 je 0x10aa1
            if fl.e() { pc = 0x10aa1; continue; }
            pc = 0x10a63;
        }
        0x10a63 => {
            // block 10a63
            // 10a63 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 10a67 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10a6a add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10a6e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a72 mov rax,rbx
            rax = rbx;
            // 10a75 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 10a79 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 10a7c mov rbx,r13
            rbx = r13;
            // 10a7f mov r13,rax
            r13 = rax;
            // 10a82 jne 0x10a63
            if fl.ne() { pc = 0x10a63; continue; }
            pc = 0x10a84;
        }
        0x10a84 => {
            // block 10a84
            // 10a84 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 10a87 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10a8b mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10a8f mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10a93 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10a97 mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10a9b mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10a9f jmp 0x10aaf
            pc = 0x10aaf; continue;
        }
        0x10aa1 => {
            // block 10aa1
            // 10aa1 mov rsi,rcx
            rsi = rcx;
            // 10aa4 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 10aa8 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x10aaf;
        }
        0x10aaf => {
            // block 10aaf
            // 10aaf mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10ab2 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 10ab6 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10aba mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 10abe mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10ac2 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 10ac6 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10aca mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 10ace lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10ad2 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10ad7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10adb;
        }
        0x10adb => {
            // block 10adb
            // 10adb mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x10adf;
        }
        0x10adf => {
            // block 10adf
            // 10adf mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 10ae2 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10ae7 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10aec => {
            // block 10aec
            // 10aec jmp 0x10812
            pc = 0x10812; continue;
        }
        0x10af1 => {
            // block 10af1
            // 10af1 mov r14,rax
            r14 = rax;
            // 10af4 mov rdi,rbx
            rdi = rbx;
            // 10af7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10afc jmp 0x10815
            pc = 0x10815; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x63714 size=1439
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00063714<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x63714;
    loop {
        match pc {
        0x63714 => {
            // block 63714
            // 63714 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63715 mov rbp,rsp
            rbp = rsp;
            // 63718 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6371a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6371c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6371d sub rsp,0xa8
            { let (r, f) = sub::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63724 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 63726 mov r15,rdi
            r15 = rdi;
            // 63729 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 6372d call 0x633ff
            { let r = { let t = m.env.code_va(0x633ff); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63732 movzx edx,BYTE PTR [r15]
            rdx = (m.r8(r15) as u64) as u32 as u64;
            // 63736 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63738 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6373a jne 0x63741
            if fl.ne() { pc = 0x63741; continue; }
            pc = 0x6373c;
        }
        0x6373c => {
            // block 6373c
            // 6373c shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6373f jmp 0x63745
            pc = 0x63745; continue;
        }
        0x63741 => {
            // block 63741
            // 63741 mov rdx,QWORD PTR [r15+0x8]
            rdx = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63745;
        }
        0x63745 => {
            // block 63745
            // 63745 movzx esi,BYTE PTR [rbp-0x30]
            rsi = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64) as u32 as u64;
            // 63749 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6374c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6374f jne 0x63756
            if fl.ne() { pc = 0x63756; continue; }
            pc = 0x63751;
        }
        0x63751 => {
            // block 63751
            // 63751 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63754 jmp 0x6375a
            pc = 0x6375a; continue;
        }
        0x63756 => {
            // block 63756
            // 63756 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            pc = 0x6375a;
        }
        0x6375a => {
            // block 6375a
            // 6375a cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6375d jne 0x637bb
            if fl.ne() { pc = 0x637bb; continue; }
            pc = 0x6375f;
        }
        0x6375f => {
            // block 6375f
            // 6375f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63761 jne 0x63769
            if fl.ne() { pc = 0x63769; continue; }
            pc = 0x63763;
        }
        0x63763 => {
            // block 63763
            // 63763 lea rdi,[r15+0x1]
            rdi = r15.wrapping_add(0x1);
            // 63767 jmp 0x6376d
            pc = 0x6376d; continue;
        }
        0x63769 => {
            // block 63769
            // 63769 mov rdi,QWORD PTR [r15+0x10]
            rdi = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x6376d;
        }
        0x6376d => {
            // block 6376d
            // 6376d test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 6376f jne 0x63777
            if fl.ne() { pc = 0x63777; continue; }
            pc = 0x63771;
        }
        0x63771 => {
            // block 63771
            // 63771 lea rsi,[rbp-0x2f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd1);
            // 63775 jmp 0x6377b
            pc = 0x6377b; continue;
        }
        0x63777 => {
            // block 63777
            // 63777 mov rsi,QWORD PTR [rbp-0x20]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            pc = 0x6377b;
        }
        0x6377b => {
            // block 6377b
            // 6377b mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 6377e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63780 jne 0x637a5
            if fl.ne() { pc = 0x637a5; continue; }
            pc = 0x63782;
        }
        0x63782 => {
            // block 63782
            // 63782 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63785 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x6378b;
        }
        0x6378b => {
            // block 6378b
            // 6378b movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 6378e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63791 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63793 jne 0x637bb
            if fl.ne() { pc = 0x637bb; continue; }
            pc = 0x63795;
        }
        0x63795 => {
            // block 63795
            // 63795 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63798 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 6379b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 6379e jne 0x6378b
            if fl.ne() { pc = 0x6378b; continue; }
            pc = 0x637a0;
        }
        0x637a0 => {
            // block 637a0
            // 637a0 jmp 0x63c85
            pc = 0x63c85; continue;
        }
        0x637a5 => {
            // block 637a5
            // 637a5 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 637a8 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x637ae;
        }
        0x637ae => {
            // block 637ae
            // 637ae call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637b3 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 637b5 je 0x63c85
            if fl.e() { pc = 0x63c85; continue; }
            pc = 0x637bb;
        }
        0x637bb => {
            // block 637bb
            // 637bb xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 637be cmp ebx,0x104782ff
            fl = sub::<32>((rbx as u32 as u64), 0x104782ffu64).1;
            // 637c4 jg 0x637f9
            if fl.g() { pc = 0x637f9; continue; }
            pc = 0x637c6;
        }
        0x637c6 => {
            // block 637c6
            // 637c6 cmp ebx,0x10294700
            fl = sub::<32>((rbx as u32 as u64), 0x10294700u64).1;
            // 637cc jne 0x6383e
            if fl.ne() { pc = 0x6383e; continue; }
            pc = 0x637ce;
        }
        0x637ce => {
            // block 637ce
            // 637ce lea rsi,[rip+0xce0b8] # 0x13188d
            rsi = m.env.data_va(286810);
            // 637d5 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 637d9 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 637de call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 637e3 movzx edx,BYTE PTR [rbp-0x60]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64) as u32 as u64;
            // 637e7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 637e9 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 637eb jne 0x63922
            if fl.ne() { pc = 0x63922; continue; }
            pc = 0x637f1;
        }
        0x637f1 => {
            // block 637f1
            // 637f1 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 637f4 jmp 0x63926
            pc = 0x63926; continue;
        }
        0x637f9 => {
            // block 637f9
            // 637f9 cmp ebx,0x105183ff
            fl = sub::<32>((rbx as u32 as u64), 0x105183ffu64).1;
            // 637ff jg 0x63875
            if fl.g() { pc = 0x63875; continue; }
            pc = 0x63801;
        }
        0x63801 => {
            // block 63801
            // 63801 cmp ebx,0x10478300
            fl = sub::<32>((rbx as u32 as u64), 0x10478300u64).1;
            // 63807 jne 0x638a8
            if fl.ne() { pc = 0x638a8; continue; }
            pc = 0x6380d;
        }
        0x6380d => {
            // block 6380d
            // 6380d lea rsi,[rip+0xce03b] # 0x13184f
            rsi = m.env.data_va(286748);
            // 63814 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 6381b mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 63820 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63825 movzx edx,BYTE PTR [rbp-0xa8]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff58)) as u64) as u32 as u64;
            // 6382c mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 6382e and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63830 jne 0x6394e
            if fl.ne() { pc = 0x6394e; continue; }
            pc = 0x63836;
        }
        0x63836 => {
            // block 63836
            // 63836 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63839 jmp 0x63955
            pc = 0x63955; continue;
        }
        0x6383e => {
            // block 6383e
            // 6383e cmp ebx,0x10322950
            fl = sub::<32>((rbx as u32 as u64), 0x10322950u64).1;
            // 63844 jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x6384a;
        }
        0x6384a => {
            // block 6384a
            // 6384a lea rsi,[rip+0xce030] # 0x131881
            rsi = m.env.data_va(286798);
            // 63851 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63855 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 6385a call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6385f movzx edx,BYTE PTR [rbp-0x48]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64) as u32 as u64;
            // 63863 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63865 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63867 jne 0x63966
            if fl.ne() { pc = 0x63966; continue; }
            pc = 0x6386d;
        }
        0x6386d => {
            // block 6386d
            // 6386d shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 63870 jmp 0x6396a
            pc = 0x6396a; continue;
        }
        0x63875 => {
            // block 63875
            // 63875 cmp ebx,0x10518400
            fl = sub::<32>((rbx as u32 as u64), 0x10518400u64).1;
            // 6387b jne 0x638e5
            if fl.ne() { pc = 0x638e5; continue; }
            pc = 0x6387d;
        }
        0x6387d => {
            // block 6387d
            // 6387d lea rsi,[rip+0xce026] # 0x1318aa
            rsi = m.env.data_va(286839);
            // 63884 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63888 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 6388d call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63892 movzx edx,BYTE PTR [rbp-0x78]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64) as u32 as u64;
            // 63896 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63898 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6389a jne 0x6397b
            if fl.ne() { pc = 0x6397b; continue; }
            pc = 0x638a0;
        }
        0x638a0 => {
            // block 638a0
            // 638a0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638a3 jmp 0x6397f
            pc = 0x6397f; continue;
        }
        0x638a8 => {
            // block 638a8
            // 638a8 cmp ebx,0x10478350
            fl = sub::<32>((rbx as u32 as u64), 0x10478350u64).1;
            // 638ae jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x638b4;
        }
        0x638b4 => {
            // block 638b4
            // 638b4 lea rsi,[rip+0xcdf87] # 0x131842
            rsi = m.env.data_va(286735);
            // 638bb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 638c2 mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 638c7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 638cc movzx edx,BYTE PTR [rbp-0xc0]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // 638d3 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 638d5 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 638d7 jne 0x63a43
            if fl.ne() { pc = 0x63a43; continue; }
            pc = 0x638dd;
        }
        0x638dd => {
            // block 638dd
            // 638dd shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 638e0 jmp 0x63a4a
            pc = 0x63a4a; continue;
        }
        0x638e5 => {
            // block 638e5
            // 638e5 cmp ebx,0x10518450
            fl = sub::<32>((rbx as u32 as u64), 0x10518450u64).1;
            // 638eb jne 0x63c85
            if fl.ne() { pc = 0x63c85; continue; }
            pc = 0x638f1;
        }
        0x638f1 => {
            // block 638f1
            // 638f1 lea rsi,[rip+0xcdfa2] # 0x13189a
            rsi = m.env.data_va(286823);
            // 638f8 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 638ff mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 63904 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63909 movzx edx,BYTE PTR [rbp-0x90]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 63910 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 63912 and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63914 jne 0x63a5b
            if fl.ne() { pc = 0x63a5b; continue; }
            pc = 0x6391a;
        }
        0x6391a => {
            // block 6391a
            // 6391a shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // 6391d jmp 0x63a62
            pc = 0x63a62; continue;
        }
        0x63922 => {
            // block 63922
            // 63922 mov rdx,QWORD PTR [rbp-0x58]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            pc = 0x63926;
        }
        0x63926 => {
            // block 63926
            // 63926 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6392a mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6392d and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63930 jne 0x63937
            if fl.ne() { pc = 0x63937; continue; }
            pc = 0x63932;
        }
        0x63932 => {
            // block 63932
            // 63932 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63935 jmp 0x6393b
            pc = 0x6393b; continue;
        }
        0x63937 => {
            // block 63937
            // 63937 mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x6393b;
        }
        0x6393b => {
            // block 6393b
            // 6393b cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 6393e jne 0x63a22
            if fl.ne() { pc = 0x63a22; continue; }
            pc = 0x63944;
        }
        0x63944 => {
            // block 63944
            // 63944 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63946 jne 0x63990
            if fl.ne() { pc = 0x63990; continue; }
            pc = 0x63948;
        }
        0x63948 => {
            // block 63948
            // 63948 lea rdi,[rbp-0x5f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa1);
            // 6394c jmp 0x63994
            pc = 0x63994; continue;
        }
        0x6394e => {
            // block 6394e
            // 6394e mov rdx,QWORD PTR [rbp-0xa0]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64;
            pc = 0x63955;
        }
        0x63955 => {
            // block 63955
            // 63955 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63959 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 6395c and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6395f jne 0x6399d
            if fl.ne() { pc = 0x6399d; continue; }
            pc = 0x63961;
        }
        0x63961 => {
            // block 63961
            // 63961 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63964 jmp 0x639a1
            pc = 0x639a1; continue;
        }
        0x63966 => {
            // block 63966
            // 63966 mov rdx,QWORD PTR [rbp-0x40]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            pc = 0x6396a;
        }
        0x6396a => {
            // block 6396a
            // 6396a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 6396e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63971 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63974 jne 0x639be
            if fl.ne() { pc = 0x639be; continue; }
            pc = 0x63976;
        }
        0x63976 => {
            // block 63976
            // 63976 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63979 jmp 0x639c2
            pc = 0x639c2; continue;
        }
        0x6397b => {
            // block 6397b
            // 6397b mov rdx,QWORD PTR [rbp-0x70]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x6397f;
        }
        0x6397f => {
            // block 6397f
            // 6397f movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63983 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63986 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63989 jne 0x639dc
            if fl.ne() { pc = 0x639dc; continue; }
            pc = 0x6398b;
        }
        0x6398b => {
            // block 6398b
            // 6398b shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 6398e jmp 0x639e0
            pc = 0x639e0; continue;
        }
        0x63990 => {
            // block 63990
            // 63990 mov rdi,QWORD PTR [rbp-0x50]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            pc = 0x63994;
        }
        0x63994 => {
            // block 63994
            // 63994 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63996 jne 0x639fa
            if fl.ne() { pc = 0x639fa; continue; }
            pc = 0x63998;
        }
        0x63998 => {
            // block 63998
            // 63998 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 6399b jmp 0x639fe
            pc = 0x639fe; continue;
        }
        0x6399d => {
            // block 6399d
            // 6399d mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639a1;
        }
        0x639a1 => {
            // block 639a1
            // 639a1 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639a4 jne 0x63b1d
            if fl.ne() { pc = 0x63b1d; continue; }
            pc = 0x639aa;
        }
        0x639aa => {
            // block 639aa
            // 639aa test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639ac jne 0x63a73
            if fl.ne() { pc = 0x63a73; continue; }
            pc = 0x639b2;
        }
        0x639b2 => {
            // block 639b2
            // 639b2 lea rdi,[rbp-0xa7]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff59);
            // 639b9 jmp 0x63a7a
            pc = 0x63a7a; continue;
        }
        0x639be => {
            // block 639be
            // 639be mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639c2;
        }
        0x639c2 => {
            // block 639c2
            // 639c2 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639c5 jne 0x63b4a
            if fl.ne() { pc = 0x63b4a; continue; }
            pc = 0x639cb;
        }
        0x639cb => {
            // block 639cb
            // 639cb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639cd jne 0x63a83
            if fl.ne() { pc = 0x63a83; continue; }
            pc = 0x639d3;
        }
        0x639d3 => {
            // block 639d3
            // 639d3 lea rdi,[rbp-0x47]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb9);
            // 639d7 jmp 0x63a87
            pc = 0x63a87; continue;
        }
        0x639dc => {
            // block 639dc
            // 639dc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x639e0;
        }
        0x639e0 => {
            // block 639e0
            // 639e0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 639e3 jne 0x63b77
            if fl.ne() { pc = 0x63b77; continue; }
            pc = 0x639e9;
        }
        0x639e9 => {
            // block 639e9
            // 639e9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 639eb jne 0x63a97
            if fl.ne() { pc = 0x63a97; continue; }
            pc = 0x639f1;
        }
        0x639f1 => {
            // block 639f1
            // 639f1 lea rdi,[rbp-0x77]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff89);
            // 639f5 jmp 0x63a9b
            pc = 0x63a9b; continue;
        }
        0x639fa => {
            // block 639fa
            // 639fa mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x639fe;
        }
        0x639fe => {
            // block 639fe
            // 639fe mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63a01 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63a03 jne 0x63a27
            if fl.ne() { pc = 0x63a27; continue; }
            pc = 0x63a05;
        }
        0x63a05 => {
            // block 63a05
            // 63a05 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a08 je 0x63a3a
            if fl.e() { pc = 0x63a3a; continue; }
            pc = 0x63a0a;
        }
        0x63a0a => {
            // block 63a0a
            // 63a0a movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63a0e movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63a11 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63a13 jne 0x63a22
            if fl.ne() { pc = 0x63a22; continue; }
            pc = 0x63a15;
        }
        0x63a15 => {
            // block 63a15
            // 63a15 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63a18 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a1b dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63a1e jne 0x63a0a
            if fl.ne() { pc = 0x63a0a; continue; }
            pc = 0x63a20;
        }
        0x63a20 => {
            // block 63a20
            // 63a20 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a22 => {
            // block 63a22
            // 63a22 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63a25 jmp 0x63a3a
            pc = 0x63a3a; continue;
        }
        0x63a27 => {
            // block 63a27
            // 63a27 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63a2a je 0x63a3a
            if fl.e() { pc = 0x63a3a; continue; }
            pc = 0x63a2c;
        }
        0x63a2c => {
            // block 63a2c
            // 63a2c mov rsi,r15
            rsi = r15;
            // 63a2f call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63a34 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63a36 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63a3a;
        }
        0x63a3a => {
            // block 63a3a
            // 63a3a lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 63a3e jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63a43 => {
            // block 63a43
            // 63a43 mov rdx,QWORD PTR [rbp-0xb8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64;
            pc = 0x63a4a;
        }
        0x63a4a => {
            // block 63a4a
            // 63a4a movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a4e mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a51 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a54 jne 0x63aab
            if fl.ne() { pc = 0x63aab; continue; }
            pc = 0x63a56;
        }
        0x63a56 => {
            // block 63a56
            // 63a56 shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a59 jmp 0x63aaf
            pc = 0x63aaf; continue;
        }
        0x63a5b => {
            // block 63a5b
            // 63a5b mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x63a62;
        }
        0x63a62 => {
            // block 63a62
            // 63a62 movzx esi,BYTE PTR [r15]
            rsi = (m.r8(r15) as u64) as u32 as u64;
            // 63a66 mov cl,sil
            rcx = (rcx & !0xff) | (((rsi as u8 as u64)) as u8 as u64);
            // 63a69 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 63a6c jne 0x63acc
            if fl.ne() { pc = 0x63acc; continue; }
            pc = 0x63a6e;
        }
        0x63a6e => {
            // block 63a6e
            // 63a6e shr rsi,1
            { let (r, f) = shr::<64>(rsi, 0x1u64, fl); rsi = r; fl = f; }
            // 63a71 jmp 0x63ad0
            pc = 0x63ad0; continue;
        }
        0x63a73 => {
            // block 63a73
            // 63a73 mov rdi,QWORD PTR [rbp-0x98]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64;
            pc = 0x63a7a;
        }
        0x63a7a => {
            // block 63a7a
            // 63a7a test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a7c jne 0x63aed
            if fl.ne() { pc = 0x63aed; continue; }
            pc = 0x63a7e;
        }
        0x63a7e => {
            // block 63a7e
            // 63a7e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a81 jmp 0x63af1
            pc = 0x63af1; continue;
        }
        0x63a83 => {
            // block 63a83
            // 63a83 mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x63a87;
        }
        0x63a87 => {
            // block 63a87
            // 63a87 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a89 jne 0x63b22
            if fl.ne() { pc = 0x63b22; continue; }
            pc = 0x63a8f;
        }
        0x63a8f => {
            // block 63a8f
            // 63a8f inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63a92 jmp 0x63b26
            pc = 0x63b26; continue;
        }
        0x63a97 => {
            // block 63a97
            // 63a97 mov rdi,QWORD PTR [rbp-0x68]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            pc = 0x63a9b;
        }
        0x63a9b => {
            // block 63a9b
            // 63a9b test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63a9d jne 0x63b4f
            if fl.ne() { pc = 0x63b4f; continue; }
            pc = 0x63aa3;
        }
        0x63aa3 => {
            // block 63aa3
            // 63aa3 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63aa6 jmp 0x63b53
            pc = 0x63b53; continue;
        }
        0x63aab => {
            // block 63aab
            // 63aab mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63aaf;
        }
        0x63aaf => {
            // block 63aaf
            // 63aaf cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ab2 jne 0x63c18
            if fl.ne() { pc = 0x63c18; continue; }
            pc = 0x63ab8;
        }
        0x63ab8 => {
            // block 63ab8
            // 63ab8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63aba jne 0x63bd3
            if fl.ne() { pc = 0x63bd3; continue; }
            pc = 0x63ac0;
        }
        0x63ac0 => {
            // block 63ac0
            // 63ac0 lea rdi,[rbp-0xbf]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff41);
            // 63ac7 jmp 0x63bda
            pc = 0x63bda; continue;
        }
        0x63acc => {
            // block 63acc
            // 63acc mov rsi,QWORD PTR [r15+0x8]
            rsi = m.r64(r15.wrapping_add(0x8)) as u64;
            pc = 0x63ad0;
        }
        0x63ad0 => {
            // block 63ad0
            // 63ad0 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 63ad3 jne 0x63c45
            if fl.ne() { pc = 0x63c45; continue; }
            pc = 0x63ad9;
        }
        0x63ad9 => {
            // block 63ad9
            // 63ad9 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63adb jne 0x63be3
            if fl.ne() { pc = 0x63be3; continue; }
            pc = 0x63ae1;
        }
        0x63ae1 => {
            // block 63ae1
            // 63ae1 lea rdi,[rbp-0x8f]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff71);
            // 63ae8 jmp 0x63be7
            pc = 0x63be7; continue;
        }
        0x63aed => {
            // block 63aed
            // 63aed mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63af1;
        }
        0x63af1 => {
            // block 63af1
            // 63af1 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63af4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63af6 jne 0x63b7c
            if fl.ne() { pc = 0x63b7c; continue; }
            pc = 0x63afc;
        }
        0x63afc => {
            // block 63afc
            // 63afc test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63aff je 0x63b8f
            if fl.e() { pc = 0x63b8f; continue; }
            pc = 0x63b05;
        }
        0x63b05 => {
            // block 63b05
            // 63b05 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b09 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b0c cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b0e jne 0x63b1d
            if fl.ne() { pc = 0x63b1d; continue; }
            pc = 0x63b10;
        }
        0x63b10 => {
            // block 63b10
            // 63b10 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b13 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b16 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b19 jne 0x63b05
            if fl.ne() { pc = 0x63b05; continue; }
            pc = 0x63b1b;
        }
        0x63b1b => {
            // block 63b1b
            // 63b1b jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b1d => {
            // block 63b1d
            // 63b1d xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b20 jmp 0x63b8f
            pc = 0x63b8f; continue;
        }
        0x63b22 => {
            // block 63b22
            // 63b22 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b26;
        }
        0x63b26 => {
            // block 63b26
            // 63b26 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b29 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b2b jne 0x63b9b
            if fl.ne() { pc = 0x63b9b; continue; }
            pc = 0x63b2d;
        }
        0x63b2d => {
            // block 63b2d
            // 63b2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b30 je 0x63bae
            if fl.e() { pc = 0x63bae; continue; }
            pc = 0x63b32;
        }
        0x63b32 => {
            // block 63b32
            // 63b32 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b36 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b39 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b3b jne 0x63b4a
            if fl.ne() { pc = 0x63b4a; continue; }
            pc = 0x63b3d;
        }
        0x63b3d => {
            // block 63b3d
            // 63b3d inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b40 inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b43 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b46 jne 0x63b32
            if fl.ne() { pc = 0x63b32; continue; }
            pc = 0x63b48;
        }
        0x63b48 => {
            // block 63b48
            // 63b48 jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4a => {
            // block 63b4a
            // 63b4a xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b4d jmp 0x63bae
            pc = 0x63bae; continue;
        }
        0x63b4f => {
            // block 63b4f
            // 63b4f mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63b53;
        }
        0x63b53 => {
            // block 63b53
            // 63b53 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63b56 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63b58 jne 0x63bb7
            if fl.ne() { pc = 0x63bb7; continue; }
            pc = 0x63b5a;
        }
        0x63b5a => {
            // block 63b5a
            // 63b5a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b5d je 0x63bca
            if fl.e() { pc = 0x63bca; continue; }
            pc = 0x63b5f;
        }
        0x63b5f => {
            // block 63b5f
            // 63b5f movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63b63 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63b66 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63b68 jne 0x63b77
            if fl.ne() { pc = 0x63b77; continue; }
            pc = 0x63b6a;
        }
        0x63b6a => {
            // block 63b6a
            // 63b6a inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63b6d inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63b70 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63b73 jne 0x63b5f
            if fl.ne() { pc = 0x63b5f; continue; }
            pc = 0x63b75;
        }
        0x63b75 => {
            // block 63b75
            // 63b75 jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b77 => {
            // block 63b77
            // 63b77 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63b7a jmp 0x63bca
            pc = 0x63bca; continue;
        }
        0x63b7c => {
            // block 63b7c
            // 63b7c test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b7f je 0x63b8f
            if fl.e() { pc = 0x63b8f; continue; }
            pc = 0x63b81;
        }
        0x63b81 => {
            // block 63b81
            // 63b81 mov rsi,r15
            rsi = r15;
            // 63b84 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63b89 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63b8b sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63b8f;
        }
        0x63b8f => {
            // block 63b8f
            // 63b8f lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 63b96 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63b9b => {
            // block 63b9b
            // 63b9b test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63b9e je 0x63bae
            if fl.e() { pc = 0x63bae; continue; }
            pc = 0x63ba0;
        }
        0x63ba0 => {
            // block 63ba0
            // 63ba0 mov rsi,r15
            rsi = r15;
            // 63ba3 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63ba8 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63baa sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bae;
        }
        0x63bae => {
            // block 63bae
            // 63bae lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 63bb2 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bb7 => {
            // block 63bb7
            // 63bb7 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bba je 0x63bca
            if fl.e() { pc = 0x63bca; continue; }
            pc = 0x63bbc;
        }
        0x63bbc => {
            // block 63bbc
            // 63bbc mov rsi,r15
            rsi = r15;
            // 63bbf call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63bc4 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63bc6 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63bca;
        }
        0x63bca => {
            // block 63bca
            // 63bca lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 63bce jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63bd3 => {
            // block 63bd3
            // 63bd3 mov rdi,QWORD PTR [rbp-0xb0]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff50)) as u64;
            pc = 0x63bda;
        }
        0x63bda => {
            // block 63bda
            // 63bda test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63bdc jne 0x63bf0
            if fl.ne() { pc = 0x63bf0; continue; }
            pc = 0x63bde;
        }
        0x63bde => {
            // block 63bde
            // 63bde inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63be1 jmp 0x63bf4
            pc = 0x63bf4; continue;
        }
        0x63be3 => {
            // block 63be3
            // 63be3 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            pc = 0x63be7;
        }
        0x63be7 => {
            // block 63be7
            // 63be7 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 63be9 jne 0x63c1d
            if fl.ne() { pc = 0x63c1d; continue; }
            pc = 0x63beb;
        }
        0x63beb => {
            // block 63beb
            // 63beb inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63bee jmp 0x63c21
            pc = 0x63c21; continue;
        }
        0x63bf0 => {
            // block 63bf0
            // 63bf0 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63bf4;
        }
        0x63bf4 => {
            // block 63bf4
            // 63bf4 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63bf7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63bf9 jne 0x63c4a
            if fl.ne() { pc = 0x63c4a; continue; }
            pc = 0x63bfb;
        }
        0x63bfb => {
            // block 63bfb
            // 63bfb test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63bfe je 0x63c5d
            if fl.e() { pc = 0x63c5d; continue; }
            pc = 0x63c00;
        }
        0x63c00 => {
            // block 63c00
            // 63c00 movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c04 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c07 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c09 jne 0x63c18
            if fl.ne() { pc = 0x63c18; continue; }
            pc = 0x63c0b;
        }
        0x63c0b => {
            // block 63c0b
            // 63c0b inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c0e inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c11 dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c14 jne 0x63c00
            if fl.ne() { pc = 0x63c00; continue; }
            pc = 0x63c16;
        }
        0x63c16 => {
            // block 63c16
            // 63c16 jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c18 => {
            // block 63c18
            // 63c18 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c1b jmp 0x63c5d
            pc = 0x63c5d; continue;
        }
        0x63c1d => {
            // block 63c1d
            // 63c1d mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x63c21;
        }
        0x63c21 => {
            // block 63c21
            // 63c21 mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            // 63c24 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 63c26 jne 0x63c66
            if fl.ne() { pc = 0x63c66; continue; }
            pc = 0x63c28;
        }
        0x63c28 => {
            // block 63c28
            // 63c28 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c2b je 0x63c79
            if fl.e() { pc = 0x63c79; continue; }
            pc = 0x63c2d;
        }
        0x63c2d => {
            // block 63c2d
            // 63c2d movzx eax,BYTE PTR [r15]
            rax = (m.r8(r15) as u64) as u32 as u64;
            // 63c31 movzx ecx,BYTE PTR [rdi]
            rcx = (m.r8(rdi) as u64) as u32 as u64;
            // 63c34 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // 63c36 jne 0x63c45
            if fl.ne() { pc = 0x63c45; continue; }
            pc = 0x63c38;
        }
        0x63c38 => {
            // block 63c38
            // 63c38 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 63c3b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // 63c3e dec rdx
            { let (r, f) = dec::<64>(rdx, fl); rdx = r; fl = f; }
            // 63c41 jne 0x63c2d
            if fl.ne() { pc = 0x63c2d; continue; }
            pc = 0x63c43;
        }
        0x63c43 => {
            // block 63c43
            // 63c43 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c45 => {
            // block 63c45
            // 63c45 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 63c48 jmp 0x63c79
            pc = 0x63c79; continue;
        }
        0x63c4a => {
            // block 63c4a
            // 63c4a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c4d je 0x63c5d
            if fl.e() { pc = 0x63c5d; continue; }
            pc = 0x63c4f;
        }
        0x63c4f => {
            // block 63c4f
            // 63c4f mov rsi,r15
            rsi = r15;
            // 63c52 call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c57 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c59 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c5d;
        }
        0x63c5d => {
            // block 63c5d
            // 63c5d lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 63c64 jmp 0x63c80
            pc = 0x63c80; continue;
        }
        0x63c66 => {
            // block 63c66
            // 63c66 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 63c69 je 0x63c79
            if fl.e() { pc = 0x63c79; continue; }
            pc = 0x63c6b;
        }
        0x63c6b => {
            // block 63c6b
            // 63c6b mov rsi,r15
            rsi = r15;
            // 63c6e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c73 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 63c75 sete r14b
            r14 = (r14 & !0xff) | (((fl.e() as u64)) as u8 as u64);
            pc = 0x63c79;
        }
        0x63c79 => {
            // block 63c79
            // 63c79 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            pc = 0x63c80;
        }
        0x63c80 => {
            // block 63c80
            // 63c80 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x63c85;
        }
        0x63c85 => {
            // block 63c85
            // 63c85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63c89 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63c8e mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 63c91 add rsp,0xa8
            { let (r, f) = add::<64>(rsp, 0xa8u64);
              rsp = r;
              fl = f; }
            // 63c98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 63c99 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 63c9b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 63c9d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 63c9e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x63c9f => {
            // block 63c9f
            // 63c9f mov rbx,rax
            rbx = rax;
            // 63ca2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 63ca6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 63cab mov rdi,rbx
            rdi = rbx;
            // 63cae call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2c3f7 size=1621
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002c3f7<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2c3f7;
    loop {
        match pc {
        0x2c3f7 => {
            // block 2c3f7
            // 2c3f7 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2c3f8 mov rbp,rsp
            rbp = rsp;
            // 2c3fb push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2c3fd push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2c3ff push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2c401 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2c403 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2c404 sub rsp,0x2e8
            { let (r, f) = sub::<64>(rsp, 0x2e8u64);
              rsp = r;
              fl = f; }
            // 2c40b mov QWORD PTR [rbp-0x2e0],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd20), (rsi) as u64);
            // 2c412 mov QWORD PTR [rbp-0x2f0],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd10), (rdi) as u64);
            // 2c419 xorps xmm0,xmm0
            x0 = 0;
            // 2c41c movaps XMMWORD PTR [rbp-0x80],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ff80), x0);
            // 2c420 mov QWORD PTR [rbp-0x70],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (0x0u64) as u64);
            // 2c428 mov r13,QWORD PTR [rsi]
            r13 = m.r64(rsi) as u64;
            // 2c42b mov rax,QWORD PTR [rsi+0x8]
            rax = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 2c42f cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2c432 je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c438;
        }
        0x2c438 => {
            // block 2c438
            // 2c438 lea rbx,[rbp-0x118]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 2c43f mov esi,0xffffffff
            rsi = (0xffffffffu64) as u32 as u64;
            // 2c444 mov rcx,QWORD PTR [rip+0x113f5d] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 2c44b add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 2c44f mov QWORD PTR [rbp-0x2c8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd38), (rcx) as u64);
            // 2c456 mov rcx,QWORD PTR [rip+0x113dab] # 0x140208
            rcx = m.r64(m.env.data_va(346581)) as u64;
            // 2c45d add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 2c461 mov QWORD PTR [rbp-0x2e8],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd18), (rcx) as u64);
            // 2c468 mov r12d,0xffffffff
            r12 = (0xffffffffu64) as u32 as u64;
            // 2c46e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            pc = 0x2c473;
        }
        0x2c473 => {
            // block 2c473
            // 2c473 mov r14d,DWORD PTR [r13+0x0]
            r14 = (m.r32(r13) as u64) as u32 as u64;
            // 2c477 cmp ecx,0xffffffff
            fl = sub::<32>((rcx as u32 as u64), 0xffffffffu64).1;
            // 2c47a je 0x2c6a7
            if fl.e() { pc = 0x2c6a7; continue; }
            pc = 0x2c480;
        }
        0x2c480 => {
            // block 2c480
            // 2c480 lea edx,[rcx+0x1]
            rdx = (rcx.wrapping_add(0x1)) as u32 as u64;
            // 2c483 cmp r14d,edx
            fl = sub::<32>((r14 as u32 as u64), (rdx as u32 as u64)).1;
            // 2c486 je 0x2c6aa
            if fl.e() { pc = 0x2c6aa; continue; }
            pc = 0x2c48c;
        }
        0x2c48c => {
            // block 2c48c
            // 2c48c cmp r14d,ecx
            fl = sub::<32>((r14 as u32 as u64), (rcx as u32 as u64)).1;
            // 2c48f jne 0x2c499
            if fl.ne() { pc = 0x2c499; continue; }
            pc = 0x2c491;
        }
        0x2c491 => {
            // block 2c491
            // 2c491 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 2c494 jmp 0x2c6ad
            pc = 0x2c6ad; continue;
        }
        0x2c499 => {
            // block 2c499
            // 2c499 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2c49c mov rax,QWORD PTR [rip+0x113d2d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 2c4a3 mov rcx,rax
            rcx = rax;
            // 2c4a6 lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 2c4aa mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c4b1 lea rax,[rcx+0x40]
            rax = rcx.wrapping_add(0x40);
            // 2c4b5 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c4bc mov rdi,rbx
            rdi = rbx;
            // 2c4bf lea rbx,[rbp-0x180]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c4c6 mov rsi,rbx
            rsi = rbx;
            // 2c4c9 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c4ce mov QWORD PTR [rbp-0x90],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff70), (0x0u64) as u64);
            // 2c4d9 mov DWORD PTR [rbp-0x88],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (0xffffffffu64) as u32);
            // 2c4e3 mov rax,QWORD PTR [rip+0x113ed6] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 2c4ea lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // 2c4ee mov QWORD PTR [rbp-0x2d0],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (rcx) as u64);
            // 2c4f5 mov QWORD PTR [rbp-0x188],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rcx) as u64);
            // 2c4fc lea rax,[rax+0x40]
            rax = rax.wrapping_add(0x40);
            // 2c500 mov QWORD PTR [rbp-0x2d8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd28), (rax) as u64);
            // 2c507 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c50e mov rdi,rbx
            rdi = rbx;
            // 2c511 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c516 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c51d mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c524 lea rax,[rbp-0x140]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c52b mov QWORD PTR [rax+0x18],0x0
            m.w64(rax.wrapping_add(0x18), (0x0u64) as u64);
            // 2c533 mov QWORD PTR [rax+0x10],0x0
            m.w64(rax.wrapping_add(0x10), (0x0u64) as u64);
            // 2c53b mov QWORD PTR [rax+0x8],0x0
            m.w64(rax.wrapping_add(0x8), (0x0u64) as u64);
            // 2c543 mov QWORD PTR [rax],0x0
            m.w64(rax, (0x0u64) as u64);
            // 2c54a mov DWORD PTR [rbp-0x120],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (0x10u64) as u32);
            // 2c554 xorps xmm0,xmm0
            x0 = 0;
            // 2c557 movaps XMMWORD PTR [rbp-0x60],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), x0);
            // 2c55b mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // 2c563 mov rdi,rbx
            rdi = rbx;
            // 2c566 lea rbx,[rbp-0x60]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2c56a mov rsi,rbx
            rsi = rbx;
            // 2c56d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c572 mov rdi,rbx
            rdi = rbx;
            // 2c575 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c57a cmp r12d,r15d
            fl = sub::<32>((r12 as u32 as u64), (r15 as u32 as u64)).1;
            // 2c57d jne 0x2c597
            if fl.ne() { pc = 0x2c597; continue; }
            pc = 0x2c57f;
        }
        0x2c57f => {
            // block 2c57f
            // 2c57f lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c586 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c589 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c58e lea rbx,[rbp-0x1a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c595 jmp 0x2c5cc
            pc = 0x2c5cc; continue;
        }
        0x2c597 => {
            // block 2c597
            // 2c597 lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c59e mov esi,r12d
            rsi = ((r12 as u32 as u64)) as u32 as u64;
            // 2c5a1 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5a6 lea rbx,[rbp-0x1a0]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c5ad mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c5b2 mov rdi,rax
            rdi = rax;
            // 2c5b5 lea rsi,[rip+0xfe3c8] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2c5bc call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5c1 mov rdi,rax
            rdi = rax;
            // 2c5c4 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c5c7 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c5cc;
        }
        0x2c5cc => {
            // block 2c5cc
            // 2c5cc mov rdi,rbx
            rdi = rbx;
            // 2c5cf lea rsi,[rbp-0x180]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c5d6 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c5db mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2c5df cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // 2c5e3 jae 0x2c621
            if fl.ae() { pc = 0x2c621; continue; }
            pc = 0x2c5e5;
        }
        0x2c5e5 => {
            // block 2c5e5
            // 2c5e5 mov rcx,QWORD PTR [rbp-0x190]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe70)) as u64;
            // 2c5ec mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2c5f0 mov rcx,QWORD PTR [rbp-0x1a0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe60)) as u64;
            // 2c5f7 mov rdx,QWORD PTR [rbp-0x198]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe68)) as u64;
            // 2c5fe mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2c602 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2c605 xorps xmm0,xmm0
            x0 = 0;
            // 2c608 movaps XMMWORD PTR [rbp-0x1a0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fe60), x0);
            // 2c60f mov QWORD PTR [rbp-0x190],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe70), (0x0u64) as u64);
            // 2c61a add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // 2c61f jmp 0x2c62d
            pc = 0x2c62d; continue;
        }
        0x2c621 => {
            // block 2c621
            // 2c621 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c625 mov rsi,rbx
            rsi = rbx;
            // 2c628 call 0xebe8e
            { let r = { let t = m.env.stub_va(0x65b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c62d;
        }
        0x2c62d => {
            // block 2c62d
            // 2c62d mov rdi,rbx
            rdi = rbx;
            // 2c630 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c635 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c63c mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c643 mov rax,QWORD PTR [rbp-0x2d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 2c64a mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c651 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c658 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c65f lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c666 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c66b lea r15,[rbp-0x180]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c672 mov rdi,r15
            rdi = r15;
            // 2c675 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c67a lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c681 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c688 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c68d lea rbx,[rbp-0x118]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            // 2c694 mov rdi,rbx
            rdi = rbx;
            // 2c697 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c69c mov rax,QWORD PTR [rbp-0x2e0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd20)) as u64;
            // 2c6a3 mov rax,QWORD PTR [rax+0x8]
            rax = m.r64(rax.wrapping_add(0x8)) as u64;
            pc = 0x2c6a7;
        }
        0x2c6a7 => {
            // block 2c6a7
            // 2c6a7 mov r12d,r14d
            r12 = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x2c6aa;
        }
        0x2c6aa => {
            // block 2c6aa
            // 2c6aa mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            pc = 0x2c6ad;
        }
        0x2c6ad => {
            // block 2c6ad
            // 2c6ad add r13,0x4
            { let (r, f) = add::<64>(r13, 0x4u64);
              r13 = r;
              fl = f; }
            // 2c6b1 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2c6b4 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 2c6b7 jne 0x2c473
            if fl.ne() { pc = 0x2c473; continue; }
            pc = 0x2c6bd;
        }
        0x2c6bd => {
            // block 2c6bd
            // 2c6bd cmp r12d,0xffffffff
            fl = sub::<32>((r12 as u32 as u64), 0xffffffffu64).1;
            // 2c6c1 je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c6c7;
        }
        0x2c6c7 => {
            // block 2c6c7
            // 2c6c7 cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // 2c6ca je 0x2c8c2
            if fl.e() { pc = 0x2c8c2; continue; }
            pc = 0x2c6d0;
        }
        0x2c6d0 => {
            // block 2c6d0
            // 2c6d0 mov r15d,r12d
            r15 = ((r12 as u32 as u64)) as u32 as u64;
            // 2c6d3 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2c6d5 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2c6dc lea r13,[rbp-0x2a0]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fd60);
            // 2c6e3 mov rax,QWORD PTR [rip+0x113ae6] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // 2c6ea mov rcx,rax
            rcx = rax;
            // 2c6ed lea rax,[rcx+0x18]
            rax = rcx.wrapping_add(0x18);
            // 2c6f1 mov QWORD PTR [rbp-0x2a8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (rax) as u64);
            // 2c6f8 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 2c6fc mov QWORD PTR [rbp-0x238],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rcx) as u64);
            // 2c703 mov rsi,r13
            rsi = r13;
            // 2c706 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c70b mov QWORD PTR [rbp-0x1b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe50), (0x0u64) as u64);
            // 2c716 mov DWORD PTR [rbp-0x1a8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fe58), (0xffffffffu64) as u32);
            // 2c720 mov rax,QWORD PTR [rip+0x113c99] # 0x1403c0
            rax = m.r64(m.env.data_va(347021)) as u64;
            // 2c727 lea r14,[rax+0x18]
            r14 = rax.wrapping_add(0x18);
            // 2c72b mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c732 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // 2c736 mov QWORD PTR [rbp-0x2d0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd30), (rax) as u64);
            // 2c73d mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c744 mov rdi,r13
            rdi = r13;
            // 2c747 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c74c mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c753 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c75a lea r12,[rbp-0x260]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_fda0);
            // 2c761 mov QWORD PTR [rbp-0x248],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb8), (0x0u64) as u64);
            // 2c76c mov QWORD PTR [rbp-0x250],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdb0), (0x0u64) as u64);
            // 2c777 mov QWORD PTR [rbp-0x258],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda8), (0x0u64) as u64);
            // 2c782 mov QWORD PTR [rbp-0x260],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fda0), (0x0u64) as u64);
            // 2c78d mov DWORD PTR [rbp-0x240],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fdc0), (0x10u64) as u32);
            // 2c797 xorps xmm0,xmm0
            x0 = 0;
            // 2c79a movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // 2c79e mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // 2c7a6 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2c7aa mov rdi,r13
            rdi = r13;
            // 2c7ad call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7b2 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2c7b6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7bb mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2c7bd cmp r15d,esi
            fl = sub::<32>((r15 as u32 as u64), (rsi as u32 as u64)).1;
            // 2c7c0 jne 0x2c7d0
            if fl.ne() { pc = 0x2c7d0; continue; }
            pc = 0x2c7c2;
        }
        0x2c7c2 => {
            // block 2c7c2
            // 2c7c2 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c7c9 call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7ce jmp 0x2c7ff
            pc = 0x2c7ff; continue;
        }
        0x2c7d0 => {
            // block 2c7d0
            // 2c7d0 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // 2c7d2 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c7d9 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2c7dc call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7e1 lea rsi,[rip+0xfe19c] # 0x12a984
            rsi = m.env.data_va(258385);
            // 2c7e8 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c7ed mov rdi,rax
            rdi = rax;
            // 2c7f0 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c7f5 mov rdi,rax
            rdi = rax;
            // 2c7f8 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 2c7fa call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c7ff;
        }
        0x2c7ff => {
            // block 2c7ff
            // 2c7ff lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c806 mov rsi,r13
            rsi = r13;
            // 2c809 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c80e mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 2c812 cmp rax,QWORD PTR [rbp-0x70]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64).1;
            // 2c816 jae 0x2c854
            if fl.ae() { pc = 0x2c854; continue; }
            pc = 0x2c818;
        }
        0x2c818 => {
            // block 2c818
            // 2c818 mov rcx,QWORD PTR [rbp-0x2b0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd50)) as u64;
            // 2c81f mov QWORD PTR [rax+0x10],rcx
            m.w64(rax.wrapping_add(0x10), (rcx) as u64);
            // 2c823 mov rcx,QWORD PTR [rbp-0x2c0]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd40)) as u64;
            // 2c82a mov rdx,QWORD PTR [rbp-0x2b8]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd48)) as u64;
            // 2c831 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            // 2c835 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // 2c838 xorps xmm0,xmm0
            x0 = 0;
            // 2c83b movaps XMMWORD PTR [rbp-0x2c0],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_fd40), x0);
            // 2c842 mov QWORD PTR [rbp-0x2b0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd50), (0x0u64) as u64);
            // 2c84d add QWORD PTR [rbp-0x78],0x18
            { let (r, f) = add::<64>(m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64, 0x18u64);
              m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (r) as u64);
              fl = f; }
            // 2c852 jmp 0x2c864
            pc = 0x2c864; continue;
        }
        0x2c854 => {
            // block 2c854
            // 2c854 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c858 lea rsi,[rbp-0x2c0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c85f call 0xebe8e
            { let r = { let t = m.env.stub_va(0x65b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c864;
        }
        0x2c864 => {
            // block 2c864
            // 2c864 lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2c86b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c870 mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c877 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c87e mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c885 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c88c mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c893 mov rdi,r12
            rdi = r12;
            // 2c896 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c89b mov rdi,r13
            rdi = r13;
            // 2c89e call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8a3 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2c8aa mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c8b1 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8b6 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2c8bd call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c8c2;
        }
        0x2c8c2 => {
            // block 2c8c2
            // 2c8c2 mov rbx,QWORD PTR [rbp-0x80]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 2c8c6 lea r14,[rip+0xfde96] # 0x12a763
            r14 = m.env.data_va(257840);
            // 2c8cd mov r15,QWORD PTR [rbp-0x2f0]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd10)) as u64;
            pc = 0x2c8d4;
        }
        0x2c8d4 => {
            // block 2c8d4
            // 2c8d4 mov rdi,r15
            rdi = r15;
            // 2c8d7 mov rsi,rbx
            rsi = rbx;
            // 2c8da call 0xeba86
            { let r = { let t = m.env.stub_va(0x253); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8df add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            // 2c8e3 cmp rbx,QWORD PTR [rbp-0x78]
            fl = sub::<64>(rbx, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64).1;
            // 2c8e7 je 0x2c903
            if fl.e() { pc = 0x2c903; continue; }
            pc = 0x2c8e9;
        }
        0x2c8e9 => {
            // block 2c8e9
            // 2c8e9 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 2c8ee mov rdi,r15
            rdi = r15;
            // 2c8f1 mov rsi,r14
            rsi = r14;
            // 2c8f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c8f9 jmp 0x2c8d4
            pc = 0x2c8d4; continue;
        }
        0x2c8fb => {
            // block 2c8fb
            // 2c8fb mov r12,rax
            r12 = rax;
            // 2c8fe jmp 0x2c99d
            pc = 0x2c99d; continue;
        }
        0x2c903 => {
            // block 2c903
            // 2c903 lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c907 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c90c mov rax,r15
            rax = r15;
            // 2c90f add rsp,0x2e8
            { let (r, f) = add::<64>(rsp, 0x2e8u64);
              rsp = r;
              fl = f; }
            // 2c916 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c917 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2c919 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2c91b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c91d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c91f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c920 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2c921 => {
            // block 2c921
            // 2c921 mov r12,rax
            r12 = rax;
            pc = 0x2c924;
        }
        0x2c924 => {
            // block 2c924
            // 2c924 mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c92b mov QWORD PTR [rbp-0x188],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe78), (rax) as u64);
            // 2c932 mov rax,QWORD PTR [rbp-0x2d8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd28)) as u64;
            // 2c939 mov QWORD PTR [rbp-0x118],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rax) as u64);
            // 2c940 mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c947 mov QWORD PTR [rbp-0x180],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe80), (rax) as u64);
            // 2c94e jmp 0x2c966
            pc = 0x2c966; continue;
        }
        0x2c950 => {
            // block 2c950
            // 2c950 mov r12,rax
            r12 = rax;
            // 2c953 jmp 0x2c991
            pc = 0x2c991; continue;
        }
        0x2c955 => {
            // block 2c955
            // 2c955 mov r12,rax
            r12 = rax;
            // 2c958 jmp 0x2c97e
            pc = 0x2c97e; continue;
        }
        0x2c95a => {
            // block 2c95a
            // 2c95a mov r12,rax
            r12 = rax;
            // 2c95d lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 2c961 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c966;
        }
        0x2c966 => {
            // block 2c966
            // 2c966 lea rdi,[rbp-0x140]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // 2c96d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c972 lea rdi,[rbp-0x180]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe80);
            // 2c979 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c97e;
        }
        0x2c97e => {
            // block 2c97e
            // 2c97e lea rdi,[rbp-0x188]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe78);
            // 2c985 mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2c98c call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c991;
        }
        0x2c991 => {
            // block 2c991
            // 2c991 lea rdi,[rbp-0x118]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fee8);
            pc = 0x2c998;
        }
        0x2c998 => {
            // block 2c998
            // 2c998 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c99d;
        }
        0x2c99d => {
            // block 2c99d
            // 2c99d lea rdi,[rbp-0x80]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff80);
            // 2c9a1 call 0xeba98
            { let r = { let t = m.env.stub_va(0x265); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9a6 mov rdi,r12
            rdi = r12;
            // 2c9a9 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9ae mov r12,rax
            r12 = rax;
            // 2c9b1 lea rdi,[rbp-0x1a0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fe60);
            // 2c9b8 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c9bd jmp 0x2c924
            pc = 0x2c924; continue;
        }
        0x2c9c2 => {
            // block 2c9c2
            // 2c9c2 mov rbx,r12
            rbx = r12;
            // 2c9c5 mov r12,rax
            r12 = rax;
            pc = 0x2c9c8;
        }
        0x2c9c8 => {
            // block 2c9c8
            // 2c9c8 mov QWORD PTR [rbp-0x2a8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd58), (r14) as u64);
            // 2c9cf mov rax,QWORD PTR [rbp-0x2d0]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd30)) as u64;
            // 2c9d6 mov QWORD PTR [rbp-0x238],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fdc8), (rax) as u64);
            // 2c9dd mov rax,QWORD PTR [rbp-0x2c8]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd38)) as u64;
            // 2c9e4 mov QWORD PTR [rbp-0x2a0],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fd60), (rax) as u64);
            // 2c9eb jmp 0x2ca06
            pc = 0x2ca06; continue;
        }
        0x2c9ed => {
            // block 2c9ed
            // 2c9ed mov r12,rax
            r12 = rax;
            // 2c9f0 jmp 0x2ca29
            pc = 0x2ca29; continue;
        }
        0x2c9f2 => {
            // block 2c9f2
            // 2c9f2 mov r12,rax
            r12 = rax;
            // 2c9f5 jmp 0x2ca16
            pc = 0x2ca16; continue;
        }
        0x2c9f7 => {
            // block 2c9f7
            // 2c9f7 mov rbx,r12
            rbx = r12;
            // 2c9fa mov r12,rax
            r12 = rax;
            // 2c9fd lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 2ca01 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca06;
        }
        0x2ca06 => {
            // block 2ca06
            // 2ca06 mov rdi,rbx
            rdi = rbx;
            // 2ca09 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ca0e mov rdi,r13
            rdi = r13;
            // 2ca11 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca16;
        }
        0x2ca16 => {
            // block 2ca16
            // 2ca16 lea rdi,[rbp-0x2a8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd58);
            // 2ca1d mov rsi,QWORD PTR [rbp-0x2e8]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fd18)) as u64;
            // 2ca24 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2ca29;
        }
        0x2ca29 => {
            // block 2ca29
            // 2ca29 lea rdi,[rbp-0x238]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fdc8);
            // 2ca30 jmp 0x2c998
            pc = 0x2c998; continue;
        }
        0x2ca35 => {
            // block 2ca35
            // 2ca35 mov rbx,r12
            rbx = r12;
            // 2ca38 mov r12,rax
            r12 = rax;
            // 2ca3b lea rdi,[rbp-0x2c0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fd40);
            // 2ca42 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ca47 jmp 0x2c9c8
            pc = 0x2c9c8; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18098 size=2576
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018098<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18098;
    loop {
        match pc {
        0x18098 => {
            // block 18098
            // 18098 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18099 mov rbp,rsp
            rbp = rsp;
            // 1809c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1809e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 180a0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 180a2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 180a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 180a5 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 180ac mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 180b3 mov r15,rdx
            r15 = rdx;
            // 180b6 mov QWORD PTR [rbp-0x130],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r15) as u64);
            // 180bd mov QWORD PTR [rbp-0x160],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rsi) as u64);
            // 180c4 mov QWORD PTR [rbp-0x138],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rdi) as u64);
            // 180cb mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 180ce mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // 180d5 mov r13,QWORD PTR [rsi]
            r13 = m.r64(rsi) as u64;
            // 180d8 mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // 180db jmp 0x18127
            pc = 0x18127; continue;
        }
        0x180dd => {
            // block 180dd
            // 180dd lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 180e4 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            // 180e8 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 180ec call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180f1 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 180f5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180fa lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 180fe call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18103 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18107 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1810c lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18113 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18118 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1811f mov rdi,rbx
            rdi = rbx;
            // 18122 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18127;
        }
        0x18127 => {
            // block 18127
            // 18127 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1812b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1812e jne 0x180dd
            if fl.ne() { pc = 0x180dd; continue; }
            pc = 0x18130;
        }
        0x18130 => {
            // block 18130
            // 18130 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18137 mov r14,QWORD PTR [rdx]
            r14 = m.r64(rdx) as u64;
            // 1813a jmp 0x1818c
            pc = 0x1818c; continue;
        }
        0x1813c => {
            // block 1813c
            // 1813c lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 18143 mov QWORD PTR [rdx+0x8],rax
            m.w64(rdx.wrapping_add(0x8), (rax) as u64);
            // 18147 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1814b mov r12,rdx
            r12 = rdx;
            // 1814e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18153 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 18157 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1815c lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 18160 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18165 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18169 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1816e lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18175 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1817a add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 18181 mov rdi,rbx
            rdi = rbx;
            // 18184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18189 mov rdx,r12
            rdx = r12;
            pc = 0x1818c;
        }
        0x1818c => {
            // block 1818c
            // 1818c mov rbx,QWORD PTR [rdx+0x8]
            rbx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18190 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 18193 jne 0x1813c
            if fl.ne() { pc = 0x1813c; continue; }
            pc = 0x18195;
        }
        0x18195 => {
            // block 18195
            // 18195 mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 1819c mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 181a3 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 181aa jmp 0x181b3
            pc = 0x181b3; continue;
        }
        0x181ac => {
            // block 181ac
            // 181ac add rbx,0xf8
            { let (r, f) = add::<64>(rbx, 0xf8u64);
              rbx = r;
              fl = f; }
            pc = 0x181b3;
        }
        0x181b3 => {
            // block 181b3
            // 181b3 mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 181ba cmp r13,QWORD PTR [r14+0x8]
            fl = sub::<64>(r13, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 181be sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 181c1 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 181c6 jne 0x1835d
            if fl.ne() { pc = 0x1835d; continue; }
            pc = 0x181cc;
        }
        0x181cc => {
            // block 181cc
            // 181cc add r13,0x90
            { let (r, f) = add::<64>(r13, 0x90u64);
              r13 = r;
              fl = f; }
            pc = 0x181d3;
        }
        0x181d3 => {
            // block 181d3
            // 181d3 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 181d5 jne 0x18900
            if fl.ne() { pc = 0x18900; continue; }
            pc = 0x181db;
        }
        0x181db => {
            // block 181db
            // 181db mov eax,DWORD PTR [r13-0x90]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 181e2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 181e4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 181e7 jb 0x18339
            if fl.b() { pc = 0x18339; continue; }
            pc = 0x181ed;
        }
        0x181ed => {
            // block 181ed
            // 181ed mov rbx,r15
            rbx = r15;
            // 181f0 mov r15,QWORD PTR [rbx+0x8]
            r15 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 181f4 cmp r15,QWORD PTR [rbx+0x10]
            fl = sub::<64>(r15, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 181f8 je 0x1831a
            if fl.e() { pc = 0x1831a; continue; }
            pc = 0x181fe;
        }
        0x181fe => {
            // block 181fe
            // 181fe mov rax,QWORD PTR [r13-0x80]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 18202 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 18206 mov rax,QWORD PTR [r13-0x90]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 1820d mov rcx,QWORD PTR [r13-0x88]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 18214 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18218 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 1821b lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // 1821f lea rsi,[r13-0x78]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18223 mov rdi,rbx
            rdi = rbx;
            // 18226 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1822b mov ax,WORD PTR [r13-0x34]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u16 as u64);
            // 18230 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18235 mov eax,DWORD PTR [r13-0x38]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 18239 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1823d mov rax,QWORD PTR [r13-0x40]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 18241 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18245 mov rax,QWORD PTR [r13-0x48]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 18249 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1824d mov rax,QWORD PTR [r13-0x50]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 18251 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18255 mov rax,QWORD PTR [r13-0x60]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 18259 mov rcx,QWORD PTR [r13-0x58]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1825d mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18261 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18265 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 18269 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18270 lea rsi,[r13-0x30]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18274 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18279 mov r12,r14
            r12 = r14;
            // 1827c lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 18280 mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18287 lea rsi,[r13-0x18]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 1828b call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18290 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 18297 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1829e mov rdi,r14
            rdi = r14;
            // 182a1 mov rsi,r13
            rsi = r13;
            // 182a4 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182a9 lea rbx,[r15+0xa8]
            rbx = r15.wrapping_add(0xa8);
            // 182b0 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 182b4 mov rdi,rbx
            rdi = rbx;
            // 182b7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182bc lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 182c3 lea rsi,[r13+0x30]
            rsi = r13.wrapping_add(0x30);
            // 182c7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182cc mov eax,DWORD PTR [r13+0x60]
            rax = (m.r32(r13.wrapping_add(0x60)) as u64) as u32 as u64;
            // 182d0 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 182d7 mov rax,QWORD PTR [r13+0x58]
            rax = m.r64(r13.wrapping_add(0x58)) as u64;
            // 182db mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 182e2 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 182e6 mov rcx,QWORD PTR [r13+0x50]
            rcx = m.r64(r13.wrapping_add(0x50)) as u64;
            // 182ea mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 182f1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 182f8 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 182ff add QWORD PTR [r15+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18307 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1830e mov r14,r12
            r14 = r12;
            // 18311 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 18318 jmp 0x18332
            pc = 0x18332; continue;
        }
        0x1831a => {
            // block 1831a
            // 1831a lea rsi,[r13-0x90]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18321 mov rdi,rbx
            rdi = rbx;
            // 18324 mov r15,rdx
            r15 = rdx;
            // 18327 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1832c mov rdx,r15
            rdx = r15;
            // 1832f mov r15,rbx
            r15 = rbx;
            pc = 0x18332;
        }
        0x18332 => {
            // block 18332
            // 18332 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            pc = 0x18339;
        }
        0x18339 => {
            // block 18339
            // 18339 lea rax,[r13+0x68]
            rax = r13.wrapping_add(0x68);
            // 1833d cmp rax,QWORD PTR [r14+0x8]
            fl = sub::<64>(rax, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 18341 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 18344 add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 1834b cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18350 je 0x181d3
            if fl.e() { pc = 0x181d3; continue; }
            pc = 0x18356;
        }
        0x18356 => {
            // block 18356
            // 18356 add r13,0xffffffffffffff70
            { let (r, f) = add::<64>(r13, 0xffff_ffff_ffff_ff70u64);
              r13 = r;
              fl = f; }
            pc = 0x1835d;
        }
        0x1835d => {
            // block 1835d
            // 1835d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1835f je 0x1848d
            if fl.e() { pc = 0x1848d; continue; }
            pc = 0x18365;
        }
        0x18365 => {
            // block 18365
            // 18365 mov r15,QWORD PTR [rdx+0x8]
            r15 = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18369 cmp r15,QWORD PTR [rdx+0x10]
            fl = sub::<64>(r15, m.r64(rdx.wrapping_add(0x10)) as u64).1;
            // 1836d je 0x186fa
            if fl.e() { pc = 0x186fa; continue; }
            pc = 0x18373;
        }
        0x18373 => {
            // block 18373
            // 18373 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 18377 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 1837b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1837e mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 18382 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18386 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 18389 lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 1838d mov QWORD PTR [rbp-0x148],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rdi) as u64);
            // 18394 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18398 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1839d mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 183a1 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 183a6 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 183a9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 183ad mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 183b1 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 183b5 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 183b9 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 183bd mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 183c1 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 183c5 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 183c9 mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 183cd mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 183d1 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 183d5 lea r14,[r15+0x60]
            r14 = r15.wrapping_add(0x60);
            // 183d9 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 183dd mov rdi,r14
            rdi = r14;
            // 183e0 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183e5 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // 183ec lea r14,[r15+0x78]
            r14 = r15.wrapping_add(0x78);
            // 183f0 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 183f4 mov rdi,r14
            rdi = r14;
            // 183f7 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183fc mov QWORD PTR [rbp-0x158],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r14) as u64);
            // 18403 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1840a lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 18411 mov rdi,r14
            rdi = r14;
            // 18414 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18419 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18420 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 18427 mov rdi,r12
            rdi = r12;
            // 1842a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1842f lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 18436 lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 1843d call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18442 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 18448 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 1844f mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 18456 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 1845d mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 18464 mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 1846b mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 18472 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 18479 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18480 add QWORD PTR [rdx+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rdx.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rdx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18488 jmp 0x188e6
            pc = 0x188e6; continue;
        }
        0x1848d => {
            // block 1848d
            // 1848d mov r14,rdx
            r14 = rdx;
            // 18490 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 18494 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1849b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1849f mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 184a3 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 184aa mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 184b1 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 184b5 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 184bc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 184c1 mov ax,WORD PTR [r13+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 184c6 lea rcx,[rbp-0xf0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 184cd mov rdx,rcx
            rdx = rcx;
            // 184d0 mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 184d4 mov eax,DWORD PTR [r13+0x58]
            rax = (m.r32(r13.wrapping_add(0x58)) as u64) as u32 as u64;
            // 184d8 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 184db mov rax,QWORD PTR [r13+0x50]
            rax = m.r64(r13.wrapping_add(0x50)) as u64;
            // 184df mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 184e3 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 184e7 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 184eb mov rax,QWORD PTR [r13+0x40]
            rax = m.r64(r13.wrapping_add(0x40)) as u64;
            // 184ef mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 184f3 mov rax,QWORD PTR [r13+0x30]
            rax = m.r64(r13.wrapping_add(0x30)) as u64;
            // 184f7 mov rcx,QWORD PTR [r13+0x38]
            rcx = m.r64(r13.wrapping_add(0x38)) as u64;
            // 184fb mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 184ff mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18502 lea rsi,[r13+0x60]
            rsi = r13.wrapping_add(0x60);
            // 18506 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1850d call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18512 lea rsi,[r13+0x78]
            rsi = r13.wrapping_add(0x78);
            // 18516 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1851d call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18522 lea rsi,[r13+0x90]
            rsi = r13.wrapping_add(0x90);
            // 18529 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18530 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18535 lea rsi,[r13+0xa8]
            rsi = r13.wrapping_add(0xa8);
            // 1853c lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18540 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18545 lea rsi,[r13+0xc0]
            rsi = r13.wrapping_add(0xc0);
            // 1854c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18550 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18555 mov eax,DWORD PTR [r13+0xf0]
            rax = (m.r32(r13.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1855c lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18560 mov rdx,rcx
            rdx = rcx;
            // 18563 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 18566 mov rax,QWORD PTR [r13+0xe8]
            rax = m.r64(r13.wrapping_add(0xe8)) as u64;
            // 1856d mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 18571 mov rax,QWORD PTR [r13+0xd8]
            rax = m.r64(r13.wrapping_add(0xd8)) as u64;
            // 18578 mov rcx,QWORD PTR [r13+0xe0]
            rcx = m.r64(r13.wrapping_add(0xe0)) as u64;
            // 1857f mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 18583 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18586 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // 18588 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 1858b cmp eax,DWORD PTR [rbp-0x120]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64).1;
            // 18591 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 18594 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 18597 lea rdx,[rax+0x1]
            rdx = rax.wrapping_add(0x1);
            // 1859b cmp ecx,DWORD PTR [rbp-0x11c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee4)) as u64).1;
            // 185a1 cmove rdx,rax
            if fl.e() { rdx = rax; }
            // 185a5 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185a9 cmp rax,QWORD PTR [rbp-0x110]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64).1;
            // 185b0 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 185b3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 185b6 add rax,rdx
            { let (r, f) = add::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 185b9 mov ecx,DWORD PTR [rbx+0x8]
            rcx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 185bc cmp ecx,DWORD PTR [rbp-0x118]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64).1;
            // 185c2 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 185c7 mov rdx,0xffffffffffffffff
            rdx = 0xffff_ffff_ffff_ffffu64;
            // 185ce cmovne rcx,rdx
            if fl.ne() { rcx = rdx; }
            // 185d2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 185d5 je 0x1889a
            if fl.e() { pc = 0x1889a; continue; }
            pc = 0x185db;
        }
        0x185db => {
            // block 185db
            // 185db mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 185df cmp r15,QWORD PTR [r14+0x10]
            fl = sub::<64>(r15, m.r64(r14.wrapping_add(0x10)) as u64).1;
            // 185e3 je 0x18717
            if fl.e() { pc = 0x18717; continue; }
            pc = 0x185e9;
        }
        0x185e9 => {
            // block 185e9
            // 185e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185ed mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 185f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 185f4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 185f8 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 185fc mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 185ff lea r12,[r15+0x18]
            r12 = r15.wrapping_add(0x18);
            // 18603 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18607 mov rdi,r12
            rdi = r12;
            // 1860a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1860f mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 18613 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18618 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1861b mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1861f mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 18623 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18627 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 1862b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1862f mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 18633 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18637 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 1863b mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 1863f mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18643 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18647 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 1864b mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18652 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 18656 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1865b lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 1865f mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18666 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 1866a call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1866f mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 18676 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1867d mov QWORD PTR [rbp-0x168],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rdi) as u64);
            // 18684 lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 1868b call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18690 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18697 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 1869e mov rdi,r12
            rdi = r12;
            // 186a1 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186a6 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 186ad lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 186b4 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186b9 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 186bf mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 186c6 mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 186cd mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 186d4 mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 186db mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 186e2 mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 186e9 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 186f0 add QWORD PTR [r14+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r14.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 186f8 jmp 0x18722
            pc = 0x18722; continue;
        }
        0x186fa => {
            // block 186fa
            // 186fa mov rdi,rdx
            rdi = rdx;
            // 186fd mov rsi,rbx
            rsi = rbx;
            // 18700 mov r15,rdx
            r15 = rdx;
            // 18703 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18708 mov rdx,r15
            rdx = r15;
            // 1870b mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 18712 jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18717 => {
            // block 18717
            // 18717 mov rdi,r14
            rdi = r14;
            // 1871a mov rsi,rbx
            rsi = rbx;
            // 1871d call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18722;
        }
        0x18722 => {
            // block 18722
            // 18722 mov eax,DWORD PTR [rbp-0x120]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64) as u32 as u64;
            // 18728 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 1872a cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 1872d jb 0x1889a
            if fl.b() { pc = 0x1889a; continue; }
            pc = 0x18733;
        }
        0x18733 => {
            // block 18733
            // 18733 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1873a mov r12,QWORD PTR [rdi+0x8]
            r12 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1873e cmp r12,QWORD PTR [rdi+0x10]
            fl = sub::<64>(r12, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 18742 je 0x1888e
            if fl.e() { pc = 0x1888e; continue; }
            pc = 0x18748;
        }
        0x18748 => {
            // block 18748
            // 18748 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1874f mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 18754 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1875b mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 18762 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 18767 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1876b lea r15,[r12+0x18]
            r15 = r12.wrapping_add(0x18);
            // 18770 mov rdi,r15
            rdi = r15;
            // 18773 lea rsi,[rbp-0x108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1877a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1877f lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 18786 mov rcx,rax
            rcx = rax;
            // 18789 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1878d mov WORD PTR [r12+0x5c],ax
            m.w16(r12.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18793 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 18796 mov DWORD PTR [r12+0x58],eax
            m.w32(r12.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1879b mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 1879f mov QWORD PTR [r12+0x50],rax
            m.w64(r12.wrapping_add(0x50), (rax) as u64);
            // 187a4 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 187a8 mov QWORD PTR [r12+0x48],rax
            m.w64(r12.wrapping_add(0x48), (rax) as u64);
            // 187ad mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 187b1 mov QWORD PTR [r12+0x40],rax
            m.w64(r12.wrapping_add(0x40), (rax) as u64);
            // 187b6 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 187b9 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 187bd mov QWORD PTR [r12+0x38],rcx
            m.w64(r12.wrapping_add(0x38), (rcx) as u64);
            // 187c2 mov QWORD PTR [r12+0x30],rax
            m.w64(r12.wrapping_add(0x30), (rax) as u64);
            // 187c7 lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 187cc mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 187d3 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 187da call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187df mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 187e6 lea rdi,[r12+0x78]
            rdi = r12.wrapping_add(0x78);
            // 187eb mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 187f2 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 187f9 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187fe mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18805 lea rbx,[r12+0x90]
            rbx = r12.wrapping_add(0x90);
            // 1880d mov rdi,rbx
            rdi = rbx;
            // 18810 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18817 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1881c lea r15,[r12+0xa8]
            r15 = r12.wrapping_add(0xa8);
            // 18824 mov rdi,r15
            rdi = r15;
            // 18827 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1882b call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18830 lea rdi,[r12+0xc0]
            rdi = r12.wrapping_add(0xc0);
            // 18838 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1883c call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18841 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18845 mov rcx,rax
            rcx = rax;
            // 18848 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1884b mov DWORD PTR [r12+0xf0],eax
            m.w32(r12.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 18853 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 18857 mov QWORD PTR [r12+0xe8],rax
            m.w64(r12.wrapping_add(0xe8), (rax) as u64);
            // 1885f mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 18862 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 18866 mov QWORD PTR [r12+0xe0],rcx
            m.w64(r12.wrapping_add(0xe0), (rcx) as u64);
            // 1886e mov QWORD PTR [r12+0xd8],rax
            m.w64(r12.wrapping_add(0xd8), (rax) as u64);
            // 18876 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1887d add QWORD PTR [rax+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rax.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rax.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18885 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1888c jmp 0x1889a
            pc = 0x1889a; continue;
        }
        0x1888e => {
            // block 1888e
            // 1888e lea rsi,[rbp-0x120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 18895 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1889a;
        }
        0x1889a => {
            // block 1889a
            // 1889a add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 188a1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 188a5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188aa lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 188ae call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188b3 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 188ba call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188bf lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 188c6 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188cb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 188d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188d7 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 188de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188e3 mov rdx,r14
            rdx = r14;
            pc = 0x188e6;
        }
        0x188e6 => {
            // block 188e6
            // 188e6 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 188ed mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 188f4 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 188fb jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18900 => {
            // block 18900
            // 18900 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 18903 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 18905 cmp rcx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rcx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 18909 jne 0x18915
            if fl.ne() { pc = 0x18915; continue; }
            pc = 0x1890b;
        }
        0x1890b => {
            // block 1890b
            // 1890b mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 1890e cmp rax,QWORD PTR [rdx+0x8]
            fl = sub::<64>(rax, m.r64(rdx.wrapping_add(0x8)) as u64).1;
            // 18912 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x18915;
        }
        0x18915 => {
            // block 18915
            // 18915 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 1891c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1891d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1891f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18921 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18923 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18925 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18926 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18927 => {
            // block 18927
            // 18927 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1892e mov r13,rax
            r13 = rax;
            // 18931 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18936 => {
            // block 18936
            // 18936 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1893d mov r13,rax
            r13 = rax;
            // 18940 jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x18942 => {
            // block 18942
            // 18942 mov r13,rax
            r13 = rax;
            // 18945 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18947 => {
            // block 18947
            // 18947 mov r13,rax
            r13 = rax;
            // 1894a jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x1894c => {
            // block 1894c
            // 1894c mov r13,rax
            r13 = rax;
            // 1894f mov rdi,rbx
            rdi = rbx;
            // 18952 jmp 0x1899e
            pc = 0x1899e; continue;
        }
        0x18954 => {
            // block 18954
            // 18954 mov r13,rax
            r13 = rax;
            // 18957 jmp 0x18a94
            pc = 0x18a94; continue;
        }
        0x1895c => {
            // block 1895c
            // 1895c mov r13,rax
            r13 = rax;
            // 1895f jmp 0x18a88
            pc = 0x18a88; continue;
        }
        0x18964 => {
            // block 18964
            // 18964 mov r13,rax
            r13 = rax;
            // 18967 jmp 0x18a7c
            pc = 0x18a7c; continue;
        }
        0x1896c => {
            // block 1896c
            // 1896c mov r13,rax
            r13 = rax;
            // 1896f jmp 0x18a70
            pc = 0x18a70; continue;
        }
        0x18974 => {
            // block 18974
            // 18974 mov r13,rax
            r13 = rax;
            // 18977 jmp 0x18a67
            pc = 0x18a67; continue;
        }
        0x1897c => {
            // block 1897c
            // 1897c mov r13,rax
            r13 = rax;
            // 1897f jmp 0x18a5e
            pc = 0x18a5e; continue;
        }
        0x18984 => {
            // block 18984
            // 18984 mov r13,rax
            r13 = rax;
            // 18987 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18989 => {
            // block 18989
            // 18989 mov r13,rax
            r13 = rax;
            // 1898c jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x1898e => {
            // block 1898e
            // 1898e mov r13,rax
            r13 = rax;
            // 18991 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18993 => {
            // block 18993
            // 18993 mov r13,rax
            r13 = rax;
            // 18996 jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x18998 => {
            // block 18998
            // 18998 mov r13,rax
            r13 = rax;
            // 1899b mov rdi,r12
            rdi = r12;
            pc = 0x1899e;
        }
        0x1899e => {
            // block 1899e
            // 1899e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189a3;
        }
        0x189a3 => {
            // block 189a3
            // 189a3 mov rdi,r14
            rdi = r14;
            // 189a6 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189ab;
        }
        0x189ab => {
            // block 189ab
            // 189ab mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 189b2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189b7;
        }
        0x189b7 => {
            // block 189b7
            // 189b7 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 189be call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189c3;
        }
        0x189c3 => {
            // block 189c3
            // 189c3 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 189ca jmp 0x18a9b
            pc = 0x18a9b; continue;
        }
        0x189cf => {
            // block 189cf
            // 189cf mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189d6 mov r13,rax
            r13 = rax;
            // 189d9 jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x189db => {
            // block 189db
            // 189db mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189e2 mov r13,rax
            r13 = rax;
            // 189e5 jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x189e7 => {
            // block 189e7
            // 189e7 mov r13,rax
            r13 = rax;
            // 189ea jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x189ec => {
            // block 189ec
            // 189ec mov r13,rax
            r13 = rax;
            // 189ef jmp 0x189fc
            pc = 0x189fc; continue;
        }
        0x189f1 => {
            // block 189f1
            // 189f1 mov r13,rax
            r13 = rax;
            // 189f4 mov rdi,r12
            rdi = r12;
            // 189f7 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189fc;
        }
        0x189fc => {
            // block 189fc
            // 189fc mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 18a03 jmp 0x18a35
            pc = 0x18a35; continue;
        }
        0x18a05 => {
            // block 18a05
            // 18a05 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a0c mov r13,rax
            r13 = rax;
            // 18a0f jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x18a11 => {
            // block 18a11
            // 18a11 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a18 mov r13,rax
            r13 = rax;
            // 18a1b jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x18a1d => {
            // block 18a1d
            // 18a1d mov r13,rax
            r13 = rax;
            // 18a20 jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x18a22 => {
            // block 18a22
            // 18a22 mov r13,rax
            r13 = rax;
            // 18a25 jmp 0x18a32
            pc = 0x18a32; continue;
        }
        0x18a27 => {
            // block 18a27
            // 18a27 mov r13,rax
            r13 = rax;
            // 18a2a mov rdi,r15
            rdi = r15;
            // 18a2d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a32;
        }
        0x18a32 => {
            // block 18a32
            // 18a32 mov rdi,rbx
            rdi = rbx;
            pc = 0x18a35;
        }
        0x18a35 => {
            // block 18a35
            // 18a35 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a3a;
        }
        0x18a3a => {
            // block 18a3a
            // 18a3a mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 18a41 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a46;
        }
        0x18a46 => {
            // block 18a46
            // 18a46 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 18a4d call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a52;
        }
        0x18a52 => {
            // block 18a52
            // 18a52 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 18a59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a5e;
        }
        0x18a5e => {
            // block 18a5e
            // 18a5e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18a62 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a67;
        }
        0x18a67 => {
            // block 18a67
            // 18a67 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18a6b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a70;
        }
        0x18a70 => {
            // block 18a70
            // 18a70 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18a77 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a7c;
        }
        0x18a7c => {
            // block 18a7c
            // 18a7c lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 18a83 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a88;
        }
        0x18a88 => {
            // block 18a88
            // 18a88 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 18a8f call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a94;
        }
        0x18a94 => {
            // block 18a94
            // 18a94 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            pc = 0x18a9b;
        }
        0x18a9b => {
            // block 18a9b
            // 18a9b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18aa0 mov rdi,r13
            rdi = r13;
            // 18aa3 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6ec8 size=114
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6ec8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6ec8;
    loop {
        match pc {
        0xd6ec8 => {
            // block d6ec8
            // d6ec8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6ec9 mov rbp,rsp
            rbp = rsp;
            // d6ecc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d6ecd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d6ece mov rbx,rdi
            rbx = rdi;
            // d6ed1 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d6ed8 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6edb call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6ee1 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // d6ee3 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d6ee5 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // d6ee7 je 0xd6f32
            if fl.e() { pc = 0xd6f32; continue; }
            pc = 0xd6ee9;
        }
        0xd6ee9 => {
            // block d6ee9
            // d6ee9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6eec mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d6ef1 mov edx,0x9f
            rdx = (0x9fu64) as u32 as u64;
            // d6ef6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6efb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6efe mov rdi,rbx
            rdi = rbx;
            // d6f01 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6f07 mov rdi,rbx
            rdi = rbx;
            // d6f0a call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f10 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6f13 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d6f17 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // d6f1c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d6f21 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d6f24 mov rdi,rbx
            rdi = rbx;
            // d6f27 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d6f2a mov eax,0xffffff
            rax = (0xffffffu64) as u32 as u64;
            // d6f2f and eax,DWORD PTR [rbp-0xc]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd6f32;
        }
        0xd6f32 => {
            // block d6f32
            // d6f32 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d6f36 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d6f37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6f38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe957c size=140
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e957c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe957c;
    loop {
        match pc {
        0xe957c => {
            // block e957c
            // e957c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e957d mov rbp,rsp
            rbp = rsp;
            // e9580 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9581 sub rsp,0x98
            { let (r, f) = sub::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9588 mov rbx,rsi
            rbx = rsi;
            // e958b mov BYTE PTR [rbp-0xa0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff60), (0x0u64) as u8);
            // e9592 mov WORD PTR [rbp-0x9e],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff62), (0x0u64) as u16);
            // e959b mov DWORD PTR [rbp-0x9c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff64), (0x0u64) as u32);
            // e95a5 mov DWORD PTR [rbp-0x98],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff68), (0x0u64) as u32);
            // e95af mov BYTE PTR [rbp-0x10],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u8);
            // e95b3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e95b6 lea rsi,[rbp-0xa0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff60);
            // e95bd call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e95c3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e95c5 je 0xe95fe
            if fl.e() { pc = 0xe95fe; continue; }
            pc = 0xe95c7;
        }
        0xe95c7 => {
            // block e95c7
            // e95c7 mov ecx,DWORD PTR [rbp-0xa0]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff60)) as u64) as u32 as u64;
            // e95cd mov BYTE PTR [rbx],cl
            m.w8(rbx, ((rcx as u8 as u64)) as u8);
            // e95cf mov edx,DWORD PTR [rbp-0x98]
            rdx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff68)) as u64) as u32 as u64;
            // e95d5 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // e95d8 shr ecx,0x10
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x10u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e95db mov WORD PTR [rbx+0x2],cx
            m.w16(rbx.wrapping_add(0x2), ((rcx as u16 as u64)) as u16);
            // e95df mov rcx,QWORD PTR [rbp-0x90]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // e95e6 mov rdx,QWORD PTR [rbp-0x88]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // e95ed mov QWORD PTR [rbx+0x18],rdx
            m.w64(rbx.wrapping_add(0x18), (rdx) as u64);
            // e95f1 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // e95f5 mov cl,BYTE PTR [rbp-0x10]
            rcx = (rcx & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u8 as u64);
            // e95f8 mov BYTE PTR [rbx+0x90],cl
            m.w8(rbx.wrapping_add(0x90), ((rcx as u8 as u64)) as u8);
            pc = 0xe95fe;
        }
        0xe95fe => {
            // block e95fe
            // e95fe add rsp,0x98
            { let (r, f) = add::<64>(rsp, 0x98u64);
              rsp = r;
              fl = f; }
            // e9605 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9606 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9607 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb799c size=236
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b799c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb799c;
    loop {
        match pc {
        0xb799c => {
            // block b799c
            // b799c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b799d mov rbp,rsp
            rbp = rsp;
            // b79a0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b79a2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b79a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b79a5 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b79a9 mov rax,rsi
            rax = rsi;
            // b79ac mov r14,rdi
            r14 = rdi;
            // b79af mov r15,QWORD PTR [rip+0x88a22] # 0x1403d8
            r15 = m.r64(m.env.data_va(347045)) as u64;
            // b79b6 mov r15,QWORD PTR [r15]
            r15 = m.r64(r15) as u64;
            // b79b9 mov QWORD PTR [rbp-0x20],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (r15) as u64);
            // b79bd mov rcx,rax
            rcx = rax;
            // b79c0 shr rcx,0x20
            { let (r, f) = shr::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // b79c4 mov BYTE PTR [rbp-0x29],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd7), ((rax as u8 as u64)) as u8);
            // b79c7 mov BYTE PTR [rbp-0x28],ah
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (((rax >> 8) as u8 as u64)) as u8);
            // b79ca mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79cc shr edx,0x10
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x10u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79cf mov BYTE PTR [rbp-0x27],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd9), ((rdx as u8 as u64)) as u8);
            // b79d2 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // b79d4 shr edx,0x18
            { let (r, f) = shr::<32>((rdx as u32 as u64), 0x18u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b79d7 mov BYTE PTR [rbp-0x26],dl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffda), ((rdx as u8 as u64)) as u8);
            // b79da mov BYTE PTR [rbp-0x25],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdb), ((rcx as u8 as u64)) as u8);
            // b79dd mov rcx,rax
            rcx = rax;
            // b79e0 shr rcx,0x28
            { let (r, f) = shr::<64>(rcx, 0x28u64, fl); rcx = r; fl = f; }
            // b79e4 mov BYTE PTR [rbp-0x24],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rcx as u8 as u64)) as u8);
            // b79e7 mov rcx,rax
            rcx = rax;
            // b79ea shr rcx,0x30
            { let (r, f) = shr::<64>(rcx, 0x30u64, fl); rcx = r; fl = f; }
            // b79ee mov BYTE PTR [rbp-0x23],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdd), ((rcx as u8 as u64)) as u8);
            // b79f1 mov rcx,rax
            rcx = rax;
            // b79f4 shr rcx,0x38
            { let (r, f) = shr::<64>(rcx, 0x38u64, fl); rcx = r; fl = f; }
            // b79f8 mov BYTE PTR [rbp-0x22],cl
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffde), ((rcx as u8 as u64)) as u8);
            // b79fb mov BYTE PTR [rbp-0x21],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffdf), (0x0u64) as u8);
            // b79ff test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a01 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a03;
        }
        0xb7a03 => {
            // block b7a03
            // b7a03 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7a05;
        }
        0xb7a05 => {
            // block b7a05
            // b7a05 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a07 and dl,0xdf
            { let r = (rdx as u8 as u64) & 0xdfu64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0a add dl,0xbf
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xbfu64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a0d movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a10 cmp edx,0x1a
            fl = sub::<32>((rdx as u32 as u64), 0x1au64).1;
            // b7a13 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a15;
        }
        0xb7a15 => {
            // block b7a15
            // b7a15 mov dl,al
            rdx = (rdx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // b7a17 add dl,0xd0
            { let (r, f) = add::<8>((rdx as u8 as u64), 0xd0u64);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7a1a movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // b7a1d cmp edx,0xa
            fl = sub::<32>((rdx as u32 as u64), 0xau64).1;
            // b7a20 jb 0xb7a2f
            if fl.b() { pc = 0xb7a2f; continue; }
            pc = 0xb7a22;
        }
        0xb7a22 => {
            // block b7a22
            // b7a22 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7a25 cmp eax,0x20
            fl = sub::<32>((rax as u32 as u64), 0x20u64).1;
            // b7a28 je 0xb7a2f
            if fl.e() { pc = 0xb7a2f; continue; }
            pc = 0xb7a2a;
        }
        0xb7a2a => {
            // block b7a2a
            // b7a2a cmp eax,0x2d
            fl = sub::<32>((rax as u32 as u64), 0x2du64).1;
            // b7a2d jne 0xb7a5d
            if fl.ne() { pc = 0xb7a5d; continue; }
            pc = 0xb7a2f;
        }
        0xb7a2f => {
            // block b7a2f
            // b7a2f lea eax,[rcx+0x1]
            rax = (rcx.wrapping_add(0x1)) as u32 as u64;
            // b7a32 cmp eax,0x7
            fl = sub::<32>((rax as u32 as u64), 0x7u64).1;
            // b7a35 ja 0xb7a46
            if fl.a() { pc = 0xb7a46; continue; }
            pc = 0xb7a37;
        }
        0xb7a37 => {
            // block b7a37
            // b7a37 mov al,BYTE PTR [rbp+rcx*1-0x28]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(rcx).wrapping_add(0xffff_ffff_ffff_ffd8)) as u64) as u8 as u64);
            // b7a3b inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // b7a3e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7a40 jne 0xb7a05
            if fl.ne() { pc = 0xb7a05; continue; }
            pc = 0xb7a42;
        }
        0xb7a42 => {
            // block b7a42
            // b7a42 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // b7a44 je 0xb7a5d
            if fl.e() { pc = 0xb7a5d; continue; }
            pc = 0xb7a46;
        }
        0xb7a46 => {
            // block b7a46
            // b7a46 lea rbx,[rbp-0x29]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffd7);
            // b7a4a mov rdi,rbx
            rdi = rbx;
            // b7a4d call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a52 mov rdi,r14
            rdi = r14;
            // b7a55 mov rsi,rbx
            rsi = rbx;
            // b7a58 mov rdx,rax
            rdx = rax;
            // b7a5b jmp 0xb7a69
            pc = 0xb7a69; continue;
        }
        0xb7a5d => {
            // block b7a5d
            // b7a5d lea rsi,[rip+0x81264] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b7a64 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b7a66 mov rdi,r14
            rdi = r14;
            pc = 0xb7a69;
        }
        0xb7a69 => {
            // block b7a69
            // b7a69 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7a6e cmp r15,QWORD PTR [rbp-0x20]
            fl = sub::<64>(r15, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64).1;
            // b7a72 jne 0xb7a82
            if fl.ne() { pc = 0xb7a82; continue; }
            pc = 0xb7a74;
        }
        0xb7a74 => {
            // block b7a74
            // b7a74 mov rax,r14
            rax = r14;
            // b7a77 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // b7a7b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7a7c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7a7e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b7a80 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7a81 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb7a82 => {
            // block b7a82
            // b7a82 call 0xec194
            { let r = { let t = m.env.stub_va(0x961); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9740 size=668
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9740<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe9740;
    loop {
        match pc {
        0xe9740 => {
            // block e9740
            // e9740 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9741 mov rbp,rsp
            rbp = rsp;
            // e9744 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e9746 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9748 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e974a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e974c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e974d sub rsp,0xc8
            { let (r, f) = sub::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e9754 mov ebx,r8d
            rbx = ((r8 as u32 as u64)) as u32 as u64;
            // e9757 mov r15,rcx
            r15 = rcx;
            // e975a mov DWORD PTR [rbp-0xe4],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rdx as u32 as u64)) as u32);
            // e9760 mov r13,rsi
            r13 = rsi;
            // e9763 mov r14,rdi
            r14 = rdi;
            // e9766 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9769 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e976c xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e976f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e9771 je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe9777;
        }
        0xe9777 => {
            // block e9777
            // e9777 mov BYTE PTR [rbp-0xc0],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff40), (0x0u64) as u8);
            // e977e mov WORD PTR [rbp-0xbe],0x0
            m.w16(rbp.wrapping_add(0xffff_ffff_ffff_ff42), (0x0u64) as u16);
            // e9787 mov DWORD PTR [rbp-0xbc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff44), (0x0u64) as u32);
            // e9791 mov DWORD PTR [rbp-0xb8],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff48), (0x0u64) as u32);
            // e979b mov BYTE PTR [rbp-0x30],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u8);
            // e979f test bl,bl
            fl = logic::<8>((rbx as u8 as u64) & (rbx as u8 as u64));
            // e97a1 je 0xe97d3
            if fl.e() { pc = 0xe97d3; continue; }
            pc = 0xe97a3;
        }
        0xe97a3 => {
            // block e97a3
            // e97a3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97a6 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e97ad mov rdi,r14
            rdi = r14;
            // e97b0 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97b6 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e97bd mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e97c3 mov rax,QWORD PTR [r14+0x288]
            rax = m.r64(r14.wrapping_add(0x288)) as u64;
            // e97ca mov QWORD PTR [r14+0x280],rax
            m.w64(r14.wrapping_add(0x280), (rax) as u64);
            // e97d1 jmp 0xe97fe
            pc = 0xe97fe; continue;
        }
        0xe97d3 => {
            // block e97d3
            // e97d3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e97d6 mov ecx,DWORD PTR [rbp-0xe4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e97dc movzx esi,cx
            rsi = ((rcx as u16 as u64)) as u32 as u64;
            // e97df mov rdi,r14
            rdi = r14;
            // e97e2 call QWORD PTR [rax+0x40]
            { let r = { let t = m.r64(rax.wrapping_add(0x40)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e97e5 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e97e8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e97ea je 0xe99a7
            if fl.e() { pc = 0xe99a7; continue; }
            pc = 0xe97f0;
        }
        0xe97f0 => {
            // block e97f0
            // e97f0 mov rax,QWORD PTR [r14+0x280]
            rax = m.r64(r14.wrapping_add(0x280)) as u64;
            // e97f7 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            pc = 0xe97fe;
        }
        0xe97fe => {
            // block e97fe
            // e97fe add rax,0x18
            { let (r, f) = add::<64>(rax, 0x18u64);
              rax = r;
              fl = f; }
            // e9802 mov QWORD PTR [r14+0x288],rax
            m.w64(r14.wrapping_add(0x288), (rax) as u64);
            // e9809 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e980c lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9813 mov rdi,r14
            rdi = r14;
            // e9816 call QWORD PTR [rax+0x98]
            { let r = { let t = m.r64(rax.wrapping_add(0x98)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e981c mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9822 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9825 jne 0xe9837
            if fl.ne() { pc = 0xe9837; continue; }
            pc = 0xe9827;
        }
        0xe9827 => {
            // block e9827
            // e9827 mov ax,WORD PTR [rbp-0xbe]
            rax = (rax & !0xffff) | ((m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u16 as u64);
            // e982e mov WORD PTR [r15],ax
            m.w16(r15, ((rax as u16 as u64)) as u16);
            // e9832 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            pc = 0xe9837;
        }
        0xe9837 => {
            // block e9837
            // e9837 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e983a jne 0xe9989
            if fl.ne() { pc = 0xe9989; continue; }
            pc = 0xe9840;
        }
        0xe9840 => {
            // block e9840
            // e9840 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e9847 lea r12,[rbp-0xdc]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ff24);
            pc = 0xe984e;
        }
        0xe984e => {
            // block e984e
            // e984e mov r15,QWORD PTR [r14+0x288]
            r15 = m.r64(r14.wrapping_add(0x288)) as u64;
            // e9855 test BYTE PTR [r15],0x1
            fl = logic::<8>(m.r8(r15) as u64 & 0x1u64);
            // e9859 jne 0xe9860
            if fl.ne() { pc = 0xe9860; continue; }
            pc = 0xe985b;
        }
        0xe985b => {
            // block e985b
            // e985b inc r15
            { let (r, f) = inc::<64>(r15, fl); r15 = r; fl = f; }
            // e985e jmp 0xe9864
            pc = 0xe9864; continue;
        }
        0xe9860 => {
            // block e9860
            // e9860 mov r15,QWORD PTR [r15+0x10]
            r15 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0xe9864;
        }
        0xe9864 => {
            // block e9864
            // e9864 mov rdi,r15
            rdi = r15;
            // e9867 call 0xec2c0
            { let r = { let t = m.env.stub_va(0xa8d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e986c mov rdi,rbx
            rdi = rbx;
            // e986f mov rsi,r15
            rsi = r15;
            // e9872 mov rdx,rax
            rdx = rax;
            // e9875 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e987a cmp BYTE PTR [rbp-0xc0],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64, 0x0u64).1;
            // e9881 lea r15,[rip+0x53a9a] # 0x13d322
            r15 = m.env.data_va(334575);
            // e9888 je 0xe9905
            if fl.e() { pc = 0xe9905; continue; }
            pc = 0xe988a;
        }
        0xe988a => {
            // block e988a
            // e988a mov ebx,0x9
            rbx = (0x9u64) as u32 as u64;
            pc = 0xe988f;
        }
        0xe988f => {
            // block e988f
            // e988f mov DWORD PTR [rbp-0xdc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff24), (0x0u64) as u32);
            // e9899 mov rdi,QWORD PTR [r14+0x288]
            rdi = m.r64(r14.wrapping_add(0x288)) as u64;
            // e98a0 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // e98a3 jne 0xe98aa
            if fl.ne() { pc = 0xe98aa; continue; }
            pc = 0xe98a5;
        }
        0xe98a5 => {
            // block e98a5
            // e98a5 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // e98a8 jmp 0xe98ae
            pc = 0xe98ae; continue;
        }
        0xe98aa => {
            // block e98aa
            // e98aa mov rdi,QWORD PTR [rdi+0x10]
            rdi = m.r64(rdi.wrapping_add(0x10)) as u64;
            pc = 0xe98ae;
        }
        0xe98ae => {
            // block e98ae
            // e98ae add rdi,rbx
            { let (r, f) = add::<64>(rdi, rbx);
              rdi = r;
              fl = f; }
            // e98b1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e98b3 mov rsi,r15
            rsi = r15;
            // e98b6 mov rdx,r12
            rdx = r12;
            // e98b9 call 0xec29c
            { let r = { let t = m.env.stub_va(0xa69); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e98be mov al,BYTE PTR [rbp-0xdc]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff24)) as u64) as u8 as u64);
            // e98c4 mov BYTE PTR [rbp-0xdd],al
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ff23), ((rax as u8 as u64)) as u8);
            // e98ca mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // e98ce cmp rcx,QWORD PTR [r13+0x10]
            fl = sub::<64>(rcx, m.r64(r13.wrapping_add(0x10)) as u64).1;
            // e98d2 jae 0xe98dc
            if fl.ae() { pc = 0xe98dc; continue; }
            pc = 0xe98d4;
        }
        0xe98d4 => {
            // block e98d4
            // e98d4 mov BYTE PTR [rcx],al
            m.w8(rcx, ((rax as u8 as u64)) as u8);
            // e98d6 inc QWORD PTR [r13+0x8]
            { let (r, f) = inc::<64>(m.r64(r13.wrapping_add(0x8)) as u64, fl); m.w64(r13.wrapping_add(0x8), (r) as u64); fl = f; }
            // e98da jmp 0xe98eb
            pc = 0xe98eb; continue;
        }
        0xe98dc => {
            // block e98dc
            // e98dc mov rdi,r13
            rdi = r13;
            // e98df lea rsi,[rbp-0xdd]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff23);
            // e98e6 call 0xeb948
            { let r = { let t = m.env.stub_va(0x115); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe98eb;
        }
        0xe98eb => {
            // block e98eb
            // e98eb movzx eax,BYTE PTR [rbp-0xc0]
            rax = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ff40)) as u64) as u32 as u64;
            // e98f2 add rax,rax
            { let (r, f) = add::<64>(rax, rax);
              rax = r;
              fl = f; }
            // e98f5 lea rcx,[rbx+0x2]
            rcx = rbx.wrapping_add(0x2);
            // e98f9 add rbx,0xfffffffffffffff9
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff9u64);
              rbx = r;
              fl = f; }
            // e98fd cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // e9900 mov rbx,rcx
            rbx = rcx;
            // e9903 jl 0xe988f
            if fl.l() { pc = 0xe988f; continue; }
            pc = 0xe9905;
        }
        0xe9905 => {
            // block e9905
            // e9905 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e990d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9910 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9917 mov rdi,r14
            rdi = r14;
            // e991a lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9921 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9923 lea rbx,[rbp-0xd8]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e992a mov eax,DWORD PTR [rbp-0xb8]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64) as u32 as u64;
            // e9930 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // e9933 je 0xe9974
            if fl.e() { pc = 0xe9974; continue; }
            pc = 0xe9935;
        }
        0xe9935 => {
            // block e9935
            // e9935 cmp eax,0x4
            fl = sub::<32>((rax as u32 as u64), 0x4u64).1;
            // e9938 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe993a;
        }
        0xe993a => {
            // block e993a
            // e993a mov eax,DWORD PTR [rbp-0xe4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c)) as u64) as u32 as u64;
            // e9940 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e9942 mov DWORD PTR [rbp-0xe4],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff1c), ((rax as u32 as u64)) as u32);
            // e9948 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // e994b movzx ecx,WORD PTR [rbp-0xbe]
            rcx = (m.r16(rbp.wrapping_add(0xffff_ffff_ffff_ff42)) as u64) as u32 as u64;
            // e9952 cmp ecx,eax
            fl = sub::<32>((rcx as u32 as u64), (rax as u32 as u64)).1;
            // e9954 jne 0xe9993
            if fl.ne() { pc = 0xe9993; continue; }
            pc = 0xe9956;
        }
        0xe9956 => {
            // block e9956
            // e9956 add QWORD PTR [r14+0x288],0x18
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x288)) as u64, 0x18u64);
              m.w64(r14.wrapping_add(0x288), (r) as u64);
              fl = f; }
            // e995e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e9961 mov rax,QWORD PTR [rax+0x98]
            rax = m.r64(rax.wrapping_add(0x98)) as u64;
            // e9968 mov rdi,r14
            rdi = r14;
            // e996b lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // e9972 call rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9974;
        }
        0xe9974 => {
            // block e9974
            // e9974 mov rdi,rbx
            rdi = rbx;
            // e9977 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e997c cmp DWORD PTR [rbp-0xb8],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff48)) as u64, 0x1u64).1;
            // e9983 je 0xe984e
            if fl.e() { pc = 0xe984e; continue; }
            pc = 0xe9989;
        }
        0xe9989 => {
            // block e9989
            // e9989 mov r12d,DWORD PTR [r13+0x8]
            r12 = (m.r32(r13.wrapping_add(0x8)) as u64) as u32 as u64;
            // e998d sub r12d,DWORD PTR [r13+0x0]
            { let (r, f) = sub::<32>((r12 as u32 as u64), m.r32(r13) as u64);
              r12 = (r) as u32 as u64;
              fl = f; }
            // e9991 jmp 0xe99a7
            pc = 0xe99a7; continue;
        }
        0xe9993 => {
            // block e9993
            // e9993 mov r12,QWORD PTR [r13+0x8]
            r12 = m.r64(r13.wrapping_add(0x8)) as u64;
            // e9997 sub r12,QWORD PTR [r13+0x0]
            { let (r, f) = sub::<64>(r12, m.r64(r13) as u64);
              r12 = r;
              fl = f; }
            // e999b lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99a2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe99a7;
        }
        0xe99a7 => {
            // block e99a7
            // e99a7 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // e99aa add rsp,0xc8
            { let (r, f) = add::<64>(rsp, 0xc8u64);
              rsp = r;
              fl = f; }
            // e99b1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e99b2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e99b4 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e99b6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e99b8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e99ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e99bb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe99bc => {
            // block e99bc
            // e99bc mov rbx,rax
            rbx = rax;
            // e99bf lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // e99c6 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99cb mov rdi,rbx
            rdi = rbx;
            // e99ce call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e99d3 jmp 0xe99bc
            pc = 0xe99bc; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb922e size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b922e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb922e;
    loop {
        match pc {
        0xb922e => {
            // block b922e
            // b922e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b922f mov rbp,rsp
            rbp = rsp;
            // b9232 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b9234 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b9236 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b9237 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b9238 mov r14,rdx
            r14 = rdx;
            // b923b mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b923e mov rbx,rdi
            rbx = rdi;
            // b9241 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b9245 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9248 call 0xcbc54
            { let r = { let t = m.env.code_va(0xcbc54); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b924d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b924f je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb9251;
        }
        0xb9251 => {
            // block b9251
            // b9251 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9254 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9259 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b925b je 0xb92b0
            if fl.e() { pc = 0xb92b0; continue; }
            pc = 0xb925d;
        }
        0xb925d => {
            // block b925d
            // b925d mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b9260 call 0xd0ac8
            { let r = { let t = m.env.code_va(0xd0ac8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9265 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b9269 movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b926c cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b926e jbe 0xb92b0
            if fl.be() { pc = 0xb92b0; continue; }
            pc = 0xb9270;
        }
        0xb9270 => {
            // block b9270
            // b9270 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b9277 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b927a lea rdx,[rip+0x496cf] # 0x102950
            rdx = m.env.data_va(94493);
            // b9281 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9284 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b9288 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b928b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9290 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9293 mov rdi,rbx
            rdi = rbx;
            // b9296 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9299 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b929b je 0xb92b3
            if fl.e() { pc = 0xb92b3; continue; }
            pc = 0xb929d;
        }
        0xb929d => {
            // block b929d
            // b929d mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b92a0 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b92a3 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b92a5 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b92a8 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b92ab mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b92ae jmp 0xb92b3
            pc = 0xb92b3; continue;
        }
        0xb92b0 => {
            // block b92b0
            // b92b0 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb92b3;
        }
        0xb92b3 => {
            // block b92b3
            // b92b3 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b92b6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b92ba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b92bb pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b92bd pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b92bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b92c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb92c2 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b92c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb92c2;
    loop {
        match pc {
        0xb92c2 => {
            // block b92c2
            // b92c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b92c3 mov rbp,rsp
            rbp = rsp;
            // b92c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b92c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b92ca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b92cb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b92cc mov r14,rdx
            r14 = rdx;
            // b92cf mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // b92d2 mov rbx,rdi
            rbx = rdi;
            // b92d5 mov BYTE PTR [r14],0x0
            m.w8(r14, (0x0u64) as u8);
            // b92d9 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92dc call 0xcc8c5
            { let r = { let t = m.env.code_va(0xcc8c5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92e1 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92e3 je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92e5;
        }
        0xb92e5 => {
            // block b92e5
            // b92e5 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92e8 call 0xcbe98
            { let r = { let t = m.env.code_va(0xcbe98); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b92ef je 0xb9344
            if fl.e() { pc = 0xb9344; continue; }
            pc = 0xb92f1;
        }
        0xb92f1 => {
            // block b92f1
            // b92f1 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b92f4 call 0xd0c6c
            { let r = { let t = m.env.code_va(0xd0c6c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b92f9 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // b92fd movzx eax,ax
            rax = ((rax as u16 as u64)) as u32 as u64;
            // b9300 cmp eax,ecx
            fl = sub::<32>((rax as u32 as u64), (rcx as u32 as u64)).1;
            // b9302 jbe 0xb9344
            if fl.be() { pc = 0xb9344; continue; }
            pc = 0xb9304;
        }
        0xb9304 => {
            // block b9304
            // b9304 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // b930b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b930e lea rdx,[rip+0x4966b] # 0x102980
            rdx = m.env.data_va(94541);
            // b9315 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // b9318 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // b931c xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // b931f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b9324 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b9327 mov rdi,rbx
            rdi = rbx;
            // b932a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b932d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b932f je 0xb9347
            if fl.e() { pc = 0xb9347; continue; }
            pc = 0xb9331;
        }
        0xb9331 => {
            // block b9331
            // b9331 mov eax,DWORD PTR [rbp-0x1c]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64) as u32 as u64;
            // b9334 shr eax,0x1c
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1cu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b9337 not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b9339 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b933c mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // b933f mov r15b,0x1
            r15 = (r15 & !0xff) | ((0x1u64) as u8 as u64);
            // b9342 jmp 0xb9347
            pc = 0xb9347; continue;
        }
        0xb9344 => {
            // block b9344
            // b9344 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9347;
        }
        0xb9347 => {
            // block b9347
            // b9347 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // b934a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b934e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b934f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b9351 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b9353 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9354 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe48b0 size=158
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e48b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe48b0;
    loop {
        match pc {
        0xe48b0 => {
            // block e48b0
            // e48b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e48b1 mov rbp,rsp
            rbp = rsp;
            // e48b4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e48b6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e48b7 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e48bb mov rbx,rsi
            rbx = rsi;
            // e48be mov r14,rdi
            r14 = rdi;
            // e48c1 mov esi,DWORD PTR [r14+0x8]
            rsi = (m.r32(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // e48c5 cmp esi,0x1a
            fl = sub::<32>((rsi as u32 as u64), 0x1au64).1;
            // e48c8 jne 0xe48d8
            if fl.ne() { pc = 0xe48d8; continue; }
            pc = 0xe48ca;
        }
        0xe48ca => {
            // block e48ca
            // e48ca lea rsi,[r14+0x10]
            rsi = r14.wrapping_add(0x10);
            // e48ce mov rdi,rbx
            rdi = rbx;
            // e48d1 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e48d6 jmp 0xe4935
            pc = 0xe4935; continue;
        }
        0xe48d8 => {
            // block e48d8
            // e48d8 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e48dc call 0x62fd6
            { let r = { let t = m.env.code_va(0x62fd6); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e48e1 test BYTE PTR [rbx],0x1
            fl = logic::<8>(m.r8(rbx) as u64 & 0x1u64);
            // e48e4 jne 0xe48ed
            if fl.ne() { pc = 0xe48ed; continue; }
            pc = 0xe48e6;
        }
        0xe48e6 => {
            // block e48e6
            // e48e6 mov WORD PTR [rbx],0x0
            m.w16(rbx, (0x0u64) as u16);
            // e48eb jmp 0xe48fc
            pc = 0xe48fc; continue;
        }
        0xe48ed => {
            // block e48ed
            // e48ed mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e48f1 mov BYTE PTR [rax],0x0
            m.w8(rax, (0x0u64) as u8);
            // e48f4 mov QWORD PTR [rbx+0x8],0x0
            m.w64(rbx.wrapping_add(0x8), (0x0u64) as u64);
            pc = 0xe48fc;
        }
        0xe48fc => {
            // block e48fc
            // e48fc xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e48fe mov rdi,rbx
            rdi = rbx;
            // e4901 call 0xec014
            { let r = { let t = m.env.stub_va(0x7e1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4906 mov rax,QWORD PTR [rbp-0x20]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64;
            // e490a mov QWORD PTR [rbx+0x10],rax
            m.w64(rbx.wrapping_add(0x10), (rax) as u64);
            // e490e mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // e4912 mov rcx,QWORD PTR [rbp-0x28]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // e4916 mov QWORD PTR [rbx+0x8],rcx
            m.w64(rbx.wrapping_add(0x8), (rcx) as u64);
            // e491a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e491d xorps xmm0,xmm0
            x0 = 0;
            // e4920 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // e4924 mov QWORD PTR [rbp-0x20],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u64);
            // e492c lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e4930 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe4935;
        }
        0xe4935 => {
            // block e4935
            // e4935 mov DWORD PTR [r14+0x8],0x0
            m.w32(r14.wrapping_add(0x8), (0x0u64) as u32);
            // e493d add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e4941 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4942 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4944 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4945 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe4946 => {
            // block e4946
            // e4946 mov rdi,rax
            rdi = rax;
            // e4949 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba782 size=206
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba782<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba782;
    loop {
        match pc {
        0xba782 => {
            // block ba782
            // ba782 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba783 mov rbp,rsp
            rbp = rsp;
            // ba786 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba788 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba78a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ba78c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba78e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba78f sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba793 mov r14,rsi
            r14 = rsi;
            // ba796 mov rbx,rdi
            rbx = rdi;
            // ba799 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // ba79c add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // ba7a0 cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // ba7a3 je 0xba82a
            if fl.e() { pc = 0xba82a; continue; }
            pc = 0xba7a9;
        }
        0xba7a9 => {
            // block ba7a9
            // ba7a9 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba7ad jmp 0xba7c6
            pc = 0xba7c6; continue;
        }
        0xba7af => {
            // block ba7af
            // ba7af mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ba7b4 mov rdi,rbx
            rdi = rbx;
            // ba7b7 lea rsi,[rip+0x6fcdf] # 0x12a49d
            rsi = m.env.data_va(257130);
            // ba7be call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7c3 mov r13,r12
            r13 = r12;
            pc = 0xba7c6;
        }
        0xba7c6 => {
            // block ba7c6
            // ba7c6 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ba7ca mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // ba7cf mov rdi,r15
            rdi = r15;
            // ba7d2 call 0x5e61c
            { let r = { let t = m.env.code_va(0x5e61c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7d7 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // ba7db test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // ba7de jne 0xba7e9
            if fl.ne() { pc = 0xba7e9; continue; }
            pc = 0xba7e0;
        }
        0xba7e0 => {
            // block ba7e0
            // ba7e0 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // ba7e3 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // ba7e7 jmp 0xba7f1
            pc = 0xba7f1; continue;
        }
        0xba7e9 => {
            // block ba7e9
            // ba7e9 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ba7ed mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xba7f1;
        }
        0xba7f1 => {
            // block ba7f1
            // ba7f1 mov rdi,rbx
            rdi = rbx;
            // ba7f4 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba7f9 mov rdi,r15
            rdi = r15;
            // ba7fc call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba801 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // ba805 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba808 je 0xba818
            if fl.e() { pc = 0xba818; continue; }
            pc = 0xba80a;
        }
        0xba80a => {
            // block ba80a
            // ba80a mov r12,rax
            r12 = rax;
            // ba80d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ba811 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba814 jne 0xba80a
            if fl.ne() { pc = 0xba80a; continue; }
            pc = 0xba816;
        }
        0xba816 => {
            // block ba816
            // ba816 jmp 0xba825
            pc = 0xba825; continue;
        }
        0xba818 => {
            // block ba818
            // ba818 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // ba81c cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // ba820 mov r13,r12
            r13 = r12;
            // ba823 jne 0xba818
            if fl.ne() { pc = 0xba818; continue; }
            pc = 0xba825;
        }
        0xba825 => {
            // block ba825
            // ba825 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // ba828 jne 0xba7af
            if fl.ne() { pc = 0xba7af; continue; }
            pc = 0xba82a;
        }
        0xba82a => {
            // block ba82a
            // ba82a mov rax,rbx
            rax = rbx;
            // ba82d add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba831 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba832 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba834 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ba836 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba838 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba83a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba83b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xba83c => {
            // block ba83c
            // ba83c mov rbx,rax
            rbx = rax;
            // ba83f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba843 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba848 mov rdi,rbx
            rdi = rbx;
            // ba84b call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba850 size=206
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba850<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba850;
    loop {
        match pc {
        0xba850 => {
            // block ba850
            // ba850 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba851 mov rbp,rsp
            rbp = rsp;
            // ba854 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba856 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba858 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ba85a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba85c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba85d sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba861 mov r14,rsi
            r14 = rsi;
            // ba864 mov rbx,rdi
            rbx = rdi;
            // ba867 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // ba86a add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // ba86e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // ba871 je 0xba8f8
            if fl.e() { pc = 0xba8f8; continue; }
            pc = 0xba877;
        }
        0xba877 => {
            // block ba877
            // ba877 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba87b jmp 0xba894
            pc = 0xba894; continue;
        }
        0xba87d => {
            // block ba87d
            // ba87d mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ba882 mov rdi,rbx
            rdi = rbx;
            // ba885 lea rsi,[rip+0x6fc11] # 0x12a49d
            rsi = m.env.data_va(257130);
            // ba88c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba891 mov r13,r12
            r13 = r12;
            pc = 0xba894;
        }
        0xba894 => {
            // block ba894
            // ba894 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ba898 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // ba89d mov rdi,r15
            rdi = r15;
            // ba8a0 call 0x5d8db
            { let r = { let t = m.env.code_va(0x5d8db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8a5 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // ba8a9 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // ba8ac jne 0xba8b7
            if fl.ne() { pc = 0xba8b7; continue; }
            pc = 0xba8ae;
        }
        0xba8ae => {
            // block ba8ae
            // ba8ae shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // ba8b1 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // ba8b5 jmp 0xba8bf
            pc = 0xba8bf; continue;
        }
        0xba8b7 => {
            // block ba8b7
            // ba8b7 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // ba8bb mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xba8bf;
        }
        0xba8bf => {
            // block ba8bf
            // ba8bf mov rdi,rbx
            rdi = rbx;
            // ba8c2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8c7 mov rdi,r15
            rdi = r15;
            // ba8ca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba8cf mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // ba8d3 test rax,rax
            fl = logic::<64>(rax & rax);
            // ba8d6 je 0xba8e6
            if fl.e() { pc = 0xba8e6; continue; }
            pc = 0xba8d8;
        }
        0xba8d8 => {
            // block ba8d8
            // ba8d8 mov r12,rax
            r12 = rax;
            // ba8db mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // ba8df test rax,rax
            fl = logic::<64>(rax & rax);
            // ba8e2 jne 0xba8d8
            if fl.ne() { pc = 0xba8d8; continue; }
            pc = 0xba8e4;
        }
        0xba8e4 => {
            // block ba8e4
            // ba8e4 jmp 0xba8f3
            pc = 0xba8f3; continue;
        }
        0xba8e6 => {
            // block ba8e6
            // ba8e6 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // ba8ea cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // ba8ee mov r13,r12
            r13 = r12;
            // ba8f1 jne 0xba8e6
            if fl.ne() { pc = 0xba8e6; continue; }
            pc = 0xba8f3;
        }
        0xba8f3 => {
            // block ba8f3
            // ba8f3 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // ba8f6 jne 0xba87d
            if fl.ne() { pc = 0xba87d; continue; }
            pc = 0xba8f8;
        }
        0xba8f8 => {
            // block ba8f8
            // ba8f8 mov rax,rbx
            rax = rbx;
            // ba8fb add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // ba8ff pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba900 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba902 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ba904 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba906 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba908 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba909 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xba90a => {
            // block ba90a
            // ba90a mov rbx,rax
            rbx = rax;
            // ba90d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ba911 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba916 mov rdi,rbx
            rdi = rbx;
            // ba919 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
