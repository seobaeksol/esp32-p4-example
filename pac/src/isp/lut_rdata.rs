#[doc = "Register `LUT_RDATA` reader"]
pub type R = crate::R<LutRdataSpec>;
#[doc = "Field `LUT_RDATA` reader - this field represents the read data of lut. read ISP_LUT_RDATA after write ISP_LUT_CMD register"]
pub type LutRdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents the read data of lut. read ISP_LUT_RDATA after write ISP_LUT_CMD register"]
    #[inline(always)]
    pub fn lut_rdata(&self) -> LutRdataR {
        LutRdataR::new(self.bits)
    }
}
#[doc = "LUT read data register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_rdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutRdataSpec;
impl crate::RegisterSpec for LutRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lut_rdata::R`](R) reader structure"]
impl crate::Readable for LutRdataSpec {}
#[doc = "`reset()` method sets LUT_RDATA to value 0"]
impl crate::Resettable for LutRdataSpec {}
