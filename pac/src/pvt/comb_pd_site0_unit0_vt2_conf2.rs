#[doc = "Register `COMB_PD_SITE0_UNIT0_VT2_CONF2` reader"]
pub type R = crate::R<CombPdSite0Unit0Vt2Conf2Spec>;
#[doc = "Register `COMB_PD_SITE0_UNIT0_VT2_CONF2` writer"]
pub type W = crate::W<CombPdSite0Unit0Vt2Conf2Spec>;
#[doc = "Field `MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0` reader - needs field desc"]
pub type MonitorEdgModVt2PdSite0Unit0R = crate::FieldReader;
#[doc = "Field `MONITOR_EDG_MOD_VT2_PD_SITE0_UNIT0` writer - needs field desc"]
pub type MonitorEdgModVt2PdSite0Unit0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DELAY_OVF_VT2_PD_SITE0_UNIT0` reader - needs field desc"]
pub type DelayOvfVt2PdSite0Unit0R = crate::BitReader;
#[doc = "Field `TIMING_ERR_CNT_O_VT2_PD_SITE0_UNIT0` reader - needs field desc"]
pub type TimingErrCntOVt2PdSite0Unit0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit0(&self) -> MonitorEdgModVt2PdSite0Unit0R {
        MonitorEdgModVt2PdSite0Unit0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - needs field desc"]
    #[inline(always)]
    pub fn delay_ovf_vt2_pd_site0_unit0(&self) -> DelayOvfVt2PdSite0Unit0R {
        DelayOvfVt2PdSite0Unit0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - needs field desc"]
    #[inline(always)]
    pub fn timing_err_cnt_o_vt2_pd_site0_unit0(&self) -> TimingErrCntOVt2PdSite0Unit0R {
        TimingErrCntOVt2PdSite0Unit0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - needs field desc"]
    #[inline(always)]
    pub fn monitor_edg_mod_vt2_pd_site0_unit0(
        &mut self,
    ) -> MonitorEdgModVt2PdSite0Unit0W<'_, CombPdSite0Unit0Vt2Conf2Spec> {
        MonitorEdgModVt2PdSite0Unit0W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`comb_pd_site0_unit0_vt2_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comb_pd_site0_unit0_vt2_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CombPdSite0Unit0Vt2Conf2Spec;
impl crate::RegisterSpec for CombPdSite0Unit0Vt2Conf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comb_pd_site0_unit0_vt2_conf2::R`](R) reader structure"]
impl crate::Readable for CombPdSite0Unit0Vt2Conf2Spec {}
#[doc = "`write(|w| ..)` method takes [`comb_pd_site0_unit0_vt2_conf2::W`](W) writer structure"]
impl crate::Writable for CombPdSite0Unit0Vt2Conf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMB_PD_SITE0_UNIT0_VT2_CONF2 to value 0"]
impl crate::Resettable for CombPdSite0Unit0Vt2Conf2Spec {}
