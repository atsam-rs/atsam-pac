#[doc = "Register `UHCON` reader"]
pub struct R(crate::R<UHCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UHCON` writer"]
pub struct W(crate::W<UHCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHCON_SPEC>;
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
impl From<crate::W<UHCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFE` reader - SOF Enable"]
pub type SOFE_R = crate::BitReader<bool>;
#[doc = "Field `SOFE` writer - SOF Enable"]
pub type SOFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHCON_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Send USB Reset"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Send USB Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHCON_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHCON_SPEC, bool, O>;
#[doc = "Field `SPDCONF` reader - Speed Configuration"]
pub type SPDCONF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPDCONF` writer - Speed Configuration"]
pub type SPDCONF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UHCON_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSTJ` reader - Test J"]
pub type TSTJ_R = crate::BitReader<bool>;
#[doc = "Field `TSTJ` writer - Test J"]
pub type TSTJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHCON_SPEC, bool, O>;
#[doc = "Field `TSTK` reader - Test K"]
pub type TSTK_R = crate::BitReader<bool>;
#[doc = "Field `TSTK` writer - Test K"]
pub type TSTK_W<'a, const O: u8> = crate::BitWriter<'a, u32, UHCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - SOF Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Test J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Test K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SOF Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofe(&mut self) -> SOFE_W<8> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<9> {
        RESET_W::new(self)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<10> {
        RESUME_W::new(self)
    }
    #[doc = "Bits 12:13 - Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn spdconf(&mut self) -> SPDCONF_W<12> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 16 - Test J"]
    #[inline(always)]
    #[must_use]
    pub fn tstj(&mut self) -> TSTJ_W<16> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 17 - Test K"]
    #[inline(always)]
    #[must_use]
    pub fn tstk(&mut self) -> TSTK_W<17> {
        TSTK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhcon](index.html) module"]
pub struct UHCON_SPEC;
impl crate::RegisterSpec for UHCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhcon::R](R) reader structure"]
impl crate::Readable for UHCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uhcon::W](W) writer structure"]
impl crate::Writable for UHCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UHCON to value 0"]
impl crate::Resettable for UHCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
