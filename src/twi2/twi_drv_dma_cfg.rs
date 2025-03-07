#[doc = "Register `TWI_DRV_DMA_CFG` reader"]
pub struct R(crate::R<TWI_DRV_DMA_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWI_DRV_DMA_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWI_DRV_DMA_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWI_DRV_DMA_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWI_DRV_DMA_CFG` writer"]
pub struct W(crate::W<TWI_DRV_DMA_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWI_DRV_DMA_CFG_SPEC>;
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
impl From<crate::W<TWI_DRV_DMA_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWI_DRV_DMA_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_rx_en` reader - "]
pub struct DMA_RX_EN_R(crate::FieldReader<u8, u8>);
impl DMA_RX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMA_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RX_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_rx_en` writer - "]
pub struct DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RX_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `rx_trig` reader - "]
pub struct RX_TRIG_R(crate::FieldReader<u8, u8>);
impl RX_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_trig` writer - "]
pub struct RX_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `dma_tx_en` reader - "]
pub struct DMA_TX_EN_R(crate::FieldReader<bool, bool>);
impl DMA_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_tx_en` writer - "]
pub struct DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_TX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `tx_trig` reader - "]
pub struct TX_TRIG_R(crate::FieldReader<u8, u8>);
impl TX_TRIG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TRIG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_trig` writer - "]
pub struct TX_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rx_trig(&self) -> RX_TRIG_R {
        RX_TRIG_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TX_TRIG_R {
        TX_TRIG_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn dma_rx_en(&mut self) -> DMA_RX_EN_W {
        DMA_RX_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rx_trig(&mut self) -> RX_TRIG_W {
        RX_TRIG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dma_tx_en(&mut self) -> DMA_TX_EN_W {
        DMA_TX_EN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_trig(&mut self) -> TX_TRIG_W {
        TX_TRIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI_DRV DMA Configure Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twi_drv_dma_cfg](index.html) module"]
pub struct TWI_DRV_DMA_CFG_SPEC;
impl crate::RegisterSpec for TWI_DRV_DMA_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twi_drv_dma_cfg::R](R) reader structure"]
impl crate::Readable for TWI_DRV_DMA_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twi_drv_dma_cfg::W](W) writer structure"]
impl crate::Writable for TWI_DRV_DMA_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TWI_DRV_DMA_CFG to value 0"]
impl crate::Resettable for TWI_DRV_DMA_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
