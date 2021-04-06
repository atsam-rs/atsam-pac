#[doc = "Reader of register VERSION"]
pub type R = crate::R<u32, super::VERSION>;
#[doc = "Reader of field `VERSION`"]
pub type VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `MFN`"]
pub type MFN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Hardware Module Version"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
