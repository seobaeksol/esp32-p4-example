#[doc = "Register `AE_BLOCK_MEAN_4` reader"]
pub type R = crate::R<AeBlockMean4Spec>;
#[doc = "Field `AE_B34_MEAN` reader - this field configures block34 Y mean data"]
pub type AeB34MeanR = crate::FieldReader;
#[doc = "Field `AE_B33_MEAN` reader - this field configures block33 Y mean data"]
pub type AeB33MeanR = crate::FieldReader;
#[doc = "Field `AE_B32_MEAN` reader - this field configures block32 Y mean data"]
pub type AeB32MeanR = crate::FieldReader;
#[doc = "Field `AE_B31_MEAN` reader - this field configures block31 Y mean data"]
pub type AeB31MeanR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - this field configures block34 Y mean data"]
    #[inline(always)]
    pub fn ae_b34_mean(&self) -> AeB34MeanR {
        AeB34MeanR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures block33 Y mean data"]
    #[inline(always)]
    pub fn ae_b33_mean(&self) -> AeB33MeanR {
        AeB33MeanR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures block32 Y mean data"]
    #[inline(always)]
    pub fn ae_b32_mean(&self) -> AeB32MeanR {
        AeB32MeanR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures block31 Y mean data"]
    #[inline(always)]
    pub fn ae_b31_mean(&self) -> AeB31MeanR {
        AeB31MeanR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "ae statistic result register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ae_block_mean_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AeBlockMean4Spec;
impl crate::RegisterSpec for AeBlockMean4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ae_block_mean_4::R`](R) reader structure"]
impl crate::Readable for AeBlockMean4Spec {}
#[doc = "`reset()` method sets AE_BLOCK_MEAN_4 to value 0"]
impl crate::Resettable for AeBlockMean4Spec {}
