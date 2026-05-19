#[doc = "Register `MAIN_STATE` reader"]
pub type R = crate::R<MainStateSpec>;
#[doc = "Register `MAIN_STATE` writer"]
pub type W = crate::W<MainStateSpec>;
#[doc = "Field `ENABLE_CALI_PMU_CNTL` reader - need_des"]
pub type EnableCaliPmuCntlR = crate::BitReader;
#[doc = "Field `ENABLE_CALI_PMU_CNTL` writer - need_des"]
pub type EnableCaliPmuCntlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_MAIN_LAST_ST_STATE` reader - need_des"]
pub type PmuMainLastStStateR = crate::FieldReader;
#[doc = "Field `PMU_MAIN_TAR_ST_STATE` reader - need_des"]
pub type PmuMainTarStStateR = crate::FieldReader;
#[doc = "Field `PMU_MAIN_CUR_ST_STATE` reader - need_des"]
pub type PmuMainCurStStateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn enable_cali_pmu_cntl(&self) -> EnableCaliPmuCntlR {
        EnableCaliPmuCntlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:17 - need_des"]
    #[inline(always)]
    pub fn pmu_main_last_st_state(&self) -> PmuMainLastStStateR {
        PmuMainLastStStateR::new(((self.bits >> 11) & 0x7f) as u8)
    }
    #[doc = "Bits 18:24 - need_des"]
    #[inline(always)]
    pub fn pmu_main_tar_st_state(&self) -> PmuMainTarStStateR {
        PmuMainTarStStateR::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:31 - need_des"]
    #[inline(always)]
    pub fn pmu_main_cur_st_state(&self) -> PmuMainCurStStateR {
        PmuMainCurStStateR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn enable_cali_pmu_cntl(&mut self) -> EnableCaliPmuCntlW<'_, MainStateSpec> {
        EnableCaliPmuCntlW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`main_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MainStateSpec;
impl crate::RegisterSpec for MainStateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`main_state::R`](R) reader structure"]
impl crate::Readable for MainStateSpec {}
#[doc = "`write(|w| ..)` method takes [`main_state::W`](W) writer structure"]
impl crate::Writable for MainStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAIN_STATE to value 0x0210_0801"]
impl crate::Resettable for MainStateSpec {
    const RESET_VALUE: u32 = 0x0210_0801;
}
