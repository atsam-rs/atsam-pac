#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NCPHA_R = crate::BitReader<bool>;
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NCPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CSAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CSAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCBR` reader - Serial Clock Baud Rate"]
pub type SCBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBR` writer - Serial Clock Baud Rate"]
pub type SCBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DLYBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DLYBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn ncpha(&mut self) -> NCPHA_W<1> {
        NCPHA_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn csnaat(&mut self) -> CSNAAT_W<2> {
        CSNAAT_W::new(self)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn csaat(&mut self) -> CSAAT_W<3> {
        CSAAT_W::new(self)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<4> {
        BITS_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Baud Rate"]
    #[inline(always)]
    #[must_use]
    pub fn scbr(&mut self) -> SCBR_W<8> {
        SCBR_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    #[must_use]
    pub fn dlybs(&mut self) -> DLYBS_W<16> {
        DLYBS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    #[must_use]
    pub fn dlybct(&mut self) -> DLYBCT_W<24> {
        DLYBCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
