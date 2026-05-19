#[doc = "Register `AE_BLOCK_MEAN_6` reader"]
pub type R = crate::R<AeBlockMean6Spec>;
#[doc = "Field `AE_B44_MEAN` reader - this field configures block44 Y mean data"]
pub type AeB44MeanR = crate::FieldReader;
impl R {
    #[doc = "Bits 24:31 - this field configures block44 Y mean data"]
    #[inline(always)]
    pub fn ae_b44_mean(&self) -> AeB44MeanR {
        AeB44MeanR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ae statistic result register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBlockMean6Spec;
impl crate::RegisterSpec for AeBlockMean6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_6::R`](R) reader structure"]
impl crate::Readable for AeBlockMean6Spec {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_6 to value 0"]
impl crate::Resettable for AeBlockMean6Spec {}
