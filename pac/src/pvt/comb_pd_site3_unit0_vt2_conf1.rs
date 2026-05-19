#[doc = "Register `COMB_PD_SITE3_UNIT0_VT2_CONF1` reader"]
pub type R = crate::R<CombPdSite3Unit0Vt2Conf1Spec>;
#[doc = "Register `COMB_PD_SITE3_UNIT0_VT2_CONF1` writer"]
pub type W = crate::W<CombPdSite3Unit0Vt2Conf1Spec>;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE3_UNIT0` reader - needs field desc"]
pub type MonitorEnVt2PdSite3Unit0R = crate::BitReader;
#[doc = "Field `MONITOR_EN_VT2_PD_SITE3_UNIT0` writer - needs field desc"]
pub type MonitorEnVt2PdSite3Unit0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMING_ERR_CNT_CLR_VT2_PD_SITE3_UNIT0` writer - needs field desc"]
pub type TimingErrCntClrVt2PdSite3Unit0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE3_UNIT0` reader - needs field desc"]
pub type DelayLimitVt2PdSite3Unit0R = crate::FieldReader;
#[doc = "Field `DELAY_LIMIT_VT2_PD_SITE3_UNIT0` writer - needs field desc"]
pub type DelayLimitVt2PdSite3Unit0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DELAY_NUM_O_VT2_PD_SITE3_UNIT0` reader - needs field desc"]
pub type DelayNumOVt2PdSite3Unit0R = crate::FieldReader;
#[doc = "Field `TIMING_ERR_VT2_PD_SITE3_UNIT0` reader - needs field desc"]
pub type TimingErrVt2PdSite3Unit0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit0(&self) -> MonitorEnVt2PdSite3Unit0R {
        MonitorEnVt2PdSite3Unit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit0(&self) -> DelayLimitVt2PdSite3Unit0R {
        DelayLimitVt2PdSite3Unit0R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 23:30 - needs field desc"]
    #[inline(always)]
    pub fn delay_num_o_vt2_pd_site3_unit0(&self) -> DelayNumOVt2PdSite3Unit0R {
        DelayNumOVt2PdSite3Unit0R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_vt2_pd_site3_unit0(&self) -> TimingErrVt2PdSite3Unit0R {
        TimingErrVt2PdSite3Unit0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn monitor_en_vt2_pd_site3_unit0(
        &mut self,
    ) -> MonitorEnVt2PdSite3Unit0W<'_, CombPdSite3Unit0Vt2Conf1Spec> {
        MonitorEnVt2PdSite3Unit0W::new(self, 0)
    }
    #[doc = "Bit 1 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_clr_vt2_pd_site3_unit0(
        &mut self,
    ) -> TimingErrCntClrVt2PdSite3Unit0W<'_, CombPdSite3Unit0Vt2Conf1Spec> {
        TimingErrCntClrVt2PdSite3Unit0W::new(self, 1)
    }
    #[doc = "Bits 2:9 - needs field desc"]
    #[inline(always)]
    pub fn delay_limit_vt2_pd_site3_unit0(
        &mut self,
    ) -> DelayLimitVt2PdSite3Unit0W<'_, CombPdSite3Unit0Vt2Conf1Spec> {
        DelayLimitVt2PdSite3Unit0W::new(self, 2)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site3_unit0_vt2_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site3_unit0_vt2_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombPdSite3Unit0Vt2Conf1Spec;
impl crate::RegisterSpec for CombPdSite3Unit0Vt2Conf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site3_unit0_vt2_conf1::R`](R) reader structure"]
impl crate::Readable for CombPdSite3Unit0Vt2Conf1Spec {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site3_unit0_vt2_conf1::W`](W) writer structure"]
impl crate::Writable for CombPdSite3Unit0Vt2Conf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE3_UNIT0_VT2_CONF1 to value 0x50"]
impl crate::Resettable for CombPdSite3Unit0Vt2Conf1Spec {
    const RESET_VALUE: u32 = 0x50;
}
