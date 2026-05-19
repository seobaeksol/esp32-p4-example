#[doc = "Register `DPI_COLOR_CODING_ACT` reader"]
pub type R = crate::R<DpiColorCodingActSpec>;
#[doc = "Field `DPI_COLOR_CODING_ACT` reader - NA"]
pub type DpiColorCodingActR = crate::FieldReader;
#[doc = "Field `LOOSELY18_EN_ACT` reader - NA"]
pub type Loosely18EnActR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn dpi_color_coding_act(&self) -> DpiColorCodingActR {
        DpiColorCodingActR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn loosely18_en_act(&self) -> Loosely18EnActR {
        Loosely18EnActR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_color_coding_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiColorCodingActSpec;
impl crate::RegisterSpec for DpiColorCodingActSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_color_coding_act::R`](R) reader structure"]
impl crate::Readable for DpiColorCodingActSpec {}
#[doc = "`reset()` method sets DPI_COLOR_CODING_ACT to value 0"]
impl crate::Resettable for DpiColorCodingActSpec {}
