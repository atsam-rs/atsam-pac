#[doc = "Writer for register DMA_IER"]
pub type W = crate::W<u32, super::DMA_IER>;
#[doc = "Write proxy for field `DMAIER`"]
pub struct DMAIER_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAIER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn dmaier(&mut self) -> DMAIER_W {
        DMAIER_W { w: self }
    }
}
