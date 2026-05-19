#[doc = "Register `INFIFO_STATUS1_CH2` reader"]
pub type R = crate::R<InfifoStatus1Ch2Spec>;
#[doc = "Field `L1INFIFO_CNT_CH2` reader - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 2."]
pub type L1infifoCntCh2R = crate::FieldReader;
#[doc = "Field `L2INFIFO_CNT_CH2` reader - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 2."]
pub type L2infifoCntCh2R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - The register stores the byte number of the data in L1 Rx FIFO for Rx channel 2."]
    #[inline(always)]
    pub fn l1infifo_cnt_ch2(&self) -> L1infifoCntCh2R {
        L1infifoCntCh2R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - The register stores the byte number of the data in L2 Rx FIFO for Rx channel 2."]
    #[inline(always)]
    pub fn l2infifo_cnt_ch2(&self) -> L2infifoCntCh2R {
        L2infifoCntCh2R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[doc = "Receive FIFO status of Rx channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`infifo_status1_ch2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfifoStatus1Ch2Spec;
impl crate::RegisterSpec for InfifoStatus1Ch2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infifo_status1_ch2::R`](R) reader structure"]
impl crate::Readable for InfifoStatus1Ch2Spec {}
#[doc = "`reset()` method sets INFIFO_STATUS1_CH2 to value 0"]
impl crate::Resettable for InfifoStatus1Ch2Spec {}
