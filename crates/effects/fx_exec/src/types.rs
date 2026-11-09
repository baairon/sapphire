//! Planar f32 images, constants as bits, the error type and the explicit kernel parameters.

use std::fmt;
use std::rc::Rc;

/// A planar f32 image: `planes.len` channels of `n` floats each (whole frame, one tile: ROI-FULLFRAME).
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub n: usize,
    pub planes: Vec<Vec<f32>>,
}

impl Image {
    pub fn zeros(channels: usize, n: usize) -> Image {
        Image { n, planes: vec![vec![0.0; n]; channels] }
    }
    pub fn from_planes(planes: Vec<Vec<f32>>) -> Result<Image, ExecError> {
        let n = planes.first().map(|p| p.len()).unwrap_or(0);
        if planes.iter().any(|p| p.len() != n) {
            return Err(ExecError::Shape("planes of different lengths".into()));
        }
        Ok(Image { n, planes })
    }
    pub fn channels(&self) -> usize {
        self.planes.len()
    }
    /// A constant broadcast to an image of `lanes` channels (used when a constant is the destination).
    pub fn from_const(lanes: &[u32; 4], channels: usize, n: usize) -> Image {
        let planes = (0..channels).map(|c| vec![f32::from_bits(lanes[c.min(3)]); n]).collect();
        Image { n, planes }
    }
}

/// A value in the plan: an image, or a constant of `n` lanes (the first `n` lanes of `lanes` count; bits).
#[derive(Clone, Debug)]
pub enum Val {
    Img(Rc<Image>),
    /// the make_mipmap result: a ripmap object (NV9-A2), not a plain planar image
    Mip(Rc<MipImage>),
    Const { n: usize, lanes: [u32; 4] },
    /// an image resident on the device (only the device backend makes one); `DevVal::materialize` brings it to the host
    Dev(Rc<DevVal>),
    /// a ripmap object resident on the device (make_mipmap run by the device backend); `DevMipVal::materialize` brings it to the host
    DevMip(Rc<DevMipVal>),
}

/// A ripmap on the device: all NL x NP padded planes of the object in one buffer (plane (l, j) at `offsets[l * np + j]`, plane (0, 0) first),
/// with the host copy made on demand for the CPU consumers.
#[derive(Debug)]
pub struct DevMipVal {
    pub dev: fx_device::DevImage,
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub filter: i32,
    pub wrap: [i32; 2],
    pub widths: Vec<usize>,
    pub heights: Vec<usize>,
    pub scale: Option<Vec<u32>>,
    /// float offset of plane (l, j) in the buffer, index l * heights.len() + j
    pub offsets: Vec<usize>,
    host: std::cell::OnceCell<MipImage>,
}

impl DevMipVal {
    pub fn new(dev: fx_device::DevImage, width: usize, height: usize, channels: usize, filter: i32, wrap: [i32; 2], widths: Vec<usize>, heights: Vec<usize>, scale: Option<Vec<u32>>) -> DevMipVal {
        let mut offsets = Vec::with_capacity(widths.len() * heights.len());
        let mut at = 0usize;
        for &w in &widths {
            for &h in &heights {
                offsets.push(at);
                at += (h + make_mipmap::EXTRA_ROWS) * (w + 3) * channels;
            }
        }
        DevMipVal { dev, width, height, channels, filter, wrap, widths, heights, scale, offsets, host: std::cell::OnceCell::new() }
    }
    /// Floats of plane (l, j).
    pub fn plane_len(&self, l: usize, j: usize) -> usize {
        (self.heights[j] + make_mipmap::EXTRA_ROWS) * (self.widths[l] + 3) * self.channels
    }
    /// Total floats of all planes.
    pub fn total(&self) -> usize {
        self.offsets.last().map(|&o| o + self.plane_len(self.widths.len() - 1, self.heights.len() - 1)).unwrap_or(0)
    }
    pub fn host(&self) -> Option<&MipImage> {
        self.host.get()
    }
    /// Downloads once and returns the host ripmap (the base image is the interior of plane (0, 0), which holds the source bits).
    pub fn materialize(&self) -> Result<&MipImage, ExecError> {
        if let Some(m) = self.host.get() {
            return Ok(m);
        }
        let mut all = self.dev.download().map_err(|e| ExecError::Kernel { class: "device".into(), msg: e.to_string() })?;
        let flat = all.pop().unwrap_or_default();
        let (nl, np, c) = (self.widths.len(), self.heights.len(), self.channels);
        let mut planes = Vec::with_capacity(nl * np);
        for l in 0..nl {
            for j in 0..np {
                let o = self.offsets[l * np + j];
                planes.push(flat[o..o + self.plane_len(l, j)].iter().map(|v| v.to_bits()).collect::<Vec<u32>>());
            }
        }
        let rm = make_mipmap::RipMap { c, widths: self.widths.clone(), heights: self.heights.clone(), planes };
        let n = self.width * self.height;
        let mut bp = vec![vec![0.0f32; n]; c];
        for y in 0..self.height {
            for x in 0..self.width {
                for k in 0..c {
                    bp[k][y * self.width + x] = f32::from_bits(rm.planes[0][rm.at(0, x as isize, y as isize, k)]);
                }
            }
        }
        let m = MipImage { width: self.width, height: self.height, channels: c, filter: self.filter, wrap: self.wrap, ripmap: rm, scale: self.scale.clone(), base: Image { n, planes: bp } };
        let _ = self.host.set(m);
        Ok(self.host.get().expect("just set"))
    }
}

/// A planar image on the device with a lazily filled host copy.
#[derive(Debug)]
pub struct DevVal {
    pub dev: fx_device::DevImage,
    host: std::cell::OnceCell<Image>,
}

impl DevVal {
    pub fn new(dev: fx_device::DevImage) -> DevVal {
        DevVal { dev, host: std::cell::OnceCell::new() }
    }
    pub fn channels(&self) -> usize {
        self.dev.channels()
    }
    /// The host copy when it has been made.
    pub fn host(&self) -> Option<&Image> {
        self.host.get()
    }
    /// Downloads once and returns the host copy.
    pub fn materialize(&self) -> Result<&Image, ExecError> {
        if let Some(i) = self.host.get() {
            return Ok(i);
        }
        let planes = self.dev.download().map_err(|e| ExecError::Kernel { class: "device".into(), msg: e.to_string() })?;
        let _ = self.host.set(Image { n: self.dev.n, planes });
        Ok(self.host.get().expect("just set"))
    }
}

/// A borrowed operand for the kernels.
#[derive(Clone, Copy, Debug)]
pub enum Arg<'a> {
    Img(&'a Image),
    Mip(&'a MipImage),
    Const { n: usize, lanes: [u32; 4] },
}

impl<'a> Arg<'a> {
    /// The operand descriptor the selector sees: channels of an image, lanes of a constant.
    pub fn count(&self) -> i32 {
        match self {
            Arg::Img(i) => i.channels() as i32,
            Arg::Mip(m) => m.channels as i32,
            Arg::Const { n, .. } => *n as i32,
        }
    }
    pub fn is_const(&self) -> bool {
        matches!(self, Arg::Const { .. })
    }
    pub fn bits(&self) -> [u32; 4] {
        match self {
            Arg::Const { lanes, .. } => *lanes,
            Arg::Img(_) | Arg::Mip(_) => [0; 4],
        }
    }
    pub fn f32s(&self) -> [f32; 4] {
        let b = self.bits();
        [f32::from_bits(b[0]), f32::from_bits(b[1]), f32::from_bits(b[2]), f32::from_bits(b[3])]
    }
    pub fn planes(&self) -> Vec<&'a [f32]> {
        match self {
            Arg::Img(i) => i.planes.iter().map(|p| p.as_slice()).collect(),
            Arg::Mip(_) | Arg::Const { .. } => Vec::new(),
        }
    }
}

impl Val {
    pub fn as_arg(&self) -> Arg<'_> {
        match self {
            Val::Img(i) => Arg::Img(i.as_ref()),
            Val::Mip(m) => Arg::Mip(m.as_ref()),
            Val::Const { n, lanes } => Arg::Const { n: *n, lanes: *lanes },
            Val::Dev(d) => Arg::Img(d.host().expect("a device image is materialized before it is used as a host argument")),
            Val::DevMip(d) => Arg::Mip(d.host().expect("a device ripmap is materialized before it is used as a host argument")),
        }
    }

    /// The operand count the selector sees (channels of an image, lanes of a constant), without bringing a device image to the host.
    pub fn count(&self) -> i32 {
        match self {
            Val::Dev(d) => d.channels() as i32,
            Val::DevMip(d) => d.channels as i32,
            other => other.as_arg().count(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExecError {
    /// a run-primitive class (or command) the executor does not rebuild: the class name is the first word
    Unsupported { class: String, reason: String },
    /// the selector returned -1 ("Can't execute %s; no valid arg combo")
    NoCombo { class: String, operands: Vec<i32> },
    MissingInput(String),
    MissingHostFrame(String),
    Shape(String),
    Kernel { class: String, msg: String },
    BadPlan(String),
    /// the convert pair at a band cell other than 1 (the stepped writer is not ported or judged)
    CellGt1NotClaimed(usize),
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::Unsupported { class, reason } => write!(f, "unsupported class {}: {}", class, reason),
            ExecError::NoCombo { class, operands } => write!(f, "Can't execute {}; no valid arg combo for operand counts {:?}", class, operands),
            ExecError::MissingInput(s) => write!(f, "missing input image {}", s),
            ExecError::MissingHostFrame(s) => write!(f, "missing host frame {}", s),
            ExecError::Shape(s) => write!(f, "shape error: {}", s),
            ExecError::Kernel { class, msg } => write!(f, "kernel {} failed: {}", class, msg),
            ExecError::BadPlan(s) => write!(f, "bad plan: {}", s),
            ExecError::CellGt1NotClaimed(c) => write!(f, "CELL-GT1-NOT-CLAIMED: convert band cell {} (only cell 1 is claimed)", c),
        }
    }
}

impl std::error::Error for ExecError {}

/// Explicit parameters the builtin crates require (no defaults inside the crates). Every field is an
/// assumption of this executor (NV5-A3, NV5-A4) and is listed in the README.
#[derive(Clone, Copy, Debug)]
pub struct KParams {
    /// Every lane of the 32 byte ymm constant (SEMANTIC 1.0f)
    pub vec_one: f32,
    /// (address of output plane 3) & 0x1f; a 32 byte aligned allocation gives 0 (the allocator's alignment is a host value)
    pub dst3_phase_bytes: u32,
    /// the pool flag of the plane service (PL1-A3): false = outside a pool worker
    pub in_pool: bool,
    /// NV9-A3 / NV9B-A1: sampler geometry; None = the binding chosen by `geom_mode` (Dump: `Geom::dump`, Nv9Old: `Geom::full_frame`)
    pub geom: Option<Geom>,
    /// NV9B-A1: which default host geometry and xy plane `geom == None` means (default Dump)
    pub geom_mode: GeomMode,
    /// NV9B-A2: the host aspect a that set_aspect stores (rec = a, rec = 1/a); INFERRED 1.0
    pub host_aspect: f32,
    /// NV9-A4: [rec] of static2 (the seed term the host feeds; [rec] is the element offset, 0 for a whole-frame call)
    pub static2_seed_a: i32,
    /// NV9-A5: make_mipmap constant pad pixel (rec, PARAM-UNCAPTURED), used by wrap mode 0
    pub pad_pixel: [f32; 4],
    /// NV9-A6: blur edge mode cell (pointee, U03): 0 zero, 1 mirror
    pub blur_edge: i32,
    /// Blur render scale (replaces the wrong default 1 of NV9-A6) = [info] = ctx = the FRAME WIDTH (scale W + mirror edge is 8192 of 8192 bit equal, scale 1, 2, 32 is 0 of 8192).
    /// A value <= 0 means "bind the frame width" (the default); a positive value is used as given. NV9-BLUR-SCALE-OLD: env NV9_BLUR_SCALE_OLD=1 makes the standin default 1 again.
    pub blur_render_scale: i32,
    /// / scalar head count of the original xy ramp (see `x_ramp`), an explicit per-run host input = (32 - (p & 31)) / 4 for the host's row pointer p (0 when p is 32 byte aligned); default 0 (HOST-ALLOC); -1 = the closed-form pixel-centre plane, an explicit non-host option
    pub xy_head: i32,
    /// S1: XY-HEAD-MODE. OneHead (the pre-WPE3 form: the head `xy_head` for every row) or PerRow (row r's head from
    /// the row pointer phase0 + r * W * 4). The form of each default-xy writer comes from its placement in the plan (exec::head_form) and is carried by
    /// the writer list; this field serves the images built outside that list and defaults to OneHead.
    pub xy_head_mode: XyHeadMode,
    /// WPH: base phase mod 32 of the perspective out2 / perspective_alpha out1 planes as per-run host inputs (-1 = not given), and the host cells
    /// [[rec]] (i32), (f64) of perspective_alpha (None = not given); state A tests may still read the WPG1_* env knobs
    pub persp_base: i32,
    pub palpha_base: i32,
    /// (part 2): the perspective out2 / perspective_alpha out1 destination is a row buffer in the counted plan (allocated in the
    /// line list of the exec block that writes it): every row is written at the same pointer, so every row takes the base phase (false = a plane:
    /// rows at base + row x W x 4, the form). Set only on the --xy-base-list route from the placement model's plan text.
    pub persp_rowbuf: bool,
    pub palpha_rowbuf: bool,
    /// base phase mod 32 of the u plane as its producer writes it (-1 = not given: the geom branch)
    pub xy_base: i32,
    pub palpha_cells: Option<(i32, i32, f64)>,
    /// destination plane base address modulo 32 (bytes) of the small planes of exec-downsampled (picks the alignment peel of the down kernel); MEASURED 0 in the emulator runs, a host allocator value otherwise
    pub ds_dst_phase: u32,
    /// DS1-OFF: skip the exec-downsampled model (the executor then runs the body at full size, the pre-DS1 behaviour)
    pub ds_off: bool,
    /// `info != 0` of the image object read by class grad. PARAM-UNCAPTURED in (GR-A1); an explicit input with a labelled STAND-IN default of false (no value was chosen by; false is the value measured at host aspect 1.0 in its E1 runs unfinished)
    /// test interface only. The executor's info_aspect is the plugin's derivation (set_aspect aspect_block_e); Some(v) different from it is an error.
    pub kparams_a: Option<bool>,
    /// HOST-COUNT (ctx, ROUTE W x H): the element count of sharpen's combine job. None makes the sharpen arm fail; the executor graph loop fills W x H.
    pub sharpen_count: Option<usize>,
    /// the plan runs sharpen in place (destination ref = first source ref), derived from the executed plan
    pub sharpen_alias: bool,
    ///: the blur aspect as a host input: Some(f32 at ctx) when the flag at ctx is nonzero, None when it is 0
    /// (the blur crate multiplies the Y sigma by it, coef/src/driver.rs). (MEASURED): the recorded 1/36 came from a harness bug (num_params written into
    /// in_data, pixel_aspect_ratio.den); at a 1/1 pixel aspect the cells are 0, 1.0, 1.0, so the claim value is None (HOST-ASPECT-SQUARE). state A
    /// is None. The state A constructor keeps None (its E-PARITY is for None); a SMART-context run must give the recorded cells (fx_run --blur-aspect-ctx).
    pub blur_aspect: Option<f32>,
}

impl KParams {
    pub fn standin() -> KParams {
        KParams { vec_one: 1.0, dst3_phase_bytes: 0, in_pool: false, geom: None, geom_mode: GeomMode::Dump, host_aspect: 1.0, static2_seed_a: 0, pad_pixel: [0.0; 4], blur_edge: 1, blur_render_scale: if std::env::var("NV9_BLUR_SCALE_OLD").map(|v| v == "1").unwrap_or(false) { 1 } else { 0 }, xy_head: 0, xy_head_mode: XyHeadMode::OneHead, persp_base: -1, palpha_base: -1, persp_rowbuf: false, palpha_rowbuf: false, xy_base: -1, palpha_cells: None, ds_dst_phase: 0, ds_off: std::env::var("DS1_OFF").map(|v| v == "1").unwrap_or(false), kparams_a: None, sharpen_count: None, sharpen_alias: false, blur_aspect: None }
    }
}

/// The make_mipmap result (NV9-A2): the original allocates a ripmap object (allocate-images kind `4 3`), a grid of NL x NP padded
/// planes (make_mipmap crate `RipMap`), not a plain planar image. It is only ever the first argument of a sampler
/// (blur_warp, warp_rel); the executor keeps the ripmap with the frame geometry it was built for.
#[derive(Clone, Debug)]
pub struct MipImage {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub filter: i32,
    pub wrap: [i32; 2],
    pub ripmap: make_mipmap::RipMap,
    /// obj reciprocal table (filter nonzero only), as f32 bits
    pub scale: Option<Vec<u32>>,
    /// the plain planar image make_mipmap was built from (the base level before padding), kept for blur at Shift 0
    pub base: Image,
}

/// Host geometry fields of the call record that the samplers read (NV9-A3, HOST-GEOMETRY, UNMEASURED): the layer extent in pixels
/// (rec, rec), the pixel aspect (rec), the vertical anchors (rec, rec) and the region origin
/// (rec, rec). The real host values are not in the dump; the defaults make a normalised `xy` plane (see `xy_grid`)
/// land exactly on the pixel it belongs to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Geom {
    pub left: f32,
    pub right: f32,
    pub aspect: f32,
    pub assumption_e: f32,
    pub assumption_h: f32,
    pub roi_x: i32,
    pub roi_y: i32,
}

impl Geom {
    /// Whole frame, centred: scx = W/2, scy = W/2 (aspect 1), centre of the frame at (W/2, H/2).
    pub fn full_frame(w: usize, h: usize) -> Geom {
        Geom { left: 0.0, right: w as f32, aspect: 1.0, assumption_e: (h as f32 - w as f32) * 0.5, assumption_h: 0.0, roi_x: 0, roi_y: 0 }
    }
    pub fn scx(&self) -> f32 {
        0.5 * (self.right - self.left)
    }
    pub fn scy(&self) -> f32 {
        (0.5 * self.aspect) * (self.right - self.left)
    }
}

/// The `xy` plane (2 channels) for a frame: x = (px - X0) / scx, y = (Y0 - py) / scy with the centre (X0, Y0) the geometry implies
/// (blur_warp S2/S3: x = (xy.x + 1) * scx + left - roi_x, y = assumption_e - roi_y - (scy * (xy.y - 1) - assumption_h)). `centre` shifts the pixel
/// position (0.0 = integer pixel centres as the bilinear sampler uses them). NV9-A3.
pub fn xy_grid(w: usize, h: usize, g: &Geom, centre: f32) -> Image {
    let (scx, scy) = (g.scx(), g.scy());
    let x0 = g.left + scx - g.roi_x as f32;
    let y0 = g.assumption_e + scy + g.assumption_h - g.roi_y as f32;
    let mut px = Vec::with_capacity(w * h);
    let mut py = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            px.push((x as f32 + centre - x0) / scx);
            py.push((y0 - (y as f32 + centre)) / scy);
        }
    }
    Image { n: w * h, planes: vec![px, py] }
}

/// NV9B-A1: which default geometry (record fields rec, rec, rec) and which default `xy` plane the
/// executor binds when the caller gives neither. `Dump` is the dump's own arithmetic (audit_rd1 GEOMETRY.tsv); `Nv9Old` is a
/// guess (label NV9-GEOM-OLD): left 0, right W, assumption_e = (H - W)/2, assumption_h 0 and the plane x = 2px/W - 1, y = (H - 2py)/W.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeomMode {
    Dump,
    Nv9Old,
}

impl Geom {
    /// The dump's geometry for an image of w x h and host aspect a (, MEASURED code, f32 operation order):
    /// assumption_f = -0.5; assumption_g = W - 0.5; t = fma(0.5, H, -0.5); u = ((assumption_g) - (-0.5)) / (a + a); assumption_h = t - u; rec = 1 / a
    /// (stored in `aspect`which the samplers read as rec); assumption_e = assumption_c = assumption_d = 0 (zero-init, INFERRED, NV9B-A3).
    pub fn dump(w: usize, h: usize, a: f32) -> Geom {
        let assumption_f = -0.5f32;
        let assumption_g = w as f32 - 0.5;
        let t = 0.5f32.mul_add(h as f32, -0.5);
        let u = (assumption_g - (-0.5f32)) / (a + a);
        Geom { left: assumption_f, right: assumption_g, aspect: 1.0 / a, assumption_e: 0.0, assumption_h: t - u, roi_x: 0, roi_y: 0 }
    }
    /// rec = (H - 1)/2 + W/(2a), written next to assumption_h; no sampler of this executor reads it (reported for the record).
    pub fn dump_f_cell_a(w: usize, h: usize, a: f32) -> f32 {
        let assumption_g = w as f32 - 0.5;
        let t = 0.5f32.mul_add(h as f32, -0.5);
        let u = (assumption_g - (-0.5f32)) / (a + a);
        t + u
    }
}

/// NV9B-A4: the identity pixel-centre `xy` plane of the dump's geometry (a = 1, assumption_e = assumption_c = assumption_d = 0): x = (2px + 1)/W - 1,
/// y = (H - 1 - 2py)/W, all in float32. The samplers map it to the source coordinate (px, py) (integer = pixel centre).
pub fn xy_pixel_centre(w: usize, h: usize) -> Image {
    let (wf, hf) = (w as f32, h as f32);
    let mut px = Vec::with_capacity(w * h);
    let mut py = Vec::with_capacity(w * h);
    for y in 0..h {
        for x in 0..w {
            px.push((2.0 * x as f32 + 1.0) / wf - 1.0);
            py.push(((hf - 1.0) - 2.0 * y as f32) / wf);
        }
    }
    Image { n: w * h, planes: vec![px, py] }
}

/// the x ramp of the original `variable 2` command (branch ecx == 2), one row of `cnt` floats.
/// Source: start s = [ctx], step d = [ctx]; `head` = number of scalar head elements the code stores before its 32 byte aligned
/// vector body ((32 - (dest & 31)) / 4, 0 when the destination is 32 byte aligned: ALIGN-ASSUMED, the allocator address is a host value).
/// Order of operations as in the listing: scalar head accumulates, 16 wide body = broadcast(base) + d * lane (lane table 0..7, constants
/// 16.0, 8.0, 1.0 measured in the emulator), the scalar accumulator runs in step, the 8 wide
/// block, then the scalar tail. cnt < 29 or cnt < head + 16 take other branches (NOT READ): there a plain
/// accumulating scalar loop is used (ASSUMED).
pub fn x_ramp(cnt: usize, head: usize, s: f32, d: f32) -> Vec<f32> {
    let mut out = vec![0.0f32; cnt];
    let mut x4 = s;
    // (READ): cnt < 16 takes the plain scalar loop; 16 <= cnt < 29 skips the alignment peel
    // (head = 0) and takes the 16 wide body, the 8 wide block and the scalar tail; cnt >= 29 keeps the head (an earlier model assumed the plain loop for every cnt < 29).
    let head = if cnt >= 16 && cnt < 29 { 0 } else { head };
    if cnt < 16 || cnt < head + 16 {
        for o in out.iter_mut() {
            *o = x4;
            x4 += d;
        }
        return out;
    }
    for o in out.iter_mut().take(head) {
        *o = x4;
        x4 += d;
    }
    let r10f = head as f32;
    let x1 = d * 16.0;
    let x10 = (d + s) - d;
    let a5 = d.mul_add(r10f, x10);
    let a11 = (r10f + 8.0).mul_add(d, x10);
    let dl: Vec<f32> = (0..8).map(|l| d * l as f32).collect();
    let mut v6: Vec<f32> = dl.iter().map(|t| a5 + t).collect();
    let mut v5: Vec<f32> = dl.iter().map(|t| a11 + t).collect();
    let r9 = cnt - ((cnt - head) & 0xf);
    let mut r11 = head;
    loop {
        out[r11..r11 + 8].copy_from_slice(&v6);
        out[r11 + 8..r11 + 16].copy_from_slice(&v5);
        x4 += x1;
        for t in v6.iter_mut() {
            *t += x1;
        }
        for t in v5.iter_mut() {
            *t += x1;
        }
        r11 += 16;
        if r11 >= r9 {
            break;
        }
    }
    if cnt > r9 {
        let rcx = cnt - r9;
        let mut k = 0usize;
        if rcx >= 8 {
            let x3 = d - s;
            let x5 = d * 8.0;
            let x6 = (r9 as f32 + 1.0).mul_add(d, -x3);
            let mut v0: Vec<f32> = dl.iter().map(|t| x6 + t).collect();
            k = rcx & !7;
            let mut r10 = 0;
            while r10 < k {
                out[r9 + r10..r9 + r10 + 8].copy_from_slice(&v0);
                x4 += x5;
                for t in v0.iter_mut() {
                    *t += x5;
                }
                r10 += 8;
            }
        }
        while k < rcx {
            out[r9 + k] = x4;
            k += 1;
            x4 += d;
        }
    }
    out
}

/// the xy plane the original generates per row (`variable 2`): x = x_ramp(W, head, s, d) with s = -1 + 1/W, d = 2/W (a = 1),
/// y = fma(-row, d, y0) with y0 = fma(assumption_h, d, 1) (vfnmsub213ss with [ctx] = -y0, [ctx] = d); constants fitted to the
/// values the emulator hook read for 50x7 and 64x32 (s, d, [ctx], [ctx] bit equal in both).
pub fn xy_ramp(w: usize, h: usize, head: usize) -> Image {
    xy_ramp_mode(w, h, head, XyHeadMode::OneHead)
}

/// S1: the xy head mode of `variable 2`'s x writer (XY-HEAD-MODE).
/// OneHead: the plan writes one x row buffer, the head is applied once to that buffer's pointer and every row repeats it (part 2, XY-ONE-ROW,
/// MEASURED for Transform_and_Blur 63x33 at Full and Half). PerRow: the plan allocates an x plane indexed by row and row r's head comes from that
/// row's pointer (, MEASURED by the record for Warp_Perspective).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XyHeadMode {
    OneHead,
    PerRow,
}

pub fn xy_ramp_mode(w: usize, h: usize, head: usize, mode: XyHeadMode) -> Image {
    let g = Geom::dump(w, h, 1.0);
    let d = 2.0f32 / w as f32;
    let s = -1.0f32 + 1.0f32 / w as f32;
    let y0 = g.assumption_h.mul_add(d, 1.0);
    // (PerRow; MEASURED by the record, XY_PHASE.md): the original writes the xy image one row per `variable 2` call into the
    // image plane at (plane base + row * W * 4); the x writer takes the head from THAT row pointer's offset mod 32, so the head changes
    // from row to row whenever W * 4 is not a multiple of 32 (W = 33: rows 0..4 have heads 0,7,6,5,4). `head` is the row 0 head; the
    // plane base phase in bytes is (32 - 4 * head) mod 32 (head 0 = a 32 byte aligned plane base, the measured case at every size).
    // OneHead: the same head for every row (the pre-WPE3 behaviour).
    let phase0 = ((8 - (head % 8)) % 8) * 4;
    let mut rows: [Option<Vec<f32>>; 8] = Default::default();
    let one = x_ramp(w, head, s, d);
    let mut px = Vec::with_capacity(w * h);
    let mut py = Vec::with_capacity(w * h);
    for y in 0..h {
        match mode {
            XyHeadMode::OneHead => px.extend_from_slice(&one),
            XyHeadMode::PerRow => {
                let hd = row_head(phase0 + y * w * 4);
                if rows[hd].is_none() {
                    rows[hd] = Some(x_ramp(w, hd, s, d));
                }
                px.extend_from_slice(rows[hd].as_ref().unwrap());
            }
        }
        let yv = (-(y as f32)).mul_add(d, y0);
        py.extend(std::iter::repeat(yv).take(w));
    }
    Image { n: w * h, planes: vec![px, py] }
}

/// the x writer's head count for a destination pointer with byte offset `off` (mod 32 used) from a 32 byte boundary:
/// r10 = 0 when (ptr & 31) == 0, else (32 - (ptr & 31)) / 4 (the ptr & 3 != 0 plain loop cannot occur for f32 rows).
pub fn row_head(off: usize) -> usize {
    let p = off & 31;
    if p == 0 {
        0
    } else {
        (32 - p) / 4
    }
}
