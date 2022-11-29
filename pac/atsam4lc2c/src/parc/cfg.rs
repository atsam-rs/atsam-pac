#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - Data Size"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Data Size"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SMODE` reader - Sampling Mode"]
pub type SMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMODE` writer - Sampling Mode"]
pub type SMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `EMODE` reader - Events Mode"]
pub type EMODE_R = crate::BitReader<bool>;
#[doc = "Field `EMODE` writer - Events Mode"]
pub type EMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `EDGE` reader - Sampling Edge Select"]
pub type EDGE_R = crate::BitReader<bool>;
#[doc = "Field `EDGE` writer - Sampling Edge Select"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `HALF` reader - Half Capture"]
pub type HALF_R = crate::BitReader<bool>;
#[doc = "Field `HALF` writer - Half Capture"]
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `ODD` reader - Odd Capture"]
pub type ODD_R = crate::BitReader<bool>;
#[doc = "Field `ODD` writer - Odd Capture"]
pub type ODD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sampling Mode"]
    #[inline(always)]
    pub fn smode(&self) -> SMODE_R {
        SMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Events Mode"]
    #[inline(always)]
    pub fn emode(&self) -> EMODE_R {
        EMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sampling Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Half Capture"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Odd Capture"]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Data Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 2:3 - Sampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smode(&mut self) -> SMODE_W<2> {
        SMODE_W::new(self)
    }
    #[doc = "Bit 4 - Events Mode"]
    #[inline(always)]
    #[must_use]
    pub fn emode(&mut self) -> EMODE_W<4> {
        EMODE_W::new(self)
    }
    #[doc = "Bit 5 - Sampling Edge Select"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<5> {
        EDGE_W::new(self)
    }
    #[doc = "Bit 6 - Half Capture"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<6> {
        HALF_W::new(self)
    }
    #[doc = "Bit 7 - Odd Capture"]
    #[inline(always)]
    #[must_use]
    pub fn odd(&mut self) -> ODD_W<7> {
        ODD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
