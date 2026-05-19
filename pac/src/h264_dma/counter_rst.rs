#[doc = "Register `COUNTER_RST` reader"]
pub type R = crate::R<CounterRstSpec>;
#[doc = "Register `COUNTER_RST` writer"]
pub type W = crate::W<CounterRstSpec>;
#[doc = "Field `RX_CH0_EXTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch0 counter."]
pub type RxCh0ExterCounterRstR = crate::BitReader;
#[doc = "Field `RX_CH0_EXTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch0 counter."]
pub type RxCh0ExterCounterRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH1_EXTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch1 counter."]
pub type RxCh1ExterCounterRstR = crate::BitReader;
#[doc = "Field `RX_CH1_EXTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch1 counter."]
pub type RxCh1ExterCounterRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH2_INTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch2 counter."]
pub type RxCh2InterCounterRstR = crate::BitReader;
#[doc = "Field `RX_CH2_INTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch2 counter."]
pub type RxCh2InterCounterRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH5_INTER_COUNTER_RST` reader - Write 1 then write 0 to this bit to reset rx ch5 counter."]
pub type RxCh5InterCounterRstR = crate::BitReader;
#[doc = "Field `RX_CH5_INTER_COUNTER_RST` writer - Write 1 then write 0 to this bit to reset rx ch5 counter."]
pub type RxCh5InterCounterRstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset rx ch0 counter."]
    #[inline(always)]
    pub fn rx_ch0_exter_counter_rst(&self) -> RxCh0ExterCounterRstR {
        RxCh0ExterCounterRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset rx ch1 counter."]
    #[inline(always)]
    pub fn rx_ch1_exter_counter_rst(&self) -> RxCh1ExterCounterRstR {
        RxCh1ExterCounterRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset rx ch2 counter."]
    #[inline(always)]
    pub fn rx_ch2_inter_counter_rst(&self) -> RxCh2InterCounterRstR {
        RxCh2InterCounterRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to this bit to reset rx ch5 counter."]
    #[inline(always)]
    pub fn rx_ch5_inter_counter_rst(&self) -> RxCh5InterCounterRstR {
        RxCh5InterCounterRstR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 then write 0 to this bit to reset rx ch0 counter."]
    #[inline(always)]
    pub fn rx_ch0_exter_counter_rst(&mut self) -> RxCh0ExterCounterRstW<'_, CounterRstSpec> {
        RxCh0ExterCounterRstW::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 then write 0 to this bit to reset rx ch1 counter."]
    #[inline(always)]
    pub fn rx_ch1_exter_counter_rst(&mut self) -> RxCh1ExterCounterRstW<'_, CounterRstSpec> {
        RxCh1ExterCounterRstW::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 then write 0 to this bit to reset rx ch2 counter."]
    #[inline(always)]
    pub fn rx_ch2_inter_counter_rst(&mut self) -> RxCh2InterCounterRstW<'_, CounterRstSpec> {
        RxCh2InterCounterRstW::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 then write 0 to this bit to reset rx ch5 counter."]
    #[inline(always)]
    pub fn rx_ch5_inter_counter_rst(&mut self) -> RxCh5InterCounterRstW<'_, CounterRstSpec> {
        RxCh5InterCounterRstW::new(self, 3)
    }
}
#[doc = "counter reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`counter_rst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter_rst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterRstSpec;
impl crate::RegisterSpec for CounterRstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter_rst::R`](R) reader structure"]
impl crate::Readable for CounterRstSpec {}
#[doc = "`write(|w| ..)` method takes [`counter_rst::W`](W) writer structure"]
impl crate::Writable for CounterRstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNTER_RST to value 0"]
impl crate::Resettable for CounterRstSpec {}
