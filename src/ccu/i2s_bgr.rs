#[doc = "Register `I2S_BGR` reader"]
pub struct R(crate::R<I2S_BGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_BGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_BGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_BGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_BGR` writer"]
pub struct W(crate::W<I2S_BGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_BGR_SPEC>;
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
impl From<crate::W<I2S_BGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_BGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_RST_A {
    #[doc = "0: `0`"]
    ASSERT = 0,
    #[doc = "1: `1`"]
    DEASSERT = 1,
}
impl From<I2S_RST_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `I2S(0-2)_RST` reader - Reset"]
pub struct I2S_RST_R(crate::FieldReader<bool, I2S_RST_A>);
impl I2S_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_RST_A {
        match self.bits {
            false => I2S_RST_A::ASSERT,
            true => I2S_RST_A::DEASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == I2S_RST_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `DEASSERT`"]
    #[inline(always)]
    pub fn is_deassert(&self) -> bool {
        **self == I2S_RST_A::DEASSERT
    }
}
impl core::ops::Deref for I2S_RST_R {
    type Target = crate::FieldReader<bool, I2S_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `I2S(0-2)_RST` writer - Reset"]
pub struct I2S_RST_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> I2S_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2S_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(I2S_RST_A::DEASSERT)
    }
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `I2S(0-2)_RST` const generic writer - Reset"]
pub struct I2S_RST_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> I2S_RST_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(I2S_RST_A::ASSERT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn deassert(self) -> &'a mut W {
        self.variant(I2S_RST_A::DEASSERT)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2S_GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<I2S_GATING_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Fields `I2S(0-2)_GATING` reader - Gating Clock"]
pub struct I2S_GATING_R(crate::FieldReader<bool, I2S_GATING_A>);
impl I2S_GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2S_GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_GATING_A {
        match self.bits {
            false => I2S_GATING_A::MASK,
            true => I2S_GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == I2S_GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == I2S_GATING_A::PASS
    }
}
impl core::ops::Deref for I2S_GATING_R {
    type Target = crate::FieldReader<bool, I2S_GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `I2S(0-2)_GATING` writer - Gating Clock"]
pub struct I2S_GATING_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> I2S_GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(I2S_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(I2S_GATING_A::PASS)
    }
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
        self.w.bits =
            (self.w.bits & !(0x01 << self.offset)) | ((value as u32 & 0x01) << self.offset);
        self.w
    }
}
#[doc = "Fields `I2S(0-2)_GATING` const generic writer - Gating Clock"]
pub struct I2S_GATING_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> I2S_GATING_CGW<'a, O> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S_GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(I2S_GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(I2S_GATING_A::PASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << O)) | ((value as u32 & 0x01) << O);
        self.w
    }
}
impl R {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn i2s_rst(&self, n: usize) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> (n + 16)) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn i2s0_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn i2s1_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn i2s2_rst(&self) -> I2S_RST_R {
        I2S_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn i2s_gating(&self, n: usize) -> I2S_GATING_R {
        I2S_GATING_R::new(((self.bits >> n) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn i2s0_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn i2s1_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn i2s2_gating(&self) -> I2S_GATING_R {
        I2S_GATING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Reset"]
    #[inline(always)]
    pub unsafe fn i2s_rst(&mut self, n: usize) -> I2S_RST_W {
        I2S_RST_W {
            w: self,
            offset: n + 16,
        }
    }
    #[doc = "Bit 16 - Reset"]
    #[inline(always)]
    pub fn i2s0_rst(&mut self) -> I2S_RST_CGW<16> {
        I2S_RST_CGW { w: self }
    }
    #[doc = "Bit 17 - Reset"]
    #[inline(always)]
    pub fn i2s1_rst(&mut self) -> I2S_RST_CGW<17> {
        I2S_RST_CGW { w: self }
    }
    #[doc = "Bit 18 - Reset"]
    #[inline(always)]
    pub fn i2s2_rst(&mut self) -> I2S_RST_CGW<18> {
        I2S_RST_CGW { w: self }
    }
    #[doc = "Gating Clock"]
    #[inline(always)]
    pub unsafe fn i2s_gating(&mut self, n: usize) -> I2S_GATING_W {
        I2S_GATING_W { w: self, offset: n }
    }
    #[doc = "Bit 0 - Gating Clock"]
    #[inline(always)]
    pub fn i2s0_gating(&mut self) -> I2S_GATING_CGW<0> {
        I2S_GATING_CGW { w: self }
    }
    #[doc = "Bit 1 - Gating Clock"]
    #[inline(always)]
    pub fn i2s1_gating(&mut self) -> I2S_GATING_CGW<1> {
        I2S_GATING_CGW { w: self }
    }
    #[doc = "Bit 2 - Gating Clock"]
    #[inline(always)]
    pub fn i2s2_gating(&mut self) -> I2S_GATING_CGW<2> {
        I2S_GATING_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Bus Gating Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_bgr](index.html) module"]
pub struct I2S_BGR_SPEC;
impl crate::RegisterSpec for I2S_BGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_bgr::R](R) reader structure"]
impl crate::Readable for I2S_BGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_bgr::W](W) writer structure"]
impl crate::Writable for I2S_BGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2S_BGR to value 0"]
impl crate::Resettable for I2S_BGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
