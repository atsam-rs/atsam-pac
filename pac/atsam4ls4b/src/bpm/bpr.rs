#[doc = "Register `BPR` reader"]
pub struct R(crate::R<BPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BPR` writer"]
pub struct W(crate::W<BPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPR_SPEC>;
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
impl From<crate::W<BPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNPSPB` reader - Run Mode Power Scaling Preset Bypass"]
pub type RUNPSPB_R = crate::BitReader<bool>;
#[doc = "Field `RUNPSPB` writer - Run Mode Power Scaling Preset Bypass"]
pub type RUNPSPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `PSMPSPB` reader - Power Save Mode Power Scaling Preset Bypass"]
pub type PSMPSPB_R = crate::BitReader<bool>;
#[doc = "Field `PSMPSPB` writer - Power Save Mode Power Scaling Preset Bypass"]
pub type PSMPSPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `SEQSTN` reader - Sequencial Startup from ULP (Active Low)"]
pub type SEQSTN_R = crate::BitReader<bool>;
#[doc = "Field `SEQSTN` writer - Sequencial Startup from ULP (Active Low)"]
pub type SEQSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `PSBTD` reader - Power Scaling Bias Timing Disable"]
pub type PSBTD_R = crate::BitReader<bool>;
#[doc = "Field `PSBTD` writer - Power Scaling Bias Timing Disable"]
pub type PSBTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `PSHFD` reader - Power Scaling Halt Flash Until VREGOK Disable"]
pub type PSHFD_R = crate::BitReader<bool>;
#[doc = "Field `PSHFD` writer - Power Scaling Halt Flash Until VREGOK Disable"]
pub type PSHFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `DLYRSTD` reader - Delaying Reset Disable"]
pub type DLYRSTD_R = crate::BitReader<bool>;
#[doc = "Field `DLYRSTD` writer - Delaying Reset Disable"]
pub type DLYRSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `BIASSEN` reader - Bias Switch Enable"]
pub type BIASSEN_R = crate::BitReader<bool>;
#[doc = "Field `BIASSEN` writer - Bias Switch Enable"]
pub type BIASSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `LATSEN` reader - Latdel Switch Enable"]
pub type LATSEN_R = crate::BitReader<bool>;
#[doc = "Field `LATSEN` writer - Latdel Switch Enable"]
pub type LATSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `BOD18CONT` reader - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
pub type BOD18CONT_R = crate::BitReader<bool>;
#[doc = "Field `BOD18CONT` writer - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
pub type BOD18CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `POBS` reader - Pico Uart Observability"]
pub type POBS_R = crate::BitReader<bool>;
#[doc = "Field `POBS` writer - Pico Uart Observability"]
pub type POBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `FFFW` reader - Force Flash Fast Wakeup"]
pub type FFFW_R = crate::BitReader<bool>;
#[doc = "Field `FFFW` writer - Force Flash Fast Wakeup"]
pub type FFFW_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `FBRDYEN` reader - Flash Bias Ready Enable"]
pub type FBRDYEN_R = crate::BitReader<bool>;
#[doc = "Field `FBRDYEN` writer - Flash Bias Ready Enable"]
pub type FBRDYEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
#[doc = "Field `FVREFSEN` reader - Flash Vref Switch Enable"]
pub type FVREFSEN_R = crate::BitReader<bool>;
#[doc = "Field `FVREFSEN` writer - Flash Vref Switch Enable"]
pub type FVREFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn runpspb(&self) -> RUNPSPB_R {
        RUNPSPB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn psmpspb(&self) -> PSMPSPB_R {
        PSMPSPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    pub fn seqstn(&self) -> SEQSTN_R {
        SEQSTN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    pub fn psbtd(&self) -> PSBTD_R {
        PSBTD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    pub fn pshfd(&self) -> PSHFD_R {
        PSHFD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    pub fn dlyrstd(&self) -> DLYRSTD_R {
        DLYRSTD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    pub fn biassen(&self) -> BIASSEN_R {
        BIASSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    pub fn latsen(&self) -> LATSEN_R {
        LATSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    pub fn bod18cont(&self) -> BOD18CONT_R {
        BOD18CONT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    pub fn pobs(&self) -> POBS_R {
        POBS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    pub fn fffw(&self) -> FFFW_R {
        FFFW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    pub fn fbrdyen(&self) -> FBRDYEN_R {
        FBRDYEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    pub fn fvrefsen(&self) -> FVREFSEN_R {
        FVREFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn runpspb(&mut self) -> RUNPSPB_W<0> {
        RUNPSPB_W::new(self)
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn psmpspb(&mut self) -> PSMPSPB_W<1> {
        PSMPSPB_W::new(self)
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    #[must_use]
    pub fn seqstn(&mut self) -> SEQSTN_W<2> {
        SEQSTN_W::new(self)
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    #[must_use]
    pub fn psbtd(&mut self) -> PSBTD_W<3> {
        PSBTD_W::new(self)
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    #[must_use]
    pub fn pshfd(&mut self) -> PSHFD_W<4> {
        PSHFD_W::new(self)
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dlyrstd(&mut self) -> DLYRSTD_W<5> {
        DLYRSTD_W::new(self)
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn biassen(&mut self) -> BIASSEN_W<6> {
        BIASSEN_W::new(self)
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn latsen(&mut self) -> LATSEN_W<7> {
        LATSEN_W::new(self)
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    #[must_use]
    pub fn bod18cont(&mut self) -> BOD18CONT_W<8> {
        BOD18CONT_W::new(self)
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    #[must_use]
    pub fn pobs(&mut self) -> POBS_W<9> {
        POBS_W::new(self)
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fffw(&mut self) -> FFFW_W<10> {
        FFFW_W::new(self)
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbrdyen(&mut self) -> FBRDYEN_W<11> {
        FBRDYEN_W::new(self)
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fvrefsen(&mut self) -> FVREFSEN_W<12> {
        FVREFSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bypass Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bpr](index.html) module"]
pub struct BPR_SPEC;
impl crate::RegisterSpec for BPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bpr::R](R) reader structure"]
impl crate::Readable for BPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bpr::W](W) writer structure"]
impl crate::Writable for BPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BPR to value 0"]
impl crate::Resettable for BPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
