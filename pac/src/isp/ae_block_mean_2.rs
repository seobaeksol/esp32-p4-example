#[doc = "Register `AE_BLOCK_MEAN_2` reader"]
pub type R = crate::R<AeBlockMean2Spec>;
#[doc = "Field `AE_B21_MEAN` reader - this field configures block21 Y mean data"]
pub type AeB21MeanR = crate::FieldReader;
#[doc = "Field `AE_B20_MEAN` reader - this field configures block20 Y mean data"]
pub type AeB20MeanR = crate::FieldReader;
#[doc = "Field `AE_B14_MEAN` reader - this field configures block14 Y mean data"]
pub type AeB14MeanR = crate::FieldReader;
#[doc = "Field `AE_B13_MEAN` reader - this field configures block13 Y mean data"]
pub type AeB13MeanR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block21 Y mean data"]
    #[inline(always)]
    pub fn ae_b21_mean(&self) -> AeB21MeanR {
        AeB21MeanR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block20 Y mean data"]
    #[inline(always)]
    pub fn ae_b20_mean(&self) -> AeB20MeanR {
        AeB20MeanR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block14 Y mean data"]
    #[inline(always)]
    pub fn ae_b14_mean(&self) -> AeB14MeanR {
        AeB14MeanR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block13 Y mean data"]
    #[inline(always)]
    pub fn ae_b13_mean(&self) -> AeB13MeanR {
        AeB13MeanR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ae statistic result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBlockMean2Spec;
impl crate::RegisterSpec for AeBlockMean2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_2::R`](R) reader structure"]
impl crate::Readable for AeBlockMean2Spec {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_2 to value 0"]
impl crate::Resettable for AeBlockMean2Spec {}
