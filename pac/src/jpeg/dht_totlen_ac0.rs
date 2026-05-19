#[doc = "Register `DHT_TOTLEN_AC0` reader"]
pub type R = crate::R<DhtTotlenAc0Spec>;
#[doc = "Field `DHT_TOTLEN_AC0` reader - write the numbers of 1~n codeword length sum from 1~16 of ac0 table"]
pub type DhtTotlenAc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the numbers of 1~n codeword length sum from 1~16 of ac0 table"]
    #[inline(always)]
    pub fn dht_totlen_ac0(&self) -> DhtTotlenAc0R {
        DhtTotlenAc0R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_totlen_ac0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtTotlenAc0Spec;
impl crate::RegisterSpec for DhtTotlenAc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_totlen_ac0::R`](R) reader structure"]
impl crate::Readable for DhtTotlenAc0Spec {}
#[doc = "`reset()` method sets DHT_TOTLEN_AC0 to value 0"]
impl crate::Resettable for DhtTotlenAc0Spec {}
