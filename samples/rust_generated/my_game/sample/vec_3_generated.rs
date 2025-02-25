// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
// struct Vec3, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3(pub [u8; 12]);
impl Default for Vec3 { 
  fn default() -> Self { 
    Self([0; 12])
  }
}
impl core::fmt::Debug for Vec3 {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("Vec3")
      .field("x", &self.x())
      .field("y", &self.y())
      .field("z", &self.z())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Vec3 {}
impl<'a> flatbuffers::Follow<'a> for Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    unsafe { <&'a Vec3>::follow(buf, loc) }
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a Vec3 {
  type Inner = &'a Vec3;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    unsafe { flatbuffers::follow_cast_ref::<Vec3>(buf, loc) }
  }
}
impl<'b> flatbuffers::Push for Vec3 {
    type Output = Vec3;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = unsafe { ::core::slice::from_raw_parts(self as *const Vec3 as *const u8, <Self as flatbuffers::Push>::size()) };
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(4)
    }
}

impl<'a> flatbuffers::Verifiable for Vec3 {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> Vec3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    x: f32,
    y: f32,
    z: f32,
  ) -> Self {
    let mut s = Self([0; 12]);
    s.set_x(x);
    s.set_y(y);
    s.set_z(z);
    s
  }

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Sample.Vec3"
  }

  pub fn x(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_x(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn y(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[4..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_y(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[4..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn z(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<<f32 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[8..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_z(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[8..].as_mut_ptr(),
        core::mem::size_of::<<f32 as EndianScalar>::Scalar>(),
      );
    }
  }

  pub fn unpack(&self) -> Vec3T {
    Vec3T {
      x: self.x(),
      y: self.y(),
      z: self.z(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Vec3T {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}
impl Vec3T {
  pub fn pack(&self) -> Vec3 {
    Vec3::new(
      self.x,
      self.y,
      self.z,
    )
  }
}

