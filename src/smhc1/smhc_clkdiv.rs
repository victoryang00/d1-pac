#[doc = "Register `SMHC_CLKDIV` reader"]
pub struct R(crate::R<SMHC_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_CLKDIV` writer"]
pub struct W(crate::W<SMHC_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CLKDIV_SPEC>;
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
impl From<crate::W<SMHC_CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASK_DATA0_A {
    #[doc = "0: Do not mask data0 when update clock"]
    NOT_MASK = 0,
    #[doc = "1: Mask data0 when update clock"]
    MASK = 1,
}
impl From<MASK_DATA0_A> for bool {
    #[inline(always)]
    fn from(variant: MASK_DATA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASK_DATA0` reader - "]
pub struct MASK_DATA0_R(crate::FieldReader<bool, MASK_DATA0_A>);
impl MASK_DATA0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_DATA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASK_DATA0_A {
        match self.bits {
            false => MASK_DATA0_A::NOT_MASK,
            true => MASK_DATA0_A::MASK,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_MASK`"]
    #[inline(always)]
    pub fn is_not_mask(&self) -> bool {
        **self == MASK_DATA0_A::NOT_MASK
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == MASK_DATA0_A::MASK
    }
}
impl core::ops::Deref for MASK_DATA0_R {
    type Target = crate::FieldReader<bool, MASK_DATA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_DATA0` writer - "]
pub struct MASK_DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_DATA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MASK_DATA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Do not mask data0 when update clock"]
    #[inline(always)]
    pub fn not_mask(self) -> &'a mut W {
        self.variant(MASK_DATA0_A::NOT_MASK)
    }
    #[doc = "Mask data0 when update clock"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(MASK_DATA0_A::MASK)
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
#[doc = "Card Clock Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_CTRL_A {
    #[doc = "0: Card clock is always on"]
    ON = 0,
    #[doc = "1: Turn off card clock when FSM is in IDLE state"]
    OFF_IDLE = 1,
}
impl From<CCLK_CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_CTRL` reader - Card Clock Output Control"]
pub struct CCLK_CTRL_R(crate::FieldReader<bool, CCLK_CTRL_A>);
impl CCLK_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_CTRL_A {
        match self.bits {
            false => CCLK_CTRL_A::ON,
            true => CCLK_CTRL_A::OFF_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CCLK_CTRL_A::ON
    }
    #[doc = "Checks if the value of the field is `OFF_IDLE`"]
    #[inline(always)]
    pub fn is_off_idle(&self) -> bool {
        **self == CCLK_CTRL_A::OFF_IDLE
    }
}
impl core::ops::Deref for CCLK_CTRL_R {
    type Target = crate::FieldReader<bool, CCLK_CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK_CTRL` writer - Card Clock Output Control"]
pub struct CCLK_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card clock is always on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CCLK_CTRL_A::ON)
    }
    #[doc = "Turn off card clock when FSM is in IDLE state"]
    #[inline(always)]
    pub fn off_idle(self) -> &'a mut W {
        self.variant(CCLK_CTRL_A::OFF_IDLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Card Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_ENB_A {
    #[doc = "0: Card Clock is off"]
    OFF = 0,
    #[doc = "1: Card Clock is on"]
    ON = 1,
}
impl From<CCLK_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CCLK_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_ENB` reader - Card Clock Enable"]
pub struct CCLK_ENB_R(crate::FieldReader<bool, CCLK_ENB_A>);
impl CCLK_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCLK_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCLK_ENB_A {
        match self.bits {
            false => CCLK_ENB_A::OFF,
            true => CCLK_ENB_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CCLK_ENB_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CCLK_ENB_A::ON
    }
}
impl core::ops::Deref for CCLK_ENB_R {
    type Target = crate::FieldReader<bool, CCLK_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK_ENB` writer - Card Clock Enable"]
pub struct CCLK_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Card Clock is off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CCLK_ENB_A::OFF)
    }
    #[doc = "Card Clock is on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CCLK_ENB_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CCLK_DIV` reader - Card Clock Divider"]
pub struct CCLK_DIV_R(crate::FieldReader<u8, u8>);
impl CCLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCLK_DIV` writer - Card Clock Divider"]
pub struct CCLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mask_data0(&self) -> MASK_DATA0_R {
        MASK_DATA0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Card Clock Output Control"]
    #[inline(always)]
    pub fn cclk_ctrl(&self) -> CCLK_CTRL_R {
        CCLK_CTRL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Card Clock Enable"]
    #[inline(always)]
    pub fn cclk_enb(&self) -> CCLK_ENB_R {
        CCLK_ENB_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Card Clock Divider"]
    #[inline(always)]
    pub fn cclk_div(&self) -> CCLK_DIV_R {
        CCLK_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mask_data0(&mut self) -> MASK_DATA0_W {
        MASK_DATA0_W { w: self }
    }
    #[doc = "Bit 17 - Card Clock Output Control"]
    #[inline(always)]
    pub fn cclk_ctrl(&mut self) -> CCLK_CTRL_W {
        CCLK_CTRL_W { w: self }
    }
    #[doc = "Bit 16 - Card Clock Enable"]
    #[inline(always)]
    pub fn cclk_enb(&mut self) -> CCLK_ENB_W {
        CCLK_ENB_W { w: self }
    }
    #[doc = "Bits 0:7 - Card Clock Divider"]
    #[inline(always)]
    pub fn cclk_div(&mut self) -> CCLK_DIV_W {
        CCLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_clkdiv](index.html) module"]
pub struct SMHC_CLKDIV_SPEC;
impl crate::RegisterSpec for SMHC_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_clkdiv::R](R) reader structure"]
impl crate::Readable for SMHC_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_clkdiv::W](W) writer structure"]
impl crate::Writable for SMHC_CLKDIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_CLKDIV to value 0"]
impl crate::Resettable for SMHC_CLKDIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
