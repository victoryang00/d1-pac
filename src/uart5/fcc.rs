#[doc = "Register `FCC` reader"]
pub struct R(crate::R<FCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCC` writer"]
pub struct W(crate::W<FCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCC_SPEC>;
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
impl From<crate::W<FCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fifo_depth` reader - "]
pub struct FIFO_DEPTH_R(crate::FieldReader<u32, u32>);
impl FIFO_DEPTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FIFO_DEPTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_DEPTH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_CLOCK_MODE_A {
    #[doc = "0: Sync mode, writing/reading clocks use apb clock"]
    WR_APB = 0,
    #[doc = "1: Sync mode, writing clock uses apb clock, reading clock uses ahb clock"]
    W_APB_R_AHB = 1,
}
impl From<RX_FIFO_CLOCK_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLOCK_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_fifo_clock_mode` reader - "]
pub struct RX_FIFO_CLOCK_MODE_R(crate::FieldReader<bool, RX_FIFO_CLOCK_MODE_A>);
impl RX_FIFO_CLOCK_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_CLOCK_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_CLOCK_MODE_A {
        match self.bits {
            false => RX_FIFO_CLOCK_MODE_A::WR_APB,
            true => RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB,
        }
    }
    #[doc = "Checks if the value of the field is `WR_APB`"]
    #[inline(always)]
    pub fn is_wr_apb(&self) -> bool {
        **self == RX_FIFO_CLOCK_MODE_A::WR_APB
    }
    #[doc = "Checks if the value of the field is `W_APB_R_AHB`"]
    #[inline(always)]
    pub fn is_w_apb_r_ahb(&self) -> bool {
        **self == RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB
    }
}
impl core::ops::Deref for RX_FIFO_CLOCK_MODE_R {
    type Target = crate::FieldReader<bool, RX_FIFO_CLOCK_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_clock_mode` writer - "]
pub struct RX_FIFO_CLOCK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLOCK_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_CLOCK_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sync mode, writing/reading clocks use apb clock"]
    #[inline(always)]
    pub fn wr_apb(self) -> &'a mut W {
        self.variant(RX_FIFO_CLOCK_MODE_A::WR_APB)
    }
    #[doc = "Sync mode, writing clock uses apb clock, reading clock uses ahb clock"]
    #[inline(always)]
    pub fn w_apb_r_ahb(self) -> &'a mut W {
        self.variant(RX_FIFO_CLOCK_MODE_A::W_APB_R_AHB)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_CLOCK_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TX_FIFO_CLOCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_CLOCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_fifo_clock_enable` reader - "]
pub struct TX_FIFO_CLOCK_ENABLE_R(crate::FieldReader<bool, TX_FIFO_CLOCK_ENABLE_A>);
impl TX_FIFO_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_CLOCK_ENABLE_A {
        match self.bits {
            false => TX_FIFO_CLOCK_ENABLE_A::DISABLE,
            true => TX_FIFO_CLOCK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TX_FIFO_CLOCK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == TX_FIFO_CLOCK_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for TX_FIFO_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, TX_FIFO_CLOCK_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_clock_enable` writer - "]
pub struct TX_FIFO_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLOCK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_CLOCK_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TX_FIFO_CLOCK_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TX_FIFO_CLOCK_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_CLOCK_ENABLE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<RX_FIFO_CLOCK_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_CLOCK_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_fifo_clock_enable` reader - "]
pub struct RX_FIFO_CLOCK_ENABLE_R(crate::FieldReader<bool, RX_FIFO_CLOCK_ENABLE_A>);
impl RX_FIFO_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_CLOCK_ENABLE_A {
        match self.bits {
            false => RX_FIFO_CLOCK_ENABLE_A::DISABLE,
            true => RX_FIFO_CLOCK_ENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RX_FIFO_CLOCK_ENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RX_FIFO_CLOCK_ENABLE_A::ENABLE
    }
}
impl core::ops::Deref for RX_FIFO_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, RX_FIFO_CLOCK_ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_clock_enable` writer - "]
pub struct RX_FIFO_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLOCK_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_CLOCK_ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RX_FIFO_CLOCK_ENABLE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RX_FIFO_CLOCK_ENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn fifo_depth(&self) -> FIFO_DEPTH_R {
        FIFO_DEPTH_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_clock_mode(&self) -> RX_FIFO_CLOCK_MODE_R {
        RX_FIFO_CLOCK_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_clock_enable(&self) -> TX_FIFO_CLOCK_ENABLE_R {
        TX_FIFO_CLOCK_ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clock_enable(&self) -> RX_FIFO_CLOCK_ENABLE_R {
        RX_FIFO_CLOCK_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_fifo_clock_mode(&mut self) -> RX_FIFO_CLOCK_MODE_W {
        RX_FIFO_CLOCK_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_clock_enable(&mut self) -> TX_FIFO_CLOCK_ENABLE_W {
        TX_FIFO_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_fifo_clock_enable(&mut self) -> RX_FIFO_CLOCK_ENABLE_W {
        RX_FIFO_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcc](index.html) module"]
pub struct FCC_SPEC;
impl crate::RegisterSpec for FCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcc::R](R) reader structure"]
impl crate::Readable for FCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcc::W](W) writer structure"]
impl crate::Writable for FCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCC to value 0"]
impl crate::Resettable for FCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
