#[doc = "Register `DIS` writer"]
pub struct W(crate::W<DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIS_SPEC>;
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
impl From<crate::W<DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMI` writer - External Non Maskable CPU interrupt"]
pub type NMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT1` writer - External Interrupt 1"]
pub type INT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT2` writer - External Interrupt 2"]
pub type INT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT3` writer - External Interrupt 3"]
pub type INT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT4` writer - External Interrupt 4"]
pub type INT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT5` writer - External Interrupt 5"]
pub type INT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT6` writer - External Interrupt 6"]
pub type INT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT7` writer - External Interrupt 7"]
pub type INT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT8` writer - External Interrupt 8"]
pub type INT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT9` writer - External Interrupt 9"]
pub type INT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT10` writer - External Interrupt 10"]
pub type INT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT11` writer - External Interrupt 11"]
pub type INT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT12` writer - External Interrupt 12"]
pub type INT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT13` writer - External Interrupt 13"]
pub type INT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT14` writer - External Interrupt 14"]
pub type INT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
#[doc = "Field `INT15` writer - External Interrupt 15"]
pub type INT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<0> {
        NMI_W::new(self)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn int1(&mut self) -> INT1_W<1> {
        INT1_W::new(self)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn int2(&mut self) -> INT2_W<2> {
        INT2_W::new(self)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn int3(&mut self) -> INT3_W<3> {
        INT3_W::new(self)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn int4(&mut self) -> INT4_W<4> {
        INT4_W::new(self)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn int5(&mut self) -> INT5_W<5> {
        INT5_W::new(self)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn int6(&mut self) -> INT6_W<6> {
        INT6_W::new(self)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn int7(&mut self) -> INT7_W<7> {
        INT7_W::new(self)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn int8(&mut self) -> INT8_W<8> {
        INT8_W::new(self)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn int9(&mut self) -> INT9_W<9> {
        INT9_W::new(self)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn int10(&mut self) -> INT10_W<10> {
        INT10_W::new(self)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn int11(&mut self) -> INT11_W<11> {
        INT11_W::new(self)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn int12(&mut self) -> INT12_W<12> {
        INT12_W::new(self)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn int13(&mut self) -> INT13_W<13> {
        INT13_W::new(self)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn int14(&mut self) -> INT14_W<14> {
        INT14_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn int15(&mut self) -> INT15_W<15> {
        INT15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](index.html) module"]
pub struct DIS_SPEC;
impl crate::RegisterSpec for DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dis::W](W) writer structure"]
impl crate::Writable for DIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
