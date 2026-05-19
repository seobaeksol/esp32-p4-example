#[doc = "Register `DHT_CODEMIN_AC1` reader"]
pub type R = crate::R<DhtCodeminAc1Spec>;
#[doc = "Field `DHT_CODEMIN_AC1` reader - write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word"]
pub type DhtCodeminAc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the minimum codeword of code length from 1~16 of ac1 table. The codeword is left shifted to the MSB position of a 16bit word"]
    #[inline(always)]
    pub fn dht_codemin_ac1(&self) -> DhtCodeminAc1R {
        DhtCodeminAc1R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_codemin_ac1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtCodeminAc1Spec;
impl crate::RegisterSpec for DhtCodeminAc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_codemin_ac1::R`](R) reader structure"]
impl crate::Readable for DhtCodeminAc1Spec {}
#[doc = "`reset()` method sets DHT_CODEMIN_AC1 to value 0"]
impl crate::Resettable for DhtCodeminAc1Spec {}
