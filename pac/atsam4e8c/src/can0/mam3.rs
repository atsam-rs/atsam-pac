#[doc = "Register `MAM3` reader"]
pub struct R(crate::R<MAM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAM3` writer"]
pub struct W(crate::W<MAM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAM3_SPEC>;
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
impl From<crate::W<MAM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIDvB` reader - Complementary bits for identifier in extended frame mode"]
pub type MIDV_B_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MIDvB` writer - Complementary bits for identifier in extended frame mode"]
pub type MIDV_B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAM3_SPEC, u32, u32, 18, O>;
#[doc = "Field `MIDvA` reader - Identifier for standard frame mode"]
pub type MIDV_A_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MIDvA` writer - Identifier for standard frame mode"]
pub type MIDV_A_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAM3_SPEC, u16, u16, 11, O>;
#[doc = "Field `MIDE` reader - Identifier Version"]
pub type MIDE_R = crate::BitReader<bool>;
#[doc = "Field `MIDE` writer - Identifier Version"]
pub type MIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAM3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MIDV_B_R {
        MIDV_B_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MIDV_A_R {
        MIDV_A_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_b(&mut self) -> MIDV_B_W<0> {
        MIDV_B_W::new(self)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    #[must_use]
    pub fn midv_a(&mut self) -> MIDV_A_W<18> {
        MIDV_A_W::new(self)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    #[must_use]
    pub fn mide(&mut self) -> MIDE_W<29> {
        MIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mailbox Acceptance Mask Register (MB = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mam3](index.html) module"]
pub struct MAM3_SPEC;
impl crate::RegisterSpec for MAM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mam3::R](R) reader structure"]
impl crate::Readable for MAM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mam3::W](W) writer structure"]
impl crate::Writable for MAM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAM3 to value 0"]
impl crate::Resettable for MAM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
