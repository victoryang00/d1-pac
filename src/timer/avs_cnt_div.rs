#[doc = "Register `avs_cnt_div` reader"]
pub struct R(crate::R<AVS_CNT_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AVS_CNT_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AVS_CNT_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AVS_CNT_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `avs_cnt_div` writer"]
pub struct W(crate::W<AVS_CNT_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AVS_CNT_DIV_SPEC>;
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
impl From<crate::W<AVS_CNT_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AVS_CNT_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Fields `AVS_CNT(0-1)_D` reader - The divisor factor of AVS"]
pub struct AVS_CNT_D_R(crate::FieldReader<u16, u16>);
impl AVS_CNT_D_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        AVS_CNT_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVS_CNT_D_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fields `AVS_CNT(0-1)_D` writer - The divisor factor of AVS"]
pub struct AVS_CNT_D_W<'a> {
    w: &'a mut W,
    offset: usize,
}
impl<'a> AVS_CNT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits =
            (self.w.bits & !(0x0fff << self.offset)) | ((value as u32 & 0x0fff) << self.offset);
        self.w
    }
}
#[doc = "Fields `AVS_CNT(0-1)_D` const generic writer - The divisor factor of AVS"]
pub struct AVS_CNT_D_CGW<'a, const O: usize> {
    w: &'a mut W,
}
impl<'a, const O: usize> AVS_CNT_D_CGW<'a, O> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << O)) | ((value as u32 & 0x0fff) << O);
        self.w
    }
}
impl R {
    #[doc = "The divisor factor of AVS"]
    #[inline(always)]
    pub unsafe fn avs_cnt_d(&self, n: usize) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new(((self.bits >> (n * 16)) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt0_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt1_d(&self) -> AVS_CNT_D_R {
        AVS_CNT_D_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "The divisor factor of AVS"]
    #[inline(always)]
    pub unsafe fn avs_cnt_d(&mut self, n: usize) -> AVS_CNT_D_W {
        AVS_CNT_D_W {
            w: self,
            offset: n * 16,
        }
    }
    #[doc = "Bits 0:11 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt0_d(&mut self) -> AVS_CNT_D_CGW<0> {
        AVS_CNT_D_CGW { w: self }
    }
    #[doc = "Bits 16:27 - The divisor factor of AVS"]
    #[inline(always)]
    pub fn avs_cnt1_d(&mut self) -> AVS_CNT_D_CGW<16> {
        AVS_CNT_D_CGW { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AVS Counter Divisor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [avs_cnt_div](index.html) module"]
pub struct AVS_CNT_DIV_SPEC;
impl crate::RegisterSpec for AVS_CNT_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [avs_cnt_div::R](R) reader structure"]
impl crate::Readable for AVS_CNT_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [avs_cnt_div::W](W) writer structure"]
impl crate::Writable for AVS_CNT_DIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets avs_cnt_div to value 0"]
impl crate::Resettable for AVS_CNT_DIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
