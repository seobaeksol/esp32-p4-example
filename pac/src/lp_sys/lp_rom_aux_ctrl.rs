#[doc = "Register `LP_ROM_AUX_CTRL` reader"]
pub type R = crate::R<LpRomAuxCtrlSpec>;
#[doc = "Register `LP_ROM_AUX_CTRL` writer"]
pub type W = crate::W<LpRomAuxCtrlSpec>;
#[doc = "Field `LP_ROM_AUX_CTRL` reader - need_des"]
pub type LpRomAuxCtrlR = crate::FieldReader<u32>;
#[doc = "Field `LP_ROM_AUX_CTRL` writer - need_des"]
pub type LpRomAuxCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_rom_aux_ctrl(&self) -> LpRomAuxCtrlR {
        LpRomAuxCtrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_rom_aux_ctrl(&mut self) -> LpRomAuxCtrlW<'_, LpRomAuxCtrlSpec> {
        LpRomAuxCtrlW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rom_aux_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rom_aux_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpRomAuxCtrlSpec;
impl crate::RegisterSpec for LpRomAuxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rom_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for LpRomAuxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lp_rom_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for LpRomAuxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_ROM_AUX_CTRL to value 0x70"]
impl crate::Resettable for LpRomAuxCtrlSpec {
    const RESET_VALUE: u32 = 0x70;
}
