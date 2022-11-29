#[doc = "Register `HSBMASK` reader"]
pub struct R(crate::R<HSBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSBMASK` writer"]
pub struct W(crate::W<HSBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSBMASK_SPEC>;
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
impl From<crate::W<HSBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCA_` reader - PDCA HSB Clock Mask"]
pub type PDCA__R = crate::BitReader<bool>;
#[doc = "Field `PDCA_` writer - PDCA HSB Clock Mask"]
pub type PDCA__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HFLASHC_` reader - HFLASHC HSB Clock Mask"]
pub type HFLASHC__R = crate::BitReader<bool>;
#[doc = "Field `HFLASHC_` writer - HFLASHC HSB Clock Mask"]
pub type HFLASHC__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HRAMC1_` reader - HRAMC1 HSB Clock Mask"]
pub type HRAMC1__R = crate::BitReader<bool>;
#[doc = "Field `HRAMC1_` writer - HRAMC1 HSB Clock Mask"]
pub type HRAMC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `USBC_` reader - USBC HSB Clock Mask"]
pub type USBC__R = crate::BitReader<bool>;
#[doc = "Field `USBC_` writer - USBC HSB Clock Mask"]
pub type USBC__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `CRCCU_` reader - CRCCU HSB Clock Mask"]
pub type CRCCU__R = crate::BitReader<bool>;
#[doc = "Field `CRCCU_` writer - CRCCU HSB Clock Mask"]
pub type CRCCU__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HTOP0_` reader - HTOP0 HSB Clock Mask"]
pub type HTOP0__R = crate::BitReader<bool>;
#[doc = "Field `HTOP0_` writer - HTOP0 HSB Clock Mask"]
pub type HTOP0__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HTOP1_` reader - HTOP1 HSB Clock Mask"]
pub type HTOP1__R = crate::BitReader<bool>;
#[doc = "Field `HTOP1_` writer - HTOP1 HSB Clock Mask"]
pub type HTOP1__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HTOP2_` reader - HTOP2 HSB Clock Mask"]
pub type HTOP2__R = crate::BitReader<bool>;
#[doc = "Field `HTOP2_` writer - HTOP2 HSB Clock Mask"]
pub type HTOP2__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `HTOP3_` reader - HTOP3 HSB Clock Mask"]
pub type HTOP3__R = crate::BitReader<bool>;
#[doc = "Field `HTOP3_` writer - HTOP3 HSB Clock Mask"]
pub type HTOP3__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
#[doc = "Field `AESA_` reader - AESA HSB Clock Mask"]
pub type AESA__R = crate::BitReader<bool>;
#[doc = "Field `AESA_` writer - AESA HSB Clock Mask"]
pub type AESA__W<'a, const O: u8> = crate::BitWriter<'a, u32, HSBMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    pub fn hramc1_(&self) -> HRAMC1__R {
        HRAMC1__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop0_(&self) -> HTOP0__R {
        HTOP0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop1_(&self) -> HTOP1__R {
        HTOP1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop2_(&self) -> HTOP2__R {
        HTOP2__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop3_(&self) -> HTOP3__R {
        HTOP3__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AESA HSB Clock Mask"]
    #[inline(always)]
    pub fn aesa_(&self) -> AESA__R {
        AESA__R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pdca_(&mut self) -> PDCA__W<0> {
        PDCA__W::new(self)
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hflashc_(&mut self) -> HFLASHC__W<1> {
        HFLASHC__W::new(self)
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hramc1_(&mut self) -> HRAMC1__W<2> {
        HRAMC1__W::new(self)
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbc_(&mut self) -> USBC__W<3> {
        USBC__W::new(self)
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn crccu_(&mut self) -> CRCCU__W<4> {
        CRCCU__W::new(self)
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn htop0_(&mut self) -> HTOP0__W<5> {
        HTOP0__W::new(self)
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn htop1_(&mut self) -> HTOP1__W<6> {
        HTOP1__W::new(self)
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn htop2_(&mut self) -> HTOP2__W<7> {
        HTOP2__W::new(self)
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn htop3_(&mut self) -> HTOP3__W<8> {
        HTOP3__W::new(self)
    }
    #[doc = "Bit 9 - AESA HSB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn aesa_(&mut self) -> AESA__W<9> {
        AESA__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbmask](index.html) module"]
pub struct HSBMASK_SPEC;
impl crate::RegisterSpec for HSBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsbmask::R](R) reader structure"]
impl crate::Readable for HSBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsbmask::W](W) writer structure"]
impl crate::Writable for HSBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSBMASK to value 0x01e2"]
impl crate::Resettable for HSBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01e2;
}
