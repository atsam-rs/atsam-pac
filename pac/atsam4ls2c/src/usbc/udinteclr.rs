#[doc = "Register `UDINTECLR` writer"]
pub struct W(crate::W<UDINTECLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTECLR_SPEC>;
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
impl From<crate::W<UDINTECLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINTECLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPEC` writer - SUSP Interrupt Enable Clear"]
pub struct SUSPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEC_W<'a> {
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
#[doc = "Field `MSOFEC` writer - MSOF Interrupt Enable Clear"]
pub struct MSOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFEC_W<'a> {
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
#[doc = "Field `SOFEC` writer - SOF Interrupt Enable Clear"]
pub struct SOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFEC_W<'a> {
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
#[doc = "Field `EORSTEC` writer - EORST Interrupt Enable Clear"]
pub struct EORSTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTEC_W<'a> {
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
#[doc = "Field `WAKEUPEC` writer - WAKEUP Interrupt Enable Clear"]
pub struct WAKEUPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEC_W<'a> {
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
#[doc = "Field `EORSMEC` writer - EORSM Interrupt Enable Clear"]
pub struct EORSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMEC_W<'a> {
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
#[doc = "Field `UPRSMEC` writer - UPRSM Interrupt Enable Clear"]
pub struct UPRSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMEC_W<'a> {
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
#[doc = "Field `EP0INTEC` writer - EP0INT Interrupt Enable Clear"]
pub struct EP0INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `EP1INTEC` writer - EP1INT Interrupt Enable Clear"]
pub struct EP1INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `EP2INTEC` writer - EP2INT Interrupt Enable Clear"]
pub struct EP2INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `EP3INTEC` writer - EP3INT Interrupt Enable Clear"]
pub struct EP3INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `EP4INTEC` writer - EP4INT Interrupt Enable Clear"]
pub struct EP4INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `EP5INTEC` writer - EP5INT Interrupt Enable Clear"]
pub struct EP5INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `EP6INTEC` writer - EP6INT Interrupt Enable Clear"]
pub struct EP6INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `EP7INTEC` writer - EP7INT Interrupt Enable Clear"]
pub struct EP7INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7INTEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn suspec(&mut self) -> SUSPEC_W {
        SUSPEC_W { w: self }
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable Clear"]
    #[inline(always)]
    pub fn msofec(&mut self) -> MSOFEC_W {
        MSOFEC_W { w: self }
    }
    #[doc = "Bit 2 - SOF Interrupt Enable Clear"]
    #[inline(always)]
    pub fn sofec(&mut self) -> SOFEC_W {
        SOFEC_W { w: self }
    }
    #[doc = "Bit 3 - EORST Interrupt Enable Clear"]
    #[inline(always)]
    pub fn eorstec(&mut self) -> EORSTEC_W {
        EORSTEC_W { w: self }
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn wakeupec(&mut self) -> WAKEUPEC_W {
        WAKEUPEC_W { w: self }
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn eorsmec(&mut self) -> EORSMEC_W {
        EORSMEC_W { w: self }
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn uprsmec(&mut self) -> UPRSMEC_W {
        UPRSMEC_W { w: self }
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep0intec(&mut self) -> EP0INTEC_W {
        EP0INTEC_W { w: self }
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep1intec(&mut self) -> EP1INTEC_W {
        EP1INTEC_W { w: self }
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep2intec(&mut self) -> EP2INTEC_W {
        EP2INTEC_W { w: self }
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep3intec(&mut self) -> EP3INTEC_W {
        EP3INTEC_W { w: self }
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep4intec(&mut self) -> EP4INTEC_W {
        EP4INTEC_W { w: self }
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep5intec(&mut self) -> EP5INTEC_W {
        EP5INTEC_W { w: self }
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep6intec(&mut self) -> EP6INTEC_W {
        EP6INTEC_W { w: self }
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep7intec(&mut self) -> EP7INTEC_W {
        EP7INTEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinteclr](index.html) module"]
pub struct UDINTECLR_SPEC;
impl crate::RegisterSpec for UDINTECLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udinteclr::W](W) writer structure"]
impl crate::Writable for UDINTECLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDINTECLR to value 0"]
impl crate::Resettable for UDINTECLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
