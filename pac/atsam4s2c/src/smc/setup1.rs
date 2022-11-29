#[doc = "Register `SETUP1` reader"]
pub struct R(crate::R<SETUP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETUP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETUP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETUP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETUP1` writer"]
pub struct W(crate::W<SETUP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETUP1_SPEC>;
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
impl From<crate::W<SETUP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETUP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_SETUP` reader - NWE Setup Length"]
pub type NWE_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NWE_SETUP` writer - NWE Setup Length"]
pub type NWE_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP1_SPEC, u8, u8, 6, O>;
#[doc = "Field `NCS_WR_SETUP` reader - NCS Setup Length in WRITE Access"]
pub type NCS_WR_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCS_WR_SETUP` writer - NCS Setup Length in WRITE Access"]
pub type NCS_WR_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP1_SPEC, u8, u8, 6, O>;
#[doc = "Field `NRD_SETUP` reader - NRD Setup Length"]
pub type NRD_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NRD_SETUP` writer - NRD Setup Length"]
pub type NRD_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP1_SPEC, u8, u8, 6, O>;
#[doc = "Field `NCS_RD_SETUP` reader - NCS Setup Length in READ Access"]
pub type NCS_RD_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NCS_RD_SETUP` writer - NCS Setup Length in READ Access"]
pub type NCS_RD_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETUP1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&self) -> NWE_SETUP_R {
        NWE_SETUP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&self) -> NCS_WR_SETUP_R {
        NCS_WR_SETUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&self) -> NRD_SETUP_R {
        NRD_SETUP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&self) -> NCS_RD_SETUP_R {
        NCS_RD_SETUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nwe_setup(&mut self) -> NWE_SETUP_W<0> {
        NWE_SETUP_W::new(self)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_wr_setup(&mut self) -> NCS_WR_SETUP_W<8> {
        NCS_WR_SETUP_W::new(self)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    #[must_use]
    pub fn nrd_setup(&mut self) -> NRD_SETUP_W<16> {
        NRD_SETUP_W::new(self)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    #[must_use]
    pub fn ncs_rd_setup(&mut self) -> NCS_RD_SETUP_W<24> {
        NCS_RD_SETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Setup Register (CS_number = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setup1](index.html) module"]
pub struct SETUP1_SPEC;
impl crate::RegisterSpec for SETUP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setup1::R](R) reader structure"]
impl crate::Readable for SETUP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setup1::W](W) writer structure"]
impl crate::Writable for SETUP1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETUP1 to value 0x0101_0101"]
impl crate::Resettable for SETUP1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0101;
}
