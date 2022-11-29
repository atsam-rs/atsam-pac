#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCRYPT` reader - Encryption"]
pub type ENCRYPT_R = crate::BitReader<bool>;
#[doc = "Field `ENCRYPT` writer - Encryption"]
pub type ENCRYPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `KEYSIZE` reader - Key Size"]
pub type KEYSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEYSIZE` writer - Key Size"]
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMA` reader - DMA Mode"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA Mode"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `OPMODE` reader - Confidentiality Mode of Operation"]
pub type OPMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPMODE` writer - Confidentiality Mode of Operation"]
pub type OPMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `CFBS` reader - Cipher Feedback Data Segment Size"]
pub type CFBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFBS` writer - Cipher Feedback Data Segment Size"]
pub type CFBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `CTYPE` reader - Countermeasure Type"]
pub type CTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTYPE` writer - Countermeasure Type"]
pub type CTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Encryption"]
    #[inline(always)]
    pub fn encrypt(&self) -> ENCRYPT_R {
        ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Confidentiality Mode of Operation"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Cipher Feedback Data Segment Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Countermeasure Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption"]
    #[inline(always)]
    #[must_use]
    pub fn encrypt(&mut self) -> ENCRYPT_W<0> {
        ENCRYPT_W::new(self)
    }
    #[doc = "Bits 1:2 - Key Size"]
    #[inline(always)]
    #[must_use]
    pub fn keysize(&mut self) -> KEYSIZE_W<1> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<3> {
        DMA_W::new(self)
    }
    #[doc = "Bits 4:6 - Confidentiality Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn opmode(&mut self) -> OPMODE_W<4> {
        OPMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Cipher Feedback Data Segment Size"]
    #[inline(always)]
    #[must_use]
    pub fn cfbs(&mut self) -> CFBS_W<8> {
        CFBS_W::new(self)
    }
    #[doc = "Bits 16:19 - Countermeasure Type"]
    #[inline(always)]
    #[must_use]
    pub fn ctype(&mut self) -> CTYPE_W<16> {
        CTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0x000f_0000"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0000;
}
