#[doc = "Register `PULSE0` reader"]
pub struct R(crate::R<PULSE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSE0` writer"]
pub struct W(crate::W<PULSE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSE0_SPEC>;
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
impl From<crate::W<PULSE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub type NWE_PULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub type NWE_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULSE0_SPEC, u8, u8, 7, O>;
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub type NCS_WR_PULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub type NCS_WR_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULSE0_SPEC, u8, u8, 7, O>;
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub type NRD_PULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub type NRD_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULSE0_SPEC, u8, u8, 7, O>;
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub type NCS_RD_PULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub type NCS_RD_PULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PULSE0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&self) -> NWE_PULSE_R {
        NWE_PULSE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&self) -> NCS_WR_PULSE_R {
        NCS_WR_PULSE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&self) -> NRD_PULSE_R {
        NRD_PULSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&self) -> NCS_RD_PULSE_R {
        NCS_RD_PULSE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_pulse(&mut self) -> NWE_PULSE_W<0> {
        NWE_PULSE_W::new(self)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_pulse(&mut self) -> NCS_WR_PULSE_W<8> {
        NCS_WR_PULSE_W::new(self)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_pulse(&mut self) -> NRD_PULSE_W<16> {
        NRD_PULSE_W::new(self)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_pulse(&mut self) -> NCS_RD_PULSE_W<24> {
        NCS_RD_PULSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Pulse Register (CS_number = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulse0](index.html) module"]
pub struct PULSE0_SPEC;
impl crate::RegisterSpec for PULSE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulse0::R](R) reader structure"]
impl crate::Readable for PULSE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulse0::W](W) writer structure"]
impl crate::Writable for PULSE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PULSE0 to value 0x0101_0101"]
impl crate::Resettable for PULSE0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0101;
}
