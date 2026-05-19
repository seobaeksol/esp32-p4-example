#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH2` reader"]
pub type R = crate::R<RxArbWeightOptDirCh2Spec>;
#[doc = "Register `RX_ARB_WEIGHT_OPT_DIR_CH2` writer"]
pub type W = crate::W<RxArbWeightOptDirCh2Spec>;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH2` reader - reserved"]
pub type RxArbWeightOptDisCh2R = crate::BitReader;
#[doc = "Field `RX_ARB_WEIGHT_OPT_DIS_CH2` writer - reserved"]
pub type RxArbWeightOptDisCh2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch2(&self) -> RxArbWeightOptDisCh2R {
        RxArbWeightOptDisCh2R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn rx_arb_weight_opt_dis_ch2(
        &mut self,
    ) -> RxArbWeightOptDisCh2W<'_, RxArbWeightOptDirCh2Spec> {
        RxArbWeightOptDisCh2W::new(self, 0)
    }
}
#[doc = "RX channel 2 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_arb_weight_opt_dir_ch2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_arb_weight_opt_dir_ch2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxArbWeightOptDirCh2Spec;
impl crate::RegisterSpec for RxArbWeightOptDirCh2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_arb_weight_opt_dir_ch2::R`](R) reader structure"]
impl crate::Readable for RxArbWeightOptDirCh2Spec {}
#[doc = "`write(|w| ..)` method takes [`rx_arb_weight_opt_dir_ch2::W`](W) writer structure"]
impl crate::Writable for RxArbWeightOptDirCh2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_ARB_WEIGHT_OPT_DIR_CH2 to value 0"]
impl crate::Resettable for RxArbWeightOptDirCh2Spec {}
