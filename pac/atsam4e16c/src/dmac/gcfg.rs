#[doc = "Register `GCFG` reader"]
pub struct R(crate::R<GCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCFG` writer"]
pub struct W(crate::W<GCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCFG_SPEC>;
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
impl From<crate::W<GCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB_CFG` reader - Arbiter Configuration"]
pub type ARB_CFG_R = crate::BitReader<ARB_CFG_A>;
#[doc = "Arbiter Configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARB_CFG_A {
    #[doc = "0: Fixed priority arbiter (see \"Basic Definitions\" )"]
    FIXED = 0,
    #[doc = "1: Modified round robin arbiter."]
    ROUND_ROBIN = 1,
}
impl From<ARB_CFG_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_CFG_A) -> Self {
        variant as u8 != 0
    }
}
impl ARB_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_CFG_A {
        match self.bits {
            false => ARB_CFG_A::FIXED,
            true => ARB_CFG_A::ROUND_ROBIN,
        }
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == ARB_CFG_A::FIXED
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == ARB_CFG_A::ROUND_ROBIN
    }
}
#[doc = "Field `ARB_CFG` writer - Arbiter Configuration"]
pub type ARB_CFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCFG_SPEC, ARB_CFG_A, O>;
impl<'a, const O: u8> ARB_CFG_W<'a, O> {
    #[doc = "Fixed priority arbiter (see \"Basic Definitions\" )"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(ARB_CFG_A::FIXED)
    }
    #[doc = "Modified round robin arbiter."]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARB_CFG_A::ROUND_ROBIN)
    }
}
impl R {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    pub fn arb_cfg(&self) -> ARB_CFG_R {
        ARB_CFG_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Arbiter Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn arb_cfg(&mut self) -> ARB_CFG_W<4> {
        ARB_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfg](index.html) module"]
pub struct GCFG_SPEC;
impl crate::RegisterSpec for GCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcfg::R](R) reader structure"]
impl crate::Readable for GCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcfg::W](W) writer structure"]
impl crate::Writable for GCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCFG to value 0x10"]
impl crate::Resettable for GCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
