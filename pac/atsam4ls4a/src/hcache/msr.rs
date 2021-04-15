#[doc = "Reader of register MSR"]
pub type R = crate::R<u32, super::MSR>;
#[doc = "Reader of field `EVENTCNT`"]
pub type EVENTCNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Monitor Event Counter"]
    #[inline(always)]
    pub fn eventcnt(&self) -> EVENTCNT_R {
        EVENTCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
