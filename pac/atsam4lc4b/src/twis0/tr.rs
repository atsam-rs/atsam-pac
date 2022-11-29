#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWS` reader - SMBus Tlow:sext Cycles"]
pub type TLOWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLOWS` writer - SMBus Tlow:sext Cycles"]
pub type TLOWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TTOUT` reader - SMBus Ttimeout Cycles"]
pub type TTOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TTOUT` writer - SMBus Ttimeout Cycles"]
pub type TTOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SUDAT` reader - Data Setup Cycles"]
pub type SUDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUDAT` writer - Data Setup Cycles"]
pub type SUDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXP` reader - Clock Prescaler"]
pub type EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXP` writer - Clock Prescaler"]
pub type EXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    pub fn ttout(&self) -> TTOUT_R {
        TTOUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    pub fn sudat(&self) -> SUDAT_R {
        SUDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SMBus Tlow:sext Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlows(&mut self) -> TLOWS_W<0> {
        TLOWS_W::new(self)
    }
    #[doc = "Bits 8:15 - SMBus Ttimeout Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn ttout(&mut self) -> TTOUT_W<8> {
        TTOUT_W::new(self)
    }
    #[doc = "Bits 16:23 - Data Setup Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn sudat(&mut self) -> SUDAT_W<16> {
        SUDAT_W::new(self)
    }
    #[doc = "Bits 28:31 - Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn exp(&mut self) -> EXP_W<28> {
        EXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
