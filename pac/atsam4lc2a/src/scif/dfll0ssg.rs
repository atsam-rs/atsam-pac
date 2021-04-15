#[doc = "Register `DFLL0SSG` writer"]
pub struct W(crate::W<DFLL0SSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0SSG_SPEC>;
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
impl core::convert::From<crate::W<DFLL0SSG_SPEC>> for W {
    fn from(writer: crate::W<DFLL0SSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` writer - Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `PRBS` writer - Pseudo Random Bit Sequence"]
pub struct PRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBS_W<'a> {
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
#[doc = "Field `AMPLITUDE` writer - SSG Amplitude"]
pub struct AMPLITUDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPLITUDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `STEPSIZE` writer - SSG Step Size"]
pub struct STEPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Pseudo Random Bit Sequence"]
    #[inline(always)]
    pub fn prbs(&mut self) -> PRBS_W {
        PRBS_W { w: self }
    }
    #[doc = "Bits 8:12 - SSG Amplitude"]
    #[inline(always)]
    pub fn amplitude(&mut self) -> AMPLITUDE_W {
        AMPLITUDE_W { w: self }
    }
    #[doc = "Bits 16:20 - SSG Step Size"]
    #[inline(always)]
    pub fn stepsize(&mut self) -> STEPSIZE_W {
        STEPSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Spread Spectrum Generator Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0ssg](index.html) module"]
pub struct DFLL0SSG_SPEC;
impl crate::RegisterSpec for DFLL0SSG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dfll0ssg::W](W) writer structure"]
impl crate::Writable for DFLL0SSG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLL0SSG to value 0"]
impl crate::Resettable for DFLL0SSG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
