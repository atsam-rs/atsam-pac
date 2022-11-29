#[doc = "Register `IDLE` reader"]
pub struct R(crate::R<IDLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLE` writer"]
pub struct W(crate::W<IDLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLE_SPEC>;
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
impl From<crate::W<IDLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIDLE` reader - Fractional Sensor Idle"]
pub type FIDLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIDLE` writer - Fractional Sensor Idle"]
pub type FIDLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDLE_SPEC, u16, u16, 12, O>;
#[doc = "Field `RIDLE` reader - Integer Sensor Idle"]
pub type RIDLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RIDLE` writer - Integer Sensor Idle"]
pub type RIDLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDLE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:11 - Fractional Sensor Idle"]
    #[inline(always)]
    pub fn fidle(&self) -> FIDLE_R {
        FIDLE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - Integer Sensor Idle"]
    #[inline(always)]
    pub fn ridle(&self) -> RIDLE_R {
        RIDLE_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional Sensor Idle"]
    #[inline(always)]
    #[must_use]
    pub fn fidle(&mut self) -> FIDLE_W<0> {
        FIDLE_W::new(self)
    }
    #[doc = "Bits 12:27 - Integer Sensor Idle"]
    #[inline(always)]
    #[must_use]
    pub fn ridle(&mut self) -> RIDLE_W<12> {
        RIDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sensor Idle Level\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle](index.html) module"]
pub struct IDLE_SPEC;
impl crate::RegisterSpec for IDLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle::R](R) reader structure"]
impl crate::Readable for IDLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idle::W](W) writer structure"]
impl crate::Writable for IDLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLE to value 0"]
impl crate::Resettable for IDLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
