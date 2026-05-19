#[doc = "Register `TX_ARB_WEIGHT_OPT_DIR_CH0` reader"]
pub type R = crate::R<TxArbWeightOptDirCh0Spec>;
#[doc = "Register `TX_ARB_WEIGHT_OPT_DIR_CH0` writer"]
pub type W = crate::W<TxArbWeightOptDirCh0Spec>;
#[doc = "Field `TX_ARB_WEIGHT_OPT_DIS_CH0` reader - reserved"]
pub type TxArbWeightOptDisCh0R = crate::BitReader;
#[doc = "Field `TX_ARB_WEIGHT_OPT_DIS_CH0` writer - reserved"]
pub type TxArbWeightOptDisCh0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weight_opt_dis_ch0(&self) -> TxArbWeightOptDisCh0R {
        TxArbWeightOptDisCh0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn tx_arb_weight_opt_dis_ch0(
        &mut self,
    ) -> TxArbWeightOptDisCh0W<'_, TxArbWeightOptDirCh0Spec> {
        TxArbWeightOptDisCh0W::new(self, 0)
    }
}
#[doc = "TX channel 0 weight arbitration optimization enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_arb_weight_opt_dir_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_arb_weight_opt_dir_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxArbWeightOptDirCh0Spec;
impl crate::RegisterSpec for TxArbWeightOptDirCh0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_arb_weight_opt_dir_ch0::R`](R) reader structure"]
impl crate::Readable for TxArbWeightOptDirCh0Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_arb_weight_opt_dir_ch0::W`](W) writer structure"]
impl crate::Writable for TxArbWeightOptDirCh0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_ARB_WEIGHT_OPT_DIR_CH0 to value 0"]
impl crate::Resettable for TxArbWeightOptDirCh0Spec {}
