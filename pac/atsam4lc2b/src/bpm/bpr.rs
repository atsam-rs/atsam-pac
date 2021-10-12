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
pub struct RUNPSPB_R(crate::FieldReader<bool, bool>);
impl RUNPSPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUNPSPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNPSPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNPSPB` writer - Run Mode Power Scaling Preset Bypass"]
pub struct RUNPSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNPSPB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PSMPSPB` reader - Power Save Mode Power Scaling Preset Bypass"]
pub struct PSMPSPB_R(crate::FieldReader<bool, bool>);
impl PSMPSPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSMPSPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSMPSPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSMPSPB` writer - Power Save Mode Power Scaling Preset Bypass"]
pub struct PSMPSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> PSMPSPB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SEQSTN` reader - Sequencial Startup from ULP (Active Low)"]
pub struct SEQSTN_R(crate::FieldReader<bool, bool>);
impl SEQSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQSTN` writer - Sequencial Startup from ULP (Active Low)"]
pub struct SEQSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PSBTD` reader - Power Scaling Bias Timing Disable"]
pub struct PSBTD_R(crate::FieldReader<bool, bool>);
impl PSBTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSBTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSBTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSBTD` writer - Power Scaling Bias Timing Disable"]
pub struct PSBTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSBTD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PSHFD` reader - Power Scaling Halt Flash Until VREGOK Disable"]
pub struct PSHFD_R(crate::FieldReader<bool, bool>);
impl PSHFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSHFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSHFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSHFD` writer - Power Scaling Halt Flash Until VREGOK Disable"]
pub struct PSHFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSHFD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DLYRSTD` reader - Delaying Reset Disable"]
pub struct DLYRSTD_R(crate::FieldReader<bool, bool>);
impl DLYRSTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYRSTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYRSTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYRSTD` writer - Delaying Reset Disable"]
pub struct DLYRSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYRSTD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `BIASSEN` reader - Bias Switch Enable"]
pub struct BIASSEN_R(crate::FieldReader<bool, bool>);
impl BIASSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIASSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASSEN` writer - Bias Switch Enable"]
pub struct BIASSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LATSEN` reader - Latdel Switch Enable"]
pub struct LATSEN_R(crate::FieldReader<bool, bool>);
impl LATSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATSEN` writer - Latdel Switch Enable"]
pub struct LATSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LATSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `BOD18CONT` reader - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
pub struct BOD18CONT_R(crate::FieldReader<bool, bool>);
impl BOD18CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD18CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD18CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD18CONT` writer - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
pub struct BOD18CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD18CONT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `POBS` reader - Pico Uart Observability"]
pub struct POBS_R(crate::FieldReader<bool, bool>);
impl POBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        POBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POBS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POBS` writer - Pico Uart Observability"]
pub struct POBS_W<'a> {
    w: &'a mut W,
}
impl<'a> POBS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `FFFW` reader - Force Flash Fast Wakeup"]
pub struct FFFW_R(crate::FieldReader<bool, bool>);
impl FFFW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FFFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FFFW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FFFW` writer - Force Flash Fast Wakeup"]
pub struct FFFW_W<'a> {
    w: &'a mut W,
}
impl<'a> FFFW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FBRDYEN` reader - Flash Bias Ready Enable"]
pub struct FBRDYEN_R(crate::FieldReader<bool, bool>);
impl FBRDYEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBRDYEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBRDYEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBRDYEN` writer - Flash Bias Ready Enable"]
pub struct FBRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FBRDYEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FVREFSEN` reader - Flash Vref Switch Enable"]
pub struct FVREFSEN_R(crate::FieldReader<bool, bool>);
impl FVREFSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FVREFSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FVREFSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FVREFSEN` writer - Flash Vref Switch Enable"]
pub struct FVREFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FVREFSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn runpspb(&self) -> RUNPSPB_R {
        RUNPSPB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn psmpspb(&self) -> PSMPSPB_R {
        PSMPSPB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    pub fn seqstn(&self) -> SEQSTN_R {
        SEQSTN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    pub fn psbtd(&self) -> PSBTD_R {
        PSBTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    pub fn pshfd(&self) -> PSHFD_R {
        PSHFD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    pub fn dlyrstd(&self) -> DLYRSTD_R {
        DLYRSTD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    pub fn biassen(&self) -> BIASSEN_R {
        BIASSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    pub fn latsen(&self) -> LATSEN_R {
        LATSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    pub fn bod18cont(&self) -> BOD18CONT_R {
        BOD18CONT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    pub fn pobs(&self) -> POBS_R {
        POBS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    pub fn fffw(&self) -> FFFW_R {
        FFFW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    pub fn fbrdyen(&self) -> FBRDYEN_R {
        FBRDYEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    pub fn fvrefsen(&self) -> FVREFSEN_R {
        FVREFSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn runpspb(&mut self) -> RUNPSPB_W {
        RUNPSPB_W { w: self }
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn psmpspb(&mut self) -> PSMPSPB_W {
        PSMPSPB_W { w: self }
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    pub fn seqstn(&mut self) -> SEQSTN_W {
        SEQSTN_W { w: self }
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    pub fn psbtd(&mut self) -> PSBTD_W {
        PSBTD_W { w: self }
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    pub fn pshfd(&mut self) -> PSHFD_W {
        PSHFD_W { w: self }
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    pub fn dlyrstd(&mut self) -> DLYRSTD_W {
        DLYRSTD_W { w: self }
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    pub fn biassen(&mut self) -> BIASSEN_W {
        BIASSEN_W { w: self }
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    pub fn latsen(&mut self) -> LATSEN_W {
        LATSEN_W { w: self }
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    pub fn bod18cont(&mut self) -> BOD18CONT_W {
        BOD18CONT_W { w: self }
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    pub fn pobs(&mut self) -> POBS_W {
        POBS_W { w: self }
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    pub fn fffw(&mut self) -> FFFW_W {
        FFFW_W { w: self }
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    pub fn fbrdyen(&mut self) -> FBRDYEN_W {
        FBRDYEN_W { w: self }
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    pub fn fvrefsen(&mut self) -> FVREFSEN_W {
        FVREFSEN_W { w: self }
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
}
#[doc = "`reset()` method sets BPR to value 0"]
impl crate::Resettable for BPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
