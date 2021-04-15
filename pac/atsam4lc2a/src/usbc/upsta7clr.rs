#[doc = "Register `UPSTA7CLR` writer"]
pub struct W(crate::W<UPSTA7CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPSTA7CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<UPSTA7CLR_SPEC>> for W {
    fn from(writer: crate::W<UPSTA7CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIC` writer - RXINI Clear"]
pub struct RXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIC_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TXOUTIC` writer - TXOUTI Clear"]
pub struct TXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `TXSTPIC` writer - TXSTPI Clear"]
pub struct TXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PERRIC` writer - PERRI Clear"]
pub struct PERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `NAKEDIC` writer - NAKEDI Clear"]
pub struct NAKEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ERRORFIC` writer - ERRORFI Clear"]
pub struct ERRORFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RXSTALLDIC` writer - RXSTALLDI Clear"]
pub struct RXSTALLDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RAMACERIC` writer - RAMACERI Clear"]
pub struct RAMACERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - RXINI Clear"]
    #[inline(always)]
    pub fn rxinic(&mut self) -> RXINIC_W {
        RXINIC_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTI Clear"]
    #[inline(always)]
    pub fn txoutic(&mut self) -> TXOUTIC_W {
        TXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPI Clear"]
    #[inline(always)]
    pub fn txstpic(&mut self) -> TXSTPIC_W {
        TXSTPIC_W { w: self }
    }
    #[doc = "Bit 3 - PERRI Clear"]
    #[inline(always)]
    pub fn perric(&mut self) -> PERRIC_W {
        PERRIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDI Clear"]
    #[inline(always)]
    pub fn nakedic(&mut self) -> NAKEDIC_W {
        NAKEDIC_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFI Clear"]
    #[inline(always)]
    pub fn errorfic(&mut self) -> ERRORFIC_W {
        ERRORFIC_W { w: self }
    }
    #[doc = "Bit 6 - RXSTALLDI Clear"]
    #[inline(always)]
    pub fn rxstalldic(&mut self) -> RXSTALLDIC_W {
        RXSTALLDIC_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERI Clear"]
    #[inline(always)]
    pub fn ramaceric(&mut self) -> RAMACERIC_W {
        RAMACERIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta7clr](index.html) module"]
pub struct UPSTA7CLR_SPEC;
impl crate::RegisterSpec for UPSTA7CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upsta7clr::W](W) writer structure"]
impl crate::Writable for UPSTA7CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPSTA7CLR to value 0"]
impl crate::Resettable for UPSTA7CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
