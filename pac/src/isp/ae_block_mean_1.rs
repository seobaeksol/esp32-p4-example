#[doc = "Register `AE_BLOCK_MEAN_1` reader"]
pub type R = crate::R<AeBlockMean1Spec>;
#[doc = "Field `AE_B12_MEAN` reader - this field configures block12 Y mean data"]
pub type AeB12MeanR = crate::FieldReader;
#[doc = "Field `AE_B11_MEAN` reader - this field configures block11 Y mean data"]
pub type AeB11MeanR = crate::FieldReader;
#[doc = "Field `AE_B10_MEAN` reader - this field configures block10 Y mean data"]
pub type AeB10MeanR = crate::FieldReader;
#[doc = "Field `AE_B04_MEAN` reader - this field configures block04 Y mean data"]
pub type AeB04MeanR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block12 Y mean data"]
    #[inline(always)]
    pub fn ae_b12_mean(&self) -> AeB12MeanR {
        AeB12MeanR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block11 Y mean data"]
    #[inline(always)]
    pub fn ae_b11_mean(&self) -> AeB11MeanR {
        AeB11MeanR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block10 Y mean data"]
    #[inline(always)]
    pub fn ae_b10_mean(&self) -> AeB10MeanR {
        AeB10MeanR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block04 Y mean data"]
    #[inline(always)]
    pub fn ae_b04_mean(&self) -> AeB04MeanR {
        AeB04MeanR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ae statistic result register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBlockMean1Spec;
impl crate::RegisterSpec for AeBlockMean1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_1::R`](R) reader structure"]
impl crate::Readable for AeBlockMean1Spec {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_1 to value 0"]
impl crate::Resettable for AeBlockMean1Spec {}
