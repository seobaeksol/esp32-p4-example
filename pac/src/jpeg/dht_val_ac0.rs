#[doc = "Register `DHT_VAl_AC0` reader"]
pub type R = crate::R<DhtValAc0Spec>;
#[doc = "Field `DHT_VAL_AC0` reader - write codeword corresponding huffman values of ac0 table"]
pub type DhtValAc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write codeword corresponding huffman values of ac0 table"]
    #[inline(always)]
    pub fn dht_val_ac0(&self) -> DhtValAc0R {
        DhtValAc0R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_val_ac0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtValAc0Spec;
impl crate::RegisterSpec for DhtValAc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_val_ac0::R`](R) reader structure"]
impl crate::Readable for DhtValAc0Spec {}
#[doc = "`reset()` method sets DHT_VAl_AC0 to value 0"]
impl crate::Resettable for DhtValAc0Spec {}
