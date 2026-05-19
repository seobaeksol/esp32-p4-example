#[doc = "Register `OUTFIFO_STATUS1_CH2` reader"]
pub type R = crate::R<OutfifoStatus1Ch2Spec>;
#[doc = "Field `L1OUTFIFO_CNT_CH2` reader - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
pub type L1outfifoCntCh2R = crate::FieldReader;
#[doc = "Field `L2OUTFIFO_CNT_CH2` reader - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 2."]
pub type L2outfifoCntCh2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn l1outfifo_cnt_ch2(&self) -> L1outfifoCntCh2R {
        L1outfifoCntCh2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Tx FIFO for Tx channel 2."]
    #[inline(always)]
    pub fn l2outfifo_cnt_ch2(&self) -> L2outfifoCntCh2R {
        L2outfifoCntCh2R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[doc = "Receive FIFO status of Tx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`outfifo_status1_ch2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutfifoStatus1Ch2Spec;
impl crate::RegisterSpec for OutfifoStatus1Ch2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outfifo_status1_ch2::R`](R) reader structure"]
impl crate::Readable for OutfifoStatus1Ch2Spec {}
#[doc = "`reset()` method sets OUTFIFO_STATUS1_CH2 to value 0"]
impl crate::Resettable for OutfifoStatus1Ch2Spec {}
