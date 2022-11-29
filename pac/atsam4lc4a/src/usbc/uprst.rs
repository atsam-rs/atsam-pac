#[doc = "Register `UPRST` reader"]
pub struct R(crate::R<UPRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPRST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPRST` writer"]
pub struct W(crate::W<UPRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPRST_SPEC>;
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
impl From<crate::W<UPRST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN0` reader - Pipe0 Enable"]
pub type PEN0_R = crate::BitReader<bool>;
#[doc = "Field `PEN0` writer - Pipe0 Enable"]
pub type PEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN1` reader - Pipe1 Enable"]
pub type PEN1_R = crate::BitReader<bool>;
#[doc = "Field `PEN1` writer - Pipe1 Enable"]
pub type PEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN2` reader - Pipe2 Enable"]
pub type PEN2_R = crate::BitReader<bool>;
#[doc = "Field `PEN2` writer - Pipe2 Enable"]
pub type PEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN3` reader - Pipe3 Enable"]
pub type PEN3_R = crate::BitReader<bool>;
#[doc = "Field `PEN3` writer - Pipe3 Enable"]
pub type PEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN4` reader - Pipe4 Enable"]
pub type PEN4_R = crate::BitReader<bool>;
#[doc = "Field `PEN4` writer - Pipe4 Enable"]
pub type PEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN5` reader - Pipe5 Enable"]
pub type PEN5_R = crate::BitReader<bool>;
#[doc = "Field `PEN5` writer - Pipe5 Enable"]
pub type PEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN6` reader - Pipe6 Enable"]
pub type PEN6_R = crate::BitReader<bool>;
#[doc = "Field `PEN6` writer - Pipe6 Enable"]
pub type PEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
#[doc = "Field `PEN7` reader - Pipe7 Enable"]
pub type PEN7_R = crate::BitReader<bool>;
#[doc = "Field `PEN7` writer - Pipe7 Enable"]
pub type PEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, UPRST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen0(&mut self) -> PEN0_W<0> {
        PEN0_W::new(self)
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen1(&mut self) -> PEN1_W<1> {
        PEN1_W::new(self)
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen2(&mut self) -> PEN2_W<2> {
        PEN2_W::new(self)
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen3(&mut self) -> PEN3_W<3> {
        PEN3_W::new(self)
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen4(&mut self) -> PEN4_W<4> {
        PEN4_W::new(self)
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen5(&mut self) -> PEN5_W<5> {
        PEN5_W::new(self)
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen6(&mut self) -> PEN6_W<6> {
        PEN6_W::new(self)
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen7(&mut self) -> PEN7_W<7> {
        PEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uprst](index.html) module"]
pub struct UPRST_SPEC;
impl crate::RegisterSpec for UPRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uprst::R](R) reader structure"]
impl crate::Readable for UPRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uprst::W](W) writer structure"]
impl crate::Writable for UPRST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPRST to value 0"]
impl crate::Resettable for UPRST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
