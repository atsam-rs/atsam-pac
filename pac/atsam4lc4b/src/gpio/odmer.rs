#[doc = "Register `ODMER%s` reader"]
pub struct R(crate::R<ODMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODMER%s` writer"]
pub struct W(crate::W<ODMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODMER_SPEC>;
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
impl From<crate::W<ODMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` reader - Open Drain Mode Enable"]
pub type P0_R = crate::BitReader<bool>;
#[doc = "Field `P0` writer - Open Drain Mode Enable"]
pub type P0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P1` reader - Open Drain Mode Enable"]
pub type P1_R = crate::BitReader<bool>;
#[doc = "Field `P1` writer - Open Drain Mode Enable"]
pub type P1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P2` reader - Open Drain Mode Enable"]
pub type P2_R = crate::BitReader<bool>;
#[doc = "Field `P2` writer - Open Drain Mode Enable"]
pub type P2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P3` reader - Open Drain Mode Enable"]
pub type P3_R = crate::BitReader<bool>;
#[doc = "Field `P3` writer - Open Drain Mode Enable"]
pub type P3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P4` reader - Open Drain Mode Enable"]
pub type P4_R = crate::BitReader<bool>;
#[doc = "Field `P4` writer - Open Drain Mode Enable"]
pub type P4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P5` reader - Open Drain Mode Enable"]
pub type P5_R = crate::BitReader<bool>;
#[doc = "Field `P5` writer - Open Drain Mode Enable"]
pub type P5_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P6` reader - Open Drain Mode Enable"]
pub type P6_R = crate::BitReader<bool>;
#[doc = "Field `P6` writer - Open Drain Mode Enable"]
pub type P6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P7` reader - Open Drain Mode Enable"]
pub type P7_R = crate::BitReader<bool>;
#[doc = "Field `P7` writer - Open Drain Mode Enable"]
pub type P7_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P8` reader - Open Drain Mode Enable"]
pub type P8_R = crate::BitReader<bool>;
#[doc = "Field `P8` writer - Open Drain Mode Enable"]
pub type P8_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P9` reader - Open Drain Mode Enable"]
pub type P9_R = crate::BitReader<bool>;
#[doc = "Field `P9` writer - Open Drain Mode Enable"]
pub type P9_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P10` reader - Open Drain Mode Enable"]
pub type P10_R = crate::BitReader<bool>;
#[doc = "Field `P10` writer - Open Drain Mode Enable"]
pub type P10_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P11` reader - Open Drain Mode Enable"]
pub type P11_R = crate::BitReader<bool>;
#[doc = "Field `P11` writer - Open Drain Mode Enable"]
pub type P11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P12` reader - Open Drain Mode Enable"]
pub type P12_R = crate::BitReader<bool>;
#[doc = "Field `P12` writer - Open Drain Mode Enable"]
pub type P12_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P13` reader - Open Drain Mode Enable"]
pub type P13_R = crate::BitReader<bool>;
#[doc = "Field `P13` writer - Open Drain Mode Enable"]
pub type P13_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P14` reader - Open Drain Mode Enable"]
pub type P14_R = crate::BitReader<bool>;
#[doc = "Field `P14` writer - Open Drain Mode Enable"]
pub type P14_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P15` reader - Open Drain Mode Enable"]
pub type P15_R = crate::BitReader<bool>;
#[doc = "Field `P15` writer - Open Drain Mode Enable"]
pub type P15_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P16` reader - Open Drain Mode Enable"]
pub type P16_R = crate::BitReader<bool>;
#[doc = "Field `P16` writer - Open Drain Mode Enable"]
pub type P16_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P17` reader - Open Drain Mode Enable"]
pub type P17_R = crate::BitReader<bool>;
#[doc = "Field `P17` writer - Open Drain Mode Enable"]
pub type P17_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P18` reader - Open Drain Mode Enable"]
pub type P18_R = crate::BitReader<bool>;
#[doc = "Field `P18` writer - Open Drain Mode Enable"]
pub type P18_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P19` reader - Open Drain Mode Enable"]
pub type P19_R = crate::BitReader<bool>;
#[doc = "Field `P19` writer - Open Drain Mode Enable"]
pub type P19_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P20` reader - Open Drain Mode Enable"]
pub type P20_R = crate::BitReader<bool>;
#[doc = "Field `P20` writer - Open Drain Mode Enable"]
pub type P20_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P21` reader - Open Drain Mode Enable"]
pub type P21_R = crate::BitReader<bool>;
#[doc = "Field `P21` writer - Open Drain Mode Enable"]
pub type P21_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P22` reader - Open Drain Mode Enable"]
pub type P22_R = crate::BitReader<bool>;
#[doc = "Field `P22` writer - Open Drain Mode Enable"]
pub type P22_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P23` reader - Open Drain Mode Enable"]
pub type P23_R = crate::BitReader<bool>;
#[doc = "Field `P23` writer - Open Drain Mode Enable"]
pub type P23_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P24` reader - Open Drain Mode Enable"]
pub type P24_R = crate::BitReader<bool>;
#[doc = "Field `P24` writer - Open Drain Mode Enable"]
pub type P24_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P25` reader - Open Drain Mode Enable"]
pub type P25_R = crate::BitReader<bool>;
#[doc = "Field `P25` writer - Open Drain Mode Enable"]
pub type P25_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P26` reader - Open Drain Mode Enable"]
pub type P26_R = crate::BitReader<bool>;
#[doc = "Field `P26` writer - Open Drain Mode Enable"]
pub type P26_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P27` reader - Open Drain Mode Enable"]
pub type P27_R = crate::BitReader<bool>;
#[doc = "Field `P27` writer - Open Drain Mode Enable"]
pub type P27_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P28` reader - Open Drain Mode Enable"]
pub type P28_R = crate::BitReader<bool>;
#[doc = "Field `P28` writer - Open Drain Mode Enable"]
pub type P28_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P29` reader - Open Drain Mode Enable"]
pub type P29_R = crate::BitReader<bool>;
#[doc = "Field `P29` writer - Open Drain Mode Enable"]
pub type P29_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P30` reader - Open Drain Mode Enable"]
pub type P30_R = crate::BitReader<bool>;
#[doc = "Field `P30` writer - Open Drain Mode Enable"]
pub type P30_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
#[doc = "Field `P31` reader - Open Drain Mode Enable"]
pub type P31_R = crate::BitReader<bool>;
#[doc = "Field `P31` writer - Open Drain Mode Enable"]
pub type P31_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODMER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Open Drain Mode Enable"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - Open Drain Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<31> {
        P31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open Drain Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odmer](index.html) module"]
pub struct ODMER_SPEC;
impl crate::RegisterSpec for ODMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odmer::R](R) reader structure"]
impl crate::Readable for ODMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odmer::W](W) writer structure"]
impl crate::Writable for ODMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ODMER%s to value 0"]
impl crate::Resettable for ODMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
