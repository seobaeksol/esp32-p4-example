#[doc = "Register `AE_BLOCK_MEAN_0` reader"]
pub type R = crate::R<AeBlockMean0Spec>;
#[doc = "Field `AE_B03_MEAN` reader - this field configures block03 Y mean data"]
pub type AeB03MeanR = crate::FieldReader;
#[doc = "Field `AE_B02_MEAN` reader - this field configures block02 Y mean data"]
pub type AeB02MeanR = crate::FieldReader;
#[doc = "Field `AE_B01_MEAN` reader - this field configures block01 Y mean data"]
pub type AeB01MeanR = crate::FieldReader;
#[doc = "Field `AE_B00_MEAN` reader - this field configures block00 Y mean data"]
pub type AeB00MeanR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block03 Y mean data"]
    #[inline(always)]
    pub fn ae_b03_mean(&self) -> AeB03MeanR {
        AeB03MeanR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block02 Y mean data"]
    #[inline(always)]
    pub fn ae_b02_mean(&self) -> AeB02MeanR {
        AeB02MeanR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block01 Y mean data"]
    #[inline(always)]
    pub fn ae_b01_mean(&self) -> AeB01MeanR {
        AeB01MeanR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block00 Y mean data"]
    #[inline(always)]
    pub fn ae_b00_mean(&self) -> AeB00MeanR {
        AeB00MeanR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ae statistic result register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBlockMean0Spec;
impl crate::RegisterSpec for AeBlockMean0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_0::R`](R) reader structure"]
impl crate::Readable for AeBlockMean0Spec {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_0 to value 0"]
impl crate::Resettable for AeBlockMean0Spec {}
