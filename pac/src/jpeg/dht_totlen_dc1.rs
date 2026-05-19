#[doc = "Register `DHT_TOTLEN_DC1` reader"]
pub type R = crate::R<DhtTotlenDc1Spec>;
#[doc = "Field `DHT_TOTLEN_DC1` reader - write the numbers of 1~n codeword length sum from 1~16 of dc1 table"]
pub type DhtTotlenDc1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - write the numbers of 1~n codeword length sum from 1~16 of dc1 table"]
    #[inline(always)]
    pub fn dht_totlen_dc1(&self) -> DhtTotlenDc1R {
        DhtTotlenDc1R::new(self.bits)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`dht_totlen_dc1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhtTotlenDc1Spec;
impl crate::RegisterSpec for DhtTotlenDc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dht_totlen_dc1::R`](R) reader structure"]
impl crate::Readable for DhtTotlenDc1Spec {}
#[doc = "`reset()` method sets DHT_TOTLEN_DC1 to value 0"]
impl crate::Resettable for DhtTotlenDc1Spec {}
