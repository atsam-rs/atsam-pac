#[doc = "Register `UPINRQ0` reader"]
pub struct R(crate::R<UPINRQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPINRQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPINRQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPINRQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPINRQ0` writer"]
pub struct W(crate::W<UPINRQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPINRQ0_SPEC>;
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
impl From<crate::W<UPINRQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPINRQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INRQ` reader - IN Request Number before Freeze"]
pub type INRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INRQ` writer - IN Request Number before Freeze"]
pub type INRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UPINRQ0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INMODE` reader - IN Request Mode"]
pub type INMODE_R = crate::BitReader<bool>;
#[doc = "Field `INMODE` writer - IN Request Mode"]
pub type INMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPINRQ0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<0> {
        INRQ_W::new(self)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    #[must_use]
    pub fn inmode(&mut self) -> INMODE_W<8> {
        INMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe In Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upinrq0](index.html) module"]
pub struct UPINRQ0_SPEC;
impl crate::RegisterSpec for UPINRQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upinrq0::R](R) reader structure"]
impl crate::Readable for UPINRQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upinrq0::W](W) writer structure"]
impl crate::Writable for UPINRQ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPINRQ0 to value 0x01"]
impl crate::Resettable for UPINRQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
