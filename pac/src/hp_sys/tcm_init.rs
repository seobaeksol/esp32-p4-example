#[doc = "Register `TCM_INIT` reader"]
pub type R = crate::R<TcmInitSpec>;
#[doc = "Register `TCM_INIT` writer"]
pub type W = crate::W<TcmInitSpec>;
#[doc = "Field `REG_TCM_INIT_EN` reader - NA"]
pub type RegTcmInitEnR = crate::BitReader;
#[doc = "Field `REG_TCM_INIT_EN` writer - NA"]
pub type RegTcmInitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TCM_INIT_CNT_RESET` reader - Set 1 to reset tcm init cnt"]
pub type RegTcmInitCntResetR = crate::BitReader;
#[doc = "Field `REG_TCM_INIT_CNT_RESET` writer - Set 1 to reset tcm init cnt"]
pub type RegTcmInitCntResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TCM_INIT_DONE` reader - NA"]
pub type RegTcmInitDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_en(&self) -> RegTcmInitEnR {
        RegTcmInitEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    pub fn reg_tcm_init_cnt_reset(&self) -> RegTcmInitCntResetR {
        RegTcmInitCntResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_done(&self) -> RegTcmInitDoneR {
        RegTcmInitDoneR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_en(&mut self) -> RegTcmInitEnW<'_, TcmInitSpec> {
        RegTcmInitEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    pub fn reg_tcm_init_cnt_reset(&mut self) -> RegTcmInitCntResetW<'_, TcmInitSpec> {
        RegTcmInitCntResetW::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmInitSpec;
impl crate::RegisterSpec for TcmInitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_init::R`](R) reader structure"]
impl crate::Readable for TcmInitSpec {}
#[doc = "`write(|w| ..)` method takes [`tcm_init::W`](W) writer structure"]
impl crate::Writable for TcmInitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INIT to value 0x02"]
impl crate::Resettable for TcmInitSpec {
    const RESET_VALUE: u32 = 0x02;
}
