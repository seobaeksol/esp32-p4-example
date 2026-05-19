#[doc = "Register `COMB_PD_SITE2_UNIT1_VT0_CONF1` reader"]
pub type R = crate::R<CombPdSite2Unit1Vt0Conf1Spec>;
#[doc = "Register `COMB_PD_SITE2_UNIT1_VT0_CONF1` writer"]
pub type W = crate::W<CombPdSite2Unit1Vt0Conf1Spec>;
#[doc = "Field `MONITOR_EN_VT0_PD_SITE2_UNIT1` reader - needs field desc"]
pub type MonitorEnVt0PdSite2Unit1R = crate::BitReader;
#[doc = "Field `MONITOR_EN_VT0_PD_SITE2_UNIT1` writer - needs field desc"]
pub type MonitorEnVt0PdSite2Unit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_ERR_CNT_CLR_VT0_PD_SITE2_UNIT1` writer - needs field desc"]
pub type TimingErrCntClrVt0PdSite2Unit1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_LIMIT_VT0_PD_SITE2_UNIT1` reader - needs field desc"]
pub type DelayLimitVt0PdSite2Unit1R = crate::FieldReader;
#[doc = "Field `DELAY_LIMIT_VT0_PD_SITE2_UNIT1` writer - needs field desc"]
pub type DelayLimitVt0PdSite2Unit1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELAY_NUM_O_VT0_PD_SITE2_UNIT1` reader - needs field desc"]
pub type DelayNumOVt0PdSite2Unit1R = crate::FieldReader;
#[doc = "Field `TIMING_ERR_VT0_PD_SITE2_UNIT1` reader - needs field desc"]
pub type TimingErrVt0PdSite2Unit1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit1(&self) -> MonitorEnVt0PdSite2Unit1R {
        MonitorEnVt0PdSite2Unit1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit1(&self) -> DelayLimitVt0PdSite2Unit1R {
        DelayLimitVt0PdSite2Unit1R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_o_vt0_pd_site2_unit1(&self) -> DelayNumOVt0PdSite2Unit1R {
        DelayNumOVt0PdSite2Unit1R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_vt0_pd_site2_unit1(&self) -> TimingErrVt0PdSite2Unit1R {
        TimingErrVt0PdSite2Unit1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt0_pd_site2_unit1(
        &mut self,
    ) -> MonitorEnVt0PdSite2Unit1W<'_, CombPdSite2Unit1Vt0Conf1Spec> {
        MonitorEnVt0PdSite2Unit1W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_clr_vt0_pd_site2_unit1(
        &mut self,
    ) -> TimingErrCntClrVt0PdSite2Unit1W<'_, CombPdSite2Unit1Vt0Conf1Spec> {
        TimingErrCntClrVt0PdSite2Unit1W::new(self, 1)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt0_pd_site2_unit1(
        &mut self,
    ) -> DelayLimitVt0PdSite2Unit1W<'_, CombPdSite2Unit1Vt0Conf1Spec> {
        DelayLimitVt0PdSite2Unit1W::new(self, 2)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site2_unit1_vt0_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site2_unit1_vt0_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombPdSite2Unit1Vt0Conf1Spec;
impl crate::RegisterSpec for CombPdSite2Unit1Vt0Conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site2_unit1_vt0_conf1::R`](R) reader structure"]
impl crate::Readable for CombPdSite2Unit1Vt0Conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site2_unit1_vt0_conf1::W`](W) writer structure"]
impl crate::Writable for CombPdSite2Unit1Vt0Conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE2_UNIT1_VT0_CONF1 to value 0x50"]
impl crate::Resettable for CombPdSite2Unit1Vt0Conf1Spec {
    const RESET_VALUE: u32 = 0x50;
}
