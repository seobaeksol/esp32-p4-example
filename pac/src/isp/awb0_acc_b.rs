#[doc = "Register `AWB0_ACC_B` reader"]
pub type R = crate::R<Awb0AccBSpec>;
#[doc = "Field `AWB0_ACC_B` reader - this field represents accumulate of channel b of all white point of algo0"]
pub type Awb0AccBR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this field represents accumulate of channel b of all white point of algo0"]
    #[inline(always)]
    pub fn awb0_acc_b(&self) -> Awb0AccBR {
        Awb0AccBR::new(self.bits)
    }
}
#[doc = "result of accumulate of b channel of all white points\n\nYou can [`read`](crate::Reg::read) this register and get [`awb0_acc_b::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awb0AccBSpec;
impl crate::RegisterSpec for Awb0AccBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb0_acc_b::R`](R) reader structure"]
impl crate::Readable for Awb0AccBSpec {}
#[doc = "`reset()` method sets AWB0_ACC_B to value 0"]
impl crate::Resettable for Awb0AccBSpec {}
