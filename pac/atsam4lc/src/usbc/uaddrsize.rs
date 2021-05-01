#[doc = "Reader of register UADDRSIZE"]
pub type R = crate::R<u32, super::UADDRSIZE>;
#[doc = "Reader of field `UADDRSIZE`"]
pub type UADDRSIZE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP PB Address Size"]
    #[inline(always)]
    pub fn uaddrsize(&self) -> UADDRSIZE_R {
        UADDRSIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
