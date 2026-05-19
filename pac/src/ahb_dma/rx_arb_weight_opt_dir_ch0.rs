#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH0` reader"]
pub type R = crate::R<RxArbWeightOptDirCh0Spec>;
#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH0` writer"]
pub type W = crate::W<RxArbWeightOptDirCh0Spec>;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH0` reader - reserved"]
pub type RxArbWeightOptDisCh0R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH0` writer - reserved"]
pub type RxArbWeightOptDisCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch0(&self) -> RxArbWeightOptDisCh0R {
        RxArbWeightOptDisCh0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch0(
        &mut self,
    ) -> RxArbWeightOptDisCh0W<'_, RxArbWeightOptDirCh0Spec> {
        RxArbWeightOptDisCh0W::new(self, 0)
    }
}
#[doc = "RX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxArbWeightOptDirCh0Spec;
impl crate::RegisterSpec for RxArbWeightOptDirCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_arb_weight_opt_dir_ch0::R`](R) reader structure"]
impl crate::Readable for RxArbWeightOptDirCh0Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_arb_weight_opt_dir_ch0::W`](W) writer structure"]
impl crate::Writable for RxArbWeightOptDirCh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ARB_WEIGHT_OPT_DIR_CH0 to value 0"]
impl crate::Resettable for RxArbWeightOptDirCh0Spec {}
