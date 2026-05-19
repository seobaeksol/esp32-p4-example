#[doc = "Register `COMB_PD_SITE1_UNIT0_VT0_CONF2` reader"]
pub type R = crate::R<CombPdSite1Unit0Vt0Conf2Spec>;
#[doc = "Register `COMB_PD_SITE1_UNIT0_VT0_CONF2` writer"]
pub type W = crate::W<CombPdSite1Unit0Vt0Conf2Spec>;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0` reader - needs field desc"]
pub type MonitorEdgModVt0PdSite1Unit0R = crate::FieldReader;
#[doc = "Field `MONITOR_EDG_MOD_VT0_PD_SITE1_UNIT0` writer - needs field desc"]
pub type MonitorEdgModVt0PdSite1Unit0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_OVF_VT0_PD_SITE1_UNIT0` reader - needs field desc"]
pub type DelayOvfVt0PdSite1Unit0R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT0_PD_SITE1_UNIT0` reader - needs field desc"]
pub type TimingErrCntOVt0PdSite1Unit0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit0(&self) -> MonitorEdgModVt0PdSite1Unit0R {
        MonitorEdgModVt0PdSite1Unit0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt0_pd_site1_unit0(&self) -> DelayOvfVt0PdSite1Unit0R {
        DelayOvfVt0PdSite1Unit0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt0_pd_site1_unit0(&self) -> TimingErrCntOVt0PdSite1Unit0R {
        TimingErrCntOVt0PdSite1Unit0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt0_pd_site1_unit0(
        &mut self,
    ) -> MonitorEdgModVt0PdSite1Unit0W<'_, CombPdSite1Unit0Vt0Conf2Spec> {
        MonitorEdgModVt0PdSite1Unit0W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site1_unit0_vt0_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site1_unit0_vt0_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombPdSite1Unit0Vt0Conf2Spec;
impl crate::RegisterSpec for CombPdSite1Unit0Vt0Conf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site1_unit0_vt0_conf2::R`](R) reader structure"]
impl crate::Readable for CombPdSite1Unit0Vt0Conf2Spec {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site1_unit0_vt0_conf2::W`](W) writer structure"]
impl crate::Writable for CombPdSite1Unit0Vt0Conf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE1_UNIT0_VT0_CONF2 to value 0"]
impl crate::Resettable for CombPdSite1Unit0Vt0Conf2Spec {}
