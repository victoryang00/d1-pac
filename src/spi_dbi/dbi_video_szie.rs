#[doc = "Register `DBI_VIDEO_SZIE` reader"]
pub struct R(crate::R<DBI_VIDEO_SZIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_VIDEO_SZIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_VIDEO_SZIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_VIDEO_SZIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_VIDEO_SZIE` writer"]
pub struct W(crate::W<DBI_VIDEO_SZIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_VIDEO_SZIE_SPEC>;
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
impl From<crate::W<DBI_VIDEO_SZIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_VIDEO_SZIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v_size` reader - "]
pub struct V_SIZE_R(crate::FieldReader<u16, u16>);
impl V_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        V_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `v_size` writer - "]
pub struct V_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> V_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
#[doc = "Field `h_size` reader - "]
pub struct H_SIZE_R(crate::FieldReader<u16, u16>);
impl H_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        H_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `h_size` writer - "]
pub struct H_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> H_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn v_size(&self) -> V_SIZE_R {
        V_SIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_size(&self) -> H_SIZE_R {
        H_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:26"]
    #[inline(always)]
    pub fn v_size(&mut self) -> V_SIZE_W {
        V_SIZE_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_size(&mut self) -> H_SIZE_W {
        H_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Video Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_video_szie](index.html) module"]
pub struct DBI_VIDEO_SZIE_SPEC;
impl crate::RegisterSpec for DBI_VIDEO_SZIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_video_szie::R](R) reader structure"]
impl crate::Readable for DBI_VIDEO_SZIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_video_szie::W](W) writer structure"]
impl crate::Writable for DBI_VIDEO_SZIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_VIDEO_SZIE to value 0"]
impl crate::Resettable for DBI_VIDEO_SZIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
