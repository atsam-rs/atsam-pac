#[doc = "Register `DCFGR` reader"]
pub struct R(crate::R<DCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFGR` writer"]
pub struct W(crate::W<DCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFGR_SPEC>;
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
impl From<crate::W<DCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fixed Burst Length for DMA Data Operations:\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FBLDO_A {
    #[doc = "1: 00001: Always use SINGLE AHB bursts"]
    SINGLE = 1,
    #[doc = "4: 001xx: Attempt to use INCR4 AHB bursts (Default)"]
    INCR4 = 4,
    #[doc = "8: 01xxx: Attempt to use INCR8 AHB bursts"]
    INCR8 = 8,
    #[doc = "16: 1xxxx: Attempt to use INCR16 AHB bursts"]
    INCR16 = 16,
}
impl From<FBLDO_A> for u8 {
    #[inline(always)]
    fn from(variant: FBLDO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FBLDO` reader - Fixed Burst Length for DMA Data Operations:"]
pub struct FBLDO_R(crate::FieldReader<u8, FBLDO_A>);
impl FBLDO_R {
    pub(crate) fn new(bits: u8) -> Self {
        FBLDO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FBLDO_A> {
        match self.bits {
            1 => Some(FBLDO_A::SINGLE),
            4 => Some(FBLDO_A::INCR4),
            8 => Some(FBLDO_A::INCR8),
            16 => Some(FBLDO_A::INCR16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == FBLDO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        **self == FBLDO_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        **self == FBLDO_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        **self == FBLDO_A::INCR16
    }
}
impl core::ops::Deref for FBLDO_R {
    type Target = crate::FieldReader<u8, FBLDO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBLDO` writer - Fixed Burst Length for DMA Data Operations:"]
pub struct FBLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> FBLDO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBLDO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(FBLDO_A::SINGLE)
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR4)
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR8)
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ESMA` reader - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub struct ESMA_R(crate::FieldReader<bool, bool>);
impl ESMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESMA` writer - Endian Swap Mode Enable for Management Descriptor Accesses"]
pub struct ESMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESMA_W<'a> {
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
#[doc = "Field `ESPA` reader - Endian Swap Mode Enable for Packet Data Accesses"]
pub struct ESPA_R(crate::FieldReader<bool, bool>);
impl ESPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ESPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ESPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ESPA` writer - Endian Swap Mode Enable for Packet Data Accesses"]
pub struct ESPA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESPA_W<'a> {
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
#[doc = "Field `DRBS` reader - DMA Receive Buffer Size"]
pub struct DRBS_R(crate::FieldReader<u8, u8>);
impl DRBS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRBS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRBS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRBS` writer - DMA Receive Buffer Size"]
pub struct DRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FBLDO_R {
        FBLDO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> ESMA_R {
        ESMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> ESPA_R {
        ESPA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DRBS_R {
        DRBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&mut self) -> FBLDO_W {
        FBLDO_W { w: self }
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&mut self) -> ESMA_W {
        ESMA_W { w: self }
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&mut self) -> ESPA_W {
        ESPA_W { w: self }
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&mut self) -> DRBS_W {
        DRBS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfgr](index.html) module"]
pub struct DCFGR_SPEC;
impl crate::RegisterSpec for DCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfgr::R](R) reader structure"]
impl crate::Readable for DCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfgr::W](W) writer structure"]
impl crate::Writable for DCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFGR to value 0x0002_0004"]
impl crate::Resettable for DCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0004
    }
}
