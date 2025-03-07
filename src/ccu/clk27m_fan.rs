#[doc = "Register `CLK27M_FAN` reader"]
pub struct R(crate::R<CLK27M_FAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK27M_FAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK27M_FAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK27M_FAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK27M_FAN` writer"]
pub struct W(crate::W<CLK27M_FAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK27M_FAN_SPEC>;
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
impl From<crate::W<CLK27M_FAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK27M_FAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating for CLK27M\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    OFF = 0,
    #[doc = "1: `1`"]
    ON = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATING` reader - Gating for CLK27M"]
pub struct GATING_R(crate::FieldReader<bool, GATING_A>);
impl GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::OFF,
            true => GATING_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == GATING_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == GATING_A::ON
    }
}
impl core::ops::Deref for GATING_R {
    type Target = crate::FieldReader<bool, GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GATING` writer - Gating for CLK27M"]
pub struct GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(GATING_A::OFF)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(GATING_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: `0`"]
    PLL_VIDEO0_1X = 0,
    #[doc = "1: `1`"]
    PLL_VIDEO1_1X = 1,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_SRC_SEL` reader - Clock Source Select"]
pub struct CLK_SRC_SEL_R(crate::FieldReader<u8, CLK_SRC_SEL_A>);
impl CLK_SRC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::PLL_VIDEO0_1X),
            1 => Some(CLK_SRC_SEL_A::PLL_VIDEO1_1X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO0_1X`"]
    #[inline(always)]
    pub fn is_pll_video0_1x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO0_1X
    }
    #[doc = "Checks if the value of the field is `PLL_VIDEO1_1X`"]
    #[inline(always)]
    pub fn is_pll_video1_1x(&self) -> bool {
        **self == CLK_SRC_SEL_A::PLL_VIDEO1_1X
    }
}
impl core::ops::Deref for CLK_SRC_SEL_R {
    type Target = crate::FieldReader<u8, CLK_SRC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK_SRC_SEL` writer - Clock Source Select"]
pub struct CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_SRC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_SRC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pll_video0_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO0_1X)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pll_video1_1x(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::PLL_VIDEO1_1X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `DIV1` reader - Factor N"]
pub struct DIV1_R(crate::FieldReader<u8, u8>);
impl DIV1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV1` writer - Factor N"]
pub struct DIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DIV0` reader - Factor M"]
pub struct DIV0_R(crate::FieldReader<u8, u8>);
impl DIV0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV0` writer - Factor M"]
pub struct DIV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn div1(&self) -> DIV1_R {
        DIV1_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div0(&self) -> DIV0_R {
        DIV0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Gating for CLK27M"]
    #[inline(always)]
    pub fn gating(&mut self) -> GATING_W {
        GATING_W { w: self }
    }
    #[doc = "Bits 24:25 - Clock Source Select"]
    #[inline(always)]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W {
        CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Factor N"]
    #[inline(always)]
    pub fn div1(&mut self) -> DIV1_W {
        DIV1_W { w: self }
    }
    #[doc = "Bits 0:4 - Factor M"]
    #[inline(always)]
    pub fn div0(&mut self) -> DIV0_W {
        DIV0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLK27M FANOUT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk27m_fan](index.html) module"]
pub struct CLK27M_FAN_SPEC;
impl crate::RegisterSpec for CLK27M_FAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk27m_fan::R](R) reader structure"]
impl crate::Readable for CLK27M_FAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk27m_fan::W](W) writer structure"]
impl crate::Writable for CLK27M_FAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK27M_FAN to value 0"]
impl crate::Resettable for CLK27M_FAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
