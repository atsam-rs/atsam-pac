#[doc = "Register `PMC_PCDR1` writer"]
pub struct W(crate::W<PMC_PCDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCDR1_SPEC>;
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
impl From<crate::W<PMC_PCDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID32` writer - Peripheral Clock 32 Disable"]
pub struct PID32_W<'a> {
    w: &'a mut W,
}
impl<'a> PID32_W<'a> {
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
#[doc = "Field `PID33` writer - Peripheral Clock 33 Disable"]
pub struct PID33_W<'a> {
    w: &'a mut W,
}
impl<'a> PID33_W<'a> {
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
#[doc = "Field `PID34` writer - Peripheral Clock 34 Disable"]
pub struct PID34_W<'a> {
    w: &'a mut W,
}
impl<'a> PID34_W<'a> {
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
#[doc = "Field `PID35` writer - Peripheral Clock 35 Disable"]
pub struct PID35_W<'a> {
    w: &'a mut W,
}
impl<'a> PID35_W<'a> {
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
#[doc = "Field `PID36` writer - Peripheral Clock 36 Disable"]
pub struct PID36_W<'a> {
    w: &'a mut W,
}
impl<'a> PID36_W<'a> {
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
#[doc = "Field `PID37` writer - Peripheral Clock 37 Disable"]
pub struct PID37_W<'a> {
    w: &'a mut W,
}
impl<'a> PID37_W<'a> {
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
#[doc = "Field `PID38` writer - Peripheral Clock 38 Disable"]
pub struct PID38_W<'a> {
    w: &'a mut W,
}
impl<'a> PID38_W<'a> {
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
#[doc = "Field `PID39` writer - Peripheral Clock 39 Disable"]
pub struct PID39_W<'a> {
    w: &'a mut W,
}
impl<'a> PID39_W<'a> {
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
#[doc = "Field `PID40` writer - Peripheral Clock 40 Disable"]
pub struct PID40_W<'a> {
    w: &'a mut W,
}
impl<'a> PID40_W<'a> {
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
#[doc = "Field `PID41` writer - Peripheral Clock 41 Disable"]
pub struct PID41_W<'a> {
    w: &'a mut W,
}
impl<'a> PID41_W<'a> {
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
#[doc = "Field `PID42` writer - Peripheral Clock 42 Disable"]
pub struct PID42_W<'a> {
    w: &'a mut W,
}
impl<'a> PID42_W<'a> {
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
#[doc = "Field `PID43` writer - Peripheral Clock 43 Disable"]
pub struct PID43_W<'a> {
    w: &'a mut W,
}
impl<'a> PID43_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PID44` writer - Peripheral Clock 44 Disable"]
pub struct PID44_W<'a> {
    w: &'a mut W,
}
impl<'a> PID44_W<'a> {
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
#[doc = "Field `PID45` writer - Peripheral Clock 45 Disable"]
pub struct PID45_W<'a> {
    w: &'a mut W,
}
impl<'a> PID45_W<'a> {
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
#[doc = "Field `PID46` writer - Peripheral Clock 46 Disable"]
pub struct PID46_W<'a> {
    w: &'a mut W,
}
impl<'a> PID46_W<'a> {
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
#[doc = "Field `PID47` writer - Peripheral Clock 47 Disable"]
pub struct PID47_W<'a> {
    w: &'a mut W,
}
impl<'a> PID47_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Disable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> PID32_W {
        PID32_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Disable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> PID33_W {
        PID33_W { w: self }
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Disable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> PID34_W {
        PID34_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Disable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> PID35_W {
        PID35_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral Clock 36 Disable"]
    #[inline(always)]
    pub fn pid36(&mut self) -> PID36_W {
        PID36_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Disable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> PID37_W {
        PID37_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral Clock 38 Disable"]
    #[inline(always)]
    pub fn pid38(&mut self) -> PID38_W {
        PID38_W { w: self }
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Disable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> PID39_W {
        PID39_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Disable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> PID40_W {
        PID40_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Disable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> PID41_W {
        PID41_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Disable"]
    #[inline(always)]
    pub fn pid42(&mut self) -> PID42_W {
        PID42_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Disable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> PID43_W {
        PID43_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Disable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> PID44_W {
        PID44_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Disable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> PID45_W {
        PID45_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Disable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> PID46_W {
        PID46_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Disable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> PID47_W {
        PID47_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcdr1](index.html) module"]
pub struct PMC_PCDR1_SPEC;
impl crate::RegisterSpec for PMC_PCDR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcdr1::W](W) writer structure"]
impl crate::Writable for PMC_PCDR1_SPEC {
    type Writer = W;
}
