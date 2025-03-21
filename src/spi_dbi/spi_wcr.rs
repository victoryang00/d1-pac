#[doc = "Register `SPI_WCR` reader"]
pub struct R(crate::R<SPI_WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_WCR` writer"]
pub struct W(crate::W<SPI_WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_WCR_SPEC>;
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
impl From<crate::W<SPI_WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swc` reader - Dual mode direction switch wait clock counter"]
pub struct SWC_R(crate::FieldReader<u8, u8>);
impl SWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swc` writer - Dual mode direction switch wait clock counter"]
pub struct SWC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `wwc` reader - Wait clock counter"]
pub struct WWC_R(crate::FieldReader<u16, u16>);
impl WWC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WWC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wwc` writer - Wait clock counter"]
pub struct WWC_W<'a> {
    w: &'a mut W,
}
impl<'a> WWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    pub fn swc(&self) -> SWC_R {
        SWC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    pub fn wwc(&self) -> WWC_R {
        WWC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19 - Dual mode direction switch wait clock counter"]
    #[inline(always)]
    pub fn swc(&mut self) -> SWC_W {
        SWC_W { w: self }
    }
    #[doc = "Bits 0:15 - Wait clock counter"]
    #[inline(always)]
    pub fn wwc(&mut self) -> WWC_W {
        WWC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Wait Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_wcr](index.html) module"]
pub struct SPI_WCR_SPEC;
impl crate::RegisterSpec for SPI_WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_wcr::R](R) reader structure"]
impl crate::Readable for SPI_WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_wcr::W](W) writer structure"]
impl crate::Writable for SPI_WCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_WCR to value 0"]
impl crate::Resettable for SPI_WCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
