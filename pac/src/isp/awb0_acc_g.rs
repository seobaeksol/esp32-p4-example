#[doc = "Register `AWB0_ACC_G` reader"]
pub type R = crate::R<Awb0AccGSpec>;
#[doc = "Field `AWB0_ACC_G` reader - this field represents accumulate of channel g of all white point of algo0"]
pub type Awb0AccGR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents accumulate of channel g of all white point of algo0"]
    #[inline(always)]
    pub fn awb0_acc_g(&self) -> Awb0AccGR {
        Awb0AccGR::new(self.bits)
    }
}
#[doc = "result of accumulate of g channel of all white points\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_acc_g::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awb0AccGSpec;
impl crate::RegisterSpec for Awb0AccGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_acc_g::R`](R) reader structure"]
impl crate::Readable for Awb0AccGSpec {}
#[doc = "`reset()` method sets AWB0_ACC_G to value 0"]
impl crate::Resettable for Awb0AccGSpec {}
