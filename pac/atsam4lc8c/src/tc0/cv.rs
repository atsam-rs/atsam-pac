#[doc = "Reader of register CV%s"]
pub type R = crate::R<u32, super::CV>;
#[doc = "Reader of field `CV`"]
pub type CV_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0xffff) as u16)
    }
}
