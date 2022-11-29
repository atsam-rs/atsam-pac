#[doc = "Register `PBAMASK` reader"]
pub struct R(crate::R<PBAMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBAMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBAMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBAMASK` writer"]
pub struct W(crate::W<PBAMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBAMASK_SPEC>;
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
impl From<crate::W<PBAMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBAMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IISC_` reader - IISC APB Clock Enable"]
pub type IISC__R = crate::BitReader<bool>;
#[doc = "Field `IISC_` writer - IISC APB Clock Enable"]
pub type IISC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `SPI_` reader - SPI APB Clock Enable"]
pub type SPI__R = crate::BitReader<bool>;
#[doc = "Field `SPI_` writer - SPI APB Clock Enable"]
pub type SPI__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type TC0__R = crate::BitReader<bool>;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type TC0__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type TC1__R = crate::BitReader<bool>;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type TC1__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIM0_` reader - TWIM0 APB Clock Enable"]
pub type TWIM0__R = crate::BitReader<bool>;
#[doc = "Field `TWIM0_` writer - TWIM0 APB Clock Enable"]
pub type TWIM0__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIS0_` reader - TWIS0 APB Clock Enable"]
pub type TWIS0__R = crate::BitReader<bool>;
#[doc = "Field `TWIS0_` writer - TWIS0 APB Clock Enable"]
pub type TWIS0__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIM1_` reader - TWIM1 APB Clock Enable"]
pub type TWIM1__R = crate::BitReader<bool>;
#[doc = "Field `TWIM1_` writer - TWIM1 APB Clock Enable"]
pub type TWIM1__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIS1_` reader - TWIS1 APB Clock Enable"]
pub type TWIS1__R = crate::BitReader<bool>;
#[doc = "Field `TWIS1_` writer - TWIS1 APB Clock Enable"]
pub type TWIS1__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `USART0_` reader - USART0 APB Clock Enable"]
pub type USART0__R = crate::BitReader<bool>;
#[doc = "Field `USART0_` writer - USART0 APB Clock Enable"]
pub type USART0__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `USART1_` reader - USART1 APB Clock Enable"]
pub type USART1__R = crate::BitReader<bool>;
#[doc = "Field `USART1_` writer - USART1 APB Clock Enable"]
pub type USART1__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `USART2_` reader - USART2 APB Clock Enable"]
pub type USART2__R = crate::BitReader<bool>;
#[doc = "Field `USART2_` writer - USART2 APB Clock Enable"]
pub type USART2__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `USART3_` reader - USART3 APB Clock Enable"]
pub type USART3__R = crate::BitReader<bool>;
#[doc = "Field `USART3_` writer - USART3 APB Clock Enable"]
pub type USART3__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `ADCIFE_` reader - ADCIFE APB Clock Enable"]
pub type ADCIFE__R = crate::BitReader<bool>;
#[doc = "Field `ADCIFE_` writer - ADCIFE APB Clock Enable"]
pub type ADCIFE__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `DACC_` reader - DACC APB Clock Enable"]
pub type DACC__R = crate::BitReader<bool>;
#[doc = "Field `DACC_` writer - DACC APB Clock Enable"]
pub type DACC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `ACIFC_` reader - ACIFC APB Clock Enable"]
pub type ACIFC__R = crate::BitReader<bool>;
#[doc = "Field `ACIFC_` writer - ACIFC APB Clock Enable"]
pub type ACIFC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `GLOC_` reader - GLOC APB Clock Enable"]
pub type GLOC__R = crate::BitReader<bool>;
#[doc = "Field `GLOC_` writer - GLOC APB Clock Enable"]
pub type GLOC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `ABDACB_` reader - ABDACB APB Clock Enable"]
pub type ABDACB__R = crate::BitReader<bool>;
#[doc = "Field `ABDACB_` writer - ABDACB APB Clock Enable"]
pub type ABDACB__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub type TRNG__R = crate::BitReader<bool>;
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub type TRNG__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `PARC_` reader - PARC APB Clock Enable"]
pub type PARC__R = crate::BitReader<bool>;
#[doc = "Field `PARC_` writer - PARC APB Clock Enable"]
pub type PARC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `CATB_` reader - CATB APB Clock Enable"]
pub type CATB__R = crate::BitReader<bool>;
#[doc = "Field `CATB_` writer - CATB APB Clock Enable"]
pub type CATB__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIM2_` reader - TWIM2 APB Clock Enable"]
pub type TWIM2__R = crate::BitReader<bool>;
#[doc = "Field `TWIM2_` writer - TWIM2 APB Clock Enable"]
pub type TWIM2__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
#[doc = "Field `TWIM3_` reader - TWIM3 APB Clock Enable"]
pub type TWIM3__R = crate::BitReader<bool>;
#[doc = "Field `TWIM3_` writer - TWIM3 APB Clock Enable"]
pub type TWIM3__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBAMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    pub fn iisc_(&self) -> IISC__R {
        IISC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    pub fn spi_(&self) -> SPI__R {
        SPI__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    pub fn twim0_(&self) -> TWIM0__R {
        TWIM0__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    pub fn twis0_(&self) -> TWIS0__R {
        TWIS0__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    pub fn twim1_(&self) -> TWIM1__R {
        TWIM1__R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    pub fn twis1_(&self) -> TWIS1__R {
        TWIS1__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    pub fn usart0_(&self) -> USART0__R {
        USART0__R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    pub fn usart1_(&self) -> USART1__R {
        USART1__R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    pub fn usart2_(&self) -> USART2__R {
        USART2__R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    pub fn usart3_(&self) -> USART3__R {
        USART3__R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    pub fn adcife_(&self) -> ADCIFE__R {
        ADCIFE__R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    pub fn dacc_(&self) -> DACC__R {
        DACC__R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    pub fn acifc_(&self) -> ACIFC__R {
        ACIFC__R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    pub fn gloc_(&self) -> GLOC__R {
        GLOC__R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    pub fn abdacb_(&self) -> ABDACB__R {
        ABDACB__R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    pub fn parc_(&self) -> PARC__R {
        PARC__R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    pub fn catb_(&self) -> CATB__R {
        CATB__R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    pub fn twim2_(&self) -> TWIM2__R {
        TWIM2__R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    pub fn twim3_(&self) -> TWIM3__R {
        TWIM3__R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iisc_(&mut self) -> IISC__W<0> {
        IISC__W::new(self)
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_(&mut self) -> SPI__W<1> {
        SPI__W::new(self)
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> TC0__W<2> {
        TC0__W::new(self)
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> TC1__W<3> {
        TC1__W::new(self)
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twim0_(&mut self) -> TWIM0__W<4> {
        TWIM0__W::new(self)
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twis0_(&mut self) -> TWIS0__W<5> {
        TWIS0__W::new(self)
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twim1_(&mut self) -> TWIM1__W<6> {
        TWIM1__W::new(self)
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twis1_(&mut self) -> TWIS1__W<7> {
        TWIS1__W::new(self)
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart0_(&mut self) -> USART0__W<8> {
        USART0__W::new(self)
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_(&mut self) -> USART1__W<9> {
        USART1__W::new(self)
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart2_(&mut self) -> USART2__W<10> {
        USART2__W::new(self)
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_(&mut self) -> USART3__W<11> {
        USART3__W::new(self)
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adcife_(&mut self) -> ADCIFE__W<12> {
        ADCIFE__W::new(self)
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacc_(&mut self) -> DACC__W<13> {
        DACC__W::new(self)
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acifc_(&mut self) -> ACIFC__W<14> {
        ACIFC__W::new(self)
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gloc_(&mut self) -> GLOC__W<15> {
        GLOC__W::new(self)
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn abdacb_(&mut self) -> ABDACB__W<16> {
        ABDACB__W::new(self)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trng_(&mut self) -> TRNG__W<17> {
        TRNG__W::new(self)
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn parc_(&mut self) -> PARC__W<18> {
        PARC__W::new(self)
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn catb_(&mut self) -> CATB__W<19> {
        CATB__W::new(self)
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twim2_(&mut self) -> TWIM2__W<21> {
        TWIM2__W::new(self)
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn twim3_(&mut self) -> TWIM3__W<22> {
        TWIM3__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBA Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbamask](index.html) module"]
pub struct PBAMASK_SPEC;
impl crate::RegisterSpec for PBAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbamask::R](R) reader structure"]
impl crate::Readable for PBAMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbamask::W](W) writer structure"]
impl crate::Writable for PBAMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBAMASK to value 0"]
impl crate::Resettable for PBAMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
