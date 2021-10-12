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
pub struct IISC__R(crate::FieldReader<bool, bool>);
impl IISC__R {
    pub(crate) fn new(bits: bool) -> Self {
        IISC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IISC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IISC_` writer - IISC APB Clock Enable"]
pub struct IISC__W<'a> {
    w: &'a mut W,
}
impl<'a> IISC__W<'a> {
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
#[doc = "Field `SPI_` reader - SPI APB Clock Enable"]
pub struct SPI__R(crate::FieldReader<bool, bool>);
impl SPI__R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_` writer - SPI APB Clock Enable"]
pub struct SPI__W<'a> {
    w: &'a mut W,
}
impl<'a> SPI__W<'a> {
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
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub struct TC0__R(crate::FieldReader<bool, bool>);
impl TC0__R {
    pub(crate) fn new(bits: bool) -> Self {
        TC0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub struct TC0__W<'a> {
    w: &'a mut W,
}
impl<'a> TC0__W<'a> {
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
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub struct TC1__R(crate::FieldReader<bool, bool>);
impl TC1__R {
    pub(crate) fn new(bits: bool) -> Self {
        TC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub struct TC1__W<'a> {
    w: &'a mut W,
}
impl<'a> TC1__W<'a> {
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
#[doc = "Field `TWIM0_` reader - TWIM0 APB Clock Enable"]
pub struct TWIM0__R(crate::FieldReader<bool, bool>);
impl TWIM0__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIM0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIM0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIM0_` writer - TWIM0 APB Clock Enable"]
pub struct TWIM0__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM0__W<'a> {
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
#[doc = "Field `TWIS0_` reader - TWIS0 APB Clock Enable"]
pub struct TWIS0__R(crate::FieldReader<bool, bool>);
impl TWIS0__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIS0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIS0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIS0_` writer - TWIS0 APB Clock Enable"]
pub struct TWIS0__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS0__W<'a> {
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
#[doc = "Field `TWIM1_` reader - TWIM1 APB Clock Enable"]
pub struct TWIM1__R(crate::FieldReader<bool, bool>);
impl TWIM1__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIM1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIM1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIM1_` writer - TWIM1 APB Clock Enable"]
pub struct TWIM1__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM1__W<'a> {
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
#[doc = "Field `TWIS1_` reader - TWIS1 APB Clock Enable"]
pub struct TWIS1__R(crate::FieldReader<bool, bool>);
impl TWIS1__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIS1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIS1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIS1_` writer - TWIS1 APB Clock Enable"]
pub struct TWIS1__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIS1__W<'a> {
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
#[doc = "Field `USART0_` reader - USART0 APB Clock Enable"]
pub struct USART0__R(crate::FieldReader<bool, bool>);
impl USART0__R {
    pub(crate) fn new(bits: bool) -> Self {
        USART0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART0_` writer - USART0 APB Clock Enable"]
pub struct USART0__W<'a> {
    w: &'a mut W,
}
impl<'a> USART0__W<'a> {
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
#[doc = "Field `USART1_` reader - USART1 APB Clock Enable"]
pub struct USART1__R(crate::FieldReader<bool, bool>);
impl USART1__R {
    pub(crate) fn new(bits: bool) -> Self {
        USART1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1_` writer - USART1 APB Clock Enable"]
pub struct USART1__W<'a> {
    w: &'a mut W,
}
impl<'a> USART1__W<'a> {
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
#[doc = "Field `USART2_` reader - USART2 APB Clock Enable"]
pub struct USART2__R(crate::FieldReader<bool, bool>);
impl USART2__R {
    pub(crate) fn new(bits: bool) -> Self {
        USART2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2_` writer - USART2 APB Clock Enable"]
pub struct USART2__W<'a> {
    w: &'a mut W,
}
impl<'a> USART2__W<'a> {
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
#[doc = "Field `USART3_` reader - USART3 APB Clock Enable"]
pub struct USART3__R(crate::FieldReader<bool, bool>);
impl USART3__R {
    pub(crate) fn new(bits: bool) -> Self {
        USART3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3_` writer - USART3 APB Clock Enable"]
pub struct USART3__W<'a> {
    w: &'a mut W,
}
impl<'a> USART3__W<'a> {
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
#[doc = "Field `ADCIFE_` reader - ADCIFE APB Clock Enable"]
pub struct ADCIFE__R(crate::FieldReader<bool, bool>);
impl ADCIFE__R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIFE__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCIFE__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIFE_` writer - ADCIFE APB Clock Enable"]
pub struct ADCIFE__W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIFE__W<'a> {
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
#[doc = "Field `DACC_` reader - DACC APB Clock Enable"]
pub struct DACC__R(crate::FieldReader<bool, bool>);
impl DACC__R {
    pub(crate) fn new(bits: bool) -> Self {
        DACC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DACC_` writer - DACC APB Clock Enable"]
pub struct DACC__W<'a> {
    w: &'a mut W,
}
impl<'a> DACC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ACIFC_` reader - ACIFC APB Clock Enable"]
pub struct ACIFC__R(crate::FieldReader<bool, bool>);
impl ACIFC__R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIFC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIFC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIFC_` writer - ACIFC APB Clock Enable"]
pub struct ACIFC__W<'a> {
    w: &'a mut W,
}
impl<'a> ACIFC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `GLOC_` reader - GLOC APB Clock Enable"]
pub struct GLOC__R(crate::FieldReader<bool, bool>);
impl GLOC__R {
    pub(crate) fn new(bits: bool) -> Self {
        GLOC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLOC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLOC_` writer - GLOC APB Clock Enable"]
pub struct GLOC__W<'a> {
    w: &'a mut W,
}
impl<'a> GLOC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ABDACB_` reader - ABDACB APB Clock Enable"]
pub struct ABDACB__R(crate::FieldReader<bool, bool>);
impl ABDACB__R {
    pub(crate) fn new(bits: bool) -> Self {
        ABDACB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABDACB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABDACB_` writer - ABDACB APB Clock Enable"]
pub struct ABDACB__W<'a> {
    w: &'a mut W,
}
impl<'a> ABDACB__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub struct TRNG__R(crate::FieldReader<bool, bool>);
impl TRNG__R {
    pub(crate) fn new(bits: bool) -> Self {
        TRNG__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub struct TRNG__W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PARC_` reader - PARC APB Clock Enable"]
pub struct PARC__R(crate::FieldReader<bool, bool>);
impl PARC__R {
    pub(crate) fn new(bits: bool) -> Self {
        PARC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARC_` writer - PARC APB Clock Enable"]
pub struct PARC__W<'a> {
    w: &'a mut W,
}
impl<'a> PARC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CATB_` reader - CATB APB Clock Enable"]
pub struct CATB__R(crate::FieldReader<bool, bool>);
impl CATB__R {
    pub(crate) fn new(bits: bool) -> Self {
        CATB__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CATB__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CATB_` writer - CATB APB Clock Enable"]
pub struct CATB__W<'a> {
    w: &'a mut W,
}
impl<'a> CATB__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `TWIM2_` reader - TWIM2 APB Clock Enable"]
pub struct TWIM2__R(crate::FieldReader<bool, bool>);
impl TWIM2__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIM2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIM2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIM2_` writer - TWIM2 APB Clock Enable"]
pub struct TWIM2__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TWIM3_` reader - TWIM3 APB Clock Enable"]
pub struct TWIM3__R(crate::FieldReader<bool, bool>);
impl TWIM3__R {
    pub(crate) fn new(bits: bool) -> Self {
        TWIM3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWIM3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWIM3_` writer - TWIM3 APB Clock Enable"]
pub struct TWIM3__W<'a> {
    w: &'a mut W,
}
impl<'a> TWIM3__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    pub fn iisc_(&self) -> IISC__R {
        IISC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    pub fn spi_(&self) -> SPI__R {
        SPI__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> TC0__R {
        TC0__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> TC1__R {
        TC1__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    pub fn twim0_(&self) -> TWIM0__R {
        TWIM0__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    pub fn twis0_(&self) -> TWIS0__R {
        TWIS0__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    pub fn twim1_(&self) -> TWIM1__R {
        TWIM1__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    pub fn twis1_(&self) -> TWIS1__R {
        TWIS1__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    pub fn usart0_(&self) -> USART0__R {
        USART0__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    pub fn usart1_(&self) -> USART1__R {
        USART1__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    pub fn usart2_(&self) -> USART2__R {
        USART2__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    pub fn usart3_(&self) -> USART3__R {
        USART3__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    pub fn adcife_(&self) -> ADCIFE__R {
        ADCIFE__R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    pub fn dacc_(&self) -> DACC__R {
        DACC__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    pub fn acifc_(&self) -> ACIFC__R {
        ACIFC__R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    pub fn gloc_(&self) -> GLOC__R {
        GLOC__R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    pub fn abdacb_(&self) -> ABDACB__R {
        ABDACB__R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    pub fn parc_(&self) -> PARC__R {
        PARC__R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    pub fn catb_(&self) -> CATB__R {
        CATB__R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    pub fn twim2_(&self) -> TWIM2__R {
        TWIM2__R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    pub fn twim3_(&self) -> TWIM3__R {
        TWIM3__R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IISC APB Clock Enable"]
    #[inline(always)]
    pub fn iisc_(&mut self) -> IISC__W {
        IISC__W { w: self }
    }
    #[doc = "Bit 1 - SPI APB Clock Enable"]
    #[inline(always)]
    pub fn spi_(&mut self) -> SPI__W {
        SPI__W { w: self }
    }
    #[doc = "Bit 2 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&mut self) -> TC0__W {
        TC0__W { w: self }
    }
    #[doc = "Bit 3 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&mut self) -> TC1__W {
        TC1__W { w: self }
    }
    #[doc = "Bit 4 - TWIM0 APB Clock Enable"]
    #[inline(always)]
    pub fn twim0_(&mut self) -> TWIM0__W {
        TWIM0__W { w: self }
    }
    #[doc = "Bit 5 - TWIS0 APB Clock Enable"]
    #[inline(always)]
    pub fn twis0_(&mut self) -> TWIS0__W {
        TWIS0__W { w: self }
    }
    #[doc = "Bit 6 - TWIM1 APB Clock Enable"]
    #[inline(always)]
    pub fn twim1_(&mut self) -> TWIM1__W {
        TWIM1__W { w: self }
    }
    #[doc = "Bit 7 - TWIS1 APB Clock Enable"]
    #[inline(always)]
    pub fn twis1_(&mut self) -> TWIS1__W {
        TWIS1__W { w: self }
    }
    #[doc = "Bit 8 - USART0 APB Clock Enable"]
    #[inline(always)]
    pub fn usart0_(&mut self) -> USART0__W {
        USART0__W { w: self }
    }
    #[doc = "Bit 9 - USART1 APB Clock Enable"]
    #[inline(always)]
    pub fn usart1_(&mut self) -> USART1__W {
        USART1__W { w: self }
    }
    #[doc = "Bit 10 - USART2 APB Clock Enable"]
    #[inline(always)]
    pub fn usart2_(&mut self) -> USART2__W {
        USART2__W { w: self }
    }
    #[doc = "Bit 11 - USART3 APB Clock Enable"]
    #[inline(always)]
    pub fn usart3_(&mut self) -> USART3__W {
        USART3__W { w: self }
    }
    #[doc = "Bit 12 - ADCIFE APB Clock Enable"]
    #[inline(always)]
    pub fn adcife_(&mut self) -> ADCIFE__W {
        ADCIFE__W { w: self }
    }
    #[doc = "Bit 13 - DACC APB Clock Enable"]
    #[inline(always)]
    pub fn dacc_(&mut self) -> DACC__W {
        DACC__W { w: self }
    }
    #[doc = "Bit 14 - ACIFC APB Clock Enable"]
    #[inline(always)]
    pub fn acifc_(&mut self) -> ACIFC__W {
        ACIFC__W { w: self }
    }
    #[doc = "Bit 15 - GLOC APB Clock Enable"]
    #[inline(always)]
    pub fn gloc_(&mut self) -> GLOC__W {
        GLOC__W { w: self }
    }
    #[doc = "Bit 16 - ABDACB APB Clock Enable"]
    #[inline(always)]
    pub fn abdacb_(&mut self) -> ABDACB__W {
        ABDACB__W { w: self }
    }
    #[doc = "Bit 17 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&mut self) -> TRNG__W {
        TRNG__W { w: self }
    }
    #[doc = "Bit 18 - PARC APB Clock Enable"]
    #[inline(always)]
    pub fn parc_(&mut self) -> PARC__W {
        PARC__W { w: self }
    }
    #[doc = "Bit 19 - CATB APB Clock Enable"]
    #[inline(always)]
    pub fn catb_(&mut self) -> CATB__W {
        CATB__W { w: self }
    }
    #[doc = "Bit 21 - TWIM2 APB Clock Enable"]
    #[inline(always)]
    pub fn twim2_(&mut self) -> TWIM2__W {
        TWIM2__W { w: self }
    }
    #[doc = "Bit 22 - TWIM3 APB Clock Enable"]
    #[inline(always)]
    pub fn twim3_(&mut self) -> TWIM3__W {
        TWIM3__W { w: self }
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
}
#[doc = "`reset()` method sets PBAMASK to value 0"]
impl crate::Resettable for PBAMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
