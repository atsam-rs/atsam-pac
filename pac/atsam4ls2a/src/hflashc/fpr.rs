#[doc = "Reader of register FPR"]
pub type R = crate::R<u32, super::FPR>;
#[doc = "Reader of field `FSZ`"]
pub type FSZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `PSZ`"]
pub type PSZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Flash Size"]
    #[inline(always)]
    pub fn fsz(&self) -> FSZ_R {
        FSZ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Page Size"]
    #[inline(always)]
    pub fn psz(&self) -> PSZ_R {
        PSZ_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
