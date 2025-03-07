#[doc = "Register `RXDMA_LMT` reader"]
pub struct R(crate::R<RXDMA_LMT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDMA_LMT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDMA_LMT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDMA_LMT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXDMA_LMT` writer"]
pub struct W(crate::W<RXDMA_LMT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXDMA_LMT_SPEC>;
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
impl From<crate::W<RXDMA_LMT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXDMA_LMT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `limit_size` reader - "]
pub struct LIMIT_SIZE_R(crate::FieldReader<u16, u16>);
impl LIMIT_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        LIMIT_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIMIT_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `limit_size` writer - "]
pub struct LIMIT_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIMIT_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn limit_size(&self) -> LIMIT_SIZE_R {
        LIMIT_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn limit_size(&mut self) -> LIMIT_SIZE_W {
        LIMIT_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART RXDMA Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdma_lmt](index.html) module"]
pub struct RXDMA_LMT_SPEC;
impl crate::RegisterSpec for RXDMA_LMT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdma_lmt::R](R) reader structure"]
impl crate::Readable for RXDMA_LMT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxdma_lmt::W](W) writer structure"]
impl crate::Writable for RXDMA_LMT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXDMA_LMT to value 0"]
impl crate::Resettable for RXDMA_LMT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
