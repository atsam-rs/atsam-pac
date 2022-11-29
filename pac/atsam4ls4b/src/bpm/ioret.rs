#[doc = "Register `IORET` reader"]
pub struct R(crate::R<IORET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IORET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IORET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IORET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IORET` writer"]
pub struct W(crate::W<IORET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IORET_SPEC>;
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
impl From<crate::W<IORET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IORET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RET` reader - Retention on I/O lines after waking up from the BACKUP mode"]
pub type RET_R = crate::BitReader<bool>;
#[doc = "Field `RET` writer - Retention on I/O lines after waking up from the BACKUP mode"]
pub type RET_W<'a, const O: u8> = crate::BitWriter<'a, u32, IORET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Retention on I/O lines after waking up from the BACKUP mode"]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Retention on I/O lines after waking up from the BACKUP mode"]
    #[inline(always)]
    #[must_use]
    pub fn ret(&mut self) -> RET_W<0> {
        RET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Output Retention Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ioret](index.html) module"]
pub struct IORET_SPEC;
impl crate::RegisterSpec for IORET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ioret::R](R) reader structure"]
impl crate::Readable for IORET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ioret::W](W) writer structure"]
impl crate::Writable for IORET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IORET to value 0"]
impl crate::Resettable for IORET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
