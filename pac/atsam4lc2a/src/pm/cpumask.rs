#[doc = "Register `CPUMASK` reader"]
pub struct R(crate::R<CPUMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CPUMASK_SPEC>> for R {
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
impl core::convert::From<crate::W<CPUMASK_SPEC>> for W {
    fn from(writer: crate::W<CPUMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OCD_` reader - OCD CPU Clock Mask"]
pub struct OCD__R(crate::FieldReader<bool, bool>);
impl OCD__R {
    pub(crate) fn new(bits: bool) -> Self {
        OCD__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCD__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCD_` writer - OCD CPU Clock Mask"]
pub struct OCD__W<'a> {
    w: &'a mut W,
}
impl<'a> OCD__W<'a> {
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
impl R {
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    pub fn ocd_(&self) -> OCD__R {
        OCD__R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OCD CPU Clock Mask"]
    #[inline(always)]
    pub fn ocd_(&mut self) -> OCD__W {
        OCD__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
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
}
#[doc = "`reset()` method sets CPUMASK to value 0x01"]
impl crate::Resettable for CPUMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
