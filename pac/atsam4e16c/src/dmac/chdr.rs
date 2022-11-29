#[doc = "Register `CHDR` writer"]
pub struct W(crate::W<CHDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDR_SPEC>;
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
impl From<crate::W<CHDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS0` writer - Disable \\[3:0\\]"]
pub type DIS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `DIS1` writer - Disable \\[3:0\\]"]
pub type DIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `DIS2` writer - Disable \\[3:0\\]"]
pub type DIS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `DIS3` writer - Disable \\[3:0\\]"]
pub type DIS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `RES0` writer - Resume \\[3:0\\]"]
pub type RES0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `RES1` writer - Resume \\[3:0\\]"]
pub type RES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `RES2` writer - Resume \\[3:0\\]"]
pub type RES2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
#[doc = "Field `RES3` writer - Resume \\[3:0\\]"]
pub type RES3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis0(&mut self) -> DIS0_W<0> {
        DIS0_W::new(self)
    }
    #[doc = "Bit 1 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis1(&mut self) -> DIS1_W<1> {
        DIS1_W::new(self)
    }
    #[doc = "Bit 2 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis2(&mut self) -> DIS2_W<2> {
        DIS2_W::new(self)
    }
    #[doc = "Bit 3 - Disable \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn dis3(&mut self) -> DIS3_W<3> {
        DIS3_W::new(self)
    }
    #[doc = "Bit 8 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res0(&mut self) -> RES0_W<8> {
        RES0_W::new(self)
    }
    #[doc = "Bit 9 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res1(&mut self) -> RES1_W<9> {
        RES1_W::new(self)
    }
    #[doc = "Bit 10 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res2(&mut self) -> RES2_W<10> {
        RES2_W::new(self)
    }
    #[doc = "Bit 11 - Resume \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn res3(&mut self) -> RES3_W<11> {
        RES3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Handler Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdr](index.html) module"]
pub struct CHDR_SPEC;
impl crate::RegisterSpec for CHDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chdr::W](W) writer structure"]
impl crate::Writable for CHDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
