#[doc = "Register `CDOR` reader"]
pub struct R(crate::R<CDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDOR` writer"]
pub struct W(crate::W<CDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDOR_SPEC>;
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
impl From<crate::W<CDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFF0` reader - Offset for Channel 0, used in Automatic Calibration Procedure"]
pub struct OFF0_R(crate::FieldReader<bool, bool>);
impl OFF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF0` writer - Offset for Channel 0, used in Automatic Calibration Procedure"]
pub struct OFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF0_W<'a> {
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
#[doc = "Field `OFF1` reader - Offset for Channel 1, used in Automatic Calibration Procedure"]
pub struct OFF1_R(crate::FieldReader<bool, bool>);
impl OFF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF1` writer - Offset for Channel 1, used in Automatic Calibration Procedure"]
pub struct OFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF1_W<'a> {
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
#[doc = "Field `OFF2` reader - Offset for Channel 2, used in Automatic Calibration Procedure"]
pub struct OFF2_R(crate::FieldReader<bool, bool>);
impl OFF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF2` writer - Offset for Channel 2, used in Automatic Calibration Procedure"]
pub struct OFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF2_W<'a> {
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
#[doc = "Field `OFF3` reader - Offset for Channel 3, used in Automatic Calibration Procedure"]
pub struct OFF3_R(crate::FieldReader<bool, bool>);
impl OFF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF3` writer - Offset for Channel 3, used in Automatic Calibration Procedure"]
pub struct OFF3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF3_W<'a> {
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
#[doc = "Field `OFF4` reader - Offset for Channel 4, used in Automatic Calibration Procedure"]
pub struct OFF4_R(crate::FieldReader<bool, bool>);
impl OFF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF4` writer - Offset for Channel 4, used in Automatic Calibration Procedure"]
pub struct OFF4_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF4_W<'a> {
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
#[doc = "Field `OFF5` reader - Offset for Channel 5, used in Automatic Calibration Procedure"]
pub struct OFF5_R(crate::FieldReader<bool, bool>);
impl OFF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF5` writer - Offset for Channel 5, used in Automatic Calibration Procedure"]
pub struct OFF5_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF5_W<'a> {
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
#[doc = "Field `OFF6` reader - Offset for Channel 6, used in Automatic Calibration Procedure"]
pub struct OFF6_R(crate::FieldReader<bool, bool>);
impl OFF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF6` writer - Offset for Channel 6, used in Automatic Calibration Procedure"]
pub struct OFF6_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF6_W<'a> {
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
#[doc = "Field `OFF7` reader - Offset for Channel 7, used in Automatic Calibration Procedure"]
pub struct OFF7_R(crate::FieldReader<bool, bool>);
impl OFF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF7` writer - Offset for Channel 7, used in Automatic Calibration Procedure"]
pub struct OFF7_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF7_W<'a> {
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
#[doc = "Field `OFF8` reader - Offset for Channel 8, used in Automatic Calibration Procedure"]
pub struct OFF8_R(crate::FieldReader<bool, bool>);
impl OFF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF8` writer - Offset for Channel 8, used in Automatic Calibration Procedure"]
pub struct OFF8_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF8_W<'a> {
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
#[doc = "Field `OFF9` reader - Offset for Channel 9, used in Automatic Calibration Procedure"]
pub struct OFF9_R(crate::FieldReader<bool, bool>);
impl OFF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF9` writer - Offset for Channel 9, used in Automatic Calibration Procedure"]
pub struct OFF9_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF9_W<'a> {
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
#[doc = "Field `OFF10` reader - Offset for Channel 10, used in Automatic Calibration Procedure"]
pub struct OFF10_R(crate::FieldReader<bool, bool>);
impl OFF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF10` writer - Offset for Channel 10, used in Automatic Calibration Procedure"]
pub struct OFF10_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF10_W<'a> {
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
#[doc = "Field `OFF11` reader - Offset for Channel 11, used in Automatic Calibration Procedure"]
pub struct OFF11_R(crate::FieldReader<bool, bool>);
impl OFF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF11` writer - Offset for Channel 11, used in Automatic Calibration Procedure"]
pub struct OFF11_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF11_W<'a> {
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
#[doc = "Field `OFF12` reader - Offset for Channel 12, used in Automatic Calibration Procedure"]
pub struct OFF12_R(crate::FieldReader<bool, bool>);
impl OFF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF12` writer - Offset for Channel 12, used in Automatic Calibration Procedure"]
pub struct OFF12_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF12_W<'a> {
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
#[doc = "Field `OFF13` reader - Offset for Channel 13, used in Automatic Calibration Procedure"]
pub struct OFF13_R(crate::FieldReader<bool, bool>);
impl OFF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF13` writer - Offset for Channel 13, used in Automatic Calibration Procedure"]
pub struct OFF13_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF13_W<'a> {
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
#[doc = "Field `OFF14` reader - Offset for Channel 14, used in Automatic Calibration Procedure"]
pub struct OFF14_R(crate::FieldReader<bool, bool>);
impl OFF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF14` writer - Offset for Channel 14, used in Automatic Calibration Procedure"]
pub struct OFF14_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF14_W<'a> {
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
#[doc = "Field `OFF15` reader - Offset for Channel 15, used in Automatic Calibration Procedure"]
pub struct OFF15_R(crate::FieldReader<bool, bool>);
impl OFF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFF15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFF15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFF15` writer - Offset for Channel 15, used in Automatic Calibration Procedure"]
pub struct OFF15_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Offset for Channel 0, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off0(&self) -> OFF0_R {
        OFF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Offset for Channel 1, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off1(&self) -> OFF1_R {
        OFF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Offset for Channel 2, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off2(&self) -> OFF2_R {
        OFF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Offset for Channel 3, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off3(&self) -> OFF3_R {
        OFF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Offset for Channel 4, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off4(&self) -> OFF4_R {
        OFF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Offset for Channel 5, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off5(&self) -> OFF5_R {
        OFF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Offset for Channel 6, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off6(&self) -> OFF6_R {
        OFF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Offset for Channel 7, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off7(&self) -> OFF7_R {
        OFF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Offset for Channel 8, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off8(&self) -> OFF8_R {
        OFF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Offset for Channel 9, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off9(&self) -> OFF9_R {
        OFF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Offset for Channel 10, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off10(&self) -> OFF10_R {
        OFF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Offset for Channel 11, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off11(&self) -> OFF11_R {
        OFF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Offset for Channel 12, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off12(&self) -> OFF12_R {
        OFF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Offset for Channel 13, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off13(&self) -> OFF13_R {
        OFF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Offset for Channel 14, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off14(&self) -> OFF14_R {
        OFF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Offset for Channel 15, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off15(&self) -> OFF15_R {
        OFF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Offset for Channel 0, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off0(&mut self) -> OFF0_W {
        OFF0_W { w: self }
    }
    #[doc = "Bit 1 - Offset for Channel 1, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off1(&mut self) -> OFF1_W {
        OFF1_W { w: self }
    }
    #[doc = "Bit 2 - Offset for Channel 2, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off2(&mut self) -> OFF2_W {
        OFF2_W { w: self }
    }
    #[doc = "Bit 3 - Offset for Channel 3, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off3(&mut self) -> OFF3_W {
        OFF3_W { w: self }
    }
    #[doc = "Bit 4 - Offset for Channel 4, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off4(&mut self) -> OFF4_W {
        OFF4_W { w: self }
    }
    #[doc = "Bit 5 - Offset for Channel 5, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off5(&mut self) -> OFF5_W {
        OFF5_W { w: self }
    }
    #[doc = "Bit 6 - Offset for Channel 6, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off6(&mut self) -> OFF6_W {
        OFF6_W { w: self }
    }
    #[doc = "Bit 7 - Offset for Channel 7, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off7(&mut self) -> OFF7_W {
        OFF7_W { w: self }
    }
    #[doc = "Bit 8 - Offset for Channel 8, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off8(&mut self) -> OFF8_W {
        OFF8_W { w: self }
    }
    #[doc = "Bit 9 - Offset for Channel 9, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off9(&mut self) -> OFF9_W {
        OFF9_W { w: self }
    }
    #[doc = "Bit 10 - Offset for Channel 10, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off10(&mut self) -> OFF10_W {
        OFF10_W { w: self }
    }
    #[doc = "Bit 11 - Offset for Channel 11, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off11(&mut self) -> OFF11_W {
        OFF11_W { w: self }
    }
    #[doc = "Bit 12 - Offset for Channel 12, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off12(&mut self) -> OFF12_W {
        OFF12_W { w: self }
    }
    #[doc = "Bit 13 - Offset for Channel 13, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off13(&mut self) -> OFF13_W {
        OFF13_W { w: self }
    }
    #[doc = "Bit 14 - Offset for Channel 14, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off14(&mut self) -> OFF14_W {
        OFF14_W { w: self }
    }
    #[doc = "Bit 15 - Offset for Channel 15, used in Automatic Calibration Procedure"]
    #[inline(always)]
    pub fn off15(&mut self) -> OFF15_W {
        OFF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Calibration DC Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdor](index.html) module"]
pub struct CDOR_SPEC;
impl crate::RegisterSpec for CDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdor::R](R) reader structure"]
impl crate::Readable for CDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdor::W](W) writer structure"]
impl crate::Writable for CDOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDOR to value 0"]
impl crate::Resettable for CDOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
