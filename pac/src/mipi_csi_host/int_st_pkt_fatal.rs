#[doc = "Register `INT_ST_PKT_FATAL` reader"]
pub type R = crate::R<IntStPktFatalSpec>;
#[doc = "Field `ST_ERR_ECC_DOUBLE` reader - NA"]
pub type StErrEccDoubleR = crate::BitReader;
#[doc = "Field `ST_SHORTER_PAYLOAD` reader - NA"]
pub type StShorterPayloadR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn st_err_ecc_double(&self) -> StErrEccDoubleR {
        StErrEccDoubleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn st_shorter_payload(&self) -> StShorterPayloadR {
        StShorterPayloadR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st_pkt_fatal::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStPktFatalSpec;
impl crate::RegisterSpec for IntStPktFatalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st_pkt_fatal::R`](R) reader structure"]
impl crate::Readable for IntStPktFatalSpec {}
#[doc = "`reset()` method sets INT_ST_PKT_FATAL to value 0"]
impl crate::Resettable for IntStPktFatalSpec {}
