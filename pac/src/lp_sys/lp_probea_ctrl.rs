#[doc = "Register `LP_PROBEA_CTRL` reader"]
pub type R = crate::R<LpProbeaCtrlSpec>;
#[doc = "Register `LP_PROBEA_CTRL` writer"]
pub type W = crate::W<LpProbeaCtrlSpec>;
#[doc = "Field `PROBE_A_MOD_SEL` reader - need_des"]
pub type ProbeAModSelR = crate::FieldReader<u16>;
#[doc = "Field `PROBE_A_MOD_SEL` writer - need_des"]
pub type ProbeAModSelW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PROBE_A_TOP_SEL` reader - need_des"]
pub type ProbeATopSelR = crate::FieldReader;
#[doc = "Field `PROBE_A_TOP_SEL` writer - need_des"]
pub type ProbeATopSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PROBE_L_SEL` reader - need_des"]
pub type ProbeLSelR = crate::FieldReader;
#[doc = "Field `PROBE_L_SEL` writer - need_des"]
pub type ProbeLSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PROBE_H_SEL` reader - need_des"]
pub type ProbeHSelR = crate::FieldReader;
#[doc = "Field `PROBE_H_SEL` writer - need_des"]
pub type ProbeHSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PROBE_GLOBAL_EN` reader - need_des"]
pub type ProbeGlobalEnR = crate::BitReader;
#[doc = "Field `PROBE_GLOBAL_EN` writer - need_des"]
pub type ProbeGlobalEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn probe_a_mod_sel(&self) -> ProbeAModSelR {
        ProbeAModSelR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn probe_a_top_sel(&self) -> ProbeATopSelR {
        ProbeATopSelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn probe_l_sel(&self) -> ProbeLSelR {
        ProbeLSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn probe_h_sel(&self) -> ProbeHSelR {
        ProbeHSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn probe_global_en(&self) -> ProbeGlobalEnR {
        ProbeGlobalEnR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn probe_a_mod_sel(&mut self) -> ProbeAModSelW<'_, LpProbeaCtrlSpec> {
        ProbeAModSelW::new(self, 0)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn probe_a_top_sel(&mut self) -> ProbeATopSelW<'_, LpProbeaCtrlSpec> {
        ProbeATopSelW::new(self, 16)
    }
    #[doc = "Bits 24:25 - need_des"]
    #[inline(always)]
    pub fn probe_l_sel(&mut self) -> ProbeLSelW<'_, LpProbeaCtrlSpec> {
        ProbeLSelW::new(self, 24)
    }
    #[doc = "Bits 26:27 - need_des"]
    #[inline(always)]
    pub fn probe_h_sel(&mut self) -> ProbeHSelW<'_, LpProbeaCtrlSpec> {
        ProbeHSelW::new(self, 26)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn probe_global_en(&mut self) -> ProbeGlobalEnW<'_, LpProbeaCtrlSpec> {
        ProbeGlobalEnW::new(self, 28)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_probea_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_probea_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpProbeaCtrlSpec;
impl crate::RegisterSpec for LpProbeaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_probea_ctrl::R`](R) reader structure"]
impl crate::Readable for LpProbeaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_probea_ctrl::W`](W) writer structure"]
impl crate::Writable for LpProbeaCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PROBEA_CTRL to value 0"]
impl crate::Resettable for LpProbeaCtrlSpec {}
