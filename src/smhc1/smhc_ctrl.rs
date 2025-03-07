#[doc = "Register `SMHC_CTRL` reader"]
pub struct R(crate::R<SMHC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_CTRL` writer"]
pub struct W(crate::W<SMHC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_CTRL_SPEC>;
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
impl From<crate::W<SMHC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO Accesss Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_AC_MOD_A {
    #[doc = "0: DMA bus"]
    DMA = 0,
    #[doc = "1: AHB bus"]
    AHB = 1,
}
impl From<FIFO_AC_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_AC_MOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_AC_MOD` reader - FIFO Accesss Mode"]
pub struct FIFO_AC_MOD_R(crate::FieldReader<bool, FIFO_AC_MOD_A>);
impl FIFO_AC_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_AC_MOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_AC_MOD_A {
        match self.bits {
            false => FIFO_AC_MOD_A::DMA,
            true => FIFO_AC_MOD_A::AHB,
        }
    }
    #[doc = "Checks if the value of the field is `DMA`"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        **self == FIFO_AC_MOD_A::DMA
    }
    #[doc = "Checks if the value of the field is `AHB`"]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        **self == FIFO_AC_MOD_A::AHB
    }
}
impl core::ops::Deref for FIFO_AC_MOD_R {
    type Target = crate::FieldReader<bool, FIFO_AC_MOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_AC_MOD` writer - FIFO Accesss Mode"]
pub struct FIFO_AC_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_AC_MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_AC_MOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DMA bus"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut W {
        self.variant(FIFO_AC_MOD_A::DMA)
    }
    #[doc = "AHB bus"]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut W {
        self.variant(FIFO_AC_MOD_A::AHB)
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
#[doc = "Time unit for command line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIME_UNIT_CMD_A {
    #[doc = "0: 1 card clock period"]
    C1 = 0,
    #[doc = "1: 256 card clock period"]
    C256 = 1,
}
impl From<TIME_UNIT_CMD_A> for bool {
    #[inline(always)]
    fn from(variant: TIME_UNIT_CMD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIME_UNIT_CMD` reader - Time unit for command line"]
pub struct TIME_UNIT_CMD_R(crate::FieldReader<bool, TIME_UNIT_CMD_A>);
impl TIME_UNIT_CMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_UNIT_CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIME_UNIT_CMD_A {
        match self.bits {
            false => TIME_UNIT_CMD_A::C1,
            true => TIME_UNIT_CMD_A::C256,
        }
    }
    #[doc = "Checks if the value of the field is `C1`"]
    #[inline(always)]
    pub fn is_c1(&self) -> bool {
        **self == TIME_UNIT_CMD_A::C1
    }
    #[doc = "Checks if the value of the field is `C256`"]
    #[inline(always)]
    pub fn is_c256(&self) -> bool {
        **self == TIME_UNIT_CMD_A::C256
    }
}
impl core::ops::Deref for TIME_UNIT_CMD_R {
    type Target = crate::FieldReader<bool, TIME_UNIT_CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_UNIT_CMD` writer - Time unit for command line"]
pub struct TIME_UNIT_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_UNIT_CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIME_UNIT_CMD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 card clock period"]
    #[inline(always)]
    pub fn c1(self) -> &'a mut W {
        self.variant(TIME_UNIT_CMD_A::C1)
    }
    #[doc = "256 card clock period"]
    #[inline(always)]
    pub fn c256(self) -> &'a mut W {
        self.variant(TIME_UNIT_CMD_A::C256)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Time unit for data line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIME_UNIT_DAT_A {
    #[doc = "0: 1 card clock period"]
    C1 = 0,
    #[doc = "1: 256 card clock period"]
    C256 = 1,
}
impl From<TIME_UNIT_DAT_A> for bool {
    #[inline(always)]
    fn from(variant: TIME_UNIT_DAT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIME_UNIT_DAT` reader - Time unit for data line"]
pub struct TIME_UNIT_DAT_R(crate::FieldReader<bool, TIME_UNIT_DAT_A>);
impl TIME_UNIT_DAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIME_UNIT_DAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIME_UNIT_DAT_A {
        match self.bits {
            false => TIME_UNIT_DAT_A::C1,
            true => TIME_UNIT_DAT_A::C256,
        }
    }
    #[doc = "Checks if the value of the field is `C1`"]
    #[inline(always)]
    pub fn is_c1(&self) -> bool {
        **self == TIME_UNIT_DAT_A::C1
    }
    #[doc = "Checks if the value of the field is `C256`"]
    #[inline(always)]
    pub fn is_c256(&self) -> bool {
        **self == TIME_UNIT_DAT_A::C256
    }
}
impl core::ops::Deref for TIME_UNIT_DAT_R {
    type Target = crate::FieldReader<bool, TIME_UNIT_DAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIME_UNIT_DAT` writer - Time unit for data line"]
pub struct TIME_UNIT_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIME_UNIT_DAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIME_UNIT_DAT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1 card clock period"]
    #[inline(always)]
    pub fn c1(self) -> &'a mut W {
        self.variant(TIME_UNIT_DAT_A::C1)
    }
    #[doc = "256 card clock period"]
    #[inline(always)]
    pub fn c256(self) -> &'a mut W {
        self.variant(TIME_UNIT_DAT_A::C256)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "DDR Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR_MOD_SEL_A {
    #[doc = "0: SDR mode"]
    SDR = 0,
    #[doc = "1: DDR mode"]
    DDR = 1,
}
impl From<DDR_MOD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DDR_MOD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DDR_MOD_SEL` reader - DDR Mode Select"]
pub struct DDR_MOD_SEL_R(crate::FieldReader<bool, DDR_MOD_SEL_A>);
impl DDR_MOD_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DDR_MOD_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR_MOD_SEL_A {
        match self.bits {
            false => DDR_MOD_SEL_A::SDR,
            true => DDR_MOD_SEL_A::DDR,
        }
    }
    #[doc = "Checks if the value of the field is `SDR`"]
    #[inline(always)]
    pub fn is_sdr(&self) -> bool {
        **self == DDR_MOD_SEL_A::SDR
    }
    #[doc = "Checks if the value of the field is `DDR`"]
    #[inline(always)]
    pub fn is_ddr(&self) -> bool {
        **self == DDR_MOD_SEL_A::DDR
    }
}
impl core::ops::Deref for DDR_MOD_SEL_R {
    type Target = crate::FieldReader<bool, DDR_MOD_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_MOD_SEL` writer - DDR Mode Select"]
pub struct DDR_MOD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_MOD_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DDR_MOD_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SDR mode"]
    #[inline(always)]
    pub fn sdr(self) -> &'a mut W {
        self.variant(DDR_MOD_SEL_A::SDR)
    }
    #[doc = "DDR mode"]
    #[inline(always)]
    pub fn ddr(self) -> &'a mut W {
        self.variant(DDR_MOD_SEL_A::DDR)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Card Detect (Data\\[3\\]
status) De-bounce Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CD_DBC_ENB_A {
    #[doc = "0: Disable de-bounce"]
    DISABLE = 0,
    #[doc = "1: Enable de-bounce"]
    ENABLE = 1,
}
impl From<CD_DBC_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: CD_DBC_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CD_DBC_ENB` reader - Card Detect (Data\\[3\\]
status) De-bounce Enable"]
pub struct CD_DBC_ENB_R(crate::FieldReader<bool, CD_DBC_ENB_A>);
impl CD_DBC_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CD_DBC_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CD_DBC_ENB_A {
        match self.bits {
            false => CD_DBC_ENB_A::DISABLE,
            true => CD_DBC_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CD_DBC_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CD_DBC_ENB_A::ENABLE
    }
}
impl core::ops::Deref for CD_DBC_ENB_R {
    type Target = crate::FieldReader<bool, CD_DBC_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CD_DBC_ENB` writer - Card Detect (Data\\[3\\]
status) De-bounce Enable"]
pub struct CD_DBC_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_DBC_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CD_DBC_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable de-bounce"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CD_DBC_ENB_A::DISABLE)
    }
    #[doc = "Enable de-bounce"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CD_DBC_ENB_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "DMA Global Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_ENB_A {
    #[doc = "0: Disable DMA to transfer data via AHB bus"]
    DISABLE = 0,
    #[doc = "1: Enable DMA to transfer data"]
    ENABLE = 1,
}
impl From<DMA_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_ENB` reader - DMA Global Enable"]
pub struct DMA_ENB_R(crate::FieldReader<bool, DMA_ENB_A>);
impl DMA_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_ENB_A {
        match self.bits {
            false => DMA_ENB_A::DISABLE,
            true => DMA_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMA_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMA_ENB_A::ENABLE
    }
}
impl core::ops::Deref for DMA_ENB_R {
    type Target = crate::FieldReader<bool, DMA_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_ENB` writer - DMA Global Enable"]
pub struct DMA_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable DMA to transfer data via AHB bus"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_ENB_A::DISABLE)
    }
    #[doc = "Enable DMA to transfer data"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_ENB_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "GLobal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INE_ENB_A {
    #[doc = "0: Disable interrupts"]
    DISABLE = 0,
    #[doc = "1: Enable interrupts"]
    ENABLE = 1,
}
impl From<INE_ENB_A> for bool {
    #[inline(always)]
    fn from(variant: INE_ENB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INE_ENB` reader - GLobal Interrupt Enable"]
pub struct INE_ENB_R(crate::FieldReader<bool, INE_ENB_A>);
impl INE_ENB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INE_ENB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INE_ENB_A {
        match self.bits {
            false => INE_ENB_A::DISABLE,
            true => INE_ENB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == INE_ENB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == INE_ENB_A::ENABLE
    }
}
impl core::ops::Deref for INE_ENB_R {
    type Target = crate::FieldReader<bool, INE_ENB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INE_ENB` writer - GLobal Interrupt Enable"]
pub struct INE_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> INE_ENB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INE_ENB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupts"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INE_ENB_A::DISABLE)
    }
    #[doc = "Enable interrupts"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INE_ENB_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DMA_RST` reader - DMA Reset"]
pub struct DMA_RST_R(crate::FieldReader<bool, bool>);
impl DMA_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_RST` writer - DMA Reset"]
pub struct DMA_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_RST_W<'a> {
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
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_RST_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<FIFO_RST_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_RST` reader - FIFO Reset"]
pub struct FIFO_RST_R(crate::FieldReader<bool, FIFO_RST_A>);
impl FIFO_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_RST_A {
        match self.bits {
            false => FIFO_RST_A::NO_EFFECT,
            true => FIFO_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == FIFO_RST_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == FIFO_RST_A::RESET
    }
}
impl core::ops::Deref for FIFO_RST_R {
    type Target = crate::FieldReader<bool, FIFO_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_RST` writer - FIFO Reset"]
pub struct FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(FIFO_RST_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FIFO_RST_A::RESET)
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
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFT_RST_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RESET = 1,
}
impl From<SOFT_RST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFT_RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFT_RST` reader - Software Reset"]
pub struct SOFT_RST_R(crate::FieldReader<bool, SOFT_RST_A>);
impl SOFT_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_RST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFT_RST_A {
        match self.bits {
            false => SOFT_RST_A::NO_EFFECT,
            true => SOFT_RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == SOFT_RST_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == SOFT_RST_A::RESET
    }
}
impl core::ops::Deref for SOFT_RST_R {
    type Target = crate::FieldReader<bool, SOFT_RST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_RST` writer - Software Reset"]
pub struct SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFT_RST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SOFT_RST_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SOFT_RST_A::RESET)
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
    #[doc = "Bit 31 - FIFO Accesss Mode"]
    #[inline(always)]
    pub fn fifo_ac_mod(&self) -> FIFO_AC_MOD_R {
        FIFO_AC_MOD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Time unit for command line"]
    #[inline(always)]
    pub fn time_unit_cmd(&self) -> TIME_UNIT_CMD_R {
        TIME_UNIT_CMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Time unit for data line"]
    #[inline(always)]
    pub fn time_unit_dat(&self) -> TIME_UNIT_DAT_R {
        TIME_UNIT_DAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DDR Mode Select"]
    #[inline(always)]
    pub fn ddr_mod_sel(&self) -> DDR_MOD_SEL_R {
        DDR_MOD_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Card Detect (Data\\[3\\]
status) De-bounce Enable"]
    #[inline(always)]
    pub fn cd_dbc_enb(&self) -> CD_DBC_ENB_R {
        CD_DBC_ENB_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Global Enable"]
    #[inline(always)]
    pub fn dma_enb(&self) -> DMA_ENB_R {
        DMA_ENB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GLobal Interrupt Enable"]
    #[inline(always)]
    pub fn ine_enb(&self) -> INE_ENB_R {
        INE_ENB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Reset"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    pub fn fifo_rst(&self) -> FIFO_RST_R {
        FIFO_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn soft_rst(&self) -> SOFT_RST_R {
        SOFT_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - FIFO Accesss Mode"]
    #[inline(always)]
    pub fn fifo_ac_mod(&mut self) -> FIFO_AC_MOD_W {
        FIFO_AC_MOD_W { w: self }
    }
    #[doc = "Bit 12 - Time unit for command line"]
    #[inline(always)]
    pub fn time_unit_cmd(&mut self) -> TIME_UNIT_CMD_W {
        TIME_UNIT_CMD_W { w: self }
    }
    #[doc = "Bit 11 - Time unit for data line"]
    #[inline(always)]
    pub fn time_unit_dat(&mut self) -> TIME_UNIT_DAT_W {
        TIME_UNIT_DAT_W { w: self }
    }
    #[doc = "Bit 10 - DDR Mode Select"]
    #[inline(always)]
    pub fn ddr_mod_sel(&mut self) -> DDR_MOD_SEL_W {
        DDR_MOD_SEL_W { w: self }
    }
    #[doc = "Bit 8 - Card Detect (Data\\[3\\]
status) De-bounce Enable"]
    #[inline(always)]
    pub fn cd_dbc_enb(&mut self) -> CD_DBC_ENB_W {
        CD_DBC_ENB_W { w: self }
    }
    #[doc = "Bit 5 - DMA Global Enable"]
    #[inline(always)]
    pub fn dma_enb(&mut self) -> DMA_ENB_W {
        DMA_ENB_W { w: self }
    }
    #[doc = "Bit 4 - GLobal Interrupt Enable"]
    #[inline(always)]
    pub fn ine_enb(&mut self) -> INE_ENB_W {
        INE_ENB_W { w: self }
    }
    #[doc = "Bit 2 - DMA Reset"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W {
        DMA_RST_W { w: self }
    }
    #[doc = "Bit 1 - FIFO Reset"]
    #[inline(always)]
    pub fn fifo_rst(&mut self) -> FIFO_RST_W {
        FIFO_RST_W { w: self }
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn soft_rst(&mut self) -> SOFT_RST_W {
        SOFT_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ctrl](index.html) module"]
pub struct SMHC_CTRL_SPEC;
impl crate::RegisterSpec for SMHC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ctrl::R](R) reader structure"]
impl crate::Readable for SMHC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ctrl::W](W) writer structure"]
impl crate::Writable for SMHC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_CTRL to value 0"]
impl crate::Resettable for SMHC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
