#[doc = "Register `TCM_SW_PARITY_BWE_MASK` reader"]
pub type R = crate::R<TcmSwParityBweMaskSpec>;
#[doc = "Register `TCM_SW_PARITY_BWE_MASK` writer"]
pub type W = crate::W<TcmSwParityBweMaskSpec>;
#[doc = "Field `REG_TCM_SW_PARITY_BWE_MASK_CTRL` reader - Set 1 to mask tcm bwe parity code bit"]
pub type RegTcmSwParityBweMaskCtrlR = crate::BitReader;
#[doc = "Field `REG_TCM_SW_PARITY_BWE_MASK_CTRL` writer - Set 1 to mask tcm bwe parity code bit"]
pub type RegTcmSwParityBweMaskCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask tcm bwe parity code bit"]
    #[inline(always)]
    pub fn reg_tcm_sw_parity_bwe_mask_ctrl(&self) -> RegTcmSwParityBweMaskCtrlR {
        RegTcmSwParityBweMaskCtrlR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask tcm bwe parity code bit"]
    #[inline(always)]
    pub fn reg_tcm_sw_parity_bwe_mask_ctrl(
        &mut self,
    ) -> RegTcmSwParityBweMaskCtrlW<'_, TcmSwParityBweMaskSpec> {
        RegTcmSwParityBweMaskCtrlW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_sw_parity_bwe_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_sw_parity_bwe_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmSwParityBweMaskSpec;
impl crate::RegisterSpec for TcmSwParityBweMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_sw_parity_bwe_mask::R`](R) reader structure"]
impl crate::Readable for TcmSwParityBweMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_sw_parity_bwe_mask::W`](W) writer structure"]
impl crate::Writable for TcmSwParityBweMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_SW_PARITY_BWE_MASK to value 0"]
impl crate::Resettable for TcmSwParityBweMaskSpec {}
