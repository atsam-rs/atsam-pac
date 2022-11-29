#[doc = "Register `UERST` reader"]
pub struct R(crate::R<UERST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UERST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UERST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UERST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UERST` writer"]
pub struct W(crate::W<UERST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UERST_SPEC>;
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
impl From<crate::W<UERST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UERST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN0` reader - Endpoint0 Enable"]
pub type EPEN0_R = crate::BitReader<bool>;
#[doc = "Field `EPEN0` writer - Endpoint0 Enable"]
pub type EPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN1` reader - Endpoint1 Enable"]
pub type EPEN1_R = crate::BitReader<bool>;
#[doc = "Field `EPEN1` writer - Endpoint1 Enable"]
pub type EPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN2` reader - Endpoint2 Enable"]
pub type EPEN2_R = crate::BitReader<bool>;
#[doc = "Field `EPEN2` writer - Endpoint2 Enable"]
pub type EPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN3` reader - Endpoint3 Enable"]
pub type EPEN3_R = crate::BitReader<bool>;
#[doc = "Field `EPEN3` writer - Endpoint3 Enable"]
pub type EPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN4` reader - Endpoint4 Enable"]
pub type EPEN4_R = crate::BitReader<bool>;
#[doc = "Field `EPEN4` writer - Endpoint4 Enable"]
pub type EPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN5` reader - Endpoint5 Enable"]
pub type EPEN5_R = crate::BitReader<bool>;
#[doc = "Field `EPEN5` writer - Endpoint5 Enable"]
pub type EPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN6` reader - Endpoint6 Enable"]
pub type EPEN6_R = crate::BitReader<bool>;
#[doc = "Field `EPEN6` writer - Endpoint6 Enable"]
pub type EPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
#[doc = "Field `EPEN7` reader - Endpoint7 Enable"]
pub type EPEN7_R = crate::BitReader<bool>;
#[doc = "Field `EPEN7` writer - Endpoint7 Enable"]
pub type EPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, UERST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Endpoint0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen0(&mut self) -> EPEN0_W<0> {
        EPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen1(&mut self) -> EPEN1_W<1> {
        EPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Endpoint2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen2(&mut self) -> EPEN2_W<2> {
        EPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Endpoint3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen3(&mut self) -> EPEN3_W<3> {
        EPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Endpoint4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen4(&mut self) -> EPEN4_W<4> {
        EPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Endpoint5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen5(&mut self) -> EPEN5_W<5> {
        EPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Endpoint6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen6(&mut self) -> EPEN6_W<6> {
        EPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Endpoint7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen7(&mut self) -> EPEN7_W<7> {
        EPEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Enable/Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uerst](index.html) module"]
pub struct UERST_SPEC;
impl crate::RegisterSpec for UERST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uerst::R](R) reader structure"]
impl crate::Readable for UERST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uerst::W](W) writer structure"]
impl crate::Writable for UERST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UERST to value 0"]
impl crate::Resettable for UERST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
