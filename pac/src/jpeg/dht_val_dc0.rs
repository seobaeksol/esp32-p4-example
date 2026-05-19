#[doc = "Register `DHT_VAl_DC0` reader"]
pub type R = crate::R<DhtValDc0Spec>;
#[doc = "Field `DHT_VAL_DC0` reader - write codeword corresponding huffman values of dc0 table"]
pub type DhtValDc0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write codeword corresponding huffman values of dc0 table"]
    #[inline(always)]
    pub fn dht_val_dc0(&self) -> DhtValDc0R {
        DhtValDc0R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_val_dc0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtValDc0Spec;
impl crate::RegisterSpec for DhtValDc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_val_dc0::R`](R) reader structure"]
impl crate::Readable for DhtValDc0Spec {}
#[doc = "`reset()` method sets DHT_VAl_DC0 to value 0"]
impl crate::Resettable for DhtValDc0Spec {}
