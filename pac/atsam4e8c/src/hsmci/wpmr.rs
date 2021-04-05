#[doc = "Reader of register WPMR"]
pub type R = crate::R<u32, super::WPMR>;
#[doc = "Writer for register WPMR"]
pub type W = crate::W<u32, super::WPMR>;
#[doc = "Reader of field `WP_EN`"]
pub type WP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WP_EN`"]
pub struct WP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `WP_KEY`"]
pub type WP_KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WP_KEY`"]
pub struct WP_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&self) -> WP_EN_R {
        WP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    pub fn wp_key(&self) -> WP_KEY_R {
        WP_KEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&mut self) -> WP_EN_W {
        WP_EN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key password"]
    #[inline(always)]
    pub fn wp_key(&mut self) -> WP_KEY_W {
        WP_KEY_W { w: self }
    }
}
