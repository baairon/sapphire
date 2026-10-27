//! Mechanically lifted functions (scripts/lift_run.py), module g018. Each one is checked against the original
//! in the world harness (tests/world_g018.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            if fl.e() { pc = 0x15294; continue; }
            pc = 0x15182;
        }
        0x15182 => {
            // block 15182
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15182 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 15185 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15188 mov r13d,eax
            r13 = ((rax as u32 as u64)) as u32 as u64;
            // 1518b cmp QWORD PTR [r14+0x78],0x0
            fl = sub::<64>(m.r64(r14.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15190 je 0x151d1
            if fl.e() { pc = 0x151d1; continue; }
            pc = 0x15192;
        }
        0x15192 => {
            // block 15192
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15192 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 15195 je 0x1519c
            if fl.e() { pc = 0x1519c; continue; }
            pc = 0x15197;
        }
        0x15197 => {
            // block 15197
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 15197 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15197 test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 1519a jle 0x151d1
            if fl.le() { pc = 0x151d1; continue; }
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
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151a5 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151a7 jne 0x151d1
            if fl.ne() { pc = 0x151d1; continue; }
            pc = 0x151a9;
        }
        0x151a9 => {
            // block 151a9
            #[cfg(feature = "trace")] eprintln!("f_00015158 pc 151a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 151a9 cmp r15d,0x3
            fl = sub::<32>((r15 as u32 as u64), 0x3u64).1;
            // 151ad jae 0x151d1
            if fl.ae() { pc = 0x151d1; continue; }
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
            if fl.le() { pc = 0x151c1; continue; }
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
            { let r = { let t = m.env.stub_va(0x9a9); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 151c9 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 151cb je 0x1526d
            if fl.e() { pc = 0x1526d; continue; }
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
            { let r = { let t = m.env.stub_va(0x9af); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1529e mov rbx,rax
            rbx = rax;
            // 152a1 mov rdi,rbx
            rdi = rbx;
            // 152a4 call 0xec122
            { let r = { let t = m.env.stub_va(0x8ef); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0xe8730; continue; }
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
            if fl.b() { pc = 0xe87bc; continue; }
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
            if fl.l() { pc = 0xe87bc; continue; }
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
            if fl.e() { pc = 0xe87bc; continue; }
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
            if fl.ne() { pc = 0xe879d; continue; }
            pc = 0xe8787;
        }
        0xe8787 => {
            // block e8787
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8787 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8787 cmp rax,0x4
            fl = sub::<64>(rax, 0x4u64).1;
            // e878b je 0xe87a7
            if fl.e() { pc = 0xe87a7; continue; }
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
            if fl.e() { pc = 0xe8787; continue; }
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
            if fl.ne() { pc = 0xe877a; continue; }
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
            if fl.e() { pc = 0xe87bc; continue; }
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
            if fl.ne() { pc = 0xe87ca; continue; }
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
            if fl.b() { pc = 0xe883f; continue; }
            pc = 0xe87e5;
        }
        0xe87e5 => {
            // block e87e5
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e87e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e87e5 cmp r13,0x4
            fl = sub::<64>(r13, 0x4u64).1;
            // e87e9 jl 0xe883f
            if fl.l() { pc = 0xe883f; continue; }
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
            if fl.e() { pc = 0xe883f; continue; }
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
            if fl.ne() { pc = 0xe882a; continue; }
            pc = 0xe8814;
        }
        0xe8814 => {
            // block e8814
            #[cfg(feature = "trace")] eprintln!("f_000e86f8 pc e8814 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e8814 cmp rdi,0x4
            fl = sub::<64>(rdi, 0x4u64).1;
            // e8818 je 0xe8834
            if fl.e() { pc = 0xe8834; continue; }
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
            if fl.e() { pc = 0xe8814; continue; }
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
            if fl.ne() { pc = 0xe8807; continue; }
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
            if fl.e() { pc = 0xe883f; continue; }
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
            { let r = { let t = m.env.stub_va(0x7f9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x10f3d; continue; }
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
            if fl.be() { pc = 0x10e6f; continue; }
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
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x10f06; continue; }
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
            if fl.ne() { pc = 0x10ece; continue; }
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
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0x6dbe size=376
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006dbe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6dbe;
    loop {
        match pc {
        0x6dbe => {
            // block 6dbe
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6dbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6dbe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6dbf mov rbp,rsp
            rbp = rsp;
            // 6dc2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6dc4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6dc6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6dc8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6dca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6dcb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6dcc mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 6dce mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 6dd1 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // 6dd4 mov r12,rdi
            r12 = rdi;
            // 6dd7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6ddb mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6ddd call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6de3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6de5 jne 0x6e8e
            if fl.ne() { pc = 0x6e8e; continue; }
            pc = 0x6deb;
        }
        0x6deb => {
            // block 6deb
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6deb test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 6dee mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6df2 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 6df6 movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // 6df9 je 0x6ea3
            if fl.e() { pc = 0x6ea3; continue; }
            pc = 0x6dff;
        }
        0x6dff => {
            // block 6dff
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6dff lea rcx,[rip+0xf85ea] # 0xff3f0
            rcx = m.env.data_va(80829);
            // 6e06 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e09 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6e0c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6e11 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6e14 mov rdi,r12
            rdi = r12;
            // 6e17 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 6e1a call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e1c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e1e je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6e20;
        }
        0x6e20 => {
            // block 6e20
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e20 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e24 lea rcx,[rip+0xf85f5] # 0xff420
            rcx = m.env.data_va(80877);
            // 6e2b mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e2e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6e31 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6e36 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6e39 mov rdi,r12
            rdi = r12;
            // 6e3c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 6e3f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e42 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e44 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6e46;
        }
        0x6e46 => {
            // block 6e46
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e46 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e4a lea rcx,[rip+0xf856f] # 0xff3c0
            rcx = m.env.data_va(80781);
            // 6e51 mov ebx,DWORD PTR [rcx+rbx*4]
            rbx = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e54 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6e59 mov ecx,0x8000000
            rcx = (0x8000000u64) as u32 as u64;
            // 6e5e mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 6e64 mov rdi,r12
            rdi = r12;
            // 6e67 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6e69 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e6c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e6e je 0x6e8e
            if fl.e() { pc = 0x6e8e; continue; }
            pc = 0x6e70;
        }
        0x6e70 => {
            // block 6e70
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e70 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e74 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6e79 mov ecx,0x4000000
            rcx = (0x4000000u64) as u32 as u64;
            // 6e7e mov r8d,0x1a
            r8 = (0x1au64) as u32 as u64;
            // 6e84 mov rdi,r12
            rdi = r12;
            // 6e87 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6e89 jmp 0x6f2b
            pc = 0x6f2b; continue;
        }
        0x6e8e => {
            // block 6e8e
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e8e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x6e91;
        }
        0x6e91 => {
            // block 6e91
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e91 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 6e94 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6e98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6e99 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6e9b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6e9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6e9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6ea1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6ea2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6ea3 => {
            // block 6ea3
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6ea3 lea rcx,[rip+0xf8516] # 0xff3c0
            rcx = m.env.data_va(80781);
            // 6eaa mov r15d,DWORD PTR [rcx+rbx*4]
            r15 = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6eae xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6eb1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6eb3 mov ecx,0x8000000
            rcx = (0x8000000u64) as u32 as u64;
            // 6eb8 mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 6ebe mov rdi,r12
            rdi = r12;
            // 6ec1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 6ec4 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6ec6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6ec8 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6eca;
        }
        0x6eca => {
            // block 6eca
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6eca mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6ece xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6ed1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6ed3 mov ecx,0x4000000
            rcx = (0x4000000u64) as u32 as u64;
            // 6ed8 mov r8d,0x1a
            r8 = (0x1au64) as u32 as u64;
            // 6ede mov rdi,r12
            rdi = r12;
            // 6ee1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 6ee4 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6ee7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6ee9 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6eeb;
        }
        0x6eeb => {
            // block 6eeb
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6eeb mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6eef lea rcx,[rip+0xf84fa] # 0xff3f0
            rcx = m.env.data_va(80829);
            // 6ef6 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6ef9 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6efc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6efe mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6f03 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6f06 mov rdi,r12
            rdi = r12;
            // 6f09 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f0c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6f0e je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6f10;
        }
        0x6f10 => {
            // block 6f10
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6f10 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6f14 lea rcx,[rip+0xf8505] # 0xff420
            rcx = m.env.data_va(80877);
            // 6f1b mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6f1e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6f20 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6f25 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6f28 mov rdi,r12
            rdi = r12;
            pc = 0x6f2b;
        }
        0x6f2b => {
            // block 6f2b
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6f2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6f2b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f2e mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6f31 jmp 0x6e91
            pc = 0x6e91; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d410 size=384
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d410;
    loop {
        match pc {
        0x2d410 => {
            // block 2d410
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d411 mov rbp,rsp
            rbp = rsp;
            // 2d414 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d416 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d418 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 2d41a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d41c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d41d sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d421 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 2d424 mov rbx,rdi
            rbx = rdi;
            // 2d427 xor r12d,r12d
            { let r = (r12 as u32 as u64) ^ (r12 as u32 as u64); let f = logic::<32>(r);
              r12 = (r) as u32 as u64;
              fl = f; }
            // 2d42a cmp r14d,0xffffffff
            fl = sub::<32>((r14 as u32 as u64), 0xffffffffu64).1;
            // 2d42e je 0x2d546
            if fl.e() { pc = 0x2d546; continue; }
            pc = 0x2d434;
        }
        0x2d434 => {
            // block 2d434
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d434 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d434 mov r15,QWORD PTR [rbx+0x18]
            r15 = m.r64(rbx.wrapping_add(0x18)) as u64;
            // 2d438 mov r13,QWORD PTR [rbx+0x30]
            r13 = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 2d43c sub r15,QWORD PTR [rbx+0x10]
            { let (r, f) = sub::<64>(r15, m.r64(rbx.wrapping_add(0x10)) as u64);
              r15 = r;
              fl = f; }
            // 2d440 mov rax,QWORD PTR [rbx+0x38]
            rax = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 2d444 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2d447 je 0x2d45a
            if fl.e() { pc = 0x2d45a; continue; }
            pc = 0x2d449;
        }
        0x2d449 => {
            // block 2d449
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d449 lea rdx,[rbx+0x58]
            rdx = rbx.wrapping_add(0x58);
            // 2d44d mov rdi,QWORD PTR [rbx+0x58]
            rdi = m.r64(rbx.wrapping_add(0x58)) as u64;
            // 2d451 lea rcx,[rbx+0x60]
            rcx = rbx.wrapping_add(0x60);
            // 2d455 jmp 0x2d4f5
            pc = 0x2d4f5; continue;
        }
        0x2d45a => {
            // block 2d45a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d45a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d45a mov r12d,0xffffffff
            r12 = (0xffffffffu64) as u32 as u64;
            // 2d460 test BYTE PTR [rbx+0x60],0x10
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x60)) as u64 & 0x10u64);
            // 2d464 je 0x2d546
            if fl.e() { pc = 0x2d546; continue; }
            pc = 0x2d46a;
        }
        0x2d46a => {
            // block 2d46a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d46a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d46a mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 2d46e mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 2d472 mov rax,QWORD PTR [rbx+0x58]
            rax = m.r64(rbx.wrapping_add(0x58)) as u64;
            // 2d476 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 2d47a lea rdi,[rbx+0x40]
            rdi = rbx.wrapping_add(0x40);
            // 2d47e mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 2d482 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2d484 call 0xec020
            { let r = { let t = m.env.stub_va(0x7ed); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d489 mov esi,0x16
            rsi = (0x16u64) as u32 as u64;
            // 2d48e mov rdi,QWORD PTR [rbp-0x38]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2d492 test BYTE PTR [rdi],0x1
            fl = logic::<8>(m.r8(rdi) as u64 & 0x1u64);
            // 2d495 je 0x2d4a1
            if fl.e() { pc = 0x2d4a1; continue; }
            pc = 0x2d497;
        }
        0x2d497 => {
            // block 2d497
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d497 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d497 mov rsi,QWORD PTR [rdi]
            rsi = m.r64(rdi) as u64;
            // 2d49a and rsi,0xfffffffffffffffe
            { let r = rsi & 0xffff_ffff_ffff_fffeu64; let f = logic::<64>(r);
              rsi = r;
              fl = f; }
            // 2d49e dec rsi
            { let (r, f) = dec::<64>(rsi, fl); rsi = r; fl = f; }
            pc = 0x2d4a1;
        }
        0x2d4a1 => {
            // block 2d4a1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4a1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2d4a3 call 0xec008
            { let r = { let t = m.env.stub_va(0x7d5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d4a8 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 2d4ac sub r13,rax
            { let (r, f) = sub::<64>(r13, rax);
              r13 = r;
              fl = f; }
            // 2d4af mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 2d4b3 sub rdi,rax
            { let (r, f) = sub::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 2d4b6 mov rsi,QWORD PTR [rbp-0x38]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 2d4ba movzx eax,BYTE PTR [rsi]
            rax = (m.r8(rsi) as u64) as u32 as u64;
            // 2d4bd test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 2d4bf jne 0x2d4c9
            if fl.ne() { pc = 0x2d4c9; continue; }
            pc = 0x2d4c1;
        }
        0x2d4c1 => {
            // block 2d4c1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4c1 inc rsi
            { let (r, f) = inc::<64>(rsi, fl); rsi = r; fl = f; }
            // 2d4c4 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 2d4c7 jmp 0x2d4d1
            pc = 0x2d4d1; continue;
        }
        0x2d4c9 => {
            // block 2d4c9
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4c9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4c9 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 2d4cd mov rsi,QWORD PTR [rbx+0x50]
            rsi = m.r64(rbx.wrapping_add(0x50)) as u64;
            pc = 0x2d4d1;
        }
        0x2d4d1 => {
            // block 2d4d1
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4d1 lea rcx,[rbx+0x60]
            rcx = rbx.wrapping_add(0x60);
            // 2d4d5 lea rdx,[rbx+0x58]
            rdx = rbx.wrapping_add(0x58);
            // 2d4d9 add rax,rsi
            { let (r, f) = add::<64>(rax, rsi);
              rax = r;
              fl = f; }
            // 2d4dc mov QWORD PTR [rbx+0x28],rsi
            m.w64(rbx.wrapping_add(0x28), (rsi) as u64);
            // 2d4e0 mov QWORD PTR [rbx+0x38],rax
            m.w64(rbx.wrapping_add(0x38), (rax) as u64);
            // 2d4e4 movsxd r13,r13d
            r13 = sx::<32>((r13 as u32 as u64)) as u64;
            // 2d4e7 add r13,rsi
            { let (r, f) = add::<64>(r13, rsi);
              r13 = r;
              fl = f; }
            // 2d4ea mov QWORD PTR [rbx+0x30],r13
            m.w64(rbx.wrapping_add(0x30), (r13) as u64);
            // 2d4ee add rdi,rsi
            { let (r, f) = add::<64>(rdi, rsi);
              rdi = r;
              fl = f; }
            // 2d4f1 mov QWORD PTR [rbx+0x58],rdi
            m.w64(rbx.wrapping_add(0x58), (rdi) as u64);
            pc = 0x2d4f5;
        }
        0x2d4f5 => {
            // block 2d4f5
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d4f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d4f5 lea rsi,[r13+0x1]
            rsi = r13.wrapping_add(0x1);
            // 2d4f9 mov QWORD PTR [rbp-0x30],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rsi) as u64);
            // 2d4fd cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 2d500 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 2d504 cmovb rdi,rdx
            if fl.b() { rdi = rdx; }
            // 2d508 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 2d50b mov QWORD PTR [rdx],rdi
            m.w64(rdx, (rdi) as u64);
            // 2d50e test BYTE PTR [rcx],0x8
            fl = logic::<8>(m.r8(rcx) as u64 & 0x8u64);
            // 2d511 je 0x2d535
            if fl.e() { pc = 0x2d535; continue; }
            pc = 0x2d513;
        }
        0x2d513 => {
            // block 2d513
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d513 test BYTE PTR [rbx+0x40],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x40)) as u64 & 0x1u64);
            // 2d517 jne 0x2d522
            if fl.ne() { pc = 0x2d522; continue; }
            pc = 0x2d519;
        }
        0x2d519 => {
            // block 2d519
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d519 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d519 lea rcx,[rbx+0x40]
            rcx = rbx.wrapping_add(0x40);
            // 2d51d inc rcx
            { let (r, f) = inc::<64>(rcx, fl); rcx = r; fl = f; }
            // 2d520 jmp 0x2d526
            pc = 0x2d526; continue;
        }
        0x2d522 => {
            // block 2d522
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d522 mov rcx,QWORD PTR [rbx+0x50]
            rcx = m.r64(rbx.wrapping_add(0x50)) as u64;
            pc = 0x2d526;
        }
        0x2d526 => {
            // block 2d526
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d526 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d526 add r15,rcx
            { let (r, f) = add::<64>(r15, rcx);
              r15 = r;
              fl = f; }
            // 2d529 mov QWORD PTR [rbx+0x10],rcx
            m.w64(rbx.wrapping_add(0x10), (rcx) as u64);
            // 2d52d mov QWORD PTR [rbx+0x18],r15
            m.w64(rbx.wrapping_add(0x18), (r15) as u64);
            // 2d531 mov QWORD PTR [rbx+0x20],rdi
            m.w64(rbx.wrapping_add(0x20), (rdi) as u64);
            pc = 0x2d535;
        }
        0x2d535 => {
            // block 2d535
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d535 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d535 cmp r13,rax
            fl = sub::<64>(r13, rax).1;
            // 2d538 je 0x2d558
            if fl.e() { pc = 0x2d558; continue; }
            pc = 0x2d53a;
        }
        0x2d53a => {
            // block 2d53a
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d53a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d53a mov QWORD PTR [rbx+0x30],rsi
            m.w64(rbx.wrapping_add(0x30), (rsi) as u64);
            // 2d53e mov BYTE PTR [r13+0x0],r14b
            m.w8(r13, ((r14 as u8 as u64)) as u8);
            // 2d542 movzx r12d,r14b
            r12 = ((r14 as u8 as u64)) as u32 as u64;
            pc = 0x2d546;
        }
        0x2d546 => {
            // block 2d546
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d546 mov eax,r12d
            rax = ((r12 as u32 as u64)) as u32 as u64;
            // 2d549 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d54d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d54e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d550 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d552 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d554 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d556 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d557 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2d558 => {
            // block 2d558
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d558 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d558 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2d55b mov rax,QWORD PTR [rax+0x68]
            rax = m.r64(rax.wrapping_add(0x68)) as u64;
            // 2d55f movzx esi,r14b
            rsi = ((r14 as u8 as u64)) as u32 as u64;
            // 2d563 mov rdi,rbx
            rdi = rbx;
            // 2d566 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // 2d56a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d56b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d56d pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 2d56f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d571 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d573 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d574 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; return Ret::from_call(r); }
        }
        0x2d576 => {
            // block 2d576
            #[cfg(feature = "trace")] eprintln!("f_0002d410 pc 2d576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d576 mov rdi,rax
            rdi = rax;
            // 2d579 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d57e call 0xec15e
            { let r = { let t = m.env.stub_va(0x92b); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d583 jmp 0x2d546
            pc = 0x2d546; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1582c size=386
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001582c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1582c;
    loop {
        match pc {
        0x1582c => {
            // block 1582c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1582c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1582c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1582d mov rbp,rsp
            rbp = rsp;
            // 15830 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 15832 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15834 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 15836 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 15838 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15839 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 1583d mov rbx,QWORD PTR [rdi+0x8]
            rbx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 15841 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 15844 je 0x15980
            if fl.e() { pc = 0x15980; continue; }
            pc = 0x1584a;
        }
        0x1584a => {
            // block 1584a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1584a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1584a mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1584e movzx eax,BYTE PTR [rdx]
            rax = (m.r8(rdx) as u64) as u32 as u64;
            // 15851 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 15854 and r15b,0x1
            { let r = (r15 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r15 = (r15 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15858 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1585b mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1585f mov rax,QWORD PTR [rdx+0x8]
            rax = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 15863 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 15867 mov rax,QWORD PTR [rdx+0x10]
            rax = m.r64(rdx.wrapping_add(0x10)) as u64;
            // 1586b mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 1586f lea rax,[rdx+0x1]
            rax = rdx.wrapping_add(0x1);
            // 15873 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 15877 jmp 0x1587c
            pc = 0x1587c; continue;
        }
        0x15879 => {
            // block 15879
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15879 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15879 mov rbx,rax
            rbx = rax;
            pc = 0x1587c;
        }
        0x1587c => {
            // block 1587c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1587c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1587c test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 1587f mov r13,QWORD PTR [rbp-0x38]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 15883 cmove r13,QWORD PTR [rbp-0x30]
            if fl.e() { r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15888 movzx eax,BYTE PTR [rbx+0x20]
            rax = (m.r8(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // 1588c mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 1588f and r12b,0x1
            { let r = (r12 as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              r12 = (r12 & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 15893 jne 0x1589d
            if fl.ne() { pc = 0x1589d; continue; }
            pc = 0x15895;
        }
        0x15895 => {
            // block 15895
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15895 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15895 mov r14,rax
            r14 = rax;
            // 15898 shr r14,1
            { let (r, f) = shr::<64>(r14, 0x1u64, fl); r14 = r; fl = f; }
            // 1589b jmp 0x158a1
            pc = 0x158a1; continue;
        }
        0x1589d => {
            // block 1589d
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1589d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1589d mov r14,QWORD PTR [rbx+0x28]
            r14 = m.r64(rbx.wrapping_add(0x28)) as u64;
            pc = 0x158a1;
        }
        0x158a1 => {
            // block 158a1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158a1 lea rdx,[rbx+0x20]
            rdx = rbx.wrapping_add(0x20);
            // 158a5 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 158a8 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 158ac cmove rdi,QWORD PTR [rbp-0x50]
            if fl.e() { rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 158b1 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 158b4 jne 0x158c0
            if fl.ne() { pc = 0x158c0; continue; }
            pc = 0x158b6;
        }
        0x158b6 => {
            // block 158b6
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158b6 lea rsi,[rdx+0x1]
            rsi = rdx.wrapping_add(0x1);
            // 158ba mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158be jmp 0x158c8
            pc = 0x158c8; continue;
        }
        0x158c0 => {
            // block 158c0
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c0 mov QWORD PTR [rbp-0x40],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdx) as u64);
            // 158c4 mov rsi,QWORD PTR [rbx+0x30]
            rsi = m.r64(rbx.wrapping_add(0x30)) as u64;
            pc = 0x158c8;
        }
        0x158c8 => {
            // block 158c8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158c8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158c8 cmp r14,r13
            fl = sub::<64>(r14, r13).1;
            // 158cb mov rdx,r13
            rdx = r13;
            // 158ce cmovb rdx,r14
            if fl.b() { rdx = r14; }
            // 158d2 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 158d5 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158d7;
        }
        0x158d7 => {
            // block 158d7
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158d7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158d7 mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 158db call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 158e0 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 158e2 mov rax,QWORD PTR [rbp-0x58]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 158e6 je 0x158ec
            if fl.e() { pc = 0x158ec; continue; }
            pc = 0x158e8;
        }
        0x158e8 => {
            // block 158e8
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158e8 jns 0x158fe
            if fl.ns() { pc = 0x158fe; continue; }
            pc = 0x158ea;
        }
        0x158ea => {
            // block 158ea
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ea jmp 0x158f1
            pc = 0x158f1; continue;
        }
        0x158ec => {
            // block 158ec
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158ec cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 158ef jae 0x158fe
            if fl.ae() { pc = 0x158fe; continue; }
            pc = 0x158f1;
        }
        0x158f1 => {
            // block 158f1
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 158f4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 158f7 jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x158f9;
        }
        0x158f9 => {
            // block 158f9
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158f9 jmp 0x15995
            pc = 0x15995; continue;
        }
        0x158fe => {
            // block 158fe
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 158fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 158fe test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15901 jne 0x1590b
            if fl.ne() { pc = 0x1590b; continue; }
            pc = 0x15903;
        }
        0x15903 => {
            // block 15903
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15903 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15903 mov r14,rbx
            r14 = rbx;
            // 15906 shr rax,1
            { let (r, f) = shr::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 15909 jmp 0x15912
            pc = 0x15912; continue;
        }
        0x1590b => {
            // block 1590b
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1590b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1590b mov rax,QWORD PTR [rbx+0x28]
            rax = m.r64(rbx.wrapping_add(0x28)) as u64;
            // 1590f mov r14,rbx
            r14 = rbx;
            pc = 0x15912;
        }
        0x15912 => {
            // block 15912
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15912 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15912 mov rdi,QWORD PTR [rbp-0x40]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 15916 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15919 mov rbx,QWORD PTR [rbp-0x38]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 1591d cmove rbx,QWORD PTR [rbp-0x30]
            if fl.e() { rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64; }
            // 15922 test r12b,r12b
            fl = logic::<8>((r12 as u8 as u64) & (r12 as u8 as u64));
            // 15925 jne 0x1592c
            if fl.ne() { pc = 0x1592c; continue; }
            pc = 0x15927;
        }
        0x15927 => {
            // block 15927
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15927 inc rdi
            { let (r, f) = inc::<64>(rdi, fl); rdi = r; fl = f; }
            // 1592a jmp 0x15930
            pc = 0x15930; continue;
        }
        0x1592c => {
            // block 1592c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1592c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1592c mov rdi,QWORD PTR [r14+0x30]
            rdi = m.r64(r14.wrapping_add(0x30)) as u64;
            pc = 0x15930;
        }
        0x15930 => {
            // block 15930
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15930 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 15933 mov rdx,rax
            rdx = rax;
            // 15936 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 1593a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 1593d je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1593f;
        }
        0x1593f => {
            // block 1593f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1593f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1593f test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 15942 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 15946 cmove rsi,QWORD PTR [rbp-0x50]
            if fl.e() { rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64; }
            // 1594b mov r12,rax
            r12 = rax;
            // 1594e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15953 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15955 mov rax,r12
            rax = r12;
            // 15958 je 0x1595e
            if fl.e() { pc = 0x1595e; continue; }
            pc = 0x1595a;
        }
        0x1595a => {
            // block 1595a
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595a js 0x15963
            if fl.s() { pc = 0x15963; continue; }
            pc = 0x1595c;
        }
        0x1595c => {
            // block 1595c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595c jmp 0x1598c
            pc = 0x1598c; continue;
        }
        0x1595e => {
            // block 1595e
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1595e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1595e cmp rax,rbx
            fl = sub::<64>(rax, rbx).1;
            // 15961 jae 0x1598c
            if fl.ae() { pc = 0x1598c; continue; }
            pc = 0x15963;
        }
        0x15963 => {
            // block 15963
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15963 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15963 mov rax,QWORD PTR [r14+0x8]
            rax = m.r64(r14.wrapping_add(0x8)) as u64;
            // 15967 test rax,rax
            fl = logic::<64>(rax & rax);
            // 1596a jne 0x15879
            if fl.ne() { pc = 0x15879; continue; }
            pc = 0x15970;
        }
        0x15970 => {
            // block 15970
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15970 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15974 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15977 add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // 1597b mov rax,r14
            rax = r14;
            // 1597e jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15980 => {
            // block 15980
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15980 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 15984 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 15987 mov rax,rdi
            rax = rdi;
            // 1598a jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x1598c => {
            // block 1598c
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1598c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1598c mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15990 mov QWORD PTR [rax],r14
            m.w64(rax, (r14) as u64);
            // 15993 jmp 0x1599f
            pc = 0x1599f; continue;
        }
        0x15995 => {
            // block 15995
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 15995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15995 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 15999 mov QWORD PTR [rax],rbx
            m.w64(rax, (rbx) as u64);
            // 1599c mov rax,rbx
            rax = rbx;
            pc = 0x1599f;
        }
        0x1599f => {
            // block 1599f
            #[cfg(feature = "trace")] eprintln!("f_0001582c pc 1599f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1599f add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 159a3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 159a4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 159a6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 159a8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 159aa pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 159ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 159ad ret
            return Ret { rax, rdx, xmm0: x0 };
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
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x811); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x6e5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x8b9); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ba lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92be mov rsi,QWORD PTR [rip+0x56ecb] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // e92c5 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92ca mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // e92cd mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // e92d1 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // e92d6 mov rdi,rax
            rdi = rax;
            // e92d9 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92db mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e92de lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // e92e2 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e92e7 movsx edx,r12b
            rdx = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // e92eb mov rdi,r13
            rdi = r13;
            // e92ee mov rsi,rbx
            rsi = rbx;
            // e92f1 call 0xebf00
            { let r = { let t = m.env.stub_va(0x6cd); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe9379;
        }
        0xe9379 => {
            // block e9379
            #[cfg(feature = "trace")] eprintln!("f_000e9208 pc e9379 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9379 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e937d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9382 mov rdi,rbx
            rdi = rbx;
            // e9385 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e938a jmp 0xe9323
            pc = 0xe9323; continue;
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.a() { pc = 0x64656; continue; }
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
            if fl.be() { pc = 0x6465b; continue; }
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

/// parity: P2v @0xfe38 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fe38<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xfe38;
    loop {
        match pc {
        0xfe38 => {
            // block fe38
            #[cfg(feature = "trace")] eprintln!("f_0000fe38 pc fe38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fe38 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fe39 mov rbp,rsp
            rbp = rsp;
            // fe3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fe3e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fe3f sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // fe43 mov rbx,rsi
            rbx = rsi;
            // fe46 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // fe49 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // fe4d mov rsi,r14
            rsi = r14;
            // fe50 call QWORD PTR [rax+0x1d00]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d00)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe56 mov rdi,r14
            rdi = r14;
            // fe59 mov rsi,rbx
            rsi = rbx;
            // fe5c call 0x645eb
            { let r = { let t = m.env.code_va(0x645eb); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe61 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // fe65 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fe66 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fe68 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fe69 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x15c78 size=408
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00015c78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x15c78;
    loop {
        match pc {
        0x15c78 => {
            // block 15c78
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15c78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15c78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 15c79 mov rbp,rsp
            rbp = rsp;
            // 15c7c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 15c7e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 15c7f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 15c82 mov rbx,rdi
            rbx = rdi;
            // 15c85 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15c87 cmp QWORD PTR [rbx+0x78],0x0
            fl = sub::<64>(m.r64(rbx.wrapping_add(0x78)) as u64, 0x0u64).1;
            // 15c8c jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15c92;
        }
        0x15c92 => {
            // block 15c92
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15c92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15c92 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 15c95 and edx,0xfffffffd
            { let r = (rdx as u32 as u64) & 0xfffffffdu64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 15c98 lea rcx,[rip+0x1145c5] # 0x12a264
            rcx = m.env.data_va(256561);
            // 15c9f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15ca1 cmp edx,0x2f
            fl = sub::<32>((rdx as u32 as u64), 0x2fu64).1;
            // 15ca4 jg 0x15cca
            if fl.g() { pc = 0x15cca; continue; }
            pc = 0x15ca6;
        }
        0x15ca6 => {
            // block 15ca6
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ca6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ca6 dec edx
            { let (r, f) = dec::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // 15ca8 cmp edx,0x1c
            fl = sub::<32>((rdx as u32 as u64), 0x1cu64).1;
            // 15cab ja 0x15d94
            if fl.a() { pc = 0x15d94; continue; }
            pc = 0x15cb1;
        }
        0x15cb1 => {
            // block 15cb1
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cb1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cb1 lea rdi,[rip+0xe4] # 0x15d9c
            rdi = m.env.code_va(0x15d9c);
            // 15cb8 movsxd rdx,DWORD PTR [rdi+rdx*4]
            rdx = sx::<32>(m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64;
            // 15cbc add rdx,rdi
            { let (r, f) = add::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 15cbf jmp rdx
            pc = (rdx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x15cc1 => {
            // block 15cc1
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cc1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cc1 lea rcx,[rip+0x11459e] # 0x12a266
            rcx = m.env.data_va(256563);
            // 15cc8 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15cca => {
            // block 15cca
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cca cmp edx,0x37
            fl = sub::<32>((rdx as u32 as u64), 0x37u64).1;
            // 15ccd jg 0x15ce6
            if fl.g() { pc = 0x15ce6; continue; }
            pc = 0x15ccf;
        }
        0x15ccf => {
            // block 15ccf
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ccf cmp edx,0x30
            fl = sub::<32>((rdx as u32 as u64), 0x30u64).1;
            // 15cd2 je 0x15d43
            if fl.e() { pc = 0x15d43; continue; }
            pc = 0x15cd4;
        }
        0x15cd4 => {
            // block 15cd4
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cd4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cd4 cmp edx,0x34
            fl = sub::<32>((rdx as u32 as u64), 0x34u64).1;
            // 15cd7 jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15cdd;
        }
        0x15cdd => {
            // block 15cdd
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cdd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cdd lea rcx,[rip+0x11458f] # 0x12a273
            rcx = m.env.data_va(256576);
            // 15ce4 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15ce6 => {
            // block 15ce6
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ce6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ce6 cmp edx,0x38
            fl = sub::<32>((rdx as u32 as u64), 0x38u64).1;
            // 15ce9 jne 0x15cf4
            if fl.ne() { pc = 0x15cf4; continue; }
            pc = 0x15ceb;
        }
        0x15ceb => {
            // block 15ceb
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15ceb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15ceb lea rcx,[rip+0x11457b] # 0x12a26d
            rcx = m.env.data_va(256570);
            // 15cf2 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15cf4 => {
            // block 15cf4
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cf4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cf4 cmp edx,0x3c
            fl = sub::<32>((rdx as u32 as u64), 0x3cu64).1;
            // 15cf7 jne 0x15d94
            if fl.ne() { pc = 0x15d94; continue; }
            pc = 0x15cfd;
        }
        0x15cfd => {
            // block 15cfd
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15cfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15cfd lea rcx,[rip+0x11457c] # 0x12a280
            rcx = m.env.data_va(256589);
            // 15d04 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d06 => {
            // block 15d06
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d06 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d06 lea rcx,[rip+0x114569] # 0x12a276
            rcx = m.env.data_va(256579);
            // 15d0d jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d0f => {
            // block 15d0f
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d0f lea rcx,[rip+0x11455a] # 0x12a270
            rcx = m.env.data_va(256573);
            // 15d16 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d18 => {
            // block 15d18
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d18 lea rcx,[rip+0x114565] # 0x12a284
            rcx = m.env.data_va(256593);
            // 15d1f jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d21 => {
            // block 15d21
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d21 lea rcx,[rip+0x114540] # 0x12a268
            rcx = m.env.data_va(256565);
            // 15d28 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d2a => {
            // block 15d2a
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d2a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d2a lea rcx,[rip+0x114548] # 0x12a279
            rcx = m.env.data_va(256582);
            // 15d31 jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d33 => {
            // block 15d33
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d33 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d33 lea rcx,[rip+0x114530] # 0x12a26a
            rcx = m.env.data_va(256567);
            // 15d3a jmp 0x15d43
            pc = 0x15d43; continue;
        }
        0x15d3c => {
            // block 15d3c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d3c lea rcx,[rip+0x114539] # 0x12a27c
            rcx = m.env.data_va(256585);
            pc = 0x15d43;
        }
        0x15d43 => {
            // block 15d43
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d43 mov rdi,rsi
            rdi = rsi;
            // 15d46 mov rsi,rcx
            rsi = rcx;
            // 15d49 call 0xec1c4
            { let r = { let t = m.env.stub_va(0x991); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d4e mov rcx,rax
            rcx = rax;
            // 15d51 mov QWORD PTR [rbx+0x78],rcx
            m.w64(rbx.wrapping_add(0x78), (rcx) as u64);
            // 15d55 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15d57 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // 15d5a je 0x15d94
            if fl.e() { pc = 0x15d94; continue; }
            pc = 0x15d5c;
        }
        0x15d5c => {
            // block 15d5c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d5c mov DWORD PTR [rbx+0x188],r14d
            m.w32(rbx.wrapping_add(0x188), ((r14 as u32 as u64)) as u32);
            // 15d63 test r14b,0x2
            fl = logic::<8>((r14 as u8 as u64) & 0x2u64);
            // 15d67 je 0x15d91
            if fl.e() { pc = 0x15d91; continue; }
            pc = 0x15d69;
        }
        0x15d69 => {
            // block 15d69
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d69 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 15d6b mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // 15d70 mov rdi,rcx
            rdi = rcx;
            // 15d73 call 0xec1d6
            { let r = { let t = m.env.stub_va(0x9a3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d78 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 15d7a je 0x15d91
            if fl.e() { pc = 0x15d91; continue; }
            pc = 0x15d7c;
        }
        0x15d7c => {
            // block 15d7c
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d7c mov rdi,QWORD PTR [rbx+0x78]
            rdi = m.r64(rbx.wrapping_add(0x78)) as u64;
            // 15d80 call 0xec1b8
            { let r = { let t = m.env.stub_va(0x985); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15d85 mov QWORD PTR [rbx+0x78],0x0
            m.w64(rbx.wrapping_add(0x78), (0x0u64) as u64);
            // 15d8d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 15d8f jmp 0x15d94
            pc = 0x15d94; continue;
        }
        0x15d91 => {
            // block 15d91
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d91 mov rax,rbx
            rax = rbx;
            pc = 0x15d94;
        }
        0x15d94 => {
            // block 15d94
            #[cfg(feature = "trace")] eprintln!("f_00015c78 pc 15d94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 15d94 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15d95 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15d97 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15d98 ret
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

/// parity: P2v @0xc62d4 size=435
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c62d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x4: u128 = fx[4] as u128;
    let mut x5: u128 = fx[5] as u128;
    let mut x6: u128 = fx[6] as u128;
    let mut x7: u128 = fx[7] as u128;
    let mut x8: u128 = 0;
    let mut x9: u128 = 0;
    let mut x10: u128 = 0;
    let mut x11: u128 = 0;
    let mut x12: u128 = 0;
    let mut x13: u128 = 0;
    let mut x14: u128 = 0;
    let mut x15: u128 = 0;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc62d4;
    loop {
        match pc {
        0xc62d4 => {
            // block c62d4
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c62d5 mov rbp,rsp
            rbp = rsp;
            // c62d8 test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c62da jle 0xc6485
            if fl.le() { pc = 0xc6485; continue; }
            pc = 0xc62e0;
        }
        0xc62e0 => {
            // block c62e0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c62e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c62e0 add ecx,ecx
            { let (r, f) = add::<32>((rcx as u32 as u64), (rcx as u32 as u64));
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e2 and ecx,0xfffffffc
            { let r = (rcx as u32 as u64) & 0xfffffffcu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c62e5 movsxd rax,ecx
            rax = sx::<32>((rcx as u32 as u64)) as u64;
            // c62e8 lea r10,[rsi+rax*2]
            r10 = rsi.wrapping_add(rax.wrapping_mul(2));
            // c62ec mov r9d,edx
            r9 = ((rdx as u32 as u64)) as u32 as u64;
            // c62ef xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // c62f1 movsd xmm10,QWORD PTR [rip+0x3caf6] # 0x102df0
            x10 = m.r64(m.env.data_va(95677)) as u128;
            // c62fa movapd xmm11,XMMWORD PTR [rip+0x3cb2d] # 0x102e30
            x11 = m.r128(m.env.data_va(95741));
            // c6303 movapd xmm12,XMMWORD PTR [rip+0x3cb34] # 0x102e40
            x12 = m.r128(m.env.data_va(95757));
            // c630c movapd xmm13,XMMWORD PTR [rip+0x3cb3b] # 0x102e50
            x13 = m.r128(m.env.data_va(95773));
            // c6315 movsd xmm14,QWORD PTR [rip+0x3caaa] # 0x102dc8
            x14 = m.r64(m.env.data_va(95637)) as u128;
            // c631e movsd xmm15,QWORD PTR [rip+0x3caa9] # 0x102dd0
            x15 = m.r64(m.env.data_va(95645)) as u128;
            // c6327 movsd xmm9,QWORD PTR [rip+0x3caa8] # 0x102dd8
            x9 = m.r64(m.env.data_va(95653)) as u128;
            // c6330 movapd xmm1,XMMWORD PTR [rip+0x3cac8] # 0x102e00
            x1 = m.r128(m.env.data_va(95693));
            // c6338 movapd xmm2,XMMWORD PTR [rip+0x3cad0] # 0x102e10
            x2 = m.r128(m.env.data_va(95709));
            // c6340 movapd xmm3,XMMWORD PTR [rip+0x3cad8] # 0x102e20
            x3 = m.r128(m.env.data_va(95725));
            pc = 0xc6348;
        }
        0xc6348 => {
            // block c6348
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6348 movzx ecx,WORD PTR [rdi+rdx*8+0x4]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4)) as u64) as u32 as u64;
            // c634d xorps xmm4,xmm4
            x4 = 0;
            // c6350 cvtsi2sd xmm4,ecx
            x4 = set_lo64(x4, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6354 movzx ecx,WORD PTR [rdi+rdx*8+0x2]
            rcx = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2)) as u64) as u32 as u64;
            // c6359 movzx esi,WORD PTR [rdi+rdx*8]
            rsi = (m.r16(rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u32 as u64;
            // c635d movapd xmm6,xmm4
            x6 = x4;
            // c6361 test r8b,r8b
            fl = logic::<8>((r8 as u8 as u64) & (r8 as u8 as u64));
            // c6364 je 0xc63d0
            if fl.e() { pc = 0xc63d0; continue; }
            pc = 0xc6366;
        }
        0xc6366 => {
            // block c6366
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6366 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6366 mulsd xmm6,QWORD PTR [rip+0x3ca72] # 0x102de0
            x6 = set_lo64(x6, (f64_of(x6) * f64_of((m.r64(m.env.data_va(95661)) as u128))).to_bits());
            // c636e movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c6371 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6375 movapd xmm5,xmm7
            x5 = x7;
            // c6379 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c637d unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c6381 mulsd xmm7,QWORD PTR [rip+0x3ca5f] # 0x102de8
            x7 = set_lo64(x7, (f64_of(x7) * f64_of((m.r64(m.env.data_va(95669)) as u128))).to_bits());
            // c6389 addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c638d movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c6390 xorps xmm6,xmm6
            x6 = 0;
            // c6393 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c6397 movddup xmm8,xmm6
            { let v = (x6) as u64; x8 = (v as u128) | ((v as u128) << 64); }
            // c639c mulsd xmm6,xmm10
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x10)).to_bits());
            // c63a1 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c63a5 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c63aa add esi,0x40
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x40u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c63ad mulpd xmm5,xmm11
            { let a = x5; let b = x11; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b2 mulpd xmm4,xmm12
            { let a = x4; let b = x12; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63b7 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63bb mulpd xmm8,xmm13
            { let a = x8; let b = x13; x8 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c0 movapd xmm4,xmm5
            x4 = x5;
            // c63c4 addpd xmm4,xmm8
            { let a = x4; let b = x8; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63c9 subpd xmm5,xmm8
            { let a = x5; let b = x8; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c63ce jmp 0xc6429
            pc = 0xc6429; continue;
        }
        0xc63d0 => {
            // block c63d0
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c63d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c63d0 mulsd xmm6,xmm14
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x14)).to_bits());
            // c63d5 movzx ecx,cx
            rcx = ((rcx as u16 as u64)) as u32 as u64;
            // c63d8 cvtsi2sd xmm7,ecx
            x7 = set_lo64(x7, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63dc movapd xmm5,xmm7
            x5 = x7;
            // c63e0 unpcklpd xmm5,xmm4
            x5 = (x5 & u64::MAX as u128) | ((x4 & u64::MAX as u128) << 64);
            // c63e4 unpcklpd xmm4,xmm7
            x4 = (x4 & u64::MAX as u128) | ((x7 & u64::MAX as u128) << 64);
            // c63e8 mulsd xmm7,xmm15
            x7 = set_lo64(x7, (f64_of(x7) * f64_of(x15)).to_bits());
            // c63ed addsd xmm7,xmm6
            x7 = set_lo64(x7, (f64_of(x7) + f64_of(x6)).to_bits());
            // c63f1 movzx ecx,si
            rcx = ((rsi as u16 as u64)) as u32 as u64;
            // c63f4 xorps xmm6,xmm6
            x6 = 0;
            // c63f7 cvtsi2sd xmm6,ecx
            x6 = set_lo64(x6, (sx::<32>((rcx as u32 as u64)) as f64).to_bits());
            // c63fb movddup xmm0,xmm6
            { let v = (x6) as u64; x0 = (v as u128) | ((v as u128) << 64); }
            // c63ff mulsd xmm6,xmm9
            x6 = set_lo64(x6, (f64_of(x6) * f64_of(x9)).to_bits());
            // c6404 addsd xmm6,xmm7
            x6 = set_lo64(x6, (f64_of(x6) + f64_of(x7)).to_bits());
            // c6408 cvttsd2si rsi,xmm6
            rsi = cvtt64(f64_of(x6)) as u64;
            // c640d mulpd xmm5,xmm1
            { let a = x5; let b = x1; x5 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6411 mulpd xmm4,xmm2
            { let a = x4; let b = x2; x4 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6415 subpd xmm5,xmm4
            { let a = x5; let b = x4; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6419 mulpd xmm0,xmm3
            { let a = x0; let b = x3; x0 = ((f64::from_bits(a as u64) * f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) * f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c641d movapd xmm4,xmm5
            x4 = x5;
            // c6421 addpd xmm4,xmm0
            { let a = x4; let b = x0; x4 = ((f64::from_bits(a as u64) + f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) + f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            // c6425 subpd xmm5,xmm0
            { let a = x5; let b = x0; x5 = ((f64::from_bits(a as u64) - f64::from_bits(b as u64)).to_bits() as u128) | (((f64::from_bits((a >> 64) as u64) - f64::from_bits((b >> 64) as u64)).to_bits() as u128) << 64); }
            pc = 0xc6429;
        }
        0xc6429 => {
            // block c6429
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6429 movsd xmm5,xmm4
            x5 = set_lo64(x5, x4 as u64);
            // c642d and esi,0x3ff
            { let r = (rsi as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // c6433 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c6436 jne 0xc6471
            if fl.ne() { pc = 0xc6471; continue; }
            pc = 0xc6438;
        }
        0xc6438 => {
            // block c6438
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6438 cvttsd2si rcx,xmm5
            rcx = cvtt64(f64_of(x5)) as u64;
            // c643d add ecx,0x200
            { let (r, f) = add::<32>((rcx as u32 as u64), 0x200u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6443 and ecx,0x3ff
            { let r = (rcx as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // c6449 shufpd xmm5,xmm5,0x1
            { let a = x5; let b = x5; x5 = (((a >> 64)) & u64::MAX as u128) | (((b) & u64::MAX as u128) << 64); }
            // c644e cvttsd2si rax,xmm5
            rax = cvtt64(f64_of(x5)) as u64;
            // c6453 add eax,0x200
            { let (r, f) = add::<32>((rax as u32 as u64), 0x200u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // c6458 and eax,0x3ff
            { let r = (rax as u32 as u64) & 0x3ffu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c645d mov WORD PTR [r10],cx
            m.w16(r10, ((rcx as u16 as u64)) as u16);
            // c6461 mov WORD PTR [r10+0x2],si
            m.w16(r10.wrapping_add(0x2), ((rsi as u16 as u64)) as u16);
            // c6466 mov WORD PTR [r10+0x4],ax
            m.w16(r10.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // c646b add r10,0x6
            { let (r, f) = add::<64>(r10, 0x6u64);
              r10 = r;
              fl = f; }
            // c646f jmp 0xc6479
            pc = 0xc6479; continue;
        }
        0xc6471 => {
            // block c6471
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6471 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6471 mov WORD PTR [r10],si
            m.w16(r10, ((rsi as u16 as u64)) as u16);
            // c6475 add r10,0x2
            { let (r, f) = add::<64>(r10, 0x2u64);
              r10 = r;
              fl = f; }
            pc = 0xc6479;
        }
        0xc6479 => {
            // block c6479
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6479 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6479 inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c647c cmp r9d,edx
            fl = sub::<32>((r9 as u32 as u64), (rdx as u32 as u64)).1;
            // c647f jne 0xc6348
            if fl.ne() { pc = 0xc6348; continue; }
            pc = 0xc6485;
        }
        0xc6485 => {
            // block c6485
            #[cfg(feature = "trace")] eprintln!("f_000c62d4 pc c6485 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c6485 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c6486 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaba1a size=454
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aba1a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaba1a;
    loop {
        match pc {
        0xaba1a => {
            // block aba1a
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc aba1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aba1a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aba1b mov rbp,rsp
            rbp = rsp;
            // aba1e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // aba20 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aba22 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // aba24 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // aba26 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aba27 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // aba2e mov r13,rdi
            r13 = rdi;
            // aba31 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // aba38 lea r15,[rbp-0x140]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // aba3f mov rax,QWORD PTR [rip+0x9478a] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // aba46 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // aba4a mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // aba51 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // aba55 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // aba5c mov rsi,r15
            rsi = r15;
            // aba5f call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aba64 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // aba6c mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // aba73 mov rbx,QWORD PTR [rip+0x94946] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // aba7a lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // aba7e mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // aba85 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // aba89 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // aba90 mov rdi,r15
            rdi = r15;
            // aba93 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aba98 mov r12,QWORD PTR [rip+0x94909] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // aba9f add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // abaa3 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // abaaa mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // abab5 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // abac0 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // abacb mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // abad6 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // abae0 xorps xmm0,xmm0
            x0 = 0;
            // abae3 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // abae7 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // abaef lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abaf3 mov rdi,r15
            rdi = r15;
            // abaf6 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abafb lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abaff call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb04 mov rdi,r13
            rdi = r13;
            // abb07 mov rsi,r15
            rsi = r15;
            // abb0a call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb0f mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // abb16 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // abb1d mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // abb24 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // abb2b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb30 mov rdi,r15
            rdi = r15;
            // abb33 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb38 mov rsi,QWORD PTR [rip+0x946c9] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // abb3f add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // abb43 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // abb4a call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb4f lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // abb56 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb5b mov rax,r13
            rax = r13;
            // abb5e add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // abb65 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // abb66 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // abb68 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // abb6a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // abb6c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // abb6e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // abb6f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xabb70 => {
            // block abb70
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abb70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abb70 mov r13,rax
            r13 = rax;
            // abb73 jmp 0xabbcb
            pc = 0xabbcb; continue;
        }
        0xabb75 => {
            // block abb75
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abb75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abb75 mov r13,rax
            r13 = rax;
            // abb78 jmp 0xabbb4
            pc = 0xabbb4; continue;
        }
        0xabb7a => {
            // block abb7a
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abb7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abb7a mov r13,rax
            r13 = rax;
            // abb7d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abb81 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abb86 jmp 0xabba0
            pc = 0xabba0; continue;
        }
        0xabb88 => {
            // block abb88
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abb88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abb88 mov r13,rax
            r13 = rax;
            // abb8b mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // abb92 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // abb99 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            pc = 0xabba0;
        }
        0xabba0 => {
            // block abba0
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abba0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abba0 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // abba7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abbac mov rdi,r15
            rdi = r15;
            // abbaf call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xabbb4;
        }
        0xabbb4 => {
            // block abbb4
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abbb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abbb4 mov rsi,QWORD PTR [rip+0x9464d] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // abbbb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // abbbf lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // abbc6 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xabbcb;
        }
        0xabbcb => {
            // block abbcb
            #[cfg(feature = "trace")] eprintln!("f_000aba1a pc abbcb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abbcb lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // abbd2 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abbd7 mov rdi,r13
            rdi = r13;
            // abbda call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xabdba size=454
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000abdba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xabdba;
    loop {
        match pc {
        0xabdba => {
            // block abdba
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abdba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abdba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // abdbb mov rbp,rsp
            rbp = rsp;
            // abdbe push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // abdc0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // abdc2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // abdc4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // abdc6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // abdc7 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // abdce mov r13,rdi
            r13 = rdi;
            // abdd1 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // abdd8 lea r15,[rbp-0x140]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // abddf mov rax,QWORD PTR [rip+0x943ea] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // abde6 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // abdea mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // abdf1 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // abdf5 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // abdfc mov rsi,r15
            rsi = r15;
            // abdff call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abe04 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // abe0c mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // abe13 mov rbx,QWORD PTR [rip+0x945a6] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // abe1a lea r14,[rbx+0x18]
            r14 = rbx.wrapping_add(0x18);
            // abe1e mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // abe25 add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // abe29 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // abe30 mov rdi,r15
            rdi = r15;
            // abe33 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abe38 mov r12,QWORD PTR [rip+0x94569] # 0x1403a8
            r12 = m.r64(m.env.data_va(346997)) as u64;
            // abe3f add r12,0x10
            { let (r, f) = add::<64>(r12, 0x10u64);
              r12 = r;
              fl = f; }
            // abe43 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // abe4a mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // abe55 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // abe60 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // abe6b mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // abe76 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // abe80 xorps xmm0,xmm0
            x0 = 0;
            // abe83 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // abe87 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // abe8f lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abe93 mov rdi,r15
            rdi = r15;
            // abe96 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abe9b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abe9f call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abea4 mov rdi,r13
            rdi = r13;
            // abea7 mov rsi,r15
            rsi = r15;
            // abeaa call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abeaf mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // abeb6 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // abebd mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            // abec4 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // abecb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abed0 mov rdi,r15
            rdi = r15;
            // abed3 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abed8 mov rsi,QWORD PTR [rip+0x94329] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // abedf add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // abee3 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // abeea call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abeef lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // abef6 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abefb mov rax,r13
            rax = r13;
            // abefe add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // abf05 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // abf06 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // abf08 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // abf0a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // abf0c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // abf0e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // abf0f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xabf10 => {
            // block abf10
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf10 mov r13,rax
            r13 = rax;
            // abf13 jmp 0xabf6b
            pc = 0xabf6b; continue;
        }
        0xabf15 => {
            // block abf15
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf15 mov r13,rax
            r13 = rax;
            // abf18 jmp 0xabf54
            pc = 0xabf54; continue;
        }
        0xabf1a => {
            // block abf1a
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf1a mov r13,rax
            r13 = rax;
            // abf1d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // abf21 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abf26 jmp 0xabf40
            pc = 0xabf40; continue;
        }
        0xabf28 => {
            // block abf28
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf28 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf28 mov r13,rax
            r13 = rax;
            // abf2b mov QWORD PTR [rbp-0x148],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r14) as u64);
            // abf32 mov QWORD PTR [rbp-0xd8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rbx) as u64);
            // abf39 mov QWORD PTR [rbp-0x140],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r12) as u64);
            pc = 0xabf40;
        }
        0xabf40 => {
            // block abf40
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf40 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf40 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // abf47 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abf4c mov rdi,r15
            rdi = r15;
            // abf4f call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xabf54;
        }
        0xabf54 => {
            // block abf54
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf54 mov rsi,QWORD PTR [rip+0x942ad] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // abf5b add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // abf5f lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // abf66 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xabf6b;
        }
        0xabf6b => {
            // block abf6b
            #[cfg(feature = "trace")] eprintln!("f_000abdba pc abf6b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // abf6b lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // abf72 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abf77 mov rdi,r13
            rdi = r13;
            // abf7a call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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

/// parity: P2v @0xc3708 size=505
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3708<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3708;
    loop {
        match pc {
        0xc3708 => {
            // block c3708
            #[cfg(feature = "trace")] eprintln!("f_000c3708 pc c3708 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3708 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3709 mov rbp,rsp
            rbp = rsp;
            // c370c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c370d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c370e lea rbx,[rip+0x104f03] # 0x1c8618
            rbx = m.env.data_va(904677);
            // c3715 mov rsi,QWORD PTR [rip+0x104efc] # 0x1c8618
            rsi = m.r64(m.env.data_va(904677)) as u64;
            // c371c lea rdi,[rip+0x104eed] # 0x1c8610
            rdi = m.env.data_va(904669);
            // c3723 call 0xebe34
            { let r = { let t = m.env.stub_va(0x601); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3728 mov QWORD PTR [rip+0x104eed],0x0 # 0x1c8620
            m.w64(m.env.data_va(904685), (0x0u64) as u64);
            // c3733 mov QWORD PTR [rip+0x104ed6],rbx # 0x1c8610
            m.w64(m.env.data_va(904669), (rbx) as u64);
            // c373a mov QWORD PTR [rip+0x104ed3],0x0 # 0x1c8618
            m.w64(m.env.data_va(904677), (0x0u64) as u64);
            // c3745 lea rbx,[rip+0x104eb4] # 0x1c8600
            rbx = m.env.data_va(904653);
            // c374c mov rsi,QWORD PTR [rip+0x104ead] # 0x1c8600
            rsi = m.r64(m.env.data_va(904653)) as u64;
            // c3753 lea rdi,[rip+0x104e9e] # 0x1c85f8
            rdi = m.env.data_va(904645);
            // c375a call 0xebe70
            { let r = { let t = m.env.stub_va(0x63d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c375f mov QWORD PTR [rip+0x104e9e],0x0 # 0x1c8608
            m.w64(m.env.data_va(904661), (0x0u64) as u64);
            // c376a mov QWORD PTR [rip+0x104e87],rbx # 0x1c85f8
            m.w64(m.env.data_va(904645), (rbx) as u64);
            // c3771 mov QWORD PTR [rip+0x104e84],0x0 # 0x1c8600
            m.w64(m.env.data_va(904653), (0x0u64) as u64);
            // c377c lea rbx,[rip+0x104ead] # 0x1c8630
            rbx = m.env.data_va(904701);
            // c3783 mov rsi,QWORD PTR [rip+0x104ea6] # 0x1c8630
            rsi = m.r64(m.env.data_va(904701)) as u64;
            // c378a lea rdi,[rip+0x104e97] # 0x1c8628
            rdi = m.env.data_va(904693);
            // c3791 call 0xebdf8
            { let r = { let t = m.env.stub_va(0x5c5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3796 mov QWORD PTR [rip+0x104e97],0x0 # 0x1c8638
            m.w64(m.env.data_va(904709), (0x0u64) as u64);
            // c37a1 mov QWORD PTR [rip+0x104e80],rbx # 0x1c8628
            m.w64(m.env.data_va(904693), (rbx) as u64);
            // c37a8 mov QWORD PTR [rip+0x104e7d],0x0 # 0x1c8630
            m.w64(m.env.data_va(904701), (0x0u64) as u64);
            // c37b3 lea rbx,[rip+0x104ea6] # 0x1c8660
            rbx = m.env.data_va(904749);
            // c37ba mov rsi,QWORD PTR [rip+0x104e9f] # 0x1c8660
            rsi = m.r64(m.env.data_va(904749)) as u64;
            // c37c1 lea rdi,[rip+0x104e90] # 0x1c8658
            rdi = m.env.data_va(904741);
            // c37c8 call 0xebe58
            { let r = { let t = m.env.stub_va(0x625); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c37cd mov QWORD PTR [rip+0x104e90],0x0 # 0x1c8668
            m.w64(m.env.data_va(904757), (0x0u64) as u64);
            // c37d8 mov QWORD PTR [rip+0x104e79],rbx # 0x1c8658
            m.w64(m.env.data_va(904741), (rbx) as u64);
            // c37df mov QWORD PTR [rip+0x104e76],0x0 # 0x1c8660
            m.w64(m.env.data_va(904749), (0x0u64) as u64);
            // c37ea lea rbx,[rip+0x104e57] # 0x1c8648
            rbx = m.env.data_va(904725);
            // c37f1 mov rsi,QWORD PTR [rip+0x104e50] # 0x1c8648
            rsi = m.r64(m.env.data_va(904725)) as u64;
            // c37f8 lea rdi,[rip+0x104e41] # 0x1c8640
            rdi = m.env.data_va(904717);
            // c37ff call 0xebe88
            { let r = { let t = m.env.stub_va(0x655); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3804 mov QWORD PTR [rip+0x104e41],0x0 # 0x1c8650
            m.w64(m.env.data_va(904733), (0x0u64) as u64);
            // c380f mov QWORD PTR [rip+0x104e2a],rbx # 0x1c8640
            m.w64(m.env.data_va(904717), (rbx) as u64);
            // c3816 mov QWORD PTR [rip+0x104e27],0x0 # 0x1c8648
            m.w64(m.env.data_va(904725), (0x0u64) as u64);
            // c3821 lea rbx,[rip+0x104e50] # 0x1c8678
            rbx = m.env.data_va(904773);
            // c3828 mov rsi,QWORD PTR [rip+0x104e49] # 0x1c8678
            rsi = m.r64(m.env.data_va(904773)) as u64;
            // c382f lea rdi,[rip+0x104e3a] # 0x1c8670
            rdi = m.env.data_va(904765);
            // c3836 call 0xebe46
            { let r = { let t = m.env.stub_va(0x613); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c383b mov QWORD PTR [rip+0x104e3a],0x0 # 0x1c8680
            m.w64(m.env.data_va(904781), (0x0u64) as u64);
            // c3846 mov QWORD PTR [rip+0x104e23],rbx # 0x1c8670
            m.w64(m.env.data_va(904765), (rbx) as u64);
            // c384d mov QWORD PTR [rip+0x104e20],0x0 # 0x1c8678
            m.w64(m.env.data_va(904773), (0x0u64) as u64);
            // c3858 lea rbx,[rip+0x104e49] # 0x1c86a8
            rbx = m.env.data_va(904821);
            // c385f mov rsi,QWORD PTR [rip+0x104e42] # 0x1c86a8
            rsi = m.r64(m.env.data_va(904821)) as u64;
            // c3866 lea rdi,[rip+0x104e33] # 0x1c86a0
            rdi = m.env.data_va(904813);
            // c386d call 0xebdce
            { let r = { let t = m.env.stub_va(0x59b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3872 mov QWORD PTR [rip+0x104e33],0x0 # 0x1c86b0
            m.w64(m.env.data_va(904829), (0x0u64) as u64);
            // c387d mov QWORD PTR [rip+0x104e1c],rbx # 0x1c86a0
            m.w64(m.env.data_va(904813), (rbx) as u64);
            // c3884 mov QWORD PTR [rip+0x104e19],0x0 # 0x1c86a8
            m.w64(m.env.data_va(904821), (0x0u64) as u64);
            // c388f lea rbx,[rip+0x104dfa] # 0x1c8690
            rbx = m.env.data_va(904797);
            // c3896 mov rsi,QWORD PTR [rip+0x104df3] # 0x1c8690
            rsi = m.r64(m.env.data_va(904797)) as u64;
            // c389d lea rdi,[rip+0x104de4] # 0x1c8688
            rdi = m.env.data_va(904789);
            // c38a4 call 0xebde6
            { let r = { let t = m.env.stub_va(0x5b3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c38a9 mov QWORD PTR [rip+0x104de4],0x0 # 0x1c8698
            m.w64(m.env.data_va(904805), (0x0u64) as u64);
            // c38b4 mov QWORD PTR [rip+0x104dcd],rbx # 0x1c8688
            m.w64(m.env.data_va(904789), (rbx) as u64);
            // c38bb mov QWORD PTR [rip+0x104dca],0x0 # 0x1c8690
            m.w64(m.env.data_va(904797), (0x0u64) as u64);
            // c38c6 mov rax,QWORD PTR [rip+0x104d3b] # 0x1c8608
            rax = m.r64(m.env.data_va(904661)) as u64;
            // c38cd or rax,QWORD PTR [rip+0x104d4c] # 0x1c8620
            { let r = rax | m.r64(m.env.data_va(904685)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38d4 or rax,QWORD PTR [rip+0x104d5d] # 0x1c8638
            { let r = rax | m.r64(m.env.data_va(904709)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38db or rax,QWORD PTR [rip+0x104dce] # 0x1c86b0
            { let r = rax | m.r64(m.env.data_va(904829)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38e2 or rax,QWORD PTR [rip+0x104d7f] # 0x1c8668
            { let r = rax | m.r64(m.env.data_va(904757)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38e9 or rax,QWORD PTR [rip+0x104d60] # 0x1c8650
            { let r = rax | m.r64(m.env.data_va(904733)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38f0 or rax,QWORD PTR [rip+0x104d89] # 0x1c8680
            { let r = rax | m.r64(m.env.data_va(904781)) as u64; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // c38f7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // c38fa add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c38fe pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c38ff pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3900 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64410 size=475
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x4: u128 = fx[4] as u128;
    let mut x5: u128 = fx[5] as u128;
    let mut x6: u128 = fx[6] as u128;
    let mut x7: u128 = fx[7] as u128;
    let mut x8: u128 = 0;
    let mut x9: u128 = 0;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64410;
    loop {
        match pc {
        0x64410 => {
            // block 64410
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64411 mov rbp,rsp
            rbp = rsp;
            // 64414 movss xmm0,DWORD PTR [rdi]
            x0 = m.r32(rdi) as u128;
            // 64418 xorps xmm1,xmm1
            x1 = 0;
            // 6441b ucomiss xmm1,xmm0
            fl = ucomi(f32_of(x1) as f64, f32_of(x0) as f64);
            // 6441e ja 0x64429
            if fl.a() { pc = 0x64429; continue; }
            pc = 0x64420;
        }
        0x64420 => {
            // block 64420
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64420 ucomiss xmm0,DWORD PTR [rip+0x9c38d] # 0x1007b4
            fl = ucomi(f32_of(x0) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64427 jbe 0x6442d
            if fl.be() { pc = 0x6442d; continue; }
            pc = 0x64429;
        }
        0x64429 => {
            // block 64429
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64429 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x6442b;
        }
        0x6442b => {
            // block 6442b
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6442b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6442b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6442c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6442d => {
            // block 6442d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6442d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6442d movss xmm1,DWORD PTR [rdi+0x4]
            x1 = m.r32(rdi.wrapping_add(0x4)) as u128;
            // 64432 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64434 xorps xmm2,xmm2
            x2 = 0;
            // 64437 ucomiss xmm2,xmm1
            fl = ucomi(f32_of(x2) as f64, f32_of(x1) as f64);
            // 6443a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6443c;
        }
        0x6443c => {
            // block 6443c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6443c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6443c ucomiss xmm1,DWORD PTR [rip+0x9c371] # 0x1007b4
            fl = ucomi(f32_of(x1) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64443 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64445;
        }
        0x64445 => {
            // block 64445
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64445 movss xmm2,DWORD PTR [rdi+0x8]
            x2 = m.r32(rdi.wrapping_add(0x8)) as u128;
            // 6444a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6444c xorps xmm3,xmm3
            x3 = 0;
            // 6444f ucomiss xmm3,xmm2
            fl = ucomi(f32_of(x3) as f64, f32_of(x2) as f64);
            // 64452 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64454;
        }
        0x64454 => {
            // block 64454
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64454 ucomiss xmm2,DWORD PTR [rip+0x9c359] # 0x1007b4
            fl = ucomi(f32_of(x2) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 6445b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6445d;
        }
        0x6445d => {
            // block 6445d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6445d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6445d movss xmm3,DWORD PTR [rdi+0xc]
            x3 = m.r32(rdi.wrapping_add(0xc)) as u128;
            // 64462 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64464 xorps xmm4,xmm4
            x4 = 0;
            // 64467 ucomiss xmm4,xmm3
            fl = ucomi(f32_of(x4) as f64, f32_of(x3) as f64);
            // 6446a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6446c;
        }
        0x6446c => {
            // block 6446c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6446c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6446c ucomiss xmm3,DWORD PTR [rip+0x9c341] # 0x1007b4
            fl = ucomi(f32_of(x3) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64473 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64475;
        }
        0x64475 => {
            // block 64475
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64475 movss xmm4,DWORD PTR [rdi+0x10]
            x4 = m.r32(rdi.wrapping_add(0x10)) as u128;
            // 6447a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6447c xorps xmm5,xmm5
            x5 = 0;
            // 6447f ucomiss xmm5,xmm4
            fl = ucomi(f32_of(x5) as f64, f32_of(x4) as f64);
            // 64482 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64484;
        }
        0x64484 => {
            // block 64484
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64484 ucomiss xmm4,DWORD PTR [rip+0x9c329] # 0x1007b4
            fl = ucomi(f32_of(x4) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 6448b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6448d;
        }
        0x6448d => {
            // block 6448d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6448d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6448d movss xmm5,DWORD PTR [rdi+0x14]
            x5 = m.r32(rdi.wrapping_add(0x14)) as u128;
            // 64492 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64494 xorps xmm6,xmm6
            x6 = 0;
            // 64497 ucomiss xmm6,xmm5
            fl = ucomi(f32_of(x6) as f64, f32_of(x5) as f64);
            // 6449a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6449c;
        }
        0x6449c => {
            // block 6449c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6449c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6449c ucomiss xmm5,DWORD PTR [rip+0x9c311] # 0x1007b4
            fl = ucomi(f32_of(x5) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644a3 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644a5;
        }
        0x644a5 => {
            // block 644a5
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644a5 movss xmm6,DWORD PTR [rdi+0x18]
            x6 = m.r32(rdi.wrapping_add(0x18)) as u128;
            // 644aa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644ac xorps xmm7,xmm7
            x7 = 0;
            // 644af ucomiss xmm7,xmm6
            fl = ucomi(f32_of(x7) as f64, f32_of(x6) as f64);
            // 644b2 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644b8;
        }
        0x644b8 => {
            // block 644b8
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644b8 ucomiss xmm6,DWORD PTR [rip+0x9c2f5] # 0x1007b4
            fl = ucomi(f32_of(x6) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644bf ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644c5;
        }
        0x644c5 => {
            // block 644c5
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644c5 movss xmm9,DWORD PTR [rdi+0x1c]
            x9 = m.r32(rdi.wrapping_add(0x1c)) as u128;
            // 644cb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644cd xorps xmm7,xmm7
            x7 = 0;
            // 644d0 ucomiss xmm7,xmm9
            fl = ucomi(f32_of(x7) as f64, f32_of(x9) as f64);
            // 644d4 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644da;
        }
        0x644da => {
            // block 644da
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644da ucomiss xmm9,DWORD PTR [rip+0x9c2d2] # 0x1007b4
            fl = ucomi(f32_of(x9) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644e2 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644e8;
        }
        0x644e8 => {
            // block 644e8
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644e8 movss xmm8,DWORD PTR [rdi+0x24]
            x8 = m.r32(rdi.wrapping_add(0x24)) as u128;
            // 644ee xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644f0 xorps xmm7,xmm7
            x7 = 0;
            // 644f3 ucomiss xmm7,xmm8
            fl = ucomi(f32_of(x7) as f64, f32_of(x8) as f64);
            // 644f7 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644fd;
        }
        0x644fd => {
            // block 644fd
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644fd cvtss2sd xmm8,xmm8
            x8 = set_lo64(x8, (f32_of(x8) as f64).to_bits());
            // 64502 ucomisd xmm8,QWORD PTR [rip+0x9c28d] # 0x100798
            fl = ucomi(f64_of(x8), f64_of((m.r64(m.env.data_va(85861)) as u128)));
            // 6450b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64511;
        }
        0x64511 => {
            // block 64511
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64511 xorps xmm7,xmm7
            x7 = 0;
            // 64514 cvtss2sd xmm7,xmm0
            x7 = set_lo64(x7, (f32_of(x0) as f64).to_bits());
            // 64518 movsd xmm0,QWORD PTR [rip+0x9c280] # 0x1007a0
            x0 = m.r64(m.env.data_va(85869)) as u128;
            // 64520 divsd xmm7,xmm0
            x7 = set_lo64(x7, (f64_of(x7) / f64_of(x0)).to_bits());
            // 64524 cvttsd2si eax,xmm7
            rax = (cvtt32(f64_of(x7)) as u64) as u32 as u64;
            // 64528 mov WORD PTR [rsi],ax
            m.w16(rsi, ((rax as u16 as u64)) as u16);
            // 6452b cvtss2sd xmm1,xmm1
            x1 = set_lo64(x1, (f32_of(x1) as f64).to_bits());
            // 6452f divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64533 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64537 mov WORD PTR [rsi+0x2],ax
            m.w16(rsi.wrapping_add(0x2), ((rax as u16 as u64)) as u16);
            // 6453b xorps xmm1,xmm1
            x1 = 0;
            // 6453e cvtss2sd xmm1,xmm2
            x1 = set_lo64(x1, (f32_of(x2) as f64).to_bits());
            // 64542 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64546 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 6454a mov WORD PTR [rsi+0x4],ax
            m.w16(rsi.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // 6454e xorps xmm1,xmm1
            x1 = 0;
            // 64551 cvtss2sd xmm1,xmm3
            x1 = set_lo64(x1, (f32_of(x3) as f64).to_bits());
            // 64555 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64559 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 6455d mov WORD PTR [rsi+0x6],ax
            m.w16(rsi.wrapping_add(0x6), ((rax as u16 as u64)) as u16);
            // 64561 xorps xmm1,xmm1
            x1 = 0;
            // 64564 cvtss2sd xmm1,xmm4
            x1 = set_lo64(x1, (f32_of(x4) as f64).to_bits());
            // 64568 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 6456c cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64570 mov WORD PTR [rsi+0x8],ax
            m.w16(rsi.wrapping_add(0x8), ((rax as u16 as u64)) as u16);
            // 64574 xorps xmm1,xmm1
            x1 = 0;
            // 64577 cvtss2sd xmm1,xmm5
            x1 = set_lo64(x1, (f32_of(x5) as f64).to_bits());
            // 6457b divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 6457f cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64583 mov WORD PTR [rsi+0xa],ax
            m.w16(rsi.wrapping_add(0xa), ((rax as u16 as u64)) as u16);
            // 64587 xorps xmm1,xmm1
            x1 = 0;
            // 6458a cvtss2sd xmm1,xmm6
            x1 = set_lo64(x1, (f32_of(x6) as f64).to_bits());
            // 6458e divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64592 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64596 mov WORD PTR [rsi+0xc],ax
            m.w16(rsi.wrapping_add(0xc), ((rax as u16 as u64)) as u16);
            // 6459a xorps xmm1,xmm1
            x1 = 0;
            // 6459d cvtss2sd xmm1,xmm9
            x1 = set_lo64(x1, (f32_of(x9) as f64).to_bits());
            // 645a2 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 645a6 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 645aa mov WORD PTR [rsi+0xe],ax
            m.w16(rsi.wrapping_add(0xe), ((rax as u16 as u64)) as u16);
            // 645ae divsd xmm8,QWORD PTR [rip+0x9c1f1] # 0x1007a8
            x8 = set_lo64(x8, (f64_of(x8) / f64_of((m.r64(m.env.data_va(85877)) as u128))).to_bits());
            // 645b7 cvttsd2si eax,xmm8
            rax = (cvtt32(f64_of(x8)) as u64) as u32 as u64;
            // 645bc mov WORD PTR [rsi+0x12],ax
            m.w16(rsi.wrapping_add(0x12), ((rax as u16 as u64)) as u16);
            // 645c0 mov ax,WORD PTR [rdi+0x20]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x20)) as u64) as u16 as u64);
            // 645c4 mov WORD PTR [rsi+0x10],ax
            m.w16(rsi.wrapping_add(0x10), ((rax as u16 as u64)) as u16);
            // 645c8 mov ax,WORD PTR [rdi+0x28]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x28)) as u64) as u16 as u64);
            // 645cc mov WORD PTR [rsi+0x14],ax
            m.w16(rsi.wrapping_add(0x14), ((rax as u16 as u64)) as u16);
            // 645d0 mov ax,WORD PTR [rdi+0x2a]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x2a)) as u64) as u16 as u64);
            // 645d4 mov WORD PTR [rsi+0x16],ax
            m.w16(rsi.wrapping_add(0x16), ((rax as u16 as u64)) as u16);
            // 645d8 mov al,BYTE PTR [rdi+0x2c]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x2c)) as u64) as u8 as u64);
            // 645db mov BYTE PTR [rsi+0x18],al
            m.w8(rsi.wrapping_add(0x18), ((rax as u8 as u64)) as u8);
            // 645de mov al,BYTE PTR [rdi+0x2d]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x2d)) as u64) as u8 as u64);
            // 645e1 mov BYTE PTR [rsi+0x19],al
            m.w8(rsi.wrapping_add(0x19), ((rax as u8 as u64)) as u8);
            // 645e4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 645e6 jmp 0x6442b
            pc = 0x6442b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfd02 size=56
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fd02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xfd02;
    loop {
        match pc {
        0xfd02 => {
            // block fd02
            #[cfg(feature = "trace")] eprintln!("f_0000fd02 pc fd02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fd02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fd03 mov rbp,rsp
            rbp = rsp;
            // fd06 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fd08 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fd09 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // fd0d mov rbx,rdi
            rbx = rdi;
            // fd10 lea r14,[rbp-0x30]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // fd14 mov rdi,rsi
            rdi = rsi;
            // fd17 mov rsi,r14
            rsi = r14;
            // fd1a call 0x64410
            { let r = { let t = m.env.code_va(0x64410); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd1f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd22 mov rdi,rbx
            rdi = rbx;
            // fd25 mov rsi,r14
            rsi = r14;
            // fd28 call QWORD PTR [rax+0x1cf0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cf0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd2e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // fd30 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // fd34 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fd35 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fd37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fd38 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8298 size=232
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8298<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8298;
    loop {
        match pc {
        0xc8298 => {
            // block c8298
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8298 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8299 mov rbp,rsp
            rbp = rsp;
            // c829c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // c829e mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c82a1 mov rsi,QWORD PTR [rbp+0x10]
            rsi = m.r64(rbp.wrapping_add(0x10)) as u64;
            // c82a5 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c82a8 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c82ab jbe 0xc82c1
            if fl.be() { pc = 0xc82c1; continue; }
            pc = 0xc82ad;
        }
        0xc82ad => {
            // block c82ad
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82ad imul eax,r8d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r8 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // c82b1 add r9,rax
            { let (r, f) = add::<64>(r9, rax);
              r9 = r;
              fl = f; }
            // c82b4 mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c82b7 mov rdi,r9
            rdi = r9;
            // c82ba call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c82bf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c82c0 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc82c1 => {
            // block c82c1
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82c1 lea rdx,[rip+0xa8] # 0xc8370
            rdx = m.env.code_va(0xc8370);
            // c82c8 movsxd rdi,DWORD PTR [rdx+rdi*4]
            rdi = sx::<32>(m.r32(rdx.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c82cc add rdi,rdx
            { let (r, f) = add::<64>(rdi, rdx);
              rdi = r;
              fl = f; }
            // c82cf jmp rdi
            pc = (rdi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc82d1 => {
            // block c82d1
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82d1 lea edx,[rcx+0x2]
            rdx = (rcx.wrapping_add(0x2)) as u32 as u64;
            // c82d4 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c82d9 imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c82dd shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c82e1 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c82e4 shl edi,0x2
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x2u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c82e7 add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c82ea mov rdi,rsi
            rdi = rsi;
            // c82ed mov rsi,r9
            rsi = r9;
            // c82f0 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c82f2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c82f4 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c82f7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c82f8 jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc82fd => {
            // block c82fd
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c82fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c82fd mov edx,r8d
            rdx = ((r8 as u32 as u64)) as u32 as u64;
            // c8300 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c8305 imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c8309 shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c830d imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c8310 add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c8313 mov rdi,rsi
            rdi = rsi;
            // c8316 mov rsi,r9
            rsi = r9;
            // c8319 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c831b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c831d mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c8320 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8321 jmp 0xc772c
            { let r = { let t = m.env.code_va(0xc772c); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc8326 => {
            // block c8326
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8326 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8326 lea edx,[rcx+0x2]
            rdx = (rcx.wrapping_add(0x2)) as u32 as u64;
            // c8329 mov edi,0xaaaaaaab
            rdi = (0xaaaaaaabu64) as u32 as u64;
            // c832e imul rdi,rdx
            { let (r, f) = imul::<64>(rdi, rdx); rdi = r; fl = f; }
            // c8332 shr rdi,0x21
            { let (r, f) = shr::<64>(rdi, 0x21u64, fl); rdi = r; fl = f; }
            // c8336 imul edi,eax
            { let (r, f) = imul::<32>((rdi as u32 as u64), (rax as u32 as u64)); rdi = (r) as u32 as u64; fl = f; }
            // c8339 shl edi,0x2
            { let (r, f) = shl::<32>((rdi as u32 as u64), 0x2u64, fl); rdi = (r) as u32 as u64; fl = f; }
            // c833c add r9,rdi
            { let (r, f) = add::<64>(r9, rdi);
              r9 = r;
              fl = f; }
            // c833f mov rdi,rsi
            rdi = rsi;
            // c8342 mov rsi,r9
            rsi = r9;
            // c8345 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8347 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8349 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c834c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c834d jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc8352 => {
            // block c8352
            #[cfg(feature = "trace")] eprintln!("f_000c8298 pc c8352 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8352 shr r8d,0x2
            { let (r, f) = shr::<32>((r8 as u32 as u64), 0x2u64, fl); r8 = (r) as u32 as u64; fl = f; }
            // c8356 imul r8d,eax
            { let (r, f) = imul::<32>((r8 as u32 as u64), (rax as u32 as u64)); r8 = (r) as u32 as u64; fl = f; }
            // c835a add r9,r8
            { let (r, f) = add::<64>(r9, r8);
              r9 = r;
              fl = f; }
            // c835d mov rdi,rsi
            rdi = rsi;
            // c8360 mov rsi,r9
            rsi = r9;
            // c8363 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8365 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // c8367 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c836a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c836b jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8168 size=304
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c8168<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8168;
    loop {
        match pc {
        0xc8168 => {
            // block c8168
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8168 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8169 mov rbp,rsp
            rbp = rsp;
            // c816c mov r11,r8
            r11 = r8;
            // c816f mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // c8172 add edi,0xffffffe6
            { let (r, f) = add::<32>((rdi as u32 as u64), 0xffffffe6u64);
              rdi = (r) as u32 as u64;
              fl = f; }
            // c8175 cmp edi,0x3
            fl = sub::<32>((rdi as u32 as u64), 0x3u64).1;
            // c8178 jbe 0xc8190
            if fl.be() { pc = 0xc8190; continue; }
            pc = 0xc817a;
        }
        0xc817a => {
            // block c817a
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c817a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c817a imul r10d,ecx
            { let (r, f) = imul::<32>((r10 as u32 as u64), (rcx as u32 as u64)); r10 = (r) as u32 as u64; fl = f; }
            // c817e add r11,r10
            { let (r, f) = add::<64>(r11, r10);
              r11 = r;
              fl = f; }
            // c8181 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // c8183 mov rdi,r11
            rdi = r11;
            // c8186 mov rsi,r9
            rsi = r9;
            // c8189 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc818e;
        }
        0xc818e => {
            // block c818e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c818e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c818e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c818f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc8190 => {
            // block c8190
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8190 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8190 lea rax,[rip+0xf1] # 0xc8288
            rax = m.env.code_va(0xc8288);
            // c8197 movsxd rsi,DWORD PTR [rax+rdi*4]
            rsi = sx::<32>(m.r32(rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64;
            // c819b add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // c819e jmp rsi
            pc = (rsi.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xc81a0 => {
            // block c81a0
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81a0 lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c81a3 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c81a8 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c81ac shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c81b0 imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c81b4 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c81b7 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c81ba mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c81c0 mov rdi,r9
            rdi = r9;
            // c81c3 mov rsi,r11
            rsi = r11;
            // c81c6 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c81c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c81ca jmp 0xc75d9
            { let r = { let t = m.env.code_va(0xc75d9); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc81cf => {
            // block c81cf
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81cf test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // c81d1 je 0xc818e
            if fl.e() { pc = 0xc818e; continue; }
            pc = 0xc81d3;
        }
        0xc81d3 => {
            // block c81d3
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81d3 mov r8d,r10d
            r8 = ((r10 as u32 as u64)) as u32 as u64;
            // c81d6 and r8d,0x1
            { let r = (r8 as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // c81da lea rax,[r9+0x2]
            rax = r9.wrapping_add(0x2);
            // c81de mov ecx,ecx
            rcx = ((rcx as u32 as u64)) as u32 as u64;
            // c81e0 mov esi,0xaaaaaaab
            rsi = (0xaaaaaaabu64) as u32 as u64;
            // c81e5 imul rsi,rcx
            { let (r, f) = imul::<64>(rsi, rcx); rsi = r; fl = f; }
            // c81e9 shr rsi,0x21
            { let (r, f) = shr::<64>(rsi, 0x21u64, fl); rsi = r; fl = f; }
            // c81ed imul esi,r10d
            { let (r, f) = imul::<32>((rsi as u32 as u64), (r10 as u32 as u64)); rsi = (r) as u32 as u64; fl = f; }
            // c81f1 add r11,rsi
            { let (r, f) = add::<64>(r11, rsi);
              r11 = r;
              fl = f; }
            // c81f4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // c81f6 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0xc81f8;
        }
        0xc81f8 => {
            // block c81f8
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81f8 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // c81fb jne 0xc8206
            if fl.ne() { pc = 0xc8206; continue; }
            pc = 0xc81fd;
        }
        0xc81fd => {
            // block c81fd
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c81fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c81fd lea rsi,[rax+0x2]
            rsi = rax.wrapping_add(0x2);
            // c8201 mov rdi,rax
            rdi = rax;
            // c8204 jmp 0xc820f
            pc = 0xc820f; continue;
        }
        0xc8206 => {
            // block c8206
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8206 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8206 lea rsi,[r9+rdx*8+0x2]
            rsi = r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x2);
            // c820b lea rdi,[r9+rdx*8]
            rdi = r9.wrapping_add(rdx.wrapping_mul(8));
            pc = 0xc820f;
        }
        0xc820f => {
            // block c820f
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c820f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c820f test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c8212 cmove rdi,rsi
            if fl.e() { rdi = rsi; }
            // c8216 mov si,WORD PTR [rdi]
            rsi = (rsi & !0xffff) | ((m.r16(rdi) as u64) as u16 as u64);
            // c8219 mov WORD PTR [r11+rdx*2],si
            m.w16(r11.wrapping_add(rdx.wrapping_mul(2)), ((rsi as u16 as u64)) as u16);
            // c821e inc rdx
            { let (r, f) = inc::<64>(rdx, fl); rdx = r; fl = f; }
            // c8221 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // c8225 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // c8227 jne 0xc81f8
            if fl.ne() { pc = 0xc81f8; continue; }
            pc = 0xc8229;
        }
        0xc8229 => {
            // block c8229
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c8229 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c8229 jmp 0xc818e
            pc = 0xc818e; continue;
        }
        0xc822e => {
            // block c822e
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c822e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c822e lea eax,[rdx+0x2]
            rax = (rdx.wrapping_add(0x2)) as u32 as u64;
            // c8231 mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8236 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c823a shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c823e imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8242 shl ecx,0x2
            { let (r, f) = shl::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // c8245 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8248 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c824e mov rdi,r9
            rdi = r9;
            // c8251 mov rsi,r11
            rsi = r11;
            // c8254 mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8257 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8258 jmp 0xc77be
            { let r = { let t = m.env.code_va(0xc77be); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
        0xc825d => {
            // block c825d
            #[cfg(feature = "trace")] eprintln!("f_000c8168 pc c825d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c825d mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            // c825f mov ecx,0xaaaaaaab
            rcx = (0xaaaaaaabu64) as u32 as u64;
            // c8264 imul rcx,rax
            { let (r, f) = imul::<64>(rcx, rax); rcx = r; fl = f; }
            // c8268 shr rcx,0x21
            { let (r, f) = shr::<64>(rcx, 0x21u64, fl); rcx = r; fl = f; }
            // c826c imul ecx,r10d
            { let (r, f) = imul::<32>((rcx as u32 as u64), (r10 as u32 as u64)); rcx = (r) as u32 as u64; fl = f; }
            // c8270 add r11,rcx
            { let (r, f) = add::<64>(r11, rcx);
              r11 = r;
              fl = f; }
            // c8273 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // c8279 mov rdi,r9
            rdi = r9;
            // c827c mov rsi,r11
            rsi = r11;
            // c827f mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // c8282 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c8283 jmp 0xc7978
            { let r = { let t = m.env.code_va(0xc7978); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7348 size=560
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7348<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7348;
    loop {
        match pc {
        0xb7348 => {
            // block b7348
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7348 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7348 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7349 mov rbp,rsp
            rbp = rsp;
            // b734c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b734e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7350 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b7352 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b7354 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7355 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // b735c mov r14,rdi
            r14 = rdi;
            // b735f mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // b7362 mov rdi,rsi
            rdi = rsi;
            // b7365 call QWORD PTR [rax+0x18a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x18a8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b736b mov bx,ax
            rbx = (rbx & !0xffff) | (((rax as u16 as u64)) as u16 as u64);
            // b736e lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b7375 lea r13,[rbp-0x140]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // b737c mov rax,QWORD PTR [rip+0x88e4d] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // b7383 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // b7387 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // b738e add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // b7392 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // b7399 mov rsi,r13
            rsi = r13;
            // b739c call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b73a1 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // b73a8 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // b73b0 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // b73b7 mov r14,QWORD PTR [rip+0x89002] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // b73be lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // b73c2 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b73c9 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // b73cd mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b73d4 mov rdi,r13
            rdi = r13;
            // b73d7 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b73dc mov r15,QWORD PTR [rip+0x88fc5] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // b73e3 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // b73e7 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b73ee mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // b73f9 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // b7404 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // b740f mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // b741a mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // b7424 xorps xmm0,xmm0
            x0 = 0;
            // b7427 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // b742b mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // b7433 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7437 mov rdi,r13
            rdi = r13;
            // b743a call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b743f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7443 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7448 mov rax,QWORD PTR [rbp-0x148]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // b744f mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // b7453 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b745a mov ecx,DWORD PTR [rax+rdi*1+0x8]
            rcx = (m.r32(rax.wrapping_add(rdi).wrapping_add(0x8)) as u64) as u32 as u64;
            // b745e and ecx,0xffffffb5
            { let r = (rcx as u32 as u64) & 0xffffffb5u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b7461 or ecx,0x8
            { let r = (rcx as u32 as u64) | 0x8u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // b7464 mov DWORD PTR [rax+rdi*1+0x8],ecx
            m.w32(rax.wrapping_add(rdi).wrapping_add(0x8), ((rcx as u32 as u64)) as u32);
            // b7468 movzx esi,bx
            rsi = ((rbx as u16 as u64)) as u32 as u64;
            // b746b mov rbx,r13
            rbx = r13;
            // b746e call 0xec0aa
            { let r = { let t = m.env.stub_va(0x877); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7473 mov r13,QWORD PTR [rbp-0x150]
            r13 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // b747a mov rdi,r13
            rdi = r13;
            // b747d mov rsi,rbx
            rsi = rbx;
            // b7480 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7485 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b748c mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b7493 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b749a lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b74a1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74a6 mov rdi,rbx
            rdi = rbx;
            // b74a9 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74ae mov rsi,QWORD PTR [rip+0x88d53] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b74b5 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b74b9 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b74c0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74c5 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b74cc call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b74d1 mov rax,r13
            rax = r13;
            // b74d4 add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // b74db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b74dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b74de pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b74e0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b74e2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b74e4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b74e5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb74e6 => {
            // block b74e6
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b74e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b74e6 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b74ed mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // b74f4 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // b74fb mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // b7502 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b7509 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b750e mov rdi,rbx
            rdi = rbx;
            // b7511 jmp 0xb7544
            pc = 0xb7544; continue;
        }
        0xb7513 => {
            // block b7513
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7513 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7513 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b751a jmp 0xb7560
            pc = 0xb7560; continue;
        }
        0xb751c => {
            // block b751c
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b751c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b751c mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b7523 jmp 0xb7549
            pc = 0xb7549; continue;
        }
        0xb7525 => {
            // block b7525
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7525 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7525 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // b752c lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b7530 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7535 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // b753c call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7541 mov rdi,r13
            rdi = r13;
            pc = 0xb7544;
        }
        0xb7544 => {
            // block b7544
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7544 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7544 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7549;
        }
        0xb7549 => {
            // block b7549
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7549 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7549 mov rsi,QWORD PTR [rip+0x88cb8] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // b7550 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // b7554 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // b755b call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb7560;
        }
        0xb7560 => {
            // block b7560
            #[cfg(feature = "trace")] eprintln!("f_000b7348 pc b7560 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7560 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // b7567 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b756c mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // b7573 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xae8a6 size=604
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ae8a6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xae8a6;
    loop {
        match pc {
        0xae8a6 => {
            // block ae8a6
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc ae8a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae8a6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ae8a7 mov rbp,rsp
            rbp = rsp;
            // ae8aa push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ae8ac push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ae8ae push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ae8b0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ae8b2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ae8b3 sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // ae8ba mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // ae8bc mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // ae8c3 lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // ae8ca lea r13,[rbp-0x140]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // ae8d1 mov rax,QWORD PTR [rip+0x918f8] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // ae8d8 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // ae8dc mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // ae8e3 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // ae8e7 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // ae8ee mov rsi,r13
            rsi = r13;
            // ae8f1 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae8f6 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // ae8fe mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // ae905 mov r14,QWORD PTR [rip+0x91ab4] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // ae90c lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // ae910 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // ae917 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // ae91b mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // ae922 mov rdi,r13
            rdi = r13;
            // ae925 call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae92a mov r15,QWORD PTR [rip+0x91a77] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // ae931 add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // ae935 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // ae93c mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // ae947 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // ae952 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // ae95d mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // ae968 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // ae972 xorps xmm0,xmm0
            x0 = 0;
            // ae975 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // ae979 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // ae981 lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ae985 mov rdi,r13
            rdi = r13;
            // ae988 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae98d lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ae991 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae996 lea rsi,[rip+0x863a4] # 0x134d41
            rsi = m.env.data_va(300302);
            // ae99d lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ae9a4 mov edx,0x22
            rdx = (0x22u64) as u32 as u64;
            // ae9a9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae9ae and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ae9b1 cmp ebx,0xc
            fl = sub::<32>((rbx as u32 as u64), 0xcu64).1;
            // ae9b4 jne 0xae9d0
            if fl.ne() { pc = 0xae9d0; continue; }
            pc = 0xae9b6;
        }
        0xae9b6 => {
            // block ae9b6
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc ae9b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae9b6 lea rsi,[rip+0x863a7] # 0x134d64
            rsi = m.env.data_va(300337);
            // ae9bd lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ae9c4 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ae9c9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae9ce jmp 0xae9ed
            pc = 0xae9ed; continue;
        }
        0xae9d0 => {
            // block ae9d0
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc ae9d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae9d0 cmp ebx,0xd
            fl = sub::<32>((rbx as u32 as u64), 0xdu64).1;
            // ae9d3 jne 0xaea16
            if fl.ne() { pc = 0xaea16; continue; }
            pc = 0xae9d5;
        }
        0xae9d5 => {
            // block ae9d5
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc ae9d5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae9d5 lea rsi,[rip+0x8638b] # 0x134d67
            rsi = m.env.data_va(300340);
            // ae9dc lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ae9e3 mov edx,0x2
            rdx = (0x2u64) as u32 as u64;
            // ae9e8 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xae9ed;
        }
        0xae9ed => {
            // block ae9ed
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc ae9ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae9ed lea rsi,[rip+0x86376] # 0x134d6a
            rsi = m.env.data_va(300343);
            // ae9f4 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ae9fb mov edx,0x21
            rdx = (0x21u64) as u32 as u64;
            // aea00 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea05 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // aea0c mov rsi,r13
            rsi = r13;
            // aea0f call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea14 jmp 0xaea2e
            pc = 0xaea2e; continue;
        }
        0xaea16 => {
            // block aea16
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aea16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aea16 lea rsi,[rip+0x8612f] # 0x134b4c
            rsi = m.env.data_va(299801);
            // aea1d mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // aea22 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // aea29 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaea2e;
        }
        0xaea2e => {
            // block aea2e
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aea2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aea2e mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // aea35 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // aea3c mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // aea43 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // aea4a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea4f mov rdi,r13
            rdi = r13;
            // aea52 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea57 mov rsi,QWORD PTR [rip+0x917aa] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // aea5e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // aea62 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // aea69 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea6e lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // aea75 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea7a mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // aea81 add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // aea88 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // aea89 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // aea8b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // aea8d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // aea8f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // aea91 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // aea92 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xaea93 => {
            // block aea93
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aea93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aea93 mov rbx,rax
            rbx = rax;
            // aea96 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // aea9d mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // aeaa4 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // aeaab jmp 0xaeac3
            pc = 0xaeac3; continue;
        }
        0xaeaad => {
            // block aeaad
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aeaad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeaad mov rbx,rax
            rbx = rax;
            // aeab0 jmp 0xaeaee
            pc = 0xaeaee; continue;
        }
        0xaeab2 => {
            // block aeab2
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aeab2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeab2 mov rbx,rax
            rbx = rax;
            // aeab5 jmp 0xaead7
            pc = 0xaead7; continue;
        }
        0xaeab7 => {
            // block aeab7
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aeab7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeab7 mov rbx,rax
            rbx = rax;
            // aeaba lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // aeabe call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaeac3;
        }
        0xaeac3 => {
            // block aeac3
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aeac3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeac3 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // aeaca call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeacf mov rdi,r13
            rdi = r13;
            // aead2 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaead7;
        }
        0xaead7 => {
            // block aead7
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aead7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aead7 mov rsi,QWORD PTR [rip+0x9172a] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // aeade add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // aeae2 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // aeae9 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xaeaee;
        }
        0xaeaee => {
            // block aeaee
            #[cfg(feature = "trace")] eprintln!("f_000ae8a6 pc aeaee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aeaee lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // aeaf5 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aeafa mov rdi,rbx
            rdi = rbx;
            // aeafd call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7d3c size=644
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007d3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7d3c;
    loop {
        match pc {
        0x7d3c => {
            // block 7d3c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7d3d mov rbp,rsp
            rbp = rsp;
            // 7d40 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7d42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7d44 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7d46 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7d48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7d49 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7d4d mov r12,rsi
            r12 = rsi;
            // 7d50 mov r13,rdi
            r13 = rdi;
            // 7d53 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7d57 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 7d5c sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 7d5f sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d63 cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d6a jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d70;
        }
        0x7d70 => {
            // block 7d70
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d70 mov rsi,QWORD PTR [rdx+0x8]
            rsi = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 7d74 sub rsi,QWORD PTR [rdx]
            { let (r, f) = sub::<64>(rsi, m.r64(rdx) as u64);
              rsi = r;
              fl = f; }
            // 7d77 sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d7b cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d82 jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d88;
        }
        0x7d88 => {
            // block 7d88
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d88 mov rsi,rcx
            rsi = rcx;
            // 7d8b mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 7d8f sub rcx,QWORD PTR [rsi]
            { let (r, f) = sub::<64>(rcx, m.r64(rsi) as u64);
              rcx = r;
              fl = f; }
            // 7d92 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 7d96 cmp rcx,0x400
            fl = sub::<64>(rcx, 0x400u64).1;
            // 7d9d jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7da3;
        }
        0x7da3 => {
            // block 7da3
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7da3 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 7da7 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 7dab mov ebx,0x8
            rbx = (0x8u64) as u32 as u64;
            // 7db0 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 7db6 movsd xmm1,QWORD PTR [rip+0xf6b1a] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7dbe mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7dc4 jmp 0x7dd9
            pc = 0x7dd9; continue;
        }
        0x7dc6 => {
            // block 7dc6
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc6 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7dca add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 7dce inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 7dd1 movsd xmm1,QWORD PTR [rip+0xf6aff] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            pc = 0x7dd9;
        }
        0x7dd9 => {
            // block 7dd9
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd9 movsd xmm0,QWORD PTR [rax+rbx*1-0x8]
            x0 = m.r64(rax.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7ddf addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7de3 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7de8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7dea mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7def js 0x7dfd
            if fl.s() { pc = 0x7dfd; continue; }
            pc = 0x7df1;
        }
        0x7df1 => {
            // block 7df1
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df1 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7df7 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7dfb mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7dfd;
        }
        0x7dfd => {
            // block 7dfd
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfd lea rsi,[r14+0x1ff]
            rsi = r14.wrapping_add(0x1ff);
            // 7e04 movsd xmm0,QWORD PTR [rax+rbx*1]
            x0 = m.r64(rax.wrapping_add(rbx)) as u128;
            // 7e09 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e0d cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7e14 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e16 js 0x7e24
            if fl.s() { pc = 0x7e24; continue; }
            pc = 0x7e18;
        }
        0x7e18 => {
            // block 7e18
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e18 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e1e cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e22 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7e24;
        }
        0x7e24 => {
            // block 7e24
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e24 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7e27 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7e2a add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e2c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7e30 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7e35 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7e38 mov rdi,r13
            rdi = r13;
            // 7e3b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7e3e cmp r14,0x1ff
            fl = sub::<64>(r14, 0x1ffu64).1;
            // 7e45 ja 0x7e4f
            if fl.a() { pc = 0x7e4f; continue; }
            pc = 0x7e47;
        }
        0x7e47 => {
            // block 7e47
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e49 jne 0x7dc6
            if fl.ne() { pc = 0x7dc6; continue; }
            pc = 0x7e4f;
        }
        0x7e4f => {
            // block 7e4f
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e4f mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7e55 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7e5a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e5c je 0x7efc
            if fl.e() { pc = 0x7efc; continue; }
            pc = 0x7e62;
        }
        0x7e62 => {
            // block 7e62
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e62 mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7e68 mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7e6c;
        }
        0x7e6c => {
            // block 7e6c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7e70 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 7e73 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7e7a movsd xmm1,QWORD PTR [rip+0xf6a56] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7e82 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e86 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e8b test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e8d mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7e92 js 0x7ea0
            if fl.s() { pc = 0x7ea0; continue; }
            pc = 0x7e94;
        }
        0x7e94 => {
            // block 7e94
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e94 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e9a cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e9e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ea0;
        }
        0x7ea0 => {
            // block 7ea0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ea0 lea rsi,[rbx+0x3ff]
            rsi = rbx.wrapping_add(0x3ff);
            // 7ea7 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7ead addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7eb1 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7eb6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7eb8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7eba js 0x7ec8
            if fl.s() { pc = 0x7ec8; continue; }
            pc = 0x7ebc;
        }
        0x7ebc => {
            // block 7ebc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ebc cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7ec2 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7ec6 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ec8;
        }
        0x7ec8 => {
            // block 7ec8
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec8 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7ecb shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7ece add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ed0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7ed4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ed9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7edc mov rdi,r13
            rdi = r13;
            // 7edf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7ee2 cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7ee9 ja 0x7f00
            if fl.a() { pc = 0x7f00; continue; }
            pc = 0x7eeb;
        }
        0x7eeb => {
            // block 7eeb
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eeb add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7eef inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ef2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7ef4 jne 0x7e6c
            if fl.ne() { pc = 0x7e6c; continue; }
            pc = 0x7efa;
        }
        0x7efa => {
            // block 7efa
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efa jmp 0x7f00
            pc = 0x7f00; continue;
        }
        0x7efc => {
            // block 7efc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efc mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7f00;
        }
        0x7f00 => {
            // block 7f00
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f00 movsd xmm1,QWORD PTR [rip+0xf69d0] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7f08 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7f0e mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7f13 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f15 je 0x7fae
            if fl.e() { pc = 0x7fae; continue; }
            pc = 0x7f1b;
        }
        0x7f1b => {
            // block 7f1b
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1b mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            pc = 0x7f21;
        }
        0x7f21 => {
            // block 7f21
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f21 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7f25 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7f2c addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f30 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f35 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f37 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7f3c js 0x7f4a
            if fl.s() { pc = 0x7f4a; continue; }
            pc = 0x7f3e;
        }
        0x7f3e => {
            // block 7f3e
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f3e cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f44 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f48 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f4a;
        }
        0x7f4a => {
            // block 7f4a
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4a lea rsi,[rbx+0x5ff]
            rsi = rbx.wrapping_add(0x5ff);
            // 7f51 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7f57 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f5b cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f60 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7f62 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f64 js 0x7f72
            if fl.s() { pc = 0x7f72; continue; }
            pc = 0x7f66;
        }
        0x7f66 => {
            // block 7f66
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f66 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f6c cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f70 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f72;
        }
        0x7f72 => {
            // block 7f72
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f72 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7f75 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7f78 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f7a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7f7e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7f83 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7f86 mov rdi,r13
            rdi = r13;
            // 7f89 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7f8c cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7f93 ja 0x7fb0
            if fl.a() { pc = 0x7fb0; continue; }
            pc = 0x7f95;
        }
        0x7f95 => {
            // block 7f95
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f95 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7f99 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7f9c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f9e movsd xmm1,QWORD PTR [rip+0xf6932] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7fa6 jne 0x7f21
            if fl.ne() { pc = 0x7f21; continue; }
            pc = 0x7fac;
        }
        0x7fac => {
            // block 7fac
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fac jmp 0x7fb0
            pc = 0x7fb0; continue;
        }
        0x7fae => {
            // block 7fae
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x7fb0;
        }
        0x7fb0 => {
            // block 7fb0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fb0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7fb4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7fb5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7fb7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7fb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7fbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7fbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7fbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xad690 size=692
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ad690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xad690;
    loop {
        match pc {
        0xad690 => {
            // block ad690
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ad691 mov rbp,rsp
            rbp = rsp;
            // ad694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ad696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ad698 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // ad69a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ad69c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ad69d sub rsp,0x128
            { let (r, f) = sub::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // ad6a4 mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // ad6a6 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // ad6ad lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // ad6b4 lea r13,[rbp-0x140]
            r13 = rbp.wrapping_add(0xffff_ffff_ffff_fec0);
            // ad6bb mov rax,QWORD PTR [rip+0x92b0e] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // ad6c2 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // ad6c6 mov QWORD PTR [rbp-0x148],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rcx) as u64);
            // ad6cd add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // ad6d1 mov QWORD PTR [rbp-0xd8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (rax) as u64);
            // ad6d8 mov rsi,r13
            rsi = r13;
            // ad6db call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad6e0 mov QWORD PTR [rbp-0x50],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (0x0u64) as u64);
            // ad6e8 mov DWORD PTR [rbp-0x48],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (0xffffffffu64) as u32);
            // ad6ef mov r14,QWORD PTR [rip+0x92cca] # 0x1403c0
            r14 = m.r64(m.env.data_va(347021)) as u64;
            // ad6f6 lea r12,[r14+0x18]
            r12 = r14.wrapping_add(0x18);
            // ad6fa mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // ad701 add r14,0x40
            { let (r, f) = add::<64>(r14, 0x40u64);
              r14 = r;
              fl = f; }
            // ad705 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // ad70c mov rdi,r13
            rdi = r13;
            // ad70f call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad714 mov r15,QWORD PTR [rip+0x92c8d] # 0x1403a8
            r15 = m.r64(m.env.data_va(346997)) as u64;
            // ad71b add r15,0x10
            { let (r, f) = add::<64>(r15, 0x10u64);
              r15 = r;
              fl = f; }
            // ad71f mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // ad726 mov QWORD PTR [rbp-0xe8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (0x0u64) as u64);
            // ad731 mov QWORD PTR [rbp-0xf0],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x0u64) as u64);
            // ad73c mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // ad747 mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // ad752 mov DWORD PTR [rbp-0xe0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff20), (0x10u64) as u32);
            // ad75c xorps xmm0,xmm0
            x0 = 0;
            // ad75f movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // ad763 mov QWORD PTR [rbp-0x30],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (0x0u64) as u64);
            // ad76b lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ad76f mov rdi,r13
            rdi = r13;
            // ad772 call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad777 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ad77b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad780 lea rsi,[rip+0x87424] # 0x134bab
            rsi = m.env.data_va(299896);
            // ad787 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad78e mov edx,0x1f
            rdx = (0x1fu64) as u32 as u64;
            // ad793 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad798 and ebx,0x1f
            { let r = (rbx as u32 as u64) & 0x1fu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ad79b add ebx,0xffffffee
            { let (r, f) = add::<32>((rbx as u32 as u64), 0xffffffeeu64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // ad79e cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // ad7a1 jbe 0xad7c0
            if fl.be() { pc = 0xad7c0; continue; }
            pc = 0xad7a3;
        }
        0xad7a3 => {
            // block ad7a3
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad7a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad7a3 lea rsi,[rip+0x873a2] # 0x134b4c
            rsi = m.env.data_va(299801);
            // ad7aa mov edx,0x15
            rdx = (0x15u64) as u32 as u64;
            // ad7af mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // ad7b6 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad7bb jmp 0xad85d
            pc = 0xad85d; continue;
        }
        0xad7c0 => {
            // block ad7c0
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad7c0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad7c0 lea rax,[rip+0x16d] # 0xad934
            rax = m.env.code_va(0xad934);
            // ad7c7 movsxd rcx,DWORD PTR [rax+rbx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
            // ad7cb add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // ad7ce jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xad7d0 => {
            // block ad7d0
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad7d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad7d0 lea rsi,[rip+0x873f4] # 0x134bcb
            rsi = m.env.data_va(299928);
            // ad7d7 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad7de mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // ad7e3 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad7e8 jmp 0xad836
            pc = 0xad836; continue;
        }
        0xad7ea => {
            // block ad7ea
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad7ea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad7ea lea rsi,[rip+0x873df] # 0x134bd0
            rsi = m.env.data_va(299933);
            // ad7f1 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad7f8 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // ad7fd call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad802 jmp 0xad836
            pc = 0xad836; continue;
        }
        0xad804 => {
            // block ad804
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad804 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad804 lea rsi,[rip+0x873ca] # 0x134bd5
            rsi = m.env.data_va(299938);
            // ad80b lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad812 mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // ad817 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad81c jmp 0xad836
            pc = 0xad836; continue;
        }
        0xad81e => {
            // block ad81e
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad81e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad81e lea rsi,[rip+0x873b5] # 0x134bda
            rsi = m.env.data_va(299943);
            // ad825 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad82c mov edx,0x4
            rdx = (0x4u64) as u32 as u64;
            // ad831 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xad836;
        }
        0xad836 => {
            // block ad836
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad836 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad836 lea rsi,[rip+0x873a2] # 0x134bdf
            rsi = m.env.data_va(299948);
            // ad83d lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad844 mov edx,0x1d
            rdx = (0x1du64) as u32 as u64;
            // ad849 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad84e mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // ad855 mov rsi,r13
            rsi = r13;
            // ad858 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xad85d;
        }
        0xad85d => {
            // block ad85d
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad85d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad85d mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // ad864 mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // ad86b mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // ad872 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // ad879 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad87e mov rdi,r13
            rdi = r13;
            // ad881 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad886 mov rsi,QWORD PTR [rip+0x9297b] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ad88d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ad891 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad898 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad89d lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // ad8a4 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad8a9 mov rax,QWORD PTR [rbp-0x150]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // ad8b0 add rsp,0x128
            { let (r, f) = add::<64>(rsp, 0x128u64);
              rsp = r;
              fl = f; }
            // ad8b7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ad8b8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ad8ba pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // ad8bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ad8be pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ad8c0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ad8c1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xad8c2 => {
            // block ad8c2
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad8c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8c2 mov rbx,rax
            rbx = rax;
            // ad8c5 mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // ad8cc mov QWORD PTR [rbp-0xd8],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff28), (r14) as u64);
            // ad8d3 mov QWORD PTR [rbp-0x140],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (r15) as u64);
            // ad8da jmp 0xad8f2
            pc = 0xad8f2; continue;
        }
        0xad8dc => {
            // block ad8dc
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8dc mov rbx,rax
            rbx = rax;
            // ad8df jmp 0xad91d
            pc = 0xad91d; continue;
        }
        0xad8e1 => {
            // block ad8e1
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad8e1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8e1 mov rbx,rax
            rbx = rax;
            // ad8e4 jmp 0xad906
            pc = 0xad906; continue;
        }
        0xad8e6 => {
            // block ad8e6
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad8e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8e6 mov rbx,rax
            rbx = rax;
            // ad8e9 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // ad8ed call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xad8f2;
        }
        0xad8f2 => {
            // block ad8f2
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad8f2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad8f2 lea rdi,[rbp-0x100]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff00);
            // ad8f9 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad8fe mov rdi,r13
            rdi = r13;
            // ad901 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xad906;
        }
        0xad906 => {
            // block ad906
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad906 mov rsi,QWORD PTR [rip+0x928fb] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // ad90d add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // ad911 lea rdi,[rbp-0x148]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_feb8);
            // ad918 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xad91d;
        }
        0xad91d => {
            // block ad91d
            #[cfg(feature = "trace")] eprintln!("f_000ad690 pc ad91d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ad91d lea rdi,[rbp-0xd8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff28);
            // ad924 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad929 mov rdi,rbx
            rdi = rbx;
            // ad92c call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

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
            if fl.ne() { pc = 0x691b5; continue; }
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
            if fl.e() { pc = 0x691b5; continue; }
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
            { let r = { let t = m.env.stub_va(0x487); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69177 cmp rax,QWORD PTR [rbp-0x38]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64).1;
            // 6917b jne 0x69188
            if fl.ne() { pc = 0x69188; continue; }
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
            { let r = { let t = m.env.stub_va(0x475); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.e() { pc = 0x6919f; continue; }
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
            if fl.ne() { pc = 0x69191; continue; }
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
            if fl.ne() { pc = 0x691a2; continue; }
            pc = 0x691af;
        }
        0x691af => {
            // block 691af
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 691af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 691af cmp r13,QWORD PTR [rbp-0x30]
            fl = sub::<64>(r13, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64).1;
            // 691b3 jne 0x69168
            if fl.ne() { pc = 0x69168; continue; }
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
            { let r = { let t = m.env.stub_va(0x46f); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 691f2 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 691f6 test rax,rax
            fl = logic::<64>(rax & rax);
            // 691f9 je 0x69208
            if fl.e() { pc = 0x69208; continue; }
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
            if fl.ne() { pc = 0x691fb; continue; }
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
            if fl.ne() { pc = 0x6920b; continue; }
            pc = 0x69217;
        }
        0x69217 => {
            // block 69217
            #[cfg(feature = "trace")] eprintln!("f_0006911e pc 69217 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69217 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 6921a jne 0x691e3
            if fl.ne() { pc = 0x691e3; continue; }
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
            { let r = { let t = m.env.stub_va(0x48d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6923d mov rdi,rbx
            rdi = rbx;
            // 69240 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
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
            { let r = { let t = m.env.stub_va(0x3a3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69013 mov rbx,rax
            rbx = rax;
            // 69016 add r15,0x1a0
            { let (r, f) = add::<64>(r15, 0x1a0u64);
              r15 = r;
              fl = f; }
            // 6901d cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69020 je 0x6910d
            if fl.e() { pc = 0x6910d; continue; }
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
            if fl.ne() { pc = 0x69057; continue; }
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
            if fl.ne() { pc = 0x6910d; continue; }
            pc = 0x6906e;
        }
        0x6906e => {
            // block 6906e
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6906e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6906e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69070 jne 0x6907b
            if fl.ne() { pc = 0x6907b; continue; }
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
            if fl.ne() { pc = 0x690a9; continue; }
            pc = 0x6908d;
        }
        0x6908d => {
            // block 6908d
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 6908d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6908d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 69090 je 0x690d5
            if fl.e() { pc = 0x690d5; continue; }
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
            if fl.ne() { pc = 0x6910d; continue; }
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
            if fl.ne() { pc = 0x69092; continue; }
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
            if fl.e() { pc = 0x690d5; continue; }
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
            { let r = { let t = m.env.stub_va(0x9e5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
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
            if fl.ne() { pc = 0x6910d; continue; }
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
            if fl.e() { pc = 0x6910f; continue; }
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
            if fl.e() { pc = 0x690f5; continue; }
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
            if fl.ne() { pc = 0x690e8; continue; }
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
            if fl.ne() { pc = 0x690f8; continue; }
            pc = 0x69104;
        }
        0x69104 => {
            // block 69104
            #[cfg(feature = "trace")] eprintln!("f_00068fea pc 69104 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69104 cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 69107 jne 0x69048
            if fl.ne() { pc = 0x69048; continue; }
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

/// parity: P2v @0x68428 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068428<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68428;
    loop {
        match pc {
        0x68428 => {
            // block 68428
            #[cfg(feature = "trace")] eprintln!("f_00068428 pc 68428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68428 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68429 mov rbp,rsp
            rbp = rsp;
            // 6842c mov rax,rsi
            rax = rsi;
            // 6842f mov ecx,edi
            rcx = ((rdi as u32 as u64)) as u32 as u64;
            // 68431 lea rdi,[rip+0x15ff48] # 0x1c8380
            rdi = m.env.data_va(904013);
            // 68438 mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // 6843a mov rdx,rax
            rdx = rax;
            // 6843d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6843e jmp 0x68fea
            { let r = { let t = m.env.code_va(0x68fea); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68444 size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068444<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68444;
    loop {
        match pc {
        0x68444 => {
            // block 68444
            #[cfg(feature = "trace")] eprintln!("f_00068444 pc 68444 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68444 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68445 mov rbp,rsp
            rbp = rsp;
            // 68448 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68449 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6844a mov rbx,rdi
            rbx = rdi;
            // 6844d lea rsi,[rip+0x15ff2c] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 68454 call 0x6911e
            { let r = { let t = m.env.code_va(0x6911e); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68459 mov rax,rbx
            rax = rbx;
            // 6845c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68460 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68461 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68462 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68464 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068464<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68464;
    loop {
        match pc {
        0x68464 => {
            // block 68464
            #[cfg(feature = "trace")] eprintln!("f_00068464 pc 68464 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68464 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68465 mov rbp,rsp
            rbp = rsp;
            // 68468 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68469 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6846a mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 6846c mov rbx,rdi
            rbx = rdi;
            // 6846f lea rsi,[rip+0x15ff0a] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 68476 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 68478 call 0x69246
            { let r = { let t = m.env.code_va(0x69246); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6847d mov rax,rbx
            rax = rbx;
            // 68480 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68484 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68485 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68486 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18ca1 size=682
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018ca1<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18ca1;
    loop {
        match pc {
        0x18ca1 => {
            // block 18ca1
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18ca1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18ca1 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18ca2 mov rbp,rsp
            rbp = rsp;
            // 18ca5 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18ca7 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18ca9 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 18cab push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18cac sub rsp,0x30
            { let (r, f) = sub::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18cb0 mov r15,rsi
            r15 = rsi;
            // 18cb3 mov r14,rdi
            r14 = rdi;
            // 18cb6 lea rsi,[rip+0x1116e5] # 0x12a3a2
            rsi = m.env.data_va(256879);
            // 18cbd mov edx,0x14
            rdx = (0x14u64) as u32 as u64;
            // 18cc2 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cc7 mov rbx,rax
            rbx = rax;
            // 18cca mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18ccd mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18cd1 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18cd4 lea r12,[rbp-0x48]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18cd8 mov rdi,r12
            rdi = r12;
            // 18cdb call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ce0 mov rsi,QWORD PTR [rip+0x1274a9] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18ce7 mov rdi,r12
            rdi = r12;
            // 18cea call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18cef mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18cf2 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18cf6 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18cfb mov rdi,rax
            rdi = rax;
            // 18cfe call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d00 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d03 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18d07 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d0c movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d10 mov rdi,rbx
            rdi = rbx;
            // 18d13 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d18 mov rdi,rbx
            rdi = rbx;
            // 18d1b call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d20 lea rsi,[rip+0x111690] # 0x12a3b7
            rsi = m.env.data_va(256900);
            // 18d27 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18d2c mov rdi,rbx
            rdi = rbx;
            // 18d2f call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d34 mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 18d37 mov rdi,rax
            rdi = rax;
            // 18d3a call 0xec09e
            { let r = { let t = m.env.stub_va(0x86b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d3f mov rbx,rax
            rbx = rax;
            // 18d42 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18d45 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18d49 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18d4c lea r12,[rbp-0x40]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d50 mov rdi,r12
            rdi = r12;
            // 18d53 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d58 mov rsi,QWORD PTR [rip+0x127431] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18d5f mov rdi,r12
            rdi = r12;
            // 18d62 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d67 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18d6a mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18d6e mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18d73 mov rdi,rax
            rdi = rax;
            // 18d76 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d78 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18d7b lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18d7f call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d84 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18d88 mov rdi,rbx
            rdi = rbx;
            // 18d8b call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d90 mov rdi,rbx
            rdi = rbx;
            // 18d93 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18d98 lea rsi,[rip+0x11162a] # 0x12a3c9
            rsi = m.env.data_va(256918);
            // 18d9f mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18da4 mov rdi,rbx
            rdi = rbx;
            // 18da7 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dac mov esi,DWORD PTR [r15+0x4]
            rsi = (m.r32(r15.wrapping_add(0x4)) as u64) as u32 as u64;
            // 18db0 mov rdi,rax
            rdi = rax;
            // 18db3 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18db8 mov rbx,rax
            rbx = rax;
            // 18dbb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18dbe mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18dc2 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18dc5 lea r12,[rbp-0x38]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18dc9 mov rdi,r12
            rdi = r12;
            // 18dcc call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dd1 mov rsi,QWORD PTR [rip+0x1273b8] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18dd8 mov rdi,r12
            rdi = r12;
            // 18ddb call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18de0 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18de3 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18de7 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18dec mov rdi,rax
            rdi = rax;
            // 18def call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18df1 mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18df4 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18df8 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18dfd movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e01 mov rdi,rbx
            rdi = rbx;
            // 18e04 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e09 mov rdi,rbx
            rdi = rbx;
            // 18e0c call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e11 lea rsi,[rip+0x1115c3] # 0x12a3db
            rsi = m.env.data_va(256936);
            // 18e18 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e1d mov rdi,rbx
            rdi = rbx;
            // 18e20 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e25 mov esi,DWORD PTR [r15+0x8]
            rsi = (m.r32(r15.wrapping_add(0x8)) as u64) as u32 as u64;
            // 18e29 mov rdi,rax
            rdi = rax;
            // 18e2c call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e31 mov rbx,rax
            rbx = rax;
            // 18e34 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18e37 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18e3b add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18e3e lea r12,[rbp-0x30]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e42 mov rdi,r12
            rdi = r12;
            // 18e45 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e4a mov rsi,QWORD PTR [rip+0x12733f] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18e51 mov rdi,r12
            rdi = r12;
            // 18e54 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e59 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18e5c mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18e60 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18e65 mov rdi,rax
            rdi = rax;
            // 18e68 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e6a mov r12b,al
            r12 = (r12 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18e6d lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18e71 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e76 movsx esi,r12b
            rsi = (sx::<8>((r12 as u8 as u64)) as u64) as u32 as u64;
            // 18e7a mov rdi,rbx
            rdi = rbx;
            // 18e7d call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e82 mov rdi,rbx
            rdi = rbx;
            // 18e85 call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e8a lea rsi,[rip+0x11155c] # 0x12a3ed
            rsi = m.env.data_va(256954);
            // 18e91 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 18e96 mov rdi,rbx
            rdi = rbx;
            // 18e99 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18e9e mov esi,DWORD PTR [r15+0xc]
            rsi = (m.r32(r15.wrapping_add(0xc)) as u64) as u32 as u64;
            // 18ea2 mov rdi,rax
            rdi = rax;
            // 18ea5 call 0xec098
            { let r = { let t = m.env.stub_va(0x865); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eaa mov rbx,rax
            rbx = rax;
            // 18ead mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 18eb0 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 18eb4 add rsi,rbx
            { let (r, f) = add::<64>(rsi, rbx);
              rsi = r;
              fl = f; }
            // 18eb7 lea r15,[rbp-0x28]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18ebb mov rdi,r15
            rdi = r15;
            // 18ebe call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ec3 mov rsi,QWORD PTR [rip+0x1272c6] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // 18eca mov rdi,r15
            rdi = r15;
            // 18ecd call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ed2 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // 18ed5 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 18ed9 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // 18ede mov rdi,rax
            rdi = rax;
            // 18ee1 call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18ee3 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 18ee6 lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 18eea call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18eef movsx esi,r15b
            rsi = (sx::<8>((r15 as u8 as u64)) as u64) as u32 as u64;
            // 18ef3 mov rdi,rbx
            rdi = rbx;
            // 18ef6 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18efb mov rdi,rbx
            rdi = rbx;
            // 18efe call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f03 mov rax,r14
            rax = r14;
            // 18f06 add rsp,0x30
            { let (r, f) = add::<64>(rsp, 0x30u64);
              rsp = r;
              fl = f; }
            // 18f0a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18f0b pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 18f0d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18f0f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18f11 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18f12 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18f13 => {
            // block 18f13
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f13 mov rbx,rax
            rbx = rax;
            // 18f16 lea rdi,[rbp-0x48]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18f1a jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f1c => {
            // block 18f1c
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f1c mov rbx,rax
            rbx = rax;
            // 18f1f lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // 18f23 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f25 => {
            // block 18f25
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f25 mov rbx,rax
            rbx = rax;
            // 18f28 lea rdi,[rbp-0x38]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 18f2c jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f2e => {
            // block 18f2e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f2e mov rbx,rax
            rbx = rax;
            // 18f31 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18f35 jmp 0x18f3e
            pc = 0x18f3e; continue;
        }
        0x18f37 => {
            // block 18f37
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f37 mov rbx,rax
            rbx = rax;
            // 18f3a lea rdi,[rbp-0x28]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x18f3e;
        }
        0x18f3e => {
            // block 18f3e
            #[cfg(feature = "trace")] eprintln!("f_00018ca1 pc 18f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f3e call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f43 mov rdi,rbx
            rdi = rbx;
            // 18f46 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18f4b size=57
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00018f4b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18f4b;
    loop {
        match pc {
        0x18f4b => {
            // block 18f4b
            #[cfg(feature = "trace")] eprintln!("f_00018f4b pc 18f4b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f4b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18f4c mov rbp,rsp
            rbp = rsp;
            // 18f4f push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 18f51 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 18f53 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 18f54 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 18f55 mov r15,rsi
            r15 = rsi;
            // 18f58 mov r14,rdi
            r14 = rdi;
            // 18f5b mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 18f5e jmp 0x18f6f
            pc = 0x18f6f; continue;
        }
        0x18f60 => {
            // block 18f60
            #[cfg(feature = "trace")] eprintln!("f_00018f4b pc 18f60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f60 mov rdi,r14
            rdi = r14;
            // 18f63 mov rsi,rbx
            rsi = rbx;
            // 18f66 call 0x18ca1
            { let r = { let t = m.env.code_va(0x18ca1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18f6b add rbx,0x18
            { let (r, f) = add::<64>(rbx, 0x18u64);
              rbx = r;
              fl = f; }
            pc = 0x18f6f;
        }
        0x18f6f => {
            // block 18f6f
            #[cfg(feature = "trace")] eprintln!("f_00018f4b pc 18f6f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f6f cmp rbx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 18f73 jne 0x18f60
            if fl.ne() { pc = 0x18f60; continue; }
            pc = 0x18f75;
        }
        0x18f75 => {
            // block 18f75
            #[cfg(feature = "trace")] eprintln!("f_00018f4b pc 18f75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18f75 mov rax,r14
            rax = r14;
            // 18f78 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 18f7c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 18f7d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18f7f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18f81 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18f82 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa5a20 size=754
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a5a20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa5a20;
    loop {
        match pc {
        0xa5a20 => {
            // block a5a20
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5a20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5a20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a5a21 mov rbp,rsp
            rbp = rsp;
            // a5a24 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // a5a26 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // a5a28 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // a5a2a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // a5a2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // a5a2d sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a5a34 mov r13d,ecx
            r13 = ((rcx as u32 as u64)) as u32 as u64;
            // a5a37 mov r14,rdi
            r14 = rdi;
            // a5a3a lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5a41 lea r15,[rbp-0x150]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_feb0);
            // a5a48 mov rax,QWORD PTR [rip+0x9a781] # 0x1401d0
            rax = m.r64(m.env.data_va(346525)) as u64;
            // a5a4f lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // a5a53 mov QWORD PTR [rbp-0x158],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rcx) as u64);
            // a5a5a add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // a5a5e mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5a65 mov rsi,r15
            rsi = r15;
            // a5a68 call 0xec0f8
            { let r = { let t = m.env.stub_va(0x8c5); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5a6d mov QWORD PTR [rbp-0x60],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (0x0u64) as u64);
            // a5a75 mov DWORD PTR [rbp-0x58],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (0xffffffffu64) as u32);
            // a5a7c mov rbx,QWORD PTR [rip+0x9a93d] # 0x1403c0
            rbx = m.r64(m.env.data_va(347021)) as u64;
            // a5a83 lea r12,[rbx+0x18]
            r12 = rbx.wrapping_add(0x18);
            // a5a87 mov QWORD PTR [rbp-0x158],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r12) as u64);
            // a5a8e add rbx,0x40
            { let (r, f) = add::<64>(rbx, 0x40u64);
              rbx = r;
              fl = f; }
            // a5a92 mov QWORD PTR [rbp-0xe8],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rbx) as u64);
            // a5a99 mov rdi,r15
            rdi = r15;
            // a5a9c call 0xec0c2
            { let r = { let t = m.env.stub_va(0x88f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5aa1 mov QWORD PTR [rbp-0x168],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rbx) as u64);
            // a5aa8 mov QWORD PTR [rbp-0x160],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (r12) as u64);
            // a5aaf mov rbx,QWORD PTR [rip+0x9a8f2] # 0x1403a8
            rbx = m.r64(m.env.data_va(346997)) as u64;
            // a5ab6 add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // a5aba mov QWORD PTR [rbp-0x150],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rbx) as u64);
            // a5ac1 mov QWORD PTR [rbp-0xf8],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff08), (0x0u64) as u64);
            // a5acc mov QWORD PTR [rbp-0x100],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff00), (0x0u64) as u64);
            // a5ad7 mov QWORD PTR [rbp-0x108],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef8), (0x0u64) as u64);
            // a5ae2 mov QWORD PTR [rbp-0x110],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (0x0u64) as u64);
            // a5aed mov DWORD PTR [rbp-0xf0],0x10
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff10), (0x10u64) as u32);
            // a5af7 xorps xmm0,xmm0
            x0 = 0;
            // a5afa movaps XMMWORD PTR [rbp-0x50],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), x0);
            // a5afe mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // a5b06 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5b0a mov rdi,r15
            rdi = r15;
            // a5b0d call 0xebaa4
            { let r = { let t = m.env.stub_va(0x271); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b12 mov QWORD PTR [rbp-0x170],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe90), (rbx) as u64);
            // a5b19 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5b1d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b22 lea rsi,[rip+0x8e506] # 0x13402f
            rsi = m.env.data_va(296956);
            // a5b29 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5b30 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // a5b35 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b3a test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // a5b3d sets cl
            rcx = (rcx & !0xff) | (((fl.s() as u64)) as u8 as u64);
            // a5b40 lea rdx,[rip+0x8e4f7] # 0x13403e
            rdx = m.env.data_va(296971);
            // a5b47 lea rsi,[rip+0x8e4f5] # 0x134043
            rsi = m.env.data_va(296976);
            // a5b4e cmovs rsi,rdx
            if fl.s() { rsi = rdx; }
            // a5b52 movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // a5b55 add rdx,0x3
            { let (r, f) = add::<64>(rdx, 0x3u64);
              rdx = r;
              fl = f; }
            // a5b59 mov rdi,rax
            rdi = rax;
            // a5b5c call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b61 mov r12,rax
            r12 = rax;
            // a5b64 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // a5b68 mov rsi,QWORD PTR [rax-0x18]
            rsi = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // a5b6c add rsi,r12
            { let (r, f) = add::<64>(rsi, r12);
              rsi = r;
              fl = f; }
            // a5b6f lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5b73 call 0xebfd2
            { let r = { let t = m.env.stub_va(0x79f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b78 mov ebx,r13d
            rbx = ((r13 as u32 as u64)) as u32 as u64;
            // a5b7b mov r13,r14
            r13 = r14;
            // a5b7e mov rsi,QWORD PTR [rip+0x9a60b] # 0x140190
            rsi = m.r64(m.env.data_va(346461)) as u64;
            // a5b85 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5b89 call 0xebfcc
            { let r = { let t = m.env.stub_va(0x799); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b8e mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // a5b91 mov rcx,QWORD PTR [rcx+0x38]
            rcx = m.r64(rcx.wrapping_add(0x38)) as u64;
            // a5b95 mov esi,0xa
            rsi = (0xau64) as u32 as u64;
            // a5b9a mov rdi,rax
            rdi = rax;
            // a5b9d call rcx
            { let r = { let t = rcx; m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5b9f mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // a5ba2 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5ba6 call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bab movsx esi,r14b
            rsi = (sx::<8>((r14 as u8 as u64)) as u64) as u32 as u64;
            // a5baf mov rdi,r12
            rdi = r12;
            // a5bb2 call 0xec068
            { let r = { let t = m.env.stub_va(0x835); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bb7 mov rdi,r12
            rdi = r12;
            // a5bba call 0xec06e
            { let r = { let t = m.env.stub_va(0x83b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bbf lea rsi,[rip+0x8e481] # 0x134047
            rsi = m.env.data_va(296980);
            // a5bc6 mov edx,0x7
            rdx = (0x7u64) as u32 as u64;
            // a5bcb mov rdi,r12
            rdi = r12;
            // a5bce call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bd3 and ebx,0x40000000
            { let r = (rbx as u32 as u64) & 0x40000000u64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // a5bd9 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // a5bdc lea rdx,[rip+0x8e46c] # 0x13404f
            rdx = m.env.data_va(296988);
            // a5be3 lea rsi,[rip+0x8e470] # 0x13405a
            rsi = m.env.data_va(296999);
            // a5bea cmovne rsi,rdx
            if fl.ne() { rsi = rdx; }
            // a5bee movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // a5bf1 lea rdx,[rcx+rcx*4+0x5]
            rdx = rcx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x5);
            // a5bf6 mov rdi,rax
            rdi = rax;
            // a5bf9 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5bfe mov rdi,r13
            rdi = r13;
            // a5c01 mov rsi,r15
            rsi = r15;
            // a5c04 call 0xeb8f4
            { let r = { let t = m.env.stub_va(0xc1); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c09 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a5c10 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a5c17 mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a5c1e mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5c25 mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a5c2c mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            // a5c33 lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a5c3a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c3f mov rdi,r15
            rdi = r15;
            // a5c42 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c47 mov rsi,QWORD PTR [rip+0x9a5ba] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a5c4e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a5c52 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5c59 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c5e lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5c65 call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5c6a mov rax,r13
            rax = r13;
            // a5c6d add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // a5c74 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a5c75 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a5c77 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a5c79 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a5c7b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a5c7d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a5c7e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xa5c7f => {
            // block a5c7f
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c7f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c7f mov r14,rax
            r14 = rax;
            // a5c82 jmp 0xa5c90
            pc = 0xa5c90; continue;
        }
        0xa5c84 => {
            // block a5c84
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c84 mov r14,rax
            r14 = rax;
            // a5c87 lea rdi,[rbp-0x30]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffd0);
            // a5c8b call 0xec0e0
            { let r = { let t = m.env.stub_va(0x8ad); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5c90;
        }
        0xa5c90 => {
            // block a5c90
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5c90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5c90 mov rax,QWORD PTR [rbp-0x160]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // a5c97 mov QWORD PTR [rbp-0x158],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rax) as u64);
            // a5c9e mov rax,QWORD PTR [rbp-0x168]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // a5ca5 mov QWORD PTR [rbp-0xe8],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff18), (rax) as u64);
            // a5cac mov rax,QWORD PTR [rbp-0x170]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe90)) as u64;
            // a5cb3 mov QWORD PTR [rbp-0x150],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rax) as u64);
            pc = 0xa5cba;
        }
        0xa5cba => {
            // block a5cba
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cba lea rdi,[rbp-0x110]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef0);
            // a5cc1 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cc6 mov rdi,r15
            rdi = r15;
            // a5cc9 call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5cce;
        }
        0xa5cce => {
            // block a5cce
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cce mov rsi,QWORD PTR [rip+0x9a533] # 0x140208
            rsi = m.r64(m.env.data_va(346581)) as u64;
            // a5cd5 add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // a5cd9 lea rdi,[rbp-0x158]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fea8);
            // a5ce0 call 0xec080
            { let r = { let t = m.env.stub_va(0x84d); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xa5ce5;
        }
        0xa5ce5 => {
            // block a5ce5
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5ce5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5ce5 lea rdi,[rbp-0xe8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff18);
            // a5cec call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cf1 mov rdi,r14
            rdi = r14;
            // a5cf4 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5cf9 mov r14,rax
            r14 = rax;
            // a5cfc jmp 0xa5ce5
            pc = 0xa5ce5; continue;
        }
        0xa5cfe => {
            // block a5cfe
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5cfe mov r14,rax
            r14 = rax;
            // a5d01 jmp 0xa5cce
            pc = 0xa5cce; continue;
        }
        0xa5d03 => {
            // block a5d03
            #[cfg(feature = "trace")] eprintln!("f_000a5a20 pc a5d03 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a5d03 mov r14,rax
            r14 = rax;
            // a5d06 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // a5d0a call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // a5d0f jmp 0xa5cba
            pc = 0xa5cba; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
