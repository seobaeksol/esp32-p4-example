#[doc = "Register `IMM_HP_APB_ICG` writer"]
pub type W = crate::W<ImmHpApbIcgSpec>;
#[doc = "Field `UPDATE_DIG_ICG_APB_EN` writer - need_des"]
pub type UpdateDigIcgApbEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn update_dig_icg_apb_en(&mut self) -> UpdateDigIcgApbEnW<'_, ImmHpApbIcgSpec> {
        UpdateDigIcgApbEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_apb_icg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImmHpApbIcgSpec;
impl crate::RegisterSpec for ImmHpApbIcgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`imm_hp_apb_icg::W`](W) writer structure"]
impl crate::Writable for ImmHpApbIcgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_HP_APB_ICG to value 0"]
impl crate::Resettable for ImmHpApbIcgSpec {}
