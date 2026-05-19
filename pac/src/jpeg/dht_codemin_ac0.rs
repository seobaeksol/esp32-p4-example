#[doc = "Register `DHT_CODEMIN_AC0` reader"]
pub type R = crate::R<DhtCodeminAc0Spec>;
#[doc = "Field `DHT_CODEMIN_AC0` reader - write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word"]
pub type DhtCodeminAc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the minimum codeword of code length from 1~16 of ac0 table. The codeword is left shifted to the MSB position of a 16bit word"]
    #[inline(always)]
    pub fn dht_codemin_ac0(&self) -> DhtCodeminAc0R {
        DhtCodeminAc0R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_codemin_ac0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtCodeminAc0Spec;
impl crate::RegisterSpec for DhtCodeminAc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_codemin_ac0::R`](R) reader structure"]
impl crate::Readable for DhtCodeminAc0Spec {}
#[doc = "`reset()` method sets DHT_CODEMIN_AC0 to value 0"]
impl crate::Resettable for DhtCodeminAc0Spec {}
