#[doc = "Register `ROM_CFG_REG` reader"]
pub struct R(crate::R<ROM_CFG_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_CFG_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_CFG_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_CFG_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_CFG_REG` writer"]
pub struct W(crate::W<ROM_CFG_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_CFG_REG_SPEC>;
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
impl From<crate::W<ROM_CFG_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_CFG_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROM_CFG` reader - ROM Configuration"]
pub struct ROM_CFG_R(crate::FieldReader<u8, u8>);
impl ROM_CFG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROM_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROM_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM_CFG` writer - ROM Configuration"]
pub struct ROM_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ROM Configuration"]
    #[inline(always)]
    pub fn rom_cfg(&self) -> ROM_CFG_R {
        ROM_CFG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ROM Configuration"]
    #[inline(always)]
    pub fn rom_cfg(&mut self) -> ROM_CFG_W {
        ROM_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_cfg_reg](index.html) module"]
pub struct ROM_CFG_REG_SPEC;
impl crate::RegisterSpec for ROM_CFG_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_cfg_reg::R](R) reader structure"]
impl crate::Readable for ROM_CFG_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_cfg_reg::W](W) writer structure"]
impl crate::Writable for ROM_CFG_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_CFG_REG to value 0"]
impl crate::Resettable for ROM_CFG_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
