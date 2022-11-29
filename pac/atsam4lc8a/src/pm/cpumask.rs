#[doc = "Register `CPUMASK` reader"]
pub struct R(crate::R<CPUMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUMASK` writer"]
pub struct W(crate::W<CPUMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUMASK_SPEC>;
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
impl From<crate::W<CPUMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCD_` reader - OCD CPU Clock Mask"]
pub type OCD__R = crate::BitReader<bool>;
#[doc = "Field `OCD_` writer - OCD CPU Clock Mask"]
pub type OCD__W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    pub fn ocd_(&self) -> OCD__R {
        OCD__R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ocd_(&mut self) -> OCD__W<0> {
        OCD__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpumask](index.html) module"]
pub struct CPUMASK_SPEC;
impl crate::RegisterSpec for CPUMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpumask::R](R) reader structure"]
impl crate::Readable for CPUMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpumask::W](W) writer structure"]
impl crate::Writable for CPUMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUMASK to value 0x01"]
impl crate::Resettable for CPUMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
