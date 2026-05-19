#[doc = "Register `RX_CH_ARB_WEIGHT_CH1` reader"]
pub type R = crate::R<RxChArbWeightCh1Spec>;
#[doc = "Register `RX_CH_ARB_WEIGHT_CH1` writer"]
pub type W = crate::W<RxChArbWeightCh1Spec>;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH1` reader - Configures the weight(i.e the number of tokens) of RX channel1"]
pub type RxArbWeightValueCh1R = crate::FieldReader;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH1` writer - Configures the weight(i.e the number of tokens) of RX channel1"]
pub type RxArbWeightValueCh1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel1"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch1(&self) -> RxArbWeightValueCh1R {
        RxArbWeightValueCh1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel1"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch1(&mut self) -> RxArbWeightValueCh1W<'_, RxChArbWeightCh1Spec> {
        RxArbWeightValueCh1W::new(self, 0)
    }
}
#[doc = "RX channel 1 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxChArbWeightCh1Spec;
impl crate::RegisterSpec for RxChArbWeightCh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch_arb_weight_ch1::R`](R) reader structure"]
impl crate::Readable for RxChArbWeightCh1Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ch_arb_weight_ch1::W`](W) writer structure"]
impl crate::Writable for RxChArbWeightCh1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CH_ARB_WEIGHT_CH1 to value 0"]
impl crate::Resettable for RxChArbWeightCh1Spec {}
