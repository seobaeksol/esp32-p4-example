#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<IntEnaSpec>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<IntEnaSpec>;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE` reader - need_des"]
pub type VddbatChargeUpvoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UPVOLTAGE` writer - need_des"]
pub type VddbatChargeUpvoltageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE` reader - need_des"]
pub type VddbatChargeUndervoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_CHARGE_UNDERVOLTAGE` writer - need_des"]
pub type VddbatChargeUndervoltageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UPVOLTAGE` reader - need_des"]
pub type VddbatUpvoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_UPVOLTAGE` writer - need_des"]
pub type VddbatUpvoltageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAT_UNDERVOLTAGE` reader - need_des"]
pub type VddbatUndervoltageR = crate::BitReader;
#[doc = "Field `VDDBAT_UNDERVOLTAGE` writer - need_des"]
pub type VddbatUndervoltageW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOD_MODE0` reader - need_des"]
pub type BodMode0R = crate::BitReader;
#[doc = "Field `BOD_MODE0` writer - need_des"]
pub type BodMode0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage(&self) -> VddbatChargeUpvoltageR {
        VddbatChargeUpvoltageR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage(&self) -> VddbatChargeUndervoltageR {
        VddbatChargeUndervoltageR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage(&self) -> VddbatUpvoltageR {
        VddbatUpvoltageR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage(&self) -> VddbatUndervoltageR {
        VddbatUndervoltageR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&self) -> BodMode0R {
        BodMode0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_upvoltage(&mut self) -> VddbatChargeUpvoltageW<'_, IntEnaSpec> {
        VddbatChargeUpvoltageW::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn vddbat_charge_undervoltage(&mut self) -> VddbatChargeUndervoltageW<'_, IntEnaSpec> {
        VddbatChargeUndervoltageW::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn vddbat_upvoltage(&mut self) -> VddbatUpvoltageW<'_, IntEnaSpec> {
        VddbatUpvoltageW::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn vddbat_undervoltage(&mut self) -> VddbatUndervoltageW<'_, IntEnaSpec> {
        VddbatUndervoltageW::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn bod_mode0(&mut self) -> BodMode0W<'_, IntEnaSpec> {
        BodMode0W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnaSpec;
impl crate::RegisterSpec for IntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for IntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for IntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for IntEnaSpec {}
