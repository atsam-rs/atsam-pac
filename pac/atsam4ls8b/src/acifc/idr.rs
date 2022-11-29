#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACINT0` writer - AC0 Interrupt Disable"]
pub type ACINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT0` writer - AC0 Startup Time Interrupt Disable"]
pub type SUTINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT1` writer - AC1 Interrupt Disable"]
pub type ACINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT1` writer - AC1 Startup Time Interrupt Disable"]
pub type SUTINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT2` writer - AC2 Interrupt Disable"]
pub type ACINT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT2` writer - AC2 Startup Time Interrupt Disable"]
pub type SUTINT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT3` writer - AC3 Interrupt Disable"]
pub type ACINT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT3` writer - AC3 Startup Time Interrupt Disable"]
pub type SUTINT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT4` writer - AC4 Interrupt Disable"]
pub type ACINT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT4` writer - AC4 Startup Time Interrupt Disable"]
pub type SUTINT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT5` writer - AC5 Interrupt Disable"]
pub type ACINT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT5` writer - AC5 Startup Time Interrupt Disable"]
pub type SUTINT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT6` writer - AC6 Interrupt Disable"]
pub type ACINT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT6` writer - AC6 Startup Time Interrupt Disable"]
pub type SUTINT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `ACINT7` writer - AC7 Interrupt Disable"]
pub type ACINT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `SUTINT7` writer - AC7 Startup Time Interrupt Disable"]
pub type SUTINT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WFINT0` writer - Window0 Mode Interrupt Disable"]
pub type WFINT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WFINT1` writer - Window1 Mode Interrupt Disable"]
pub type WFINT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WFINT2` writer - Window2 Mode Interrupt Disable"]
pub type WFINT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `WFINT3` writer - Window3 Mode Interrupt Disable"]
pub type WFINT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - AC0 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint0(&mut self) -> ACINT0_W<0> {
        ACINT0_W::new(self)
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint0(&mut self) -> SUTINT0_W<1> {
        SUTINT0_W::new(self)
    }
    #[doc = "Bit 2 - AC1 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint1(&mut self) -> ACINT1_W<2> {
        ACINT1_W::new(self)
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint1(&mut self) -> SUTINT1_W<3> {
        SUTINT1_W::new(self)
    }
    #[doc = "Bit 4 - AC2 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint2(&mut self) -> ACINT2_W<4> {
        ACINT2_W::new(self)
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint2(&mut self) -> SUTINT2_W<5> {
        SUTINT2_W::new(self)
    }
    #[doc = "Bit 6 - AC3 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint3(&mut self) -> ACINT3_W<6> {
        ACINT3_W::new(self)
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint3(&mut self) -> SUTINT3_W<7> {
        SUTINT3_W::new(self)
    }
    #[doc = "Bit 8 - AC4 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint4(&mut self) -> ACINT4_W<8> {
        ACINT4_W::new(self)
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint4(&mut self) -> SUTINT4_W<9> {
        SUTINT4_W::new(self)
    }
    #[doc = "Bit 10 - AC5 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint5(&mut self) -> ACINT5_W<10> {
        ACINT5_W::new(self)
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint5(&mut self) -> SUTINT5_W<11> {
        SUTINT5_W::new(self)
    }
    #[doc = "Bit 12 - AC6 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint6(&mut self) -> ACINT6_W<12> {
        ACINT6_W::new(self)
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint6(&mut self) -> SUTINT6_W<13> {
        SUTINT6_W::new(self)
    }
    #[doc = "Bit 14 - AC7 Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn acint7(&mut self) -> ACINT7_W<14> {
        ACINT7_W::new(self)
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sutint7(&mut self) -> SUTINT7_W<15> {
        SUTINT7_W::new(self)
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wfint0(&mut self) -> WFINT0_W<24> {
        WFINT0_W::new(self)
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wfint1(&mut self) -> WFINT1_W<25> {
        WFINT1_W::new(self)
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wfint2(&mut self) -> WFINT2_W<26> {
        WFINT2_W::new(self)
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wfint3(&mut self) -> WFINT3_W<27> {
        WFINT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
