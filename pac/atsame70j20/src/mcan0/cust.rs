#[doc = "Register `CUST` reader"]
pub struct R(crate::R<CUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUST` writer"]
pub struct W(crate::W<CUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUST_SPEC>;
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
impl From<crate::W<CUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSV` reader - Customer-specific Value"]
pub struct CSV_R(crate::FieldReader<u32, u32>);
impl CSV_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSV` writer - Customer-specific Value"]
pub struct CSV_W<'a> {
    w: &'a mut W,
}
impl<'a> CSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    pub fn csv(&self) -> CSV_R {
        CSV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    pub fn csv(&mut self) -> CSV_W {
        CSV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Customer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cust](index.html) module"]
pub struct CUST_SPEC;
impl crate::RegisterSpec for CUST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cust::R](R) reader structure"]
impl crate::Readable for CUST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cust::W](W) writer structure"]
impl crate::Writable for CUST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CUST to value 0"]
impl crate::Resettable for CUST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
