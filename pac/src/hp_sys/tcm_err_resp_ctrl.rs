#[doc = "Register `TCM_ERR_RESP_CTRL` reader"]
pub type R = crate::R<TcmErrRespCtrlSpec>;
#[doc = "Register `TCM_ERR_RESP_CTRL` writer"]
pub type W = crate::W<TcmErrRespCtrlSpec>;
#[doc = "Field `TCM_ERR_RESP_EN` reader - Set 1 to turn on tcm error response"]
pub type TcmErrRespEnR = crate::BitReader;
#[doc = "Field `TCM_ERR_RESP_EN` writer - Set 1 to turn on tcm error response"]
pub type TcmErrRespEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to turn on tcm error response"]
    #[inline(always)]
    pub fn tcm_err_resp_en(&self) -> TcmErrRespEnR {
        TcmErrRespEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to turn on tcm error response"]
    #[inline(always)]
    pub fn tcm_err_resp_en(&mut self) -> TcmErrRespEnW<'_, TcmErrRespCtrlSpec> {
        TcmErrRespEnW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_err_resp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_err_resp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmErrRespCtrlSpec;
impl crate::RegisterSpec for TcmErrRespCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_err_resp_ctrl::R`](R) reader structure"]
impl crate::Readable for TcmErrRespCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_err_resp_ctrl::W`](W) writer structure"]
impl crate::Writable for TcmErrRespCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_ERR_RESP_CTRL to value 0"]
impl crate::Resettable for TcmErrRespCtrlSpec {}
