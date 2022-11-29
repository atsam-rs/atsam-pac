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
#[doc = "Field `ACTEST0` reader - AC0 Output Override Value"]
pub type ACTEST0_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST0` writer - AC0 Output Override Value"]
pub type ACTEST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST1` reader - AC1 Output Override Value"]
pub type ACTEST1_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST1` writer - AC1 Output Override Value"]
pub type ACTEST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST2` reader - AC2 Output Override Value"]
pub type ACTEST2_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST2` writer - AC2 Output Override Value"]
pub type ACTEST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST3` reader - AC3 Output Override Value"]
pub type ACTEST3_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST3` writer - AC3 Output Override Value"]
pub type ACTEST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST4` reader - AC4 Output Override Value"]
pub type ACTEST4_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST4` writer - AC4 Output Override Value"]
pub type ACTEST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST5` reader - AC5 Output Override Value"]
pub type ACTEST5_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST5` writer - AC5 Output Override Value"]
pub type ACTEST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST6` reader - AC6 Output Override Value"]
pub type ACTEST6_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST6` writer - AC6 Output Override Value"]
pub type ACTEST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
#[doc = "Field `ACTEST7` reader - AC7 Output Override Value"]
pub type ACTEST7_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST7` writer - AC7 Output Override Value"]
pub type ACTEST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    pub fn actest0(&self) -> ACTEST0_R {
        ACTEST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    pub fn actest1(&self) -> ACTEST1_R {
        ACTEST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    pub fn actest2(&self) -> ACTEST2_R {
        ACTEST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    pub fn actest3(&self) -> ACTEST3_R {
        ACTEST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    pub fn actest4(&self) -> ACTEST4_R {
        ACTEST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    pub fn actest5(&self) -> ACTEST5_R {
        ACTEST5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    pub fn actest6(&self) -> ACTEST6_R {
        ACTEST6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    pub fn actest7(&self) -> ACTEST7_R {
        ACTEST7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest0(&mut self) -> ACTEST0_W<0> {
        ACTEST0_W::new(self)
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest1(&mut self) -> ACTEST1_W<1> {
        ACTEST1_W::new(self)
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest2(&mut self) -> ACTEST2_W<2> {
        ACTEST2_W::new(self)
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest3(&mut self) -> ACTEST3_W<3> {
        ACTEST3_W::new(self)
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest4(&mut self) -> ACTEST4_W<4> {
        ACTEST4_W::new(self)
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest5(&mut self) -> ACTEST5_W<5> {
        ACTEST5_W::new(self)
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest6(&mut self) -> ACTEST6_W<6> {
        ACTEST6_W::new(self)
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    #[must_use]
    pub fn actest7(&mut self) -> ACTEST7_W<7> {
        ACTEST7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
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
