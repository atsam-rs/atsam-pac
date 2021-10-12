#[doc = "Register `IER` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC0RDY` writer - OSC0 Ready"]
pub struct OSC0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC0RDY_W<'a> {
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
#[doc = "Field `DFLL0LOCKC` writer - DFLL0 Lock Coarse"]
pub struct DFLL0LOCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0LOCKC_W<'a> {
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
#[doc = "Field `DFLL0LOCKF` writer - DFLL0 Lock Fine"]
pub struct DFLL0LOCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0LOCKF_W<'a> {
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
#[doc = "Field `DFLL0RDY` writer - DFLL0 Ready"]
pub struct DFLL0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0RDY_W<'a> {
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
#[doc = "Field `DFLL0RCS` writer - DFLL0 Reference Clock Stopped"]
pub struct DFLL0RCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0RCS_W<'a> {
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
#[doc = "Field `DFLL0OOB` writer - DFLL0 Out Of Bounds"]
pub struct DFLL0OOB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0OOB_W<'a> {
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
#[doc = "Field `PLL0LOCK` writer - PLL0 Lock"]
pub struct PLL0LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL0LOCK_W<'a> {
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
#[doc = "Field `PLL0LOCKLOST` writer - PLL0 Lock Lost"]
pub struct PLL0LOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL0LOCKLOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RCFASTLOCK` writer - RCFAST Lock"]
pub struct RCFASTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCFASTLOCK_W<'a> {
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
#[doc = "Field `RCFASTLOCKLOST` writer - RCFAST Lock Lost"]
pub struct RCFASTLOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> RCFASTLOCKLOST_W<'a> {
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
#[doc = "Field `AE` writer - Access Error"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - OSC0 Ready"]
    #[inline(always)]
    pub fn osc0rdy(&mut self) -> OSC0RDY_W {
        OSC0RDY_W { w: self }
    }
    #[doc = "Bit 1 - DFLL0 Lock Coarse"]
    #[inline(always)]
    pub fn dfll0lockc(&mut self) -> DFLL0LOCKC_W {
        DFLL0LOCKC_W { w: self }
    }
    #[doc = "Bit 2 - DFLL0 Lock Fine"]
    #[inline(always)]
    pub fn dfll0lockf(&mut self) -> DFLL0LOCKF_W {
        DFLL0LOCKF_W { w: self }
    }
    #[doc = "Bit 3 - DFLL0 Ready"]
    #[inline(always)]
    pub fn dfll0rdy(&mut self) -> DFLL0RDY_W {
        DFLL0RDY_W { w: self }
    }
    #[doc = "Bit 4 - DFLL0 Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfll0rcs(&mut self) -> DFLL0RCS_W {
        DFLL0RCS_W { w: self }
    }
    #[doc = "Bit 5 - DFLL0 Out Of Bounds"]
    #[inline(always)]
    pub fn dfll0oob(&mut self) -> DFLL0OOB_W {
        DFLL0OOB_W { w: self }
    }
    #[doc = "Bit 6 - PLL0 Lock"]
    #[inline(always)]
    pub fn pll0lock(&mut self) -> PLL0LOCK_W {
        PLL0LOCK_W { w: self }
    }
    #[doc = "Bit 7 - PLL0 Lock Lost"]
    #[inline(always)]
    pub fn pll0locklost(&mut self) -> PLL0LOCKLOST_W {
        PLL0LOCKLOST_W { w: self }
    }
    #[doc = "Bit 13 - RCFAST Lock"]
    #[inline(always)]
    pub fn rcfastlock(&mut self) -> RCFASTLOCK_W {
        RCFASTLOCK_W { w: self }
    }
    #[doc = "Bit 14 - RCFAST Lock Lost"]
    #[inline(always)]
    pub fn rcfastlocklost(&mut self) -> RCFASTLOCKLOST_W {
        RCFASTLOCKLOST_W { w: self }
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
