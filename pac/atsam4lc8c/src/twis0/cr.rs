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
#[doc = "Field `SEN` reader - Slave Enable"]
pub struct SEN_R(crate::FieldReader<bool, bool>);
impl SEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEN` writer - Slave Enable"]
pub struct SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN_W<'a> {
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
#[doc = "Field `SMEN` reader - SMBus Mode Enable"]
pub struct SMEN_R(crate::FieldReader<bool, bool>);
impl SMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMEN` writer - SMBus Mode Enable"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
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
#[doc = "Field `SMATCH` reader - Slave Address Match"]
pub struct SMATCH_R(crate::FieldReader<bool, bool>);
impl SMATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMATCH` writer - Slave Address Match"]
pub struct SMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMATCH_W<'a> {
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
#[doc = "Field `GCMATCH` reader - General Call Address Match"]
pub struct GCMATCH_R(crate::FieldReader<bool, bool>);
impl GCMATCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCMATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCMATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCMATCH` writer - General Call Address Match"]
pub struct GCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> GCMATCH_W<'a> {
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
#[doc = "Field `STREN` reader - Clock Stretch Enable"]
pub struct STREN_R(crate::FieldReader<bool, bool>);
impl STREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        STREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STREN` writer - Clock Stretch Enable"]
pub struct STREN_W<'a> {
    w: &'a mut W,
}
impl<'a> STREN_W<'a> {
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
#[doc = "Field `SWRST` reader - Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
#[doc = "Field `SMBALERT` reader - SMBus Alert"]
pub struct SMBALERT_R(crate::FieldReader<bool, bool>);
impl SMBALERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMBALERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMBALERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMBALERT` writer - SMBus Alert"]
pub struct SMBALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBALERT_W<'a> {
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
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub struct SMDA_R(crate::FieldReader<bool, bool>);
impl SMDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub struct SMDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDA_W<'a> {
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
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub struct SMHH_R(crate::FieldReader<bool, bool>);
impl SMHH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMHH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMHH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub struct SMHH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMHH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ACK` reader - Slave Receiver Data Phase ACK Value"]
pub struct ACK_R(crate::FieldReader<bool, bool>);
impl ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK` writer - Slave Receiver Data Phase ACK Value"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
#[doc = "Field `CUP` reader - NBYTES Count Up"]
pub struct CUP_R(crate::FieldReader<bool, bool>);
impl CUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CUP` writer - NBYTES Count Up"]
pub struct CUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CUP_W<'a> {
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
#[doc = "Field `SOAM` reader - Stretch Clock on Address Match"]
pub struct SOAM_R(crate::FieldReader<bool, bool>);
impl SOAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOAM` writer - Stretch Clock on Address Match"]
pub struct SOAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOAM_W<'a> {
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
#[doc = "Field `SODR` reader - Stretch Clock on Data Byte Reception"]
pub struct SODR_R(crate::FieldReader<bool, bool>);
impl SODR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SODR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SODR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SODR` writer - Stretch Clock on Data Byte Reception"]
pub struct SODR_W<'a> {
    w: &'a mut W,
}
impl<'a> SODR_W<'a> {
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
#[doc = "Field `ADR` reader - Slave Address"]
pub struct ADR_R(crate::FieldReader<u16, u16>);
impl ADR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADR` writer - Slave Address"]
pub struct ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `TENBIT` reader - Ten Bit Address Match"]
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
#[doc = "Field `TENBIT` writer - Ten Bit Address Match"]
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `BRIDGE` reader - Bridge Control Enable"]
pub struct BRIDGE_R(crate::FieldReader<bool, bool>);
impl BRIDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRIDGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRIDGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRIDGE` writer - Bridge Control Enable"]
pub struct BRIDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRIDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    pub fn smatch(&self) -> SMATCH_R {
        SMATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    pub fn gcmatch(&self) -> GCMATCH_R {
        GCMATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    pub fn cup(&self) -> CUP_R {
        CUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    pub fn soam(&self) -> SOAM_R {
        SOAM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    pub fn sodr(&self) -> SODR_R {
        SODR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    pub fn bridge(&self) -> BRIDGE_R {
        BRIDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W { w: self }
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    pub fn smatch(&mut self) -> SMATCH_W {
        SMATCH_W { w: self }
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    pub fn gcmatch(&mut self) -> GCMATCH_W {
        GCMATCH_W { w: self }
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn stren(&mut self) -> STREN_W {
        STREN_W { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W { w: self }
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> SMDA_W {
        SMDA_W { w: self }
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> SMHH_W {
        SMHH_W { w: self }
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    pub fn cup(&mut self) -> CUP_W {
        CUP_W { w: self }
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    pub fn soam(&mut self) -> SOAM_W {
        SOAM_W { w: self }
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    pub fn sodr(&mut self) -> SODR_W {
        SODR_W { w: self }
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn adr(&mut self) -> ADR_W {
        ADR_W { w: self }
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    pub fn tenbit(&mut self) -> TENBIT_W {
        TENBIT_W { w: self }
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    pub fn bridge(&mut self) -> BRIDGE_W {
        BRIDGE_W { w: self }
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
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
