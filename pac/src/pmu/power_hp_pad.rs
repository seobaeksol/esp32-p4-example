#[doc = "Register `POWER_HP_PAD` reader"]
pub type R = crate::R<PowerHpPadSpec>;
#[doc = "Register `POWER_HP_PAD` writer"]
pub type W = crate::W<PowerHpPadSpec>;
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` reader - need_des"]
pub type ForceHpPadNoIsoAllR = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_NO_ISO_ALL` writer - need_des"]
pub type ForceHpPadNoIsoAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` reader - need_des"]
pub type ForceHpPadIsoAllR = crate::BitReader;
#[doc = "Field `FORCE_HP_PAD_ISO_ALL` writer - need_des"]
pub type ForceHpPadIsoAllW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_no_iso_all(&self) -> ForceHpPadNoIsoAllR {
        ForceHpPadNoIsoAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_iso_all(&self) -> ForceHpPadIsoAllR {
        ForceHpPadIsoAllR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_no_iso_all(&mut self) -> ForceHpPadNoIsoAllW<'_, PowerHpPadSpec> {
        ForceHpPadNoIsoAllW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn force_hp_pad_iso_all(&mut self) -> ForceHpPadIsoAllW<'_, PowerHpPadSpec> {
        ForceHpPadIsoAllW::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_hp_pad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_hp_pad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerHpPadSpec;
impl crate::RegisterSpec for PowerHpPadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_hp_pad::R`](R) reader structure"]
impl crate::Readable for PowerHpPadSpec {}
#[doc = "`write(|w| ..)` method takes [`power_hp_pad::W`](W) writer structure"]
impl crate::Writable for PowerHpPadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER_HP_PAD to value 0"]
impl crate::Resettable for PowerHpPadSpec {}
