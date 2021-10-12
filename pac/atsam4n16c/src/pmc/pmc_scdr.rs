#[doc = "Register `PMC_SCDR` writer"]
pub struct W(crate::W<PMC_SCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SCDR_SPEC>;
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
impl From<crate::W<PMC_SCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Disable"]
pub struct PCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK0_W<'a> {
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
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Disable"]
pub struct PCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK1_W<'a> {
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
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Disable"]
pub struct PCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK2_W<'a> {
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
    #[doc = "Bit 8 - Programmable Clock 0 Output Disable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W {
        PCK0_W { w: self }
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Disable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W {
        PCK1_W { w: self }
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Disable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> PCK2_W {
        PCK2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scdr](index.html) module"]
pub struct PMC_SCDR_SPEC;
impl crate::RegisterSpec for PMC_SCDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_scdr::W](W) writer structure"]
impl crate::Writable for PMC_SCDR_SPEC {
    type Writer = W;
}
