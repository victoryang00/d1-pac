#[doc = "Register `RXDMA_WADDRH` reader"]
pub struct R(crate::R<RXDMA_WADDRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_WADDRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_WADDRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_WADDRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `waddr` reader - RXDMA Current Write Address \\[33:32\\]"]
pub struct WADDR_R(crate::FieldReader<u8, u8>);
impl WADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - RXDMA Current Write Address \\[33:32\\]"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new((self.bits & 0x03) as u8)
    }
}
#[doc = "UART RXDMA Write Address High Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_waddrh](index.html) module"]
pub struct RXDMA_WADDRH_SPEC;
impl crate::RegisterSpec for RXDMA_WADDRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_waddrh::R](R) reader structure"]
impl crate::Readable for RXDMA_WADDRH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDMA_WADDRH to value 0"]
impl crate::Resettable for RXDMA_WADDRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
