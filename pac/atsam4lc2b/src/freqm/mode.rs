#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Reference Clock Selection"]
pub type REFSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFSEL` writer - Reference Clock Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `REFNUM` reader - Number of Reference CLock Cycles"]
pub type REFNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFNUM` writer - Number of Reference CLock Cycles"]
pub type REFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKSEL` reader - Clock Source Selection"]
pub type CLKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKSEL` writer - Clock Source Selection"]
pub type CLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 5, O>;
#[doc = "Field `REFCEN` reader - Reference Clock Enable"]
pub type REFCEN_R = crate::BitReader<bool>;
#[doc = "Field `REFCEN` writer - Reference Clock Enable"]
pub type REFCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - Number of Reference CLock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Clock Source Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Reference Clock Enable"]
    #[inline(always)]
    pub fn refcen(&self) -> REFCEN_R {
        REFCEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Number of Reference CLock Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn refnum(&mut self) -> REFNUM_W<8> {
        REFNUM_W::new(self)
    }
    #[doc = "Bits 16:20 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<16> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bit 31 - Reference Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn refcen(&mut self) -> REFCEN_W<31> {
        REFCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
