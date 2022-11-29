#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Module Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Module Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RUN` reader - Start Operation"]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - Start Operation"]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `IIDLE` reader - Initialize Idle Value"]
pub type IIDLE_R = crate::BitReader<bool>;
#[doc = "Field `IIDLE` writer - Initialize Idle Value"]
pub type IIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ETRIG` reader - Event Triggered Operation"]
pub type ETRIG_R = crate::BitReader<bool>;
#[doc = "Field `ETRIG` writer - Event Triggered Operation"]
pub type ETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `INTRES` reader - Internal Resistors"]
pub type INTRES_R = crate::BitReader<bool>;
#[doc = "Field `INTRES` writer - Internal Resistors"]
pub type INTRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CKSEL` reader - Clock Select"]
pub type CKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CKSEL` writer - Clock Select"]
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<bool>;
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ESAMPLES` reader - Number of Event Samples"]
pub type ESAMPLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ESAMPLES` writer - Number of Event Samples"]
pub type ESAMPLES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CHARGET` reader - Charge Time"]
pub type CHARGET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHARGET` writer - Charge Time"]
pub type CHARGET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Operation"]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Initialize Idle Value"]
    #[inline(always)]
    pub fn iidle(&self) -> IIDLE_R {
        IIDLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Triggered Operation"]
    #[inline(always)]
    pub fn etrig(&self) -> ETRIG_R {
        ETRIG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal Resistors"]
    #[inline(always)]
    pub fn intres(&self) -> INTRES_R {
        INTRES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Select"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Number of Event Samples"]
    #[inline(always)]
    pub fn esamples(&self) -> ESAMPLES_R {
        ESAMPLES_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - Charge Time"]
    #[inline(always)]
    pub fn charget(&self) -> CHARGET_R {
        CHARGET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Start Operation"]
    #[inline(always)]
    #[must_use]
    pub fn run(&mut self) -> RUN_W<1> {
        RUN_W::new(self)
    }
    #[doc = "Bit 2 - Initialize Idle Value"]
    #[inline(always)]
    #[must_use]
    pub fn iidle(&mut self) -> IIDLE_W<2> {
        IIDLE_W::new(self)
    }
    #[doc = "Bit 3 - Event Triggered Operation"]
    #[inline(always)]
    #[must_use]
    pub fn etrig(&mut self) -> ETRIG_W<3> {
        ETRIG_W::new(self)
    }
    #[doc = "Bit 4 - Internal Resistors"]
    #[inline(always)]
    #[must_use]
    pub fn intres(&mut self) -> INTRES_W<4> {
        INTRES_W::new(self)
    }
    #[doc = "Bit 5 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<5> {
        CKSEL_W::new(self)
    }
    #[doc = "Bit 6 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<6> {
        DIFF_W::new(self)
    }
    #[doc = "Bit 7 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<7> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 8:14 - Number of Event Samples"]
    #[inline(always)]
    #[must_use]
    pub fn esamples(&mut self) -> ESAMPLES_W<8> {
        ESAMPLES_W::new(self)
    }
    #[doc = "Bits 16:19 - Charge Time"]
    #[inline(always)]
    #[must_use]
    pub fn charget(&mut self) -> CHARGET_W<16> {
        CHARGET_W::new(self)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<31> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
