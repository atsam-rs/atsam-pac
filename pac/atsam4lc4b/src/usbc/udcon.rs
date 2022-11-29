#[doc = "Register `UDCON` reader"]
pub struct R(crate::R<UDCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDCON` writer"]
pub struct W(crate::W<UDCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDCON_SPEC>;
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
impl From<crate::W<UDCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UADD` reader - USB Address"]
pub type UADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UADD` writer - USB Address"]
pub type UADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDCON_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDEN` reader - Address Enable"]
pub type ADDEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDEN` writer - Address Enable"]
pub type ADDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `DETACH` reader - Detach"]
pub type DETACH_R = crate::BitReader<bool>;
#[doc = "Field `DETACH` writer - Detach"]
pub type DETACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `RMWKUP` reader - Remote Wake-Up"]
pub type RMWKUP_R = crate::BitReader<bool>;
#[doc = "Field `RMWKUP` writer - Remote Wake-Up"]
pub type RMWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `SPDCONF` reader - Speed configuration"]
pub type SPDCONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPDCONF` writer - Speed configuration"]
pub type SPDCONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `LS` reader - Low Speed Mode Force"]
pub type LS_R = crate::BitReader<bool>;
#[doc = "Field `LS` writer - Low Speed Mode Force"]
pub type LS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TSTJ_R = crate::BitReader<bool>;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TSTJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TSTK_R = crate::BitReader<bool>;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TSTK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `TSTPCKT` reader - Test Packet mode"]
pub type TSTPCKT_R = crate::BitReader<bool>;
#[doc = "Field `TSTPCKT` writer - Test Packet mode"]
pub type TSTPCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `OPMODE2` reader - Specific Operational mode"]
pub type OPMODE2_R = crate::BitReader<bool>;
#[doc = "Field `OPMODE2` writer - Specific Operational mode"]
pub type OPMODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
#[doc = "Field `GNAK` reader - Global NAK"]
pub type GNAK_R = crate::BitReader<bool>;
#[doc = "Field `GNAK` writer - Global NAK"]
pub type GNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDCON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&self) -> UADD_R {
        UADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&self) -> RMWKUP_R {
        RMWKUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Speed configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Low Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test Packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Global NAK"]
    #[inline(always)]
    pub fn gnak(&self) -> GNAK_R {
        GNAK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    #[must_use]
    pub fn uadd(&mut self) -> UADD_W<0> {
        UADD_W::new(self)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adden(&mut self) -> ADDEN_W<7> {
        ADDEN_W::new(self)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    #[must_use]
    pub fn detach(&mut self) -> DETACH_W<8> {
        DETACH_W::new(self)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    #[must_use]
    pub fn rmwkup(&mut self) -> RMWKUP_W<9> {
        RMWKUP_W::new(self)
    }
    #[doc = "Bits 10:11 - Speed configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<10> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 12 - Low Speed Mode Force"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LS_W<12> {
        LS_W::new(self)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TSTJ_W<13> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TSTK_W<14> {
        TSTK_W::new(self)
    }
    #[doc = "Bit 15 - Test Packet mode"]
    #[inline(always)]
    #[must_use]
    pub fn tstpckt(&mut self) -> TSTPCKT_W<15> {
        TSTPCKT_W::new(self)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    #[must_use]
    pub fn opmode2(&mut self) -> OPMODE2_W<16> {
        OPMODE2_W::new(self)
    }
    #[doc = "Bit 17 - Global NAK"]
    #[inline(always)]
    #[must_use]
    pub fn gnak(&mut self) -> GNAK_W<17> {
        GNAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udcon](index.html) module"]
pub struct UDCON_SPEC;
impl crate::RegisterSpec for UDCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udcon::R](R) reader structure"]
impl crate::Readable for UDCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udcon::W](W) writer structure"]
impl crate::Writable for UDCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDCON to value 0x0100"]
impl crate::Resettable for UDCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
