#[doc = "Register `UDINTSET` writer"]
pub struct W(crate::W<UDINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTSET_SPEC>;
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
impl core::convert::From<crate::W<UDINTSET_SPEC>> for W {
    fn from(writer: crate::W<UDINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPS` writer - SUSP Interrupt Set"]
pub struct SUSPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPS_W<'a> {
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
#[doc = "Field `MSOFS` writer - MSOF Interrupt Set"]
pub struct MSOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFS_W<'a> {
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
#[doc = "Field `SOFS` writer - SOF Interrupt Set"]
pub struct SOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFS_W<'a> {
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
#[doc = "Field `EORSTS` writer - EORST Interrupt Set"]
pub struct EORSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTS_W<'a> {
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
#[doc = "Field `WAKEUPS` writer - WAKEUP Interrupt Set"]
pub struct WAKEUPS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPS_W<'a> {
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
#[doc = "Field `EORSMS` writer - EORSM Interrupt Set"]
pub struct EORSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMS_W<'a> {
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
#[doc = "Field `UPRSMS` writer - UPRSM Interrupt Set"]
pub struct UPRSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Set"]
    #[inline(always)]
    pub fn susps(&mut self) -> SUSPS_W {
        SUSPS_W { w: self }
    }
    #[doc = "Bit 1 - MSOF Interrupt Set"]
    #[inline(always)]
    pub fn msofs(&mut self) -> MSOFS_W {
        MSOFS_W { w: self }
    }
    #[doc = "Bit 2 - SOF Interrupt Set"]
    #[inline(always)]
    pub fn sofs(&mut self) -> SOFS_W {
        SOFS_W { w: self }
    }
    #[doc = "Bit 3 - EORST Interrupt Set"]
    #[inline(always)]
    pub fn eorsts(&mut self) -> EORSTS_W {
        EORSTS_W { w: self }
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Set"]
    #[inline(always)]
    pub fn wakeups(&mut self) -> WAKEUPS_W {
        WAKEUPS_W { w: self }
    }
    #[doc = "Bit 5 - EORSM Interrupt Set"]
    #[inline(always)]
    pub fn eorsms(&mut self) -> EORSMS_W {
        EORSMS_W { w: self }
    }
    #[doc = "Bit 6 - UPRSM Interrupt Set"]
    #[inline(always)]
    pub fn uprsms(&mut self) -> UPRSMS_W {
        UPRSMS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Set Regsiter\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udintset](index.html) module"]
pub struct UDINTSET_SPEC;
impl crate::RegisterSpec for UDINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udintset::W](W) writer structure"]
impl crate::Writable for UDINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDINTSET to value 0"]
impl crate::Resettable for UDINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
