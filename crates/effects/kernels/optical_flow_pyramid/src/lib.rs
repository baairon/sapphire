//! Typed level table shared by the pyramid build and the solve.

/// Identifier of an image plane held by a plane store.
pub type PlaneId = u64;

/// A per-level plane that the host image service allocates for every level above level 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LevelPlane {
    ImageA,
    ImageB,
    FlowU,
    FlowV,
    Nest1T,
    Nest1U,
    Nest2T,
    Nest2U,
}

impl LevelPlane {
    /// Position in declaration order, 0 to 7.
    pub fn ordinal(self) -> u32 {
        match self {
            LevelPlane::ImageA => 0,
            LevelPlane::ImageB => 1,
            LevelPlane::FlowU => 2,
            LevelPlane::FlowV => 3,
            LevelPlane::Nest1T => 4,
            LevelPlane::Nest1U => 5,
            LevelPlane::Nest2T => 6,
            LevelPlane::Nest2U => 7,
        }
    }
}

/// One pyramid level: its rectangle, its level number and its planes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LevelRecord {
    pub width: i32,
    pub height: i32,
    /// Row stride in elements.
    pub stride: i32,
    /// Fourth word of the rectangle record, carried unchanged.
    pub rect_word3: u32,
    /// Bytes per element of the rectangle record.
    pub elem_bytes: u64,
    /// Level number the build writes.
    pub index: u32,
    pub image_a: PlaneId,
    pub image_b: PlaneId,
    pub grad_x: PlaneId,
    pub grad_y: PlaneId,
    pub warped_image: PlaneId,
    pub warped_grad_x: PlaneId,
    pub warped_grad_y: PlaneId,
    pub coeff: PlaneId,
    pub flow_u: PlaneId,
    pub flow_v: PlaneId,
    pub flow_u_next: PlaneId,
    pub flow_v_next: PlaneId,
    pub nest1_t: PlaneId,
    pub nest1_u: PlaneId,
    pub nest2_t: PlaneId,
    pub nest2_u: PlaneId,
    pub relax_first: PlaneId,
    pub relax_second: PlaneId,
    pub median_out: PlaneId,
    /// Descriptor plane the build copies into every record; no ported code reads it.
    pub copied_plane: PlaneId,
}

/// The levels of one pyramid, with the smallest side and the image table handle.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LevelTable {
    pub levels: Vec<LevelRecord>,
    pub min_side: i32,
    pub image_table: u64,
}

impl LevelRecord {
    /// The record field that holds `plane`.
    pub fn plane_mut(&mut self, plane: LevelPlane) -> &mut PlaneId {
        match plane {
            LevelPlane::ImageA => &mut self.image_a,
            LevelPlane::ImageB => &mut self.image_b,
            LevelPlane::FlowU => &mut self.flow_u,
            LevelPlane::FlowV => &mut self.flow_v,
            LevelPlane::Nest1T => &mut self.nest1_t,
            LevelPlane::Nest1U => &mut self.nest1_u,
            LevelPlane::Nest2T => &mut self.nest2_t,
            LevelPlane::Nest2U => &mut self.nest2_u,
        }
    }
}

impl LevelTable {
    /// Index of the last level, or -1 when the table is empty.
    pub fn first(&self) -> i32 {
        self.levels.len() as i32 - 1
    }
}

/// Storage of image planes addressed by `PlaneId`.
pub trait PlaneStore {
    fn read(&self, id: PlaneId) -> Vec<f32>;
    fn write(&mut self, id: PlaneId, data: &[f32]);
    /// Byte offset of the plane's first element modulo 64.
    fn align_bytes(&self, id: PlaneId) -> u32;
}
