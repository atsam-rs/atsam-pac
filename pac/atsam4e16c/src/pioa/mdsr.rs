#[doc = "Register `MDSR` reader"]
pub struct R(crate::R<MDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P0` reader - Multi-drive Status"]
pub struct P0_R(crate::FieldReader<bool, bool>);
impl P0_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1` reader - Multi-drive Status"]
pub struct P1_R(crate::FieldReader<bool, bool>);
impl P1_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2` reader - Multi-drive Status"]
pub struct P2_R(crate::FieldReader<bool, bool>);
impl P2_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3` reader - Multi-drive Status"]
pub struct P3_R(crate::FieldReader<bool, bool>);
impl P3_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4` reader - Multi-drive Status"]
pub struct P4_R(crate::FieldReader<bool, bool>);
impl P4_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5` reader - Multi-drive Status"]
pub struct P5_R(crate::FieldReader<bool, bool>);
impl P5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6` reader - Multi-drive Status"]
pub struct P6_R(crate::FieldReader<bool, bool>);
impl P6_R {
    pub(crate) fn new(bits: bool) -> Self {
        P6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7` reader - Multi-drive Status"]
pub struct P7_R(crate::FieldReader<bool, bool>);
impl P7_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8` reader - Multi-drive Status"]
pub struct P8_R(crate::FieldReader<bool, bool>);
impl P8_R {
    pub(crate) fn new(bits: bool) -> Self {
        P8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9` reader - Multi-drive Status"]
pub struct P9_R(crate::FieldReader<bool, bool>);
impl P9_R {
    pub(crate) fn new(bits: bool) -> Self {
        P9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10` reader - Multi-drive Status"]
pub struct P10_R(crate::FieldReader<bool, bool>);
impl P10_R {
    pub(crate) fn new(bits: bool) -> Self {
        P10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P11` reader - Multi-drive Status"]
pub struct P11_R(crate::FieldReader<bool, bool>);
impl P11_R {
    pub(crate) fn new(bits: bool) -> Self {
        P11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P12` reader - Multi-drive Status"]
pub struct P12_R(crate::FieldReader<bool, bool>);
impl P12_R {
    pub(crate) fn new(bits: bool) -> Self {
        P12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P13` reader - Multi-drive Status"]
pub struct P13_R(crate::FieldReader<bool, bool>);
impl P13_R {
    pub(crate) fn new(bits: bool) -> Self {
        P13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P14` reader - Multi-drive Status"]
pub struct P14_R(crate::FieldReader<bool, bool>);
impl P14_R {
    pub(crate) fn new(bits: bool) -> Self {
        P14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P15` reader - Multi-drive Status"]
pub struct P15_R(crate::FieldReader<bool, bool>);
impl P15_R {
    pub(crate) fn new(bits: bool) -> Self {
        P15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P16` reader - Multi-drive Status"]
pub struct P16_R(crate::FieldReader<bool, bool>);
impl P16_R {
    pub(crate) fn new(bits: bool) -> Self {
        P16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P17` reader - Multi-drive Status"]
pub struct P17_R(crate::FieldReader<bool, bool>);
impl P17_R {
    pub(crate) fn new(bits: bool) -> Self {
        P17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P18` reader - Multi-drive Status"]
pub struct P18_R(crate::FieldReader<bool, bool>);
impl P18_R {
    pub(crate) fn new(bits: bool) -> Self {
        P18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P19` reader - Multi-drive Status"]
pub struct P19_R(crate::FieldReader<bool, bool>);
impl P19_R {
    pub(crate) fn new(bits: bool) -> Self {
        P19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P20` reader - Multi-drive Status"]
pub struct P20_R(crate::FieldReader<bool, bool>);
impl P20_R {
    pub(crate) fn new(bits: bool) -> Self {
        P20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P21` reader - Multi-drive Status"]
pub struct P21_R(crate::FieldReader<bool, bool>);
impl P21_R {
    pub(crate) fn new(bits: bool) -> Self {
        P21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P22` reader - Multi-drive Status"]
pub struct P22_R(crate::FieldReader<bool, bool>);
impl P22_R {
    pub(crate) fn new(bits: bool) -> Self {
        P22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P23` reader - Multi-drive Status"]
pub struct P23_R(crate::FieldReader<bool, bool>);
impl P23_R {
    pub(crate) fn new(bits: bool) -> Self {
        P23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P24` reader - Multi-drive Status"]
pub struct P24_R(crate::FieldReader<bool, bool>);
impl P24_R {
    pub(crate) fn new(bits: bool) -> Self {
        P24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P25` reader - Multi-drive Status"]
pub struct P25_R(crate::FieldReader<bool, bool>);
impl P25_R {
    pub(crate) fn new(bits: bool) -> Self {
        P25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P26` reader - Multi-drive Status"]
pub struct P26_R(crate::FieldReader<bool, bool>);
impl P26_R {
    pub(crate) fn new(bits: bool) -> Self {
        P26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P27` reader - Multi-drive Status"]
pub struct P27_R(crate::FieldReader<bool, bool>);
impl P27_R {
    pub(crate) fn new(bits: bool) -> Self {
        P27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P28` reader - Multi-drive Status"]
pub struct P28_R(crate::FieldReader<bool, bool>);
impl P28_R {
    pub(crate) fn new(bits: bool) -> Self {
        P28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P29` reader - Multi-drive Status"]
pub struct P29_R(crate::FieldReader<bool, bool>);
impl P29_R {
    pub(crate) fn new(bits: bool) -> Self {
        P29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P30` reader - Multi-drive Status"]
pub struct P30_R(crate::FieldReader<bool, bool>);
impl P30_R {
    pub(crate) fn new(bits: bool) -> Self {
        P30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P31` reader - Multi-drive Status"]
pub struct P31_R(crate::FieldReader<bool, bool>);
impl P31_R {
    pub(crate) fn new(bits: bool) -> Self {
        P31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Multi-drive Status"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Multi-drive Status"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Multi-drive Status"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Multi-drive Status"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Multi-drive Status"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Multi-drive Status"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multi-drive Status"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Multi-drive Status"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Multi-drive Status"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-drive Status"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multi-drive Status"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Multi-drive Status"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Multi-drive Status"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Multi-drive Status"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Multi-drive Status"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Multi-drive Status"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Multi-drive Status"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Multi-drive Status"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Multi-drive Status"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Multi-drive Status"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Multi-drive Status"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Multi-drive Status"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Multi-drive Status"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Multi-drive Status"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Multi-drive Status"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Multi-drive Status"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Multi-drive Status"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Multi-drive Status"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Multi-drive Status"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Multi-drive Status"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Multi-drive Status"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Multi-drive Status"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Multi-driver Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsr](index.html) module"]
pub struct MDSR_SPEC;
impl crate::RegisterSpec for MDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdsr::R](R) reader structure"]
impl crate::Readable for MDSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDSR to value 0"]
impl crate::Resettable for MDSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
