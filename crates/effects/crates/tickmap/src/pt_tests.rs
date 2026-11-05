use super::*;

struct H { px: i32, py: i32, co: u32, ci: u32, last: (u64, i32, i32, i32, i32), pre: (i32, i32), npre: u32 }
impl PointHost for H {
    fn checkout_point(&mut self, r: u64, i: i32, c: i32, s: i32, sc: i32) -> (i32, i32) {
        self.co += 1;
        self.last = (r, i, c, s, sc);
        (self.px, self.py)
    }
    fn checkin(&mut self, _r: u64) { self.ci += 1; }
    fn checkout_type(&mut self, _r: u64, _i: i32, _c: i32, _s: i32, _sc: i32) -> (i32, i32) { self.npre += 1; self.pre }
}
fn host(px: f32, py: f32) -> H { H { px: (px * 65536.0) as i32, py: (py * 65536.0) as i32, co: 0, ci: 0, last: (0, 0, 0, 0, 0), pre: (0, 6), npre: 0 } }
fn pinfo() -> Pinfo { Pinfo { time_frames: 0.0, selector: 0, unit_x3: 1.0, unit_d4: 0.0, unit_d5: 0.0 } }
fn hrec() -> PointHostRec {
    PointHostRec { effect_ref: 77, cur_time: 5, time_step: 2, time_scale: 24, size_x: 64, size_y: 32, off_a: 0, off_b: 0, scaled: false, hcv_host_a: 0, hcv_host_b: 0, hcv_host_c: 0, hcv_host_d: 0 }
}
fn rec(nib: u8, ptype: i32) -> ParamRec { ParamRec { ptype, nlanes: 2, kind_byte: 2, subtype_byte: nib, flags_a_byte: 0, flags_b_byte: 0, index: 23, rec_unit_value: 1.0 } }

#[test]
fn default_binding_nibble4_claim_values() {
    let mut h = host(50.0, 50.0);
    let r = map_point_param(&pinfo(), &hrec(), 0, &rec(4, 1), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0].to_bits(), 0x3f100000);
    assert_eq!(r[1].to_bits(), 0xbf880000);
    assert_eq!((h.co, h.ci), (1, 1));
    assert_eq!(h.last, (77, 23, 5, 2, 24));
}

#[test]
fn nonzero_binding_and_offsets() {
    // point (10, 20), offsets a = 4, b = 6: v0 = 10 - 4 = 6, v1 = (32 - 20) + 6 = 18
    let mut hr = hrec();
    hr.off_a = 4;
    hr.off_b = 6;
    let mut h = host(10.0, 20.0);
    let r = map_point_param(&pinfo(), &hr, 0, &rec(4, 2), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0], (6.0f32 - 32.0) * 0.03125);
    assert_eq!(r[1], ((18.0f32 - 16.0) * 0.03125) * 1.0);
    // global 3 skips the offsets
    let mut h = host(10.0, 20.0);
    let r = map_point_param(&pinfo(), &hr, 3, &rec(4, 2), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0], (10.0f32 - 32.0) * 0.03125);
    // an offset beyond 50000 is ignored
    hr.off_a = 50001;
    let mut h = host(10.0, 20.0);
    let r = map_point_param(&pinfo(), &hr, 0, &rec(4, 2), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0], (10.0f32 - 32.0) * 0.03125);
}

#[test]
fn identity_nibble_and_refusals() {
    let mut h = host(1.5, 2.5);
    let r = map_point_param(&pinfo(), &hrec(), 0, &rec(0, 1), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0], 1.5);
    assert_eq!(r[1], 32.0 - 2.5);
    for n in [1u8, 2, 3, 6, 7] {
        assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &rec(n, 1), Some(6), &mut host(0.0, 0.0)), Err(Error::UnitConversion));
    }
    assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &rec(4, 3), Some(6), &mut host(0.0, 0.0)), Err(Error::PointRecType));
    assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &rec(4, 1), Some(5), &mut host(0.0, 0.0)), Err(Error::PointNotRouted));
    let mut hh = host(0.0, 0.0);
    hh.pre = (0, 5);
    assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &rec(4, 1), None, &mut hh), Err(Error::PointNotRouted));
    hh.pre = (0x205, 6);
    assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &rec(4, 1), None, &mut hh), Err(Error::PointPrelude(0x205)));
    // global 9 routes nibble 4 on a non-point object
    assert!(map_point_param(&pinfo(), &hrec(), 9, &rec(4, 1), Some(5), &mut host(0.0, 0.0)).is_ok());
    assert_eq!(map_point_param(&pinfo(), &hrec(), 9, &rec(5, 1), Some(5), &mut host(0.0, 0.0)), Err(Error::PointNotRouted));
    let mut p = pinfo();
    p.selector = 1;
    assert_eq!(map_point_param(&p, &hrec(), 0, &rec(4, 1), Some(6), &mut host(0.0, 0.0)), Err(Error::PointNoTime));
    let mut r = rec(4, 1);
    r.index = -1;
    assert_eq!(map_point_param(&pinfo(), &hrec(), 0, &r, Some(6), &mut host(0.0, 0.0)), Ok(None));
}

#[test]
fn scaled_sizes() {
    let mut hr = hrec();
    hr.scaled = true;
    hr.hcv_host_a = 3;
    hr.hcv_host_b = 2; // 1.5 * 64 = 96 + 0.5 -> 96
    hr.hcv_host_c = 1;
    hr.hcv_host_d = 0; // 1.0 * 32 = 32
    let mut h = host(0.0, 0.0);
    let r = map_point_param(&pinfo(), &hr, 0, &rec(4, 1), Some(6), &mut h).unwrap().unwrap();
    assert_eq!(r[0], (0.0f32 - 48.0) * (2.0f32 / 96.0));
    // non-positive scaled size becomes 1
    hr.hcv_host_a = 0;
    let r = map_point_param(&pinfo(), &hr, 0, &rec(4, 1), Some(6), &mut host(0.0, 0.0)).unwrap().unwrap();
    assert_eq!(r[0], (0.0f32 - 0.5) * 2.0);
}

#[test]
fn global9_nibble3_is_not_point_routed_and_big_sizes_wrap() {
    assert_eq!(map_point_param(&pinfo(), &hrec(), 9, &rec(3, 1), Some(5), &mut host(0.0, 0.0)), Err(Error::PointNotRouted));
    // size 0x7fffffff is 2147483648.0 as f32 and the truncating conversion gives the indefinite value -2^31
    let mut hr = hrec();
    hr.size_x = i32::MAX;
    let r = map_point_param(&pinfo(), &hr, 0, &rec(4, 1), Some(6), &mut host(0.0, 0.0)).unwrap().unwrap();
    // lane 1 = ((32 - 0 - 16) * (2 / x1)) * 1 with x1 = -2^31 is negative (it would be positive without the wrap)
    assert_eq!(r[1], 16.0f32 * (2.0f32 / -2147483648.0f32));
}

fn recorded(sx: i32, sy: i32, px: f32, py: f32, rec_idx: i16) -> ([f32; 2], H) {
    // the claim record: no host list (prelude checkout answers type 6), global 1, scaled bit set with ratios 1/1, offsets 0, d4 = d5 = 1.0
    let mut hr = hrec();
    hr.size_x = sx;
    hr.size_y = sy;
    hr.cur_time = 0;
    hr.time_step = 1;
    hr.scaled = true;
    hr.hcv_host_a = 1;
    hr.hcv_host_b = 1;
    hr.hcv_host_c = 1;
    hr.hcv_host_d = 1;
    let mut r = rec(4, 1);
    r.index = rec_idx;
    let mut p = pinfo();
    p.unit_d4 = 1.0;
    p.unit_d5 = 1.0;
    let mut h = host(px, py);
    let o = map_point_param(&p, &hr, 1, &r, None, &mut h).unwrap().unwrap();
    (o, h)
}

#[test]
fn recorded_claim_bits_64x32_and_50x7() {
    let (o, h) = recorded(64, 32, 50.0, 50.0, 21);
    assert_eq!((o[0].to_bits(), o[1].to_bits()), (0x3f100000, 0xbf880000));
    assert_eq!((h.npre, h.co, h.ci), (1, 1, 1));
    let (o, _) = recorded(50, 7, 50.0, 50.0, 21);
    assert_eq!((o[0].to_bits(), o[1].to_bits()), (0x3f800000, 0xbfee147b));
}

struct GH;
impl Host for GH {
    fn checkout(&mut self, _r: u64, _i: i32, _t: i32, _s: i32, _sc: i32) -> (i32, i32) { (0, 0) }
    fn checkin(&mut self, _r: u64) {}
    fn fetch_no_time(&mut self, _a: i32, _i: i32, _c: u64, _o: &mut [u8; 41]) {}
}
fn wired(global: i32, nib: u8, obj: Option<i32>, kind_byte: u8, h: &mut H, d: &mut [f32]) -> Result<(), Error> {
    let hr = HostRec { effect_ref: 77, time_mult: 1, time_scale: 24, double_flag: 0, size_x: 64, size_y: 32 };
    let hp = hrec();
    let mut r = rec(nib, 1);
    r.kind_byte = kind_byte;
    map_param(&pinfo(), Some(&hr), None, global, &r, 0.5, &mut GH, d, Some(PointCtx { h: &hp, obj_type: obj, host: h }))
}

#[test]
fn map_param_wires_the_point_route() {
    let mut d = [f32::from_bits(0xa5a5a5a5); 4];
    wired(0, 4, Some(6), 2, &mut host(50.0, 50.0), &mut d).unwrap();
    assert_eq!((d[0].to_bits(), d[1].to_bits(), d[2].to_bits()), (0x3f100000, 0xbf880000, 0xa5a5a5a5));
    // prelude route (no list)
    let mut d = [0.0f32; 4];
    wired(1, 4, None, 2, &mut host(50.0, 50.0), &mut d).unwrap();
    assert_eq!((d[0].to_bits(), d[1].to_bits()), (0x3f100000, 0xbf880000));
    // global 9 with nibble 4 on a non-point object
    let mut d = [0.0f32; 4];
    wired(9, 4, Some(5), 2, &mut host(50.0, 50.0), &mut d).unwrap();
    assert_eq!(d[0].to_bits(), 0x3f100000);
    // a non-point object, global 0, kind_byte 2 and nibble 4 keeps the old refusal; the generic nibble 4 refuses UNRECORDED
    assert_eq!(wired(0, 4, Some(5), 2, &mut host(0.0, 0.0), &mut d), Err(Error::PointRoute));
    assert_eq!(wired(0, 4, Some(5), 1, &mut host(0.0, 0.0), &mut d), Err(Error::N4GenericUnrecorded));
    let t = refusal_text(&Error::N4GenericUnrecorded, "Transform_and_Blur_Autogen", "Center XY");
    assert!(t.starts_with("TICKMAP-N4-GENERIC-UNRECORDED (Transform_and_Blur_Autogen Center XY: nibble 4 on the generic mapping path"));
}

#[test]
fn recorded_nonzero_binding_center_12_5_minus_7_25() {
    // record ptnz: 'Center XY' bound 12.5,-7.25 at 64x32: checkout x = 819200, y = -475136; stored bits 0xbf1c0000 and 0x3f3a0000
    let (o, _) = recorded(64, 32, 12.5, -7.25, 21);
    assert_eq!((o[0].to_bits(), o[1].to_bits()), (0xbf1c0000, 0x3f3a0000));
}
