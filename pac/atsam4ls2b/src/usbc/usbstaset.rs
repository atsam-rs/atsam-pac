#[doc = "Register `USBSTASET` writer"]
pub struct W(crate::W<USBSTASET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTASET_SPEC>;
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
impl From<crate::W<USBSTASET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTASET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMACERIS` writer - RAMACERI Set"]
pub struct RAMACERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `VBUSRQS` writer - VBUSRQ Set"]
pub struct VBUSRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl W {
    #[doc = "Bit 8 - RAMACERI Set"]
    #[inline(always)]
    pub fn ramaceris(&mut self) -> RAMACERIS_W {
        RAMACERIS_W { w: self }
    }
    #[doc = "Bit 9 - VBUSRQ Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W {
        VBUSRQS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstaset](index.html) module"]
pub struct USBSTASET_SPEC;
impl crate::RegisterSpec for USBSTASET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbstaset::W](W) writer structure"]
impl crate::Writable for USBSTASET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBSTASET to value 0"]
impl crate::Resettable for USBSTASET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
