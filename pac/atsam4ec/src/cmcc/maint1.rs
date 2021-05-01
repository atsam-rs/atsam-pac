#[doc = "Writer for register MAINT1"]
pub type W = crate::W<u32, super::MAINT1>;
#[doc = "Write proxy for field `INDEX`"]
pub struct INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | (((value as u32) & 0x1f) << 4);
        self.w
    }
}
#[doc = "Invalidate Way"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAY_AW {
    #[doc = "0: Way 0 is selection for index invalidation"]
    WAY0 = 0,
    #[doc = "1: Way 1 is selection for index invalidation"]
    WAY1 = 1,
    #[doc = "2: Way 2 is selection for index invalidation"]
    WAY2 = 2,
    #[doc = "3: Way 3 is selection for index invalidation"]
    WAY3 = 3,
}
impl From<WAY_AW> for u8 {
    #[inline(always)]
    fn from(variant: WAY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `WAY`"]
pub struct WAY_W<'a> {
    w: &'a mut W,
}
impl<'a> WAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAY_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Way 0 is selection for index invalidation"]
    #[inline(always)]
    pub fn way0(self) -> &'a mut W {
        self.variant(WAY_AW::WAY0)
    }
    #[doc = "Way 1 is selection for index invalidation"]
    #[inline(always)]
    pub fn way1(self) -> &'a mut W {
        self.variant(WAY_AW::WAY1)
    }
    #[doc = "Way 2 is selection for index invalidation"]
    #[inline(always)]
    pub fn way2(self) -> &'a mut W {
        self.variant(WAY_AW::WAY2)
    }
    #[doc = "Way 3 is selection for index invalidation"]
    #[inline(always)]
    pub fn way3(self) -> &'a mut W {
        self.variant(WAY_AW::WAY3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl W {
    #[doc = "Bits 4:8 - Invalidate Index"]
    #[inline(always)]
    pub fn index(&mut self) -> INDEX_W {
        INDEX_W { w: self }
    }
    #[doc = "Bits 30:31 - Invalidate Way"]
    #[inline(always)]
    pub fn way(&mut self) -> WAY_W {
        WAY_W { w: self }
    }
}
