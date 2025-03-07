#[doc = "Register `DBI_CTL_1` reader"]
pub struct R(crate::R<DBI_CTL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CTL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CTL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CTL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_CTL_1` writer"]
pub struct W(crate::W<DBI_CTL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CTL_1_SPEC>;
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
impl From<crate::W<DBI_CTL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CTL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dbi_soft_trg` reader - DBI Soft Trigger"]
pub struct DBI_SOFT_TRG_R(crate::FieldReader<bool, bool>);
impl DBI_SOFT_TRG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_SOFT_TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_SOFT_TRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_soft_trg` writer - DBI Soft Trigger"]
pub struct DBI_SOFT_TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_SOFT_TRG_W<'a> {
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
#[doc = "DBI Enable Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBI_EN_MODE_SEL_A {
    #[doc = "0: `0`"]
    DBI = 0,
    #[doc = "1: `1`"]
    SOFTWARE = 1,
    #[doc = "2: `10`"]
    TIMER = 2,
    #[doc = "3: `11`"]
    TE = 3,
}
impl From<DBI_EN_MODE_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_EN_MODE_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dbi_en_mode_sel` reader - DBI Enable Mode Select"]
pub struct DBI_EN_MODE_SEL_R(crate::FieldReader<u8, DBI_EN_MODE_SEL_A>);
impl DBI_EN_MODE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_EN_MODE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBI_EN_MODE_SEL_A {
        match self.bits {
            0 => DBI_EN_MODE_SEL_A::DBI,
            1 => DBI_EN_MODE_SEL_A::SOFTWARE,
            2 => DBI_EN_MODE_SEL_A::TIMER,
            3 => DBI_EN_MODE_SEL_A::TE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DBI`"]
    #[inline(always)]
    pub fn is_dbi(&self) -> bool {
        **self == DBI_EN_MODE_SEL_A::DBI
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        **self == DBI_EN_MODE_SEL_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        **self == DBI_EN_MODE_SEL_A::TIMER
    }
    #[doc = "Checks if the value of the field is `TE`"]
    #[inline(always)]
    pub fn is_te(&self) -> bool {
        **self == DBI_EN_MODE_SEL_A::TE
    }
}
impl core::ops::Deref for DBI_EN_MODE_SEL_R {
    type Target = crate::FieldReader<u8, DBI_EN_MODE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_en_mode_sel` writer - DBI Enable Mode Select"]
pub struct DBI_EN_MODE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_EN_MODE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_EN_MODE_SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dbi(self) -> &'a mut W {
        self.variant(DBI_EN_MODE_SEL_A::DBI)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(DBI_EN_MODE_SEL_A::SOFTWARE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(DBI_EN_MODE_SEL_A::TIMER)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn te(self) -> &'a mut W {
        self.variant(DBI_EN_MODE_SEL_A::TE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "2 Data Lane RGB666 Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGB666_FMT_A {
    #[doc = "0: `0`"]
    NORMAL = 0,
    #[doc = "1: `1`"]
    SPECIAL_ILITEK = 1,
    #[doc = "2: `10`"]
    SPECIAL_NEW_VISION = 2,
}
impl From<RGB666_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB666_FMT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RGB666_FMT` reader - 2 Data Lane RGB666 Format"]
pub struct RGB666_FMT_R(crate::FieldReader<u8, RGB666_FMT_A>);
impl RGB666_FMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RGB666_FMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGB666_FMT_A> {
        match self.bits {
            0 => Some(RGB666_FMT_A::NORMAL),
            1 => Some(RGB666_FMT_A::SPECIAL_ILITEK),
            2 => Some(RGB666_FMT_A::SPECIAL_NEW_VISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == RGB666_FMT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SPECIAL_ILITEK`"]
    #[inline(always)]
    pub fn is_special_ilitek(&self) -> bool {
        **self == RGB666_FMT_A::SPECIAL_ILITEK
    }
    #[doc = "Checks if the value of the field is `SPECIAL_NEW_VISION`"]
    #[inline(always)]
    pub fn is_special_new_vision(&self) -> bool {
        **self == RGB666_FMT_A::SPECIAL_NEW_VISION
    }
}
impl core::ops::Deref for RGB666_FMT_R {
    type Target = crate::FieldReader<u8, RGB666_FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGB666_FMT` writer - 2 Data Lane RGB666 Format"]
pub struct RGB666_FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB666_FMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB666_FMT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RGB666_FMT_A::NORMAL)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn special_ilitek(self) -> &'a mut W {
        self.variant(RGB666_FMT_A::SPECIAL_ILITEK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn special_new_vision(self) -> &'a mut W {
        self.variant(RGB666_FMT_A::SPECIAL_NEW_VISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "DBI RX Clock Inverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBI_RXCLK_INV_A {
    #[doc = "0: `0`"]
    POSITIVE = 0,
    #[doc = "1: `1`"]
    NEGATIVE = 1,
}
impl From<DBI_RXCLK_INV_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_RXCLK_INV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dbi_rxclk_inv` reader - DBI RX Clock Inverse"]
pub struct DBI_RXCLK_INV_R(crate::FieldReader<bool, DBI_RXCLK_INV_A>);
impl DBI_RXCLK_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_RXCLK_INV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBI_RXCLK_INV_A {
        match self.bits {
            false => DBI_RXCLK_INV_A::POSITIVE,
            true => DBI_RXCLK_INV_A::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        **self == DBI_RXCLK_INV_A::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        **self == DBI_RXCLK_INV_A::NEGATIVE
    }
}
impl core::ops::Deref for DBI_RXCLK_INV_R {
    type Target = crate::FieldReader<bool, DBI_RXCLK_INV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_rxclk_inv` writer - DBI RX Clock Inverse"]
pub struct DBI_RXCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_RXCLK_INV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_RXCLK_INV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(DBI_RXCLK_INV_A::POSITIVE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(DBI_RXCLK_INV_A::NEGATIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "DBI Output Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBI_CLKO_MOD_A {
    #[doc = "0: `0`"]
    ALWAYS_ON = 0,
    #[doc = "1: `1`"]
    AUTO_GATING = 1,
}
impl From<DBI_CLKO_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_CLKO_MOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dbi_clko_mod` reader - DBI Output Clock Mode"]
pub struct DBI_CLKO_MOD_R(crate::FieldReader<bool, DBI_CLKO_MOD_A>);
impl DBI_CLKO_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_CLKO_MOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBI_CLKO_MOD_A {
        match self.bits {
            false => DBI_CLKO_MOD_A::ALWAYS_ON,
            true => DBI_CLKO_MOD_A::AUTO_GATING,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        **self == DBI_CLKO_MOD_A::ALWAYS_ON
    }
    #[doc = "Checks if the value of the field is `AUTO_GATING`"]
    #[inline(always)]
    pub fn is_auto_gating(&self) -> bool {
        **self == DBI_CLKO_MOD_A::AUTO_GATING
    }
}
impl core::ops::Deref for DBI_CLKO_MOD_R {
    type Target = crate::FieldReader<bool, DBI_CLKO_MOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_clko_mod` writer - DBI Output Clock Mode"]
pub struct DBI_CLKO_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_CLKO_MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_CLKO_MOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(DBI_CLKO_MOD_A::ALWAYS_ON)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn auto_gating(self) -> &'a mut W {
        self.variant(DBI_CLKO_MOD_A::AUTO_GATING)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `dbi_clko_inv` reader - DBI Clock Output Inverse"]
pub struct DBI_CLKO_INV_R(crate::FieldReader<bool, bool>);
impl DBI_CLKO_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_CLKO_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_CLKO_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_clko_inv` writer - DBI Clock Output Inverse"]
pub struct DBI_CLKO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_CLKO_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `dcx_data` reader - DCX Data Value"]
pub struct DCX_DATA_R(crate::FieldReader<bool, bool>);
impl DCX_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCX_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCX_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcx_data` writer - DCX Data Value"]
pub struct DCX_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCX_DATA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `RGB16_data_source_select` reader - RGB 16 Data Source Select"]
pub struct RGB16_DATA_SOURCE_SELECT_R(crate::FieldReader<bool, bool>);
impl RGB16_DATA_SOURCE_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGB16_DATA_SOURCE_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGB16_DATA_SOURCE_SELECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGB16_data_source_select` writer - RGB 16 Data Source Select"]
pub struct RGB16_DATA_SOURCE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB16_DATA_SOURCE_SELECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `rdat_lsb` reader - Bit Order of Read Data"]
pub struct RDAT_LSB_R(crate::FieldReader<bool, bool>);
impl RDAT_LSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDAT_LSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDAT_LSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rdat_lsb` writer - Bit Order of Read Data"]
pub struct RDAT_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> RDAT_LSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `rcdc` reader - Read Command Dummy Cycles"]
pub struct RCDC_R(crate::FieldReader<u8, u8>);
impl RCDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RCDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCDC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rcdc` writer - Read Command Dummy Cycles"]
pub struct RCDC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `rdbn` reader - Read Data Number of Bytes"]
pub struct RDBN_R(crate::FieldReader<u8, u8>);
impl RDBN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RDBN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDBN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rdbn` writer - Read Data Number of Bytes"]
pub struct RDBN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDBN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - DBI Soft Trigger"]
    #[inline(always)]
    pub fn dbi_soft_trg(&self) -> DBI_SOFT_TRG_R {
        DBI_SOFT_TRG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - DBI Enable Mode Select"]
    #[inline(always)]
    pub fn dbi_en_mode_sel(&self) -> DBI_EN_MODE_SEL_R {
        DBI_EN_MODE_SEL_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - 2 Data Lane RGB666 Format"]
    #[inline(always)]
    pub fn rgb666_fmt(&self) -> RGB666_FMT_R {
        RGB666_FMT_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 25 - DBI RX Clock Inverse"]
    #[inline(always)]
    pub fn dbi_rxclk_inv(&self) -> DBI_RXCLK_INV_R {
        DBI_RXCLK_INV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DBI Output Clock Mode"]
    #[inline(always)]
    pub fn dbi_clko_mod(&self) -> DBI_CLKO_MOD_R {
        DBI_CLKO_MOD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DBI Clock Output Inverse"]
    #[inline(always)]
    pub fn dbi_clko_inv(&self) -> DBI_CLKO_INV_R {
        DBI_CLKO_INV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - DCX Data Value"]
    #[inline(always)]
    pub fn dcx_data(&self) -> DCX_DATA_R {
        DCX_DATA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RGB 16 Data Source Select"]
    #[inline(always)]
    pub fn rgb16_data_source_select(&self) -> RGB16_DATA_SOURCE_SELECT_R {
        RGB16_DATA_SOURCE_SELECT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit Order of Read Data"]
    #[inline(always)]
    pub fn rdat_lsb(&self) -> RDAT_LSB_R {
        RDAT_LSB_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Read Command Dummy Cycles"]
    #[inline(always)]
    pub fn rcdc(&self) -> RCDC_R {
        RCDC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Read Data Number of Bytes"]
    #[inline(always)]
    pub fn rdbn(&self) -> RDBN_R {
        RDBN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - DBI Soft Trigger"]
    #[inline(always)]
    pub fn dbi_soft_trg(&mut self) -> DBI_SOFT_TRG_W {
        DBI_SOFT_TRG_W { w: self }
    }
    #[doc = "Bits 29:30 - DBI Enable Mode Select"]
    #[inline(always)]
    pub fn dbi_en_mode_sel(&mut self) -> DBI_EN_MODE_SEL_W {
        DBI_EN_MODE_SEL_W { w: self }
    }
    #[doc = "Bits 26:27 - 2 Data Lane RGB666 Format"]
    #[inline(always)]
    pub fn rgb666_fmt(&mut self) -> RGB666_FMT_W {
        RGB666_FMT_W { w: self }
    }
    #[doc = "Bit 25 - DBI RX Clock Inverse"]
    #[inline(always)]
    pub fn dbi_rxclk_inv(&mut self) -> DBI_RXCLK_INV_W {
        DBI_RXCLK_INV_W { w: self }
    }
    #[doc = "Bit 24 - DBI Output Clock Mode"]
    #[inline(always)]
    pub fn dbi_clko_mod(&mut self) -> DBI_CLKO_MOD_W {
        DBI_CLKO_MOD_W { w: self }
    }
    #[doc = "Bit 23 - DBI Clock Output Inverse"]
    #[inline(always)]
    pub fn dbi_clko_inv(&mut self) -> DBI_CLKO_INV_W {
        DBI_CLKO_INV_W { w: self }
    }
    #[doc = "Bit 22 - DCX Data Value"]
    #[inline(always)]
    pub fn dcx_data(&mut self) -> DCX_DATA_W {
        DCX_DATA_W { w: self }
    }
    #[doc = "Bit 21 - RGB 16 Data Source Select"]
    #[inline(always)]
    pub fn rgb16_data_source_select(&mut self) -> RGB16_DATA_SOURCE_SELECT_W {
        RGB16_DATA_SOURCE_SELECT_W { w: self }
    }
    #[doc = "Bit 20 - Bit Order of Read Data"]
    #[inline(always)]
    pub fn rdat_lsb(&mut self) -> RDAT_LSB_W {
        RDAT_LSB_W { w: self }
    }
    #[doc = "Bits 8:15 - Read Command Dummy Cycles"]
    #[inline(always)]
    pub fn rcdc(&mut self) -> RCDC_W {
        RCDC_W { w: self }
    }
    #[doc = "Bits 0:7 - Read Data Number of Bytes"]
    #[inline(always)]
    pub fn rdbn(&mut self) -> RDBN_W {
        RDBN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_ctl_1](index.html) module"]
pub struct DBI_CTL_1_SPEC;
impl crate::RegisterSpec for DBI_CTL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_ctl_1::R](R) reader structure"]
impl crate::Readable for DBI_CTL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_ctl_1::W](W) writer structure"]
impl crate::Writable for DBI_CTL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_CTL_1 to value 0"]
impl crate::Resettable for DBI_CTL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
