#[doc = "Register `AWB0_WHITE_CNT` reader"]
pub type R = crate::R<Awb0WhiteCntSpec>;
#[doc = "Field `AWB0_WHITE_CNT` reader - this field configures number of white point detected of algo0"]
pub type Awb0WhiteCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - this field configures number of white point detected of algo0"]
    #[inline(always)]
    pub fn awb0_white_cnt(&self) -> Awb0WhiteCntR {
        Awb0WhiteCntR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "result of awb white point number\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_white_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awb0WhiteCntSpec;
impl crate::RegisterSpec for Awb0WhiteCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_white_cnt::R`](R) reader structure"]
impl crate::Readable for Awb0WhiteCntSpec {}
#[doc = "`reset()` method sets AWB0_WHITE_CNT to value 0"]
impl crate::Resettable for Awb0WhiteCntSpec {}
