#[doc = "Register `RX_CH2_COUNTER` reader"]
pub type R = crate::R<RxCh2CounterSpec>;
#[doc = "Field `RX_CH2_CNT` reader - rx ch2 counter register"]
pub type RxCh2CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - rx ch2 counter register"]
    #[inline(always)]
    pub fn rx_ch2_cnt(&self) -> RxCh2CntR {
        RxCh2CntR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "rx ch2 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch2_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCh2CounterSpec;
impl crate::RegisterSpec for RxCh2CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch2_counter::R`](R) reader structure"]
impl crate::Readable for RxCh2CounterSpec {}
#[doc = "`reset()` method sets RX_CH2_COUNTER to value 0"]
impl crate::Resettable for RxCh2CounterSpec {}
