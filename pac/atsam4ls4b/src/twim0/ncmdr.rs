#[doc = "Register `NCMDR` reader"]
pub struct R(crate::R<NCMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCMDR` writer"]
pub struct W(crate::W<NCMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCMDR_SPEC>;
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
impl From<crate::W<NCMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - Transfer Direction"]
pub struct READ_R(crate::FieldReader<bool, bool>);
impl READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ` writer - Transfer Direction"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
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
#[doc = "Field `SADR` reader - Slave Address"]
pub struct SADR_R(crate::FieldReader<u16, u16>);
impl SADR_R {
    pub(crate) fn new(bits: u16) -> Self {
        SADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR` writer - Slave Address"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | ((value as u32 & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Field `TENBIT` reader - Ten Bit Addressing Mode"]
pub struct TENBIT_R(crate::FieldReader<bool, bool>);
impl TENBIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TENBIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TENBIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TENBIT` writer - Ten Bit Addressing Mode"]
pub struct TENBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TENBIT_W<'a> {
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
#[doc = "Field `REPSAME` reader - Transfer is to same address as previous address"]
pub struct REPSAME_R(crate::FieldReader<bool, bool>);
impl REPSAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        REPSAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REPSAME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REPSAME` writer - Transfer is to same address as previous address"]
pub struct REPSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> REPSAME_W<'a> {
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
#[doc = "Field `START` reader - Send START condition"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Send START condition"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Field `STOP` reader - Send STOP condition"]
pub struct STOP_R(crate::FieldReader<bool, bool>);
impl STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOP` writer - Send STOP condition"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Field `VALID` reader - CMDR Valid"]
pub struct VALID_R(crate::FieldReader<bool, bool>);
impl VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALID` writer - CMDR Valid"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
#[doc = "Field `NBYTES` reader - Number of data bytes in transfer"]
pub struct NBYTES_R(crate::FieldReader<u8, u8>);
impl NBYTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBYTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBYTES` writer - Number of data bytes in transfer"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PECEN` reader - Packet Error Checking Enable"]
pub struct PECEN_R(crate::FieldReader<bool, bool>);
impl PECEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PECEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PECEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `ACKLAST` reader - ACK Last Master RX Byte"]
pub struct ACKLAST_R(crate::FieldReader<bool, bool>);
impl ACKLAST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKLAST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKLAST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKLAST` writer - ACK Last Master RX Byte"]
pub struct ACKLAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKLAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `HS` reader - HS-mode"]
pub struct HS_R(crate::FieldReader<bool, bool>);
impl HS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS` writer - HS-mode"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `HSMCODE` reader - HS-mode Master Code"]
pub struct HSMCODE_R(crate::FieldReader<u8, u8>);
impl HSMCODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSMCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSMCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMCODE` writer - HS-mode Master Code"]
pub struct HSMCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    pub fn repsame(&self) -> REPSAME_R {
        REPSAME_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    pub fn acklast(&self) -> ACKLAST_R {
        ACKLAST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    pub fn hsmcode(&self) -> HSMCODE_R {
        HSMCODE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    pub fn tenbit(&mut self) -> TENBIT_W {
        TENBIT_W { w: self }
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    pub fn repsame(&mut self) -> REPSAME_W {
        REPSAME_W { w: self }
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    pub fn acklast(&mut self) -> ACKLAST_W {
        ACKLAST_W { w: self }
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    pub fn hsmcode(&mut self) -> HSMCODE_W {
        HSMCODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncmdr](index.html) module"]
pub struct NCMDR_SPEC;
impl crate::RegisterSpec for NCMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncmdr::R](R) reader structure"]
impl crate::Readable for NCMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncmdr::W](W) writer structure"]
impl crate::Writable for NCMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NCMDR to value 0"]
impl crate::Resettable for NCMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
