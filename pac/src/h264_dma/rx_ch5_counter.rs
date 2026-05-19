#[doc = "Register `RX_CH5_COUNTER` reader"]
pub type R = crate::R<RxCh5CounterSpec>;
#[doc = "Field `RX_CH5_CNT` reader - rx ch5 counter register"]
pub type RxCh5CntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:16 - rx ch5 counter register"]
    #[inline(always)]
    pub fn rx_ch5_cnt(&self) -> RxCh5CntR {
        RxCh5CntR::new(self.bits & 0x0001_ffff)
    }
}
#[doc = "rx ch5 counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch5_counter::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCh5CounterSpec;
impl crate::RegisterSpec for RxCh5CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch5_counter::R`](R) reader structure"]
impl crate::Readable for RxCh5CounterSpec {}
#[doc = "`reset()` method sets RX_CH5_COUNTER to value 0"]
impl crate::Resettable for RxCh5CounterSpec {}
