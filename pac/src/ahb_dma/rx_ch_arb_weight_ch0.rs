#[doc = "Register `RX_CH_ARB_WEIGHT_CH0` reader"]
pub type R = crate::R<RxChArbWeightCh0Spec>;
#[doc = "Register `RX_CH_ARB_WEIGHT_CH0` writer"]
pub type W = crate::W<RxChArbWeightCh0Spec>;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH0` reader - Configures the weight(i.e the number of tokens) of RX channel0"]
pub type RxArbWeightValueCh0R = crate::FieldReader;
#[doc = "Field `RX_ARB_WEIGHT_VALUE_CH0` writer - Configures the weight(i.e the number of tokens) of RX channel0"]
pub type RxArbWeightValueCh0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel0"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch0(&self) -> RxArbWeightValueCh0R {
        RxArbWeightValueCh0R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures the weight(i.e the number of tokens) of RX channel0"]
    #[inline(always)]
    pub fn rx_arb_weight_value_ch0(&mut self) -> RxArbWeightValueCh0W<'_, RxChArbWeightCh0Spec> {
        RxArbWeightValueCh0W::new(self, 0)
    }
}
#[doc = "RX channel 0 arbitration weight configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ch_arb_weight_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ch_arb_weight_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxChArbWeightCh0Spec;
impl crate::RegisterSpec for RxChArbWeightCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ch_arb_weight_ch0::R`](R) reader structure"]
impl crate::Readable for RxChArbWeightCh0Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_ch_arb_weight_ch0::W`](W) writer structure"]
impl crate::Writable for RxChArbWeightCh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CH_ARB_WEIGHT_CH0 to value 0"]
impl crate::Resettable for RxChArbWeightCh0Spec {}
