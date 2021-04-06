#[doc = "Writer for register IDR"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Write proxy for field `ERRIDR`"]
pub struct ERRIDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRIDR_W<'a> {
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
    #[doc = "Bit 0 - CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn erridr(&mut self) -> ERRIDR_W {
        ERRIDR_W { w: self }
    }
}
