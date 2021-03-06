#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NMI` reader - External Non Maskable CPU interrupt"]
pub struct NMI_R(crate::FieldReader<bool, bool>);
impl NMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1` reader - External Interrupt 1"]
pub struct INT1_R(crate::FieldReader<bool, bool>);
impl INT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2` reader - External Interrupt 2"]
pub struct INT2_R(crate::FieldReader<bool, bool>);
impl INT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3` reader - External Interrupt 3"]
pub struct INT3_R(crate::FieldReader<bool, bool>);
impl INT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4` reader - External Interrupt 4"]
pub struct INT4_R(crate::FieldReader<bool, bool>);
impl INT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT5` reader - External Interrupt 5"]
pub struct INT5_R(crate::FieldReader<bool, bool>);
impl INT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT6` reader - External Interrupt 6"]
pub struct INT6_R(crate::FieldReader<bool, bool>);
impl INT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT7` reader - External Interrupt 7"]
pub struct INT7_R(crate::FieldReader<bool, bool>);
impl INT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT8` reader - External Interrupt 8"]
pub struct INT8_R(crate::FieldReader<bool, bool>);
impl INT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT9` reader - External Interrupt 9"]
pub struct INT9_R(crate::FieldReader<bool, bool>);
impl INT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT10` reader - External Interrupt 10"]
pub struct INT10_R(crate::FieldReader<bool, bool>);
impl INT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT11` reader - External Interrupt 11"]
pub struct INT11_R(crate::FieldReader<bool, bool>);
impl INT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT12` reader - External Interrupt 12"]
pub struct INT12_R(crate::FieldReader<bool, bool>);
impl INT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT13` reader - External Interrupt 13"]
pub struct INT13_R(crate::FieldReader<bool, bool>);
impl INT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT14` reader - External Interrupt 14"]
pub struct INT14_R(crate::FieldReader<bool, bool>);
impl INT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT15` reader - External Interrupt 15"]
pub struct INT15_R(crate::FieldReader<bool, bool>);
impl INT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
