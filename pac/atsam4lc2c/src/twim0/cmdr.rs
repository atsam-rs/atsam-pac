#[doc = "Register `CMDR` reader"]
pub struct R(crate::R<CMDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDR` writer"]
pub struct W(crate::W<CMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDR_SPEC>;
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
impl From<crate::W<CMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - Transfer Direction"]
pub type READ_R = crate::BitReader<bool>;
#[doc = "Field `READ` writer - Transfer Direction"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `SADR` reader - Slave Address"]
pub type SADR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SADR` writer - Slave Address"]
pub type SADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u16, u16, 10, O>;
#[doc = "Field `TENBIT` reader - Ten Bit Addressing Mode"]
pub type TENBIT_R = crate::BitReader<bool>;
#[doc = "Field `TENBIT` writer - Ten Bit Addressing Mode"]
pub type TENBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `REPSAME` reader - Transfer is to same address as previous address"]
pub type REPSAME_R = crate::BitReader<bool>;
#[doc = "Field `REPSAME` writer - Transfer is to same address as previous address"]
pub type REPSAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `START` reader - Send START condition"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Send START condition"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Send STOP condition"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Send STOP condition"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `VALID` reader - CMDR Valid"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - CMDR Valid"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `NBYTES` reader - Number of data bytes in transfer"]
pub type NBYTES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NBYTES` writer - Number of data bytes in transfer"]
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PECEN` reader - Packet Error Checking Enable"]
pub type PECEN_R = crate::BitReader<bool>;
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `ACKLAST` reader - ACK Last Master RX Byte"]
pub type ACKLAST_R = crate::BitReader<bool>;
#[doc = "Field `ACKLAST` writer - ACK Last Master RX Byte"]
pub type ACKLAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `HS` reader - HS-mode"]
pub type HS_R = crate::BitReader<bool>;
#[doc = "Field `HS` writer - HS-mode"]
pub type HS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMDR_SPEC, bool, O>;
#[doc = "Field `HSMCODE` reader - HS-mode Master Code"]
pub type HSMCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSMCODE` writer - HS-mode Master Code"]
pub type HSMCODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMDR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    pub fn repsame(&self) -> REPSAME_R {
        REPSAME_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    pub fn acklast(&self) -> ACKLAST_R {
        ACKLAST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    pub fn hsmcode(&self) -> HSMCODE_R {
        HSMCODE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<0> {
        READ_W::new(self)
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn sadr(&mut self) -> SADR_W<1> {
        SADR_W::new(self)
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tenbit(&mut self) -> TENBIT_W<11> {
        TENBIT_W::new(self)
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    #[must_use]
    pub fn repsame(&mut self) -> REPSAME_W<12> {
        REPSAME_W::new(self)
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<15> {
        VALID_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<24> {
        PECEN_W::new(self)
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    #[must_use]
    pub fn acklast(&mut self) -> ACKLAST_W<25> {
        ACKLAST_W::new(self)
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<26> {
        HS_W::new(self)
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    #[must_use]
    pub fn hsmcode(&mut self) -> HSMCODE_W<28> {
        HSMCODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdr](index.html) module"]
pub struct CMDR_SPEC;
impl crate::RegisterSpec for CMDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdr::R](R) reader structure"]
impl crate::Readable for CMDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdr::W](W) writer structure"]
impl crate::Writable for CMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
